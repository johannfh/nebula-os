use alloc::vec::Vec;

use uefi::{
    Result as UefiResult,
    proto::console::gop::{BltOp, BltPixel, BltRegion, GraphicsOutput},
};

pub struct Buffer {
    width: usize,
    height: usize,
    pixels: Vec<BltPixel>,
}

impl Buffer {
    pub fn new(width: usize, height: usize) -> Self {
        let pixels = vec![BltPixel::new(0, 0, 0); width * height];
        Buffer {
            width,
            height,
            pixels,
        }
    }

    #[allow(unused)]
    pub fn pixel(&self, x: usize, y: usize) -> Option<&BltPixel> {
        self.pixels.get(y * self.width + x)
    }

    pub fn pixel_mut(&mut self, x: usize, y: usize) -> Option<&mut BltPixel> {
        self.pixels.get_mut(y * self.width + x)
    }

    #[inline]
    pub fn region_mut(&mut self, coords: (usize, usize), dims: (usize, usize)) -> RegionIterMut {
        let (start_x, start_y) = coords;
        let (width, height) = dims;

        // Ensure the region is within bounds
        // TODO: Handle out-of-bounds gracefully with clamping or error handling
        assert!(start_x + width <= self.width);
        assert!(start_y + height <= self.height);

        RegionIterMut {
            buffer: &mut self.pixels,
            buffer_width: self.width,
            buffer_height: self.height,
            start_x,
            start_y,
            width,
            height,
            x: start_x,
            y: start_y,
        }
    }

    #[inline]
    pub fn blit(&mut self, gop: &mut GraphicsOutput) -> UefiResult {
        gop.blt(BltOp::BufferToVideo {
            buffer: &self.pixels,
            src: BltRegion::Full,
            dest: (0, 0),
            dims: (self.width, self.height),
        })
    }

    #[inline]
    pub fn blit_pixel(&mut self, gop: &mut GraphicsOutput, coords: (usize, usize)) -> UefiResult {
        gop.blt(BltOp::BufferToVideo {
            buffer: &self.pixels,
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
            buffer: &self.pixels,
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

        self.blit(gop)
    }
}

pub struct RegionIterMut<'a> {
    // -- The buffer containing the pixels --
    buffer: &'a mut [BltPixel],

    // -- Dimensions of the buffer --
    buffer_width: usize,
    buffer_height: usize,

    // -- Starting coordinates for the region --
    start_x: usize,
    start_y: usize,

    // -- Dimensions of the region to iterate over --
    width: usize,
    height: usize,

    // -- Current coordinates in the iteration --
    x: usize,
    y: usize,
}

impl<'a> Iterator for RegionIterMut<'a> {
    type Item = &'a mut BltPixel;

    fn next(&mut self) -> Option<Self::Item> {
        // If we've reached the vertical end of the region, return `None`
        if self.y >= self.start_y + self.height {
            return None;
        }

        // If we've reached the vertical end of the buffer, return `None`
        if self.y >= self.buffer_height {
            return None;
        }

        // Calculate the index in the buffer
        let index = self.y * self.buffer_width + self.x;

        // Check if the index is within the bounds of the buffer
        if index >= self.buffer.len() {
            return None;
        }

        // Move to the next pixel
        self.x += 1;
        // If we've reached the horizontal end of the row or the horizontal end of the buffer,
        // reset `x` to `start_x` and increment `y`
        if self.x >= self.start_x + self.width || self.x >= self.buffer_width {
            // Move to the next row
            self.x = self.start_x;
            self.y += 1;
        }

        // Get the pixel at the current coordinates
        // NOTE: We use raw pointer arithmetic to circumvent Rust's borrow checker
        let buf_ptr = self.buffer.as_mut_ptr();
        // SAFETY: We ensure that the index is within bounds and the buffer is valid.
        let pixel = unsafe { buf_ptr.add(index).as_mut() };
        return pixel;
    }
}
