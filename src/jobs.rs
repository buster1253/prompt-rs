use std::io::BufRead;
use std::os::unix::process::parent_id;
use std::process::{Command, Stdio};

pub fn get_jobs() -> Option<String> {
    let ppid = parent_id().to_string();
    let cmd = match Command::new("ps")
        .args(&["--ppid", ppid.as_str(), "-oppid="])
        .stdout(Stdio::piped())
        .output()
        {
            Ok(output) => output,
            Err(err) => {
                println!("Error getting jobs: {}", err);
                return None
            }
    };

    let lines = cmd.stdout.lines().count() - 1;
    if lines > 0 {
        Some(lines.to_string())
    }
    else {
        None
    }
}
