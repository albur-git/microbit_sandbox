#![no_main]
#![no_std]

mod button;

use core::cell::RefCell;
use core::sync::atomic::{AtomicU32, Ordering};
use button::Button;
use cortex_m::{peripheral::NVIC, interrupt::Mutex};
use nrf52833_pac::{self as pac, gpiote, interrupt, GPIOTE, P0};
use rtt_target::{rprintln, rtt_init_print};

static TICK_COUNT: AtomicU32 = AtomicU32::new(0);

static P_0: Mutex<RefCell<Option<P0>>> = Mutex::new(RefCell::new(None));


static LED_ROW1_P0_INDEX: usize = 21;
static LED_ROW1_P0_BIT: u32 = 1u32 << LED_ROW1_P0_INDEX;

#[cortex_m_rt::entry]
fn start() -> ! {
    rtt_init_print!();

    // take ownership of nRF52833 device peripherals
    let peripherals = pac::Peripherals::take().unwrap();
    // take ownership of  the Cortex-M4 core peripherals
    let core_peripherals = cortex_m::Peripherals::take().unwrap();

    // take peripherals
    let port_0_shared = peripherals.P0;

    // Initialize a Button 
    Button::init(peripherals.GPIOTE, port_0_shared);
   
    // // == [LEDs] ==
    // let led_col1_p0_index: usize = 28;
    // let led_col1_p0_bit: u32 = 1u32 << led_col1_p0_index;
    // unsafe { port_0.outclr.write(|w|w.bits(LED_ROW1_P0_BIT)); }
    //     //p0.outclr.write(|w|w.pin21().variant(nrf52833_pac::p0::outclr::PIN21_AW::CLEAR));
    //     //p0.outclr.write(|w|w.pin21().bit(true));
    // port_0.pin_cnf[LED_ROW1_P0_INDEX].write(|w| {
    //     w.dir().output();          // Set as output
    //     w.input().disconnect();    // Disconnect input buffer
    //     w.pull().disabled();       // Disable pull-up and pull-down resistors
    //     w.drive().s0s1();          // Standard '0', standard '1' drive configuration
    //     w.sense().disabled();      // No sensing mechanism
    //     w
    // });

    // unsafe { port_0.outclr.write(|w|w.bits(led_col1_p0_bit)); }
    //     //p0.outclr.write(|w|w.pin28().variant(nrf52833_pac::p0::outclr::PIN28_AW::CLEAR));
    //     //p0.outclr.write(|w|w.pin28().bit(true));
    // port_0.pin_cnf[led_col1_p0_index].write(|w| {
    //     w.dir().output();          // Set as output
    //     w.input().disconnect();    // Disconnect input buffer
    //     w.pull().disabled();       // Disable pull-up and pull-down resistors
    //     w.drive().s0s1();          // Standard '0', standard '1' drive configuration
    //     w.sense().disabled();      // No sensing mechanism
    //     w
    // });

    // // set pin to turn led on
    // unsafe { port_0.outset.write(|w|w.bits(LED_ROW1_P0_BIT)); }

    // // == [Timer] ==
    // let timer_index: usize = 0;
    // let timer = &peripherals.TIMER0;
    // timer.tasks_stop.write(|w| w.tasks_stop().set_bit());   // disable timer before configuration
    // timer.bitmode.write(|w| w.bitmode()._32bit());          // Set 32-bit mode to make counter 32 bit wide
    // timer.prescaler.write(|w| w.prescaler().variant(4));    // Prescaler = 2^4 = 16 (16 MHz / 16 = 1 MHz)
    // timer.cc[timer_index].write(|w| w.cc().variant(1000_000));        // Set the compare value for 1-second intervals (1 Mhz clock => 1000_000_000 ticks = 1 second)
    // timer.intenset.write(|w| w.compare0().set());           // Enable the compare event interrupt
    // timer.shorts.write(|w| w.compare0_clear().enabled());   // Enable counter clear on compare match, writes to SHORTS register
    // timer.tasks_clear.write(|w| w.tasks_clear().set_bit());
    // timer.tasks_start.write(|w| w.tasks_start().set_bit());

    // // cortex_m::interrupt::free critical section disables all interrupts momentarily
    // // As the code cannot be interrupted anway it is not necessary to persist critical sections for selective mutex functionality
    // cortex_m::interrupt::free(move |cs| {
    //     unsafe {
    //         NVIC::unmask(pac::Interrupt::TIMER0);
    //     }
    //     pac::NVIC::unpend(pac::interrupt::TIMER0);

    //     *P_0.borrow(cs).borrow_mut() = Some(port_0);
    // });

    rprintln!("Starting Main-Loop");
    loop {
        cortex_m::asm::wfi();  // Wait for interrupt
    }
}


// // Define the TIMER0 interrupt handler
// #[interrupt]
// fn TIMER0() {
//     // Access the peripherals
//     let peripherals = unsafe { pac::Peripherals::steal() };
//     let timer = &peripherals.TIMER0;

//     // Clear the compare event flag
//     timer.events_compare[0].write(|w| w.events_compare().clear_bit());

//     // Increment the seconds count
//     TICK_COUNT.fetch_add(1, Ordering::Relaxed);
//     rprintln!("Timer interrupd fired {} times",TICK_COUNT.load(Ordering::Relaxed));

//     cortex_m::interrupt::free(|cs| {
//         if let Some(p0) = P_0.borrow(cs).borrow().as_ref() {
//             if p0.out.read().bits() & LED_ROW1_P0_BIT != 0 {
//                 unsafe { p0.outclr.write(|w|w.bits(LED_ROW1_P0_BIT)); }
//             } else {
//                 unsafe { p0.outset.write(|w|w.bits(LED_ROW1_P0_BIT)); }
//             }
//         }
//     });
// }

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
