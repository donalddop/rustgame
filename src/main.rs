#![allow(unused_comparisons)]

mod game;
use game::Cell;
use game::Game;
type GameType = Game;

extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;
extern crate rand;

use glutin_window::GlutinWindow as Window;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::event_loop::{EventSettings, Events};
use piston::input::{RenderArgs, RenderEvent, UpdateArgs, UpdateEvent};
use piston::{Button, ButtonEvent, ButtonState, MouseButton};
use piston::window::WindowSettings;
const TARGET_FPS: f64 = 144.0;
const GRID_SIZE: (i32, i32) = (200, 200);
const CELL_SIZE: f64 = 3.0;

pub struct App {
    gl: GlGraphics, // OpenGL drawing backend.
    game: GameType
}

impl App {

    fn render(&mut self, _args: &RenderArgs) {
        use graphics::*;
        const WHITE: [f32; 4] = [1.0, 1.0, 1.0, 1.0];
        const BLACK: [f32; 4] = [0.0, 0.0, 0.0, 1.0];

        let square = rectangle::square(0.0, 0.0, self.game.cell_size);
        let cell_size = self.game.cell_size;
        let grid = &self.game.grid;

        self.gl.draw(_args.viewport(), |c, gl| {
            // Clear the screen.
            clear(BLACK, gl);
            // Draw each of the cells
            let mut transform;
            let mut x_move;
            let mut y_move = 0.0;
            for row in grid {
                let mut curr_y_move = y_move;
                x_move = 0.0;
                for cell in row {
                    transform = c
                        .transform
                        .trans(x_move, curr_y_move);
                    if Cell::Alive.eq(cell) {
                        rectangle(WHITE, square, transform, gl)
                    }
                    x_move += cell_size;
                }
                curr_y_move += cell_size;
                y_move = curr_y_move;
            }
        });
    }

    fn update(&mut self, _args: &UpdateArgs) {
        // self.rotation += 2.0 * args.dt;
        self.game.count_neighbors();
        self.game.apply_rules();

    }
}

fn main() {
    println!("Starting!");
    let mut game: GameType = Game::initialize(GRID_SIZE, CELL_SIZE);
    game.create_neighbor_map(); // TODO: Initialize the map without this call
    run_the_game(game);
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
        game
    };

    let mut events = Events::new(EventSettings::new());
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
            std::thread::sleep(std::time::Duration::from_secs_f64(1.0 / TARGET_FPS));
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cell_is_dead_after_kill() {
        let mut cell: Cell = Cell::Alive;
        cell.kill();
        assert_eq!(Cell::Dead, cell);
    }
}