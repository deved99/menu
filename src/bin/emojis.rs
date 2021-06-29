use std::collections::HashMap;
// file processing
use std::fs::{self,File};
// Run command
use std::env::args;
use std::process::Command;
// Local
use menu::{terminate,ask};
// JSON
use serde::{Serialize,Deserialize};
use serde_json as json;

use itertools::Itertools;

//// Constants
// PATH is the path to a json file that contains a dictionary {emoji:description}
const PATH: &str = "/home/davide/.config/local/emoji.json";

fn main() {
    let argv: Vec<String> = args().collect();
    match argv.len() {
        1 => dmenu(),
        2 => match argv[1].as_str() {
            "dmenu" => dmenu(),
            "refresh" => refresh(),
            a => terminate(format!("Option not recognized: {}", a)),
        },
        3 => {
            let src = match argv.get(2) {
                Some(s) => s,
                None => terminate("Not enough args")
            };
            match argv[1].as_str() {
                "from" => update(src),
                a => terminate(format!("Option not recognized: {}", a)),
            }
        },
        _ => terminate( format!("Too manu arguments: {:?}", &argv) )
    }
}

//// Subroutines
fn dmenu() {
    let mut emojis = match Emojis::from(PATH) {
        Err(why) => terminate(why),
        Ok(e) => e
    };
    emojis.dmenu();
}

fn refresh() {
    let mut emojis = match Emojis::from(PATH) {
        Err(why) => terminate(why),
        Ok(e) => e
    };
    emojis.refresh();
    if let Err(why) = emojis.save(PATH) {
        terminate(why)
    }
}

fn update(src: &str) {
    let mut emojis = match Emojis::from(PATH) {
        Err(why) => terminate(why),
        Ok(e) => e
    };
    let text: Vec<String> = match fs::read(src) {
        Err(why) => terminate(why),
        Ok(v) => match String::from_utf8(v) {
            Err(why) => terminate(why),
            Ok(s) => s.split("\n")
                .map(|s| s.to_string())
                .collect()
        }
    };
    emojis.update_from_txt(text)
}

//// Emojis object

#[derive(Deserialize,Serialize)]
struct Emojis {
    current: HashMap<String,String>,
    backup: HashMap<String,String>,
    renames: HashMap<String,String>,
    most_used: HashMap<String,u16>
}
impl Emojis {
    // Get config from path
    fn from(path: &str) -> Result<Self,String> {
        // Open file read-only
        let f = match File::open(path) {
            Err(why) => panic!("File couldn't be opened: {}", why),
            Ok(f) => f,
        };
        // Read file content to string
        match json::from_reader(f) {
            Err(why) => Err(
                    format!("File couldn't be parsed as json: {}", why),
                ),
            Ok(d) => Ok(d),
        }
    }
    fn save(&self, path: &str) -> Result<(),String> {
        // Create file
        let f = match File::create(path) {
            Err(why) => return Err(why.to_string()),
            Ok(f) => f
        };
        // Write to path
        match json::to_writer_pretty(f, &self) {
            Err(why) => Err(why.to_string()),
            Ok(_) => Ok(())
        }
    }
    fn dmenu(&mut self) {
        let uses = |a| {
            match self.most_used.get(a) {
                None => 0,
                Some(n) => *n
            }
        };
        let pretty = self.current.iter()
            // First, most used, than alphabetical order
            .sorted_by( |a,b| (uses(b.0), a.1).cmp(&(uses(a.0), b.1)) )
            // then map to displayed format
            .map(|(k,v)| format!("{}: {}", k, v));
        let choice = match ask(pretty) {
            Err(why) => terminate(why),
            Ok(s) => match s.split(":").next() {
                None => terminate("No emoji found"),
                Some(e) => e.to_string()
            }
        };
        write(&choice);
        // Now add e to the uses counter
        let n = uses(&choice);
        self.most_used.insert(choice, n+1);
        // and save
        if let Err(why) = self.save(PATH) {
            terminate(why)
        }
    }
    fn refresh(&mut self) {
        let mut foo = self.backup.clone();
        for (k,v) in self.renames.iter() {
            foo.insert(k.clone(), v.clone());
        }
        self.current = foo;
        if let Err(why) = self.save(PATH) {
            terminate(why)
        }
    }
    fn update_from_txt(&mut self, lines: Vec<String>) {
        for line in lines {
            let (k,v) = match line.split_once(":") {
                None => continue,
                Some((k,v)) => ( k.to_string(), v.trim().to_string() )
            };
            self.backup.insert(k,v);
        }
        self.refresh();
    }
}


fn write(emoji: &str) {
    match Command::new("xdotool").args(&["type", emoji]).spawn() {
        Err(why) => terminate(format!("Error typing the emoji: {}", why)),
        Ok(_) => (),
    }
}
