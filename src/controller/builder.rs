use std::mem;

use bindings::ws2811_t;

use channel::Channel;
use controller::Controller;

const MAX_NUM_CHANNELS: usize = 2;

/// Builds a Controller
///
/// ```
/// // Construct a single channel controller (note: the second
/// // channel is created by default). Note that the Controller
/// // is initialized by default and is cleaned up on drop
/// let controller = controller::Builder::default()
///     // default
///     .freq(800_000)
///     // default
///     .dma(10)
///     .channel(
///         channel::Builder::default()
///             .gpio_num(18)
///             .led_count(10)
///             .strip_type(Strip::WS2811_STRIP_RGB)
///             .brightness(255)
///             .build()
///      )
///     .build();
///
/// // set the 5th pixel to RGB white on the first channel
/// controller.set(0, 4, 0x00ffffff);
/// // render it to the strand
/// controller.render();
/// ```
pub struct Builder {
    /// the frequency of the LED clock
    freq: u32,
    /// a DMA (direct memory access) number not already in use by the pi.
    /// 5 is in use by the file system on Pi 3, so 10 is a good default.
    dma: i32,
    channels: [Channel; 2],
}

impl Default for Builder {
    /// Initializes some common defaults
    fn default() -> Self {
        Builder {
            freq: 800000,
            dma: 10,
            channels: [Channel::empty(), Channel::empty()],
        }
    }
}

impl Builder {
    /// Set the controller frequency (in Hz); defaults to 800KHz
    pub fn freq(&mut self, frequency: u32) -> &mut Self {
        self.freq = frequency;
        self
    }

    /// set the DMA number; defaults to 10
    pub fn dma(&mut self, dma: i32) -> &mut Self {
        self.dma = dma;
        self
    }

    /// set a Channel at the given index (0 or 1).
    pub fn channel(&mut self, channel: Channel, idx: usize) -> &mut Self {
        self.channels[idx] = channel;
        self
    }

    /// Build the final controller object.
    pub fn build(&self) -> Controller {
        unsafe {
            let mut c_struct: ws2811_t = mem::zeroed();
            c_struct.freq = self.freq;
            c_struct.dmanum = self.dma;
            for i in 0..MAX_NUM_CHANNELS {
                c_struct.channel[i] = self.channels[i].inner();
            }
            let mut controller = Controller::new(c_struct);
            if let Err(e) = controller.init() {
                panic!(format!("Error initializing controller: {}", e))
            }
            controller
        }
    }
}
