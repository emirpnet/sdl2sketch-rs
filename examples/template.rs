//#![feature(trace_macros)]
//trace_macros!(true);
#[macro_use] extern crate sdl2sketch;

use sdl2sketch::*;


struct GlobalState {
	pos: (i32, i32),
	vel: (i32, i32),
	size: i32,
}


fn main() {
	let mut s = Sketch::new(640, 480, "Title");
	let mut g = GlobalState {
		pos: (50, 50),
		vel: (4, 2),
		size: 15,
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
	if g.pos.0 - g.size <= 0 || g.pos.0 + g.size >= s.width() {
		g.vel.0 *= -1;
	}
	if g.pos.1 - g.size <= 0 || g.pos.1 + g.size >= s.height() {
		g.vel.1 *= -1;
	}

	//draw
	s.background(Color::RGB(33, 33, 33));

	s.stroke(Color::RGB(255, 255, 255));
	s.line(10, 10, 630, 470);

	s.stroke(Color::RGB(0, 0, 255));
	s.fill(Color::RGB(0, 255, 255));
	s.rect(256, 192, 128, 96);

	s.no_stroke();
	s.fill(Color::RGB(255, 0, 0));
	s.circle(g.pos.0, g.pos.1, g.size as u32);
}

