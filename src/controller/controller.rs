use std::slice::{from_raw_parts, from_raw_parts_mut};

use super::super::bindings::{ws2811_fini, ws2811_render, ws2811_t};
use super::super::util::{Result, RawColor};

#[derive(Debug)]
pub struct Controller {
    c_struct: ws2811_t,
}

impl Controller {
    /// Creates a new Controller
    /// Note: This is only to be called from the Builder struct, because some unsafe
    /// things have to be done, and thus it is appropriately marked unsafe.
    pub unsafe fn new(c_struct: ws2811_t) -> Self {
        Controller { c_struct }
    }

    /// Render the colors to the string.
    ///
    /// It doesn't automatically do this because it
    /// is a somewhat costly operation that should
    /// be batched.
    pub fn render(&mut self) -> Result<()> {
        unsafe {
            return ws2811_render(&mut self.c_struct).into();
        }
    }

    /// Gets a slice view to the color array to be written to the LEDs.
    /// See `leds_mut` for a mutable slice view to this data.
    ///
    /// ## Safety
    /// This function is moderately unsafe because we rely on the promise
    /// from the C library that it will stick to its memory layout and that
    /// the pointer is valid.
    pub fn leds(&mut self, channel: usize) -> &[RawColor] {
        /*
         * Using unsafe here because we want to construct a slice
         * from just the raw pointer and the supposed number of elements
         * which is safe as long as our friends in "C land" hold to their
         * memory layout and we use a data type with compatible layout.
         */
        unsafe {
            from_raw_parts(
                self.c_struct.channel[channel].leds as *const RawColor,
                self.c_struct.channel[channel].count as usize
            )
        }
    }

    /// Gets a mutable slice pointing to the color array to be written to
    /// the LEDs.
    ///
    /// ## Safety
    /// This function is moderately unsafe because we rely on the promise
    /// from the C library that it will stick to its memory layout and that
    /// the pointer is valid.
    pub fn leds_mut(&mut self, channel: usize) -> &mut [RawColor] {
        /*
         * Using unsafe here because we want to construct a slice
         * from just the raw pointer and the supposed number of elements
         * which is safe as long as our friends in "C land" hold to their
         * memory layout and we use a data type with compatible layout.
         */
        unsafe {
            from_raw_parts_mut(
                self.c_struct.channel[channel].leds as *mut RawColor,
                self.c_struct.channel[channel].count as usize
            )
        }
    }
}
impl Drop for Controller {
    fn drop(&mut self) {
        unsafe {
            ws2811_fini(&mut self.c_struct);
        }
    }
}
