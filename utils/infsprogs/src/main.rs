use std::ffi::OsString;

mod common;
mod sysvars;
mod vfs;

use clap::{Parser, Subcommand};

use crate::vfs::VirtualFileSystem;
use std::io::Write;

const DESC: &str = const_format::formatcp!(
    "\x1b[39;49;1minfsprogs\x1b[0m {} - {}\nCopyright (c) 2024 {}",
    clap::crate_version!(),
    clap::crate_description!(),
    clap::crate_authors!()
);

// TODO: verbose mode
#[derive(Parser)]
#[command(name = "infsprogs")]
#[command(author, version)]
#[command(about = DESC)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Unpack {
        target: OsString,
        #[clap(short)]
        outdir: OsString,
    },
    Build {
        dir: OsString,
        #[clap(short)]
        outfile: OsString,
        #[clap(short, long)]
        inodes: Option<u32>,
        #[clap(short, long)]
        block_size: Option<u32>,
        #[clap(short, long)]
        num_blocks: Option<u64>,
    },
    Mkfs {
        #[clap(short, long)]
        inodes: Option<u32>,
        #[clap(short, long)]
        block_size: Option<u32>,
        #[clap(short, long)]
        num_blocks: Option<u64>,
        // can be one of file (default), dev (device like /dev/sda1)
        // target_type sets defaults: 4M data section (not counting sup/others)
        // for file, length of device (minus formatting) for dev
        #[clap(short, long)]
        target_type: Option<String>,
        dest: OsString,
    },
}

fn main() -> std::io::Result<()> {
    let args = Cli::parse();
    match args.command {
        Commands::Unpack { target, outdir } => unpack(target, outdir),
        Commands::Build {
            dir,
            outfile,
            inodes,
            block_size,
            num_blocks,
        } => build(
            dir,
            outfile,
            inodes.unwrap_or(256),
            block_size.unwrap_or(4096),
            num_blocks.unwrap_or(1024),
        ),
        Commands::Mkfs {
            inodes,
            block_size,
            num_blocks,
            target_type,
            dest,
        } => {
            let tt = target_type.unwrap_or("file".to_string());
            // more comprehension here...
            mkfs()
        }
    }
}

// TODO: better error handling library
fn unpack(target: OsString, outdir: OsString) -> std::io::Result<()> {
    let buf = match std::fs::read(target) {
        Ok(v) => v,
        Err(e) => exito(&e.to_string()),
    };
    let mut fs = match vfs::infs::FileSystem::from_bytes(&buf) {
        Some(f) => f,
        None => exito("filesystem creation failed"),
    };
    let root = std::path::PathBuf::from(outdir);
    std::fs::create_dir_all(&root).unwrap_or_else(|e| exito(&e.to_string()));
    let dent = fs
        .vfd_as_dentry(
            &fs.get_fd(1, 0)
                .unwrap_or_else(|| exito("could not get root fd")),
        )
        .unwrap_or_else(|| exito("root is not dentry"));
    _recurse_write_dentry(root, &mut fs, dent)
}
// TODO: copy perms
fn build(
    dir: OsString,
    outfile: OsString,
    inodes: u32,
    block_size: u32,
    num_blocks: u64,
) -> std::io::Result<()> {
    let mut fs = vfs::infs::mknrfs(inodes, block_size, num_blocks);
    let root = fs.vfd_as_dentry(&fs.get_fd(1, 0).unwrap()).unwrap();
    _recurse_read_localfs(std::path::PathBuf::from(&dir), &mut fs, root);
    let mut file = std::fs::File::create(outfile)?;
    file.write_all(&fs.to_bytes())?;
    Ok(())
}
fn mkfs() -> std::io::Result<()> {
    Ok(())
}

fn exito(s: &str) -> ! {
    eprintln!("infsprogs: error: {}", s);
    std::process::exit(1);
}

// Method for handling hardlinks: duplicate the file
//   We don't know what the mounting structure is going to look like
//   Preserving hardlinks is not safe
// Method for handling symlinks: insert as written
//   If the symlink is absolute this will generally break it
//   But that's also true for mounting an fs in different locations
fn _recurse_write_dentry(
    cwd: std::path::PathBuf,
    fs: &mut vfs::infs::FileSystem,
    dent: Box<dyn vfs::VirtualDentry>,
) -> std::io::Result<()> {
    std::fs::create_dir_all(&cwd).unwrap_or_else(|e| exito(&e.to_string()));
    for ent in dent.get_entries() {
        if ent.filename == "." || ent.filename == ".." {
            continue;
        }
        let mut fd = fs
            .get_fd(ent.inum, 0)
            .unwrap_or_else(|| exito("inode does not exist for dentry file"));
        let mut npath = cwd.clone();
        npath.push(ent.filename.clone());
        match (fs.file_perms(&fd).unwrap() & 0xf000) >> 12 {
            0 => {
                let mut file = std::fs::File::create(npath)?;
                file.write_all(&fs.read_to_eof(&mut fd).unwrap())?;
            }
            1 => {
                std::fs::create_dir_all(&npath)?;
                let stor = fs.vfd_as_dentry(&fd).unwrap();
                _recurse_write_dentry(npath, fs, stor)?;
            }
            2 => _gen_symlink(
                npath,
                common::bytes_to_string(&fs.read_to_eof(&mut fd).unwrap()),
            )?,
            _ => exito("unknown file type encountered"),
        }
    }
    Ok(())
}
fn _recurse_read_localfs(
    cwd: std::path::PathBuf,
    fs: &mut vfs::infs::FileSystem,
    mut cdent: Box<dyn vfs::VirtualDentry>,
) {
    let mut ents = cdent.get_entries();
    let parent_inode = cdent.get_inode();
    for de in std::fs::read_dir(&cwd).unwrap() {
        let d = de.unwrap();
        let ft = d.file_type().unwrap();
        let f = d.file_name().into_string().unwrap();
        let mut p = cwd.clone();
        p.push(&f);
        if ft.is_dir() {
            let ino = fs.create_directory(parent_inode, f.clone()).unwrap();
            let vfd = fs.vfd_as_dentry(&fs.get_fd(ino, 0).unwrap()).unwrap();
            _recurse_read_localfs(p, fs, vfd);
            ents.push(vfs::VirtualDentryEntry {
                inum: ino,
                filename: f,
            });
        } else if ft.is_symlink() {
            let ino = fs
                .create_file(
                    parent_inode,
                    f,
                    std::fs::read_link(&p)
                        .unwrap()
                        .into_os_string()
                        .into_string()
                        .unwrap()
                        .as_bytes(),
                )
                .unwrap();
            fs.chmod(&fs.get_fd(ino, 0).unwrap(), 0o20777);
        } else {
            let ino = fs.create_file(parent_inode, f, &std::fs::read(&p).unwrap());
        }
    }
}
#[cfg(unix)]
fn _gen_symlink(from: std::path::PathBuf, to: String) -> std::io::Result<()> {
    std::os::unix::fs::symlink(to, from)?;
    Ok(())
}
#[cfg(not(unix))]
fn _gen_symlink() -> ! {
    panic!("windows/non-unix not supported yet for symlinks due to divergence in directory/file symlinks. if you'd like to help add windows support, visit https://github.com/amyipdev/amyip.net");
}
