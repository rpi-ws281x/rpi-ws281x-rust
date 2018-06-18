use std::{mem, ptr};

use libc::{c_int};

use super::super::bindings::{ws2811_t, ws2811_init, ws2811_return_t};
use super::super::channel::{Channel, ChannelBuilder};

pub struct ControllerBuilder(ws2811_t);

impl ControllerBuilder {
    pub fn new() -> Self {
        unsafe {
            return ControllerBuilder(mem::zeroed());
        }
    }
    pub fn freq(&mut self, value: u32) -> &mut Self {
        self.0.freq = value;
        self
    }
    pub fn channel(&mut self, index: usize, channel: ChannelBuilder) -> &mut Self {
        self.0.channel[index] = channel.0;
        self
    }
    pub fn dma(&mut self, value: i32) -> &mut Self {
        self.0.dmanum = value as c_int;
        self
    }
    pub fn render_wait_time(&mut self, value: u64) -> &mut Self {
        self.0.render_wait_time = value;
        self
    }
    pub fn build(&mut self) -> Result<Controller, ()> {
        unsafe {
            let ret = ws2811_init(&mut self.0);
            if ret != ws2811_return_t::WS2811_SUCCESS {
                Err(());
            } else {
                return Controller::new(self.0, self.0.device, self.0.rpi_hw);
            }
        }
    }
}