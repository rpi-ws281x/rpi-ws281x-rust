use std::slice::{from_raw_parts_mut}

use palette::{LinSrgba, Rgba, Srgb, Linear};

use super::super::util::{RawColor};
use super::super::bindings::{ws2811_channel_t};

#[derive(Deubg)]
pub struct Channel {
    leds: Vec<LinSrgba>,
    raw_leds: &mut[RawColor],
    c_struct: ws2811_channel_t,
}

impl Channel {
    pub unsafe fn new(c_struct: ws2811_channel_t) -> Channel<'a> {
        let raw_leds = from_raw_parts_mut(c_struct.leds as *mut RawColor, c_struct.count as usize);
        Channel {
            leds,
            raw_leds,
            c_struct,
        }
    }
    pub fn render(&mut self) {
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