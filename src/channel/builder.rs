use std::{ptr, slice, mem};
use channel::Channel;
pub struct Builder(Channel);

impl Builder {
    pub fn new() -> Self {
        unsafe {
            let mut channel: Channel = mem::zeroed();
            channel.gpio_pin = &channel.c_struct.gpionum;
            channel.led_count = &channel.c_struct.count;
            channel.invert = &channel.c_struct.invert;
            channel.brightness = &channel.c_struct.brightness;
            return Builder(channel);
        }
    }
    fn pin(&mut self, val: i32) -> Self {
        self.0.c_struct.gpionum = val;
        return self;
    }
    fn led_count(&mut self, val: i32) -> Self {
        self.0.c_struct.count = val;
        return self;
    }
    fn invert(&mut self, val: bool) -> Self {
        match val {
            true => {
                self.0.c_struct.invert = 1_i32;
                return self;
            },
            false => {
                self.0.c_struct.invert = 0_i32;
                return self;
            }
        }
    }
    fn brightness(&mut self, val: u8) -> Self {
        self.0.c_struct.brightness = val;
        return self;
    }
    fn build(&mut self) -> Result<Channel, ()>{
        unsafe {
            return Channel::new(ptr::read(&self.0))
        }
    }
}