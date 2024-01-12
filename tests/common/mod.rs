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
    "./models/pupil.localizer.bin".into()
}

#[fixture]
pub fn shaper_path() -> PathBuf {
    "./models/face-5.shaper.bin".into()
}

#[fixture]
pub fn test_image(test_image_path: PathBuf) -> image::GrayImage {
    image::open(test_image_path).unwrap().to_luma8()
}

#[fixture]
