use std::{mem};

use libc::c_int;

use super::super::bindings;
use super::super::util::{StripType};


pub struct ChannelBuilder(bindings::ws2811_channel_t);

impl ChannelBuilder {
    pub fn new() -> Self {
        unsafe {
            ChannelBuilder(mem::zeroed())
        }
    }
    pub fn pin(&mut self, value: i32) -> &mut Self {
        self.0.gpionum = value as c_int;
        self
    }
    pub fn count(&mut self, value: i32) -> &mut Self {
        self.0.count = value as c_int;
        self
    }
    pub fn strip_type(&mut self, value: StripType) -> &mut Self {
        self.0.strip_type = value.into();
        self
    }
    pub fn invert(&mut self, value: bool) -> &mut Self {
        if value {
            self.0.invert = 1 as c_int;
        } else {
            self.0.invert = 0 as c_int;
        }
        self
    }
    pub fn brightness(&mut self, value: u8) -> &mut Self {
        self.0.brightness = value;
        self
    }
    pub fn wshift(&mut self, value: u8) -> &mut Self {
        self.0.wshift = value;
        self
    }
    pub fn rshift(&mut self, value: u8) -> &mut Self {
        self.0.rshift = value;
        self
    }
    pub fn gshift(&mut self, value: u8) -> &mut Self {
        self.0.gshift = value;
        self
    }
    pub fn bshift(&mut self, value: u8) -> &mut Self {
        self.0.bshift = value;
        self
    }
}