use std::{
    fs::File,
    path::{Path, PathBuf}, io::BufReader,
};

use rstest::fixture;

use image::GrayImage;
use nalgebra::Point2;
use rand::SeedableRng;
use rand_xoshiro::Xoroshiro128PlusPlus;

use pico_detect::{
    clusterize::Clusterizer, multiscale::Multiscaler, DetectMultiscale, Detector,
    Localizer, Padding, Shaper, Square, Target, LocalizePerturbate,
};

#[fixture]
pub fn test_image_path() -> PathBuf {
    "./assets/test.png".into()
}

#[fixture]
pub fn detector_path() -> PathBuf {
    "./models/face.detector.bin".into()
}

#[fixture]
pub fn localizer_path() -> PathBuf {
    "./mode