use std::env;
use std::fs;
use std::path::PathBuf;

fn main() {
    let dll_name = "wcpopup_win_hook.dll";
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
    let dll_dest = out_dir.join(dll_name);

    let manifest_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
    let dll_src = manifest_dir.join("out").join(dll_name);

    let msg = format!("Failed to copy DLL:{}", out_dir.to_str().unwrap());
    fs::copy(&dll_src, &dll_dest).unwrap_or_else(|_| panic!("{}", msg));

    // Available as DEP_WCPOPUPHOOK_DLL
    println!("cargo::metadata=DLL={}", dll_dest.display())
}
