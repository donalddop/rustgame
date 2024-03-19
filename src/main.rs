#![allow(unused_comparisons)]

mod game;

use std::collections::HashSet;
use game::Game;
type GameType = Game;

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

const TARGET_FPS: u64 = 144;
const GRID_SIZE: (i32, i32) = (100, 150);
const CELL_SIZE: f64 = 3.0;
const BLACK: [f32; 4] = [0.0, 0.0, 0.0, 1.0];
struct Batch {
    colors: Vec<[f32; 4]>,
    transforms: Vec<graphics::math::Matrix2d>,
}
pub struct App {
    gl: GlGraphics, // OpenGL drawing backend.
    game: GameType,
    batch: Batch,
}

impl App {

    fn draw_grid<G>(
        live_cells: &HashSet<GridCoords>,
        cell_size: f64,
        c: Context,
        _gl: &mut G,
        batch: &mut Batch,

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
            batch.colors.push(color);
            batch.transforms.push(transform);
        }
    }

    fn render(&mut self, _args: &RenderArgs) {
        use graphics::*;

        let square = rectangle::square(0.0, 0.0, self.game.cell_size);
        let cell_size = self.game.cell_size;
        let live_cells = &self.game.live_cells;

        self.batch.colors.clear();
        self.batch.transforms.clear();

        self.gl.draw(_args.viewport(), |c, gl| {
            App::draw_grid(live_cells, cell_size, c, gl, &mut self.batch);
            clear(BLACK, gl);
            for (&color, &transform) in self.batch.colors.iter().zip(self.batch.transforms.iter()) {
                rectangle(color, square, transform, gl);
            }
        });
    }

    fn update(&mut self, _args: &UpdateArgs) {
        self.game.count_neighbors();
        self.game.apply_rules();
    }
}

fn run_the_game(game: GameType) {
// Change this to OpenGL::V2_1 if not working.
    let opengl = OpenGL::V3_2;

    // Create a Glutin window.
    let mut window: Window = WindowSettings::new(
        "Game of Life",
        [game.grid_size.0 as f64 * game.cell_size, game.grid_size.1 as f64 * game.cell_size]
    )
        .graphics_api(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();

    // Create a new game and run it.
    let mut app = App {
        gl: GlGraphics::new(opengl),
        game,
        batch: Batch { colors: vec![], transforms: vec![] },
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

fn main() {
    println!("Starting!");
    let mut game: GameType = Game::initialize(GRID_SIZE, CELL_SIZE);
    game.create_neighbor_map(); // TODO: Initialize the map without this call
    run_the_game(game);
}
