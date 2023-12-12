i2c-master-test
===============

Rust project for the _Arduino Nano_ used as i2c master to test [i2c slave implementation](https://github.com/kirillfx/avr-i2c-slave).

To trigger i2c transactions interruption is used. I2C used an external pullup resistors.

]
## Build Instructions

- Specify `RAVEDUDE_PORT` in `.envrc` if direnv is used. If you're on linux with nix, change env var in `flake.nix`

- Install prerequisites as described in the [`avr-hal` README] (`avr-gcc`, `avr-libc`, `avrdude`, [`ravedude`]).

- Run `cargo build` to build the firmware.

- Run `cargo run` to flash the firmware to a connected board.  If `ravedude`
   fails to detect your board, check its documentation at
   <https://crates.io/crates/ravedude>.

- `ravedude` will open a console session after flashing where you can interact
   with the UART console of your board.

[`avr-hal` README]: https://github.com/Rahix/avr-hal#readme
[`ravedude`]: https://crates.io/crates/ravedude

## License
Licensed under either of

 - Apache License, Version 2.0
   ([LICENSE-APACHE](LICENSE-APACHE) or <http://www.apache.org/licenses/LICENSE-2.0>)
 - MIT license
   ([LICENSE-MIT](LICENSE-MIT) or <http://opensource.org/licenses/MIT>)

at your option.

## Contribution
Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.
