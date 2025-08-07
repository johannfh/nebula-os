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
    /// Creates a new buffer with the specified width and height, filling it with the given value.
    /// # Arguments
    /// * `width` - The width of the buffer in pixels.
    /// * `height` - The height of the buffer in pixels.
    /// * `fill` - The value to fill the buffer with.
    /// # Example
    ///
    /// ```
    /// # use nebula_core::buffer::Buffer;
    /// let buffer = Buffer::<u8>::new(10, 10, 0);
    /// assert_eq!(buffer.width(), 10);
    /// assert_eq!(buffer.height(), 10);
    /// for y in 0..10 {
    ///     for x in 0..10 {
    ///         assert_eq!(*buffer.pixel(x, y).unwrap(), 0);
    ///     }
    /// }
    /// ```
    pub fn new<F>(width: usize, height: usize, fill: F) -> Self
    where
        F: Into<T>,
    {
        let fill: T = fill.into();
        let inner = vec![fill; width * height];
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

    /// Resizes the buffer to the new dimensions, filling new pixels with the specified value.
    /// /// # Arguments
    /// * `new_width` - The new width of the buffer.
    /// * `new_height` - The new height of the buffer.
    /// * `fill` - The value to fill the new pixels with.
    ///
    /// # Example
    ///
    /// ```
    /// # use nebula_core::buffer::Buffer;
    /// let mut buffer = Buffer::<u8>::new(10, 10, 0);
    /// buffer.resize(5, 5, 1);
    /// assert_eq!(buffer.width(), 5);
    /// assert_eq!(buffer.height(), 5);
    /// for y in 0..5 {
    ///     for x in 0..5 {
    ///         assert_eq!(*buffer.pixel(x, y).unwrap(), 0);
    ///     }
    /// }
    /// ```
    pub fn resize(&mut self, new_width: usize, new_height: usize, fill: T) {
        // Create a new buffer with the new dimensions
        let mut new_inner = vec![fill; new_width * new_height];

        // Copy the existing data into the new buffer
        for y in 0..self.height.min(new_height) {
            for x in 0..self.width.min(new_width) {
                // Use value of same row or use fill value if out of bounds
                let src_index = y * self.width + x;
                let dst_index = y * new_width + x;
                if src_index < self.inner.len() {
                    new_inner[dst_index] = self.inner[src_index];
                } else {
                    new_inner[dst_index] = fill; // Fill value for out of bounds
                }
            }
        }

        // Update the width and height
        self.width = new_width;
        self.height = new_height;

        // Replace the inner buffer with the new one
        self.inner = new_inner;
    }

    /// Fills the entire buffer with the specified value.
    ///
    /// # Arguments
    /// * `value` - The value to fill the buffer with.
    ///
    /// # Example
    ///
    /// ```
    /// # use nebula_core::buffer::Buffer;
    /// let mut buffer = Buffer::<u8>::new(10, 10, 0);
    /// buffer.fill(5);
    /// for y in 0..10 {
    ///     for x in 0..10 {
    ///         assert_eq!(*buffer.pixel(x, y).unwrap(), 5);
    ///     }
    /// }
    /// ```
    pub fn fill(&mut self, value: T) {
        for item in &mut self.inner {
            *item = value;
        }
    }
}

impl<T> AsRef<[T]> for Buffer<T> {
    fn as_ref(&self) -> &[T] {
        &self.inner
    }
}

impl<T> AsMut<[T]> for Buffer<T> {
    fn as_mut(&mut self) -> &mut [T] {
        &mut self.inner
    }
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

    #[test]
    fn test_resize_buffer() {
        let mut buffer = Buffer::<u8>::new(10, 10, 0);
        buffer.fill(1);
        buffer.resize(5, 5, 0);
        assert_eq!(buffer.width, 5);
        assert_eq!(buffer.height, 5);
        assert_eq!(buffer.inner.len(), 25);
        for y in 0..5 {
            for x in 0..5 {
                assert_eq!(*buffer.pixel(x, y).unwrap(), 1);
            }
        }
    }
}
