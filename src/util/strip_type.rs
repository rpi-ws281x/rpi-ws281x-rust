use libc::{c_int};
use super::super::bindings::{
    SK6812_STRIP_RGBW,
    SK6812_STRIP_RBGW,
    SK6812_STRIP_GRBW,
    SK6812_STRIP_GBRW,
    SK6812_STRIP_BRGW,
    SK6812_STRIP_BGRW,
    WS2811_STRIP_RGB,
    WS2811_STRIP_RBG,
    WS2811_STRIP_GRB,
    WS2811_STRIP_GBR,
    WS2811_STRIP_BRG,
    WS2811_STRIP_BGR,
    WS2812_STRIP,
    SK6812_STRIP,
    SK6812W_STRIP,
}

#[derive(Debug)]
pub enum StripType {
    SK6812_RGBW,
    SK6812_RBGW,
    SK6812_GRBW,
    SK6812_BRGW,
    SK6812_BGRW,
    WS2811_RGB,
    WS2811_RBG,
    WS2811_GRB,
    WS2811_GBR,
    WS2811_BRG,
    WS2811_BGR,
    WS2812,
    SK6812,
    SK6812W,
}

impl Into<c_int> for StripType {
    fn into(val: StripType) -> c_int {
        match val {
            StripType::SK6812_RGBW => SK6812_STRIP_RGBW,
            StripType::SK6812_RBGW => SK6812_STRIP_RBGW,
            StripType::SK6812_GRBW => SK6812_STRIP_GRBW,
            StripType::SK6812_BRGW => SK6812_STRIP_BRGW,
            StripType::SK6812_BGRW => SK6812_STRIP_BGRW,
            StripType::WS2811_RGB => WS2811_STRIP_RGB,
            StripType::WS2811_RBG => WS2811_STRIP_RBG,
            StripType::WS2811_GRB => WS2811_STRIP_GRB,
            StripType::WS2811_GBR => WS2811_STRIP_GBR,
            StripType::WS2811_BRG => WS2811_STRIP_BRG,
            StripType::WS2811_BGR => WS2811_STRIP_BGR,
            StripType::WS2812 => WS2812,
            StripType::SK6812 => SK6812,
            StripType::SK6812W => SK6812W,
        }
    }
}