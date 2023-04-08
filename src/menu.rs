use std::env::set_current_dir;
use std::fs::File;
use std::process::Command;

use serde::Deserialize;

use crate::{ask, get_conf_dir, Error, Result};

pub fn main() -> Result<()> {
    let config_dir = get_menu_dir();
    set_current_dir(config_dir)?;
    menu()
}

fn menu() -> Result<()> {
    let items = load()?;
    let s = ask(items.iter().map(|x| x.get_name()))?;
    for item in items {
        if item.get_name() == s {
            item.exec()?;
        }
    }
    Ok(())
}

fn load() -> Result<Vec<MenuItem>> {
    let r = File::open("config.json")?;
    serde_json::from_reader(r).map_err(Error::from)
}

#[derive(Deserialize)]
#[serde(untagged)]
enum MenuItem {
    Cmd { name: String, cmd: String },
    Dir { name: String, dir: String },
}
impl MenuItem {
    fn get_name(&self) -> &str {
        match self {
            Self::Cmd { name, .. } => name,
            Self::Dir { name, .. } => name,
        }
    }
    fn exec(&self) -> Result<()> {
        match &self {
            Self::Cmd { cmd: c, .. } => Command::new("/bin/sh")
                .args(["-c", c])
                .spawn()
                .map(|_| ())
                .map_err(Error::from),
            Self::Dir { dir: d, .. } => {
                set_current_dir(d).unwrap();
                menu()
            }
        }
    }
}

fn get_menu_dir() -> String {
    format!("{}/menu", get_conf_dir())
}
