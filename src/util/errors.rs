#[derive(Debug)]
pub enum WS281xError {
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
    WS281X_ERROR_SPI_TRANSFER
}

impl From<ws2811_return_t> for WS281xError {
    fn from(value: ws2811_return_t) {
        match value {
            bindings::ws2811_return_t::WS2811_ERROR_GENERIC => WS281xError::WS281X_ERROR_GENERIC,
            bindings::ws2811_return_t::WS2811_ERROR_OUT_OF_MEMORY => WS281xError::WS281X_ERROR_OUT_OF_MEMORY,
            bindings::ws2811_return_t::WS2811_ERROR_HW_NOT_SUPPORTED => WS281xError::WS281X_ERROR_HW_NOT_SUPPORTED,
            bindings::ws2811_return_t::WS2811_ERROR_MEM_LOCK => WS281xError::WS281X_ERROR_MEM_LOCK,
            bindings::ws2811_return_t::WS2811_ERROR_MMAP => WS281xError::WS281X_ERROR_MMAP,
            bindings::ws2811_return_t::WS2811_ERROR_MAP_REGISTERS => WS281xError::WS281X_ERROR_MAP_REGISTERS,
            bindings::ws2811_return_t::WS2811_ERROR_GPIO_INIT => WS281xError::WS281X_ERROR_GPIO_INIT,
            bindings::ws2811_return_t::WS2811_ERROR_PWM_SETUP => WS281xError::WS281X_ERROR_PWM_SETUP,
            bindings::ws2811_return_t::WS2811_ERROR_MAILBOX_DEVICE => WS281xError::WS281X_ERROR_MAILBOX_DEVICE,
            bindings::ws2811_return_t::WS2811_ERROR_DMA => WS281xError::WS281X_ERROR_DMA,
            bindings::ws2811_return_t::WS2811_ERROR_ILLEGAL_GPIO => WS281xError::WS281X_ERROR_ILLEGAL_GPIO,
            bindings::ws2811_return_t::WS2811_ERROR_PCM_SETUP => WS281xError::WS281X_ERROR_PCM_SETUP,
            bindings::ws2811_return_t::WS2811_ERROR_SPI_SETUP => WS281xError::WS281X_ERROR_SPI_SETUP,
            bindings::ws2811_return_t::WS2811_ERROR_SPI_TRANSFER => WS281xError::WS281X_ERROR_SPI_TRANSFER
        }
    }
}

impl Into<ws2811_return_t> for WS281xError {
    fn into(self){
        match self {
            WS281xError::WS281X_ERROR_GENERIC => bindings::ws2811_return_t::WS2811_ERROR_GENERIC,
            WS281xError::WS281X_ERROR_OUT_OF_MEMORY => bindings::ws2811_return_t::WS2811_ERROR_OUT_OF_MEMORY,
            WS281xError::WS281X_ERROR_HW_NOT_SUPPORTED => bindings::ws2811_return_t::WS2811_ERROR_HW_NOT_SUPPORTED,
            WS281xError::WS281X_ERROR_MEM_LOCK => bindings::ws2811_return_t::WS2811_ERROR_MEM_LOCK,
            WS281xError::WS281X_ERROR_MMAP => bindings::ws2811_return_t::WS2811_ERROR_MMAP,
            WS281xError::WS281X_ERROR_MAP_REGISTERS => bindings::ws2811_return_t::WS2811_ERROR_MAP_REGISTERS,
            WS281xError::WS281X_ERROR_GPIO_INIT => bindings::ws2811_return_t::WS2811_ERROR_GPIO_INIT,
            WS281xError::WS281X_ERROR_PWM_SETUP => bindings::ws2811_return_t::WS2811_ERROR_PWM_SETUP,
            WS281xError::WS281X_ERROR_MAILBOX_DEVICE => bindings::ws2811_return_t::WS2811_ERROR_MAILBOX_DEVICE,
            WS281xError::WS281X_ERROR_DMA => bindings::ws2811_return_t::WS2811_ERROR_DMA,
            WS281xError::WS281X_ERROR_ILLEGAL_GPIO => bindings::ws2811_return_t::WS2811_ERROR_ILLEGAL_GPIO,
            WS281xError::WS281X_ERROR_PCM_SETUP => bindings::ws2811_return_t::WS2811_ERROR_PCM_SETUP,
            WS281xError::WS281X_ERROR_SPI_SETUP => bindings::ws2811_return_t::WS2811_ERROR_SPI_SETUP,
            WS281xError::WS281X_ERROR_SPI_TRANSFER => bindings::ws2811_return_t::WS2811_ERROR_SPI_TRANSFER

        }
    }
}

impl Error for WS281xError {
    fn description(&self) -> &str {
        let output: str = match self {
            WS281X_ERROR_GENERIC => "Something went wrong with the WS281x driver (WS2811_ERROR_GENERIC)",
            WS281X_ERROR_OUT_OF_MEMORY => "Out of memory (WS2811_ERROR_OUT_OF_MEMORY)",
            WS281X_ERROR_HW_NOT_SUPPORTED => "Hardware not supported (WS2811_ERROR_HW_NOT_SUPPORTED",
            WS281X_ERROR_MEM_LOCK => "Memory lock (WS2811_ERROR_MEM_LOCK)",
            WS281X_ERROR_MMAP => "Memory map error (WS2811_ERROR_MMAP)",
            WS281X_ERROR_MAP_REGISTERS => "Register map error (WS2811_ERROR_MAP_REGISTERS)",
            WS281X_ERROR_GPIO_INIT => "Failed to initialize GPIO (WS2811_ERROR_GPIO_INIT)",
            WS281X_ERROR_PWM_SETUP => "Failed to set up PWM (WS2811_ERROR_PWM_SETUP)",
            WS281X_ERROR_MAILBOX_DEVICE => "Mailbox device error (WS2811_ERROR_MAILBOX_DEVICE)",
            WS281X_ERROR_DMA => "DMA error (WS2811_ERROR_DMA)",
            WS281X_ERROR_ILLEGAL_GPIO => "Illegal GPIO (WS2811_ERROR_ILLEGAL_GPIO)",
            WS281X_ERROR_PCM_SETUP => "Failed to set up PCM (WS2811_ERROR_PCM_SETUP)",
            WS281X_ERROR_SPI_SETUP => "Failed to set up SPI (WS2811_ERROR_SPI_SETUP)",
            WS281X_ERROR_SPI_TRANSFER => "SPI transfer failed (WS2811_ERROR_SPI_TRANSFER)"
        }
    }
}

impl fmt::Display for WS281xError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let output: str = self.description();
        write!(f, output);
    }
}

/*
WS2811_ERROR_GENERIC = -1,
WS2811_ERROR_OUT_OF_MEMORY = -2,
WS2811_ERROR_HW_NOT_SUPPORTED = -3,
WS2811_ERROR_MEM_LOCK = -4,
WS2811_ERROR_MMAP = -5,
WS2811_ERROR_MAP_REGISTERS = -6,
WS2811_ERROR_GPIO_INIT = -7,
WS2811_ERROR_PWM_SETUP = -8,
WS2811_ERROR_MAILBOX_DEVICE = -9,
WS2811_ERROR_DMA = -10,
WS2811_ERROR_ILLEGAL_GPIO = -11,
WS2811_ERROR_PCM_SETUP = -12,
WS2811_ERROR_SPI_SETUP = -13,
WS2811_ERROR_SPI_TRANSFER = -14,
*/