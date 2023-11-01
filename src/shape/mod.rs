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
