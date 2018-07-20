//#![feature(trace_macros)]
//trace_macros!(true);

#[macro_use] extern crate sdl2sketch;
use sdl2sketch::Sketch;


struct Globals {
	xpos: i32,
	ypos: i32,
}

fn setup(_s: &mut Sketch, _global: &mut Globals) {
	
}

fn update(_s: &mut Sketch, global: &mut Globals) {
	global.xpos += 1;
	global.ypos += 1;
}

fn draw(s: &mut Sketch, global: &mut Globals) {
	s.background(33, 33, 33);
	s.set_color(0, 0, 255);
	s.draw_rect(global.xpos, global.ypos, 20, 20);
}

fn main() {
	let mut s = Sketch::new(640, 480, "Title");
	let mut g = Globals { xpos: 50, ypos: 50 };
	sdl2sketch_run!(&mut s, &mut g);
}

