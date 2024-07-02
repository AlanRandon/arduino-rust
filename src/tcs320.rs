use arduino_hal::port;
use atmega_hal::port::{PB2, PB3, PD2, PD3, PD4, PD7};

/// Failed to read from TCS320 color sensor
#[derive(Debug)]
pub struct ReadError;

#[derive(Debug)]
pub struct Reading {
    pub red: u32,
    pub green: u32,
    pub blue: u32,
}

/// TCS230 color sensor
pub struct TCS320 {
    /// Pin 7
    pub oe: port::Pin<port::mode::Output, PD7>,
    /// Pin 10
    pub s0: port::Pin<port::mode::Output, PB2>,
    /// Pin 11
    pub s1: port::Pin<port::mode::Output, PB3>,
    /// Pin 2
    pub s2: port::Pin<port::mode::Output, PD2>,
    /// Pin 3
    pub s3: port::Pin<port::mode::Output, PD3>,
    /// Pin 4
    pub out: port::Pin<port::mode::Input<port::mode::Floating>, PD4>,
}

impl TCS320 {
    fn select_red(&mut self) {
        self.s2.set_low();
        self.s3.set_low();
    }

    fn select_green(&mut self) {
        self.s2.set_high();
        self.s3.set_high();
    }

    fn select_blue(&mut self) {
        self.s2.set_low();
        self.s3.set_high();
    }

    fn select_clear(&mut self) {
        self.s2.set_high();
        self.s3.set_low();
    }

    fn get_reading(&self) -> Result<u32, ReadError> {
        pulse_high_width(&self.out, 20000).ok_or(ReadError)
    }

    pub fn read(&mut self) -> Result<Reading, ReadError> {
        Ok(Reading {
            red: {
                self.select_red();
                self.get_reading()?
            },
            green: {
                self.select_green();
                self.get_reading()?
            },
            blue: {
                self.select_blue();
                self.get_reading()?
            },
        })
    }

    pub fn setup(&mut self) {
        self.s0.set_low();
        self.s1.set_high();
        self.oe.set_low();
    }
}

fn pulse_high_width<PIN: port::PinOps, M: port::mode::InputMode>(
    pin: &port::Pin<port::mode::Input<M>, PIN>,
    mut max_iterations: u32,
) -> Option<u32> {
    // wait for previous pulse to end
    while pin.is_high() {
        max_iterations -= 1;
        if max_iterations == 0 {
            return None;
        }
    }

    // wait for the pulse to start
    while pin.is_low() {
        max_iterations -= 1;
        if max_iterations == 0 {
            return None;
        }
    }

    let mut width = 0;

    // wait for the pulse to stop
    while pin.is_high() {
        width += 1;
        if width == max_iterations {
            return None;
        }
    }

    Some(width)
}
