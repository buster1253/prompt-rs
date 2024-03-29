use std::env;
use std::process::{Command, Stdio};

pub fn check_path() -> Option<String> {
    let cmd = Command::new("df")
        .args(&["-tfuse.sshfs", "--output=target"])
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .output()
        .expect("sshfs failed running command");

    let pwd = match env::current_dir() {
        Ok(pwd) => pwd,
        Err(err) => panic!("Err getting pwd: {}", err),
    };

    if cmd.status.success() {
        let output = String::from_utf8(cmd.stdout).unwrap();
        let mut paths: Vec<&str> = output.split("\n").collect();
        paths.remove(0);
        paths.remove(paths.len() - 1);

        for path_str in paths {
            if pwd.starts_with(path_str) {
                let comps: Vec<&str> = path_str.split("/").collect();
                return Some(String::from(comps[comps.len() - 1]));
            }
        }
    }
    return None;
}
