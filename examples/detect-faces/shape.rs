use nalgebra::Point2;
use pico_detect::Square;

pub enum Shape5 {
    LeftOuterEyeCorner = 0,
    LeftInnerEyeCorner = 1,
    RightOuterEyeCorner = 2,
    RightInnerEyeCorner = 3,
    #[allow(dead_code)]
    Nose = 4,
}

impl Shape5 {
    pub fn size() -> usize {
        5
    }

    #[allow(dead_code)]
    pub fn find_eye_centers(shape: &[Point2