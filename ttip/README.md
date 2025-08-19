# ttip - Tip of the Day

A command-line tool that gives you a new tip every day.

## Features

- Get a new Vim command each day.
- Get a new Korean word each day.

## Build

To build the project, you need to have Rust and Cargo installed.

```sh
cargo build --release
```

## Usage

To get a Vim tip of the day:

```sh
./target/release/ttip -v
```
or
```sh
./target/release/ttip --vim
```

To get a Korean word of the day:

```sh
./target/release/ttip -k
```
or
```sh
./target/release/ttip --korean
```
