#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

pub const RPI_PWM_CHANNELS: usize = 2;
pub const WS2811_TARGET_FREQ: u32 = 800000; // Can go as low as 400000

// 4 color R, G, B and W ordering
pub const SK6812_STRIP_RGBW: u32 = 0x18100800;
pub const SK6812_STRIP_RBGW: u32 = 0x18100008;
pub const SK6812_STRIP_GRBW: u32 = 0x18081000;
pub const SK6812_STRIP_GBRW: u32 = 0x18080010;
pub const SK6812_STRIP_BRGW: u32 = 0x18001008;
pub const SK6812_STRIP_BGRW: u32 = 0x18000810;
pub const SK6812_SHIFT_WMASK: u32 = 0xf0000000;

// 3 color R, G and B ordering
pub const WS2811_STRIP_RGB: u32 = 0x00100800;
pub const WS2811_STRIP_RBG: u32 = 0x00100008;
pub const WS2811_STRIP_GRB: u32 = 0x00081000;
pub const WS2811_STRIP_GBR: u32 = 0x00080010;
pub const WS2811_STRIP_BRG: u32 = 0x00001008;
pub const WS2811_STRIP_BGR: u32 = 0x00000810;

// predefined fixed LED types
pub const WS2812_STRIP: u32 = WS2811_STRIP_GRB;
pub const SK6812_STRIP: u32 = WS2811_STRIP_GRB;
pub const SK6812W_STRIP: u32 = SK6812_STRIP_GRBW;

#[derive(Clone, Debug)]
#[repr(C)]
pub struct ws2811_channel_t {
    pub gpionum: i32,     //< GPIO Pin with PWM alternate function, 0 if unused
    pub invert: i32,      //< Invert output signal
    pub count: i32,       //< Number of LEDs, 0 if channel is unused
    pub strip_type: i32,  //< Strip color layout -- one of WS2811_STRIP_xxx constants
    pub leds: *mut u32,   //< LED buffers, allocated by driver based on count
    pub brightness: u8,   //< Brightness value between 0 and 255
    pub wshift: u8,       //< White shift value
    pub rshift: u8,       //< Red shift value
    pub gshift: u8,       //< Green shift value
    pub bshift: u8,       //< Blue shift value
    pub gamma: *const u8, //< Gamma correction table
}

#[repr(i32)]
pub enum ws2811_return_t {
    WS2811_SUCCESS = 0,
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
}

#[repr(C)]
#[derive(Debug, Clone)]
pub struct ws2811_t {
    ///< time in µs before the next render can run
    pub render_wait_time: u64,
    ///< Private data for driver use
    pub device: *mut ws2811_device,
    ///< RPI Hardware Information
    pub rpi_hw: *const rpi_hw_t,
    ///< Required output frequency
    pub freq: u32,
    ///< DMA number _not_ already in use
    pub dmanum: i32,
    pub channel: [ws2811_channel_t; RPI_PWM_CHANNELS],
}

#[repr(C)]
#[derive(Debug, Clone)]
pub struct videocore_mbox_t {
    handle: i32,        /* From mbox_open() */
    mem_ref: u32,       /* From mem_alloc() */
    bus_addr: u32,      /* From mem_lock() */
    size: u32,          /* Size of allocation */
    virt_addr: *mut u8, /* From mapmem() */
}

#[repr(C)]
#[derive(Debug, Clone)]
pub struct ws2811_device {
    driver_mode: i32,
    pxl_raw: *mut u8,
    dma: *mut u8,
    pwm: *mut u8,
    pcm: *mut u8,
    spi_fd: i32,
    dma_cb: *mut u8,
    dma_cb_addr: u32,
    gpio: *mut u8,
    cm_clk: *mut u8,
    mbox: videocore_mbox_t,
    max_count: i32,
}

#[repr(C)]
#[derive(Debug, Clone)]
pub struct rpi_hw_t {
    r#type: u32,
    hwver: u32,
    periph_base: u32,
    videocore_base: u32,
    desc: *const u8,
}

extern "C" {
    ///< Initialize buffers/hardware
    pub fn ws2811_init(ws2811: *mut ws2811_t) -> ws2811_return_t;
    ///< Tear it all down
    pub fn ws2811_fini(ws2811: *mut ws2811_t);
    ///< Send LEDs off to hardware
    pub fn ws2811_render(ws2811: *mut ws2811_t) -> ws2811_return_t;
    ///< Wait for DMA completion
    pub fn ws2811_wait(ws2811: *mut ws2811_t) -> ws2811_return_t;
}
