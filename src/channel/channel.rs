use std::slice::{from_raw_parts, from_raw_parts_mut};

use super::super::bindings::ws2811_channel_t;
use super::super::util::RawColor;

#[derive(Debug)]
pub struct Channel<'a> {
    c_struct: &'a mut ws2811_channel_t,
}

impl<'a> Channel<'a> {
    /// Build a channel from the C struct.
    pub fn new(c_struct: &'a mut ws2811_channel_t) -> Channel<'a> {
        Channel { c_struct }
    }

    /// Gets an immutable reference to the colors currently
    /// on the string.  Let's go for immutable and mutable
    /// versions first and see if that's too unergonomic.
    ///
    /// This function is moderately unsafe, it assumes that
    /// the C library holds to its memory layout, which isn't
    /// a total given.
    pub fn leds(&self) -> &[RawColor] {
        unsafe {
            return from_raw_parts(
                self.c_struct.leds as *const RawColor,
                self.c_struct.count as usize,
            );
        }
    }

    /// Get a mutable reference to the leds themselves.
    ///
    /// This function is like moderately unsafe.  This assumes that
    /// the C library actually holds to its memory layout.
    pub fn leds_mut(&mut self) -> &mut [RawColor] {
        unsafe {
            return from_raw_parts_mut(
                self.c_struct.leds as *mut RawColor,
                self.c_struct.count as usize,
            );
        }
    }
}
