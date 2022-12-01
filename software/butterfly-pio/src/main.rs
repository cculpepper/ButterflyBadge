#![no_std]
#![no_main]

use embedded_hal::blocking::spi::write;
// The macro for our start-up function
use rp_pico::entry;

// Ensure we halt the program on panic (if we don't mention this crate it won't
// be linked)
use panic_halt as _;

// Pull in any important traits
use rp_pico::hal::prelude::*;

// A shorter alias for the Peripheral Access Crate, which provides low-level
// register access
use rp_pico::hal::pac;

// Import the Timer for Ws2812:
use rp_pico::hal::timer::Timer;

// A shorter alias for the Hardware Abstraction Layer, which provides
// higher-level drivers.
use rp_pico::hal;

// PIOExt for the split() method that is needed to bring
// PIO0 into useable form for Ws2812:
use rp_pico::hal::pio::PIOExt;

// Import useful traits to handle the ws2812 LEDs:
use smart_leds::{brightness, SmartLedsWrite, RGB8};

// Import the actual crate to handle the Ws2812 protocol:
use ws2812_pio::Ws2812;

// Currently 3 consecutive LEDs are driven by this example
// to keep the power draw compatible with USB:
const STRIP_LEN: usize = 256;

mod frames;
#[entry]
fn main() -> ! {
    // Grab our singleton objects
    let mut pac = pac::Peripherals::take().unwrap();
    let core = pac::CorePeripherals::take().unwrap();

    // Set up the watchdog driver - needed by the clock setup code
    let mut watchdog = hal::Watchdog::new(pac.WATCHDOG);

    // Configure the clocks
    //
    // The default is to generate a 125 MHz system clock
    let clocks = hal::clocks::init_clocks_and_plls(
        rp_pico::XOSC_CRYSTAL_FREQ,
        pac.XOSC,
        pac.CLOCKS,
        pac.PLL_SYS,
        pac.PLL_USB,
        &mut pac.RESETS,
        &mut watchdog,
    )
    .ok()
    .unwrap();

    // The single-cycle I/O block controls our GPIO pins
    let sio = hal::Sio::new(pac.SIO);

    // Set the pins up according to their function on this particular board
    let pins = rp_pico::Pins::new(
        pac.IO_BANK0,
        pac.PADS_BANK0,
        sio.gpio_bank0,
        &mut pac.RESETS,
    );

    // Setup a delay for the LED blink signals:
    let mut frame_delay =
        cortex_m::delay::Delay::new(core.SYST, clocks.system_clock.freq().to_Hz());

    // Import the `sin` function for a smooth hue animation from the
    // Pico rp2040 ROM:
    let sin = hal::rom_data::float_funcs::fsin::ptr();

    // Create a count down timer for the Ws2812 instance:
    let timer = Timer::new(pac.TIMER, &mut pac.RESETS);

    // Split the PIO state machine 0 into individual objects, so that
    // Ws2812 can use it:
    let (mut pio_stbd, sm0_stbd, _, _, _) = pac.PIO0.split(&mut pac.RESETS);
    let (mut pio_port, sm0_port, _, _, _) = pac.PIO1.split(&mut pac.RESETS);

    let mut ws_port = Ws2812::new(
        // Use pin 6 on the Raspberry Pi Pico (which is GPIO4 of the rp2040 chip)
        // for the LED data output:
        pins.gpio21.into_mode(),
        &mut pio_port,
        sm0_port,
        clocks.peripheral_clock.freq(),
        timer.count_down(),
    );

    // Instanciate a Ws2812 LED strip:
    let mut ws_stbd = Ws2812::new(
        // Use pin 6 on the Raspberry Pi Pico (which is GPIO4 of the rp2040 chip)
        // for the LED data output:
        pins.gpio20.into_mode(),
        &mut pio_stbd,
        sm0_stbd,
        clocks.peripheral_clock.freq(),
        timer.count_down(),
    );

    let mut leds = [RGB8::default(); 2 * STRIP_LEN];
    let mut write_leds = |brightness_level| {
        ws_stbd
            .write(brightness(
                leds[..STRIP_LEN].iter().copied(),
                brightness_level,
            ))
            .unwrap();
        ws_port
            .write(brightness(
                leds[STRIP_LEN..].iter().copied(),
                brightness_level,
            ))
            .unwrap();
    };

    let mut frame_num = 0;
    let frame_player = move |_dt: f32| {
        frame_num += 1;
        if frame_num >= frames::frames.len() {
            frame_num = 0;
        }

        for (i, led) in leds.iter_mut().enumerate() {
            *led = frames::frames[frame_num][i].into();
        }

        // Bring down the overall brightness of the strip to not blow
        // the USB power supply: every LED draws ~60mA, RGB means 3 LEDs per
        // ws2812 LED, for 3 LEDs that would be: 3 * 3 * 60mA, which is
        // already 540mA for just 3 white LEDs!
        let strip_brightness = 4; // Limit brightness to 64/256
        write_leds(strip_brightness)
    };

    tick_driver(
        20.,
        |delay| {
            frame_delay.delay_ms(delay);
        },
        frame_player,
    );
}

fn tick_driver(hz: f32, mut delayer: impl FnMut(u32), mut f: impl FnMut(f32)) -> ! {
    loop {
        f(1. / hz);
        delayer((1000. / hz) as u32);
    }
}
