use std::os::raw::c_uint;

use super::super::bindings::{
    SK6812W_STRIP, SK6812_STRIP, SK6812_STRIP_BGRW, SK6812_STRIP_BRGW, SK6812_STRIP_GBRW,
    SK6812_STRIP_GRBW, SK6812_STRIP_RBGW, SK6812_STRIP_RGBW, WS2811_STRIP_BGR, WS2811_STRIP_BRG,
    WS2811_STRIP_GBR, WS2811_STRIP_GRB, WS2811_STRIP_RBG, WS2811_STRIP_RGB, WS2812_STRIP,
};

#[derive(Clone, Copy, Debug)]
pub enum StripType {
    Sk6812Rgbw,
    Sk6812Rbgw,
    Sk6812Gbrw,
    Sk6812Grbw,
    Sk6812Brgw,
    Sk6812Bgrw,
    Ws2811Rgb,
    Ws2811Rbg,
    Ws2811Grb,
    Ws2811Gbr,
    Ws2811Brg,
    Ws2811Bgr,
    Ws2812,
    Sk6812,
    Sk6812W,
}

impl Into<c_uint> for StripType {
    fn into(self) -> c_uint {
        match self {
            StripType::Sk6812Rgbw => SK6812_STRIP_RGBW,
            StripType::Sk6812Rbgw => SK6812_STRIP_RBGW,
            StripType::Sk6812Gbrw => SK6812_STRIP_GBRW,
            StripType::Sk6812Grbw => SK6812_STRIP_GRBW,
            StripType::Sk6812Brgw => SK6812_STRIP_BRGW,
            StripType::Sk6812Bgrw => SK6812_STRIP_BGRW,
            StripType::Ws2811Rgb => WS2811_STRIP_RGB,
            StripType::Ws2811Rbg => WS2811_STRIP_RBG,
            StripType::Ws2811Grb => WS2811_STRIP_GRB,
            StripType::Ws2811Gbr => WS2811_STRIP_GBR,
            StripType::Ws2811Brg => WS2811_STRIP_BRG,
            StripType::Ws2811Bgr => WS2811_STRIP_BGR,
            StripType::Ws2812 => WS2812_STRIP,
            StripType::Sk6812 => SK6812_STRIP,
            StripType::Sk6812W => SK6812W_STRIP,
        }
    }
}
