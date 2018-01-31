use bindings::*;
use std::ptr::null_mut;

pub struct Channel<'a> {
    c_struct: ws2811_channel_t,
    gpio_pin: i32,
    led_count: i32,
    invert: i32,
    brightness: u8,
    strip_type: Strip,
    leds:  &'a[u8],
}

impl Channel {
    unsafe fn init(&mut self) {
        self.leds = std::slice::from_raw_parts_mut(self.c_struct.leds, self.led_count*std::mem::size_of(u32));
    }
}