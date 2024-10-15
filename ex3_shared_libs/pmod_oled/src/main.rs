use pmod_oled::PmodOled;
use std::env;

fn main() {
    let mut pmod_oled = PmodOled::new(0, 1, 2, 3, 4, 5);
    env::set_var("RUST_BACKTRACE", "1");
    pmod_oled.turn_on("/dev/gpiochip0").unwrap();
}