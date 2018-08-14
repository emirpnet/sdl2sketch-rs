extern crate sdl2sketch;
extern crate rand;

use sdl2sketch::*;
use sdl2sketch::utils::map;


struct Point {
	x: f32,
	y: f32
}

impl Point {
	fn next_pt(&mut self) {
		
		// point transformations for Barnsley fern
		let trans = [
			[ 0.00, 0.00, 0.00, 0.16, 0.00, 0.00, 0.01],
			[ 0.85, 0.04,-0.04, 0.85, 0.00, 1.60, 0.86],
			[ 0.20,-0.26, 0.23, 0.22, 0.00, 1.60, 0.93],
			[-0.15, 0.28, 0.26, 0.24, 0.00, 0.44, 1.00]
		];

		// point transformations for alternative fern
		// let trans = [
		//	 [ 0.000, 0.000, 0.000, 0.250, 0.000,-0.400, 0.020],
		//	 [ 0.950, 0.005,-0.005, 0.930,-0.002, 0.500, 0.860],
		//	 [ 0.035,-0.200, 0.160, 0.040,-0.090, 0.020, 0.930],
		//	 [-0.040, 0.200, 0.160, 0.040, 0.083, 0.120, 1.000]
		// ];

		// choose transformation randomly based on probabilities in the last column
		let r: f32 = rand::random();
		let mut func: usize = 0;
		for t in &trans {
			if r < t[6] {
				break;
			}
			func += 1;
		}

		// perform transformation
		let t = &trans[func];
		let new_x = t[0] * self.x + t[1] * self.y + t[4];
		let new_y = t[2] * self.x + t[3] * self.y + t[5];
		self.x = new_x;
		self.y = new_y;
	}
}

impl MainLoopMethods for Point {
	fn setup(&mut self, s: &mut Sketch) {
		s.set_framerate(120);
		s.background(Color::RGB(33, 33, 33));
		s.stroke(Color::RGB(0, 220, 0));
	}

	fn draw(&mut self, s: &mut Sketch) {
		for _i in 0..50 {
			self.next_pt();
			let px = map(self.x, -2.1820, 2.6558, 0.0, s.width() as f32) as i32;
			let py = map(self.y, 0.0, 9.9983, s.height() as f32, 0.0) as i32;
			s.point(px, py);
		}
	}
}

fn main() {
	let mut s = Sketch::new(480, 640, "Barnsley fern");
	let mut pt = Point { x: 0.0, y: 0.0 };
	sdl2sketch::run(&mut s, &mut pt);
}

