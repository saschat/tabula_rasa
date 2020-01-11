use std::process::Command;
use std::path::Path;

fn main() {
    Command::new("npm").args(&["run", "build"])
                      .current_dir(&Path::new("./dashboard"))
                      .status().unwrap();

    println!("cargo:rerun-if-changed=./dashboard/");
}
