use std::slice::{from_raw_parts_mut}

use palette::{LinSrgba, Rgba, Srgb, Linear};

use super::super::util::{RawColor, Result, WS2811Error};
use super::super::bindings::{ws2811_channel_t, ws2811_render};

#[derive(Deubg)]
pub struct Channel {
    c_struct: ws2811_channel_t,
}

impl Channel {
    /// Build a channel from the C struct.
    pub fn new(c_struct: ws2811_channel_t) -> Channel {
        Channel {
            c_struct,
        }
    }

    /// Gets an immutable reference to the colors currently
    /// on the string.  Let's go for immutable and mutable
    /// versions first and see if that's too unergonomic.
    ///
    /// This function is moderately unsafe, it assumes that
    /// the C library holds to its memory layout, which isn't
    /// a total given.
    pub fn leds() -> &[RawColor] {
        unsafe {
            return from_raw_parts(c_struct.leds as *RawColor, c_struct.count as usize);
        }
    }

    /// Get a mutable reference to the leds themselves.
    ///
    /// This function is like moderately unsafe.  This assumes that
    /// the C library actually holds to its memory layout.
    pub fn leds_mut() -> &mut [RawColor] {
        unsafe {
            return from_raw_parts_mut(c_struct.leds as *mut RawColor, c_struct.count as usize);
        }
    }

    /// Render the colors to the string.
    ///
    /// It doesn't automatically do this because it
    /// is a somewhat costly operation that should 
    /// be batched.
    pub fn render(&mut self) -> Result<()> {
        unsafe {
            let ret: WS2811Error = ws2811_render(self.c_struct).into();
            return match ret {
                WS2811Error::Ok => Ok(()),
                x => Err(x),
            }
        }
    }
}


impl<I> Index<I> for Channel
where
    I: std::slice::SliceIndex<RawColor>
{
    type Output = I::Output;

    fn index(&self, index: I) -> &Self::Output {
        Index::index(self.raw_leds, index)
    }
}

impl IndexMut<usize> for Channel
where
    I: std::slice::SliceIndex<RawColor>
{
    type Output = I::Output;

    fn index(&mut self, index: I) -> &mut Self::Output {
        IndexMut::index_mut(self.raw_leds, index)
    }
}