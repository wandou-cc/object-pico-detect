use std::io::{Error, Read};

use nalgebra::Vector2;

use crate::nodes::ThresholdNode;

use super::utils::read_shape;

#[derive(Debug, Clone)]
pub struct ShaperTree {
    nodes: Vec<ThresholdNode>,
    shifts: Vec<Vec<Vector2<f32>>>,
}

impl ShaperTree {
    #[cfg(test)]
    pub fn nodes(&self) -> usize {
        self.nodes.len()
    }

    #[cfg(test)]
    pub fn shifts(&self) -> usize {
        self.shifts.len()
    }

    #[inline]
    pub fn node(&self, index: usize) -> &Threshold