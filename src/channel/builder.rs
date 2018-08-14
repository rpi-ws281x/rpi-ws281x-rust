use std::mem;

use bindings::ws2811_channel_t;

use channel::Channel;
use util::strip::Strip;

/// Builds a Channel.
///
/// ```
/// let channel = channel::builder::Builder::default()
///     .gpio_pin(18)
///     .led_count(100)
///     .strip_type(Strip::WS2811_STRIP_RGB)
///     .brightness(128)
///     .build();
/// ```
pub struct Builder {
    /// the GPIO pin on the Pi to send data from
    /// (Note: not the physical hardware pin number, not
    /// the WiringPi number, but the GPIO number).
    /// defaults to 18 (which has PWM)
    gpio_num: i32,
    /// The number of LEDs in the strand (or in series)
    led_count: i32,
    /// Invert the output signal; defaults to false.
    invert: bool,
    /// Set the type of LED Strip; defaults to Strip::WS2811_STRIP_RGB
    strip_type: Strip,
    /// the initial brightness level of the LEDs; defaults to 255
    brightness: u8,
}

impl Default for Builder {
    fn default() -> Self {
        Builder {
            gpio_num: 18,
            led_count: 0,
            invert: false,
            strip_type: Strip::WS2811_STRIP_RGB,
            brightness: 255,
        }
    }
}

impl Builder {
    /// set the gpio pin to use; defaults to 18 (must have PWM support)
    pub fn gpio_num(&mut self, val: i32) -> &mut Self {
        self.gpio_num = val;
        self
    }

    /// set the number of LEDs in the strand
    pub fn led_count(&mut self, val: i32) -> &mut Self {
        self.led_count = val;
        self
    }

    /// invert the signal? defaults to false
    pub fn invert(&mut self, val: bool) -> &mut Self {
        self.invert = val;
        self
    }

    /// sets the strip type; defaults to Strip::WS2811_STRIP_RGB
    pub fn strip_type(&mut self, val: Strip) -> &mut Self {
        self.strip_type = val;
        self
    }

    /// set the initial brightness of the LEDs; defaults to 255
    pub fn brightness(&mut self, val: u8) -> &mut Self {
        self.brightness = val;
        self
    }

    /// build the configured Channel
    pub fn build(&self) -> Channel {
        let invert: i32 = self.invert.into();
        unsafe {
            let mut c_struct: ws2811_channel_t = mem::zeroed();
            c_struct.gpionum = self.gpio_num;
            c_struct.invert = invert;
            c_struct.count = self.led_count;
            c_struct.strip_type = self.strip_type.into();
            c_struct.brightness = self.brightness;
            Channel::new(c_struct)
        }
    }
}
