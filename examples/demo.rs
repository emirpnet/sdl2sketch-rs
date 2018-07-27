extern crate sdl2sketch;
use sdl2sketch::*;


struct MainState {
	pos: (i32, i32),
	vel: (i32, i32),
	size: i32,
}

impl MainLoopMethods for MainState {
	fn setup(&mut self, s: &mut Sketch) {
		s.set_framerate(30);
		//s.no_smooth();
	}

	fn update(&mut self, s: &mut Sketch) {
		self.pos.0 += self.vel.0;
		self.pos.1 += self.vel.1;
		if self.pos.0 - self.size <= 0 || self.pos.0 + self.size >= s.width() {
			self.vel.0 *= -1;
		}
		if self.pos.1 - self.size <= 0 || self.pos.1 + self.size >= s.height() {
			self.vel.1 *= -1;
		}

		if s.key_is_down(Keycode::Num0) {
			println!("Num0 down while mouse at {:?}", s.mouse_pos());
		}
	}

	fn draw(&mut self, s: &mut Sketch) {
		s.background(Color::RGB(33, 33, 33));

		s.stroke(Color::RGB(255, 255, 255));
		s.line(10, 10, 630, 470);

		s.stroke(Color::RGB(0, 0, 255));
		s.fill(Color::RGB(0, 255, 255));
		s.rect_mode(RectMode::CENTER);
		let center_x = (0.5*s.width() as f32) as i32;
		let center_y = (0.5*s.height() as f32) as i32;
		let ratio_w = (0.2*s.width() as f32) as u32;
		let ratio_h = (0.2*s.height() as f32) as u32;
		s.rect(center_x, center_y, ratio_w, ratio_h);

		s.stroke(Color::RGB(0, 0, 255));
		s.fill(Color::RGB(0, 255, 0));
		s.triangle(400, 90, 470, 80, 430, 50);

		s.stroke(Color::RGB(0, 0, 255));
		s.no_fill();
		s.quad(500, 350, 550, 350, 550, 400, 500, 400);

		s.stroke(Color::RGB(255, 0, 255));
		s.no_fill();
		s.arc(150, 240, 40, 0, 270);

		s.stroke(Color::RGB(0, 0, 255));
		s.fill(Color::RGB(255, 255, 0));
		s.ellipse(100, 350, 50, 30);

		s.no_stroke();
		s.fill(Color::RGB(255, 0, 0));
		s.circle(self.pos.0, self.pos.1, self.size as u32);
	}

	fn key_released(&mut self, _s: &mut Sketch, code: Keycode) {
		match code {
			Keycode::A => { println!("A released"); }
			_ => {}
		}
	}

	/*
	fn mouse_moved(&mut self, _s: &mut Sketch, x: i32, y: i32, xrel: i32, yrel: i32) {
		println!("mouse_moved: ({},{}) -> ({},{})", xrel, yrel, x, y);
	}
	fn mouse_dragged(&mut self, _s: &mut Sketch, x: i32, y: i32, xrel: i32, yrel: i32) {
		println!("mouse_dragged: ({},{}) -> ({},{})", xrel, yrel, x, y);
	}
	*/

	fn mouse_pressed(&mut self, _s: &mut Sketch, button: MouseButton, x: i32, y: i32) {
		println!("Mouse \"{:?}\" pressed at ({},{})", button, x, y);
	}

	fn mouse_released(&mut self, _s: &mut Sketch, button: MouseButton, x: i32, y: i32) {
		println!("Mouse \"{:?}\" released at ({},{})", button, x, y);
	}
}

fn main() {
	let mut s = Sketch::new(640, 480, "SDL2Sketch Demo");
	let mut m = MainState {
		pos: (50, 50),
		vel: (4, 2),
		size: 15,
	};
	sdl2sketch::run(&mut s, &mut m);
}
