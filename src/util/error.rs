use super::super::bindings::ws2811_return_t;
use std::{error, fmt, result};

#[derive(Clone, Copy, Debug)]
pub enum WS2811Error {
    Generic,
    OutOfMemory,
    HwNotSupported,
    MemLock,
    Mmap,
    MapRegisters,
    GpioInit,
    PwmSetup,
    MailboxDevice,
    Dma,
    IllegalGpio,
    PcmSetup,
    SpiSetup,
    SpiTransfer,
}

impl fmt::Display for WS2811Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let message = match *self {
            WS2811Error::Generic => "Generic error",
            WS2811Error::OutOfMemory => "Out of memory",
            WS2811Error::HwNotSupported => "Hardware not supported",
            WS2811Error::MemLock => "Memory lock",
            WS2811Error::Mmap => "mmap error",
            WS2811Error::MapRegisters => "Map registers error",
            WS2811Error::GpioInit => "GPIO initialization error",
            WS2811Error::PwmSetup => "PWM setup error",
            WS2811Error::MailboxDevice => "Mailbox device error",
            WS2811Error::Dma => "DMA error",
            WS2811Error::IllegalGpio => "Illegal GPIO error",
            WS2811Error::PcmSetup => "PCM setup error",
            WS2811Error::SpiSetup => "SPI setup error",
            WS2811Error::SpiTransfer => "SPI transfer error",
        };
        write!(f, "{}", message)
    }
}

impl error::Error for WS2811Error {
    fn description(&self) -> &str {
        match *self {
            WS2811Error::Generic => "Generic error",
            WS2811Error::OutOfMemory => "Out of memory",
            WS2811Error::HwNotSupported => "Hardware not supported",
            WS2811Error::MemLock => "Memory lock",
            WS2811Error::Mmap => "mmap error",
            WS2811Error::MapRegisters => "Map registers error",
            WS2811Error::GpioInit => "GPIO initialization error",
            WS2811Error::PwmSetup => "PWM setup error",
            WS2811Error::MailboxDevice => "Mailbox device error",
            WS2811Error::Dma => "DMA error",
            WS2811Error::IllegalGpio => "Illegal GPIO error",
            WS2811Error::PcmSetup => "PCM setup error",
            WS2811Error::SpiSetup => "SPI setup error",
            WS2811Error::SpiTransfer => "SPI transfer error",
        }
    }
    fn cause(&self) -> Option<&dyn error::Error> {
        return None;
    }
}

pub type Result<T> = result::Result<T, WS2811Error>;

impl From<ws2811_return_t> for Result<()> {
    fn from(val: ws2811_return_t) -> Result<()> {
        match val {
            ws2811_return_t::WS2811_SUCCESS => Ok(()),
            ws2811_return_t::WS2811_ERROR_GENERIC => Err(WS2811Error::Generic),
            ws2811_return_t::WS2811_ERROR_OUT_OF_MEMORY => Err(WS2811Error::OutOfMemory),
            ws2811_return_t::WS2811_ERROR_HW_NOT_SUPPORTED => Err(WS2811Error::HwNotSupported),
            ws2811_return_t::WS2811_ERROR_MEM_LOCK => Err(WS2811Error::MemLock),
            ws2811_return_t::WS2811_ERROR_MMAP => Err(WS2811Error::Mmap),
            ws2811_return_t::WS2811_ERROR_MAP_REGISTERS => Err(WS2811Error::MapRegisters),
            ws2811_return_t::WS2811_ERROR_GPIO_INIT => Err(WS2811Error::GpioInit),
            ws2811_return_t::WS2811_ERROR_PWM_SETUP => Err(WS2811Error::PwmSetup),
            ws2811_return_t::WS2811_ERROR_MAILBOX_DEVICE => Err(WS2811Error::MailboxDevice),
            ws2811_return_t::WS2811_ERROR_DMA => Err(WS2811Error::Dma),
            ws2811_return_t::WS2811_ERROR_ILLEGAL_GPIO => Err(WS2811Error::IllegalGpio),
            ws2811_return_t::WS2811_ERROR_PCM_SETUP => Err(WS2811Error::PcmSetup),
            ws2811_return_t::WS2811_ERROR_SPI_SETUP => Err(WS2811Error::SpiSetup),
            ws2811_return_t::WS2811_ERROR_SPI_TRANSFER => Err(WS2811Error::SpiTransfer),
        }
    }
}
