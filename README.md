# USB-PIO

A wrapper to be able to use the RP2040 (and RP2350?) PIO as a cheap generi USB
GPIO/SPI/I2C/UART device, like the FTDI chips.

## To do

- [ ] Add basic RP2040 firmware
  - [ ] Add support for loading code to interact with the PIO?
- [ ] Test on RP2350
- [ ] Test throughput
- Add host-side USB wrappers
  - [ ] Add Rust wrapper (rusb? nusb?)
  - [ ] Add Go wrapper (gousb?)
  - [ ] Add Python wrapper (pyusb?)
  - [ ] Add Zig wrapper
