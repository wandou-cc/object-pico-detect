use imageproc::rect::Rect;
use thiserror::Error;

use crate::geometry::Square;

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Multiscaler {
    min_size: u32,
    max_size: u32,
    shift_factor: f32,
    scale_factor: f32,
}

#[derive(Debug, Error)]
pub enum MultiscalerError {
    #[error("`min_size` should be non zero")]
    MinSizeIsZero,
    #[error("`max_size` should be greater than `min_size`")]
    MaxSizeLessThanMinSize,
    #[error("`shift_factor` should be in `(0, 1]` range")]
    ShiftFactorOutOfRange,
    #[error("`scale_factor` should be greater than 1")]
    ScaleFactorLessThanOne,
}

impl Multiscaler {
    #[inline]
    pub fn new(
        min_size: u32,
        max_size: u32,
        shift_factor: f32,
        scale_factor: f32,
    ) -> Res