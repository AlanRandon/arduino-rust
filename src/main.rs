#![no_std]
#![no_main]

use arduino_hal::prelude::*;
use panic_halt as _;

mod keypad;
mod tcs320;

#[arduino_hal::entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);
    let mut serial = arduino_hal::default_serial!(dp, pins, 57600);

    let mut led = pins.d13.into_output();
    ufmt::uwriteln!(&mut serial, "Hello world!\r").unwrap_infallible();

    let mut tcs320 = tcs320::TCS320 {
        oe: pins.d7.into_output(),
        s0: pins.d10.into_output(),
        s1: pins.d11.into_output(),
        s2: pins.d2.into_output(),
        s3: pins.d3.into_output(),
        out: pins.d4.into_floating_input(),
    };
    tcs320.setup();

    loop {
        arduino_hal::delay_ms(100);
        match tcs320.read() {
            Ok(tcs320::Reading { red, green, blue }) => {
                ufmt::uwriteln!(&mut serial, "R: {} G: {} B: {}\r", red, green, blue)
                    .unwrap_infallible();
            }
            Err(_) => led.toggle(),
        }
    }
}
