#![no_main]
#![no_std]

use cortex_m::asm::nop;
use embedded_hal::digital::{OutputPin, PinState};   // note: nrf52833_hal implements embedded_hal. This allows to write platform agnostic code by using the definitions in embedded_hal instead of nrf52833_hal
use nrf52833_hal::gpio::Level;
use rtt_target::{rprintln, rtt_init_print};
use nrf52833_hal::pac as pac;   // Peripheral access crate (contains the microcontroller specific register information and forms the foundation of the hal)
use nrf52833_hal as hal;    // hardware abstraction layer


#[cortex_m_rt::entry]
fn start() -> ! {
    rtt_init_print!();
    // take ownership of peripherals
    // Get a handle to the nRF52833 device peripherals
    let peripherals = pac::Peripherals::take().unwrap();
    // Get a handle to the Cortex-M4 core peripherals
    let core_periperals = cortex_m::Peripherals::take().unwrap();

    let port0 = hal::gpio::p0::Parts::new(peripherals.P0);  // use Parts constructor to get handle on port0 from nrf52833 peripherals

    let _led_col1 = port0.p0_28.into_push_pull_output(Level::Low).degrade();
    let mut led_row1 = port0.p0_21.into_push_pull_output(Level::Low).degrade();

    match led_row1.set_high() {
        Ok(()) => {
            rprintln!("LED set high successfully");
        }
        Err(e) => {
            rprintln!("Failed to set LED low: {:?}", e);
        }
    }  

    rprintln!("Hello World!");

    let mut is_on = false;
    // Endless loop needed as function return value is specified as ! which means it never treturns.
    loop {
        let _ = led_row1.set_state(PinState::from(is_on));
        for _ in 0..40_000 {
            nop();
        }
        is_on = !is_on;
    }
}

#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    rprintln!("{}", info);
    exit();
}

fn exit() -> ! {
    loop {
        rprintln!("Exiting now");
        cortex_m::asm::bkpt();
    }
}
