use std::collections::HashMap;
// to find ~/.config
use std::env::var;
// for reading dir and filename manipulation
use std::fs;
use std::path::PathBuf;
// to spawn a new process
use std::process::Command;

use menu::{ask,terminate};

use itertools::Itertools;
use inflector::Inflector;

fn main() {
    let configs_dir = get_conf_dir();
    let files = list_dir(&configs_dir);
    let map: HashMap<String,&PathBuf> = files.iter()
        .map(|f| (pretty(&f), f))
        .collect();
    let choice_raw = match ask(map.keys().sorted()) {
        Err(why) => terminate(why),
        Ok(o) => o
    };
    let choice = match map.get(choice_raw.as_str()) {
        None => {
            println!("Choice invalid: {:?}", choice_raw);
            return
        },
        Some(i) => i.to_str().unwrap()
    };
    let cmd = Command::new(&choice).spawn();
    match cmd {
        Err(why) => panic!("Error running {:?}: {}", choice, why),
        Ok(_) => ()
    }
}

fn list_dir(dir: &str) -> Vec<PathBuf> {
    let files_raw = match fs::read_dir(dir) {
        Err(why) => panic!("Error reading {}: {}", dir, why),
        Ok(fs) => fs,
    };
    let mut files = Vec::new();
    for f in files_raw {
        let path = match &f {
            Err(why) => panic!("Error iterating over {:?}: {}", f, why),
            Ok(s) => s.path(),
        };
        files.push(path)
    }
    files
}

fn get_conf_dir() -> String {
    let foo = match var("XDG_CONFIG_DIR") {
        Err(_) => match var("HOME") {
            Err(_) => panic!("No home?"),
            Ok(i) => format!("{}/.config/dmenu", i)
        }
        Ok(i) => format!("{}/dmenu", i)
    };
    match fs::create_dir_all(&foo) {
        Err(why) => panic!("Couldn't create {}: {}", &foo, why),
        Ok(_) => ()
    };
    foo
}

fn pretty(path: &PathBuf) -> String {
    match path.file_stem() {
        None => terminate( format!("{:?}: no stem?", path) ),
        Some(i) => match i.to_str() {
            None => terminate(format!("{:?} has no filename?", path)),
            Some(s) => s.to_sentence_case(),
        }
    }
}
