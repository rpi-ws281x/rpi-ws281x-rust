#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use bindings::{
    SK6812_STRIP_RGBW,
    SK6812_STRIP_RBGW,
    SK6812_STRIP_GRBW,
    SK6812_STRIP_GBRW,
    SK6812_STRIP_BRGW,
    SK6812_STRIP_BGRW,
    SK6812_SHIFT_WMASK,
    WS2811_STRIP_RGB,
    WS2811_STRIP_RBG,
    WS2811_STRIP_GRB,
    WS2811_STRIP_GBR,
    WS2811_STRIP_BRG,
    WS2811_STRIP_BGR,
    WS2812_STRIP,
    SK6812_STRIP,
    SK6812W_STRIP,
};

pub const SK6812_STRIP_RGBW_I32: i32 = SK6812_STRIP_RGBW as i32;
pub const SK6812_STRIP_RBGW_I32: i32 = SK6812_STRIP_RBGW as i32;
pub const SK6812_STRIP_GRBW_I32: i32 = SK6812_STRIP_GRBW as i32;
pub const SK6812_STRIP_GBRW_I32: i32 = SK6812_STRIP_GBRW as i32;
pub const SK6812_STRIP_BRGW_I32: i32 = SK6812_STRIP_BRGW as i32;
pub const SK6812_STRIP_BGRW_I32: i32 = SK6812_STRIP_BGRW as i32;
pub const SK6812_SHIFT_WMASK_I32: i32 = SK6812_SHIFT_WMASK as i32;
pub const WS2811_STRIP_RGB_I32: i32 = WS2811_STRIP_RGB as i32;
pub const WS2811_STRIP_RBG_I32: i32 = WS2811_STRIP_RBG as i32;
pub const WS2811_STRIP_GRB_I32: i32 = WS2811_STRIP_GRB as i32;
pub const WS2811_STRIP_GBR_I32: i32 = WS2811_STRIP_GBR as i32;
pub const WS2811_STRIP_BRG_I32: i32 = WS2811_STRIP_BRG as i32;
pub const WS2811_STRIP_BGR_I32: i32 = WS2811_STRIP_BGR as i32;
pub const WS2812_STRIP_I32: i32 = WS2812_STRIP as i32;
pub const SK6812_STRIP_I32: i32 = SK6812_STRIP as i32;
pub const SK6812W_STRIP_I32: i32 = SK6812W_STRIP as i32;

#[derive(Debug, Copy, Clone)]
pub enum Strip {
    SK6812_STRIP_RGBW,
    SK6812_STRIP_RBGW,
    SK6812_STRIP_GRBW,
    SK6812_STRIP_GBRW,
    SK6812_STRIP_BRGW,
    SK6812_STRIP_BGRW,
    SK6812_SHIFT_WMASK,
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

impl From<i32> for Strip {
    fn from(value: i32) -> Strip {
        match value {
            a if a == SK6812_STRIP_RGBW_I32 => Strip::SK6812_STRIP_RGBW,
            a if a == SK6812_STRIP_RBGW_I32 => Strip::SK6812_STRIP_RBGW,
            a if a == SK6812_STRIP_GRBW_I32 => Strip::SK6812_STRIP_GRBW,
            a if a == SK6812_STRIP_GBRW_I32 => Strip::SK6812_STRIP_GBRW,
            a if a == SK6812_STRIP_BRGW_I32 => Strip::SK6812_STRIP_BRGW,
            a if a == SK6812_STRIP_BGRW_I32 => Strip::SK6812_STRIP_BGRW,
            a if a == SK6812_SHIFT_WMASK_I32 => Strip::SK6812_SHIFT_WMASK,
            a if a == WS2811_STRIP_RGB_I32 => Strip::WS2811_STRIP_RGB,
            a if a == WS2811_STRIP_RBG_I32 => Strip::WS2811_STRIP_RBG,
            a if a == WS2811_STRIP_GRB_I32 => Strip::WS2811_STRIP_GRB,
            a if a == WS2811_STRIP_GBR_I32 => Strip::WS2811_STRIP_GBR,
            a if a == WS2811_STRIP_BRG_I32 => Strip::WS2811_STRIP_BRG,
            a if a == WS2811_STRIP_BGR_I32 => Strip::WS2811_STRIP_BGR,
            a if a == WS2812_STRIP_I32 => Strip::WS2812_STRIP,
            a if a == SK6812_STRIP_I32 => Strip::SK6812_STRIP,
            a if a == SK6812W_STRIP_I32 => Strip::SK6812W_STRIP,
            _ => Strip::WS2811_STRIP_RGB
        }
    }
}

impl Into<i32> for Strip {
    fn into(self) -> i32 {
        match self {
            Strip::SK6812_STRIP_RGBW => SK6812_STRIP_RGBW_I32,
            Strip::SK6812_STRIP_RBGW => SK6812_STRIP_RBGW_I32,
            Strip::SK6812_STRIP_GRBW => SK6812_STRIP_GRBW_I32,
            Strip::SK6812_STRIP_GBRW => SK6812_STRIP_GBRW_I32,
            Strip::SK6812_STRIP_BRGW => SK6812_STRIP_BRGW_I32,
            Strip::SK6812_STRIP_BGRW => SK6812_STRIP_BGRW_I32,
            Strip::SK6812_SHIFT_WMASK => SK6812_SHIFT_WMASK_I32,
            Strip::WS2811_STRIP_RGB => WS2811_STRIP_RGB_I32,
            Strip::WS2811_STRIP_RBG => WS2811_STRIP_RBG_I32,
            Strip::WS2811_STRIP_GRB => WS2811_STRIP_GRB_I32,
            Strip::WS2811_STRIP_GBR => WS2811_STRIP_GBR_I32,
            Strip::WS2811_STRIP_BRG => WS2811_STRIP_BRG_I32,
            Strip::WS2811_STRIP_BGR => WS2811_STRIP_BGR_I32,
            Strip::WS2812_STRIP => WS2812_STRIP_I32,
            Strip::SK6812_STRIP => SK6812_STRIP_I32,
            Strip::SK6812W_STRIP => SK6812W_STRIP_I32,
        }
    }
}