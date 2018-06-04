# L6470 driver for Linux in Rust

This library can be used to control one or more STMicroelectronics L6470 motor driver over SPI using the Rust programming language.

## Getting Started

cargo add L6470

### Prerequisites

You need a Linux Single Board Computer (SBC) with a SPI port enabled and some GPIO

See [Linux SPIDEV doc](https://www.kernel.org/doc/Documentation/spi/spidev)
See [Linux Sysfs GPIO doc](https://www.kernel.org/doc/Documentation/gpio/sysfs.txt)

### Installing

A step by step series of examples that tell you how to get a development env running

Say what the step will be

```
extern crate linux_l6470;

use linux_l6470::L6470;

fn main() {
    let motors = L6470::new("/dev/spi")
         .with_daisy_chain_length(4)
         .build()

    

}
```

And repeat

```
until finished
```

End with an example of getting some data out of the system or using it for a little demo

## Running the tests

Explain how to run the automated tests for this system

### Break down into end to end tests

Explain what these tests test and why

```
Give an example
```

### And coding style tests

Explain what these tests test and why

```
Give an example
```

## Deployment

Add additional notes about how to deploy this on a live system

## Built With

* [Rust Spidev](https://github.com/rust-embedded/rust-spidev) - Linux SPI API with Rust
* [Rust Sysfs GPIO](https://github.com/rust-embedded/rust-sysfs-gpio) - Linux GPIO API with Rust

## Versioning

We use [SemVer](http://semver.org/) for versioning. For the versions available, see the [tags on this repository](https://github.com/your/project/tags). 

## Authors

* **Dolt.ch (Samuel Dolt)** - *Initial work* - [Dolt.ch](https://dolt.ch)

See also the list of [contributors](https://github.com/your/project/contributors) who participated in this project.

## License

This project is Licensed under either of

 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

This project is licensed under the MIT License - see the [LICENSE.md](LICENSE.md) file for details

## Contribution

Please read [CONTRIBUTING.md](https://gist.github.com/PurpleBooth/b24679402957c63ec426) for details on our code of conduct, and the process for submitting pull requests to us.

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any
additional terms or conditions.

## Acknowledgments

* Hat tip to anyone whose code was used
* Inspiration
* etc

## Bibliography

* [ST L6470 - Fully integrated microstepping motor driver with motion engine
and SPI](http://www.st.com/content/ccc/resource/technical/document/datasheet/a5/86/06/1c/fa/b2/43/db/CD00255075.pdf/files/CD00255075.pdf/jcr:content/translations/en.CD00255075.pdf)
* [ST DT0065 - Design Tip: A Guide to understanding L6470, L6480, and powerSTEP01 output voltage levels](http://www.st.com/content/ccc/resource/technical/document/design_tip/group0/23/c2/31/36/86/77/45/41/DM00311185/files/DM00311185.pdf/jcr:content/translations/en.DM00311185.pdf)
* [ST AN4241 - Application note: L6470 and L6472: fully integrated stepper motor drivers](http://www.st.com/content/ccc/resource/technical/document/application_note/f3/f5/78/c8/38/11/44/02/DM00075650.pdf/files/DM00075650.pdf/jcr:content/translations/en.DM00075650.pdf)
