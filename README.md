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

## Notes

### 2023-12-07

I wanted to solve Part Two by making the "card" and "hand" structs generic with respect to whether jokers are allowed, then have scoring and ordering methods dispatch based on the value of the const generic `const JOKERS: bool`:

```rust
part_one = generic_solution<false>(...)
part_two = generic_solution<true>(...)
```

Unfortunately, `rustc` (1.72.0) doesn't yet consider the `true` and `false` implementations of a trait with const generic booleans to be exhaustive: [issue](https://github.com/rust-lang/project-const-generics/issues/26).

```
the trait bound `HAND<JOKERS_ALLOWED>: FromStr` is not satisfied
            the following other typer implement trait `FromStr`:
                                                     Hand<false>
                                                      Hand<true>
                                                         (E0277)
```
