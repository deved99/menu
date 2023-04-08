use std::env::var;
use std::fmt::{Debug, Display};
use std::io::Write;
use std::process::{Child, Command, Stdio};

mod error;
mod menu;
pub use error::{Error, Result};
pub use menu::main;

const DMENU: &str = "dmenu";

pub fn ask(iter: impl IntoIterator<Item = impl Display + Debug>) -> Result<String> {
    let mut cmd = Command::new(DMENU)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()?;
    // Write cmd stdin with iter content
    // !It would be nice to convert these to returns
    write_stdin(&mut cmd, iter)?;
    // Get cmd output
    let cmd = cmd.wait_with_output()?;
    let stdout = String::from_utf8(cmd.stdout)?;
    let result = stdout.trim();
    match result.is_empty() {
        true => Err(Error::EmptyResult),
        false => Ok(result.to_string()),
    }
}

fn write_stdin<T, U>(c: &mut Child, iter: T) -> Result<()>
where
    T: IntoIterator<Item = U>,
    U: Display + Debug,
{
    // let to_write: String = iter
    //     .into_iter()
    //     .map(|x| x.to_string())
    //     .intersperse("\n".to_string())
    //     .collect();
    let stdin = c.stdin.as_mut().ok_or(Error::NoneStdin)?;
    for i in iter {
        stdin.write_all(format!("{}\n", i).as_bytes())?;
    }
    Ok(())
}

pub fn get_conf_dir() -> String {
    match var("XDG_CONFIG_DIR") {
        Err(_) => {
            let home = var("HOME").expect("No $HOME?");
            format!("{}/.config", home)
        },
        Ok(i) => i,
    }
}
