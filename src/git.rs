use std::process::{Command, Stdio};

struct PromptPart2 {
    fg: String,
    //bg: String,
    //inverse: String,
    //text: Option<String>
}

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
            Git{fg: String::from("abc"),
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

//impl Prompt for PromptPart2 {
    //fn new(&self) -> PromptPart {
        //return PromptPart{fg=String::from(""), bg=}
    //}
    //fn get(&self) -> Option<String> {
        //Some(String::from("abc"))
    //}
//}
//pub fn new(fg: String, bg: String, inverted: String) -> Prompt {

//}

pub fn get_branch() -> Option<String> {
    let cmd = Command::new("git")
        .args(&["rev-parse", "--abbrev-ref", "HEAD"])
        .stdout(Stdio::piped())
        .stderr(Stdio::null())
        .output()
        .expect("shit");

    if cmd.status.success() {
        let mut output = String::from_utf8(cmd.stdout).unwrap();
        output = output.replace("\n", "");
        return Some(output);
        //return Some(String::from_utf8(cmd.stdout.pop().unwrap()).unwrap());
    }
    else {
        return None
    }
}
