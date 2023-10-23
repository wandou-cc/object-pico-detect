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
    trees: Vec<ShaperTree>,
}

impl ShaperForest {
    #[cfg(test)]
    pub fn trees(&self) -> usize {
        self.trees.len()
    }

    #[cfg(test)]
    pub fn deltas(&self) -> usize {
        self.deltas.len()
    }

    #[cfg(test)]
    pub fn tree(&self, index: usize) -> &ShaperTree {
        self.trees.get(index).unwrap()
    }

    #[inline]
    pub fn trees_slice(&self) -> &[ShaperTree] {
        &self.trees
    }

    #[inline]
    pub(super) fn extract_features<I>(
        &self,
        image: &I,
        transform_