use std::env;
use spi_interface::SpiInterface;
use tcp_interface::Interface;
pub const BUFFER_SIZE: usize = 1024;

fn test_spi_write() {
    let mut spi_interface = SpiInterface::new("/dev/spidev2.0").unwrap();
    if let Ok(n) = SpiInterface::send(&mut spi_interface, &[48, 48, 48, 48, 48]) {
        println!("Sent {} bytes", n);
    } else {
        // couldn't send bytes
    }
}

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

fn main() {
    test_spi_write();
    test_spi_read();
}