#[macro_use] extern crate sdl2sketch;
extern crate mylib;
extern crate rand;

use sdl2sketch::*;
use rand::Rng;


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
}

fn draw(s: &mut Sketch, universe: &mut Cells) {
	s.background(&Color::RGB(33, 33, 33));

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
	pub color: Color,
}

impl Cell {
	pub fn new(row: i32, col: i32, alive: bool) -> Self {
		let rgb = utils::hsv_to_rgb(rand::thread_rng().gen_range(0, 360), 1.0, 1.0);
		let color = Color::RGB(rgb.0, rgb.1, rgb.2);
		Cell {
			row,
			col,
			alive,
			color,
		}
	}

	pub fn draw(&self, sketch: &mut Sketch) {
		if self.alive {
			let x = self.col * BS;
			let y = self.row * BS;
			sketch.set_color(&self.color);
			sketch.draw_rect(x, y, BS as u32, BS as u32);
		}
	}

	pub fn update(&mut self, universe: &Cells) {
		self.alive = match (self.alive, self.count_neighbours(universe)) {
			(true, 2...3) => true,
			(true, _) => false,
			(false, 3) => true,
			(false, _) => false,
		}
	}

	fn count_neighbours(&self, universe: &Cells) -> i32 {
		let neighbours = self.get_neighbour_indices(true);
		let mut count = 0;
		for i in neighbours {
			if universe[i].alive { count += 1; }
		}
		count
	}

	fn get_neighbour_indices(&self, wrap: bool) -> Vec<usize> {

		let mut coords = Vec::with_capacity(8);
		coords.push((self.row-1, self.col  ));
		coords.push((self.row+1, self.col  ));
		coords.push((self.row,   self.col-1));
		coords.push((self.row,   self.col+1));
		coords.push((self.row-1, self.col-1));
		coords.push((self.row-1, self.col+1));
		coords.push((self.row+1, self.col-1));
		coords.push((self.row+1, self.col+1));

		if !wrap {
			coords.retain(|c| c.0 >= 0 && c.0 < NROWS && c.1 >= 0 && c.1 < NCOLS);
		} else {
			for c in &mut coords {
				if c.0 < 0 {
					c.0 += NROWS;
				} else {
					c.0 = c.0 % NROWS;
				}
				if c.1 < 0 {
					c.1 += NCOLS;
				} else {
					c.1 = c.1 % NCOLS;
				}
			}
		}

		let mut indices = Vec::with_capacity(8);
		for c in coords {
			indices.push(index(c.0, c.1));
		}
		indices
	}
}

