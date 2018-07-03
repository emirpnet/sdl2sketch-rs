extern crate rand;
extern crate sdl2;
extern crate sdl2sketch;

use std::{thread,time};
use rand::Rng;
use sdl2::render::Canvas;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;


const WIDTH: i32 = 649;
const HEIGHT: i32 = WIDTH;
const BS: i32 = 2;
const NROWS: i32 = (WIDTH+1) / BS;
const NCOLS: i32 = (HEIGHT+1) / BS;


fn main() {

	//let black = Color::RGB(0, 0, 0);
	//let red = Color::RGB(255, 0, 0);

	let (mut canvas, mut event_pump) = sdl2sketch::new_canvas(WIDTH as u32, HEIGHT as u32, "Game of Life");
	let mut life = life_random();

	'running: loop {

		for event in event_pump.poll_iter() {
			match event {
				Event::KeyDown { keycode: Some(Keycode::Escape), .. } => { break 'running; },
				_ => {}
			}
		}

		display_frame(&mut canvas, &life);
		life = life_next(&life);

		thread::sleep(time::Duration::from_millis(40));
	}
}

fn life_random() -> Vec<bool> {
	let mut rng = rand::thread_rng();
	let mut v: Vec<bool> = Vec::new();
	
	for _i in 0..(NROWS*NCOLS) {
		v.push(rng.gen());
	}
	v
}

fn life_next(v: &Vec<bool>) -> Vec<bool> {
	let mut v_new: Vec<bool> = vec![false; (NROWS*NCOLS) as usize];
	for i in 0..NROWS {
		for j in 0..NCOLS {
			v_new[index(i, j)] = alive(i, j, v);
		}
	}
	v_new
}

fn display_frame(canvas: &mut Canvas<sdl2::video::Window>, v: &Vec<bool>) {
	canvas.set_draw_color(Color::RGB(0, 0, 0));
	canvas.clear();
	for i in 0..NROWS {
		for j in 0..NCOLS {
			if v[index(i, j)] {
				display_cell(canvas, i, j);	
			}
		}
	}
	canvas.present();
}

fn index(row: i32, col: i32) -> usize {
	(row * NCOLS + col) as usize
}

fn display_cell(canvas: &mut Canvas<sdl2::video::Window>, row: i32, col: i32) {
	let x = col * BS;
	let y = row * BS;
	canvas.set_draw_color(Color::RGB(220, 220, 220));
	canvas.fill_rect(Rect::new(x, y, BS as u32, BS as u32)).unwrap();
}

fn alive(row: i32, col: i32, v: &Vec<bool>) -> bool {
	let current_state = v[index(row, col)];
	let num_neighbours = count_neighbours(row, col, v);

	match (current_state, num_neighbours) {
		(true, 2...3) => true,
		(true, _) => false,
		(false, 3) => true,
		(false, _) => false,
	}
}

fn count_neighbours(row: i32, col: i32, v: &Vec<bool>) -> i32 {
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

