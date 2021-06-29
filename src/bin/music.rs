use std::fs;
use std::process::Command;

use rand::{seq::SliceRandom, thread_rng};

const SONGS: &str = "/home/davide/.config/mpd/songs.txt";

fn main() {
    let songs = get_songs();
    let mut lines: Vec<&str> = songs.split("\n").collect();
    lines.shuffle(&mut thread_rng());
    let choice = match menu::ask(lines) {
        Err(why) => menu::terminate(why),
        Ok(c) => c
    };
    play(&choice);
}

fn get_songs() -> String {
    let bytes = match fs::read(SONGS) {
        Err(why) => panic!("Error reading file {}: {}", SONGS, why),
        Ok(s) => s
    };
    String::from_utf8(bytes).unwrap()
}

fn play(song: &str) {
    match Command::new("openfile")
        .args(&[song])
        .spawn() {
            Err(why) => panic!("Error spawning song: {}", why),
            Ok(_) => ()
        }
}
