use core::cell::RefCell;
use cortex_m::interrupt::Mutex;
use nrf52833_pac::{self as pac, GPIOTE, P0, interrupt};
use rtt_target::{rprintln, rtt_init_print};

// Define a struct to encapsulate the button's state and operations
pub struct Button {
    gpioe_channel_number: usize,
    button_pin: usize,
    //p0_shared: Mutex<RefCell<Option<P0>>>,
}

// Static variables cannot be mutable, therefore it is necessary to use a RefCell (internal mutability) wrapped into a mutex (thread safety)
static BUTTON_A: Mutex<RefCell<Option<Button>>> = Mutex::new(RefCell::new(None));
static BUTTON_B: Mutex<RefCell<Option<Button>>> = Mutex::new(RefCell::new(None));

// Use a static Mutex to protect shared resources
static GPIO_TASKS_AND_EVENTS: Mutex<RefCell<Option<GPIOTE>>> = Mutex::new(RefCell::new(None));
static PORT_0: Mutex<RefCell<Option<P0>>> = Mutex::new(RefCell::new(None));

impl Button {
    // Initialize the button and related peripherals
    pub fn init(
        gpio_tasks_and_events: GPIOTE,
        //port_0: Mutex<RefCell<Option<P0>>>,
        port_0: P0,
    ) {
        // Store the button and peripherals in the static Mutexes
        cortex_m::interrupt::free(|cs| {
            GPIO_TASKS_AND_EVENTS.borrow(cs).replace(Some(gpio_tasks_and_events));
            PORT_0.borrow(cs).replace(Some(port_0));
            *BUTTON_A.borrow(cs).borrow_mut() = Some(Button::init_button(0, 14));
            *BUTTON_B.borrow(cs).borrow_mut() = Some(Button::init_button(1, 23));
        });
        unsafe { pac::NVIC::unmask(pac::Interrupt::GPIOTE); }
        pac::NVIC::unpend(pac::Interrupt::GPIOTE);
    }

    fn init_button(
        gpioe_channel_number: usize,
        button_pin: usize,
    ) -> Button {
        let button_1_bit = 1u32 << button_pin;
        cortex_m::interrupt::free(|cs| {
            if let Some(ref mut port_0) = *PORT_0.borrow(cs).borrow_mut() {
                unsafe { port_0.outclr.write(|w|w.bits(button_1_bit)); } 
                port_0.pin_cnf[button_pin].write(|w| {
                    w.dir().input();
                    w.input().connect();
                    w.pull().disabled();
                    w.drive().s0s1();
                    w.sense().disabled();
                    w
                });
            }
        });

        let gpioe_event_channel_bit: u32 = 1u32 << gpioe_channel_number;
        cortex_m::interrupt::free(|cs| {
            if let Some(ref mut gpio_tasks_and_events) = *GPIO_TASKS_AND_EVENTS.borrow(cs).borrow_mut() {
                gpio_tasks_and_events.config[gpioe_channel_number].write(|w| {
                    w.mode().event().polarity().hi_to_lo();
                    w.port().clear_bit();
                    unsafe { w.psel().bits(button_pin as u8) }
                });
            
                unsafe { gpio_tasks_and_events.intenset.write(|w| w.bits(gpioe_event_channel_bit)); }
                gpio_tasks_and_events.events_in[gpioe_channel_number].write(|w| w);
            }
        });
        Button {
            gpioe_channel_number,
            button_pin,
        }
    }

    // Handle the button interrupt
    fn handle_interrupt() {
        cortex_m::interrupt::free(|cs| {
            if let Some(ref gpio_tasks_and_events) =
                *GPIO_TASKS_AND_EVENTS.borrow(cs).borrow()
            {
                if let Some(ref button_a) = *BUTTON_A.borrow(cs).borrow() {
                    if gpio_tasks_and_events.events_in[button_a.gpioe_channel_number].read().bits() != 0 {
                        rprintln!("Button A has been pressed!");
                        gpio_tasks_and_events.events_in[button_a.gpioe_channel_number].write(|w| w);    // clear event
                    }
                }
                if let Some(ref button_b) = *BUTTON_B.borrow(cs).borrow() {
                    if gpio_tasks_and_events.events_in[button_b.gpioe_channel_number].read().bits() != 0 {
                        rprintln!("Button B has been pressed!");
                        gpio_tasks_and_events.events_in[button_b.gpioe_channel_number].write(|w| w);    // clear event
                    }
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
