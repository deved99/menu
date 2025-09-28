// Get cmd-line arguments
use std::collections::HashMap;
use std::env;
use std::fmt::Debug;
// File handling
use std::fs;
use std::path::Path;
use std::path::PathBuf;
// Exec commands, write/read stdin/stdout
use std::process::Command;

use itertools::Itertools;
// JSON handling
use serde::{Deserialize, Serialize};

use menu::{Error, Result, get_home};

const VIDEOS_EXT: [&str; 3] = ["mp4", "mkv", "webm"];

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Playlist {
    path: String,
    name: String,
    ep: u8,
}
impl Playlist {
    fn next_ep(&mut self) {
        self.ep += 1;
    }
}

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    match args.len() {
        1 => menu(),
        2 => {
            let opt = &args[1];
            match opt.as_str() {
                "refresh" => refresh(),
                _ => panic!("Option not recognized: {}", opt),
            }
        }
        _ => panic!("Too many options: {:?}", args),
    }
}

//// Menu subroutine

fn menu() -> Result<()> {
    // Get playlists
    let mut map = load_watched(get_watched_path())?;
    // Ask the user
    let choice = menu::ask(map.keys().sorted())?;
    let pl = match map.get_mut(&choice) {
        None => {
            warn_invalid(choice)?;
            return menu();
        }
        Some(i) => i,
    };
    // Print what has been choosen
    println!("User choice: {}", choice);
    println!("Matching playlist: {:#?}", pl);
    let next = if_next(pl)?;
    println!("Next? {}", next);
    if next {
        pl.next_ep();
    }
    play_pl(pl)?;
    // println!("{:#?}", pl);
    // println!("{:#?}", map);
    write_yaml(&map, get_watched_path())?;
    Ok(())
}

fn if_next(pl: &Playlist) -> Result<bool> {
    let res = match pl.ep {
        0 => true,
        _ => {
            let choices = [
                format!("Play next episode ({})", pl.ep + 1),
                format!("Continue previous episode ({})", pl.ep),
            ];
            let choice = menu::ask(choices)?;
            if choice.starts_with("Play") {
                true
            } else if choice.starts_with("Continue") {
                false
            } else {
                warn_invalid(choice)?;
                if_next(pl)?
            }
        }
    };
    Ok(res)
}

fn play_pl(pl: &Playlist) -> Result<()> {
    // Get videos in playlist
    let videos = list_videos(&pl.path);
    let s = format!("^0*{}[^0-9]", pl.ep);
    let regex = regex::Regex::new(&s)?;
    for video in videos {
        let f = match video.rfind('/') {
            None => &video,
            Some(i) => video.split_at(i + 1).1,
        };
        if regex.is_match(f) {
            return play(&video);
        }
    }
    panic!("No valid video found, episode {}: {}", pl.ep, pl.path);
}

fn play(path: &str) -> Result<()> {
    Command::new("openfile")
        .args([path])
        .spawn()
        .map(|_| ())
        .map_err(Error::from)
}

//// Refresh subroutine

fn refresh() -> Result<()> {
    let old = load_watched(&get_watched_path())?;
    let dirs = list_dirs(&get_videos_path())?;
    let mut new = HashMap::new();
    for dir in dirs.iter() {
        let title = pretty(dir).to_string();
        let n = if old.contains_key(&title) {
            old[&title].ep
        } else {
            0
        };
        let p = Playlist {
            path: path2string(dir),
            name: title.clone(),
            ep: n,
        };
        new.insert(title, p);
    }
    write_yaml(&new, get_watched_path())
}

//// IO Handling

fn list_dirs(path: impl AsRef<Path>) -> Result<Vec<PathBuf>> {
    let files = fs::read_dir(path)?;
    let files = files
        .map(|x| x.expect("Path is none!?"))
        .map(|s| s.path())
        .filter(|p| !pretty(p).starts_with('.'))
        .filter(|p| p.is_dir())
        .collect();
    Ok(files)
}

fn list_videos(path: &str) -> Vec<String> {
    let files = match fs::read_dir(path) {
        Err(why) => panic!("Failed listing files in {}: {}", path, why),
        Ok(fs) => fs,
    };
    let mut videos = Vec::new();
    for f_maybe in files {
        let f = f_maybe.unwrap();
        let f_path = f.path();
        match f_path.extension() {
            Some(i) => {
                let i_str = i.to_str().unwrap();
                if VIDEOS_EXT.contains(&i_str) {
                    videos.push(path2string(&f_path))
                }
            }
            None => continue,
        }
    }
    videos.sort();
    videos
}

//// JSON Handling

fn load_watched(path: impl AsRef<Path>) -> Result<HashMap<String, Playlist>> {
    let f = fs::File::open(path)?;
    serde_yaml::from_reader(f).map_err(Error::from)
}

fn write_yaml(data: impl serde::Serialize, path: impl AsRef<Path>) -> Result<()> {
    let f = fs::File::create(path)?;
    serde_yaml::to_writer(f, &data)
        .map_err(Error::from)
        .map(|_| ())
}

//// Misc

//// Get last folder name
fn pretty(path: &Path) -> &str {
    let filename = match path.file_name() {
        None => path.to_str(),
        Some(i) => i.to_str(),
    };
    filename.unwrap_or("")
}

fn path2string(path: &Path) -> String {
    path.to_str().unwrap_or("").to_string()
}

fn warn_invalid(choice: impl Debug) -> Result<()> {
    let s = format!("Invalid choice: {:?}", choice);
    menu::ask([s]).map_err(Error::from).map(|_| ())
}

fn get_videos_path() -> PathBuf {
    let home = get_home();
    Path::new(&home).join("Videos")
}
fn get_watched_path() -> PathBuf {
    get_videos_path().join("watched.json")
}
