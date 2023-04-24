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
        std::path::Path::new("./models/pupil.localizer.bin")
    };

    (shaper) => {
        std::path::Path::new("./models/face-5.shaper.bin")
    };
}

macro_rules! model_file {
    (facefinder) => {
        std::fs::File::open("./models/face.detector.bin")
            .expect("cannot open facefinder model file")
    };

    (puploc) => {
        std::fs::File::open("./models/pupil.localizer.bin")
            .expect("cannot open puploc model file")
    };

    (shaper) => {
        std::fs::File::open("./models/face-5.shaper.bin")
            .expect("cannot open shaper mo