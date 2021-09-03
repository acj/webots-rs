use std::env;
use std::path::PathBuf;

static WEBOTS_LINUX_PATH: &'static str = "/usr/local/webots";
static WEBOTS_MACOS_PATH: &'static str = "/Applications/Webots.app";
static WEBOTS_WINDOWS_PATH: &'static str = "C:\\Program Files\\Webots";

fn main() {
    let env_path = env::var("WEBOTS_PATH").ok();
    let webots_path = if cfg!(target_os = "linux") {
        WEBOTS_LINUX_PATH.to_string()
    } else if cfg!(target_os = "macos") {
        WEBOTS_MACOS_PATH.to_string()
    } else if let Some(path) = env_path {
        path
    } else if cfg!(target_os = "windows") {
        WEBOTS_WINDOWS_PATH.to_string()
    } else {
        panic!(
            "Unrecognized OS. Please set WEBOTS_PATH so that we can find your Webots installation."
        );
    };
    let lib_path = PathBuf::from(&webots_path).join("lib/controller");
    let include_path = PathBuf::from(&webots_path).join("include/controller/c");

    println!("cargo:rustc-link-search={}", lib_path.display());
    println!("cargo:rustc-link-lib=Controller");
    println!("cargo:rustc-env=LD_LIBRARY_PATH={}", lib_path.display());
    println!("cargo:rerun-if-changed=wrapper.h");

    let bindings = bindgen::Builder::default()
        .header("wrapper.h")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .clang_args(vec!["-I", include_path.to_str().unwrap()])
        .blocklist_item("FP_INFINITE")
        .blocklist_item("FP_NAN")
        .blocklist_item("FP_NORMAL")
        .blocklist_item("FP_SUBNORMAL")
        .blocklist_item("FP_ZERO")
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from("src");
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Unable to write bindings");
}
