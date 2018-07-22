//#![feature(trace_macros)]
//trace_macros!(true);
#[macro_use] extern crate sdl2sketch;

use sdl2sketch::*;


const MOVER_SIZE: u32 = 25;

struct GlobalState {
	pos: (i32, i32),
	vel: (i32, i32),
}

fn main() {
	let mut s = Sketch::new(640, 480, "Title");
	let mut g = GlobalState {
		pos: (50, 50),
		vel: (2, 1),
	};
	sdl2sketch_run!(&mut s, &mut g);
}

fn setup(s: &mut Sketch, _global: &mut GlobalState) {
	s.set_framerate(30);
	//s.no_smooth();
}

fn draw(s: &mut Sketch, g: &mut GlobalState) {

	//update
	g.pos.0 += g.vel.0;
	g.pos.1 += g.vel.1;
	if g.pos.0 <= 0 || g.pos.0 + MOVER_SIZE as i32 >= s.width() {
		g.vel.0 *= -1;
	}
	if g.pos.1 <= 0 || g.pos.1 + MOVER_SIZE as i32 >= s.height() {
		g.vel.1 *= -1;
	}

	//draw
	s.background(Color::RGB(33, 33, 33));

	s.stroke(Color::RGB(255, 255, 255));
	s.line(10, 10, 630, 470);

	s.no_stroke();
	s.fill(Color::RGB(255, 0, 0));
	s.circle(320, 240, 80);

	s.stroke(Color::RGB(0, 0, 255));
	s.fill(Color::RGB(0, 255, 255));
	s.rect(g.pos.0, g.pos.1, MOVER_SIZE, MOVER_SIZE);
}

