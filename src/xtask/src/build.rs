use crate::command;
use vors_filesystem::{self as vfs, Layout};
use std::{
    fmt::{self, Display, Formatter},
    fs,
};
use xshell::{cmd, Shell};

#[derive(Clone, Copy)]
pub enum Profile {
    Debug,
    Release,
    Distribution,
}

impl Display for Profile {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let string = match self {
            Profile::Distribution => "distribution",
            Profile::Release => "release",
            Profile::Debug => "debug",
        };
        write!(f, "{string}")
    }
}

pub fn build_server(
    profile: Profile,
    root: Option<String>,
    reproducible: bool,
    experiments: bool,
    keep_config: bool,
) {
    let sh = Shell::new().unwrap();

    let build_layout = Layout::new(&vfs::server_build_dir());

    let mut common_flags = vec![];
    match profile {
        Profile::Distribution => {
            common_flags.push("--profile");
            common_flags.push("distribution");
        }
        Profile::Release => common_flags.push("--release"),
        Profile::Debug => (),
    }
    if reproducible {
        common_flags.push("--locked");
    }
    let common_flags_ref = &common_flags;

    let artifacts_dir = vfs::target_dir().join(profile.to_string());

    let maybe_config = if keep_config {
        fs::read_to_string(build_layout.session()).ok()
    } else {
        None
    };

    sh.remove_path(&vfs::server_build_dir()).unwrap();
    sh.create_dir(&vfs::server_build_dir()).unwrap();
    sh.create_dir(&build_layout.resources_lib_dir())
        .unwrap();
    sh.create_dir(&build_layout.executables_dir).unwrap();

    if let Some(config) = maybe_config {
        fs::write(build_layout.session(), config).ok();
    }

    if let Some(root) = root {
        sh.set_var("VORS_ROOT_DIR", root);
    }

    // build server
    {
        let _push_guard = sh.push_dir(vfs::crate_dir("server"));
        cmd!(sh, "cargo build {common_flags_ref...}")
            .run()
            .unwrap();

        sh.copy_file(
            artifacts_dir.join(vfs::dynlib_fname("vors_server")),
            build_layout.resources_lib(),
        )
        .unwrap();

        if cfg!(windows) {
            sh.copy_file(
                artifacts_dir.join("vors_server.pdb"),
                build_layout
                    .resources_lib_dir()
                    .join("driver_vors_server.pdb"),
            )
            .unwrap();
        }
    }

    // build launcher
    {
        let _push_guard = sh.push_dir(vfs::crate_dir("launcher"));
        cmd!(sh, "cargo build {common_flags_ref...}").run().unwrap();

        sh.copy_file(
            artifacts_dir.join(vfs::exec_fname("vors_launcher")),
            build_layout.launcher_exe(),
        )
        .unwrap();
    }

    // Build dashboard
    {
        let _push_guard = sh.push_dir(vfs::crate_dir("dashboard"));
        cmd!(sh, "cargo build {common_flags_ref...}").run().unwrap();

        sh.copy_file(
            artifacts_dir.join(vfs::exec_fname("vors_dashboard")),
            build_layout.dashboard_exe(),
        )
        .unwrap();
    }

    // copy dependencies
    if cfg!(windows) {
        command::copy_recursive(
            &sh,
            &vfs::crate_dir("server").join("cpp/bin/windows"),
            &build_layout.resources_lib_dir(),
        )
        .unwrap();
    }

    // copy static resources
    {
        // copy dashboard
        command::copy_recursive(
            &sh,
            &vfs::workspace_dir().join("dashboard"),
            &build_layout.dashboard_dir(),
        )
        .unwrap();

        // copy presets
        command::copy_recursive(
            &sh,
            &vfs::crate_dir("xtask").join("resources/presets"),
            &build_layout.presets_dir(),
        )
        .ok();
    }

    // build experiments
    if experiments {
        command::copy_recursive(
            &sh,
            &vfs::workspace_dir().join("experiments/gui/resources/languages"),
            &build_layout.static_resources_dir.join("languages"),
        )
        .unwrap();

        let _push_guard = sh.push_dir(vfs::workspace_dir().join("experiments/launcher"));
        cmd!(sh, "cargo build {common_flags_ref...}").run().unwrap();
        sh.copy_file(
            artifacts_dir.join(vfs::exec_fname("launcher")),
            build_layout
                .executables_dir
                .join(vfs::exec_fname("experimental_launcher")),
        )
        .unwrap();
    }
}

pub fn build_client(
    profile: Profile,
    root: Option<String>,
    reproducible: bool,
    experiments: bool,
    keep_config: bool,
) {
    let sh = Shell::new().unwrap();

    let build_layout = Layout::new(&vfs::server_build_dir());

    let mut common_flags = vec![];
    match profile {
        Profile::Distribution => {
            common_flags.push("--profile");
            common_flags.push("distribution");
        }
        Profile::Release => common_flags.push("--release"),
        Profile::Debug => (),
    }
    if reproducible {
        common_flags.push("--locked");
    }
    let common_flags_ref = &common_flags;

    let artifacts_dir = vfs::target_dir().join(profile.to_string());

    let maybe_config = if keep_config {
        fs::read_to_string(build_layout.session()).ok()
    } else {
        None
    };

    sh.remove_path(&vfs::server_build_dir()).unwrap();
    sh.create_dir(&vfs::server_build_dir()).unwrap();
    sh.create_dir(&build_layout.resources_lib_dir())
        .unwrap();
    sh.create_dir(&build_layout.executables_dir).unwrap();

    if let Some(config) = maybe_config {
        fs::write(build_layout.session(), config).ok();
    }

    if let Some(root) = root {
        sh.set_var("VORS_ROOT_DIR", root);
    }

    // build server
    {
        let _push_guard = sh.push_dir(vfs::crate_dir("server"));
        cmd!(sh, "cargo build {common_flags_ref...}")
            .run()
            .unwrap();

        sh.copy_file(
            artifacts_dir.join(vfs::dynlib_fname("vors_server")),
            build_layout.resources_lib(),
        )
        .unwrap();

        if cfg!(windows) {
            sh.copy_file(
                artifacts_dir.join("vors_server.pdb"),
                build_layout
                    .resources_lib_dir()
                    .join("driver_vors_server.pdb"),
            )
            .unwrap();
        }
    }

    // build launcher
    {
        let _push_guard = sh.push_dir(vfs::crate_dir("launcher"));
        cmd!(sh, "cargo build {common_flags_ref...}").run().unwrap();

        sh.copy_file(
            artifacts_dir.join(vfs::exec_fname("vors_launcher")),
            build_layout.launcher_exe(),
        )
        .unwrap();
    }

    // Build dashboard
    {
        let _push_guard = sh.push_dir(vfs::crate_dir("dashboard"));
        cmd!(sh, "cargo build {common_flags_ref...}").run().unwrap();

        sh.copy_file(
            artifacts_dir.join(vfs::exec_fname("vors_dashboard")),
            build_layout.dashboard_exe(),
        )
        .unwrap();
    }

    // copy dependencies
    if cfg!(windows) {
        command::copy_recursive(
            &sh,
            &vfs::crate_dir("server").join("cpp/bin/windows"),
            &build_layout.resources_lib_dir(),
        )
        .unwrap();
    }

    // copy static resources
    {
        // copy dashboard
        command::copy_recursive(
            &sh,
            &vfs::workspace_dir().join("dashboard"),
            &build_layout.dashboard_dir(),
        )
        .unwrap();

        // copy presets
        command::copy_recursive(
            &sh,
            &vfs::crate_dir("xtask").join("resources/presets"),
            &build_layout.presets_dir(),
        )
        .ok();
    }

    // build experiments
    if experiments {
        command::copy_recursive(
            &sh,
            &vfs::workspace_dir().join("experiments/gui/resources/languages"),
            &build_layout.static_resources_dir.join("languages"),
        )
        .unwrap();

        let _push_guard = sh.push_dir(vfs::workspace_dir().join("experiments/launcher"));
        cmd!(sh, "cargo build {common_flags_ref...}").run().unwrap();
        sh.copy_file(
            artifacts_dir.join(vfs::exec_fname("launcher")),
            build_layout
                .executables_dir
                .join(vfs::exec_fname("experimental_launcher")),
        )
        .unwrap();
    }
}

