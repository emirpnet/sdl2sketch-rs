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
	}

	fn draw(&mut self, s: &mut Sketch) {
		s.background(Color::RGB(33, 33, 33));

		s.stroke(Color::RGB(255, 255, 255));
		s.line(10, 10, 630, 470);

		s.stroke(Color::RGB(0, 0, 255));
		s.fill(Color::RGB(0, 255, 255));
		s.rect(256, 192, 128, 96);

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
}

fn main() {
	let mut s = Sketch::new(640, 480, "Title");
	let mut m = MainState {
		pos: (50, 50),
		vel: (4, 2),
		size: 15,
	};
	sdl2sketch::run(&mut s, &mut m);
}

