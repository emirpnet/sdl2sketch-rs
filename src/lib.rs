extern crate sdl2;

use sdl2::render::Canvas;
use sdl2::EventPump;
use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;


#[macro_export]
macro_rules! sdl2sketch_run {
	($sketch:expr, $globals:expr) => {
		$sketch.running = true;
		setup($sketch, $globals);
		while $sketch.running {
			$sketch.handle_keyevents(); // TODO
			update($sketch, $globals);
			draw($sketch, $globals);
			$sketch.present();
		}
	};
	($sketch:expr) => {
		$sketch.running = true;
		setup($sketch);
		while $sketch.running {
			$sketch.handle_keyevents(); // TODO
			update($sketch);
			draw($sketch);
			$sketch.present();
		}
	};
}


pub struct Sketch {
	pub width: u32,
	pub height: u32,
	pub running: bool,
	framerate: u32, // TODO
	canvas: Canvas<sdl2::video::Window>,
	event_pump: EventPump,
}


impl Sketch {
	
	pub fn new(width: u32, height: u32, title: &str) -> Self {
		let (canvas, event_pump) = new_canvas(width, height, title);
		let framerate = 60;

		Sketch {
			width,
			height,
			running: false,
			framerate,
			canvas,
			event_pump,
		}
	}

	pub fn quit(&mut self) {
		self.running = false;
	}

	pub fn handle_keyevents(&mut self) {
		for event in self.event_pump.poll_iter() {
			match event {
				Event::KeyDown { keycode: Some(Keycode::Escape), .. } => { self.running = false; },
				//Event::KeyDown { keycode: Some(Keycode::Escape), .. } => { self.quit(); },
				_ => {}
			}
		}
	}

	pub fn set_color(&mut self, r: u8, g: u8, b: u8) {
		self.canvas.set_draw_color(Color::RGB(r, g, b));
	}

	pub fn background(&mut self, r: u8, g: u8, b: u8) {
		self.canvas.set_draw_color(Color::RGB(r, g, b));
		self.canvas.clear();
	}

	pub fn present(&mut self) {
		self.canvas.present();
	}

	pub fn draw_point(&mut self, x: i32, y: i32) {
		self.canvas.draw_point(sdl2::rect::Point::new(x, y)).unwrap();
	}

	pub fn draw_rect(&mut self, x: i32, y: i32, w: u32, h: u32) {
		self.canvas.fill_rect(sdl2::rect::Rect::new(x, y, w, h)).unwrap();
	}
}


fn new_canvas(width: u32, height: u32, title: &str) -> (Canvas<sdl2::video::Window>, EventPump) {
	let sdl_context = sdl2::init().unwrap();
	let video_subsystem = sdl_context.video().unwrap();
	let window = video_subsystem.window(title, width, height)
		.position_centered()
		.opengl()
		.build()
		.unwrap();
	//let mut renderer = window.renderer().build().unwrap();
	let canvas = window.into_canvas().accelerated().build().unwrap();
	let event_pump = sdl_context.event_pump().unwrap();

	(canvas, event_pump)
}

