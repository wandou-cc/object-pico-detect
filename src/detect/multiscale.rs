use imageproc::rect::Rect;
use thiserror::Error;

use crate::geometry::Square;

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Multiscaler {
    min_size: u32,
    max_size: u32,
    shift_factor: f32,
    scale_factor: 