use std::fs;
use std::path::PathBuf;
use std::process::Command;
use menu::term;

fn main() {
    let songs = get_songs();
    let lines: Vec<&str> = songs.split("\n").collect();
    let choice = match menu::ask(lines) {
        Err(why) => term!(why),
        Ok(c) => c
    };
    play(&choice);
}

fn get_songs() -> String {
    let p = get_path();
    let bytes = match fs::read(&p) {
        Err(why) => panic!("Error reading file {}: {}", &p, why),
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

fn get_path() -> String {
    let conf = menu::get_conf_dir();
    let path: PathBuf = [ &conf, "mpd", "songs.txt" ].iter().collect();
    path.to_str().unwrap().to_string()
}
