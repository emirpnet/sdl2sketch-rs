extern crate num_traits;
use num_traits::Float;

const FTOL: f32 = 0.0001;

/// utility function to constrain a value between low and high
pub fn constrain<T: PartialOrd>(x: T, low: T, high: T) -> T {
	if x < low  {
		return low;
	} else if x > high  {
		return high;
	} else {
		return x;
	}
}

/// utility function to map a Float from one value range to another
pub fn map<T: Float>(x: T, x_min: T, x_max: T, out_min: T, out_max: T) -> T {
	out_min + (out_max - out_min) * (x - x_min) / (x_max - x_min)
}

/// utility function to map a Float from one value range to [0,1]
pub fn norm<T: Float>(x: T, x_min: T, x_max: T) -> T {
	(x - x_min) / (x_max - x_min)
}

/// utility function to convert a HSV color value to RGB (EXPERIMENTAL)
///
/// source of algorithm: https://www.rapidtables.com/convert/color/hsv-to-rgb.html
pub fn hsv_to_rgb(hue: u16, sat: f32, val: f32) -> (u8, u8, u8) {
	let c = sat * val;
	let x = (1 - (((hue % 360) as f32 / 60.0).round() as i32 % 2 - 1).abs()) as f32 * c;
	let m = val - c;

	let rgb_pre = match hue % 360 {
		0...60    => (c,x,0.0),
		61...120  => (x,c,0.0),
		121...180 => (0.0,c,x),
		181...240 => (0.0,x,c),
		241...300 => (x,0.0,c),
		301...360 => (c,0.0,x),
		_         => (c,x,0.0)
	};
	
	(((rgb_pre.0+m)*255.0).ceil() as u8, ((rgb_pre.1+m)*255.0).ceil() as u8, ((rgb_pre.2+m)*255.0).ceil() as u8)
}

/// utility function to convert a RGB color value to HSV (EXPERIMENTAL)
///
/// source of algorithm : https://www.rapidtables.com/convert/color/rgb-to-hsv.html
pub fn rgb_to_hsv(r: u8, g: u8, b: u8) -> (u16, f32, f32) {
	let r = r as f32 / 255.0;
	let g = g as f32 / 255.0;
	let b = b as f32 / 255.0;
	let c_min = r.min(g.min(b));
	let c_max = r.max(g.max(b));
	let delta = c_max - c_min;

	let mut hue;
	if delta.abs() < FTOL {
		hue = 0;
	} else if (c_max - r).abs() < FTOL {
		hue = 60 * (((g-b)/delta).round() as i32 % 6);
	} else if (c_max - g).abs() < FTOL {
		hue = 60 * (((b-r)/delta).round() as i32 + 2);
	} else  {
		hue = 60 * (((r-g)/delta).round() as i32 + 4);
	}

	// if result for hue is negative add 360 until it's positive
	while hue < 0 {
		hue += 360;
	}

	let sat;
	if c_max.abs() < FTOL {
		sat = 0.0;
	} else {
		sat = delta / c_max;
	}

	(hue as u16, sat, c_max)
}

