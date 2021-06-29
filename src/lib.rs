use std::fmt::{Display,Debug};
use std::io::Write;
use std::process::{Command, Child, Stdio, exit};

const DMENU: &str = "dmenu";

pub fn ask(iter: impl IntoIterator<Item = impl Display+Debug>) -> Result<String,String> {
    let mut cmd = match Command::new(DMENU)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn() {
            Err(why) => return Err(why.to_string()),
            Ok(p) => p
        };
    // Write cmd stdin with iter content
    // !It would be nice to convert these to returns
    if let Err(why) = write_stdin(&mut cmd, iter) {
        return Err(why)
    };
    // Get cmd output
    match cmd.wait_with_output() {
        Err(why) => Err( format!("Error getting dmenu's output: {}", why) ),
        Ok(o) => match String::from_utf8(o.stdout) { // Convert 2 string
            Err(why) => Err(format!("Error: dmenu's output -> string: {}", why)),
            Ok(s) => match s.trim() {
                x if x.is_empty() => Err("Output of dmenu is empty".to_string()),
                x => Ok(x.to_string())
            }
        }
    }
}

fn write_stdin<T,U>(c: &mut Child, iter: T) -> Result<(),String>
where T: IntoIterator<Item = U>,
      U: Display+Debug {
    let stdin = match c.stdin.as_mut() {
        None => return Err("Failed to get stdin".to_string()),
        Some(s) => s
    };
    for i in iter {
        match stdin.write(format!("{}\n", i).as_bytes()) {
            Err(why) => return Err(format!("Error writing {:?} to stdin: {}", i, why)),
            Ok(_) => (),
        }
    }
    Ok(())
}

pub fn terminate(s: impl Display) -> ! {
    println!("{}", s);
    exit(1)
}
