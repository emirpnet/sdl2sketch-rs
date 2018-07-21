extern crate sdl2;

use sdl2::render::Canvas;
use sdl2::EventPump;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::gfx::framerate::FPSManager;

pub mod utils;
pub type Color = sdl2::pixels::Color;


#[macro_export]
macro_rules! sdl2sketch_run {
	($sketch:expr, $globals:expr) => {
		$sketch.running = true;
		setup($sketch, $globals);
		while $sketch.running {
			$sketch.handle_keyevents(); // TODO
			draw($sketch, $globals);
			$sketch.present();
			$sketch.delay();
		}
	};
	($sketch:expr) => {
		$sketch.running = true;
		setup($sketch);
		while $sketch.running {
			$sketch.handle_keyevents(); // TODO
			draw($sketch);
			$sketch.present();
			$sketch.delay();
		}
	};
}


pub struct Sketch {
	pub width: u32,
	pub height: u32,
	pub running: bool,
	canvas: Canvas<sdl2::video::Window>,
	event_pump: EventPump,
	fps_manager: FPSManager,
}


impl Sketch {
	
	pub fn new(width: u32, height: u32, title: &str) -> Self {
		let (canvas, event_pump) = new_canvas(width, height, title);

		Sketch {
			width,
			height,
			running: false,
			canvas,
			event_pump,
			fps_manager: FPSManager::new(),
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

	pub fn set_framerate(&mut self, fps: u32) {
		self.fps_manager.set_framerate(fps).unwrap();
	}

	pub fn delay(&mut self) {
		self.fps_manager.delay();
	}

	pub fn set_color(&mut self, c: &Color) {
		self.canvas.set_draw_color(*c);
	}

	pub fn background(&mut self, c: &Color) {
		self.canvas.set_draw_color(*c);
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

