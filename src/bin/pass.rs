// Std
use std::env::var;
use std::fs::read_dir;
use std::process::{Command,Stdio};
// Local
use menu::ask;

fn main() {
    let mut names = get_names().unwrap();
    names.sort();
    let c = ask(&names).unwrap();
    write_pass(&c);
}


fn get_names() -> Result<Vec<String>,String> {
    let i = match read_dir( pass_dir() ) {
        Err(why) => return Err(why.to_string()),
        Ok(r) => r
    };
    let mut v = Vec::new();
    for f in i {
        let f = match f {
            Err(_) => continue,
            Ok(s) => s
        };
        let p = f.path();
        let e = match p.extension() {
            None => continue,
            Some(s) => s
        };
        if e == "gpg" {
            let s = p.file_stem().unwrap()
                .to_str().unwrap()
                .to_string();
            v.push(s)
        }
    }
    Ok(v)
}

fn write_pass(s: &str) {
    let o = match Command::new("pass")
        .arg(s)
        .stdout(Stdio::piped())
        .spawn() {
            Err(why) => {
                println!("Error executing pass: {}", why);
                return
            },
            Ok(s) => s
        };
    if let Some(i) = o.stdout {
        let mut p = Command::new("xclip")
            .args(&["-selection", "primary", "-i"])
            .stdin(i)
            .spawn().unwrap();
        // Wait for copying, crash if problems
        p.wait().unwrap();
        Command::new("xdotool")
            .args(&["click", "2"])
            .status().unwrap();
        // Clear clipboard
        Command::new("xclip")
            .args(&["-selection", "primary", "-i", "/dev/null"])
            .status().unwrap();
    }
}

fn pass_dir() -> String {
    match var("PASSWORD_STORE_DIR") {
        Err(_) => format!("{}/.password_store", var("HOME").unwrap() ),
        Ok(s) => s
    }
}
