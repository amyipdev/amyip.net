use colored::Colorize;
use rocket::fs::relative;

fn main() {
    println!("cargo:rerun-if-changed=NULL");
    if std::env::var("GITHUB_WEBHOOK_AUTHENTICATION_AMYIPNET").is_ok() {
        println!("cargo:rustc-cfg=feature=\"reload_github\"");
    }
    let rcv = rustc_version::version().unwrap();
    match std::process::Command::new("wasm-pack")
        .arg("build")
        .arg("--target")
        .arg("web")
        .env(
            "RUSTC_VERSION",
            format!("{}.{}.{}", rcv.major, rcv.minor, rcv.patch),
        )
        .current_dir(relative!("svelte/wasm"))
        .status()
    {
        Ok(c) => {
            if !c.success() {
                eprintln!("{}", "amyip.net: shell-wasm build failed".red());
                std::process::exit(-3);
            }
            println!("{}", "amyip.net: shell-wasm built".bright_cyan());
        }
        Err(e) => {
            eprintln!("{} {}", "amyip.net: could not run wasm-pack: ".red(), e);
            std::process::exit(-4);
        }
    }
    match std::process::Command::new("npx")
        .arg("rollup")
        .arg("-c")
        .current_dir(relative!("svelte"))
        .status()
    {
        Ok(c) => {
            if !c.success() {
                eprintln!("{}", "amyip.net: svelte build failed".red());
                eprintln!(
                    "{}",
                    "amyip.net: try running `npx rollup -c' in the svelte directory".red()
                );
                std::process::exit(-2);
            }
            println!("{}", "amyip.net: svelte built".bright_cyan());
        }
        Err(e) => {
            eprintln!("{} {}", "amyip.net: could not run svelte build: ".red(), e);
            std::process::exit(-1);
        }
    }
    match std::process::Command::new("bash")
        .arg(relative!("utils/infs-stage/buildstage.sh"))
        .status()
    {
        Ok(c) => {
            if !c.success() {
                eprintln!("{}", "amyip.net: failed to build stage".red());
                std::process::exit(-5);
            }
            println!("{}", "amyip.net: stage built".bright_cyan());
        }
        Err(e) => {
            eprintln!("{} {}", "amyip.net: failed to build stage: ".red(), e);
            std::process::exit(-6);
        }
    }
}
