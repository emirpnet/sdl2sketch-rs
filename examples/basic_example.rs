extern crate sdl2sketch;
use sdl2sketch::{Sketch, MainLoopMethods, Color};

struct BallStatus {
	pos: (i32, i32),
	vel: (i32, i32),
	size: i32,
}

impl MainLoopMethods for BallStatus {
	fn update(&mut self, s: &mut Sketch) {
		// add velocity to current position
		self.pos.0 += self.vel.0;
		self.pos.1 += self.vel.1;
		// invert velocity at canvas borders
		if self.pos.0 - self.size <= 0 || self.pos.0 + self.size >= s.width() {
			self.vel.0 *= -1;
		}
		if self.pos.1 - self.size <= 0 || self.pos.1 + self.size >= s.height() {
			self.vel.1 *= -1;
		}
	}

	fn draw(&mut self, s: &mut Sketch) {
		s.background(Color::RGB(33, 33, 33));
		s.no_stroke();
		s.fill(Color::RGB(255, 0, 0));
		s.circle(self.pos.0, self.pos.1, self.size as u32);
	}
}

fn main() {
	let mut s = Sketch::new(640, 480, "Bouncing Ball with SDL2Sketch");
	let mut b = BallStatus {
		pos: (50, 50),
		vel: (4, 2),
		size: 15,
	};
	sdl2sketch::run(&mut s, &mut b);
}
