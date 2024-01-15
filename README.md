# my-rust-template

My template repository of rust project

## Features

- [rust-toolchain.toml](#rust-toolchaintoml)
- [Nix Flakes](#nix-flakes)
- [direnv](#direnv)
- [GitHub Actions](#github-actions)

## rust-toolchain.toml

This file is referenced by rustup or Nix Flakes, in order to [override](https://rust-lang.github.io/rustup/overrides.html) rust toolchain on this repository. This is useful for sharing toolchain setup between project members.

## Nix Flakes

[Nix Flakes](https://nixos.wiki/wiki/Flakes) is a nix feature, which enables us to reproduce *environment* with ease. The files [`flake.nix`](./flake.nix) and [`flake.lock`](./flake.lock) are used by this feature.

## direnv

[direnv](https://direnv.net) is used to load development environment of Nix Flakes quickly. To support non-nix users, [`.envrc`](./.envrc) exits quickly without any changes if `nix` not found in `$PATH`.

## GitHub Actions

GitHub Actions is already configured. In [`rust.yml`](./.github/workflows/rust.yml), `cargo build` and `rustfmt` and `clippy` are checked using both rustup and Nix Flakes.
