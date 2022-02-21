use std::env::set_current_dir;
use std::fs::File;
// to spawn a new process
use std::process::Command;
// Local
use menu::{get_conf_dir,ask,term};
// External
use serde::Deserialize;
use serde_json as json;

fn main() {
    set_current_dir( get_menu_dir() ).unwrap();
    menu();
}

fn menu() {
    let items = load();
    let s = ask( items.iter().map(|x| x.name()) )
        .unwrap();
    for i in items {
        if i.name() == &s {
            i.exec();
            break
        }
    }
}

fn load() -> Vec<MenuItem> {
    let r = match File::open("config.json") {
        Err(why) => term!(why),
        Ok(r) => r
    };
    match json::from_reader(r) {
        Err(why) => term!(why),
        Ok(j) => j
    }
}

#[derive(Deserialize)]
#[serde(untagged)]
enum MenuItem {
    Cmd { name: String, cmd: String },
    Dir { name: String, dir: String }
}
impl MenuItem {
    fn name(&self) -> &str {
        use MenuItem::*;
        match &self {
            Cmd { name: n, .. } => &n,
            Dir { name: n, .. } => &n
        }
    }
    fn exec(&self) {
        use MenuItem::*;
        match &self {
            Cmd { cmd: c, .. } => {
                Command::new("/bin/sh")
                    .args( [ "-c", &c ] )
                    .spawn()
                    .unwrap();
            },
            Dir { dir: d, .. } => {
                set_current_dir(&d)
                    .unwrap();
                menu()
            }
        }
    }
}

fn get_menu_dir() -> String {
    format!("{}/menu", get_conf_dir())
}
