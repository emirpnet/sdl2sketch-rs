# SDL2Sketch for Rust
SDL2Sketch for Rust tries to simplify the use of [rust-sdl2](https://github.com/Rust-SDL2/rust-sdl2) by following the style of the [p5.js](https://p5js.org) API. It does not try to be a complete game engine, but just wants to make it as easy as possible to create visual applications in Rust without much boilerplate code.

**The library is mostly still a work in progress!**

## Basic example
The code for a simple bouncing ball looks like this. It can be run with ```cargo run --release --example basic_example```.
```rust
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
```

## Further examples
```shell
cargo run --release --example demo
cargo run --release --example barnsleyfern
cargo run --release --example gameoflife
```

## Documentation
The documentation of the API can be built via ```cargo doc``` or be found [here](https://emirpnet.github.io/rustdoc/sdl2sketch/).
