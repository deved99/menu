// STD
use std::env::var;
use std::fs;
use std::process::Command;
// Local
use menu::{term,ask};

fn main() {
    let path = match var("PATH") {
        Err(why) => panic!("Error getting $PATH: {}", why),
        Ok(v) => v
    };
    let mut bins = Vec::new();
    for p in path.split(":") {
        let foo = read_path(p);
        bins.extend(foo);
    }
    bins.sort();
    bins.dedup();
    let c = match ask(bins) {
        Err(why) => term!(why),
        Ok(c) => c,
    };
    Command::new("/bin/sh")
        .args(&[ "-c", &c ])
        .spawn()
        .unwrap();
}

fn read_path(path: &str) -> Vec<String> {
    let raw = match fs::read_dir(path) {
        Err(why) => {
            println!("Error getting files in folder {}: {}", path, why);
            return Vec::new()
        },
        Ok(l) => l
    };
    let mut ret = Vec::new();
    for i_opt in raw {
        let i = match &i_opt {
            Err(why) => panic!("Error iterating over {:?}: {}", i_opt, why),
            Ok(j) => {
                let path = j.path();
                let path_str = match path.file_stem() {
                    None => continue,
                    Some(s) => s.to_str().unwrap()
                };
                if path_str.starts_with(".") {
                    continue
                }
                path_str.to_string()
            }
        };
        ret.push(i)
    }
    ret
}
