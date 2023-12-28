use xterm_js_rs::Terminal;

const LS_HELP: &str = "Usage: ls [OPTION]... [FILE]...
List information about the FILEs (the current directory by default).
Entries are sorted alphabetically.


 -a, --all             do not ignore entries starting with .
 -A, --almost-all      do not list implied . and ..
 -h, --human-readable  with -l, print sizes like 1K 234M 2G etc.
     --si              likewise, but use powers of 1000 not 1024
 -i, --inode           print the index number of each file
 -l                    use a long listing format
 --help                display this help and exit
 --version             output version information and exit 

Many POSIX ls features are not supported; see ls --version for more info.";
const LS_VSN: &str = "ls (IrisOS-nano) 0.1
Copyright (C) Amy Parker, 2023
License AGPLv3+: GNU AGPL version 3 or later <https://gnu.org/licenses/agpl.html>
This is free software; you are free to change and redistribute it.
There is NO WARRANTY, to the extent permitted by law.

Written by Amy Parker <amy@amyip.net>.
Based on ls by Richard M. Stallman and David MacKenzie.";
struct LsOpts {
    all: bool,
    almost_all: bool,
    human_readable: bool,
    si: bool,
    inode: bool,
    longlist: bool,
    help: bool,
    version: bool,
}
// TODO: check if /etc/passwd, /etc/group exists, and if so, print user/group name
pub fn ls(term: &Terminal, mut args: Vec<&str>) -> i32 {
    return 0;
}
