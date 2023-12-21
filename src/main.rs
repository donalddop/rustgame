extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;
extern crate rand;

use std::collections::HashMap;
use glutin_window::GlutinWindow as Window;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::event_loop::{EventSettings, Events};
use piston::input::{RenderArgs, RenderEvent, UpdateArgs, UpdateEvent};
use piston::window::WindowSettings;
use rand::Rng;

pub struct Grid {
    grid_size: (u32, u32),
    cell_size: f64,
    grid: Vec<Vec<bool>>,
    // neighbors_map: HashMap<(usize, usize), u32>
}

impl Grid {
    pub fn new(grid_size: (u32, u32), cell_size: f64) -> Self {
        let mut grid = Vec::new();
        for _ in 0..grid_size.1 {
            let mut row = Vec::new();
            for _ in 0..grid_size.0 {
                row.push(rand::thread_rng().gen::<bool>());
            }
            grid.push(row);
        }
        Grid { grid_size, cell_size, grid }
    }

    // pub fn count_neighbors(&self) {
    //     let mut neighbors_map: HashMap<(usize, usize), u32> = HashMap::new();
    //     let grid_size = self.grid_size;
    //     let grid = &self.grid;
    //
    //     for i in 0..grid_size.0 {
    //         for j in 0..grid_size.1 {
    //             let mut positive_neighbors = 0;
    //
    //             for x in (i - 1)..(i + 2) {
    //                 for y in (j - 1)..(j + 2) {
    //                     if x < grid_size.0 && x >= 0 && y < grid_size.1 && y >= 0 {
    //                         if grid[x as usize][y as usize] {
    //                             positive_neighbors += 1;
    //                         }
    //                     }
    //                 }
    //             }
    //             let (x, y) = (i, j);
    //             neighbors_map.insert((x as usize, y as usize), positive_neighbors);
    //         }
    //     }
    //     neighbors_map
    // }
}

pub struct App {
    gl: GlGraphics, // OpenGL drawing backend.
    rotation: f64,  // Rotation for the square.
    grid: Grid
}

impl App {

    fn render(&mut self, args: &RenderArgs) {
        use graphics::*;

        const GREEN: [f32; 4] = [0.0, 1.0, 0.0, 1.0];
        const RED: [f32; 4] = [1.0, 0.0, 0.0, 1.0];

        let square = rectangle::square(0.0, 0.0, self.grid.cell_size);
        // let (x, y) = (0.0, 0.0);
        // let (window_width, window_height) = (args.window_size[0], args.window_size[1]);
        let grid_size = self.grid.grid_size;
        let cell_size = self.grid.cell_size;
        let grid = &self.grid.grid;

        self.gl.draw(args.viewport(), |c, gl| {
            // Clear the screen.
            clear(GREEN, gl);
            let mut transform;
            let mut x_move;
            let mut y_move = 0.0;
            for i in 0..grid_size.1 {
                let mut curr_y_move = y_move;
                x_move = 0.0;
                for j in 0..grid_size.0 {
                    transform = c
                        .transform
                        .trans(x_move, curr_y_move);
                    match grid[i as usize][j as usize] {
                        true => rectangle(RED, square, transform, gl),
                        false => rectangle(GREEN, square, transform, gl)
                    }
                    x_move += cell_size;
                }
                curr_y_move += cell_size;
                y_move = curr_y_move;

            }
        });
    }

    fn update(&mut self, args: &UpdateArgs) {
        // Rotate 2 radians per second.
        self.rotation += 2.0 * args.dt;
        // self.grid.count_neighbors();
    }
}
type GridType = Grid;
fn main() {
    let grid: GridType = Grid::new((10, 10), 30.0);
    run_the_game(grid);
}

fn run_the_game(grid: Grid) {
// Change this to OpenGL::V2_1 if not working.
    let opengl = OpenGL::V3_2;

    // Create a Glutin window.
    let mut window: Window = WindowSettings::new(
        "spinning-square",
        [grid.grid_size.0 as f64 * grid.cell_size, grid.grid_size.1 as f64 * grid.cell_size]
    )
        .graphics_api(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();

    // Create a new game and run it.
    let mut app = App {
        gl: GlGraphics::new(opengl),
        rotation: 0.0,
        grid
    };

    let mut events = Events::new(EventSettings::new());
    while let Some(e) = events.next(&mut window) {
        if let Some(args) = e.render_args() {
            app.render(&args);
        }

        if let Some(args) = e.update_args() {
            app.update(&args);
        }
    }
}
