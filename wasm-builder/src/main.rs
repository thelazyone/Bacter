use std::process::Command;

fn main() {
    let status = Command::new("wasm-pack")
        .arg("build")
        .arg("./bacter-wasm")
        .arg("--target")
        .arg("web")
        .status()
        .expect("Failed to execute wasm-pack");

    if !status.success() {
        std::process::exit(status.code().unwrap_or(1));
    }
}