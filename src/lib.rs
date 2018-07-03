extern crate sdl2;

use sdl2::render::Canvas;
use sdl2::EventPump;
use sdl2::pixels::Color;


pub fn new_canvas(width: u32, height: u32, title: &str) -> (Canvas<sdl2::video::Window>, EventPump) {

	let sdl_context = sdl2::init().unwrap();
	let video_subsystem = sdl_context.video().unwrap();
	let window = video_subsystem.window(title, width, height)
		.position_centered()
		.opengl()
		.build()
		.unwrap();
	//let mut renderer = window.renderer().build().unwrap();
	let mut canvas = window.into_canvas().accelerated().build().unwrap();
	let event_pump = sdl_context.event_pump().unwrap();

	canvas.set_draw_color(Color::RGB(0, 0, 0));
	canvas.clear();
	canvas.present();

	(canvas, event_pump)
}

