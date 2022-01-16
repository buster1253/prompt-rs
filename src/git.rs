use std::process::{Command, Stdio};

trait Prompt {
    fn get(&self) -> Option<String>;
    fn new() -> Git;
}

struct Git {
    fg: String,
    bg: String,
    inverse: String,
}

impl Prompt for Git {
    fn new() -> Git {
        Git {
            fg: String::from("abc"),
            bg: String::from("def"),
            inverse: String::from("abc"),
        }
    }
    fn get(&self) -> Option<String> {
        let cmd = Command::new("git")
            .args(&["rev-parse", "--abbrev-ref", "HEAD"])
            .stdout(Stdio::piped())
            .stderr(Stdio::null())
            .output()
            .expect("Failed getting git branch");

        if cmd.status.success() {
            match String::from_utf8(cmd.stdout) {
                Ok(out) => Some(out.replace("\n", "")),
                Err(e) => {
                    println!("Git convert from utf8 failed: {:?}", e);
                    None
                }
            };
        }
        None
    }
}

pub fn get_branch() -> Option<String> {
    let cmd = match Command::new("git")
        .args(&["rev-parse", "--abbrev-ref", "HEAD"])
        .stdout(Stdio::piped())
        .stderr(Stdio::null())
        .output()
        {
        Ok(output) => output,
        Err(err) => return None
    };

    if cmd.status.success() {
        match String::from_utf8(cmd.stdout) {
            Ok(output) => return Some(output.replace("\n", "")),
            Err(err) => {
                println!("Error getting git status: {}", err);
                return None
            }
        };
    }
    else {
        return None
    }
}
