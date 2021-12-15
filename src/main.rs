#![no_std]
#![no_main]
#![feature(global_asm)]
#![feature(asm)]

#[macro_use]
extern crate static_assertions;
extern crate rlibc;

mod aux;
mod gpio;
mod panic;
mod uart;
mod utils;

use aux::*;
use gpio::*;
use uart::*;
use utils::*;

struct Peripherial {
    aux: Aux,
    uart: Uart,
    gpio: GPIO,
}

impl Peripherial {
    pub fn new() -> Peripherial {
        let periph_base = 0x3F000000;
        Peripherial {
            aux: Aux::new(periph_base + 0x215000),
            uart: Uart::new(periph_base + 0x215040),
            gpio: GPIO::new(periph_base + 0x200000),
        }
    }
}

global_asm!(include_str!("start.s"));

#[no_mangle]
pub extern "C" fn main() {
    let mut periph = Peripherial::new();
    // periph.aux.enable_uart();
    // periph.gpio.set_mini_uart_tx(GPIOMiniUartTxPin::GPIO14);
    // periph.gpio.set_mini_uart_rx(GPIOMiniUartRxPin::GPIO15);
    // periph.gpio.set_pull_state(&[14, 15], GPIOPullState::Off);
    // periph.uart.enable_tx_rx();
    // let mut uart = periph.uart;
    // uart.send('h' as u8);
    // uart.send('!' as u8);
    // uart.send('\n' as u8);
    // loop {
    //     let c = uart.recv();
    //     uart.send(c);
    // }
    periph.gpio.set_gpio_func(16, GPIOFunc::Out);
    loop {
        periph.gpio.set_pin_out(&[16]);
        delay_s(1);
        periph.gpio.clear_pin_out(&[16]);
        delay_s(1);
    }
}
