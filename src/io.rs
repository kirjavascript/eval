use std::io::{Read, Error};
use std::process::{Child};
use std::time::Duration;
use std::fs::{self, File};
use std::io::prelude::*;
use std::path::Path;
use wait_timeout::ChildExt;

pub type Output = (bool, String);

static TIMEOUT: Duration = Duration::from_secs(1);

#[macro_export]
macro_rules! run {
    (
        $( $( .$meth:ident($( $arg:expr ),*)),+)+
    ) => {{
        use std::process::{Command, Stdio};
        let pod = Command::new("podman")
            .arg("run")
            .arg("-v")
            .arg("./repl:/repl:ro")
            .arg("--rm")
            .arg("eval")
            $(
                $(.$meth($($arg)*) )+
            )+
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn();

        crate::io::run_command(pod)
            .map_or_else(|a| (false, a), |b| (true, b)) // this line copyright j`ey
    }};
}

pub fn run_command(command: Result<Child, Error>) -> Result<String, String> {
    let mut child = command.map_err(|e| e.to_string())?;

    let status_code = match child.wait_timeout(TIMEOUT).unwrap() {
        Some(status) => status.code(),
        None => {
            child.kill().expect("timeout");
            child.wait().expect("timeout").code()
        }
    }.unwrap_or(0);

    let mut buffer = String::new();
    if status_code == 0 {
        child.stdout.expect("stdout").read_to_string(&mut buffer).ok();
        Ok(buffer)
    } else {
        child.stderr.expect("stderr").read_to_string(&mut buffer).ok();
        Err(buffer)
    }
}


pub fn write(path: &str, data: &str) -> Result<(), Error> {
    let path = Path::new(path);
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }
    let mut file = File::create(path)?;
    file.write_all(data.as_bytes())?;
    Ok(())
}
