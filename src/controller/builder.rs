use std::mem;

use std::os::raw::{c_int};

use super::super::bindings::{ws2811_init, ws2811_t, ws2811_channel_t};
use super::super::util::Result;

use super::controller::Controller;

/// A struct to assist in the process of initializing
/// a Controller instance.  One is pretty much required to use
/// this to construct a Controller, unless one happens to have
/// an instance of ws2811_t sitting around.
///
/// ```
/// // Construct a single channel controller. Note that the
/// // Controller is initialized by default and is cleaned up on drop
/// let controller = ControllerBuilder::new()
///     // default
///     .freq(800_000)
///     // default
///     .dma(10)
///     .channel(
///         ChannelBuilder::new()
///             .pin(18)
///             .count(10)
///             .strip_type(StripType::Ws2811Rgb)
///             .brightness(255)
///             .build()
///      )
///     .build();
///
/// // get the strand of LEDs on channel 1
/// let leds = controller.leds_mut(0);
/// // set the first LED to white (with the configured
/// // strip above, this is BGRW)
/// leds[0] = [255, 255, 255, 0]
///
/// // render it to the strand
/// controller.render();
#[derive(Debug)]
pub struct ControllerBuilder(pub ws2811_t);

impl ControllerBuilder {
    /// Create a new ControllerBuilder
    pub fn new() -> Self {
        unsafe {
            let mut cb = ControllerBuilder(mem::zeroed());
            // Set some good/common defaults.
            cb.freq(800_000);
            // DMA 5 is used for the file system on the Pi 3,
            // setting this value to 5 can cause corruptions
            cb.dma(10);
            cb
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
    pub fn channel(&mut self, index: usize, channel: ws2811_channel_t) -> &mut Self {
        self.0.channel[index] = channel;
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
        // all of the pointers will be initialized as a part of
        // ws2811_init(), so clone here so that this builder
        // can have .build() called multiple times.
        let mut c_struct = self.0.clone();
        unsafe {
            let res: Result<()> = ws2811_init(&mut c_struct).into();
            match res {
                Ok(_) => {}
                Err(e) => return Err(e),
            }
            return Ok(Controller::new(c_struct));
        }
    }
}
