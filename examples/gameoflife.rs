#[macro_use] extern crate sdl2sketch;
extern crate rand;

use sdl2sketch::Sketch;
use rand::Rng;
use std::{thread,time};


const WIDTH: i32 = 649;
const HEIGHT: i32 = WIDTH;
const BS: i32 = 2;
const NROWS: i32 = (WIDTH+1) / BS;
const NCOLS: i32 = (HEIGHT+1) / BS;

type LifeState = Vec<bool>;


fn setup(s: &mut Sketch, _life: &mut LifeState) {
	s.background(33, 33, 33);
}

fn update(_s: &mut Sketch, life: &mut LifeState) {
	let old = life.clone();
	for i in 0..NROWS {
		for j in 0..NCOLS {
			life[index(i, j)] = alive(i, j, &old);
		}
	}
}

fn draw(s: &mut Sketch, life: &mut LifeState) {
	s.background(33, 33, 33);
	s.set_color(0, 255, 0);
	display_frame(s, life);
	thread::sleep(time::Duration::from_millis(40));
}

fn main() {
	let mut s = Sketch::new(WIDTH as u32, HEIGHT as u32, "Game of Life");
	let mut life = life_random();
	sdl2sketch_run!(&mut s, &mut life);
}

fn life_random() -> LifeState {
	let mut rng = rand::thread_rng();
	let mut v: LifeState = Vec::new();
	
	for _i in 0..(NROWS*NCOLS) {
		v.push(rng.gen());
	}
	v
}

fn display_frame(sketch: &mut Sketch, v: &LifeState) {
	for i in 0..NROWS {
		for j in 0..NCOLS {
			if v[index(i, j)] {
				display_cell(sketch, i, j);	
			}
		}
	}
}

fn index(row: i32, col: i32) -> usize {
	(row * NCOLS + col) as usize
}

fn display_cell(sketch: &mut Sketch, row: i32, col: i32) {
	let x = col * BS;
	let y = row * BS;
	sketch.draw_rect(x, y, BS as u32, BS as u32);
}

fn alive(row: i32, col: i32, v: &LifeState) -> bool {
	let current_state = v[index(row, col)];
	let num_neighbours = count_neighbours(row, col, v);

	match (current_state, num_neighbours) {
		(true, 2...3) => true,
		(true, _) => false,
		(false, 3) => true,
		(false, _) => false,
	}
}

fn count_neighbours(row: i32, col: i32, v: &LifeState) -> i32 {
	let mut count = 0;

	if v[index(dec(row, NROWS), col)] { count += 1; }
	if v[index(inc(row, NROWS), col)] { count += 1; }
	if v[index(row, dec(col, NCOLS))] { count += 1; }
	if v[index(row, inc(col, NCOLS))] { count += 1; }
	if v[index(dec(row, NROWS), dec(col, NCOLS))] { count += 1; }
	if v[index(dec(row, NROWS), inc(col, NCOLS))] { count += 1; }
	if v[index(inc(row, NROWS), dec(col, NCOLS))] { count += 1; }
	if v[index(inc(row, NROWS), inc(col, NCOLS))] { count += 1; }

	count
}

fn dec(x: i32, boundary: i32) -> i32 {
	if x == 0 {
		boundary - 1	
	} else {
		x - 1
	}
}

fn inc(x: i32, boundary: i32) -> i32 {
	(x + 1) % boundary
}

