use pmod_oled::PmodOled;
use std::env;

fn main() {
    let mut pmod_oled = PmodOled::new(1, 2, 3, 4, 5, 6);
    env::set_var("RUST_BACKTRACE", "1");
    pmod_oled.turn_on("/dev/spidev1.0").unwrap();
}