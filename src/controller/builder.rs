use std::{mem,ptr};
use channel::Channel;

pub struct Builder(Controller);

impl Builder {
    fn new() -> Self {
        unsafe {
            let mut controller: Controller = mem::zeroed();
            controller.c_struct.freq = 800000;
            return Builder(controller);
        }
    }
    fn freq(&mut self, frequency: u32 ) -> Self {
        self.0.c_struct.freq = frequency;
        return self;
    }
    fn dma(&mut self, dmanum: i32 ) -> Self {
        self.0.c_struct.dmanum = dmanum;
        return self;
    }
    fn channel(&mut self, channel: Channel, idx: u8) -> Self {
        unsafe {
            self.0.c_struct.channel[idx] = channel.c_struct
        }
        self.0.c_struct.channel[idx] = channel;
        return self;
    }
    fn build(&mut self) -> Result<Controller, ()> {
        unsafe {
            return Ok(ptr::read(&self.0));
        }
    }
}