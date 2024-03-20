use std::collections::{HashMap, HashSet};
use rand::{random};
pub(crate) type GridCoords = (usize, usize);
type GridSize = (i32, i32);


// Represents the game state and logic for the Game of Life.
pub struct Game {
    pub(crate) grid_size: GridSize,
    pub(crate) cell_size: f64,
    pub(crate) live_cells: HashSet<GridCoords>,
    neighbors_count: HashMap<GridCoords, u8>,
    neighbors_map: HashMap<GridCoords,  Vec<(usize, usize)>>
}

impl Game {
    // Method to initialize the game with a grid size and cell size, creating a random initial grid.
    pub fn initialize(grid_size: GridSize, cell_size: f64) -> Self {
        let neighbors_count: HashMap<GridCoords, u8> = HashMap::new();
        let neighbors_map: HashMap<GridCoords, Vec<(usize, usize)>> = HashMap::new();
        let live_cells = Self::create_random_grid(grid_size);
        Game { grid_size, cell_size, live_cells, neighbors_count, neighbors_map}
    }

    // Method to create a random grid.
    pub fn create_random_grid(grid_size: GridSize) -> HashSet<GridCoords> {
        (0..grid_size.1 as usize)
            .flat_map(|row| {
                (0..grid_size.0 as usize)
                    .filter(|_| random())
                    .map(move |col| (col, row))
            })
            .collect()
    }

    // Method to pre-calculate the neighbors of each cell in the grid for faster neighbor counting.
    pub fn create_neighbor_map(&mut self) {
        let grid_size = self.grid_size;
        for col in 0..grid_size.0 as usize {
            for row in 0..grid_size.1 as usize {
                let neighbors: Vec<(usize, usize)> = self.get_neighbors(col, row);
                self.neighbors_map.insert((col, row), neighbors);
            }
        }
    }

    pub fn count_neighbors(&mut self) {
        for col in 0..self.grid_size.0 as usize {
            for row in 0..self.grid_size.1 as usize {
                if let Some(neighbors_to_check) = self.neighbors_map.get(&(col, row)) {
                    let mut positive_neighbors: u8 = 0;
                    for pos in neighbors_to_check {
                        if self.live_cells.contains(&(pos.0, pos.1)) {
                            positive_neighbors += 1;
                        }
                    }
                    self.neighbors_count.insert((col, row), positive_neighbors);
                }
            }
        }
    }

    // Helper method to retrieve the neighboring cells of a given cell.
    fn get_neighbors(&mut self, input_col: usize, input_row: usize) -> Vec<(usize, usize)> {
        let mut neighbors_to_check: Vec<(usize, usize)> = Vec::new();
        let rows: Vec<i32> = vec![-1, 0, 1];
        let cols: Vec<i32> = vec![-1, 0, 1];

        let wrap_position = |position: i32, size: i32| -> usize {
            ((position + size) % size) as usize
        };

        for row in &rows {
            for col in &cols {
                if *row != 0 || *col != 0 {
                    let new_col = wrap_position(input_col as i32 + col, self.grid_size.0);
                    let new_row = wrap_position(input_row as i32 + row, self.grid_size.1);

                    // Check if the calculated indices are within bounds
                    if new_col < self.grid_size.0 as usize && new_row < self.grid_size.1 as usize {
                        if !neighbors_to_check.contains(&(new_col, new_row)) {
                            neighbors_to_check.push((new_col, new_row));
                        }
                    } else {
                        println!("Index out of bounds: new_col={}, new_row={}", new_col, new_row);
                    }
                }
            }
        }
        neighbors_to_check
    }

    // Method to apply the rules of the Game of Life to update the game state.
    pub fn apply_rules(&mut self) {
        let grid_size_col = self.grid_size.0 as usize;
        let grid_size_row = self.grid_size.1 as usize;

        for col in 0..grid_size_col {
            for row in 0..grid_size_row {
                let is_alive = self.live_cells.contains(&(col, row));
                let alive_neighbors = self.neighbors_count[&(col, row)];
                match is_alive {
                    true => {
                        if alive_neighbors <2 || alive_neighbors > 3 {
                            self.live_cells.remove(&(col, row));
                        }
                    }
                    false => {
                        if alive_neighbors == 3 {
                            self.live_cells.insert((col, row));
                        }
                    }
                }
            }
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_apply_rules() {
        let mut game = Game::initialize((1, 1), 1.0);
        let coords: GridCoords = (0, 0);

        game.live_cells.insert((0, 0)); // Living cell
        game.neighbors_count.insert((0, 0), 1); // Set up neighbors count
        game.apply_rules();
        assert_eq!(game.live_cells.contains(&coords), true);

        game.live_cells.insert((0, 0)); // Living cell
        game.neighbors_count.insert((0, 0), 2); // Set up neighbors count
        game.apply_rules();
        assert_eq!(game.live_cells.contains(&coords), true);

        game.live_cells.insert((0, 0)); // Living cell
        game.neighbors_count.insert((0, 0), 4); // Set up neighbors count
        game.apply_rules();
        assert_eq!(game.live_cells.contains(&coords), false);

        game.live_cells.remove(&(0, 0)); // Dead cell
        game.neighbors_count.insert((0, 0), 3); // Set up neighbors count
        game.apply_rules();
        assert_eq!(game.live_cells.contains(&coords), true);
    }

    #[test]
    fn test_get_neighbors() {
        let mut game = Game::initialize((3, 3), 1.0);
        let neighbors = game.get_neighbors(0, 0);
        assert_eq!(
            neighbors,
            vec![(0, 1), (0, 2), (1, 0), (1, 1), (1, 2), (2, 0), (2, 1), (2, 2)]
                .into_iter()
                .map(|(col, row)| (col as u32, row as u32))
                .collect::<Vec<(u32, u32)>>()
        );

        let neighbors = game.get_neighbors(2, 2);
        assert_eq!(
            neighbors,
            vec![(0, 0), (0, 1), (0, 2), (1, 0), (1, 1), (1, 2), (2, 0), (2, 1)]
                .into_iter()
                .map(|(col, row)| (col as u32, row as u32))
                .collect::<Vec<(u32, u32)>>()
        );

        let mut game = Game::initialize((4, 3), 1.0);
        let neighbors = game.get_neighbors(0, 0);
        assert_eq!(
            neighbors,
            vec![(0, 1), (0, 2), (1, 0), (1, 1), (1, 2), (3, 0), (3, 1), (3, 2)]
                .into_iter()
                .map(|(col, row)| (col as u32, row as u32))
                .collect::<Vec<(u32, u32)>>(),
            "For cell (0, 0)"
        );

        let mut game = Game::initialize((3, 4), 1.0);
        let neighbors = game.get_neighbors(0, 0);
        assert_eq!(
            neighbors,
            vec![(0, 1), (0, 3), (1, 0), (1, 1), (1, 3), (2, 0), (2, 1), (2, 3)]
                .into_iter()
                .map(|(col, row)| (col as u32, row as u32))
                .collect::<Vec<(u32, u32)>>(),
            "For cell (0, 0)"
        );
    }

    #[test]
    fn test_count_neighbors() {
        let mut game = Game::initialize((3, 4), 1.0);
        game.live_cells.clear(); // Kill all cells

        // Set up the grid with specific cells alive
        game.live_cells.insert((0, 1));
        game.live_cells.insert((1, 0));

        game.create_neighbor_map();
        game.count_neighbors();

        assert_eq!(game.neighbors_count[&(0, 0)], 2, "For cell (0, 0)");
        assert_eq!(game.neighbors_count[&(0, 1)], 1, "For cell (0, 1)");
        assert_eq!(game.neighbors_count[&(0, 2)], 1, "For cell (0, 2)");
        assert_eq!(game.neighbors_count[&(0, 3)], 1, "For cell (0, 3)");
        assert_eq!(game.neighbors_count[&(1, 0)], 1, "For cell (1, 0)");
        assert_eq!(game.neighbors_count[&(1, 1)], 2, "For cell (1, 1)");
        assert_eq!(game.neighbors_count[&(1, 2)], 1, "For cell (1, 2)");
        assert_eq!(game.neighbors_count[&(1, 3)], 1, "For cell (1, 3)");
        assert_eq!(game.neighbors_count[&(2, 0)], 2, "For cell (2, 0)");
        assert_eq!(game.neighbors_count[&(2, 1)], 2, "For cell (2, 1)");
        assert_eq!(game.neighbors_count[&(2, 2)], 1, "For cell (2, 2)");
        assert_eq!(game.neighbors_count[&(2, 3)], 1, "For cell (2, 3)");
    }
}