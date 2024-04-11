/*****************************************************************/
//! [Conway's Game of Life]
/*****************************************************************/
//!
//! Parallel implementation of John Conway's 1970 "Game of Life".
//! Takes advantage of the Rayon Crate for automagically managed
//! parallel iterators, as drop-in replacements for standard
//! Rust iterators.
//!
//! All graphics are generated using OpenGL with help from
//! Rust's Piston API. Currently, each individual pixel is rendered
//! as an OpenGL shape. There would be much more noticeable
//! performance gains if this limitation were to be overcome,
//! however this was not ameliorated due to time constraints.
//!
//! [Authors]
//! Aiden Manuel (Original programming and idea),
//! Matthew Peterson (Parallel programming and optimizations)
//!
//! [Class] CS 3123, Dr. Jeff Mark McNally
//!
//! [Date] Submitted April 11, 2024
/*****************************************************************/

// Define external libraries.
extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;
extern crate rand;
extern crate chrono;
extern crate rayon;
extern crate conv;

// Import necessary functions from external libraries.
use graphics::*;
use glutin_window::GlutinWindow as Window;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::event_loop::{EventSettings, Events};
use piston::input::{RenderArgs, RenderEvent, UpdateArgs, UpdateEvent};
use piston::window::WindowSettings;
use piston::GenericEvent;
use std::time::{Instant};

// Window dimensions (in pixels), as well as
// visible scale-factor and other metrics.
const HEIGHT: usize = 1080;
const WIDTH: usize = 1920;
const SCALE: usize = 4;
const ROWS: usize = HEIGHT / SCALE;
const COLS: usize = WIDTH / SCALE;
const SIZE: usize = (ROWS) * (COLS);


/// [App]
/// The App struct defines the Piston application and associated
/// data. All fields within this structure are statically accessible
/// from within the application's associated methods.
///
/// Fields:
/// [gl] OpenGL graphics backend;
/// [state] State of the game board as a flat array of booleans;
/// [cursor_pos] Actively tracked location of the user's mouse cursor;
/// [paused] Game state.
pub struct App {
    gl: GlGraphics,
    state: [bool; SIZE],
    cursor_pos: [f64; 2],
    paused: bool
}

/// [App]
/// Application related methods.
impl App {

    /// [Render]
    /// The render method is required by Piston in order to service
    /// the application control-flow, using callbacks. The render
    /// method is specifically meant to be where all calls to OpenGL
    /// happen, and is meant to be called every frame.
    ///
    /// This program implements the render method by checking each cell
    /// of the game's state individually, and drawing the corresponding
    /// pixel upon a blank background if the cell is alive.
    ///
    /// Being a Piston callback, its only parameters are itself,
    /// and the Piston render arguments.
    fn render(&mut self, args: &RenderArgs) {

        // Local constants:
        const WHITE: [f32; 4] = [0.9, 0.9, 0.85, 1.0];
        const BLACK: [f32; 4] = [0.6, 0.5, 0.52, 1.0];

        // Local variables:
        let mut colour: [f32; 4] = WHITE;

        // The following block of code will overwrite the OpenGL window with white.
        self.gl.draw(args.viewport(), |c, gl| {
            // Create the necessary components to draw with:
            let background_fill =
                rectangle::rectangle_by_corners(0.0, 0.0, WIDTH as f64, HEIGHT as f64);
            let transform = c.transform;

            // Collect all components and write to the screen.
            rectangle(colour, background_fill, transform, gl);
        });

        // Begin iterating over all individual cells within the state array.
        colour = BLACK;
        for y in 0usize..(ROWS) {
            for x in 0usize..(COLS) {
                // We only want to draw a square to OpenGL if the cell is alive:
                if self.state[x + y * COLS] {

                    // We draw the living cell as a square, which is a data structure
                    // with 3 floating point values representing position and size.
                    let square = rectangle::square((x * SCALE) as f64, (y * SCALE) as f64, SCALE as f64);
                    self.gl.draw(args.viewport(), |c, gl| {
                        // Must update the current OpenGL transformation
                        // before drawing the pixel.
                        let transform = c.transform;
                        rectangle(colour, square, transform, gl);
                    });
                }
            }
        }
    }
    
    /// [Update]
    ///
    /// The update method is required by Piston in order to service
    /// the application logic (as opposed to rendering) using callbacks.
    /// The update method contains user-defined logic which does not
    /// necessarily have to do with drawing to OpenGL.
    ///
    /// Therefore, this method updates the game state for the current
    /// Game of Life instance by checking each individual cell from the
    /// previous state, and updating the focused cell for the next state
    /// accordingly. This method has been parallelized using the Rayon
    /// crate, in order to allow each cell to be analyzed by the next
    /// available parallel thread.
    ///
    /// Being a Piston callback, its only parameters are itself,
    /// and the Piston update arguments.
    fn update(&mut self, _args: &UpdateArgs) {
        // Only update frames if the game is un-paused.
        if !self.paused {

            // Copy the previous state for later reference. This
            // is necessary, as each cell's update relies on the
            // previous state of the board.
            let previous_state: [bool; SIZE] = self.state;
            use rayon::prelude::*;

            // Take initial time
            let time_initial = Instant::now();


            // Rayon parallel iterator:
            // .enumerate() -> Provides us with an index for each iterated value.
            //                 this is necessary for the Game of Life.
            // .for_each()  -> Iterates over each value of the parallel iterator.
            //                 Provides the index of the focused value, and a
            //                 reference to the focused value itself within its
            //                 closure (straight brackets).
            self.state.par_iter_mut()
                .enumerate()
                .for_each( |(i, pixel)| {

                    // Observe state of neighbouring cells:
                    let mut neighbour = 0;

                    neighbour += previous_state[(SIZE + i - 1 - COLS) % SIZE] as i32;
                    neighbour += previous_state[(SIZE + i - COLS) % SIZE] as i32;
                    neighbour += previous_state[(SIZE + i + 1 - COLS) % SIZE] as i32;
                    neighbour += previous_state[(SIZE + i - 1) % SIZE] as i32;
                    neighbour += previous_state[(SIZE + i + 1) % SIZE] as i32;
                    neighbour += previous_state[(SIZE + i - 1 + COLS) % SIZE] as i32;
                    neighbour += previous_state[(SIZE + i + COLS) % SIZE] as i32;
                    neighbour += previous_state[(SIZE + i + 1 + COLS) % SIZE] as i32;

                    // Based on current state, change to new state!
                    if previous_state[i] {
                        if neighbour < 2 || neighbour > 3 {
                            *pixel = !previous_state[i];
                        }
                    } else if neighbour == 3 {
                        *pixel = !previous_state[i];
                    } else {
                        *pixel = previous_state[i];
                    }
                });

            // For collecting CSV output:
            //print!("{},", now.elapsed().as_millis());

            // For demonstrative output:
            println!("Rendered in {}ms", time_initial.elapsed().as_millis());
        }
    }

    /// [Event]
    ///
    /// The event method is required by Piston in order to service
    /// user interaction using callbacks. This includes key presses,
    /// and support for mouse interaction. Such input is necessary
    /// for clearing the board, regenerating the board, and drawing
    /// directly to the board.
    
    fn event<E: GenericEvent>(&mut self, pos: [f64; 2], e: &E) {
        use piston::input::{Button, Key, MouseButton};

        // Mouse Function Added!
        // Left Click to change the flip the state of a cell
        if let Some(pos) = e.mouse_cursor_args() {
            self.cursor_pos = pos;
        }
        if let Some(Button::Mouse(MouseButton::Left)) = e.press_args() {
            // Find coordinates relative to upper left corner.
            let x = self.cursor_pos[0] - pos[0];
            let y = self.cursor_pos[1] - pos[1];
            
            // Check that coordinates are inside board boundaries.
            if x >= 0.0 && x <= WIDTH as f64 && y >= 0.0 && y <= HEIGHT as f64 {
                // Compute the cell position.
                let cell_x = (x / SCALE as f64) as usize;
                let cell_y = (y / SCALE as f64) as usize;
                // Flip the state of that cell
                self.state[cell_x + cell_y * COLS] = !self.state[cell_x + cell_y * COLS];
            }
        }

        // Key Functions
        // Space:   pause the game
        // C:       cull all living cells
        // R:       create a random starting board
        if let Some(Button::Keyboard(key)) = e.press_args() {
                let mut i = 0;
                match key {
                    Key::Space => self.paused = !self.paused,
                    Key::C => self.state = [false; SIZE],
                    Key::R => while i < SIZE { self.state[i] = rand::random(); i = i + 1; },
                    _ => {}
            }
        }
    }
}

/// [Main]
///
/// Note: Most of this main method comes from a Piston tutorial.
/// https://github.com/PistonDevelopers/Piston-Tutorials/tree/master/getting-started
///
/// This method sets up the application state, and initializes the OpenGL backend for
/// execution by Piston.

fn main() {
    // Change this to OpenGL::V2_1 if not working.
    let opengl = OpenGL::V3_2;

    // Check to make sure the command-line arguments are valid:
    use std::env;
    let args = env::args().nth(1);
    let threads = args.expect("I wasn't given an argument!").parse::<usize>().ok().expect("I wasn't given an integer!");
    rayon::ThreadPoolBuilder::new().num_threads(threads).build_global().unwrap();

    // Create a Glutin window.
    let mut window: Window = WindowSettings::new( format!("Game of Life ({} Threads) {} x {} Scale = {}", threads, WIDTH, HEIGHT, SCALE), [WIDTH as f64, HEIGHT as f64])
        .graphics_api(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();

    // Creating and Populating State Array Randomly
    let mut state: [bool; SIZE] = [false; SIZE];
    let mut i = 0;

    // state array will determine whether a cell is "alive" or "dead"
    while i < SIZE {
        state[i] = rand::random();
        i = i + 1;
    }

    // Create a new game, and run it.
    let mut app = App {
        gl: GlGraphics::new(opengl),
        state: state,
        cursor_pos: [0.0, 0.0],
        paused: false,
    };

    // Count for demonstration's frame-limiter.
    // let mut frame = 0;

    let mut events = Events::new(EventSettings::new());
    while let Some(e) = events.next(&mut window) {
        app.event([0.0, 0.0], &e);

        if let Some(args) = e.render_args() {
            app.render(&args);

            //frame += 1;
            //if frame == 50 {
            //    break;
            //}
        }

        if let Some(args) = e.update_args() {
            app.update(&args);
        }
    }
}
