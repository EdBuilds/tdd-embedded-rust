#![deny(unsafe_code)]
#![cfg_attr(not(test), no_main)]
#![cfg_attr(not(test), no_std)]

#[cfg(not(test))]
use panic_halt as _;

#[cfg(not(test))]
use cortex_m_rt::entry;

use stm32f4xx_hal::{
    pac,
    prelude::*,
};
use crate::app::App;

#[cfg(test)]
mod mock_peripherals;
mod app;
#[allow(clippy::empty_loop)]
#[cfg_attr(not(test), entry)]
fn main() -> ! {
    let p = pac::Peripherals::take().unwrap();
    let gpioc = p.GPIOC.split();
    let gpioa = p.GPIOA.split();
    let led = gpioa.pa5.into_push_pull_output();
    let button = gpioc.pc13.into_pull_up_input();
    let mut app = App::new(led, button);
    loop {
        app.run()
    }
}
