mod detection;
mod detector;
mod padding;

pub mod clusterize;
pub mod multiscale;

use image::{GenericImageView, Luma};
use derive_builder::Builder;

use crate::geometry::Target;

use clusterize::Clusterizer;
use multiscale::Multiscaler;

pub use detection::Detection;
pub use detector::Detector;
pub use padding::Padding;

#[derive(Debug, Clone, Copy, Builder)]
#