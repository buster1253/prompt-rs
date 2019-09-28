// read the .git/HEAD file and remove ref: refs/heads/ leaving only the branch
// if ref: is not present then assume it's a SHA-1 of the commit
// find the refs/* in packed-refs with the same hash
// if it's refs/tags/ then prepend a symbol to tell it's a tag

use std::process::{Command, Stdio};

struct PromptPart2 {
    fg: String,
    //bg: String,
    //inverse: String,
    //text: Option<String>
}

trait Prompt {
    fn get(&self) -> Option<String>;
}

impl Prompt for PromptPart2 {
    fn new(&self) -> PromptPart {
        return PromptPart{fg=String::from(""), bg=}
    }
    fn get(&self) -> Option<String> {
        Some(String::from("abc"))
    }
}

//struct Sheep { naked: bool, name: &'static str }

//trait Animal {
    //// Static method signature; `Self` refers to the implementor type.
    //fn new(name: &'static str) -> Self;
    //// Instance method signatures; these will return a string.
    //fn name(&self) -> &'static str;
    //fn noise(&self) -> &'static str;
    //// Traits can provide default method definitions.
    //fn talk(&self) {
        //println!("{} says {}", self.name(), self.noise());
    //}
//}



//impl Sheep {
    //fn is_naked(&self) -> bool {
        //self.naked
    //}
    //fn shear(&mut self) {
        //if self.is_naked() {
            //// Implementor methods can use the implementor's trait methods.
            //println!("{} is already naked...", self.name());
        //} else {
            //println!("{} gets a haircut!", self.name);
            //self.naked = true;
        //}
    //}
//}

//// Implement the `Animal` trait for `Sheep`.
//impl Animal for Sheep {
    //// `Self` is the implementor type: `Sheep`.
    //fn new(name: &'static str) -> Sheep {
        //Sheep { name: name, naked: false }
    //}
    //fn name(&self) -> &'static str {
        //self.name
    //}
    //fn noise(&self) -> &'static str {
        //if self.is_naked() {
            //"baaaaah?"
        //} else {
            //"baaaaah!"
        //}
    //}
    //// Default trait methods can be overridden.
    //fn talk(&self) {
        //// For example, we can add some quiet contemplation.
        //println!("{} pauses briefly... {}", self.name, self.noise());
    //}
//}


//fn main() {
    //// Type annotation is necessary in this case.
    //let mut dolly: Sheep = Animal::new("Dolly");
    //// TODO ^ Try removing the type annotations.
    //dolly.talk();
    //dolly.shear();
    //dolly.talk();
//}

pub fn new() -> Prompt {
    
}

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
