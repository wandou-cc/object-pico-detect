
use anyhow::{Context, Result};
use clap::Parser;
use image::DynamicImage;
use pico_detect::{
    clusterize::Clusterizer, multiscale::Multiscaler, DetectMultiscale, Detector,
    LocalizePerturbate, Localizer, Padding, Shaper,
};
use std::path::PathBuf;
