mod delta;
mod forest;
mod tree;
mod utils;

use std::{
    fmt::Debug,
    io::{Error, ErrorKind, Read},
};

use image::{GenericImageView, Luma};
use imageproc::rect::Rect;
use nalgebra::{Affine2, DimName, Matrix3, Point2, SimilarityMatrix2, U2};

use forest::ShaperForest;

/// Implements object alignment using an ensemble of regression trees.
#[derive(Clone)]
pub struct Shaper {
    depth: usize,
    dsize: usize,
    shape: Vec<Point2<f32>>,
    forests: Vec<ShaperForest>,
}

impl Debug for Shaper {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct(stringify!(Shaper))
            .field("depth", &self.depth)
            .field("dsize", &self.dsize)
            .field("shape", &self.shape.len())
            .field("forests", &self.forests.len())
            .finish()
    }
}

impl Shaper {
    #[inline]
    pub fn size(&