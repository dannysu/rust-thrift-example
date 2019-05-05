use std::env;
use std::process::Command;

fn main() {
    Command::new("docker")
        .arg("run")
        .arg("--rm")
        .arg("-v")
        .arg(format!("{}:/data", env::var("CARGO_MANIFEST_DIR").unwrap()))
        .arg("thrift")
        .arg("thrift")
        .arg("-o")
        .arg("/data/src")
        .arg("--gen")
        .arg("rs")
        .arg("/data/thrift/example_service.thrift")
        .output()
        .unwrap();
}
