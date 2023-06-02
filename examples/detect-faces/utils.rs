use ab_glyph::FontRef;
use image::{Rgb, RgbImage};
use imageproc::drawing;

use crate::face::Face;

pub fn draw_face(image: &mut RgbImage, face: &Face, font: &FontRef<'_>, scale: f32) {
    drawing::draw_hollow_rect_mut(image, face.region.into(), Rgb([0, 0, 255]));

    let color = Rgb([0, 255, 0]);
    for (i, point) in face.shape.iter().enumerate() {
        let x = point.x as i32;
        let y = point.y as i32;
        drawing::draw_cross_mut(image, color, x, y);
        drawing::draw_text_mut(image, color, x + 1, y + 1, scale, font, &format!("{}", i));
    }

    let color = Rgb([255, 0, 0]);
    drawing::draw_cross_mut(image, color, face.pupil