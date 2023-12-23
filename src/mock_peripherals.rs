use mockall::*;
use embedded_hal::digital::v2::OutputPin;
use embedded_hal::digital::v2::InputPin;
mock! {
    pub Output {} // Name of the mock struct, less the "Mock" prefix
    impl OutputPin for Output { // specification of the trait to mock
        type Error = i32;
        fn set_high(&mut self) -> Result<(), <Self as OutputPin>::Error> { todo!() }
        fn set_low(&mut self) -> Result<(), <Self as OutputPin>::Error> { todo!() }
    }
}
mock! {
    pub Input {} // Name of the mock struct, less the "Mock" prefix
    impl InputPin for Input { // specification of the trait to mock
        type Error = i32;
        fn is_high(&self) -> Result<bool, <Self as InputPin>::Error> { todo!() }
        fn is_low(&self) -> Result<bool, <Self as InputPin>::Error> { todo!() }
    }
}
