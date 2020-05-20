use std::io::{Read, Error};
use std::process::{Child};
use std::time::Duration;
use std::fs::{self, File};
use std::io::prelude::*;
use std::path::Path;
use wait_timeout::ChildExt;

pub type Output = (bool, String);

static TIMEOUT: Duration = Duration::from_secs(8);


#[macro_export]
macro_rules! run {
    (
        $( $( .$meth:ident($( $arg:expr ),*)),+)+
    ) => {{

        use std::process::{Command, Stdio};
        let name = format!("eval-{}", uuid::Uuid::new_v4().to_string());

        let pod = Command::new("podman")
            .arg("run")
            .arg("--name")
            .arg(&name)
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

        let output = crate::io::run_command(pod)
            .map_or_else(|a| (false, a), |b| (true, b)); // this line copyright j`ey

        // cleanup
        Command::new("podman")
            .arg("kill").arg(&name).output().expect("could not kill container");

        output
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
        child.stdout.expect("stdout").read_to_string(&mut buffer).ok();
        child.stderr.expect("stderr").read_to_string(&mut buffer).ok();
    if status_code == 0 {
        Ok(buffer)
    } else {
        Err(buffer)
    }
}


fn write(path: &str, data: &str) -> Result<(), Error> {
    let path = Path::new(path);
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }
    let mut file = File::create(path)?;
    file.write_all(data.as_bytes())?;
    Ok(())
}

pub fn add_file<F>(path: &str, data: &str, out: F ) -> Output where
F: Fn() -> Output {
    match write(path, data) {
        Ok(_) => out(),
        Err(e) => (false, e.to_string()),
    }
}
