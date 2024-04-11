# Conway's Game of Life in Rust

![alt text](https://github.com/MKP157/Rust-Game-of-Life/blob/main/demo.gif)

## Authors

**Matthew Peterson** (mpeters9@unb.ca)

**Aiden Manuel** (aiden.manuel@unb.ca)

## What is this Project?
Written for our Parallel Computing course (CS 3123) at the University of New Brunswick, Saint John, this program is implementation of John Conway's "Game of Life" cellular automata simulation in Rust. It uses the [Rayon](https://docs.rs/rayon) Crate for parallizing the process of updating each frame of the simulation, and [Piston](https://github.com/PistonDevelopers/piston) for implementing a simple graphical interface on top of OpenGL to provide the user with a visual of the simulation as it runs.

## What is Conway's Game of Life?

The Game of Life is a zero-player game with simple rules that generate emergent complex behavior. Cells on a grid can be alive or dead, and their fate is determined by their living neighbors:
- A living cell with 2 or 3 living neighbors survives.
- In all other cases, a cell dies or becomes alive (if it has exactly 3 living neighbors).

This simple algorithm can lead to fascinating and unpredictable patterns, making it a captivating exploration of cellular automata.

# Getting Started

## Prerequisites:

- Rust compiler (https://doc.rust-lang.org/)
- Cargo package manager (usually comes bundled with Rust)

## To running the Simulation:

1. Clone this repository:
```git clone https://github.com/MKP157/Rust-Game-of-Life```

2. Navigate to the project directory:
```cd conway-game-of-life```

3. Build and run the simulation with Rust. Replace `<threads>` with the numberof software threads you wish to allocate to Rayon.
```cargo run <threads>```

This will launch a window with a grid where you can click to toggle cells alive or dead. Press `Space` to pause/unpause the simulation and, `C` to clear the simulation grid, and `R` to randomly initialize the grid.
