mod io;
use warp::{Filter};

fn bash(script: &str) -> io::Output {
    run!(
        .arg("bash")
        .arg("-c")
        .arg(script)
    )
}

fn node(script: &str) -> io::Output {
    run!(
        .arg("node")
        .arg("-p")
        .arg(script)
    )
}

fn deno(script: &str) -> io::Output {
    run!(
        .arg("deno")
        .arg("eval")
        .arg(script)
    )
}

fn ruby(script: &str) -> io::Output {
    run!(
        .arg("ruby")
        .arg("-e")
        .arg(script)
    )
}

fn perl(script: &str) -> io::Output {
    run!(
        .arg("perl")
        .arg("-e")
        .arg(script)
    )
}

fn haskell(script: &str) -> io::Output {
    match io::write("./repl/repl.hs", script) {
        Ok(_) => run!(.arg("bash")
                .arg("-c")
                .arg("ghci -v0 < /repl/repl.hs")),
        Err(err) => (false, err.to_string()),
    }
}

fn go(script: &str) -> io::Output {
    let code = format!(r#"
        package main
        import "fmt"
        func main() {{
            {}
        }}
    "#, script);
    match io::write("./repl/repl.go", &code) {
        Ok(_) => run!(.arg("bash")
                .arg("-c")
                .arg("go run /repl/repl.go")),
        Err(err) => (false, err.to_string()),
    }
}

#[tokio::main]
async fn main() {
    let repl = warp::path!(String)
        .and(warp::body::content_length_limit(1024))
        .and(warp::body::bytes())
        .map(|lang: String, bytes: bytes::Bytes| {
            if let Ok(script) = std::str::from_utf8(&bytes) {
                match lang.as_ref() {
                    "bash" => warp::reply::json(&bash(script)),
                    "node" => warp::reply::json(&node(script)),
                    "deno" => warp::reply::json(&deno(script)),
                    "ruby" => warp::reply::json(&ruby(script)),
                    "perl" => warp::reply::json(&perl(script)),
                    "haskell" => warp::reply::json(&haskell(script)),
                    "go" => warp::reply::json(&go(script)),
                    _ => {
                        warp::reply::json(&(false, "invalid language"))
                    }
                }
            } else {
                warp::reply::json(&(false, "pls provide valid utf8"))
            }

        });

    warp::serve(repl).run(([127, 0, 0, 1], 8010)).await;
}
