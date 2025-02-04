use std::{env, process::Command};

fn main() {
    let pwd = env::current_dir().expect("Expected to get a result from env::current_dir()!");

    let dst_str = pwd
        .to_str()
        .expect("env::current_dir() returned a result but could not be converted to a string");

    let mut src = pwd.clone();
    src.pop(); // Go up to parent
    src.push("assets"); // Add "assets" to path
    let src_str = src
        .to_str()
        .expect("env::current_dir() returned a result but could not be converted to a string");

    println!("src: {}", src_str);
    println!("dst: {}", dst_str);

    // Perform platform-specific logic
    if cfg!(windows) {
        // Use xcopy on Windows
        let status = Command::new("powershell")
            .arg("-Command")
            .arg(format!("cp -r -Force \"{}\" \"{}\"", &src_str, dst_str))
            .status()
            .expect("Failed to execute cp");

        if !status.success() {
            panic!("Failed to copy folder using cp -r");
        }
    } else {
        // Use cp on Unix-based systems (Linux, macOS)
        let status = Command::new("cp")
            .arg("-r")
            .arg(&src_str)
            .arg(dst_str)
            .status()
            .expect("Failed to execute cp");

        if !status.success() {
            panic!("Failed to copy folder using cp");
        }
    }

    println!("cargo:rerun-if-changed={}", src_str); // Invalidate build cache if source folder changes
}
