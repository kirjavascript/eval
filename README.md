## safely evaluate code from user input

built on [warp](https://github.com/seanmonstar/warp) + [podman](https://podman.io/)

build container

```bash
podman build -t eval .
```

start HTTP server

```bash
cargo build --release
./target/release/eval
```

run some code

```bash
curl -X POST --data-binary 'whoami' http://localhost:8010/bash
```
