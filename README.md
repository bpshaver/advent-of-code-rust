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

## Template

This repo includes a template for new Rust crates using the Python project template library `cookiecutter`. To use it:

```
$ pip install -r requirements.txt
$ python -m cookiecutter template
```

And give the new project a name like `aoc_23_02` for e.g. Advent of Code 2023 day 2.

The newly generated crate will not be part of the workspace defined in the `Cargo.toml` at the root of this repo. To do so, add the crate under the `[packages]` section in that file and remove the empty `[workspace]` section from the new project's `Cargo.toml` file.

So that the newly generated crate can compile, there is a placeholder `input.txt` in a newly generated crate. You should replace this with an `input.txt` file from the Advent of Code website.
