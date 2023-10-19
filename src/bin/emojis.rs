use std::collections::HashMap;
// file processing
use std::fs::{self, File};
use std::io::Write;
use std::path::PathBuf;
// Run command
use std::env::args;
use std::process::{Command, Stdio};
// Local
use menu::{ask, get_conf_dir, Error, Result};
// JSON
use serde::{Deserialize, Serialize};

use itertools::Itertools;

fn main() -> Result<()> {
    let argv: Vec<String> = args().collect();
    match argv.len() {
        1 => dmenu(),
        2 => match argv[1].as_str() {
            "dmenu" => dmenu(),
            "refresh" => refresh(),
            "used" => most_used(),
            a => panic!("Option not recognized: {}", a),
        },
        3 => {
            let src = &argv[2];
            match argv[1].as_str() {
                "from" => update(src),
                a => panic!("Option not recognized: {}", a),
            }
        }
        _ => panic!("Too manu arguments: {:?}", &argv),
    }
}

//// Subroutines
fn dmenu() -> Result<()> {
    let mut emojis = Emojis::from(&get_path())?;
    emojis.dmenu()?;
    Ok(())
}

fn refresh() -> Result<()> {
    let mut emojis = Emojis::from(&get_path())?;
    emojis.refresh()?;
    emojis.save(&get_path())
}

fn most_used() -> Result<()> {
    let emojis = Emojis::from(&get_path())?;
    emojis.most_used();
    Ok(())
}

fn update(src: &str) -> Result<()> {
    let mut emojis = Emojis::from(&get_path())?;
    let source = fs::read_to_string(src)?;
    let text: Vec<&str> = source.split('\n').collect();
    emojis.update_from_txt(&text)
}

//// Emojis object

#[derive(Deserialize, Serialize)]
struct Emojis {
    current: HashMap<String, String>,
    backup: HashMap<String, String>,
    renames: HashMap<String, String>,
    most_used: HashMap<String, u16>,
}
impl Emojis {
    // Get config from path
    fn from(path: &str) -> Result<Self> {
        // Open file read-only
        let f = File::open(path)?;
        // Read file content to string
        serde_yaml::from_reader(f).map_err(Error::from)
    }
    // Print most used
    fn most_used(&self) {
        let iter = self
            .most_used
            .iter()
            .sorted_by(|j, i| Ord::cmp(i.1, j.1))
            .take(10);
        for (k, i) in iter {
            println!("{}: {}", k, i)
        }
    }
    fn save(&self, path: &str) -> Result<()> {
        // Create file
        let f = File::create(path)?;
        // Write to path
        serde_yaml::to_writer(f, &self)
            .map(|_| ())
            .map_err(Error::from)
    }
    fn dmenu(&mut self) -> Result<()> {
        let uses = |a| match self.most_used.get(a) {
            None => 0,
            Some(n) => *n,
        };
        let pretty = self
            .current
            .iter()
            // First, most used, than alphabetical order
            .sorted_by(|a, b| (uses(b.0), a.1).cmp(&(uses(a.0), b.1)))
            // then map to displayed format
            .map(|(k, v)| format!("{}: {}", k, v));
        let choice = ask(pretty)?.split(':').next()
            .ok_or(Error::EmptyResult)?
            .to_string();
        write(&choice);
        // Now add e to the uses counter
        let n = uses(&choice);
        self.most_used.insert(choice, n + 1);
        // and save
        self.save(&get_path())
    }
    fn refresh(&mut self) -> Result<()> {
        let mut result = self.backup.clone();
        for (k, v) in self.renames.clone().into_iter() {
            result.insert(k, v);
        }
        self.current = result;
        self.save(&get_path())
    }
    fn update_from_txt(&mut self, lines: &[&str]) -> Result<()> {
        for line in lines {
            let (k, v) = match line.split_once(':') {
                None => continue,
                Some((k, v)) => (k.to_string(), v.trim().to_string()),
            };
            self.backup.insert(k, v);
        }
        self.refresh()
    }
}

fn write(emoji: &str) {
    let mut p = Command::new("xclip")
        .stdin(Stdio::piped())
        .args(["-selection", "clipboard", "-i"])
        .spawn()
        .unwrap();
    {
        let stdin = p.stdin.as_mut().unwrap();
        stdin.write_all(emoji.as_bytes()).unwrap();
    }
    // Wait for copying, crash if problems
    p.wait().unwrap();
    Command::new("xdotool")
        .args(["key", "--clearmodifiers", "Shift+Insert"])
        .status()
        .unwrap();
}

fn get_path() -> String {
    let conf = get_conf_dir();
    let path: PathBuf = [&conf, "local", "emoji.json"].iter().collect();
    path.to_str().unwrap().to_string()
}
