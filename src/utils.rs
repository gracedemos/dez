use std::{
    process::Command,
    fs
};
use regex::Regex;
use crate::dezfile::Dezfile;

pub const DEZ_SPLASH: &str = "
         ___           ___           ___     
        /\\  \\         /\\  \\         /\\  \\    
       /::\\  \\       /::\\  \\        \\:\\  \\   
      /:/\\:\\  \\     /:/\\:\\  \\        \\:\\  \\  
     /:/  \\:\\__\\   /::\\~\\:\\  \\        \\:\\  \\ 
    /:/__/ \\:|__| /:/\\:\\ \\:\\__\\ _______\\:\\__\\
    \\:\\  \\ /:/  / \\:\\~\\:\\ \\/__/ \\::::::::/__/
     \\:\\  /:/  /   \\:\\ \\:\\__\\    \\:\\~~\\~~    
      \\:\\/:/  /     \\:\\ \\/__/     \\:\\  \\     
       \\::/__/       \\:\\__\\        \\:\\__\\    
        ~~            \\/__/         \\/__/    
";

pub fn build(dezfile: Dezfile) {
    println!("[dez] Buliding");

    let build_path = get_build_path();
    
    Command::new("cmake")
        .args([".", "-Bbuild", "-G Ninja"])
        .args(dezfile.cmake_args)
        .spawn()
        .unwrap()
        .wait()
        .expect("CMake failed");

    Command::new("ninja")
        .args(["-C", &build_path])
        .spawn()
        .unwrap()
        .wait()
        .expect("Ninja failed");

    println!("[dez] Done");
}

pub fn run(dezfile: Dezfile) {
    build(dezfile);
    println!("[dez] Running");

    let project_path_output = Command::new("pwd")
        .output()
        .unwrap();
    let mut project_path = String::from_utf8(project_path_output.stdout)
        .unwrap();
    project_path.pop();
    
    let exec_name_regex = Regex::new(r".*/(.+)$")
        .unwrap();
    let exec_name = exec_name_regex.captures(&project_path)
        .unwrap()
        .get(1)
        .unwrap()
        .as_str();
    let exec_path = get_build_path() + "/" + exec_name;

    Command::new(exec_path)
        .spawn()
        .unwrap()
        .wait()
        .unwrap();

    println!("[dez] Done");
}

fn get_build_path() -> String {
    let build_path_output = Command::new("pwd")
        .output()
        .unwrap();
    let mut build_path = String::from_utf8(build_path_output.stdout)
        .unwrap();
    build_path.pop();
    build_path += "/build";

    return build_path;
}
