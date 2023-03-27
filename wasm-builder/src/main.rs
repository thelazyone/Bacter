use std::process::Command;

fn main() {
    let status = Command::new("wasm-pack")
        .arg("build")
        .arg("--target")
        .arg("web")
        .arg("--")
        .arg("-p")
        .arg("bacter-wasm")
        .status()
        .expect("Failed to execute wasm-pack");

    if !status.success() {
        std::process::exit(status.code().unwrap_or(1));
    }
}