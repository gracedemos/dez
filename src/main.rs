mod utils;
mod dezfile;

use std::{
    env,
    process::{self, Command},
    fs
};
use colored::Colorize;
use dezfile::Dezfile;

fn main() {
    let mut args = env::args();
    let _ = args.next()
        .unwrap();
    let operation = args.next()
        .unwrap_or_else(|| {
            println!("{}", utils::DEZ_SPLASH.magenta());
            println!("Usage:");
            println!("    dez <operation>");
            println!("    <operation>: build, run");

            process::exit(1);
        });

    let mut run_args = Vec::new();
    while let Some(arg) = args.next() {
        run_args.push(arg);
    }

    let output = Command::new("pwd")
        .output()
        .unwrap();
    let mut current_path = String::from_utf8(output.stdout)
        .unwrap();
    current_path.pop();

    let dezfile_path = current_path + "/.dez";
    let dezfile_data = fs::read_to_string(dezfile_path);
    let dezfile = Dezfile::new(dezfile_data);
    let dezfile = if let Ok(dezfile) = dezfile {
        println!("{} Dezfile found", "[dez]".green());
        dezfile
    } else {
        Dezfile::default()
    };

    match operation.as_str() {
        "build" => utils::build(&dezfile),
        "run" => utils::run(&dezfile, run_args),
        _ => ()
    }
}
