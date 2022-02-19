//! Raspberry Pi Pico でLチカ (SysTickを利用)
#![no_std]
#![no_main]

use core::cell::RefCell;
use core::ops::DerefMut;
use cortex_m::interrupt::{free, Mutex};
use cortex_m_rt::entry;
use cortex_m_rt::exception; // SysTick割り込み
use defmt_rtt as _;
use embedded_hal::digital::v2::ToggleableOutputPin;
use panic_probe as _;

use rp_pico as bsp;

use bsp::hal::gpio::{bank0::Gpio25, PushPullOutput};
use bsp::hal::{clocks::init_clocks_and_plls, pac, sio::Sio, watchdog::Watchdog};

// 割り込みハンドラからハードウェア制御できるように、static変数にする
// Mutex<RefCell<Option<共有変数>>> = Mutex::new(RefCell::new(None));
static G_LED: Mutex<RefCell<Option<bsp::hal::gpio::Pin<Gpio25, PushPullOutput>>>> =
    Mutex::new(RefCell::new(None));

// SysTickハンドラ
#[exception]
fn SysTick() {
    // クリティカルセクション内で操作
    free(|cs| {
        // LEDをトグルする
        if let Some(ref mut led) = G_LED.borrow(cs).borrow_mut().deref_mut() {
            let _ = led.toggle();
        }
    });
}

#[entry]
fn main() -> ! {
    let mut pac = pac::Peripherals::take().unwrap();
    let mut core = pac::CorePeripherals::take().unwrap();
    let mut watchdog = Watchdog::new(pac.WATCHDOG);
    let sio = Sio::new(pac.SIO);

    // External high-speed crystal on the pico board is 12Mhz
    let external_xtal_freq_hz = 12_000_000u32;
    let _clocks = init_clocks_and_plls(
        external_xtal_freq_hz,
        pac.XOSC,
        pac.CLOCKS,
        pac.PLL_SYS,
        pac.PLL_USB,
        &mut pac.RESETS,
        &mut watchdog,
    )
    .ok()
    .unwrap();

    let pins = bsp::Pins::new(
        pac.IO_BANK0,
        pac.PADS_BANK0,
        sio.gpio_bank0,
        &mut pac.RESETS,
    );

    // LEDピンの初期化
    let led_pin = pins.led.into_push_pull_output();

    // 所有権をstatic変数に移す
    // 操作はクリティカルセクション内で行う
    free(|cs| {
        G_LED.borrow(cs).replace(Some(led_pin));
    });

    // SysTickの設定
    // 自前でSysTickを制御するときは cortex_m::delay::Delay が使えないので注意
    core.SYST.disable_counter();
    core.SYST.clear_current();
    // set_reloadで設定する値は、(割り込み周期のクロック数 - 1)
    // Raspberry Pi Picoでは、1クロック=1マイクロ秒。
    core.SYST.set_reload(1_000_000 - 1); // 1秒周期
    core.SYST.enable_interrupt();
    core.SYST.enable_counter();

    // メインループでは何もしない
    loop {}
}
