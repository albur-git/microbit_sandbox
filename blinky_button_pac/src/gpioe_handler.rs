use core::cell::RefCell;
use cortex_m::interrupt::Mutex;
use nrf52833_pac::{self as pac, GPIOTE, interrupt};

use crate::{gpio_handler::GpioInput, gpioe_task::ButtonATask, gpioe_task::ButtonBTask};

pub struct GpioeHandler {
    gpio_tasks_and_events: Mutex<RefCell<Option<GPIOTE>>>,
    button_a_task: Mutex<RefCell<Option<ButtonATask>>>,
    button_b_task: Mutex<RefCell<Option<ButtonBTask>>>,
}

static GPIOE_HANDLER: GpioeHandler = GpioeHandler {
    gpio_tasks_and_events: Mutex::new(RefCell::new(None)),
    button_a_task: Mutex::new(RefCell::new(None)),
    button_b_task: Mutex::new(RefCell::new(None)),
};

impl GpioeHandler {
    // Initialize the button and related peripherals
    pub fn init(
        gpio_tasks_and_events: GPIOTE,
        button_a_task: ButtonATask,
        button_b_task: ButtonBTask,
    ) {
        GpioeHandler::init_input_event(&gpio_tasks_and_events, &button_a_task.input_button);
        GpioeHandler::init_input_event(&gpio_tasks_and_events, &button_b_task.input_button);
        // Store the button and peripherals in the stati
        cortex_m::interrupt::free(|cs| {
            GPIOE_HANDLER.gpio_tasks_and_events.borrow(cs).replace(Some(gpio_tasks_and_events));
            GPIOE_HANDLER.button_a_task.borrow(cs).replace(Some(button_a_task));
            GPIOE_HANDLER.button_b_task.borrow(cs).replace(Some(button_b_task));
        });
        unsafe { pac::NVIC::unmask(pac::Interrupt::GPIOTE); }
        pac::NVIC::unpend(pac::Interrupt::GPIOTE);
    }

    fn init_input_event(
        gpio_tasks_and_events: &GPIOTE,
        button: &GpioInput
    ) {
        gpio_tasks_and_events.config[button.event_channel as usize].write(|w| {
            w.mode().event().polarity().hi_to_lo();
            w.port().clear_bit();
            unsafe { w.psel().bits(button.pin as u8) }
        });
            
        unsafe { gpio_tasks_and_events.intenset.write(|w| w.bits(button.event_channel_bit_mask)); }
        gpio_tasks_and_events.events_in[button.event_channel as usize].write(|w| w);
    }

    // Handle the button interrupt
    fn handle_interrupt() {
        cortex_m::interrupt::free(|cs| {
            if let Some(ref gpio_tasks_and_events) = *GPIOE_HANDLER.gpio_tasks_and_events.borrow(cs).borrow() {
                if let Some(ref button_a_task) = *GPIOE_HANDLER.button_a_task.borrow(cs).borrow() {
                    if gpio_tasks_and_events.events_in[button_a_task.input_button.event_channel as usize].read().bits() != 0 {
                        gpio_tasks_and_events.events_in[button_a_task.input_button.event_channel as usize].write(|w| w);    // clear event
                        button_a_task.execute(cs);
                    }
                }

                if let Some(ref button_b_task) = *GPIOE_HANDLER.button_b_task.borrow(cs).borrow() {
                    if gpio_tasks_and_events.events_in[button_b_task.input_button.event_channel as usize].read().bits() != 0 {
                        gpio_tasks_and_events.events_in[button_b_task.input_button.event_channel as usize].write(|w| w);    // clear event
                        button_b_task.execute(cs);
                    }
                }
            }
        });
    }
}

// Interrupt handler for the button
#[interrupt]
fn GPIOTE() {
    GpioeHandler::handle_interrupt();
}
