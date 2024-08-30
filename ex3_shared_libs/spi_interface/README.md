# Interfaces for connecting components

This library provides interfaces which are use by handlers and allow them to communicate with external peripherals, such as subsystems and payloads. OLED power on and off sequences are included

## What is this

Read and send functions are part of the SpiInterface struct and can be called whenever a process wants to simulate communicating with a peripheral.

## Testing

### OLED SPI sub
- [Pmod OLED](https://digilent.com/reference/pmod/pmodoled/reference-manual)
    - Pmod JE pins
        - D/C: 1
        - Reset: 2
        - VBATC: 3
        - VDDC: 4
    - Corresponding CHIP[GPIO](https://crates.io/crates/gpio-cdev)1
        - D/C: 0
        - Reset: 1
        - VBATC: 2
        - VDDC: 3

### Testing the SpiInterface

TBD
