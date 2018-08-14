#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use std::fmt;
use std::error::Error;

use bindings::ws2811_return_t;

#[derive(Debug)]
pub enum WS281xResult {
    WS281X_ERROR_GENERIC,
    WS281X_ERROR_OUT_OF_MEMORY,
    WS281X_ERROR_HW_NOT_SUPPORTED,
    WS281X_ERROR_MEM_LOCK,
    WS281X_ERROR_MMAP,
    WS281X_ERROR_MAP_REGISTERS,
    WS281X_ERROR_GPIO_INIT,
    WS281X_ERROR_PWM_SETUP,
    WS281X_ERROR_MAILBOX_DEVICE,
    WS281X_ERROR_DMA,
    WS281X_ERROR_ILLEGAL_GPIO,
    WS281X_ERROR_PCM_SETUP,
    WS281X_ERROR_SPI_SETUP,
    WS281X_ERROR_SPI_TRANSFER,
    WS2811_SUCCESS,
}

impl From<ws2811_return_t> for WS281xResult {
    fn from(value: ws2811_return_t) -> WS281xResult {
        match value {
            ws2811_return_t::WS2811_ERROR_GENERIC => WS281xResult::WS281X_ERROR_GENERIC,
            ws2811_return_t::WS2811_ERROR_OUT_OF_MEMORY => WS281xResult::WS281X_ERROR_OUT_OF_MEMORY,
            ws2811_return_t::WS2811_ERROR_HW_NOT_SUPPORTED => WS281xResult::WS281X_ERROR_HW_NOT_SUPPORTED,
            ws2811_return_t::WS2811_ERROR_MEM_LOCK => WS281xResult::WS281X_ERROR_MEM_LOCK,
            ws2811_return_t::WS2811_ERROR_MMAP => WS281xResult::WS281X_ERROR_MMAP,
            ws2811_return_t::WS2811_ERROR_MAP_REGISTERS => WS281xResult::WS281X_ERROR_MAP_REGISTERS,
            ws2811_return_t::WS2811_ERROR_GPIO_INIT => WS281xResult::WS281X_ERROR_GPIO_INIT,
            ws2811_return_t::WS2811_ERROR_PWM_SETUP => WS281xResult::WS281X_ERROR_PWM_SETUP,
            ws2811_return_t::WS2811_ERROR_MAILBOX_DEVICE => WS281xResult::WS281X_ERROR_MAILBOX_DEVICE,
            ws2811_return_t::WS2811_ERROR_DMA => WS281xResult::WS281X_ERROR_DMA,
            ws2811_return_t::WS2811_ERROR_ILLEGAL_GPIO => WS281xResult::WS281X_ERROR_ILLEGAL_GPIO,
            ws2811_return_t::WS2811_ERROR_PCM_SETUP => WS281xResult::WS281X_ERROR_PCM_SETUP,
            ws2811_return_t::WS2811_ERROR_SPI_SETUP => WS281xResult::WS281X_ERROR_SPI_SETUP,
            ws2811_return_t::WS2811_ERROR_SPI_TRANSFER => WS281xResult::WS281X_ERROR_SPI_TRANSFER,
            ws2811_return_t::WS2811_SUCCESS => WS281xResult::WS2811_SUCCESS,
        }
    }
}

impl Into<ws2811_return_t> for WS281xResult {
    fn into(self) -> ws2811_return_t {
        match self {
            WS281xResult::WS281X_ERROR_GENERIC => ws2811_return_t::WS2811_ERROR_GENERIC,
            WS281xResult::WS281X_ERROR_OUT_OF_MEMORY => ws2811_return_t::WS2811_ERROR_OUT_OF_MEMORY,
            WS281xResult::WS281X_ERROR_HW_NOT_SUPPORTED => ws2811_return_t::WS2811_ERROR_HW_NOT_SUPPORTED,
            WS281xResult::WS281X_ERROR_MEM_LOCK => ws2811_return_t::WS2811_ERROR_MEM_LOCK,
            WS281xResult::WS281X_ERROR_MMAP => ws2811_return_t::WS2811_ERROR_MMAP,
            WS281xResult::WS281X_ERROR_MAP_REGISTERS => ws2811_return_t::WS2811_ERROR_MAP_REGISTERS,
            WS281xResult::WS281X_ERROR_GPIO_INIT => ws2811_return_t::WS2811_ERROR_GPIO_INIT,
            WS281xResult::WS281X_ERROR_PWM_SETUP => ws2811_return_t::WS2811_ERROR_PWM_SETUP,
            WS281xResult::WS281X_ERROR_MAILBOX_DEVICE => ws2811_return_t::WS2811_ERROR_MAILBOX_DEVICE,
            WS281xResult::WS281X_ERROR_DMA => ws2811_return_t::WS2811_ERROR_DMA,
            WS281xResult::WS281X_ERROR_ILLEGAL_GPIO => ws2811_return_t::WS2811_ERROR_ILLEGAL_GPIO,
            WS281xResult::WS281X_ERROR_PCM_SETUP => ws2811_return_t::WS2811_ERROR_PCM_SETUP,
            WS281xResult::WS281X_ERROR_SPI_SETUP => ws2811_return_t::WS2811_ERROR_SPI_SETUP,
            WS281xResult::WS281X_ERROR_SPI_TRANSFER => ws2811_return_t::WS2811_ERROR_SPI_TRANSFER,
            WS281xResult::WS2811_SUCCESS => ws2811_return_t::WS2811_SUCCESS,
        }
    }
}

impl Error for WS281xResult {
    fn description(&self) -> &str {
        match self {
            WS281xResult::WS281X_ERROR_GENERIC => "Something went wrong with the WS281x driver (WS2811_ERROR_GENERIC)",
            WS281xResult::WS281X_ERROR_OUT_OF_MEMORY => "Out of memory (WS2811_ERROR_OUT_OF_MEMORY)",
            WS281xResult::WS281X_ERROR_HW_NOT_SUPPORTED => "Hardware not supported (WS2811_ERROR_HW_NOT_SUPPORTED",
            WS281xResult::WS281X_ERROR_MEM_LOCK => "Memory lock (WS2811_ERROR_MEM_LOCK)",
            WS281xResult::WS281X_ERROR_MMAP => "Memory map error (WS2811_ERROR_MMAP)",
            WS281xResult::WS281X_ERROR_MAP_REGISTERS => "Register map error (WS2811_ERROR_MAP_REGISTERS)",
            WS281xResult::WS281X_ERROR_GPIO_INIT => "Failed to initialize GPIO (WS2811_ERROR_GPIO_INIT)",
            WS281xResult::WS281X_ERROR_PWM_SETUP => "Failed to set up PWM (WS2811_ERROR_PWM_SETUP)",
            WS281xResult::WS281X_ERROR_MAILBOX_DEVICE => "Mailbox device error (WS2811_ERROR_MAILBOX_DEVICE)",
            WS281xResult::WS281X_ERROR_DMA => "DMA error (WS2811_ERROR_DMA)",
            WS281xResult::WS281X_ERROR_ILLEGAL_GPIO => "Illegal GPIO (WS2811_ERROR_ILLEGAL_GPIO)",
            WS281xResult::WS281X_ERROR_PCM_SETUP => "Failed to set up PCM (WS2811_ERROR_PCM_SETUP)",
            WS281xResult::WS281X_ERROR_SPI_SETUP => "Failed to set up SPI (WS2811_ERROR_SPI_SETUP)",
            WS281xResult::WS281X_ERROR_SPI_TRANSFER => "SPI transfer failed (WS2811_ERROR_SPI_TRANSFER)",
            // not an error. this should never get reported.
            WS281xResult::WS2811_SUCCESS => "",
        }
    }
}

impl fmt::Display for WS281xResult {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let output = self.description();
        write!(f, "{}", output)
    }
}
