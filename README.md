# Advent of Code Tooling

This repo holds a Rust crate to help with my [Advent of Code](https://adventofcode.com/) solutions.

## Usage

Either add this to your `Cargo.toml`:

```toml
[dependencies]
aoc = { git = "https://github.com/thecjharries/aoc-rust", branch = "main" }
```

or from the command line:

```bash
cargo add --rename aoc --git "https://github.com/thecjharries/aoc-rust" --branch main
```

## Important Setup

You must have an `AOC_SESSION` environment variable set to your Advent of Code session cookie. You can find this cookie by logging into the Advent of Code website and looking at the value of the `session` cookie.

I recommend [direnv](https://direnv.net/) for managing environment variables. Make sure to add `.envrc` to your `.gitignore` file.
