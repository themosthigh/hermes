# Hermes: God of Thieves & Messengers

This is a very tiny REST client that currently only supports GET requests.

<img width="756" height="467" alt="Screenshot 2026-05-14 at 23 51 30" src="https://github.com/user-attachments/assets/666fc907-ba3e-4569-8c86-72dacee0cdfa" />

## But WHY???

This is a continuation of the [pat](https://github.com/themosthigh/pat) project I started a few years ago, but I quicky got the ick for webviews.

1. Postman froze my friend's macbook. MacOS skill issue to be honest
2. There are many beautiful and very funciton Electron-base solutions that take up +250MB of my storage
3. I don't have friends and ...
4. I needed something to do for the weekend.

## Project setup

Follow the [Get started](https://rust-lang.org/learn/get-started/) guide to get rust setup.

#### Linux

(I haven't yet confirmed these instructions)

```bash
# RHEL/Fedora
sudo dnf install gtk4-devel libadwaita-devel meson desktop-file-utils pkgconf-devel

# Arch
sudo pacman -S gtk4 libadwaita meson desktop-file-utils pkg-config
```

#### MacOS

On macos you will need to run this verbatim

```sh
brew install gtk4 libadwaita meson desktop-file-utils pkg-config gtksourceview5
```

## Running the project

```rs
cargo run # obviously
```

## Progress so far

I cannot stress this enough, I've been tempted to quit and opt for a different solution such as [gtkx](https://gtkx.dev/) or [flutter](https://flutter.dev).
Rust + GTK is a tall order. That said, we have the following working:

#### Request manipulation

- [x] Custom url
- [x] Custom request methods
- [x] Custom headers (not the best experience but we're getting somewhere)
- [x] Custom JSON body
- [ ] Query params tracking
- [ ] File requests
- [ ] Custom request body
  - [ ] Change type
  - [ ] Update formatting

#### Data exporting

- [ ] Import request
  - [x] Paste `cURL`
  - [x] `.http` file
  - [ ] `.rest` file
- [ ] Export request
  - [ ] `cURL` string
  - [ ] Save as `.http` file
  - [ ] Save as `.rest` file

#### Response presentation

- [ ] Custom repsonse type
- [ ] Trigger formatting

#### Quality of life features

- [ ] Global keyboard shortcuts
- [ ] Open workspaces
- [ ] Environment variable support
- [ ] Multiple windows
- [ ] Multiple tabs
- [ ] Home Screen based on history

<img width="1837" height="1151" alt="Screenshot From 2026-05-17 23-50-08-min" src="https://github.com/user-attachments/assets/97539551-2d31-40b1-95d4-b9f0a9b5ebb2" />

The goal here is to get somewhere between [HTTPie](https://httpie.io/) and [Cartero](https://cartero.danirod.es/)
