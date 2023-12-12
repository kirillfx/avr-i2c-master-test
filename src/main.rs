#![no_std]
#![no_main]
#![feature(abi_avr_interrupt)]

use core::sync::atomic::{AtomicBool, Ordering};

use arduino_hal::i2c::Error as I2CError;
use arduino_hal::prelude::*;
use panic_halt as _;
use ufmt::{uwrite, uwriteln};

static BTN_FLAG: AtomicBool = AtomicBool::new(false);

// Attach btn interrupt handler connected to pin d2
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

    let mut led = pins.d13.into_output();

    // Use internal pullup resistor
    let _btn_pin = pins.d2.into_pull_up_input();

    dp.EXINT.eicra.modify(|_, w| w.isc0().bits(0x03));
    dp.EXINT.eimsk.modify(|_, w| w.int0().set_bit());

    // Configure with external pullup resistors. I use a pair of 1k8
    let mut i2c = arduino_hal::I2c::with_external_pullup(
        dp.TWI,
        pins.a4.into_floating_input(),
        pins.a5.into_floating_input(),
        100000,
    );

    let slave_address: u8 = 0x26;
    // GCA (Global Call Address)
    let global_addr: u8 = 0x00;

    uwriteln!(&mut serial, "Initialized").unwrap();

    // uwriteln!(&mut serial, "Devices to write to:").unwrap();
    // let _ = i2c.i2cdetect(&mut serial, arduino_hal::i2c::Direction::Write);

    // uwriteln!(&mut serial, "Devices to read from:").unwrap();
    // let _ = i2c.i2cdetect(&mut serial, arduino_hal::i2c::Direction::Read);

    loop {
        if BTN_FLAG.load(Ordering::SeqCst) {
            led.set_high();

            // WRITE
            uwriteln!(&mut serial, "Writing to i2c address: 0x{:X}", slave_address).unwrap();
            let b = [1, 2, 3, 4];
            uwrite!(&mut serial, "Sending: ").unwrap();
            b.iter().for_each(|b| {
                uwrite!(&mut serial, "{} ", b).unwrap();
            });

            uwrite!(&mut serial, "\n").unwrap();

            match i2c.write(slave_address, &b) {
                Err(err) => uwriteln!(&mut serial, "I2C error: {:?}", err as I2CError).unwrap(),
                Ok(_) => uwriteln!(&mut serial, "Data has been sent").unwrap(),
            };

            arduino_hal::delay_ms(100);

            // READ
            // Expect back only 2 bytes just to see how slave handles Stop condition
            let mut read_buf: [u8; 2] = [0u8; 2];
            match i2c.read(slave_address, &mut read_buf) {
                Ok(_) => {
                    uwrite!(&mut serial, "Received: ").unwrap();
                    read_buf.iter().for_each(|b| {
                        uwrite!(&mut serial, "{} ", b).unwrap();
                    });
                    uwrite!(&mut serial, "\n").unwrap();
                }
                Err(err) => uwriteln!(&mut serial, "I2C error: {:?}", err as I2CError).unwrap(),
            }

            uwriteln!(&mut serial, "\n\n").unwrap();

            led.set_low();
            BTN_FLAG.store(false, Ordering::SeqCst);
        }
    }
}
