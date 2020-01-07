use std::slice::{from_raw_parts, from_raw_parts_mut};

use super::super::bindings::{ws2811_fini, ws2811_render, ws2811_t};
use super::super::util::{RawColor, Result};

/// The main struct used to control lights.  Provides ways of
/// accessing the light color values and rendering those values to
/// the string.
#[derive(Clone, Debug)]
pub struct Controller {
    c_struct: ws2811_t,
}

impl Controller {
    /// Creates a new Controller
    ///
    /// Note: This is only to be called from the Builder struct
    pub fn new(c_struct: ws2811_t) -> Self {
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

    /// Gets the channels with non-zero number of LED's associated with them.
    ///
    /// I know this is somewhat non-intuitive, but naming it something like
    /// `active_channels(&self)` seemed overly verbose.
    pub fn channels(&self) -> Vec<usize> {
        (0..self.c_struct.channel.len())
            .filter(|x: _| self.c_struct.channel[x.clone()].count > 0)
            .collect::<Vec<_>>()
    }

    /// Gets the brightness of the LEDs
    pub fn brightness(&self, channel: usize) -> u8 {
        self.c_struct.channel[channel].brightness
    }

    /// Sets the brighness of the LEDs
    pub fn set_brightness(&mut self, channel: usize, value: u8) {
        self.c_struct.channel[channel].brightness = value;
    }

    /// Gets a slice view to the color array to be written to the LEDs.
    /// See `leds_mut` for a mutable slice view to this data.
    ///
    /// # Safety
    /// This function is moderately unsafe because we rely on the promise
    /// from the C library that it will stick to its memory layout and that
    /// the pointer is valid.
    pub fn leds(&self, channel: usize) -> &[RawColor] {
        /*
         * Using unsafe here because we want to construct a slice
         * from just the raw pointer and the supposed number of elements
         * which is safe as long as our friends in "C land" hold to their
         * memory layout and we use a data type with compatible layout.
         */
        unsafe {
            from_raw_parts(
                self.c_struct.channel[channel].leds as *const RawColor,
                self.c_struct.channel[channel].count as usize,
            )
        }
    }

    /// Gets a mutable slice pointing to the color array to be written to
    /// the LEDs.
    ///
    /// # Safety
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
                self.c_struct.channel[channel].count as usize,
            )
        }
    }
}

impl Drop for Controller {
    fn drop(&mut self) {
        /*
         * Unsafe used here because we need to call an externed
         * function during the drop process.  Unfortunately,
         * I don't have a better way of dealing with this.
         */
        unsafe {
            ws2811_fini(&mut self.c_struct);
        }
    }
}
