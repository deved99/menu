use std::env::{set_current_dir,var};
use std::fs;
use std::process::Command;

use menu::{ask,term};

fn main() {
    set_current_dir( var("HOME").unwrap() )
        .unwrap();
    ask_files();
}

fn ask_files() {
    let c = ask( &get_files() ).unwrap();
    match set_current_dir(&c) {
        Err(_) => open(&c),
        Ok(_) => ask_files()
    }
}

fn get_files() -> Vec<String> {
    let mut v = Vec::new();
    v.push("..".to_string());
    // list files
    let files = match fs::read_dir(".") {
        Err(why) => term!(why),
        Ok(i) => i
    };
    for f in files {
        let f = f.unwrap();
        let s = f.path()
            .file_name().unwrap()
            .to_str().unwrap()
            .to_string();
        if !s.starts_with(".") {
            v.push(s)
        }
    }
    v.sort();
    v
}

fn open(c: &str) {
    Command::new("openfile")
        .arg(c)
        .spawn()
        .unwrap();
}
