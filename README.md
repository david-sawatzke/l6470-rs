# L6470 driver for Linux in Rust

This library can be used to control one or more STMicroelectronics L6470 motor driver over SPI using the Rust programming language.


### Prerequisites

You need a Linux Single Board Computer (SBC) with a SPI port enabled and some GPIO

See [Linux SPIDEV doc](https://www.kernel.org/doc/Documentation/spi/spidev)
See [Linux Sysfs GPIO doc](https://www.kernel.org/doc/Documentation/gpio/sysfs.txt)

### Installing

A step by step series of examples that tell you how to get a development env running

Say what the step will be

```rust
extern crate linux_l6470;

use linux_l6470::L6470;
use linux_l6470::Motors;
use linux_l6470::Direction;

fn main() -> Result<(), std::io::Error> {

    let driver = linux_l6470::L6470Connector::new("/dev/spidev1.0")
        .build()?;




    driver.init();

    driver.send_run(Motors::all(), Direction::CW, 0xFFFFFFFF);
    // driver.send_go_until(Motors::all(), Direction::CW, 0xFFFFFFFF);

    Ok(())
}
```

## Built With

* [Rust Spidev](https://github.com/rust-embedded/rust-spidev) - Linux SPI API with Rust

## Versioning

We use [SemVer](http://semver.org/) for versioning.


## Authors

* **Dolt.ch (Samuel Dolt)** - *Initial work* - [Dolt.ch](https://dolt.ch)


## License

This project is Licensed under either of

 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any
additional terms or conditions.

## Bibliography

* [ST L6470 - Fully integrated microstepping motor driver with motion engine
and SPI](http://www.st.com/content/ccc/resource/technical/document/datasheet/a5/86/06/1c/fa/b2/43/db/CD00255075.pdf/files/CD00255075.pdf/jcr:content/translations/en.CD00255075.pdf)
* [ST DT0065 - Design Tip: A Guide to understanding L6470, L6480, and powerSTEP01 output voltage levels](http://www.st.com/content/ccc/resource/technical/document/design_tip/group0/23/c2/31/36/86/77/45/41/DM00311185/files/DM00311185.pdf/jcr:content/translations/en.DM00311185.pdf)
* [ST AN4241 - Application note: L6470 and L6472: fully integrated stepper motor drivers](http://www.st.com/content/ccc/resource/technical/document/application_note/f3/f5/78/c8/38/11/44/02/DM00075650.pdf/files/DM00075650.pdf/jcr:content/translations/en.DM00075650.pdf)
