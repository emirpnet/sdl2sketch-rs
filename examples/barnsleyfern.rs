extern crate rand;
extern crate sdl2;
extern crate sdl2sketch;
extern crate mylib;

use std::{thread,time};
use sdl2::render::Canvas;
use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use mylib::useful::map_float;


const WIDTH: u32 = 480;
const HEIGHT: u32 = 640;

struct Point {
	x: f32,
	y: f32
}

fn main() {
	let (mut canvas, mut event_pump) = sdl2sketch::new_canvas(WIDTH, HEIGHT, "Barnsley fern");
	let mut pt = Point { x: 0.0, y: 0.0 };

	'running: loop {

		for event in event_pump.poll_iter() {
			match event {
				Event::KeyDown { keycode: Some(Keycode::Escape), .. } => { break 'running; },
				_ => {}
			}
		}

		pt = next_pt(pt);
		display_pt(&mut canvas, &pt);

		thread::sleep(time::Duration::from_millis(1));
	}
}


fn next_pt(pt: Point) -> Point {
	
	let trans = [ // Barnsley fern
		[ 0.00, 0.00, 0.00, 0.16, 0.00, 0.00, 0.01],
		[ 0.85, 0.04,-0.04, 0.85, 0.00, 1.60, 0.86],
		[ 0.20,-0.26, 0.23, 0.22, 0.00, 1.60, 0.93],
		[-0.15, 0.28, 0.26, 0.24, 0.00, 0.44, 1.00]
	];

	/*
	let trans = [ // alternative fern
		[ 0.000, 0.000, 0.000, 0.250, 0.000,-0.400, 0.020],
		[ 0.950, 0.005,-0.005, 0.930,-0.002, 0.500, 0.860], 
		[ 0.035,-0.200, 0.160, 0.040,-0.090, 0.020, 0.930], 
		[-0.040, 0.200, 0.160, 0.040, 0.083, 0.120, 1.000]
	];
	*/

	let r: f32 = rand::random();
	let mut func: usize = 0;
	for t in &trans {
		if r < t[6] {
			break;
		}
		func += 1;
	}

	let t = &trans[func];
	let x = t[0] * pt.x + t[1] * pt.y + t[4];
	let y = t[2] * pt.x + t[3] * pt.y + t[5];

	Point {x, y}
}

fn display_pt(canvas: &mut Canvas<sdl2::video::Window>, pt: &Point) {
	let px = map_float(pt.x, -2.1820, 2.6558, 0.0, WIDTH as f32) as i32;
	let py = map_float(pt.y, 0.0, 9.9983, HEIGHT as f32, 0.0) as i32;
	canvas.set_draw_color(Color::RGB(255, 255, 255));
	let _ = canvas.draw_point(sdl2::rect::Point::new(px, py));
	canvas.present();
}

