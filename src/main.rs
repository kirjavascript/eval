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

fn python(script: &str) -> io::Output {
    run!(
        .arg("python")
        .arg("-c")
        .arg(script)
    )
}

fn php(script: &str) -> io::Output {
    run!(
        .arg("php")
        .arg("-r")
        .arg(script)
    )
}

fn lua(script: &str) -> io::Output {
    run!(
        .arg("lua")
        .arg("-e")
        .arg(script)
    )
}

fn racket(script: &str) -> io::Output {
    run!(
        .arg("racket")
        .arg("-e")
        .arg(script)
    )
}

fn elixir(script: &str) -> io::Output {
    run!(
        .arg("elixir")
        .arg("-e")
        .arg(script)
    )
}

fn quickjs(script: &str) -> io::Output {
    run!(
        .arg("qjs")
        .arg("-e")
        .arg(script)
        .arg("--bignum")
        .arg("--std")
    )
}

fn go(script: &str) -> io::Output {
    run!(
        .arg("yaegi")
        .arg("-e")
        .arg(script)
    )
}

fn guile(script: &str) -> io::Output {
    run!(
        .arg("guile")
        .arg("-c")
        .arg(format!(r#"
            (call-with-values (lambda () (values {}))
                (lambda args (display (cond ((null? args) "(No value)")
                ((= 1 (length args)) (car args)) (else (cons 'values args)))) (newline)))
        "#, script))
    )
}

fn rust(script: &str) -> io::Output {
    io::add_file(
        "./eval/script.rs",
        &format!(r#"
            fn main() {{
                println!("{{:?}}", {{
                    {}
                }})
            }}
        "#, script),
        || run!(
            .arg("bash")
            .arg("-c")
            .arg("rustc /eval/script.rs -o a.out && ./a.out")
        )
    )
}

fn haskell(script: &str) -> io::Output {
    io::add_file(
        "./eval/script.hs",
        script,
        || run!(
            .arg("bash")
            .arg("-c")
            .arg("ghci -v0 < /eval/script.hs")
        )
    )
}

fn vim(script: &str) -> io::Output {
    io::add_file(
        "./eval/script.vim",
        script,
        || run!(
            .arg("bash")
            .arg("-c")
            .arg(r#"
                vim --cmd "$(cat /eval/script.vim)" \
                    --cmd "execute \"set noreadonly\nset modifiable\nnormal kdggd07jdG\"" \
                    --cmd "write buffer" \
                    --cmd "cq" > vimout 2> vimerr
                if test -f "buffer"; then
                    cat buffer
                else
                    sed "s/^.*a terminal//" vimerr | sed "s/^.*pre-vimrc command line://"
                fi
            "#)
        )
    )
}

fn smalltalk(script: &str) -> io::Output {
    io::add_file(
        "./eval/script.st",
        script,
        || run!(
            .arg("gst")
            .arg("/eval/script.st")
        )
    )
}

fn gcc(script: &str) -> io::Output {
    io::add_file(
        "./eval/script.c",
        script,
        || run!(
            .arg("bash")
            .arg("-c")
            .arg("gcc -x c -o /a.out -w /eval/script.c && /a.out")
        )
    )
}

fn gpp(script: &str) -> io::Output {
    io::add_file(
        "./eval/script.cpp",
        script,
        || run!(
            .arg("bash")
            .arg("-c")
            .arg("g++ -x 'c++' -o /a.out -w /eval/script.cpp && /a.out")
        )
    )
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
                    "python" => warp::reply::json(&python(script)),
                    "php" => warp::reply::json(&php(script)),
                    "lua" => warp::reply::json(&lua(script)),
                    "gcc" => warp::reply::json(&gcc(script)),
                    "g++" => warp::reply::json(&gpp(script)),
                    "guile" => warp::reply::json(&guile(script)),
                    "racket" => warp::reply::json(&racket(script)),
                    "vim" => warp::reply::json(&vim(script)),
                    "smalltalk" => warp::reply::json(&smalltalk(script)),
                    "elixir" => warp::reply::json(&elixir(script)),
                    "quickjs" => warp::reply::json(&quickjs(script)),
                    "rust" => warp::reply::json(&rust(script)),
                    _ => warp::reply::json(&(false, "invalid language")),
                }
            } else {
                warp::reply::json(&(false, "pls provide valid utf8"))
            }

        });

    warp::serve(repl).run(([127, 0, 0, 1], 8010)).await;
}
