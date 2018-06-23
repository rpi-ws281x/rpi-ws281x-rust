use std::mem;

use libc::{c_int, c_uint};

use super::super::bindings;
use super::super::util::StripType;

/// A struct used to build a channel configuration for
/// a ControllerBuilder.
pub struct ChannelBuilder(pub bindings::ws2811_channel_t);

impl ChannelBuilder {
    /// Creates a new ChannelBuilder
    pub fn new() -> Self {
        unsafe { ChannelBuilder(mem::zeroed()) }
    }
    /// Sets the GPIO pin used by this channel
    pub fn pin(&mut self, value: i32) -> &mut Self {
        self.0.gpionum = value as c_int;
        self
    }
    /// Sets the number of LEDs attached to this channel
    pub fn count(&mut self, value: i32) -> &mut Self {
        self.0.count = value as c_int;
        self
    }
    /// Sets the strip type of this channel
    pub fn strip_type(&mut self, value: StripType) -> &mut Self {
        let tmp: c_uint = value.into();
        self.0.strip_type = tmp as i32;
        self
    }
    /// Sets the invert flag on the channel
    pub fn invert(&mut self, value: bool) -> &mut Self {
        if value {
            self.0.invert = 1 as c_int;
        } else {
            self.0.invert = 0 as c_int;
        }
        self
    }
    /// Sets the brightness of this channel
    pub fn brightness(&mut self, value: u8) -> &mut Self {
        self.0.brightness = value;
        self
    }
    /// Sets the white shift of this channel
    pub fn wshift(&mut self, value: u8) -> &mut Self {
        self.0.wshift = value;
        self
    }
    /// Sets the red shift of this channel
    pub fn rshift(&mut self, value: u8) -> &mut Self {
        self.0.rshift = value;
        self
    }
    /// Sets the green shift of this channel
    pub fn gshift(&mut self, value: u8) -> &mut Self {
        self.0.gshift = value;
        self
    }
    /// Sets the blue shift of this channel
    pub fn bshift(&mut self, value: u8) -> &mut Self {
        self.0.bshift = value;
        self
    }
}
