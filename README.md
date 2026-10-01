# Airshipper

[![Discord](https://img.shields.io/discord/449602562165833758?logo=discord&logoColor=%23f8f8f8&label=discord&color=%23788dd5)](https://veloren.net/discord)
[![License](https://img.shields.io/github/license/tylerjgoodhue1995/Spaceship?color=blue)](https://github.com/tylerjgoodhue1995/Spaceship/blob/main/LICENSE)
[![GitHub Release](https://img.shields.io/github/v/release/tylerjgoodhue1995/Spaceship?color=blue)](https://github.com/tylerjgoodhue1995/Spaceship/releases)
[![AUR version](https://img.shields.io/aur/version/airshipper?label=AUR)](https://aur.archlinux.org/packages/airshipper/)

A cross-platform Veloren launcher.

![Airshipper](https://i.imgur.com/1VkndRZ.gif)

## Features

- [x] Update/Download and start nightly/weekly.
- [x] Fancy UI with batteries included.
- [x] Updates itself on windows.

## Download

**NOTE:** Airshipper cannot be considered stable yet.

For binary packages, use the GitHub releases for this repository.

For *source* packages **do not** use the `master` branch. Always package latest release either via tag (`v*.*.*`) or branch (`r*.*`) as master is unstable and contains work in progress features.

#### Compile from source

```bash
git clone https://github.com/tylerjgoodhue1995/Spaceship.git
cd Spaceship
cargo run --release
```

Make sure to have [rustup](https://rustup.rs/) installed to compile rust code and [git lfs](https://book.veloren.net/contributors/development-tools.html#git-lfs) for assets.

#### For NixOS users

You can install Airshipper with:

- Flakes enabled Nix: `nix profile install github:tylerjgoodhue1995/Spaceship`
- Flakes disabled Nix: `nix-env -i -f "https://github.com/tylerjgoodhue1995/Spaceship/archive/refs/heads/main.tar.gz"`

## Code of conduct

Our code of conduct is available here:

<https://veloren.net/code-of-conduct>
