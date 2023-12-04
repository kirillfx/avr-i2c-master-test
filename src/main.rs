#![no_std]
#![no_main]

use arduino_hal::prelude::*;
// use embedded_hal::blocking::i2c::Write;
use arduino_hal::i2c::Error as I2CError;
use panic_halt as _;
use ufmt::uwriteln;

#[arduino_hal::entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);
    let mut serial = arduino_hal::default_serial!(dp, pins, 57600);

    /*
     * For examples (and inspiration), head to
     *
     *     https://github.com/Rahix/avr-hal/tree/main/examples
     *
     * NOTE: Not all examples were ported to all boards!  There is a good chance though, that code
     * for a different board can be adapted for yours.  The Arduino Uno currently has the most
     * examples available.
     */

    let mut led = pins.d13.into_output();

    let mut i2c = arduino_hal::I2c::new(
        dp.TWI,
        pins.a4.into_pull_up_input(),
        pins.a5.into_pull_up_input(),
        50000,
    );

    led.set_low();

    loop {
        led.set_high();

        let b: [u8; 2] = [124, 125];
        if let Err(err) = i2c.write(125, &b) {
            uwriteln!(&mut serial, "I2C error: {:?}", err as I2CError).unwrap();
        }
        arduino_hal::delay_ms(1000);
    }
}
