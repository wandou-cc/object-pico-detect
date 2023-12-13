mod delta;
mod forest;
mod tree;
mod utils;

use std::{
    fmt::Debug,
    io::{Error, ErrorKind, Read},
};

use image::{GenericImageView, Luma};
use imageproc::rect::Rect;
use nalgebra::{Affine2, DimName, Matrix3, Point2, SimilarityMatrix2, U2};

use forest::ShaperForest;

/// Implements object alignment using an ensemble of regression trees.
#[derive(Clone)]
pub struct Shaper {
    depth: usize,
    dsize: usize,
    shape: Vec<Point2<f32>>,
    forests: Vec<ShaperForest>,
}

impl Debug for Shaper {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct(stringify!(Shaper))
            .field("depth", &self.depth)
            .field("dsize", &self.dsize)
            .field("shape", &self.shape.len())
            .field("forests", &self.forests.len())
            .finish()
    }
}

impl Shaper {
    #[inline]
    pub fn size(&self) -> usize {
        self.shape.len()
    }

    #[inline]
    pub fn init_points(&self) -> &[Point2<f32>] {
        self.shape.as_ref()
    }

    /// Create a shaper object from a readable source.
    #[inline]
    pub fn load<R: Read>(mut reader: R) -> Result<Self, Error> {
        let mut buf = [0u8; 4];
        reader.read_exact(&mut buf[0..1])?;
        let version = buf[0];
        if version != 1 {
            return Err(Error::new(ErrorKind::InvalidData, "wrong version"));
        }

        reader.read_exact(&mut buf)?;
        let nrows = u32::from_be_bytes(buf) as usize;

        reader.read_exact(&mut buf)?;
        let ncols = u32::from_be_bytes(buf) as usize;

        let shape_size = nrows * ncols / U2::USIZE;

        reader.read_exact(&mut buf)?;
        let nforests = u32::from_be_bytes(buf) as usize;

        reader.read_exact(&mut buf)?;
        let forest_size = u32::from_be_bytes(buf) as usize;

        reader.read_exact(&mut buf)?;
        let tree_depth = u32::from_be_bytes(buf);

        reader.read_exact(&mut buf)?;
        let nfeatures = u32::from_be_bytes(buf) as usize;

        let shifts_count = 2u32.pow(tree_depth) as usize;
        let nodes_count = shifts_count - 1;

        // dbg!(nrows, ncols, nforests, forest_size, tree_depth, nfeatures);
        let shape: Vec<Point2<f32>> = utils::read_shape(reader.by_ref(), shape_size)?
            .column_iter()
            .map(|col| Point2::new(col.x, col.y))
            .collect();

        let mut forests = Vec::with_capacity(nforests);
        for _ in 0..nforests {
            forests.push(ShaperForest::load(
                reader.by_ref(),
                forest_size,
                nodes_count,
                shifts_count,
                shape_size,
                nfeatures,
            )?);
        }

        Ok(Self {
            depth: tree_depth as usize,
            dsize: nodes_count,
            shape,
            forests,
        })
    }

    /// Estimate object shape on the image
    ///
    /// ### Arguments
    ///
    /// * `image` - Target image.
    /// TODO:
    ///
    /// ### Returns
    ///
    /// A collection of points each one corresponds to landmark location.
    /// Points count is defined by a loaded shaper model.
    #[inline]
    pub fn shape<I>(&self, image: &I, rect: Rect) -> Vec<Point2<f32>>
    where
        I: GenericImageView<Pixel = Luma<u8>>,
    {
        let mut shape = self.shape.clone();

        let transform_to_image = find_transform_to_image(rect);

        for forest in self.forests.iter() {
            let transform_to_shape = Self::find_transform(self, shape.as_slice());

            let features =
                forest.extract_features(image, &transform_to_shape, &transform_to_image, &shape);

            for tree in forest.trees_slice().iter() {
                let idx = (0..self.depth).fold(0, |idx, _| {
                    2 * idx + 1 + tree.node(idx).bintest(features.as_slice()) as usize
                }) - self.dsize;

                shape.iter_mut().zip(tree.shift(idx).iter()).for_each(
                    |(shape_point, shift_vector)| {
                        *shape_point += shift_vector;
                    },
                );
            }
        }

        shape
            .iter_mut()
            .for_each(|point| *point 