pub mod button_action;
pub mod gpio_input;
mod button_task;

use core::cell::RefCell;
use cortex_m::interrupt::Mutex;
use nrf52833_pac::{self as pac, GPIOTE, interrupt};
use gpio_input::GpioInput;

use self::button_action::{ButtonAAction, ButtonAction, ButtonBAction};
use self::button_task::ButtonTask;

pub struct GpioeHandler<A: ButtonAction, B: ButtonAction> {
    gpio_tasks_and_events: Mutex<RefCell<Option<GPIOTE>>>,
    button_a_task: Mutex<RefCell<Option<ButtonTask<A>>>>,
    button_b_task: Mutex<RefCell<Option<ButtonTask<B>>>>,
}

static GPIOE_HANDLER: GpioeHandler<ButtonAAction, ButtonBAction> = GpioeHandler {
    gpio_tasks_and_events: Mutex::new(RefCell::new(None)),
    button_a_task: Mutex::new(RefCell::new(None)),
    button_b_task: Mutex::new(RefCell::new(None)),
};

// concrete implementation of GpioeHandler for ButtonAAction and ButtonBAction --> provides a specific instantiation of GpioeHandler with those types
impl GpioeHandler<ButtonAAction, ButtonBAction> {
    // Initialize the button and related peripherals
    pub fn init(
        gpio_tasks_and_events: GPIOTE,
        button_a_task: ButtonTask<ButtonAAction>,
        button_b_task: ButtonTask<ButtonBAction>,
    ) {
        GpioeHandler::init_input_event(&gpio_tasks_and_events, &button_a_task.input_button);
        GpioeHandler::init_input_event(&gpio_tasks_and_events, &button_b_task.input_button);
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
                    button_a_task.check_event_clear_and_execute_task(gpio_tasks_and_events);
                }

                if let Some(ref button_b_task) = *GPIOE_HANDLER.button_b_task.borrow(cs).borrow() {
                    button_b_task.check_event_clear_and_execute_task(gpio_tasks_and_events);
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
