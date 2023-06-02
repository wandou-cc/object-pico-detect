use ab_glyph::FontRef;
use image::{Rgb, RgbImage};
use imageproc::drawing;

use crate::face::Face;

pub fn draw_face(image: &mut RgbImage, face: &Face, font: &FontRef<'_>, scale: f32) {
    drawing::draw_hollow_rect_mut(image, face.region.into(), Rgb([0, 0, 255]));

    let color = Rgb([0, 255, 0]);
    for (i, point) in face.shape.iter().enumerate() {
        let x = point.x as i32;
      