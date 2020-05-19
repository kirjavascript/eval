use std::io::{Read, Error};
use std::process::{Command, Child, Stdio};
use std::time::Duration;

use warp::{Filter};
use wait_timeout::ChildExt;

static TIMEOUT: Duration = Duration::from_secs(1);

fn run_command(command: String) -> Result<String, String> {
    let mut child = Command::new("podman")
        .arg("run")
        .arg("--rm")
        .arg("eval")
        .arg("bash")
        .arg("-c")
        .arg(command)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .map_err(|e| e.to_string())?;

        let status_code = match child.wait_timeout(TIMEOUT).unwrap() {
            Some(status) => status.code(),
            None => {
                child.kill().unwrap();
                child.wait().unwrap().code()
            }
        }.unwrap_or(0);

        let mut buffer = String::new();
        if status_code == 0 {
            child.stdout.unwrap().read_to_string(&mut buffer).unwrap();
            Ok(buffer)
        } else {
            child.stderr.unwrap().read_to_string(&mut buffer).unwrap();
            Err(buffer)
        }
}

fn output<T>(result: Result<T, T>) -> (bool, T) {
    result.map_or_else(|a| (false, a), |b| (true, b))
}

#[tokio::main]
async fn main() {
    let repl = warp::path!(String)
        .and(warp::body::content_length_limit(1024))
        .and(warp::body::bytes())
        .map(|
            _type: String,
            bytes: bytes::Bytes
            | {
            if let Ok(script) = std::str::from_utf8(&bytes) {
                let response = output(run_command(script.to_string()));
                warp::reply::json(&response)
            } else {
                warp::reply::json(&(false, "pls provide valid utf8"))
            }

        });

    warp::serve(repl).run(([127, 0, 0, 1], 8010)).await;
}
