FROM archlinux:latest

RUN pacman -Syu --noconfirm base-devel

RUN pacman -S git --noconfirm


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

# kotlin
# swift
# basic
# intercal
# lua
# c++
# rust
# vim
# nim
# zig
# scheme
# clojure

RUN pacman -S cowsay --noconfirm

# yay
# RUN useradd -ms /bin/bash repl \
#  && echo "repl ALL=NOPASSWD:/usr/bin/pacman,/usr/bin/aura" >> /etc/sudoers
# USER repl
# WORKDIR /home/repl
# RUN git clone https://aur.archlinux.org/yay.git && cd yay && makepkg -si --noconfirm
# USER root


WORKDIR /root
