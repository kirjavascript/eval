FROM archlinux:latest

RUN pacman -Syu --noconfirm base-devel

RUN pacman -S git --noconfirm

RUN useradd -ms /bin/bash repl \
 && echo "repl ALL=NOPASSWD:/usr/bin/pacman,/usr/bin/aura" >> /etc/sudoers

WORKDIR /home/repl
COPY clickbait .
WORKDIR /

# yay
# USER repl
# RUN git clone https://aur.archlinux.org/yay.git && cd yay && makepkg -si --noconfirm

# perl -e 'script'

# deno
RUN pacman -S unzip --noconfirm
RUN curl -fsSL https://deno.land/x/install/install.sh | sh
ENV PATH="/root/.deno/bin:$PATH"
# deno eval 'script'

# haskell
RUN pacman -S --quiet --needed --noconfirm ghc cabal-install
# ghci -v0 -fdiagnostics-color=never < test.hs

# ruby
RUN pacman -S ruby --noconfirm
# ruby -e 'script'

# node
RUN pacman -S nodejs --noconfirm
# node -e 'script'

# go
# rust
# swift
# python
# vim

RUN pacman -S cowsay --noconfirm
