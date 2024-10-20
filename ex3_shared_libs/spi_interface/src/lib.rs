/*
Written by Brian Lin
Summer 2024
References:
    ex3_shared_libs/tcp_interface - Devin Headrick and Rowan Rasmusson, Summer 2024
    https://github.com/rust-embedded/rust-spidev, Accessed 2024-07-24
*/
extern crate spidev;
use std::io::{Error, Read, Write};
use spidev::{Spidev, SpidevOptions, SpiModeFlags};
use tcp_interface::Interface;

pub const BUFFER_SIZE: usize = 1024;

/// SPI Interface for communication with simulated external peripherals
pub struct SpiInterface {
    spi: Spidev,
}

impl SpiInterface {
    pub fn new(path: &str) -> Result<SpiInterface, Error> {
        let mut spi = Spidev::open(path)?;
        let options = SpidevOptions::new()
            .bits_per_word(8)
            .max_speed_hz(500_000)
            .mode(SpiModeFlags::SPI_MODE_0)
            .build();
        spi.configure(&options)?;
        Ok(SpiInterface { spi })
    }
}

impl Interface for SpiInterface {
    fn send(&mut self, data: &[u8]) -> Result<usize, Error> {
        let n = self.spi.write(data)?;
        Ok(n)
    }

    fn read(&mut self, buffer: &mut [u8]) -> Result<usize, Error> {
        let n = self.spi.read(buffer)?;
        Ok(n)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // These tests are meant to be run with an emulator to
    // ensure the functionality of read and write

    #[test]
    fn test_spi_write() {
        let mut spi_interface = SpiInterface::new("/dev/spidev2.0").unwrap();
        if let Ok(n) = SpiInterface::send(&mut spi_interface, &[48, 48, 48, 48, 48]) {
            println!("Sent {} bytes", n);
        } else {
            // couldn't send bytes
        }
    }
    #[test]
    fn test_spi_read() {
        let mut spi_interface = SpiInterface::new("/dev/spidev2.0").unwrap();
        let mut buffer = [0u8; BUFFER_SIZE];
        loop {
            if let Ok(n) = SpiInterface::read(&mut spi_interface, &mut buffer) {
                println!("got dem bytes: {:?}", buffer);
                if n > 0 {
                    break;
                } else {
                    continue;
                }
            } else {
                println!("No bytes to read");
            }
        }
    }
}
