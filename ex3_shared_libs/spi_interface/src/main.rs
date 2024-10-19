extern crate spidev;
use std::io;
use std::io::prelude::*;
use spidev::{Spidev, SpidevOptions, SpidevTransfer, SpiModeFlags};

fn create_spi() -> io::Result<Spidev> {
    let mut spi = Spidev::open("/dev/spidev2.0")?;
    let options = SpidevOptions::new()
         .bits_per_word(8)
         .max_speed_hz(20_000)
         .mode(SpiModeFlags::SPI_MODE_0)
         .build();
    spi.configure(&options)?;
    Ok(spi)
}

/// perform half duplex operations using Read and Write traits
fn half_duplex(spi: &mut Spidev) -> io::Result<()> {
    let mut rx_buf = [0_u8; 10];
    spi.write(&[0x01, 0x02, 0x03])?;
    spi.read(&mut rx_buf)?;
    println!("{:?}", rx_buf);
    Ok(())
}

/// Perform full duplex operations using Ioctl
fn full_duplex(spi: &mut Spidev) -> io::Result<()> {
    // "write" transfers are also reads at the same time with
    // the read having the same length as the write
    let tx_buf = [0x01, 0x02, 0x03];
    let mut rx_buf = [0; 3];
    {
        let mut transfer = SpidevTransfer::read_write(&tx_buf, &mut rx_buf);
        spi.transfer(&mut transfer)?;
    }
    println!("{:?}", rx_buf);
    Ok(())
}

fn main() {
    let mut spi = create_spi().unwrap();
    println!("{:?}", half_duplex(&mut spi).unwrap());
    println!("{:?}", full_duplex(&mut spi).unwrap());
}