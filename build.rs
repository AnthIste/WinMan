use std::process::Command;
use std::env;
use std::path::Path;

fn main() {
    // let out_dir = env::var("OUT_DIR").ok().expect("Environment variable OUT_DIR not set");

    // Command::new("windres").args(&["res/winman.rc",  "-o"])
    //                    .arg(&format!("{}/winman.rc.o", out_dir))
    //                    .status().expect("Command 'windres' failed");
    // Command::new("ar").args(&["crus", "libwinman_rc.a"])
    // 				  .arg(&format!("{}/hello_rc.o", out_dir))
    //                   .current_dir(&Path::new(&out_dir))
    //                   .status().expect("Command 'ar' failed");

    // println!("cargo:rustc-link-search=native={}", out_dir);
    // println!("cargo:rustc-link-lib=static=winman_rc");
}
