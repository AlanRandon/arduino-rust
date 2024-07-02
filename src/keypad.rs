use arduino_hal::port;

pub struct Keypad {
    row_pins: [port::Pin<port::mode::Output>; 4],
    column_pins: [port::Pin<port::mode::Input<port::mode::Floating>>; 4],
}

pub enum Row {
    One,
    Two,
    Three,
    Four,
}

impl Keypad {
    pub fn check_row(&mut self, row: Row) -> [bool; 4] {
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

    pub fn new(pins: arduino_hal::Pins) -> Self {
        Self {
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
        }
    }
}
