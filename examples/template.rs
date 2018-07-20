#[macro_use]
extern crate sdl2sketch;
use sdl2sketch::Sketch;


struct SketchGlobals {
	xpos: i32,
	ypos: i32,
}

fn setup(s: &mut Sketch<SketchGlobals>, _g: &mut SketchGlobals) {
	s.set_background(33, 33, 33);
}

fn update(s: &mut Sketch<SketchGlobals>, g: &mut SketchGlobals) {
	s.set_color(0, 0, 255);
	s.draw_rect(g.xpos, g.ypos, 20, 20);
	g.xpos += 1;
	g.ypos += 1;

}

fn main() {
	let mut s: Sketch<SketchGlobals> = Sketch::new(640, 480, "Title", &setup, &update);
	let mut g = SketchGlobals { xpos: 50, ypos: 50 };
	s.run(&mut g);
}

