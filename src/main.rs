#![allow(unused_comparisons)]
// This file contains the main entry point for the Game of Life program.

// Define the game module, which contains the implementation of the Game struct.
mod game;

use std::collections::HashSet;
// Alias for the Game struct to simplify code readability.
use game::Game;
type GameType = Game;

// External crate dependencies required for graphics and window management.
extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;
extern crate rand;

use glutin_window::GlutinWindow as Window;
use graphics::{Context, Graphics, Transformed};
use opengl_graphics::{GlGraphics, OpenGL};
use piston::event_loop::{EventSettings, Events};
use piston::input::{RenderArgs, RenderEvent, UpdateArgs, UpdateEvent};
use piston::{Button, ButtonEvent, ButtonState, EventLoop, MouseButton};
use piston::window::WindowSettings;
use rand::random;
use crate::game::GridCoords;

const TARGET_FPS: u64 = 100;
const GRID_SIZE: (i32, i32) = (200, 150);
const CELL_SIZE: f64 = 3.0;
const BLACK: [f32; 4] = [0.0, 0.0, 0.0, 1.0];
pub struct App {
    gl: GlGraphics, // OpenGL drawing backend.
    game: GameType
}

impl App {

    fn draw_grid<G>(
        live_cells: &HashSet<GridCoords>,
        cell_size: f64,
        c: Context,
        gl: &mut G,
        square: graphics::types::Rectangle,
    ) where
        G: Graphics,
    {
        let colors: Vec<[f32; 4]> =
            (0..live_cells.len())
                .map(|_| [random(), random(), random(), 1.0])
                .collect();
        let transform_base = c.transform;
        for (&coords, &color) in live_cells.iter().zip(colors.iter()) {
            let x_move = coords.0 as f64 * cell_size;
            let y_move = coords.1 as f64 * cell_size;
            let transform = transform_base.trans(x_move, y_move);
            rectangle(color, square, transform, gl)
        }
    }

    // Function for rendering the grid based on live cells and handling OpenGL drawing.
    fn render(&mut self, _args: &RenderArgs) {
        use graphics::*;

        let square = rectangle::square(0.0, 0.0, self.game.cell_size);
        let cell_size = self.game.cell_size;
        let live_cells = &self.game.live_cells;

        self.gl.draw(_args.viewport(), |c, gl| {
            App::draw_grid(live_cells, cell_size, c, gl, &mut self.batch);
            clear(BLACK, gl);
            App::draw_grid(live_cells, cell_size, c, gl, square);
        });
    }

    fn update(&mut self, _args: &UpdateArgs) {
        self.game.count_neighbors();
        self.game.apply_rules();
    }
}

// Function to initialize the game, create the window, and run the game loop.
fn run_the_game(game: GameType) {
// Change this to OpenGL::V2_1 if not working.
    let opengl = OpenGL::V3_2;

    let mut window: Window = WindowSettings::new(
        "Game of Life",
        [game.grid_size.0 as f64 * game.cell_size, game.grid_size.1 as f64 * game.cell_size]
    )
        .graphics_api(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();

    let mut app = App {
        gl: GlGraphics::new(opengl),
        game
    };

    let mut events = Events::new(EventSettings::new().max_fps(TARGET_FPS));
    while let Some(e) = events.next(&mut window) {
        if let Some(args) = e.button_args() {
            if args.button == Button::Mouse(MouseButton::Left) && args.state == ButtonState::Press {
                app.game = Game::initialize(GRID_SIZE, CELL_SIZE);
                app.game.create_neighbor_map()
            }
        }

        if let Some(args) = e.render_args() {
            app.render(&args);
        }

        if let Some(args) = e.update_args() {
            app.update(&args);
        }
    }
}

// The main entry point of the program, initializes the game and starts the game loop.
fn main() {
    println!("Starting!");
    let mut game: GameType = Game::initialize(GRID_SIZE, CELL_SIZE);
    game.create_neighbor_map(); // TODO: Initialize the map without this call
    run_the_game(game);
}
