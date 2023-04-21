#![macro_use]
#![allow(unused_macros)]

macro_rules! load_test_image {
    () => {
        image::open("./assets/test.png")
            .expect("failed to load test image")
            .to_luma8()
    };
}

macro_rules! model_path {
    (facefinder) => {
        std::path::Path::new("./models/face.detector.bin")
    };

    (puploc) => {
        std::pat