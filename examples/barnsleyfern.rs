#[macro_use] extern crate sdl2sketch;
extern crate rand;
extern crate mylib;

use sdl2sketch::*;
use mylib::useful::map;


struct Point {
	x: f32,
	y: f32
}

fn main() {
	let mut s = Sketch::new(480, 640, "Barnsley fern");
	let mut pt = Point { x: 0.0, y: 0.0 };
	sdl2sketch_run!(&mut s, &mut pt);
}

fn setup(s: &mut Sketch, _pt: &mut Point) {
	s.set_framerate(120);
	s.background(&Color::RGB(33, 33, 33));
	s.set_color(&Color::RGB(0, 220, 0));
}

fn draw(s: &mut Sketch, pt: &mut Point) {
	for _i in 0..20 {
		next_pt(pt);
		let px = map(pt.x, -2.1820, 2.6558, 0.0, s.width as f32) as i32;
		let py = map(pt.y, 0.0, 9.9983, s.height as f32, 0.0) as i32;
		s.draw_point(px, py);
	}
}

fn next_pt(pt: &mut Point) {
	
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

	pt.x = x;
	pt.y = y;
}

