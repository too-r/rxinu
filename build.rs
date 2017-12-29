// Found here: https://github.com/intermezzOS/kernel/blob/master/build.rs

use std::process::Command;
use std::env;
use std::path::Path;
use std::fs;

fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();

    let asm_dir = "src/arch/x86/asm";

    // Common assembly files
    let mut nasm_files = vec![
        "check-common",
        "multiboot_header",
        "sse",
        "startup-common",
    ];

    // Architecture specific files and flags
    let mut arch_files: Vec<&str> = Vec::new();
    let mut asm_flags: Vec<&str> = Vec::new();

    #[cfg(target_arch = "x86")]
    {
        let files32 = vec![
            "i686/paging",
            "i686/start",
        ];
        arch_files.extend(&files32);

        asm_flags.push("-felf32");

        fs::create_dir(format!("{}/i686", out_dir))?;
    }

    #[cfg(target_arch = "x86_64")]
    {
        let files64 = vec![
            "x86_64/check_long_mode",
            "x86_64/long_mode_start",
            "x86_64/paging",
            "x86_64/start",
        ];
        arch_files.extend(&files64);

        asm_flags.push("-felf64");
        fs::create_dir(format!("{}/x86_64", out_dir));
    }

    nasm_files.extend(&arch_files);

    for file in nasm_files.iter() {
        let file_path: String = format!("{}/{}.nasm", asm_dir, file);
        assert!(Command::new("nasm")
                    .arg(file_path.as_str())
                    .args(asm_flags.as_slice())
                    .arg("-o")
                    .arg(&format!("{}/{}.o", out_dir, file))
                    .status()
                    .expect("failed to execute nasm")
                    .success(),
                "compilation of boot.asm failed");

        println!("cargo:rerun-if-changed={}/{}", asm_dir, file_path);
    }

    let obj_files: Vec<String> = nasm_files.iter().map(|&s| format!("{}.o", s)).collect();
    assert!(Command::new("ar")
                .args(&["crus", "libboot.a"])
                .args(obj_files.as_slice())
                .current_dir(&Path::new(&out_dir))
                .status()
                .expect("failed to execute ar")
                .success(),
            "ar command failed");

    println!("cargo:rustc-link-search=native={}", out_dir);
    println!("cargo:rustc-link-lib=static=boot");
}
