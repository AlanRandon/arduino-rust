#![no_std]
#![no_main]

use arduino_hal::{port, prelude::*};
use panic_halt as _;

struct Keypad {
    row_pins: [port::Pin<port::mode::Output>; 4],
    column_pins: [port::Pin<port::mode::Input<port::mode::Floating>>; 4],
}

enum Row {
    One,
    Two,
    Three,
    Four,
}

impl Keypad {
    fn check_row(&mut self, row: Row) -> [bool; 4] {
        let [r1, r2, r3, r4] = &mut self.row_pins;
        let (low, high) = match row {
            Row::One => (r1, [r2, r3, r4]),
            Row::Two => (r2, [r1, r3, r4]),
            Row::Three => (r3, [r1, r2, r4]),
            Row::Four => (r4, [r1, r2, r3]),
        };

        for high in high {
            high.set_low();
        }

        low.set_high();

        [0, 1, 2, 3].map(|i| self.column_pins[i].is_high())
    }
}

#[arduino_hal::entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);
    let mut serial = arduino_hal::default_serial!(dp, pins, 57600);

    let mut led = pins.d13.into_output();
    ufmt::uwriteln!(&mut serial, "Hello world!\r").unwrap_infallible();

    let mut keypad = Keypad {
        row_pins: [
            pins.d9.downgrade().into_output(),
            pins.d8.downgrade().into_output(),
            pins.d7.downgrade().into_output(),
            pins.d6.downgrade().into_output(),
        ],
        column_pins: [
            pins.d5.downgrade(),
            pins.d4.downgrade(),
            pins.d3.downgrade(),
            pins.d2.downgrade(),
        ],
    };

    loop {
        arduino_hal::delay_ms(100);
        match keypad.check_row(Row::One) {
            [true, false, false, false] => led.set_high(),
            [false, true, false, false] => led.set_low(),
            _ => {}
        }
    }
}
