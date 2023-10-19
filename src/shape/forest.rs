use core::mem::size_of;
use std::io::{Error, Read};

use image::{GenericImageView, Luma};
use nalgebra::{Affine2, Point2, SimilarityMatrix2};
use pixelutil_image::get_pixel;

use super::delta::ShaperDelta;
use super::tree::ShaperTree;

#[derive(Debug, Clone)]
pub struct ShaperForest {
    deltas: Vec<ShaperDelta>,
    t