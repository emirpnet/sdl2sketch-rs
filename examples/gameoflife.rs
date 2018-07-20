#[macro_use] extern crate sdl2sketch;
extern crate rand;

use sdl2sketch::Sketch;
use rand::Rng;
use std::cmp::{min,max};


const WIDTH: i32 = 649;
const HEIGHT: i32 = WIDTH;
const BS: i32 = 2;
const NROWS: i32 = (WIDTH+1) / BS;
const NCOLS: i32 = (HEIGHT+1) / BS;

type Cells = Vec<Cell>;

fn index(row: i32, col: i32) -> usize {
	(row * NCOLS + col) as usize
}


fn main() {
	let mut s = Sketch::new(WIDTH as u32, HEIGHT as u32, "Game of Life");
	let mut universe = generate_random_start();
	sdl2sketch_run!(&mut s, &mut universe);
}

fn generate_random_start() -> Cells {
	let mut rng = rand::thread_rng();
	let mut universe: Cells = Vec::new();
	for i in 0..NROWS {
		for j in 0..NCOLS {
			universe.push(Cell::new(i, j, rng.gen()));
		}
	}
	universe
}

fn setup(s: &mut Sketch, _universe: &mut Cells) {
	s.set_framerate(30);
	s.background(33, 33, 33);
}

fn draw(s: &mut Sketch, universe: &mut Cells) {
	s.background(33, 33, 33);
	s.set_color(220, 220, 220);

	let prev = universe.clone();

	for i in 0..NROWS {
		for j in 0..NCOLS {
			universe[index(i, j)].update(&prev);
			universe[index(i, j)].draw(s);
		}
	}
}


#[derive(Clone)]
struct Cell {
	row: i32,
	col: i32,
	pub alive: bool,
	color: (u8,u8,u8), // TODO
}

impl Cell {
	pub fn new(row: i32, col: i32, alive: bool) -> Self {
		Cell {
			row,
			col,
			alive,
			color: (255,255,255), // TODO
		}
	}

	pub fn draw(&self, sketch: &mut Sketch) {
		if self.alive {
			let x = self.col * BS;
			let y = self.row * BS;
			sketch.draw_rect(x, y, BS as u32, BS as u32);
		}
	}

	pub fn update(&mut self, universe: &Cells) {
		self.alive = match (self.alive, Cell::count_neighbours(self.row, self.col, universe)) {
			(true, 2...3) => true,
			(true, _) => false,
			(false, 3) => true,
			(false, _) => false,
		}
	}

	fn count_neighbours(row: i32, col: i32, universe: &Cells) -> i32 {
		let mut count = 0;
		if universe[index(max(row-1, 0), col)].alive { count += 1; }
		if universe[index(min(row+1, NROWS-1), col)].alive { count += 1; }
		if universe[index(row, max(col-1, 0))].alive { count += 1; }
		if universe[index(row, min(col+1, NCOLS-1))].alive { count += 1; }
		if universe[index(max(row-1, 0), max(col-1, 0))].alive { count += 1; }
		if universe[index(max(row-1, 0), min(col+1, NCOLS-1))].alive { count += 1; }
		if universe[index(min(row+1, NROWS-1), max(col-1, 0))].alive { count += 1; }
		if universe[index(min(row+1, NROWS-1), min(col+1, NCOLS-1))].alive { count += 1; }
		count
	}
}

