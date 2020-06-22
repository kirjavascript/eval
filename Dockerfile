FROM archlinux:latest

RUN pacman -Syu --noconfirm base-devel
RUN pacman -S git --noconfirm
RUN pacman -S cowsay --noconfirm
RUN pacman -S cmatrix --noconfirm
RUN pacman -S fortune-mod --noconfirm

RUN pacman -S unzip --noconfirm
RUN curl -fsSL https://deno.land/x/install/install.sh | sh
ENV PATH="/root/.deno/bin:$PATH"

RUN pacman -S --quiet --needed --noconfirm ghc cabal-install

RUN pacman -S ruby --noconfirm

RUN pacman -S nodejs --noconfirm

RUN pacman -S python --noconfirm

RUN pacman -S php --noconfirm

RUN pacman -S lua --noconfirm

RUN pacman -S racket-minimal --noconfirm

RUN pacman -S vim --noconfirm

RUN pacman -S smalltalk --noconfirm

RUN pacman -S elixir --noconfirm

ADD https://bellard.org/quickjs/binary_releases/quickjs-linux-x86_64-2020-04-12.zip /.qjs
RUN unzip /.qjs -d /bin && rm /.qjs

RUN pacman -S go --noconfirm
RUN go get -u github.com/containous/yaegi/cmd/yaegi
RUN mv /root/go/bin/yaegi /bin/yaegi && rm -rf /root/go

RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | bash -s -- -y
ENV PATH="/root/.cargo/bin:${PATH}"

RUN pacman -S cmake --noconfirm
RUN git clone https://github.com/justinmeza/lci.git /root/.lci
WORKDIR /root/.lci
RUN cmake .
RUN make && make install

# freebies: bash, perl, guile, gcc, g++

# kotlin
# swift
# basic
# intercal
# nim
# zig
# clojure
# freebasic / fbc


WORKDIR /root
