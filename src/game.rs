use std::collections::HashMap;
use rand::Rng;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cell {
    Dead = 0,
    Alive = 1
}

impl Cell {
    pub(crate) fn kill(&mut self) {
        *self = Cell::Dead
    }
    fn live(&mut self) {
        *self = Cell::Alive
    }
}

pub struct Game {
    pub(crate) grid_size: (i32, i32),
    pub(crate) cell_size: f64,
    pub(crate) grid: Vec<Vec<Cell>>,
    neighbors_map: HashMap<(usize, usize), u8>
}

impl Game {
    pub fn initialize(grid_size: (i32, i32), cell_size: f64) -> Self {
        let neighbors_map: HashMap<(usize, usize), u8> = HashMap::new();
        let grid = Self::create_random_grid(grid_size);
        Game { grid_size, cell_size, grid, neighbors_map }
    }

    pub fn create_random_grid(grid_size: (i32, i32)) -> Vec<Vec<Cell>> {
        let mut grid = Vec::new();
        for _ in 0..grid_size.1 {
            let mut row = Vec::new();
            for _ in 0..grid_size.0 {
                row.push(match rand::thread_rng().gen::<bool>() {
                    true => Cell::Alive,
                    false => Cell::Dead
                });
            }
            grid.push(row);
        }
        grid
    }

    pub fn count_neighbors(&mut self) {
        for col in 0..self.grid_size.0 as usize {
            for row in 0..self.grid_size.1 as usize {
                let neighbors_to_check: Vec<(u8, u8)> = self.get_neighbors(col, row);
                let mut positive_neighbors: u8 = 0;
                for pos in neighbors_to_check {
                    if self.grid[pos.0 as usize][pos.1 as usize] == Cell::Alive {
                        positive_neighbors += 1;
                    }
                }
                self.neighbors_map.insert((col, row), positive_neighbors);
            }
        }
    }

    fn get_neighbors(&mut self, input_col: usize, input_row: usize) -> Vec<(u8, u8)> {
        let mut neighbors_to_check: Vec<(u8, u8)> = Vec::new();
        let mut rows: Vec<i32> = vec![-1, 0, 1];
        let mut cols: Vec<i32> = vec![-1, 0, 1];

        if input_col == 0 {
            cols.remove(0);
            cols.push(self.grid_size.0 - 1);
        }
        else if input_col == self.grid_size.0 as usize - 1 {
            cols.remove(cols.len() - 1);
            cols.push(-(self.grid_size.0 - 1));
        }
        if input_row == 0 {
            rows.remove(0);
            rows.push(self.grid_size.1 - 1);
        }
        else if input_row == self.grid_size.1 as usize - 1 {
            rows.remove(rows.len() - 1);
            rows.push(-(self.grid_size.1 - 1));
        }
        for row in rows {
            for col in &cols {
                if (row, col) != (0, &0) {
                    neighbors_to_check.push(((input_col as i32 + col) as u8, (input_row as i32 + row) as u8))
                }
            }
        }
        neighbors_to_check
    }

    pub fn apply_rules(&mut self) {
        for col in 0..self.grid_size.0 as usize {
            for row in 0..self.grid_size.1 as usize {
                let cell: Cell = self.grid[col][row];
                let alive_neighbors = self.neighbors_map[&(col, row)];
                match cell {
                    Cell::Alive => {
                        if alive_neighbors < 2 {
                            self.grid[col][row].kill();
                        }
                        else if alive_neighbors > 3 {
                            self.grid[col][row].kill();
                        }
                    }
                    Cell::Dead => {
                        if alive_neighbors == 3 {
                            self.grid[col][row].live()
                        }
                    }
                }
            }
        }
    }
}
