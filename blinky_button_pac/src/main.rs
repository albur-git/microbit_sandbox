#![no_main]
#![no_std]

mod gpioe_handler;
mod led_matrix;
mod timer;
mod gpio_handler;
mod timer_task;

use gpio_handler::{GpioHandler, GpioPortId};
use gpioe_handler::GpioeHandler;
use gpioe_handler::button_action::{ButtonAAction, ButtonBAction};
use nrf52833_pac as pac;
use rtt_target::{rprintln, rtt_init_print};
use timer::Timer;
use timer_task::BlinkyTask;

#[cortex_m_rt::entry]
fn start() -> ! {
    rtt_init_print!();

    // take ownership of nRF52833 device peripherals
    let peripherals = pac::Peripherals::take().unwrap();
    // take ownership of  the Cortex-M4 core peripherals
    let _core_peripherals = cortex_m::Peripherals::take().unwrap();

   
    // == [LEDs] ==
    let gpio_p0_handler = GpioHandler::new(GpioPortId::Port0, peripherals.P0);
    let led_col1_output = gpio_p0_handler.get_push_pull_output(GpioPortId::Port0, 28);
    let led_col2_output = gpio_p0_handler.get_push_pull_output(GpioPortId::Port0, 11);
    let led_col3_output = gpio_p0_handler.get_push_pull_output(GpioPortId::Port0, 31);
    let led_columns = [led_col1_output, led_col2_output, led_col3_output];
    //let _led_col4_output = gpio_p0_handler.get_push_pull_output(GpioPortId::Port1, 5);
    //let _led_col5_output = gpio_p0_handler.get_push_pull_output(GpioPortId::Port0, 30);
    let led_row1_output = gpio_p0_handler.get_push_pull_output(GpioPortId::Port0, 21);
    let led_row2_output = gpio_p0_handler.get_push_pull_output(GpioPortId::Port0, 22);
    let led_row3_output = gpio_p0_handler.get_push_pull_output(GpioPortId::Port0, 15);
    let led_rows = [led_row1_output, led_row2_output, led_row3_output];

    // Initialize a Button 
    let button_a_task = gpio_p0_handler.get_input(0, 14).into_button_event_task(ButtonAAction);
    let button_b_task = gpio_p0_handler.get_input(1, 23).into_button_event_task(ButtonBAction);
    GpioeHandler::init(peripherals.GPIOTE, button_a_task, button_b_task);

    // == [Timer] ==
    let blinky_task = BlinkyTask::new(led_columns, led_rows);
    Timer::init(peripherals.TIMER0, blinky_task);
   
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

