FROM archlinux:latest

RUN pacman -Syu --noconfirm base-devel
RUN pacman -S git --noconfirm
RUN pacman -S cowsay --noconfirm
RUN pacman -S cmatrix --noconfirm

# deno
RUN pacman -S unzip --noconfirm
RUN curl -fsSL https://deno.land/x/install/install.sh | sh
ENV PATH="/root/.deno/bin:$PATH"

# haskell
RUN pacman -S --quiet --needed --noconfirm ghc cabal-install

# ruby
RUN pacman -S ruby --noconfirm

# node
RUN pacman -S nodejs --noconfirm

# go
RUN pacman -S go --noconfirm

# python
RUN pacman -S python --noconfirm

# php
RUN pacman -S php --noconfirm

# lua
RUN pacman -S lua --noconfirm

# racket
RUN pacman -S racket-minimal --noconfirm

# vim
RUN pacman -S vim --noconfirm

# kotlin
# swift
# basic
# intercal
# c++
# rust
# vim
# nim
# zig
# scheme
# clojure


WORKDIR /root
