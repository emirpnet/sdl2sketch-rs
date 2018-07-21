//#![feature(trace_macros)]
//trace_macros!(true);
#[macro_use] extern crate sdl2sketch;

use sdl2sketch::*;


struct Globals {
	xpos: i32,
	ypos: i32,
}

fn main() {
	let mut s = Sketch::new(640, 480, "Title");
	let mut g = Globals { xpos: 50, ypos: 50 };
	sdl2sketch_run!(&mut s, &mut g);
}

fn setup(s: &mut Sketch, _global: &mut Globals) {
	s.set_framerate(30);
}

fn draw(s: &mut Sketch, global: &mut Globals) {
	//update
	global.xpos += 2;
	global.ypos += 1;
	//draw
	s.background(&Color::RGB(33, 33, 33));
	s.set_color(&Color::RGB(0, 255, 255));
	s.draw_circle(320, 240, 80);
	s.set_color(&Color::RGB(0, 0, 255));
	s.draw_rect(global.xpos, global.ypos, 20, 20);
}

