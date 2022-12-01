# Advent of Code Solutions in Rust

Run all tests:

```
$ cargo test
```

To run a particular binary crate:

```
$ cargo run -p <crate>
```

or `cd` into that crate and `cargo run` there.
## `aoc_utils`

This repo contains a crate called `aoc_utils`. Documentation is available [here](https://bpshaver.github.io/advent-of-code-rust/aoc_utils/).

You can use it within your own Rust project by adding this to your `Cargo.toml`:

```
aoc_utils = { git = "https://github.com/bpshaver/advent-of-code-rust", branch = "main" }
```

PRs are welcome.

## Template

This repo includes a template for new Rust crates using the Python project template library `cookiecutter`. To use it:

```
$ pip install -r requirements.txt
$ python -m cookiecutter template
```

And give the new project a name like `aoc23_02` for e.g. Advent of Code 2023 day 2.

The newly generated crate will not be part of the workspace defined in the `Cargo.toml` at the root of this repo. To do so, add the crate under the `[packages]` section in that file and remove the empty `[workspace]` section from the new project's `Cargo.toml` file.

So that the newly generated crate can compile, there is a placeholder `input.txt` in a newly generated crate. You should replace this with an `input.txt` file from the Advent of Code website.
