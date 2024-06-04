#![no_std]
#![no_main]

use {defmt_rtt as _, panic_probe as _};

use defmt::{info, unwrap};

use embassy_executor::Executor;
use static_cell::StaticCell;
static EMBASSY_EXECUTOR: StaticCell<Executor> = StaticCell::new();

use embassy_rp::multicore::{spawn_core1, Stack};
static mut EMBASSY_STACK: Stack<4096> = Stack::new();

use rp_pac::{RESETS, TIMER};
use rtic_monotonics::rp2040::prelude::*;
rp2040_timer_monotonic!(Mono);

#[rtic::app(device = embassy_rp)]
mod app {
    use super::*;
    use embassy_rp::gpio::{Level, Output};

    #[shared]
    struct Shared {}

    #[local]
    struct Local {}

    #[init()]
    fn init(_ctx: init::Context) -> (Shared, Local) {
        let p = embassy_rp::init(Default::default());
        Mono::start(TIMER, &RESETS);

        // Spawn heartbeat task
        let green_led = Output::new(p.PIN_19, Level::High);
        rtic_task::spawn(green_led).ok();

        let red_led = Output::new(p.PIN_18, Level::High);
        spawn_core1(
            p.CORE1,
            unsafe { &mut *core::ptr::addr_of_mut!(EMBASSY_STACK) },
            move || {
                let executor = EMBASSY_EXECUTOR.init(Executor::new());
                executor.run(|spawner| unwrap!(spawner.spawn(embassy_task(red_led))));
            },
        );

        (Shared {}, Local {})
    }

    #[task()]
    async fn rtic_task(
        _ctx: rtic_task::Context,
        mut green_led: Output<'static, embassy_rp::peripherals::PIN_19>,
    ) {
        info!("Hello, from RTIC!");
        let mut state = true;
        loop {
            if state {
                green_led.set_high();
            } else {
                green_led.set_low();
            }
            state = !state;
            Mono::delay(100.millis()).await;
        }
    }

    #[embassy_executor::task]
    async fn embassy_task(mut red_led: Output<'static, embassy_rp::peripherals::PIN_18>) {
        info!("Hello, from Embassy!");
        let mut state = true;
        loop {
            if state {
                red_led.set_high();
            } else {
                red_led.set_low();
            }
            state = !state;
            Mono::delay(1000.millis()).await;
        }
    }
}
