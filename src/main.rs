use std::process::{Command, Stdio};
use structopt::StructOpt;

mod git;
mod jobs;
mod path;
mod sshfs;

#[derive(StructOpt)]
struct Opt {
    #[structopt(short, long)]
    jobs: u32,
}

pub struct PromptPart<'a> {
    fg: &'a str,
    text: Option<String>,
}

fn main() {
    let opt = Opt::from_args();
    let jobs_count = if opt.jobs == 0 {
        None
    } else {
        Some(opt.jobs.to_string())
    };

    let path_parser = path::PathParser {
        parsers: vec![&path::PathParser::git, &path::PathParser::default],
    };

    let esc = String::from("\x1B[");
    let fg = esc.clone() + "3";

    let black = "0";
    let red = "1";
    let green = "2";
    let orange = "3";
    let blue = "4";
    let magenta = "5";
    let cyan = "6";
    let white = "7";

    let fg_red = fg.clone() + red + "m";
    let fg_blue = fg.clone() + blue + "m";
    let fg_cyan = fg.clone() + cyan + "m";
    let fg_black = fg.clone() + black + "m";
    let fg_white = fg.clone() + white + "m";
    let fg_green = fg.clone() + green + "m";
    let fg_orange = fg.clone() + orange + "m";
    let fg_magenta = fg.clone() + magenta + "m";

    let path_part = PromptPart {
        fg: &fg_cyan,
        text: path_parser.path(2),
    };

    let jobs = PromptPart {
        fg: &fg_blue,
        text: jobs_count,
    };
    let git = PromptPart {
        fg: &fg_magenta,
        text: git::get_branch(),
    };

    let sshfs = PromptPart {
        fg: &fg_white,
        text: sshfs::check_path(),
    };

    vec![jobs, path_part, git, sshfs]
        .iter()
        .filter(|p| p.text.is_some())
        .for_each(|p| print!("%{{{}%}}{} ", p.fg, p.text.as_ref().unwrap()));
    print!("%{{\x1B[0m%}}: ");
}
