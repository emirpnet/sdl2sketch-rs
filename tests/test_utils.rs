extern crate sdl2sketch;
use sdl2sketch::utils::*;

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

