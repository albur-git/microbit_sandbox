use core::cell::RefCell;
use cortex_m::interrupt::Mutex;
use nrf52833_pac::{self as pac, GPIOTE, P0, interrupt};
use rtt_target::{rprintln, rtt_init_print};

// Define a struct to encapsulate the button's state and operations
pub struct Button {
    event_channel: usize,
    //button_port: u32,
    button_pin: usize,
    //p0_shared: Mutex<RefCell<Option<P0>>>,
}

static BUTTON_A : Button = Button {
    event_channel: 0,
    button_pin: 14,
};

static BUTTON_B : Button = Button {
    event_channel: 1,
    button_pin: 23,
};

// Static variables cannot be mutable, therefore it is necessary to use a RefCell (internal mutability) wrapped into a mutex (thread safety)
// Option is used to safely initialize set to and replace at a later point
static GPIO_TASKS_AND_EVENTS: Mutex<RefCell<Option<GPIOTE>>> = Mutex::new(RefCell::new(None));
static PORT_0: Mutex<RefCell<Option<P0>>> = Mutex::new(RefCell::new(None));

impl Button {
    // Initialize the button and related peripherals
    pub fn init(
        gpio_tasks_and_events: GPIOTE,
        port_0: P0, // TODO: should take port abstraction instead of concrete port P0. makes it also necessary to pass in the button_port: usize
    ) {
        // Store the button and peripherals in the static Mutexes
        cortex_m::interrupt::free(|cs| {
            GPIO_TASKS_AND_EVENTS.borrow(cs).replace(Some(gpio_tasks_and_events));
            PORT_0.borrow(cs).replace(Some(port_0));
        });
        Button::init_button(&BUTTON_A);
        Button::init_button(&BUTTON_B);
        unsafe { pac::NVIC::unmask(pac::Interrupt::GPIOTE); }
        pac::NVIC::unpend(pac::Interrupt::GPIOTE);
    }

    fn init_button(
        button: &Button
    ) {
        cortex_m::interrupt::free(|cs| {
            // initialize pin to input
            let button_1_bit = 1u32 << button.button_pin;
            if let Some(ref mut port_0) = *PORT_0.borrow(cs).borrow_mut() {
                unsafe { port_0.outclr.write(|w|w.bits(button_1_bit)); } 
                port_0.pin_cnf[button.button_pin].write(|w| {
                    w.dir().input();
                    w.input().connect();
                    w.pull().disabled();
                    w.drive().s0s1();
                    w.sense().disabled();
                    w
                });
            }

            // initialize event channel
            let gpioe_event_channel_bit: u32 = 1u32 << button.event_channel;
            if let Some(ref mut gpio_tasks_and_events) = *GPIO_TASKS_AND_EVENTS.borrow(cs).borrow_mut() {
                gpio_tasks_and_events.config[button.event_channel].write(|w| {
                    w.mode().event().polarity().hi_to_lo();
                    w.port().clear_bit();
                    unsafe { w.psel().bits(button.button_pin as u8) }
                });
            
                unsafe { gpio_tasks_and_events.intenset.write(|w| w.bits(gpioe_event_channel_bit)); }
                gpio_tasks_and_events.events_in[button.event_channel].write(|w| w);
            }
        });
    }

    // Handle the button interrupt
    fn handle_interrupt() {
        cortex_m::interrupt::free(|cs| {
            if let Some(ref gpio_tasks_and_events) = *GPIO_TASKS_AND_EVENTS.borrow(cs).borrow() {
                if gpio_tasks_and_events.events_in[BUTTON_A.event_channel].read().bits() != 0 {
                    rprintln!("Button A has been pressed!");
                    gpio_tasks_and_events.events_in[BUTTON_A.event_channel].write(|w| w);    // clear event
                }

                if gpio_tasks_and_events.events_in[BUTTON_B.event_channel].read().bits() != 0 {
                    rprintln!("Button B has been pressed!");
                    gpio_tasks_and_events.events_in[BUTTON_B.event_channel].write(|w| w);    // clear event
                }
            }
        });
    }
}

// Interrupt handler for the button
#[interrupt]
fn GPIOTE() {
    Button::handle_interrupt();
}
