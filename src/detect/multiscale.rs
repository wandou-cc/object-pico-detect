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
    ) -> Result<Self, MultiscalerError> {
        if min_size == 0 {
            return Err(MultiscalerError::MinSizeIsZero);
        }

        if min_size > max_size {
            return Err(MultiscalerError::MaxSizeLessThanMinSize);
        }

        if !(0.0..=1.0).contains(&shift_factor) {
            return Err(MultiscalerError::ShiftFactorOutOfRange);
        }

        if scale_factor < 1.0 {
            return Err(MultiscalerError::ScaleFactorLessThanOne);
        }

        Ok(Self {
            min_size,
            max_size,
            shift_factor,
            scale_factor,
        })
    }

    pub fn min_size(&self) -> u32 {
        self.min_size
    }

    pub fn max_size(&self) -> u32 {
        self.max_size
    }

    pub fn shift_factor(&self) -> f32 {
        self.shift_factor
    }

    pub fn scale_factor(&self) -> f32 {
        self.scale_factor
    }

    #[inline]
    pub fn run<F>(&self, rect: Rect, f: F)
    where
        F: FnMut(Square),
    {
        multiscale(
            self.min_size,
            self.max_size,
            self.shift_factor,
            self.scale_factor,
