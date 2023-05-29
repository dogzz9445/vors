use crate::{
    build::{self, Profile},
    command, version,
};
use vors_build_filesystem as vfs;
use std::path::PathBuf;
use xshell::{cmd, Shell};

fn build_windows_server_installer() {
    let sh = Shell::new().unwrap();

    let wix_path = PathBuf::from(r"C:\Program Files (x86)\WiX Toolset v3.11\bin");
    let heat_cmd = wix_path.join("heat.exe");
    let candle_cmd = wix_path.join("candle.exe");
    let light_cmd = wix_path.join("light.exe");

    // Clear away build and prerelease version specifiers, MSI can have only dot-separated numbers.
    let mut version = version::version();
    if let Some(idx) = version.find('-') {
        version = version[..idx].to_owned();
    }
    if let Some(idx) = version.find('+') {
        version = version[..idx].to_owned();
    }

    let server_build_dir = vfs::server_build_dir();
    let wix_source_dir = vfs::crate_dir("xtask").join("wix");
    let wix_target_dir = vfs::target_dir().join("wix");
    let main_source = wix_source_dir.join("main.wxs");
    let main_object = wix_target_dir.join("main.wixobj");
    let harvested_source = wix_target_dir.join("harvested.wxs");
    let harvested_object = wix_target_dir.join("harvested.wixobj");
    let vors_msi = vfs::build_dir().join("vors_server_windows.msi");
    let bundle_source = wix_source_dir.join("bundle.wxs");
    let bundle_object = wix_target_dir.join("bundle.wixobj");
    let installer = vfs::build_dir().join(format!("VORS_Installer_v{version}.exe"));

    cmd!(sh, "{heat_cmd} dir {server_build_dir} -ag -sreg -srd -dr APPLICATIONFOLDER -cg BuildFiles -var var.BuildRoot -o {harvested_source}").run().unwrap();
    cmd!(sh, "{candle_cmd} -arch x64 -dBuildRoot={server_build_dir} -ext WixUtilExtension -dVersion={version} {main_source} {harvested_source} -o {wix_target_dir}\\").run().unwrap();
    cmd!(sh, "{light_cmd} {main_object} {harvested_object} -ext WixUIExtension -ext WixUtilExtension -o {vors_msi}").run().unwrap();
    cmd!(sh, "{candle_cmd} -arch x64 -dBuildRoot={server_build_dir} -ext WixUtilExtension -ext WixBalExtension {bundle_source} -o {wix_target_dir}\\").run().unwrap();
    cmd!(
        sh,
        "{light_cmd} {bundle_object} -ext WixUtilExtension -ext WixBalExtension -o {installer}"
    )
    .run()
    .unwrap();
}

fn build_windows_client_installer() {
    let sh = Shell::new().unwrap();

    let wix_path = PathBuf::from(r"C:\Program Files (x86)\WiX Toolset v3.11\bin");
    let heat_cmd = wix_path.join("heat.exe");
    let candle_cmd = wix_path.join("candle.exe");
    let light_cmd = wix_path.join("light.exe");

    // Clear away build and prerelease version specifiers, MSI can have only dot-separated numbers.
    let mut version = version::version();
    if let Some(idx) = version.find('-') {
        version = version[..idx].to_owned();
    }
    if let Some(idx) = version.find('+') {
        version = version[..idx].to_owned();
    }

    let client_build_dir = vfs::client_build_dir();
    let wix_source_dir = vfs::crate_dir("xtask").join("wix");
    let wix_target_dir = vfs::target_dir().join("wix");
    let main_source = wix_source_dir.join("main.wxs");
    let main_object = wix_target_dir.join("main.wixobj");
    let harvested_source = wix_target_dir.join("harvested.wxs");
    let harvested_object = wix_target_dir.join("harvested.wixobj");
    let vors_msi = vfs::build_dir().join("vors_client_windows.msi");
    let bundle_source = wix_source_dir.join("bundle.wxs");
    let bundle_object = wix_target_dir.join("bundle.wixobj");
    let installer = vfs::build_dir().join(format!("VORS_Installer_v{version}.exe"));

    cmd!(sh, "{heat_cmd} dir {client_build_dir} -ag -sreg -srd -dr APPLICATIONFOLDER -cg BuildFiles -var var.BuildRoot -o {harvested_source}").run().unwrap();
    cmd!(sh, "{candle_cmd} -arch x64 -dBuildRoot={client_build_dir} -ext WixUtilExtension -dVersion={version} {main_source} {harvested_source} -o {wix_target_dir}\\").run().unwrap();
    cmd!(sh, "{light_cmd} {main_object} {harvested_object} -ext WixUIExtension -ext WixUtilExtension -o {vors_msi}").run().unwrap();
    cmd!(sh, "{candle_cmd} -arch x64 -dBuildRoot={client_build_dir} -ext WixUtilExtension -ext WixBalExtension {bundle_source} -o {wix_target_dir}\\").run().unwrap();
    cmd!(
        sh,
        "{light_cmd} {bundle_object} -ext WixUtilExtension -ext WixBalExtension -o {installer}"
    )
    .run()
    .unwrap();
}

pub fn package_server(root: Option<String>, appimage: bool, zsync: bool) {
    let sh = Shell::new().unwrap();

    build::build_server(Profile::Distribution, root, true, false, false);

    // Add licenses
    let licenses_dir = vfs::server_build_dir().join("licenses");
    sh.create_dir(&licenses_dir).unwrap();
    sh.copy_file(
        vfs::workspace_dir().join("LICENSE"),
        licenses_dir.join("VORS.txt"),
    )
    .unwrap();
    sh.copy_file(
        vfs::crate_dir("server").join("LICENSE-Valve"),
        licenses_dir.join("Valve.txt"),
    )
    .unwrap();

    // Gather licenses with cargo about
    cmd!(sh, "cargo install cargo-about").run().unwrap();
    let licenses_template = vfs::crate_dir("xtask").join("licenses_template.hbs");
    let licenses_content = cmd!(sh, "cargo about generate {licenses_template}")
        .read()
        .unwrap();
    sh.write_file(licenses_dir.join("dependencies.html"), licenses_content)
        .unwrap();

    // Finally package everything
    if cfg!(windows) {
        command::zip(&sh, &vfs::server_build_dir()).unwrap();

        build_windows_server_installer();
    } else {
        command::targz(&sh, &vfs::server_build_dir()).unwrap();

        if appimage {
            package_server_appimage(true, zsync);
        }
    }
}

fn package_server_appimage(release: bool, update: bool) {
    let sh = Shell::new().unwrap();

    let appdir = &vfs::build_dir().join("VORS.AppDir");
    let bin = &vfs::build_dir().join("vors_server_linux");

    let icon = &vfs::workspace_dir().join("resources/vors.png");
    let desktop = &vfs::workspace_dir().join("packaging/freedesktop/vors.desktop");

    let linuxdeploy = vfs::build_dir().join("linuxdeploy-x86_64.AppImage");

    if !sh.path_exists(&linuxdeploy) {
        command::download(&sh, "https://github.com/linuxdeploy/linuxdeploy/releases/download/continuous/linuxdeploy-x86_64.AppImage", &linuxdeploy).ok();
    }
    cmd!(&sh, "chmod a+x {linuxdeploy}").run().ok();

    if sh.path_exists(appdir) {
        sh.remove_path(appdir).ok();
    }

    cmd!(&sh, "{linuxdeploy} --appdir={appdir}").run().ok();

    sh.cmd("sh")
        .arg("-c")
        .arg(format!(
            "cp -r {}/* {}/usr",
            bin.to_string_lossy(),
            appdir.to_string_lossy()
        ))
        .run()
        .ok();

    sh.set_var("ARCH", "x86_64");
    sh.set_var("OUTPUT", "VORS-x86_64.AppImage");

    if release {
        let version = version::version();
        sh.set_var("VERSION", &version);

        if update {
            let repo = if version.contains("nightly") {
                "VORS-nightly"
            } else {
                "VORS"
            };
            sh.set_var(
                "UPDATE_INFORMATION",
                format!("gh-releases-zsync|vors-org|{repo}|latest|VORS-x86_64.AppImage.zsync"),
            );
        }
    }

    sh.set_var("VERBOSE", "1");
    sh.set_var("NO_APPSTREAM", "1");
    // sh.set_var("APPIMAGE_COMP", "xz");

    sh.change_dir(vfs::build_dir());

    cmd!(&sh, "{linuxdeploy} --appdir={appdir} -i{icon} -d{desktop} --deploy-deps-only={appdir}/usr/lib64/vors/bin/linux64/driver_vors_server.so --deploy-deps-only={appdir}/usr/lib64/libvors_vulkan_layer.so --output appimage").run().unwrap();
}


pub fn package_client(root: Option<String>, appimage: bool, zsync: bool) {
    let sh = Shell::new().unwrap();

    build::build_client(Profile::Distribution, root, true, false, false);

    // Add licenses
    let licenses_dir = vfs::client_build_dir().join("licenses");
    sh.create_dir(&licenses_dir).unwrap();
    sh.copy_file(
        vfs::workspace_dir().join("LICENSE"),
        licenses_dir.join("VORS.txt"),
    )
    .unwrap();
    sh.copy_file(
        vfs::crate_dir("client").join("LICENSE-Valve"),
        licenses_dir.join("Valve.txt"),
    )
    .unwrap();

    // Gather licenses with cargo about
    cmd!(sh, "cargo install cargo-about").run().unwrap();
    let licenses_template = vfs::crate_dir("xtask").join("licenses_template.hbs");
    let licenses_content = cmd!(sh, "cargo about generate {licenses_template}")
        .read()
        .unwrap();
    sh.write_file(licenses_dir.join("dependencies.html"), licenses_content)
        .unwrap();

    // Finally package everything
    if cfg!(windows) {
        command::zip(&sh, &vfs::client_build_dir()).unwrap();

        build_windows_client_installer();
    } else {
        command::targz(&sh, &vfs::client_build_dir()).unwrap();

        if appimage {
            package_client_appimage(true, zsync);
        }
    }
}

fn package_client_appimage(release: bool, update: bool) {
    let sh = Shell::new().unwrap();

    let appdir = &vfs::build_dir().join("VORS.AppDir");
    let bin = &vfs::build_dir().join("vors_client_linux");

    let icon = &vfs::workspace_dir().join("resources/vors.png");
    let desktop = &vfs::workspace_dir().join("packaging/freedesktop/vors.desktop");

    let linuxdeploy = vfs::build_dir().join("linuxdeploy-x86_64.AppImage");

    if !sh.path_exists(&linuxdeploy) {
        command::download(&sh, "https://github.com/linuxdeploy/linuxdeploy/releases/download/continuous/linuxdeploy-x86_64.AppImage", &linuxdeploy).ok();
    }
    cmd!(&sh, "chmod a+x {linuxdeploy}").run().ok();

    if sh.path_exists(appdir) {
        sh.remove_path(appdir).ok();
    }

    cmd!(&sh, "{linuxdeploy} --appdir={appdir}").run().ok();

    sh.cmd("sh")
        .arg("-c")
        .arg(format!(
            "cp -r {}/* {}/usr",
            bin.to_string_lossy(),
            appdir.to_string_lossy()
        ))
        .run()
        .ok();

    sh.set_var("ARCH", "x86_64");
    sh.set_var("OUTPUT", "VORS-x86_64.AppImage");

    if release {
        let version = version::version();
        sh.set_var("VERSION", &version);

        if update {
            let repo = if version.contains("nightly") {
                "VORS-nightly"
            } else {
                "VORS"
            };
            sh.set_var(
                "UPDATE_INFORMATION",
                format!("gh-releases-zsync|vors-org|{repo}|latest|VORS-x86_64.AppImage.zsync"),
            );
        }
    }

    sh.set_var("VERBOSE", "1");
    sh.set_var("NO_APPSTREAM", "1");
    // sh.set_var("APPIMAGE_COMP", "xz");

    sh.change_dir(vfs::build_dir());

    cmd!(&sh, "{linuxdeploy} --appdir={appdir} -i{icon} -d{desktop} --deploy-deps-only={appdir}/usr/lib64/vors/bin/linux64/driver_vors_client.so --deploy-deps-only={appdir}/usr/lib64/libvors_vulkan_layer.so --output appimage").run().unwrap();
}
