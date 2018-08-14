use std::slice;

use bindings::ws2811_t;
use util::{ws2811_init, ws2811_fini, ws2811_render};
use util::errors::WS281xResult;

#[derive(Debug)]
/// A manager object that provides a safe interface to the low-level rpi-ws281x functions.
pub struct Controller {
    c_struct: ws2811_t
}

impl Controller {

    pub fn new(c_struct: ws2811_t) -> Self {
        Controller {
            c_struct
        }
    }
    /// Initializes the underlying ws2811_t struct based on the partially initialized data.
    ///
    /// See the controller::builder::Builder for what needs to be initialized before calling.
    ///
    /// Note: this structure automatically calls ws2811_fini on Drop to free any internal memory.
    pub fn init(&mut self) -> Result<(), WS281xResult> {
        return ws2811_init(&mut self.c_struct);
    }

    /// Set the LED at index `idx` to `pixel` on the given channel.
    ///
    /// Note: there are a maximum of two channels and most of the time
    ///       channel `0` will be used.
    pub fn set(&mut self, channel_idx: usize, idx: usize, pixel: u32) {
        let channel = self.c_struct.channel[channel_idx];
        unsafe {
            let leds = slice::from_raw_parts_mut(channel.leds, channel.count as usize);
            leds[idx] = pixel;
        }
    }

    /// Get the LED pixel value at index `idx` on the given channel.
    ///
    /// Note: there are a maximum of two channels and most of the time
    ///       channel `0` will be used.
    pub fn get(&self, channel_idx: usize, idx: usize) -> u32 {
        let channel = self.c_struct.channel[channel_idx];
        unsafe {
            let leds = slice::from_raw_parts(channel.leds, channel.count as usize);
            leds[idx]
        }
    }

    /// Send the LED data to the strand.
    pub fn render(&mut self) {
        ws2811_render(&mut self.c_struct as *mut ws2811_t).expect("Unable to render");
    }
}

impl Drop for Controller {
    /// Calls ws2811_fini on to free any internal memory from init().
    fn drop(&mut self) {
        ws2811_fini(&mut self.c_struct);
    }
}
