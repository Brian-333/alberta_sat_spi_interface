/*
Written by Brian Lin
Fall 2024
References:
    https://digilent.com/reference/pmod/pmodoled/reference-manual
    https://www.adafruit.com/datasheets/SSD1306.pdf                     - datasheet (commands included)
*/
use gpio_cdev::{Chip, LineRequestFlags, EventRequestFlags, EventType};
use spi_interface::SpiInterface;
use tcp_interface::Interface;

pub struct PmodOled {
    dc: u32,
    reset: u32,
    vbatc: u32,
    vddc: u32,
}

impl PmodOled {
    pub fn new(dc: u32, reset: u32, vbatc: u32, vddc: u32) -> PmodOled {
        PmodOled {
            dc,
            reset,
            vbatc,
            vddc,
        }
    }

    pub fn turn_on(&mut self, path: &str) -> Result<(), gpio_cdev::Error> {
        let mut chip = Chip::new(path).unwrap();
        let line_numbers: &[u32] = &[self.dc, self.reset, self.vbatc, self.vddc];
        let lines = chip.get_lines(line_numbers)?;

        let mut spi_interface = SpiInterface::new("/dev/spidev2.0").unwrap();

        let dc_handle = lines[0].request(LineRequestFlags::OUTPUT, 0, "dc-output")?;
        let reset_handle = lines[1].request(LineRequestFlags::OUTPUT, 1, "reset-output")?;
        let vbatc_handle = lines[2].request(LineRequestFlags::OUTPUT, 0, "vbatc-output")?;
        let vddc_handle = lines[3].request(LineRequestFlags::OUTPUT, 0, "vddc-output")?;

        std::thread::sleep(std::time::Duration::from_millis(10000));

        // 1. Power on vdd
        vddc_handle.set_value(1)?;
        std::thread::sleep(std::time::Duration::from_millis(100));
        // send display off command 
        let _ = SpiInterface::send(&mut spi_interface, &[0xAE]);
        print!("Display OFF \n");
        let _ = SpiInterface::send(&mut spi_interface, &[0x00]);
        print!("Mode Set \n");
        vbatc_handle.set_value(1)?;

        // let n = SpiInterface::send(&mut spi_interface, &[0xAE]);
        // 2. After VDD become stable, set RES# pin LOW (logic low) for at least 3us (t1) (4) and then HIGH (logic high).
        reset_handle.set_value(0)?;
        std::thread::sleep(std::time::Duration::from_micros(3));
        reset_handle.set_value(1)?;
        // 3. After set RES# pin LOW (logic low), wait for at least 3us (t2). Then Power ON VCC. (1)
        // std::thread::sleep(std::time::Duration::from_micros(3));
        // 4. After VCC become stable, send command AFh for display ON. SEG/COM will be ON after 100ms (tAF).

        std::thread::sleep(std::time::Duration::from_millis(100));
        let n = SpiInterface::send(&mut spi_interface, &[0xAF]);
        print!("Display on \n");
        std::thread::sleep(std::time::Duration::from_millis(10000));
        print!("n: {}\n", n.unwrap());
        dc_handle.set_value(0)?;

        Ok(())
    }
    
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn turn_on() {
        let mut pmod_oled = PmodOled::new(0, 1, 2, 3);
        pmod_oled.turn_on("/dev/gpiochip0").unwrap();
    }

}
