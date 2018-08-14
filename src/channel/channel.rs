use std::mem;

use bindings::ws2811_channel_t;

#[derive(Debug)]
/// A wrapper around the low-level ws2811_channel_t struct.
///
/// Use the channel::builder::Builder to initialize.
pub struct Channel {
    c_struct: ws2811_channel_t,
}

impl  Channel {
    pub fn new(c_struct: ws2811_channel_t ) -> Self {
        Channel {
            c_struct
        }
    }
    /// Create an empty (literally mem-zeroed) channel.
    pub fn empty() -> Self {
        unsafe {
            let c: ws2811_channel_t = mem::zeroed();
            Channel {
                c_struct: c
            }
        }
    }

    pub fn inner(&self) -> ws2811_channel_t {
        self.c_struct
    }
}
