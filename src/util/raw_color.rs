pub type RawColor = [u8; 4];

#[cfg(test)]
mod tests {
    use std::mem;
    use super::super::super::bindings::{ws2811_led_t};
    use super::RawColor;
    #[test]
    fn test_size_compatible() {
        assert_eq!(mem::size_of::<ws2811_led_t>(), mem::size_of::<RawColor>());
    }
}