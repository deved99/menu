// Std
use std::env::var;
use std::fs::read_dir;
use std::process::Command;
// Local
use menu::ask;

fn main() {
    let names = get_names().unwrap();
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
        .output() {
            Err(why) => {
                println!("Error executing pass: {}", why);
                return
            },
            Ok(s) => s
        };
    let raw = String::from_utf8(o.stdout).unwrap();
    let out = raw.trim_end();
    Command::new("xdotool")
        .arg("type").arg(out)
        .spawn().unwrap();
}

fn pass_dir() -> String {
    format!("{}/.password-store", var("HOME").unwrap())
}
