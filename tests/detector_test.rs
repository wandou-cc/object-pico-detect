mod common;

use approx::assert_abs_diff_eq;
use image::GrayImage;
use rstest::rstest;

use pico_detect::{DetectMultiscale, Detector, Square, Target};

use common::{classify_case, detect_multiscale, detect_multiscale_case, detector};

#[rstest]
fn test_detector_classify(detector: Detector, classify_case: (GrayImage, Square, Option<f32>)) {
    let (image,