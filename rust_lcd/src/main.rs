// src/main.rs

// std and main are not available for bare metal software
#![no_std]
#![no_main]

mod lcd;

use lcd::LCD;
use panic_halt as _;

use cortex_m_rt::entry;
use hal::{delay::Delay, pac, prelude::*};
use cortex_m_semihosting::{hprintln};
use stm32f1xx_hal as hal;

#[entry]
fn main() -> ! {
    /* Get access to device and core peripherals */
    let dp = pac::Peripherals::take().unwrap();
    let cp = cortex_m::Peripherals::take().unwrap();

    /* Get access to RCC, AFIO and GPIOA */
    let mut rcc = dp.RCC.constrain();
    let mut flash = dp.FLASH.constrain();
    let mut gpioa = dp.GPIOA.split(&mut rcc.apb2);
    let mut gpiob = dp.GPIOB.split(&mut rcc.apb2);

    // Freeze clocks
    let clocks = rcc.cfgr.freeze(&mut flash.acr);

    // Get delay instance
    let delay = Delay::new(cp.SYST, clocks);

    // Configure pin for LCD
    let rs = gpioa.pa0.into_push_pull_output(&mut gpioa.crl);
    let en = gpioa.pa1.into_push_pull_output(&mut gpioa.crl);
    let d4 = gpiob.pb12.into_push_pull_output(&mut gpiob.crh);
    let d5 = gpiob.pb13.into_push_pull_output(&mut gpiob.crh);
    let d6 = gpiob.pb14.into_push_pull_output(&mut gpiob.crh);
    let d7 = gpiob.pb15.into_push_pull_output(&mut gpiob.crh);

    let mut lcd = LCD::new(rs, en, d4, d5, d6, d7, delay);
    lcd.init();

    lcd.send_string("Hello World!");
    hprintln!("should work").unwrap();

    loop {}
}
