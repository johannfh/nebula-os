use alloc::vec::Vec;

pub struct Buffer<T> {
    /// The width of the buffer in pixels.
    width: usize,
    /// The height of the buffer in pixels.
    height: usize,
    /// The internal buffer containing the data.
    inner: Vec<T>,
}

impl<T> Buffer<T>
where
    T: Clone + Copy,
{
    pub fn new<F>(width: usize, height: usize, fill: F) -> Self
    where
        F: Into<T>,
    {
        let fill: T = fill.into();
        let inner = vec![fill.clone(); width * height];
        Buffer {
            width,
            height,
            inner,
        }
    }

    #[allow(unused)]
    pub fn pixel(&self, x: usize, y: usize) -> Option<&T> {
        self.inner.get(y * self.width + x)
    }

    pub fn pixel_mut(&mut self, x: usize, y: usize) -> Option<&mut T> {
        self.inner.get_mut(y * self.width + x)
    }

    #[inline]
    pub fn region_mut(&mut self, coords: (usize, usize), dims: (usize, usize)) -> RegionIterMut<T> {
        let (start_x, start_y) = coords;
        let (width, height) = dims;

        // Ensure the region is within bounds
        // TODO: Handle out-of-bounds gracefully with clamping or error handling
        assert!(start_x + width <= self.width);
        assert!(start_y + height <= self.height);

        RegionIterMut {
            buffer: &mut self.inner,
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
    pub fn width(&self) -> usize {
        self.width
    }

    #[inline]
    pub fn height(&self) -> usize {
        self.height
    }

    /* TODO: Abstract the following logic away into a super type / wrapper around GraphicsOutput

    #[inline]
    pub fn blit(&mut self, gop: &mut GraphicsOutput) -> UefiResult {
        gop.blt(BltOp::BufferToVideo {
            buffer: &self.inner,
            src: BltRegion::Full,
            dest: (0, 0),
            dims: (self.width, self.height),
        })
    }

    #[inline]
    pub fn blit_pixel(&mut self, gop: &mut GraphicsOutput, coords: (usize, usize)) -> UefiResult {
        gop.blt(BltOp::BufferToVideo {
            buffer: &self.inner,
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
            buffer: &self.inner,
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
    }*/
}

pub struct RegionIterMut<'a, T> {
    // -- The buffer containing the pixels --
    buffer: &'a mut [T],

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

impl<'a, T> Iterator for RegionIterMut<'a, T> {
    type Item = &'a mut T;

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

        // Get the item at the current coordinates
        // NOTE: We use raw pointer arithmetic to circumvent Rust's borrow checker
        let buf_ptr = self.buffer.as_mut_ptr();
        // SAFETY: We ensure that the index is within bounds and the buffer is valid.
        unsafe { buf_ptr.add(index).as_mut() }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_buffer_creation() {
        let buffer = Buffer::<u8>::new(10, 10, 0);
        assert_eq!(buffer.width, 10);
        assert_eq!(buffer.height, 10);
        assert_eq!(buffer.inner.len(), 100);
    }

    #[test]
    fn test_pixel_access() {
        let mut buffer = Buffer::<u8>::new(10, 10, 0);
        *buffer.pixel_mut(5, 5).unwrap() = 42;
        assert_eq!(*buffer.pixel(5, 5).unwrap(), 42);
    }

    #[test]
    fn test_region_mut() {
        let mut buffer = Buffer::<u8>::new(10, 10, 0);
        buffer.region_mut((2, 2), (3, 3)).for_each(|pixel| {
            *pixel = 1;
        });

        for y in 2..5 {
            for x in 2..5 {
                assert_eq!(*buffer.pixel(x, y).unwrap(), 1);
            }
        }
    }
}
