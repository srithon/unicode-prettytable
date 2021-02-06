use std::ptr;

/// Mutable string buffer represented internally with a byte vector
///
/// While core::String allows for mutation, it doesn't support operations necessary for efficient
/// writing
pub struct StringBuffer {
    buffer: Vec<u8>,
    index: usize,
}

impl StringBuffer {
    /// Returns a StringBuffer with a given capacity, and fills the starting buffer with a certain
    /// character
    pub fn with_capacity_fill(num_chars: usize, fill_char: char) -> StringBuffer {
        // TODO refine this
        let buffer = fill_char.to_string().into_bytes().repeat(num_chars * 4);

        StringBuffer { buffer, index: 0 }
    }

    /// Creates a new string buffer with the given capacity
    pub fn with_capacity(num_chars: usize) -> StringBuffer {
        StringBuffer {
            buffer: Vec::with_capacity(num_chars),
            index: 0,
        }
    }

    /// Returns a pointer to self.buffer[index]
    fn get_start_pointer(&mut self) -> *mut u8 {
        unsafe { self.buffer.as_mut_ptr().offset(self.index as isize) }
    }

    /// Writes a byte slice to the internal buffer
    ///
    /// _NOTE_: this function does not update self.index
    fn write(&mut self, bytes: &[u8]) {
        unsafe { ptr::copy_nonoverlapping(bytes.as_ptr(), self.get_start_pointer(), bytes.len()) }
    }

    /// Writes a byte slice to the internal buffer
    ///
    /// Increments the internal pointer by a certain number of characters
    pub fn push_bytes_fixed_width(&mut self, c: &[u8], num_chars: usize) {
        self.write(c);
        self.index += num_chars;
    }

    /// Writes a byte slice to the internal buffer
    ///
    /// Increments the internal pointer by the size of the slice
    pub fn push_bytes(&mut self, c: &[u8]) {
        self.write(c);
        self.index += c.len();
    }

    /// Convenience function for using [push_bytes](StringBuffer::push_bytes) with string slices
    pub fn push_chars(&mut self, c: &str) {
        self.push_bytes(c.as_bytes())
    }

    /// Convenience function for using [push_bytes_fixed_width](StringBuffer::push_bytes_fixed_width) with string slices
    pub fn push_chars_fixed_width(&mut self, c: &str, num_chars: usize) {
        self.push_bytes_fixed_width(c.as_bytes(), num_chars)
    }
}

impl std::fmt::Display for StringBuffer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            std::str::from_utf8(&self.buffer[..self.index]).unwrap()
        )
    }
}
