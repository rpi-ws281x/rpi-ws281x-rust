use std::{mem, ptr};

use libc::c_int;

use super::super::bindings::{ws2811_init, ws2811_t};
use super::super::channel::ChannelBuilder;
use super::super::util::Result;

use super::controller::Controller;

/// A struct to assist in the process of initializing
/// a Controller instance.  One is pretty much required to use
/// this to construct a Controller, unless one happens to have
/// an instance of ws2811_t sitting around.
#[derive(Debug)]
pub struct ControllerBuilder(pub ws2811_t);

impl ControllerBuilder {
    /// Create a new ControllerBuilder
    pub fn new() -> Self {
        unsafe {
            return ControllerBuilder(mem::zeroed());
        }
    }
    /// Sets the frequency of the signal to the LED's,
    /// usually like 800kHz IIRC.
    pub fn freq(&mut self, value: u32) -> &mut Self {
        self.0.freq = value;
        self
    }
    /// Sets up a channel on the Controller, there should
    /// be two per controller for current versions of the driver.
    pub fn channel(&mut self, index: usize, channel: ChannelBuilder) -> &mut Self {
        self.0.channel[index] = channel.0;
        self
    }
    /// Sets the DMA channel of the controller
    pub fn dma(&mut self, value: i32) -> &mut Self {
        self.0.dmanum = value as c_int;
        self
    }
    /// Sets the time to wait before rendering for this controller
    pub fn render_wait_time(&mut self, value: u64) -> &mut Self {
        self.0.render_wait_time = value;
        self
    }
    /// Attempts to build and initialize the Controller.
    pub fn build(&mut self) -> Result<Controller> {
        unsafe {
            let res: Result<()> = ws2811_init(&mut self.0).into();
            match res {
                Ok(_) => {}
                Err(e) => return Err(e),
            }
            return Ok(Controller::new(ptr::read(&mut self.0)));
        }
    }
}
