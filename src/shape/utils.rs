use std::io::{Error, Read};

use nalgebra::{allocator::Allocator, DefaultAllocator, Dim, Dyn, OMatrix, UninitMatrix, U2};

#[inline]
pub fn read_shape<R: Read>(mut reader: R, size: usize) -> Result<OMatrix<f32, U2, Dyn>, Error>
where
    DefaultAllocator: Allocator<f32, U2, Dyn>,
{
    let mut m = UninitMatrix::<f32, U2, Dyn>::uninit(U2, Dyn::from_usize(size));

    let mut bu