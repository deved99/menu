// STD
use std::env::var;
use std::fs;
use std::process::Command;
// Local
use menu::{ask, Error, Result};

fn main() -> Result<()> {
    let path = var("PATH").expect("No $PATH?");
    let mut bins: Vec<String> = path
        .split(':')
        .flat_map(|s| {
            read_path(s).unwrap_or_else(|e| {
                eprintln!("Error reading {}: {}", path, e);
                Vec::new()
            })
        })
        .collect();
    bins.sort();
    bins.dedup();
    let c = ask(bins)?;
    Command::new("/bin/sh")
        .args(["-c", &c])
        .spawn()
        .map(|_| ())
        .map_err(Error::from)
}

fn read_path(path: &str) -> Result<Vec<String>> {
    let raw = fs::read_dir(path)?;
    let mut ret = Vec::new();
    for i_maybe in raw {
        let path = i_maybe?.path();
        let filename = path.file_name().and_then(|x| x.to_str());
        match filename {
            // Ignore files that start with a .
            Some(a) if a.starts_with('.') => (),
            Some(a) => ret.push(a.to_string()),
            None => (),
        }
    }
    Ok(ret)
}
