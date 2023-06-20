
use std::fs::File;
use std::path::PathBuf;

use anyhow::{Context, Result};
use clap::{Parser, ValueEnum};
use pico_detect::{Square, Detector, Localizer, Shaper};

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum, Debug)]
enum ModelType {
    Detector,