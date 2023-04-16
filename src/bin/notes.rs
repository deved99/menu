use std::fs;
use std::path::Path;
use std::process::Command;

use uuid::Uuid;

use menu::{ask, get_home, Error, Result};

fn main() -> Result<()> {
    let notes_dir = get_notes_dir();
    let mut notes = read_path(&notes_dir)?;
    notes.sort_by(|a, b| {
        let a = a.split_once(' ').map(|(_, x)| x).unwrap_or(&a);
        let b = b.split_once(' ').map(|(_, x)| x).unwrap_or(&b);
        a.cmp(b)
    });
    let mut note = ask(&notes)?;
    if !notes.contains(&note) {
        note = format!("{} {}.md", get_id(), note);
    }
    let fullpath_note = format!("{}/{}", &notes_dir, note);
    open(&fullpath_note)
}

fn open(path: &str) -> Result<()> {
    Command::new("alacritty")
        .args(&["-e", "nvim", path])
        .spawn()
        .map(|_| ())
        .map_err(Error::from)
}

fn read_path(path: impl AsRef<Path>) -> Result<Vec<String>> {
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

fn get_id() -> String {
    let mut u = Uuid::new_v4().to_string();
    u.truncate(8);
    u
}

fn get_notes_dir() -> String {
    format!("{}/Notes", get_home())
}
