use std::{
    error::Error,
    io
};
use regex::Regex;

#[derive(Default)]
pub struct Dezfile {
    pub cmake_args: Vec<String>,
    pub build_type: String
}

impl Dezfile {
    pub fn new(dezfile: io::Result<String>) -> Result<Dezfile, Box<dyn Error>> {
        if let Ok(dezfile) = dezfile {
            let mut cmake_args = Vec::new();

            let build_type_regex = Regex::new(r"build-type\s*=\s*(\w+)")
                .unwrap();
            let build_type = build_type_regex.captures(&dezfile)
                .unwrap()
                .get(1)
                .unwrap()
                .as_str();

            let cmake_args_regex = Regex::new(r"(-D\w+)")
                .unwrap();
            for (_, [arg]) in cmake_args_regex.captures_iter(&dezfile).map(|c| c.extract()) {
                cmake_args.push(String::from(arg));
            }

            Ok(Dezfile {
                cmake_args,
                build_type: String::from(build_type)
            })
        } else {
            Err("Dezfile not found".into())
        }
    }
}
