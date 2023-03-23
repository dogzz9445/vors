use once_cell::sync::Lazy;
use std::{
    env::{
        self,
        consts::{DLL_EXTENSION, DLL_PREFIX, DLL_SUFFIX, EXE_SUFFIX, OS},
    },
    path::{Path, PathBuf},
};

pub fn exec_fname(name: &str) -> String {
    format!("{name}{EXE_SUFFIX}")
}

pub fn dynlib_fname(name: &str) -> String {
    format!("{DLL_PREFIX}{name}{DLL_SUFFIX}")
}

pub fn target_dir() -> PathBuf {
    // use `.parent().unwrap()` instead of `../` to maintain canonicalized form
    Path::new(env!("OUT_DIR"))
        .parent()
        .unwrap()
        .parent()
        .unwrap()
        .parent()
        .unwrap()
        .parent()
        .unwrap()
        .to_owned()
}

pub fn workspace_dir() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .parent()
        .unwrap()
        .to_owned()
}

pub fn crate_dir(name: &str) -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .join(name)
}

pub fn deps_dir() -> PathBuf {
    workspace_dir().join("deps")
}

pub fn build_dir() -> PathBuf {
    workspace_dir().join("build")
}

pub fn server_build_dir() -> PathBuf {
    build_dir().join(format!("vors_server_{OS}"))
}

pub fn client_build_dir() -> PathBuf {
    build_dir().join(format!("vors_client_{OS}"))
}

pub fn installer_path() -> PathBuf {
    env::temp_dir().join(exec_fname("vors_installer"))
}

// Layout of the VORS installation. All paths are absolute
#[derive(Clone)]
pub struct Layout {
    // directory containing the launcher executable
    pub executables_dir: PathBuf,
    // (linux only) directory where vors_vulkan_layer.so is saved
    pub libraries_dir: PathBuf,
    // parent directory of resources like the dashboard and presets folders
    pub static_resources_dir: PathBuf,
    // directory for storing configuration files (session.json)
    pub config_dir: PathBuf,
    // directory for storing log
    pub log_dir: PathBuf,
    // directory for resources
    pub resources_root_dir: PathBuf,
}

impl Layout {
    pub fn new(root: &Path) -> Self {
        if cfg!(target_os = "linux") {
            // Get paths from environment or use FHS compliant paths
            let executables_dir = if !env!("executables_dir").is_empty() {
                PathBuf::from(env!("executables_dir"))
            } else {
                root.join("bin")
            };
            let libraries_dir = if !env!("libraries_dir").is_empty() {
                PathBuf::from(env!("libraries_dir"))
            } else {
                root.join("lib64")
            };
            let static_resources_dir = if !env!("static_resources_dir").is_empty() {
                PathBuf::from(env!("static_resources_dir"))
            } else {
                root.join("share/vors")
            };
            let config_dir = if !env!("config_dir").is_empty() {
                PathBuf::from(env!("config_dir"))
            } else {
                dirs::config_dir().unwrap().join("vors")
            };
            let log_dir = if !env!("log_dir").is_empty() {
                PathBuf::from(env!("log_dir"))
            } else {
                dirs::home_dir().unwrap()
            };
            let resources_root_dir = if !env!("resources_root_dir").is_empty() {
                PathBuf::from(env!("resources_root_dir"))
            } else {
                root.join("lib64/vors")
            };

            Self {
                executables_dir,
                libraries_dir,
                static_resources_dir,
                config_dir,
                log_dir,
                resources_root_dir,
            }
        } else {
            Self {
                executables_dir: root.to_owned(),
                libraries_dir: root.to_owned(),
                static_resources_dir: root.to_owned(),
                config_dir: root.to_owned(),
                log_dir: root.to_owned(),
                resources_root_dir: root.to_owned(),
            }
        }
    }

    pub fn launcher_exe(&self) -> PathBuf {
        let exe = if cfg!(windows) {
            "VORS Launcher.exe"
        } else {
            "vors_launcher"
        };
        self.executables_dir.join(exe)
    }

    pub fn dashboard_exe(&self) -> PathBuf {
        let exe = if cfg!(windows) {
            "VORS Dashboard.exe"
        } else {
            "vors_dashboard"
        };
        self.executables_dir.join(exe)
    }

    pub fn dashboard_dir(&self) -> PathBuf {
        self.static_resources_dir.join("dashboard")
    }

    pub fn presets_dir(&self) -> PathBuf {
        self.static_resources_dir.join("presets")
    }

    pub fn session(&self) -> PathBuf {
        self.config_dir.join("session.json")
    }

    pub fn session_log(&self) -> PathBuf {
        if cfg!(target_os = "linux") {
            self.log_dir.join("vors_session_log.txt")
        } else {
            self.log_dir.join("session_log.txt")
        }
    }

    pub fn crash_log(&self) -> PathBuf {
        self.log_dir.join("crash_log.txt")
    }

    pub fn resources_lib_dir(&self) -> PathBuf {
        let platform = if cfg!(windows) {
            "win64"
        } else if cfg!(target_os = "linux") {
            "linux64"
        } else if cfg!(target_os = "macos") {
            "macos"
        } else {
            unimplemented!()
        };

        self.resources_root_dir.join("bin").join(platform)
    }

    
    // path to the shared library to be loaded by openVR
    pub fn resources_lib(&self) -> PathBuf {
        self.resources_lib_dir()
            .join(format!("driver_vors_server.{DLL_EXTENSION}"))
    }
}

static LAYOUT_FROM_ENV: Lazy<Option<Layout>> =
    Lazy::new(|| (!env!("root").is_empty()).then(|| Layout::new(Path::new(env!("root")))));

// The path should include the executable file name
// The path argument is used only if VORS is built as portable
pub fn filesystem_layout_from_launcher_exe(path: &Path) -> Layout {
    LAYOUT_FROM_ENV.clone().unwrap_or_else(|| {
        let root = if cfg!(target_os = "linux") {
            // FHS path is expected
            path.parent().unwrap().parent().unwrap().to_owned()
        } else {
            path.parent().unwrap().to_owned()
        };

        Layout::new(&root)
    })
}

// The dir argument is used only if VORS is built as portable
pub fn filesystem_layout_from_openvr_driver_root_dir(dir: &Path) -> Layout {
    LAYOUT_FROM_ENV.clone().unwrap_or_else(|| {
        let root = if cfg!(target_os = "linux") {
            // FHS path is expected
            dir.parent().unwrap().parent().unwrap().to_owned()
        } else {
            dir.to_owned()
        };

        Layout::new(&root)
    })
}

// Use this when there is no way of determining the current path. The resulting Layout paths will
// be invalid, except for the ones that disregard the relative path (for example the config dir) and
// the ones that have been overridden.
pub fn filesystem_layout_invalid() -> Layout {
    LAYOUT_FROM_ENV
        .clone()
        .unwrap_or_else(|| Layout::new(Path::new("")))
}
