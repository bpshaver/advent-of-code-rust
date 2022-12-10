#![warn(missing_docs)]

//! Utilities for solving Advent of Code problems in rust.

/// Utilities for dealing with input strings.
pub mod input;

/// It was on the seventh day of Advent of Code, 2022 that Ben learned that
/// tree structures are hard in Rust. Here are some tools for working with
/// trees that are ~borrowed~ and adapted from:
/// <https://dev.to/deciduously/no-more-tears-no-more-knots-arena-allocated-trees-in-rust-44k6>
pub mod tree;

/// Implementations of 2D mazes.
///
/// The `HashMapOccupiedMaze` combined with the `Block` enum can be used to represent a 2D maze
/// with a single "occupant" of the maze.
///
/// # Examples
/// ```
/// use aoc_utils::maze::*;
///
/// let mut maze = HashMapMaze::new();
///
/// maze.add_loc((0,0), Block::Path('.'));
/// maze.add_loc((0,1), Block::Path('.'));
/// maze.add_loc((0,2), Block::Wall('x'));
///
/// let mut maze = HashMapOccupiedMaze::from_hash_map_maze(maze, (0,0)).unwrap();
/// maze.add_loc((1,1), Block::Path('$'));
///
/// // Maze now looks like this:
/// //
/// //         x
/// //         .$
/// //         .
/// //
/// maze.move_up().unwrap();
/// maze.move_right().unwrap();
///
/// match maze.get_value() {
///     Block::Path('$') => println!("We found the treasure!"),
///     _ => panic!()
/// };
/// ```
pub mod maze;
