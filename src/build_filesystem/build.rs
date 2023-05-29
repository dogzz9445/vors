use std::env;

fn main() {
    // Convert build environment variables into compile-time variables

    // Used only for Linux. These paths should be absolute and override the installation type
    println!(
        "cargo:rustc-env=executables_dir={}",
        env::var("VORS_EXECUTABLES_DIR").unwrap_or_else(|_| "".to_owned())
    );
    println!(
        "cargo:rustc-env=libraries_dir={}",
        env::var("VORS_LIBRARIES_DIR").unwrap_or_else(|_| "".to_owned())
    );
    println!(
        "cargo:rustc-env=static_resources_dir={}",
        env::var("VORS_STATIC_RESOURCES_DIR").unwrap_or_else(|_| "".to_owned())
    );
    println!(
        "cargo:rustc-env=config_dir={}",
        env::var("VORS_CONFIG_DIR").unwrap_or_else(|_| "".to_owned())
    );
    println!(
        "cargo:rustc-env=log_dir={}",
        env::var("VORS_LOG_DIR").unwrap_or_else(|_| "".to_owned())
    );
    println!(
        "cargo:rustc-env=resources_root_dir={}",
        env::var("VORS_RESOURCES_ROOT_DIR").unwrap_or_else(|_| "".to_owned())
    );

    println!(
        "cargo:rustc-env=root={}",
        env::var("VORS_ROOT_DIR").unwrap_or_else(|_| "".to_owned())
    );
}
