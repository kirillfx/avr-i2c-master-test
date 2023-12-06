#![no_std]
#![no_main]
#![feature(abi_avr_interrupt)]

use core::sync::atomic::{AtomicBool, Ordering};

use arduino_hal::prelude::*;
// use embedded_hal::blocking::i2c::Write;
use arduino_hal::i2c::Error as I2CError;
use panic_halt as _;
use ufmt::uwriteln;

static BTN_FLAG: AtomicBool = AtomicBool::new(false);

#[avr_device::interrupt(atmega328p)]
fn INT0() {
    avr_device::interrupt::free(|_| {
        BTN_FLAG.store(true, Ordering::SeqCst);
    });
}

#[arduino_hal::entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);
    let mut serial = arduino_hal::default_serial!(dp, pins, 57600);

    unsafe { avr_device::interrupt::enable() };

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

    let mut btn_pin = pins.d2.into_output_high();

    dp.EXINT.eicra.modify(|_, w| w.isc0().bits(0x03));
    dp.EXINT.eimsk.modify(|_, w| w.int0().set_bit());

    // let mut i2c = arduino_hal::I2c::new(
    //     dp.TWI,
    //     pins.a4.into_pull_up_input(),
    //     pins.a5.into_pull_up_input(),
    //     100000,
    // );

    let mut i2c = arduino_hal::I2c::with_external_pullup(
        dp.TWI,
        pins.a4.into_floating_input(),
        pins.a5.into_floating_input(),
        100000,
    );

    led.set_low();

    arduino_hal::delay_ms(1000);
    led.set_high();

    let addr: u8 = 38;
    let global_addr: u8 = 0x00;

    uwriteln!(&mut serial, "Initialized").unwrap();

    let b: [u8; 1] = [0; 1];

    // i2c.i2cdetect(&mut serial, arduino_hal::i2c::Direction::Write);
    // i2c.i2cdetect(&mut serial, arduino_hal::i2c::Direction::Read);

    loop {
        if BTN_FLAG.load(Ordering::SeqCst) {
            BTN_FLAG.store(false, Ordering::SeqCst);

            uwriteln!(&mut serial, "Writing to i2c at {}", addr).unwrap();
            let b: [u8; 1] = [124];
            if let Err(err) = i2c.write(addr, &b) {
                uwriteln!(&mut serial, "I2C error: {:?}", err as I2CError).unwrap();
            };
        }

        // {
        //     uwriteln!(&mut serial, "Writing to i2c at {}", global_addr).unwrap();
        //     let b: [u8; 1] = [124];
        //     if let Err(err) = i2c.write(global_addr, &b) {
        //         uwriteln!(&mut serial, "I2C error: {:?}", err as I2CError).unwrap();
        //     };
        // }
        // if let Err(err) = i2c.read(0x0f, &mut b) {
        //     uwriteln!(&mut serial, "I2C error: {:?}", err as I2CError).unwrap();
        // };

        // led.set_low();
        // arduino_hal::delay_ms(1000);
    }
}
