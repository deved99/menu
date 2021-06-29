use std::env::var;
use std::fs;
use std::process::Command;


fn main() {
    let path = match var("PATH") {
        Err(why) => panic!("Error getting $PATH: {}", why),
        Ok(v) => v
    };
    let mut bins = Vec::new();
    for p in path.split(":") {
        let foo = read_path(p);
        bins.extend(foo);
    }
    bins.sort();
    let c = match menu::ask(bins) {
        Err(why) => menu::terminate(why),
        Ok(c) => c,
    };
    let mut c_iter = c.split(" ");
    let program = match c_iter.next() {
        None => menu::terminate("No choice given"),
        Some(i) => i
    };
    let mut args = Vec::new();
    for a in c_iter {
        args.push(a);
    };
    match Command::new(program)
        .args(&args)
        .spawn() {
            Err(why) => println!("Error spawning {} with args {:?}: {}", program, args, why),
            Ok(_) => ()
        }
}

fn read_path(path: &str) -> Vec<String> {
    let raw = match fs::read_dir(path) {
        Err(why) => {
            println!("Error getting files in folder {}: {}", path, why);
            return Vec::new()
        },
        Ok(l) => l
    };
    let mut ret = Vec::new();
    for i_opt in raw {
        let i = match &i_opt {
            Err(why) => panic!("Error iterating over {:?}: {}", i_opt, why),
            Ok(j) => {
                let path = j.path();
                let path_str = match path.file_stem() {
                    None => "",
                    Some(s) => match s.to_str() {
                        None => "",
                        Some(i) => i
                    }
                };
                path_str.to_string()
            }
        };
        ret.push(i)
    }
    ret
}
