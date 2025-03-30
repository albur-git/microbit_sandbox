#![no_main]
#![no_std]

mod button;
mod led_matrix;
mod timer;
mod gpio_handler;

use gpio_handler::{GpioHandler, GpioPortId};
use button::Button;
use nrf52833_pac as pac;
use rtt_target::{rprintln, rtt_init_print};
use timer::Timer;



#[cortex_m_rt::entry]
fn start() -> ! {
    rtt_init_print!();

    // take ownership of nRF52833 device peripherals
    let peripherals = pac::Peripherals::take().unwrap();
    // take ownership of  the Cortex-M4 core peripherals
    let _core_peripherals = cortex_m::Peripherals::take().unwrap();

    // take peripherals
    let port_0 = peripherals.P0;

    // Initialize a Button 
    Button::init(peripherals.GPIOTE, &port_0);
    
    // == [Timer] ==
    Timer::init(peripherals.TIMER0);
   
    // == [LEDs] ==
    let gpio_handler = GpioHandler::init(GpioPortId::Port0, port_0);
    let led_col1_output = gpio_handler.get_push_pull_output(GpioPortId::Port0, 28);
    let led_row1_output = gpio_handler.get_push_pull_output(GpioPortId::Port0, 21);

    led_col1_output.set_low();
    led_row1_output.set_high();

    rprintln!("Starting Main-Loop");
    loop {
        cortex_m::asm::wfi();  // Wait for interrupt
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

