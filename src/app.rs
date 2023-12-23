use core::fmt::Debug;
use embedded_hal as hal;
pub struct App<OP,IP>
    where
        OP: hal::digital::v2::OutputPin,
        IP: hal::digital::v2::InputPin,
{
    led_pin: OP,
    button_pin: IP,
}
impl<OP,IP> App<OP, IP>
    where
        OP: hal::digital::v2::OutputPin,
        IP: hal::digital::v2::InputPin,
        <IP as hal::digital::v2::InputPin>::Error: Debug,
        <OP as hal::digital::v2::OutputPin>::Error: Debug
{
    pub fn new(led_pin: OP, button_pin: IP) -> App<OP,IP> {
        App{led_pin, button_pin}
    }
    pub fn run(&mut self)
    {
        if self.button_pin.is_high().unwrap() {
            self.led_pin.set_high().unwrap();
        } else {
            self.led_pin.set_low().unwrap();
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::mock_peripherals::{MockOutput, MockInput};
    #[test]
    fn test_button_down() {
        let mut mock_button = MockInput::new();
        let mut mock_led = MockOutput::new();
        let _ = mock_button.expect_is_high().times(1).returning(||Ok(true));
        let _ = mock_led.expect_set_high().times(1).returning(||Ok(()));
        let mut test_app = App::new(mock_led, mock_button);
        test_app.run()
    }
    #[test]
    fn test_button_up() {
        let mut mock_button = MockInput::new();
        let mut mock_led = MockOutput::new();
        let _ = mock_button.expect_is_high().times(1).returning(||Ok(false));
        let _ = mock_led.expect_set_low().times(1).returning(||Ok(()));
        let mut test_app = App::new(mock_led, mock_button);
        test_app.run()
    }
}
