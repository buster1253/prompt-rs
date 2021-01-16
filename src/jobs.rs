use std::process::{Command, Stdio};

pub fn get() -> Option<String> {
    let cmd = Command::new("ps")
        .stdout(Stdio::piped())
        .stderr(Stdio::null())
        .output()
        .expect("Failed to run 'ps'");

    let num_jobs = if cmd.status.success() {
        match String::from_utf8(cmd.stdout) {
            Ok(out) => {
                let jobs = out.lines().into_iter().count();
                if jobs < 4 {
                    0
                } else {
                    jobs - 4
                }
            },
            Err(_) => 0
        }
    } else {
        0
    };

    if num_jobs > 0 {
        Some(num_jobs.to_string())
    } else {
        None
    }
}
