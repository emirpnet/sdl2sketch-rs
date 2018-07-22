extern crate sdl2;

use sdl2::render::Canvas;
use sdl2::EventPump;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::gfx::framerate::FPSManager;
use sdl2::gfx::primitives::DrawRenderer;

pub mod utils;
pub type Color = sdl2::pixels::Color;


#[macro_export]
macro_rules! sdl2sketch_run {
	($sketch:expr, $globals:expr) => {
		$sketch.running = true;
		setup($sketch, $globals);
		while $sketch.running {
			$sketch.handle_events(); // TODO
			draw($sketch, $globals);
			$sketch.present();
			$sketch.delay();
		}
	};
	($sketch:expr) => {
		$sketch.running = true;
		setup($sketch);
		while $sketch.running {
			$sketch.handle_events(); // TODO
			draw($sketch);
			$sketch.present();
			$sketch.delay();
		}
	};
}


pub struct Sketch {
	pub running: bool,
	width: u32,
	height: u32,
	fill_color: Option<Color>,
	stroke_color: Option<Color>,
	stroke_weight: u8,
	smooth: bool,
	canvas: Canvas<sdl2::video::Window>,
	event_pump: EventPump,
	fps_manager: FPSManager,
}


impl Sketch {
	
	/* general methods */

	// TODO:
	// flexible event handling (!)

	pub fn new(width: u32, height: u32, title: &str) -> Self {
		let (canvas, event_pump) = init_sdl_subsystems(width, height, title);
		Sketch {
			running: false,
			width,
			height,
			fill_color: Some(Color::RGB(255, 255, 255)),
			stroke_color: Some(Color::RGB(255, 255, 255)),
			stroke_weight: 1,
			smooth: true,
			canvas,
			event_pump,
			fps_manager: FPSManager::new(),
		}
	}

	pub fn handle_events(&mut self) {
		for event in self.event_pump.poll_iter() {
			match event {
				Event::KeyDown { keycode: Some(Keycode::Escape), .. } => { self.running = false; },
				//Event::KeyDown { keycode: Some(Keycode::Escape), .. } => { self.quit(); },
				_ => {}
			}
		}
	}

	pub fn width(&self) -> i32 {
		self.width as i32
	}

	pub fn height(&self) -> i32 {
		self.height as i32
	}
	
	pub fn set_framerate(&mut self, fps: u32) {
		self.fps_manager.set_framerate(fps).unwrap();
	}

	pub fn delay(&mut self) {
		self.fps_manager.delay();
	}

	pub fn quit(&mut self) {
		self.running = false;
	}

	pub fn present(&mut self) {
		self.canvas.present();
	}

	pub fn background(&mut self, color: Color) {
		self.canvas.set_draw_color(color);
		self.canvas.clear();
		if let Some(c) = self.fill_color {
			self.canvas.set_draw_color(c);
		}
	}


	/* draw settings */

	pub fn stroke(&mut self, color: Color) {
		self.stroke_color = Some(color);
		self.canvas.set_draw_color(color); // TODO: check ok!?
	}

	pub fn no_stroke(&mut self) {
		self.stroke_color = None;
	}

	pub fn fill(&mut self, color: Color) {
		self.fill_color = Some(color);
	}

	pub fn no_fill(&mut self) {
		self.fill_color = None;
	}

	pub fn stroke_weight(&mut self, weight: u8) {
		self.stroke_weight = weight;
	}

	pub fn smooth(&mut self) {
		self.smooth = true;
	}

	pub fn no_smooth(&mut self) {
		self.smooth = false;
	}


	/* draw primitives */

	// TODO:
	// handle stroke_width (!)
	// pub fn quad(&mut self, ...
	// pub fn triangle(&mut self, ...
	// pub fn arc(&mut self, ...
	// pub fn ellipse(&mut self, ...
	// pub fn vertex(&mut self, ...

	pub fn point(&mut self, x: i32, y: i32) {
		if let Some(c) = self.stroke_color {
			self.canvas.set_draw_color(c);
			self.canvas.draw_point(sdl2::rect::Point::new(x, y)).unwrap();
		}
	}

	pub fn rect(&mut self, x: i32, y: i32, w: u32, h: u32) {
		if let Some(c) = self.fill_color {
			self.canvas.set_draw_color(c);
			self.canvas.fill_rect(sdl2::rect::Rect::new(x, y, w, h)).unwrap();
		}
		if let Some(c) = self.stroke_color {
			self.canvas.set_draw_color(c);
			self.canvas.draw_rect(sdl2::rect::Rect::new(x, y, w, h)).unwrap();
			self.canvas.draw_point(sdl2::rect::Point::new(x-1+w as i32, y-1+h as i32)).unwrap(); // fix for missing point in bottom-right corner of draw_rect()
		}
	}

	pub fn line(&mut self, x1: i32, y1: i32, x2: i32, y2: i32) {
		if let Some(c) = self.stroke_color {
			self.canvas.set_draw_color(c);
			if self.smooth {
				self.canvas.aa_line(x1 as i16, y1 as i16, x2 as i16, y2 as i16, c).unwrap();
			} else {
				self.canvas.line(x1 as i16, y1 as i16, x2 as i16, y2 as i16, c).unwrap();
				//self.canvas.thick_line(x1 as i16, y1 as i16, x2 as i16, y2 as i16, self.stroke_weight, c).unwrap();
			}
		}
	}

	pub fn circle(&mut self, x: i32, y: i32, r: u32) {
		if let Some(c) = self.fill_color {
			self.canvas.set_draw_color(c);
			self.canvas.filled_circle(x as i16, y as i16, r as i16, c).unwrap();
			if self.smooth && self.stroke_color == None {
				self.canvas.aa_circle(x as i16, y as i16, r as i16, c).unwrap();
			}
		}
		if let Some(c) = self.stroke_color {
			self.canvas.set_draw_color(c);
			if self.smooth {
				self.canvas.aa_circle(x as i16, y as i16, r as i16, c).unwrap();
			} else {
				self.canvas.circle(x as i16, y as i16, r as i16, c).unwrap();
			}
		}
	}

}


fn init_sdl_subsystems(width: u32, height: u32, title: &str) -> (Canvas<sdl2::video::Window>, EventPump) {
	let sdl_context = sdl2::init().unwrap();
	let video_subsystem = sdl_context.video().unwrap();
	let window = video_subsystem.window(title, width, height)
		.position_centered()
		.opengl()
		.build().unwrap();
	let canvas = window.into_canvas()
		.accelerated()
		.build().unwrap();
	let event_pump = sdl_context.event_pump().unwrap();
	(canvas, event_pump)
}

