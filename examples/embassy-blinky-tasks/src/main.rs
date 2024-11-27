/*
 * this is an example from 
 * https://github.com/embassy-rs/embassy/
 * in examples/rp/src/bin/blinky_two_tasks.rs
 *
 *
 */

/*
// this code works on RP2040
#![no_std]
#![no_main]
//#![feature(type_alias_impl_trait)]

use defmt_rtt as _;
use embassy_executor::Spawner;
use embassy_rp::gpio::{Level, Output};
use embassy_time::{Duration, Timer};
use panic_probe as _;

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    defmt::info!("Initializing...");

    let peripherals = embassy_rp::init(Default::default());
    let mut led = Output::new(peripherals.PIN_25, Level::Low);

    defmt::info!("Initialized.");

    loop {
        defmt::info!("LED on!");
        led.set_high();
        Timer::after(Duration::from_millis(50)).await;

        defmt::info!("LED off!");
        led.set_low();
        Timer::after(Duration::from_millis(950)).await;
    }
}
*/


#![no_std]
#![no_main]
/// This example demonstrates how to access a given pin from more than one embassy task
/// The on-board LED is toggled by two tasks with slightly different periods, leading to the
/// apparent duty cycle of the LED increasing, then decreasing, linearly. The phenomenon is similar
/// to interference and the 'beats' you can hear if you play two frequencies close to one another
/// [Link explaining it](https://www.physicsclassroom.com/class/sound/Lesson-3/Interference-and-Beats)
use defmt::*;
use embassy_executor::Spawner;
use embassy_rp::gpio;
use embassy_sync::blocking_mutex::raw::ThreadModeRawMutex;
use embassy_sync::mutex::Mutex;
use embassy_time::{Duration, Ticker};
use gpio::{AnyPin, Level, Output};
use {defmt_rtt as _, panic_probe as _};

type LedType = Mutex<ThreadModeRawMutex, Option<Output<'static>>>;
static LED: LedType = Mutex::new(None);

//use rp2040_boot2;
//#[link_section = ".boot2"]
//#[used]
//pub static BOOT_LOADER: [u8; 256] = rp2040_boot2::BOOT_LOADER_W25Q080;

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    let p = embassy_rp::init(Default::default());
    // set the content of the global LED reference to the real LED pin
    let led = Output::new(AnyPin::from(p.PIN_25), Level::High);
    // inner scope is so that once the mutex is written to, the MutexGuard is dropped, thus the
    // Mutex is released
    {
        *(LED.lock().await) = Some(led);
    }
    let dt = 100 * 1_000_000;
    let k = 1.003;

    unwrap!(spawner.spawn(toggle_led(&LED, Duration::from_nanos(dt))));
    unwrap!(spawner.spawn(toggle_led(&LED, Duration::from_nanos((dt as f64 * k) as u64))));
}

#[embassy_executor::task(pool_size = 2)]
async fn toggle_led(led: &'static LedType, delay: Duration) {
    let mut ticker = Ticker::every(delay);
    loop {
        {
            let mut led_unlocked = led.lock().await;
            if let Some(pin_ref) = led_unlocked.as_mut() {
                pin_ref.toggle();
            }
        }
        ticker.next().await;
    }
}

