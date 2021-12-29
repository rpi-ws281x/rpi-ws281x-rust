use rs_ws281x::ControllerBuilder;

fn main() {
    // Construct a single channel controller. Note that the
    // Controller is initialized by default and is cleaned up on drop

    let mut controller = ControllerBuilder::new()
        .freq(800_000)
        .dma(10)
        .channel(
            0, // Channel Index
            ChannelBuilder::new()
                .pin(10) // GPIO 10 = SPI0 MOSI
                .count(64) // Number of LEDs
                .strip_type(StripType::Ws2812)
                .brightness(20) // default: 255
                .build(),
        )
        .build()
        .unwrap();

    let leds = controller.leds_mut(0);

    for led in leds {
        *led = [0, 0, 255, 0];
    }

    controller.render().unwrap();
}
