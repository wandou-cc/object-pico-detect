use std::io::{Error, Read};

use nalgebra::Vector2;

use crate::nodes::ThresholdNode;

use super::utils::read_shape;

#[derive(Debug, Clone)]
pub struct ShaperTree {
    nodes: Vec<ThresholdNode>,
    shifts: V