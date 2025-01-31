use std::{env, path::PathBuf};

const MGBA_VERSION: &str = "0.9.1";

fn main() {
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    let mgba_directory = out_path.join(format!("mgba-{}", MGBA_VERSION));
    let out = std::process::Command::new("bash")
        .arg("build-mgba.sh")
        .arg(MGBA_VERSION)
        .arg(&out_path)
        .output()
        .expect("should be able to build mgba");
    if !out.status.success() {
        panic!(
            "failed to build mgba!\n{}",
            String::from_utf8_lossy(&out.stderr),
        );
    }

    cc::Build::new()
        .file("c/test-runner.c")
        .include(&mgba_directory.join("include"))
        .static_flag(true)
        .debug(true)
        .compile("test-runner");

    println!("cargo:rustc-link-search={}", out_path.to_str().unwrap());
    println!("cargo:rustc-link-lib=static=mgba-cycle");
    println!("cargo:rustc-link-lib=elf");

    println!("cargo:rerun-if-changed=build-mgba.sh");
    println!("cargo:rerun-if-changed=add_cycles_register.patch");
}
