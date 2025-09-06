#![no_std]
#![no_main]

use defmt_rtt as _;
use panic_probe as _;
use cortex_m_rt::entry;
// Import HAL traits and timer delay
use rp_pico::hal::prelude::*;
use embedded_hal::blocking::delay::DelayMs;
use rp_pico::hal::{
    clocks::init_clocks_and_plls,
    pac,
    sio::Sio,
    watchdog::Watchdog,
    gpio::Pins,
    timer::Timer,
};

#[entry]
fn main() -> ! {
    // Peripherals, clocks, and watchdog initialization
    let mut pac = pac::Peripherals::take().unwrap();
    let mut watchdog = Watchdog::new(pac.WATCHDOG);
    let clocks = init_clocks_and_plls(
        rp_pico::XOSC_CRYSTAL_FREQ,
        pac.XOSC,
        pac.CLOCKS,
        pac.PLL_SYS,
        pac.PLL_USB,
        &mut pac.RESETS,
        &mut watchdog,
    )
    .unwrap();

    // GPIOとタイマー
    let sio = Sio::new(pac.SIO);
    let pins = Pins::new(pac.IO_BANK0, pac.PADS_BANK0, sio.gpio_bank0, &mut pac.RESETS);
    // Onboard LED is GPIO25
    // Onboard LED is GPIO25
    let mut led = pins.gpio25.into_push_pull_output();
    let mut timer = Timer::new(pac.TIMER, &mut pac.RESETS, &clocks);

    // LEDが点滅して点滅するたびにコンソールに点灯状態を表示する
    loop {
        led.set_high().unwrap();
        defmt::println!("点灯！");
        timer.delay_ms(1000u32);
        led.set_low().unwrap();
        defmt::println!("消灯！");
        timer.delay_ms(1000u32);
    }
}
