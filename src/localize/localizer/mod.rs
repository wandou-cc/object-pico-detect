use std::fmt::Debug;
use std::io::{Error, ErrorKind, Read};

use image::{GenericImageView, Luma};
use nalgebra::{Point2, Translation2, Vector2};

use crate::geometry::Target;
use crate::nodes::ComparisonNode;

type Tree = Vec<ComparisonNode>;
type Predictions = Vec<Vector2<f32>>;
type Stage = Vec<(Tree, Predictions)>;

/// Implements object localization using decision trees.
///
/// Details available [here](https://tehnokv.com/posts/puploc-with-trees/).
#[derive(Clone)]
pub struct Localizer {
    depth: usize,
    dsize: usize,
    scale: f32,
    stages: Vec<Stage>,
}

impl Debug for Localizer {
    fn fmt(&self, f: &mut std