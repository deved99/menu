use std::fs;
use std::process::Command;

use menu::{Error, Result};

fn main() -> Result<()> {
    let songs = get_songs()?;
    let lines: Vec<&str> = songs.split('\n').collect();
    let choice = menu::ask(lines)?;
    play(&choice)?;
    Ok(())
}

fn get_songs() -> Result<String> {
    let p = get_path();
    let bytes = fs::read(p)?;
    String::from_utf8(bytes)
        .map_err(Error::from)
}

fn play(song: &str) -> Result<()> {
    Command::new("openfile")
        .arg(song)
        .spawn()
        .map(|_| ())
        .map_err(Error::from)
}

fn get_path() -> String {
    let conf = menu::get_conf_dir();
    format!("{}/mpd/songs.txt", conf)
}
