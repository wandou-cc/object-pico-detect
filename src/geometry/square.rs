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
    pub fn at(x: i32, y: i32) -> Self {
        Self {
            left: x,
            top: y,
            size: 1,
        }
    }

    #[inline]
    pub fn of_size(mut self, value: u32) -> Self {
        self.size = value;
        self
    }

    #[inline]
    pub fn from_region<T: Region>(value: T) -> Re