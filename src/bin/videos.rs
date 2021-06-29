// Get cmd-line arguments
use std::env;
use std::collections::HashMap;
use std::fmt::Debug;
// File handling
use std::fs;
use std::io::Write;
use std::path::PathBuf;
// Exec commands, write/read stdin/stdout
use std::process::Command;

// JSON handling
use serde::{Serialize, Deserialize};
use serde_json as json;
// Sort iterator
use itertools::Itertools;
// Regular expressions
use regex;

const VIDEOS: &str = "/home/davide/Videos/";
const WATCHED: &str = "/home/davide/Videos/watched.json";
const VIDEOS_EXT: [&str; 3] = ["mp4", "mkv", "webm"];

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Playlist {
    path: String,
    name: String,
    ep: u8
}
impl Playlist {
    fn next_ep(&mut self) {
        self.ep += 1;
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    match args.len() {
        1 => menu(),
        2 => {
            let opt = &args[1];
            match opt.as_str() {
                "refresh" => refresh(),
                _ => println!("Option not recognized: {}", opt)
            }
        }
        _ => println!("Too many options: {:?}", args)
    }
}

//// Menu subroutine

fn menu() {
    // Get playlists
    let mut map = load_watched(WATCHED);
    // Ask the user
    let choice = match menu::ask(map.keys().sorted()) {
        Err(why) => menu::terminate(why),
        Ok(s) => s
    };
    let pl = match map.get_mut(&choice) {
        None => {
            warn_invalid(choice);
            menu();
            return
        }
        Some(i) => i
    };
    // Print what has been choosen
    println!("User choice: {}", choice);
    println!("Matching playlist: {:#?}", pl);
    let next = if_next(&pl);
    println!("Next? {}", next);
    if next {
        pl.next_ep();
    }
    play_pl(pl);
    // println!("{:#?}", pl);
    // println!("{:#?}", map);
    write_watched(&map, WATCHED);
}

fn if_next(pl: &Playlist) -> bool {
    match pl.ep {
        0 => true,
        _ => {
            let mut choices = HashMap::new();
            choices.insert(format!("Play next episode ({})", pl.ep+1), true);
            choices.insert(format!("Continue previous episode ({})", pl.ep), false);
            let c = match menu::ask(choices.keys().sorted()) {
                Err(why) => menu::terminate(why),
                Ok(s) => s
            };
            let choice = choices.get(&c);
            match choice {
                None => {
                    warn_invalid(c);
                    if_next(pl)
                }
                Some(b) => b.clone()
            }
        }
    }
}

fn play_pl(pl: &Playlist) {
    // Get videos in playlist
    let videos = list_videos(&pl.path);
    let s = format!("^0*{}[^0-9]", pl.ep);
    let regex = regex::RegexBuilder::new(&s).build().unwrap();
    for video in videos {
	let f = match video.rfind("/") {
	    None => &video,
	    Some(i) => video.split_at(i+1).1
	};
	if regex.is_match(f) {
	    play(&video);
	    return
	}
    }
    panic!("No valid video found, episode {}: {}", pl.ep, pl.path);
}

fn play(path: &str) {
    match Command::new("openfile")
        .args(&[path])
        .spawn() {
            Err(why) => panic!("Playing {} with mpv has failed: {}", path, why),
            Ok(_) => ()
        };
}

//// Refresh subroutine

fn refresh() {
    let old = load_watched(WATCHED);
    let dirs = list_dirs(VIDEOS);
    let mut new = HashMap::new();
    for dir in dirs.iter() {
        let title = pretty(dir);
        let n = if old.contains_key(&title) {
            old[&title].ep
        } else {
            0
        };
        let p = Playlist {
            path: path2string(&dir),
            name: title.clone(),
            ep: n
        };
        new.insert(title, p);
    };
    write_watched(&new, WATCHED)
}

//// IO Handling

fn list_dirs(path: &str) -> Vec<PathBuf> {
    let files = match fs::read_dir(path) {
        Err(why) => panic!("Failed listing files in {}: {}", path, why),
        Ok(fs) => fs
    };
    let mut ret = Vec::new();
    for dir in files {
        let d_entry = dir.unwrap();
        let d = d_entry.path();
        if pretty(&d).starts_with(".") {
            continue
        }
        match d.is_dir() {
            false => println!("{:?} is file", d),
            true => ret.push(d.clone())
        }
    };
    ret
}

fn list_videos(path: &str) -> Vec<String> {
    let files = match fs::read_dir(path) {
        Err(why) => panic!("Failed listing files in {}: {}", path, why),
        Ok(fs) => fs
    };
    let mut videos = Vec::new();
    for f_maybe in files {
        let f = f_maybe.unwrap();
        let f_path = f.path();
        match f_path.extension() {
            Some(i) => {
                let i_str = i.to_str().unwrap();
                if array_has(&VIDEOS_EXT, &i_str) {
                    videos.push(path2string(&f_path))
                }
            }
            None => continue
        }
    };
    videos.sort();
    videos
}

//// JSON Handling

fn load_watched(path: &str) -> HashMap<String,Playlist> {
    let f = match fs::File::open(path) {
        Err(why) => panic!("Error opening {}: {}", path, why),
        Ok(v) => v
    };
    match serde_json::from_reader(f) {
        Err(why) => panic!("Error parsing {} as String:Person: {}", path, why),
        Ok(j) => j
    }
}

fn write_watched(map: &HashMap<String,Playlist>, path: &str) {
    let json = json::json!(map);
    write_json(&json, path);
}

fn write_json(json: &json::Value, path: &str) {
    let mut f = match fs::File::create(path) {
        Err(why) => panic!("Error creating {}: {}", path, why),
        Ok(f) => f
    };
    let s = format!("{:#}", json);
    match f.write_all(s.as_bytes()) {
        Err(why) => panic!("Error writing to {}: {}", path, why),
        Ok(_) => ()
    }
}

//// Misc

//// Get last folder name
fn pretty(path: &PathBuf) -> String {
    let foo = match path.file_stem() {
        None => path.to_str().unwrap(),
        Some(i) => i.to_str().unwrap()
    };
    String::from(foo)
}

fn path2string(path: &PathBuf) -> String {
    let foo = path.to_str().unwrap();
    String::from(foo)
}

fn array_has<T: std::cmp::PartialEq>(array: &[T], elem: &T) -> bool {
    for i in array {
        if i == elem {
            return true
        }
    };
    return false
}

fn warn_invalid(choice: impl Debug) {
    let s = format!("Invalid choice: {:?}", choice);
    match menu::ask(&[s]) {
        Err(why) => menu::terminate(why),
        Ok(_) => ()
    };
}
