#![no_std]
#![no_main]

extern crate cortex_m;
extern crate cortex_m_semihosting;
#[cfg(feature = "feather_m0")]
extern crate feather_m0 as hal;
#[cfg(feature = "feather_m4")]
extern crate feather_m4 as hal;
#[cfg(feature = "grand_central_m4")]
extern crate grand_central_m4 as hal;
#[cfg(not(feature = "use_semihosting"))]
extern crate panic_halt;
#[cfg(feature = "use_semihosting")]
extern crate panic_semihosting;
#[cfg(feature = "wio_terminal")]
extern crate wio_terminal as hal;

use hal::clock::GenericClockController;
use hal::delay::Delay;
use hal::entry;
use hal::pac::{CorePeripherals, Peripherals};
use hal::prelude::*;

#[entry]
fn main() -> ! {
    let mut peripherals = Peripherals::take().unwrap();
    let core = CorePeripherals::take().unwrap();

    #[cfg(not(feature = "feather_m0"))]
    let mut clocks = GenericClockController::with_external_32kosc(
        peripherals.GCLK,
        &mut peripherals.MCLK,
        &mut peripherals.OSC32KCTRL,
        &mut peripherals.OSCCTRL,
        &mut peripherals.NVMCTRL,
    );

    #[cfg(feature = "feather_m0")]
    let mut clocks = GenericClockController::with_external_32kosc(
        peripherals.GCLK,
        &mut peripherals.PM,
        &mut peripherals.SYSCTRL,
        &mut peripherals.NVMCTRL,
    );

    let mut pins = hal::Pins::new(peripherals.PORT);

    #[cfg(not(feature = "grand_central_m4"))]
    let mut red_led = pins.d13.into_open_drain_output(&mut pins.port);

    #[cfg(feature = "grand_central_m4")]
    let mut red_led = pins.red_led.into_open_drain_output(&mut pins.port);

    let mut delay = Delay::new(core.SYST, &mut clocks);
    loop {
        delay.delay_ms(2000u16);
        red_led.set_high().unwrap();
        delay.delay_ms(2000u16);
        red_led.set_low().unwrap();
    }
}
