extern crate sdl2sketch;
use sdl2sketch::utils::*;

use std::f32::consts::PI;

#[test]
fn test_constrain() {
	assert_eq!(constrain(15, 10, 20), 15);
	assert_eq!(constrain(9.9, 10.0, 20.0), 10.0);
	assert_eq!(constrain(100, 10, 20), 20);
}

#[test]
fn test_map() {
	assert_eq!(map(1.0, 0.0, 10.0, 0.0, 100.0), 10.0);
	assert_eq!(map(1.0, 0.0, 10.0, 100.0, 0.0), 90.0);
	assert_eq!(map(1.0, 10.0, 0.0, 0.0, 100.0), 90.0);
	assert_eq!(map(0.0, -1.0, 1.0, 0.0, 100.0), 50.0);
	assert_eq!(map(0.0, -1.0, 1.0, 0.0, -100.0), -50.0);
	assert_eq!(map(8.0, 0.0, 10.0, -10.0, 0.0), -2.0);
	assert_eq!(map(8.0, 0.0, 10.0, 0.0, -10.0), -8.0);
	assert_eq!(map(15.0, 0.0, 10.0, 0.0, 100.0), 150.0);
	assert_eq!(map(-1.0, 0.0, 10.0, 0.0, 100.0), -10.0);
}

#[test]
fn test_norm() {
	assert_eq!(norm(11.0, 10.0, 20.0), 0.1);
	assert_eq!(norm(10.0, 10.0, 20.0), 0.0);
	assert_eq!(norm(20.0, 10.0, 20.0), 1.0);
	assert_eq!(norm(-15.0, -20.0, -10.0), 0.5);
	assert_eq!(norm(1.0, -5.0, 5.0), 0.6);
	assert_eq!(norm(9.0, 10.0, 20.0), -0.1);
	assert_eq!(norm(25.0, 10.0, 20.0), 1.5);
}

#[test]
fn test_rad_to_deg() {
	assert_eq!(rad_to_deg(0.0), 0.0);
	assert_eq!(rad_to_deg(PI/2.0), 90.0);
	assert_eq!(rad_to_deg(PI), 180.0);
	assert_eq!(rad_to_deg(3.0*PI/2.0), 270.0);
}

#[test]
fn test_deg_to_rad() {
	assert_eq!(deg_to_rad(0.0), 0.0);
	assert_eq!(deg_to_rad(90.0), PI/2.0);
	assert_eq!(deg_to_rad(180.0), PI);
	assert_eq!(deg_to_rad(270.0), 3.0*PI/2.0);
}

#[test]
fn test_hsv_to_rgb() {
 	assert_eq!(hsv_to_rgb(  0, 0.0, 0.00), (  0,   0,   0)); // Black
 	assert_eq!(hsv_to_rgb(  0, 0.0, 1.00), (255, 255, 255)); // White
 	assert_eq!(hsv_to_rgb(  0, 1.0, 1.00), (255,   0,   0)); // Red
 	assert_eq!(hsv_to_rgb(120, 1.0, 1.00), (  0, 255,   0)); // Lime
 	assert_eq!(hsv_to_rgb(240, 1.0, 1.00), (  0,   0, 255)); // Blue
 	assert_eq!(hsv_to_rgb( 60, 1.0, 1.00), (255, 255,   0)); // Yellow
 	assert_eq!(hsv_to_rgb(180, 1.0, 1.00), (  0, 255, 255)); // Cyan
 	assert_eq!(hsv_to_rgb(300, 1.0, 1.00), (255,   0, 255)); // Magenta
 	assert_eq!(hsv_to_rgb(  0, 0.0, 0.75), (192, 192, 192)); // Silver
 	assert_eq!(hsv_to_rgb(  0, 0.0, 0.50), (128, 128, 128)); // Gray
 	assert_eq!(hsv_to_rgb(  0, 1.0, 0.50), (128,   0,   0)); // Maroon
 	assert_eq!(hsv_to_rgb( 60, 1.0, 0.50), (128, 128,   0)); // Olive
 	assert_eq!(hsv_to_rgb(120, 1.0, 0.50), (  0, 128,   0)); // Green
 	assert_eq!(hsv_to_rgb(300, 1.0, 0.50), (128,   0, 128)); // Purple
 	assert_eq!(hsv_to_rgb(180, 1.0, 0.50), (  0, 128, 128)); // Teal
 	assert_eq!(hsv_to_rgb(240, 1.0, 0.50), (  0,   0, 128)); // Navy
}

#[test]
fn test_rgb_to_hsv() {
 	assert_eq!(rgb_to_hsv(  0,   0,   0), (  0, 0.0, 0.00)); // Black
 	assert_eq!(rgb_to_hsv(255, 255, 255), (  0, 0.0, 1.00)); // White
 	assert_eq!(rgb_to_hsv(255,   0,   0), (  0, 1.0, 1.00)); // Red
 	assert_eq!(rgb_to_hsv(  0, 255,   0), (120, 1.0, 1.00)); // Lime
 	assert_eq!(rgb_to_hsv(  0,   0, 255), (240, 1.0, 1.00)); // Blue
 	assert_eq!(rgb_to_hsv(255, 255,   0), ( 60, 1.0, 1.00)); // Yellow
 	assert_eq!(rgb_to_hsv(  0, 255, 255), (180, 1.0, 1.00)); // Cyan
 	assert_eq!(rgb_to_hsv(255,   0, 255), (300, 1.0, 1.00)); // Magenta
 	assert_eq!(rgb_to_hsv(192, 192, 192), (  0, 0.0, 0.75)); // Silver
 	assert_eq!(rgb_to_hsv(128, 128, 128), (  0, 0.0, 0.50)); // Gray
 	assert_eq!(rgb_to_hsv(128,   0,   0), (  0, 1.0, 0.50)); // Maroon
 	assert_eq!(rgb_to_hsv(128, 128,   0), ( 60, 1.0, 0.50)); // Olive
 	assert_eq!(rgb_to_hsv(  0, 128,   0), (120, 1.0, 0.50)); // Green
 	assert_eq!(rgb_to_hsv(128,   0, 128), (300, 1.0, 0.50)); // Purple
 	assert_eq!(rgb_to_hsv(  0, 128, 128), (180, 1.0, 0.50)); // Teal
 	assert_eq!(rgb_to_hsv(  0,   0, 128), (240, 1.0, 0.50)); // Navy
}

