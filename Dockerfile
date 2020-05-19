FROM archlinux:latest

RUN pacman -Syu --noconfirm base-devel

RUN pacman -S git --noconfirm

# yay
# RUN useradd -ms /bin/bash repl \
#  && echo "repl ALL=NOPASSWD:/usr/bin/pacman,/usr/bin/aura" >> /etc/sudoers
# USER repl
# RUN git clone https://aur.archlinux.org/yay.git && cd yay && makepkg -si --noconfirm

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
# c/c++
# rust
# swift
# python
# vim

RUN pacman -S cowsay --noconfirm

WORKDIR /root