use std::{
    fs::{set_permissions, File}, io::Write, os::unix::fs::PermissionsExt, path::PathBuf
};
use std::process::Command as SysCommand;
use clap::{arg, Command};

fn main() {
    const FILE_SUFFIX: &str = ".service";
    const FILE_PREFIX: &str = "/lib/systemd/system/";
    let matches = Command::new("Systemd Creator")
        .version("1.0")
        .about("Quickly create a systemd in linux")
        .arg(arg!(-n  <NAME_VALUE> "Set the name of service").required(true))
        .arg(
            arg!(-e  <EXEC_VALUE> "Set the executable file of service")
                .required(true)
                .value_parser(clap::value_parser!(PathBuf)),
        )
        .get_matches();
    let name = matches.get_one::<String>("NAME_VALUE").expect("requires");
    let exec = matches.get_one::<PathBuf>("EXEC_VALUE").expect("requires");
    let parent = exec.parent().expect("No parent directory").to_path_buf();

    let exec_binding = exec.canonicalize().expect("Failed to get absolute path");
    let parent_binding = parent.canonicalize().expect("Failed to get absolute path");
    let absolute_exec = exec_binding
        .as_path()
        .to_str()
        .expect("Path is not valid UTF-8");
    let absolute_parent = parent_binding
        .as_path()
        .to_str()
        .expect("Path is not valid UTF-8");
    let service_file = FILE_PREFIX.to_owned() + &name + FILE_SUFFIX;
    let mut file = File::create(&service_file).expect("Failed to create file");
    set_permissions(&service_file, std::fs::Permissions::from_mode(0o777))
        .expect("Failed to set permissions for the desktop file");
    set_permissions(exec, std::fs::Permissions::from_mode(0o777)).expect("Failed to set permissions for the executable");
    let content = format!(
        "[Unit]\nAfter=network-online.target\n\n[Service]\nType=simple\nWorkingDirectory={}\nExecStart={}\nExecStartPre=/bin/sleep 10\nRestart=on-abort\nUser=root\n[Install]\nWantedBy=multi-user.target\n",
        absolute_parent, absolute_exec
    );
    file.write_all(content.as_bytes()).expect("Fail to write to file");
    SysCommand::new("systemctl")
        .args(["enable", name])
        .output()
        .expect("Failed to execute systemctl enable");
    SysCommand::new("systemctl")
        .args(["start", name])
        .output()
        .expect("Failed to execute systemctl start");

}
