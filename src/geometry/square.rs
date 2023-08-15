use imageproc::rect::Rect;

use crate::traits::region::Region;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Square {
    pub(crate) left: i32,
    pub(crate) top: i32,
    pub(crate) size: u32,
}

impl Square {
    #[inline]
    pub fn at(x: i32, y: i32) 