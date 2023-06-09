use std::{fs::File, io::BufReader};
use std::path::PathBuf;

use anyhow::{Context, Result};
use clap::Parser;
use image::{Rgb, RgbImage};
use imageproc::{
    drawing::{draw_filled_circle_mut, draw_text_mut},
    geometry::min_area_rect,
    point::Point,
    rect::Rect,
};
use pico_detect::Shaper;
use ab_glyph::FontRef;


#[derive(Parser, Debug)]
#[co