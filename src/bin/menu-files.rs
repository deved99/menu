use std::env::set_current_dir;
use std::fs;
use std::process::Command;

use menu::{ask, get_home, Error, Result};

fn main() {
    set_current_dir(get_home()).expect("Failed to cd $HOME?");
    ask_files().unwrap();
}

fn ask_files() -> Result<()> {
    let files = get_files()?;
    let c = ask(files)?;
    match set_current_dir(&c) {
        Err(_) => open(&c),
        Ok(_) => ask_files(),
    }
}

fn get_files() -> Result<Vec<String>> {
    let mut v = vec![String::from("..")];
    // list files
    let files = fs::read_dir(".")?;
    for f in files {
        let f = f.unwrap();
        let s = f.path().file_name().unwrap().to_str().unwrap().to_string();
        if !s.starts_with('.') {
            v.push(s)
        }
    }
    v.sort();
    Ok(v)
}

fn open(c: &str) -> Result<()> {
    Command::new("openfile")
        .arg(c)
        .spawn()
        .map(|_| ())
        .map_err(Error::from)
}
