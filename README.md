# Conway's Game of Life (Rust & Piston Implementation)

This project is an implementation of Conway's Game of Life, built using the Rust programming language and the Piston game engine. It simulates the classic cellular automaton with a graphical visualization.

## Features

*   **Random World Generation:** Start with a randomly generated grid of live and dead cells.
*   **Toroidal Grid:** The grid wraps around, meaning cells on the edges interact with cells on the opposite side, creating a seamless, endless world.
*   **Interactive Reset:** Generate a new random world with a single click.

## Prerequisites

*   **Rust:** The Rust programming language and its package manager, Cargo.
    *   You can install Rust by following the instructions on [rust-lang.org](https://www.rust-lang.org/tools/install).

## Installation

1.  Clone the repository:
    ```bash
    git clone <repository_url>
    cd rustgame
    ```

## Usage

1.  Build and run the game using Cargo:
    ```bash
    cargo run
    ```
    This command will compile your project and then execute it. A new window will appear displaying the Game of Life simulation.

## Controls

*   **Left Mouse Button:** Click to generate a new random world.
*   **ESC:** Press the Escape key to exit the application.

## Tech Stack

*   **Rust:** The primary programming language.
*   **Piston Game Engine:** A modular game engine for Rust, used for graphics and window management.
    *   `pistoncore-glutin_window`
    *   `piston2d-graphics`
    *   `piston2d-opengl_graphics`
    *   `rand`
