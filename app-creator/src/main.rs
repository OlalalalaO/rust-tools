use clap::{arg, Command};
use std::fs::{set_permissions, File};
use std::io::Write;
use std::os::unix::fs::PermissionsExt;
use std::path::PathBuf;
extern crate clap;
fn main() {
    const FILE_SUFFIX: &str = ".desktop";
    const FILE_PREFIX: &str = "/usr/share/applications/";
    let matches = Command::new("App Creator")
        .version("1.0")
        .about("Quickly create a app in linux")
        .arg(arg!(-n  <NAME_VALUE> "Set the name of app").required(true))
        .arg(arg!(-c  <COVER_VALUE> "Set the cover of app").required(true).value_parser(clap::value_parser!(PathBuf)))
        .arg(arg!(-e  <EXEC_VALUE> "Set the executable file of app").required(true).value_parser(clap::value_parser!(PathBuf)))
        .get_matches();
    let name = matches.get_one::<String>("NAME_VALUE").expect("requires");
    let cover = matches.get_one::<PathBuf>("COVER_VALUE").expect("requires");
    let exec = matches.get_one::<PathBuf>("EXEC_VALUE").expect("requires");
    let cover_binding = cover.canonicalize().expect("Failed to get absolute path");
    let absolute_cover = cover_binding.as_path().to_str().expect("Cover is not valid UTF-8");
    let exec_binding = exec.canonicalize().expect("Failed to get absolute path");
    let absolute_exec = exec_binding.as_path().to_str().expect("Path is not valid UTF-8");
    let desktop_file = FILE_PREFIX.to_owned() + &name + FILE_SUFFIX;
    let mut file = File::create(&desktop_file).expect("Failed to create file");
    set_permissions(&desktop_file, std::fs::Permissions::from_mode(0o777)).expect("Failed to set permissions for the desktop file");
    set_permissions(exec, std::fs::Permissions::from_mode(0o777)).expect("Failed to set permissions for the executable");
    let content = format!(
        "[Desktop Entry]\nName={}\nComment={}\nExec={}\nIcon={}\nTerminal=false\nType=Application\nCategories=Application;Development;",
        name, name, absolute_exec, absolute_cover
    );
    file.write_all(content.as_bytes()).expect("Failed to write to the file");
    println!("Successfully create {:?}",name);

}
