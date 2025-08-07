use core::ops::{Deref, DerefMut};

use alloc::vec::Vec;
use nebula_core::buffer::Buffer;

use uefi::{
    Result as UefiResult,
    proto::console::gop::{BltOp, BltPixel, BltRegion, GraphicsOutput},
};

pub struct Screen {
    pub width: usize,
    pub height: usize,
    buffer: Buffer<BltPixel>,
}

impl Screen {
    pub fn new(width: usize, height: usize) -> Self {
        let buffer = Buffer::new(width, height, BltPixel::new(0, 0, 0));
        Self {
            width,
            height,
            buffer,
        }
    }

    pub fn resize(&mut self, width: usize, height: usize) {
        self.width = width;
        self.height = height;
        self.buffer.resize(width, height, BltPixel::new(0, 0, 0));
    }

    #[inline]
    pub fn blit(&self, gop: &mut GraphicsOutput) -> UefiResult {
        gop.blt(BltOp::BufferToVideo {
            buffer: &self.buffer.as_ref(),
            src: BltRegion::Full,
            dest: (0, 0),
            dims: (self.width, self.height),
        })
    }

    #[inline]
    pub fn blit_pixel(&self, gop: &mut GraphicsOutput, coords: (usize, usize)) -> UefiResult {
        gop.blt(BltOp::BufferToVideo {
            buffer: &self.buffer.as_ref(),
            src: BltRegion::SubRectangle {
                coords,
                px_stride: self.width,
            },
            dest: coords,
            dims: (1, 1),
        })
    }

    #[inline]
    pub fn blit_region(
        &mut self,
        gop: &mut GraphicsOutput,
        coords: (usize, usize),
        dims: (usize, usize),
    ) -> UefiResult {
        gop.blt(BltOp::BufferToVideo {
            buffer: &self.buffer.as_ref(),
            src: BltRegion::SubRectangle {
                coords,
                px_stride: self.width,
            },
            dest: coords,
            dims,
        })
    }

    #[inline]
    pub fn draw_rect(
        &mut self,
        gop: &mut GraphicsOutput,
        coords: (usize, usize),
        dims: (usize, usize),
        color: BltPixel,
    ) -> UefiResult {
        self.region_mut(coords, dims).for_each(|pixel| {
            *pixel = color;
        });

        self.blit_region(gop, coords, dims)
    }

    pub fn draw_char(
        &mut self,
        gop: &mut GraphicsOutput,
        bitmap: Vec<u8>,
        coords: (usize, usize),
        dims: (usize, usize),
    ) -> UefiResult {
        let (x, y) = coords;
        let (width, _height) = dims;

        for (i, pixel) in bitmap.iter().enumerate() {
            let char_x = x + (i % width);
            let char_y = y + (i / width);
            if char_x < self.width && char_y < self.height {
                if let Some(blt) = self.buffer.pixel_mut(char_x, char_y) {
                    *blt = BltPixel::new(*pixel, *pixel, *pixel);
                }
            }
        }

        self.blit_region(gop, coords, dims)
    }
}

impl Deref for Screen {
    type Target = Buffer<BltPixel>;

    fn deref(&self) -> &Self::Target {
        &self.buffer
    }
}

impl DerefMut for Screen {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.buffer
    }
}
