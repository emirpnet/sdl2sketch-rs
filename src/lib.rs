extern crate sdl2;

use sdl2::render::Canvas;
use sdl2::EventPump;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::gfx::framerate::FPSManager;
use sdl2::gfx::primitives::DrawRenderer;

/// module containing utility functions
pub mod utils;

/// alias for sdl2::pixels::Color
pub type Color = sdl2::pixels::Color;


/// starts the sketch and runs the main loop
pub fn run<T: MainLoopMethods>(s: &mut Sketch, m: &mut T) {
	m.setup(s);
	s.running = true;
	while s.running {
		handle_events(s, m);
		m.update(s);
		m.draw(s);
		s.present();
		s.delay();
	}
}

/// subroutine of the main loop to handle events
fn handle_events<T: MainLoopMethods>(s: &mut Sketch, m: &mut T) {
	while let Some(event) = s.event_pump.poll_event() {
		match event {
			Event::Quit { .. }    => { s.quit(); }
			Event::KeyDown { .. } => { m.key_pressed(s, &event); },
			_ => {}
		}
	}
}

/// This trait must be implemented by the state struct of the application which is provided to run().
pub trait MainLoopMethods {

	/// called once before entering the main loop
	fn setup(&mut self, _s: &mut Sketch) {}

	/// called every frame inside the main loop before draw()
	fn update(&mut self, _s: &mut Sketch){}

	/// called every frame inside the main loop
	fn draw(&mut self, _s: &mut Sketch) {}
	
	/// called inside the main loop on a KeyDown event 
	///
	/// The default implementation quits the main loop when the ESC-key is pressed.
	fn key_pressed(&mut self, s: &mut Sketch, e: &Event) {
		match e {
			&Event::KeyDown { keycode: Some(Keycode::Escape), .. } => { s.quit(); },
			_ => {}
		}
	}
}

/// Options for the interpretation of the parameters given to rect()
pub enum RectMode {
	/// CORNER (default): Coordinates of the upper left corner (x, y), width (w) and height (h)
	CORNER,
	/// CORNERS: Coordinates of the upper left corner (x, y) and the lower right corner (w, h)
	CORNERS,
	/// CENTER: Coordinates of the center (x, y), width (w) and height (h)
	CENTER,
	/// RADIUS: Coordinates of the center (x, y), half width (w) and half height (h)
	RADIUS,
}

/// This struct contains the necessary SDL2 subsystem objects and provides most of the API.
pub struct Sketch {
	running: bool,
	width: u32,
	height: u32,
	fill_color: Option<Color>,
	stroke_color: Option<Color>,
	stroke_weight: u8,
	smooth: bool,
	rect_mode: RectMode,
	canvas: Canvas<sdl2::video::Window>,
	event_pump: EventPump,
	fps_manager: FPSManager,
}


impl Sketch {
	
	/* general methods */

	/// create a new sketch
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
			rect_mode: RectMode::CORNER,
			canvas,
			event_pump,
			fps_manager: FPSManager::new(),
		}
	}

	/// returns the width in pixels
	pub fn width(&self) -> i32 {
		self.width as i32
	}

	/// returns the height in pixels
	pub fn height(&self) -> i32 {
		self.height as i32
	}
	
	/// sets the framerate in frames per second
	pub fn set_framerate(&mut self, fps: u32) {
		self.fps_manager.set_framerate(fps).unwrap();
	}

	/// delays the sketch to provide a constant framerate
	fn delay(&mut self) {
		self.fps_manager.delay();
	}

	/// exits the main loop
	pub fn quit(&mut self) {
		self.running = false;
	}

	/// refresh display of sketch
	fn present(&mut self) {
		self.canvas.present();
	}

	/// clears the sketch by filling the whole sketch with the provided color
	pub fn background(&mut self, color: Color) {
		self.canvas.set_draw_color(color);
		self.canvas.clear();
		if let Some(c) = self.fill_color {
			self.canvas.set_draw_color(c);
		}
	}


	/* draw settings */

	/// After calling this function primitives will be drawn with an outline in the provided color.
	pub fn stroke(&mut self, color: Color) {
		self.stroke_color = Some(color);
		self.canvas.set_draw_color(color);
	}

	/// After calling this function primitives will be drawn without outline.
	pub fn no_stroke(&mut self) {
		self.stroke_color = None;
	}

	/// After calling this function primitives will be drawn filled in the provided color.
	pub fn fill(&mut self, color: Color) {
		self.fill_color = Some(color);
	}

	/// After calling this function primitives will be drawn without fill.
	pub fn no_fill(&mut self) {
		self.fill_color = None;
	}

	/// After calling this function the outline of drawn primitives will be in the width of the provided stroke weight in pixels (provided stroke() is set).
	pub fn stroke_weight(&mut self, weight: u8) {
		self.stroke_weight = weight;
	}

	/// After calling this function primitives will be drawn with anti-aliasing. (nicer outline but slower)
	pub fn smooth(&mut self) {
		self.smooth = true;
	}

	/// After calling this function primitives will be drawn without anti-aliasing. (rugged outline but faster)
	pub fn no_smooth(&mut self) {
		self.smooth = false;
	}

	/// After calling this function primitives will be drawn without anti-aliasing. (rugged outline but faster)
	pub fn rect_mode(&mut self, mode: RectMode) {
		self.rect_mode = mode;
	}

	/* draw primitives */

	// TODO:
	// handle stroke_width (!)
	// remove unwrap() in favor of error message and continue
	// pub fn arc(&mut self, ...
	// pub fn vertex(&mut self, ...

	/// draws pixel-sized point at the provided coordinates
	pub fn point(&mut self, x: i32, y: i32) {
		if let Some(c) = self.stroke_color {
			self.canvas.set_draw_color(c);
			self.canvas.draw_point(sdl2::rect::Point::new(x, y)).unwrap();
		}
	}

	/// draws a rectangle
	pub fn rect(&mut self, x: i32, y: i32, w: u32, h: u32) {

		let (x, y, w, h) = self.rect_parameters(x, y, w, h);

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

	/// converts parameters for rect() accroding to setting of rect_mode
	fn rect_parameters(&mut self, x: i32, y: i32, w: u32, h: u32) -> (i32, i32, u32, u32) {
		match self.rect_mode {
			RectMode::CORNER  => (x, y, w, h),
			RectMode::CORNERS => (x, y, (w as i32 - x).abs() as u32, (h as i32 - y).abs() as u32), // TODO: What happens with negative and mixed up coordinates?
			RectMode::CENTER  => ((x as f32 - 0.5*w as f32) as i32, (y as f32 - 0.5*h as f32) as i32, w, h),
			RectMode::RADIUS  => ((x as f32 - 0.5*w as f32) as i32, (y as f32 - 0.5*h as f32) as i32, 2*w, 2*h),
		}
	}

	/// draws a line
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

	/// draws a polygon (EXPERIMENTAL)
	///
	/// SDL2-gfx API, not p5.js
	pub fn polygon(&mut self, vx32: &[i32], vy32: &[i32]) {

		// check if coordinates slices are same length and > 0
		if vx32.len() != vy32.len() {
			eprintln!("Error drawing polygon: unequal number of coordinates ({}/{})", vx32.len(), vy32.len());
			return;
		}
		if vx32.len() == 0 {
			eprintln!("Error drawing polygon: no coordinates provided");
			return;
		}
		
		// convert i32 to i16 (TODO: find a more efficient way)
		let mut vx = Vec::new();
		let mut vy = Vec::new();
		for i in 0..vx32.len() {
			//println!("{}: ({},{})", i, vx32[i], vy32[i]); // DEBUG
			vx.push(vx32[i] as i16);
			vy.push(vy32[i] as i16);
		}

		if let Some(c) = self.fill_color {
			self.canvas.set_draw_color(c);
			self.canvas.filled_polygon(&vx, &vy, c).unwrap();
			if self.smooth && self.stroke_color == None {
				self.canvas.aa_polygon(&vx, &vy, c).unwrap();
			}
		}
		if let Some(c) = self.stroke_color {
			self.canvas.set_draw_color(c);
			if self.smooth {
				self.canvas.aa_polygon(&vx, &vy, c).unwrap();
			} else {
				self.canvas.polygon(&vx, &vy, c).unwrap();
			}
		}
	}

	/// draws a triangle
	pub fn triangle(&mut self, x1: i32, y1: i32, x2: i32, y2: i32, x3: i32, y3: i32) {
		if let Some(c) = self.fill_color {
			self.canvas.set_draw_color(c);
			self.canvas.filled_trigon(x1 as i16, y1 as i16, x2 as i16, y2 as i16, x3 as i16, y3 as i16, c).unwrap();
			if self.smooth && self.stroke_color == None {
				self.canvas.aa_trigon(x1 as i16, y1 as i16, x2 as i16, y2 as i16, x3 as i16, y3 as i16, c).unwrap();
			}
		}
		if let Some(c) = self.stroke_color {
			self.canvas.set_draw_color(c);
			if self.smooth {
				self.canvas.aa_trigon(x1 as i16, y1 as i16, x2 as i16, y2 as i16, x3 as i16, y3 as i16, c).unwrap();
			} else {
				self.canvas.trigon(x1 as i16, y1 as i16, x2 as i16, y2 as i16, x3 as i16, y3 as i16, c).unwrap();
			}
		}
	}

	/// draws a quad (NOT COMPLETE)
	pub fn quad(&mut self, x1: i32, y1: i32, x2: i32, y2: i32, x3: i32, y3: i32, x4: i32, y4: i32) {
		let vx = [x1, x2, x3, x4, x1];
		let vy = [y1, y2, y3, y4, y1];
		// TODO: coordinate sorting necessary here!
		self.polygon(&vx, &vy);
	}

	/// draws arc (NOT COMPLETE)
	///
	/// parameters are from SDL2-gfx API, not p5.js, (TODO)
	/// which means this is a circle arc not an ellipse arc and
	/// the start/end parameters are in DEG not RAD (TODO),
	/// fill option not available (TODO)
	pub fn arc(&mut self, x: i32, y: i32, r: u32, start: i32, end: i32) {
		if let Some(c) = self.stroke_color {
			self.canvas.arc(x as i16, y as i16, r as i16, start as i16, end as i16, c).unwrap();
		}
	}

	/// draws a circle
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

	/// draws an ellipse
	pub fn ellipse(&mut self, x: i32, y: i32, w: u32, h: u32) {
		if let Some(c) = self.fill_color {
			self.canvas.set_draw_color(c);
			self.canvas.filled_ellipse(x as i16, y as i16, w as i16, h as i16, c).unwrap();
			if self.smooth && self.stroke_color == None {
				self.canvas.aa_ellipse(x as i16, y as i16, w as i16, h as i16, c).unwrap();
			}
		}
		if let Some(c) = self.stroke_color {
			self.canvas.set_draw_color(c);
			if self.smooth {
				self.canvas.aa_ellipse(x as i16, y as i16, w as i16, h as i16, c).unwrap();
			} else {
				self.canvas.ellipse(x as i16, y as i16, w as i16, h as i16, c).unwrap();
			}
		}
	}
	
}


/// initializes the necessary SDL2 subsystems and returns a SDL2 window/renderer and event pump
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

