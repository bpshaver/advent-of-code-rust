#![warn(missing_docs)]
use std::collections::HashMap;
use std::hash::Hash;

/// Error type for 2D mazes and associated functions.
#[derive(Debug)]
pub enum MazeError {
    /// Raised when a move in a 2D maze is impossible
    ImpossibleMove,
    /// Raised when a location in a 2D maze does not exist
    LocationDoesNotExist,
}

/// This trait describes a navigable maze, where each generic `Self::Location` type has optionally has a
/// loc above, below, to the right, and to the left of it.
pub trait NavigableMaze {
    /// Represents a location in a 2D maze.
    type Location;

    /// Find the location in the 2D maze above `loc`
    fn loc_above(&self, loc: Self::Location) -> Result<Self::Location, MazeError>;
    /// Find the location in the 2D maze below `loc`
    fn loc_below(&self, loc: Self::Location) -> Result<Self::Location, MazeError>;
    /// Find the location in the 2D maze to the right of `loc`
    fn loc_right(&self, loc: Self::Location) -> Result<Self::Location, MazeError>;
    /// Find the location in the 2D maze to the left of `loc`
    fn loc_left(&self, loc: Self::Location) -> Result<Self::Location, MazeError>;
}

/// This supertrait is a describes a `NavigableMaze` with state representing the current location
/// of a single occupant of the maze.
pub trait SingleOccupantMaze: NavigableMaze {
    /// Move the occupant of the maze up one
    fn move_up(&mut self) -> Result<(), MazeError>;
    /// Move the occupant of the maze down one
    fn move_down(&mut self) -> Result<(), MazeError>;
    /// Move the occupant of the maze to the right one
    fn move_right(&mut self) -> Result<(), MazeError>;
    /// Move the occupant of the maze to the left one
    fn move_left(&mut self) -> Result<(), MazeError>;
}

/// A 2D maze where each location and value in the maze is stored in a `HashMap`. The idea here is
/// that each key in the hashmap uniquely identifies a location in a 2D grid. Therefore, a natural
/// choice for the generic type `L` is `(usize, usize)`
pub struct HashMapMaze<L, V>
where
    L: Eq + Hash,
{
    map: HashMap<L, V>,
}

/// Wraps a `HashMapMaze` with a current location `L` to enable a set of associated functions for
/// mutably moving around in the maze.
pub struct HashMapOccupiedMaze<L, V>
where
    L: Eq + Hash,
{
    maze: HashMapMaze<L, V>,
    loc: L,
}

impl<L, V> HashMapMaze<L, V>
where
    L: Hash + Eq,
{
    /// Create a new `HashMapMaze` with no locations in it.
    pub fn new() -> Self {
        let map = HashMap::new();
        Self { map }
    }

    /// Add a new location to the maze.
    pub fn add_loc(&mut self, loc: L, value: V) {
        self.map.insert(loc, value);
    }

    /// Get the value of the maze at `loc`
    pub fn get_value_at_loc(&self, loc: &L) -> Result<&V, MazeError> {
        match self.map.get(&loc) {
            None => Err(MazeError::LocationDoesNotExist),
            Some(value) => Ok(value),
        }
    }
}

impl<L, V> HashMapOccupiedMaze<L, V>
where
    L: Hash + Eq,
{
    /// Create a `HashMapOccupiedMaze` from a pre-existing `HashMapMaze` by providing the location
    /// of the occupant of the maze.
    pub fn from_hash_map_maze(maze: HashMapMaze<L, V>, loc: L) -> Result<Self, MazeError> {
        if !maze.map.contains_key(&loc) {
            Err(MazeError::LocationDoesNotExist)
        } else {
            Ok(HashMapOccupiedMaze { maze, loc })
        }
    }

    /// Get the value of the maze at the current location.
    pub fn get_value(&self) -> &V {
        self.maze
            .get_value_at_loc(&self.loc)
            .expect("self.loc is always valid")
    }
}

impl<V> NavigableMaze for HashMapMaze<(usize, usize), V> {
    /// Represents a location in a 2D maze.
    type Location = (usize, usize);

    /// Find the location in the 2D maze above `loc`
    fn loc_above(&self, loc: Self::Location) -> Result<Self::Location, MazeError> {
        let other_loc = (loc.0, loc.1 + 1);
        if self.map.contains_key(&other_loc) {
            Ok(other_loc)
        } else {
            Err(MazeError::LocationDoesNotExist)
        }
    }

    /// Find the location in the 2D maze below `loc`
    fn loc_below(&self, loc: Self::Location) -> Result<Self::Location, MazeError> {
        if loc.1 == 0 {
            return Err(MazeError::ImpossibleMove);
        }
        let other_loc = (loc.0, loc.1 - 1);
        if self.map.contains_key(&other_loc) {
            Ok(other_loc)
        } else {
            Err(MazeError::LocationDoesNotExist)
        }
    }
    /// Find the location in the 2D maze to the right of `loc`
    fn loc_right(&self, loc: Self::Location) -> Result<Self::Location, MazeError> {
        let other_loc = (loc.0 + 1, loc.1);
        if self.map.contains_key(&other_loc) {
            Ok(other_loc)
        } else {
            Err(MazeError::LocationDoesNotExist)
        }
    }
    /// Find the location in the 2D maze to the left of `loc`
    fn loc_left(&self, loc: Self::Location) -> Result<Self::Location, MazeError> {
        if loc.0 == 0 {
            return Err(MazeError::ImpossibleMove);
        }
        let other_loc = (loc.0 - 1, loc.1);
        if self.map.contains_key(&other_loc) {
            Ok(other_loc)
        } else {
            Err(MazeError::LocationDoesNotExist)
        }
    }
}

impl<V> NavigableMaze for HashMapOccupiedMaze<(usize, usize), V> {
    type Location = (usize, usize);
    /// Find the location in the 2D maze above `loc`
    fn loc_above(&self, loc: Self::Location) -> Result<Self::Location, MazeError> {
        self.maze.loc_above(loc)
    }
    /// Find the location in the 2D maze below `loc`
    fn loc_below(&self, loc: Self::Location) -> Result<Self::Location, MazeError> {
        self.maze.loc_below(loc)
    }
    /// Find the location in the 2D maze to the right of `loc`
    fn loc_right(&self, loc: Self::Location) -> Result<Self::Location, MazeError> {
        self.maze.loc_right(loc)
    }
    /// Find the location in the 2D maze to the left of `loc`
    fn loc_left(&self, loc: Self::Location) -> Result<Self::Location, MazeError> {
        self.maze.loc_left(loc)
    }
}

/// Enum for defining locations in a maze which are accessible or inaccessible.
#[derive(Debug)]
pub enum Block<P, W> {
    /// An accessible location containing an object of type `P`
    Path(P),
    /// An inaccessible location containing an object of type `W`
    Wall(W),
}

impl<P, W> SingleOccupantMaze for HashMapOccupiedMaze<(usize, usize), Block<P, W>> {
    fn move_up(&mut self) -> Result<(), MazeError> {
        let loc = self.loc_above(self.loc)?;
        match self.maze.get_value_at_loc(&loc)? {
            Block::Path(_) => {
                self.loc = loc;
                Ok(())
            }
            Block::Wall(_) => Err(MazeError::ImpossibleMove),
        }
    }
    fn move_down(&mut self) -> Result<(), MazeError> {
        let loc = self.loc_below(self.loc)?;
        match self.maze.get_value_at_loc(&loc)? {
            Block::Path(_) => {
                self.loc = loc;
                Ok(())
            }
            Block::Wall(_) => Err(MazeError::ImpossibleMove),
        }
    }
    fn move_right(&mut self) -> Result<(), MazeError> {
        let loc = self.loc_right(self.loc)?;
        match self.maze.get_value_at_loc(&loc)? {
            Block::Path(_) => {
                self.loc = loc;
                Ok(())
            }
            Block::Wall(_) => Err(MazeError::ImpossibleMove),
        }
    }
    fn move_left(&mut self) -> Result<(), MazeError> {
        let loc = self.loc_left(self.loc)?;
        match self.maze.get_value_at_loc(&loc)? {
            Block::Path(_) => {
                self.loc = loc;
                Ok(())
            }
            Block::Wall(_) => Err(MazeError::ImpossibleMove),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hash_map_occupied_maze_basic() {
        let mut maze = HashMapMaze::new();
        maze.add_loc((0, 0), "start");
        let mut maze = HashMapOccupiedMaze::from_hash_map_maze(maze, (0, 0)).unwrap();
        maze.maze.add_loc((1, 0), "one_right");
        assert_eq!(maze.get_value(), &"start");
        assert_eq!(
            maze.maze
                .get_value_at_loc(&maze.loc_right(maze.loc).unwrap())
                .unwrap(),
            &"one_right"
        );
    }

    #[test]
    fn hash_map_maze_impl_navigable_maze() {
        let mut maze = HashMapMaze::new();
        for i in 0..10 {
            for j in 0..10 {
                maze.add_loc((i, j), ".")
            }
        }
        assert_eq!(maze.loc_above((5, 5)).unwrap(), (5, 6));
        assert_eq!(maze.loc_below((5, 5)).unwrap(), (5, 4));
        assert_eq!(maze.loc_right((5, 5)).unwrap(), (6, 5));
        assert_eq!(maze.loc_left((5, 5)).unwrap(), (4, 5));
    }

    #[test]
    fn hash_map_maze_impl_navigable_maze_panics() {
        let mut maze = HashMapMaze::new();
        for i in 0..10 {
            for j in 0..10 {
                maze.add_loc((i, j), ".")
            }
        }
        assert_eq!(maze.loc_below((0, 0)).unwrap_or((666, 666)), (666, 666));
        assert_eq!(maze.loc_left((0, 0)).unwrap_or((666, 666)), (666, 666));
        assert_eq!(maze.loc_above((9, 9)).unwrap_or((666, 666)), (666, 666));
        assert_eq!(maze.loc_right((9, 9)).unwrap_or((666, 666)), (666, 666));
    }

    #[test]
    fn hash_map_occupied_maze_traverse_chars() {
        let mut maze = HashMapMaze::new();
        maze.add_loc((0, 0), Block::Path('.'));
        maze.add_loc((0, 1), Block::Path('.'));
        maze.add_loc((0, 2), Block::Path('.'));
        maze.add_loc((1, 0), Block::Path('.'));
        maze.add_loc((1, 1), Block::Wall('x'));
        maze.add_loc((1, 2), Block::Path('.'));
        maze.add_loc((2, 0), Block::Path('.'));
        maze.add_loc((2, 1), Block::Path('.'));
        maze.add_loc((2, 2), Block::Path('.'));
        let mut maze = HashMapOccupiedMaze::from_hash_map_maze(maze, (0, 0)).unwrap();
        maze.move_up().unwrap();
        maze.move_up().unwrap();
        assert_eq!(maze.loc, (0, 2));
        maze.move_right().unwrap();
        maze.move_right().unwrap();
        assert_eq!(maze.loc, (2, 2));
        maze.move_down().unwrap();
        maze.move_down().unwrap();
        assert_eq!(maze.loc, (2, 0));
        maze.move_left().unwrap();
        assert_eq!(maze.loc, (1, 0));
        let mut move_failed = false;
        maze.move_up().unwrap_or_else(|_| {
            move_failed = true;
            ()
        });
        assert!(move_failed)
    }
}
