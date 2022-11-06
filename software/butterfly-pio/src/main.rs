//! # Pico WS2812 RGB LED Example
//! Drives 3 WS2812 LEDs connected directly to the Raspberry Pi Pico.
//! This assumes you drive the Raspberry Pi Pico via USB power, so that VBUS
//! delivers the 5V and at least enough amperes to drive the LEDs.
//!
//! For a more large scale and longer strips you should use an extra power
//! supply for the LED strip (or know what you are doing ;-) ).
//!
//! The example also comes with an utility function to calculate the colors
//! from HSV color space. It also limits the brightness a bit to save a
//! few milliamperes - be careful if you increase the strip length you will
//! quickly get into power consumption of multiple amperes.
//!
//! The example assumes you connected the data input to pin 6 of the
//! Raspberry Pi Pico, which is GPIO4 of the rp2040. Here is a circuit
//! diagram that shows the assumed setup:
//!
//! ```text
//!  _______________      /----------------------\
//! |+5V  /---\  +5V|----/         _|USB|_       |
//! |DO <-|LED|<- DI|-\           |1  R 40|-VBUS-/
//! |GND  \---/  GND|--+---\      |2  P 39|
//!  """""""""""""""   |    \-GND-|3    38|
//!                    |          |4  P 37|
//!                    |          |5  I 36|
//!                    \------GP4-|6  C   |
//!                               |7  O   |
//!                               |       |
//!                               .........
//!                               |20   21|
//!                                """""""
//! Symbols:
//!     - (+) crossing lines, not connected
//!     - (o) connected lines
//! ```
//!
//! See the `Cargo.toml` file for Copyright and license details.

#![no_std]
#![no_main]

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

    let mut leds_stbd: [RGB8; STRIP_LEN] = [(0, 0, 0).into(); STRIP_LEN];
    let mut leds_port: [RGB8; STRIP_LEN] = [(0, 0, 0).into(); STRIP_LEN];
    let mut t = 0.0;

    // Bring down the overall brightness of the strip to not blow
    // the USB power supply: every LED draws ~60mA, RGB means 3 LEDs per
    // ws2812 LED, for 3 LEDs that would be: 3 * 3 * 60mA, which is
    // already 540mA for just 3 white LEDs!
    let strip_brightness = 15; // Limit brightness to 64/256

    // Slow down timer by this factor (0.1 will result in 10 seconds):
    let animation_speed = 0.2;
    let mut chase_led_num= 0;
    let mut num_leds_on= 3;
    let mut count = 0;
    
    let mut rainbow_driver = RainbowyDriver { hue_state: 0. };

    loop {
        for (i, led) in leds_stbd.iter_mut().enumerate() {
            if ((i+chase_led_num) % num_leds_on) == 0{
                // An offset to give 3 consecutive LEDs a different color:

                let hue_offs = match i % 10 {
                    1 => 0.25,
                    2 => 0.5,
                    _ => 0.0,
                };

                let sin_11 = sin((t + hue_offs) * 2.0 * core::f32::consts::PI);
                // Bring -1..1 sine range to 0..1 range:
                let sin_01 = (sin_11 + 1.0) * 0.2;

                let hue = 360.0 * sin_01;
                let sat = 1.0;
                let val = 1.0;

                let rgb = hsv2rgb_u8(hue, sat, val);
                *led = rgb.into();
            } else {
                let rgb = hsv2rgb_u8(0.0,0.0,0.0);
                *led = rgb.into();
            }
        }
        
        for (i, led) in leds_port.iter_mut().enumerate() {
            if (in_chain(i, chase_led_num)){

                let hue_offs = match i % 3 {
                    1 => 0.25,
                    2 => 0.5,
                    _ => 0.0,
                };

                let sin_11 = sin((t + hue_offs) * 2.0 * core::f32::consts::PI);
                // Bring -1..1 sine range to 0..1 range:
                let sin_01 = (sin_11 + 1.0) * 0.2;

                let hue = 360.0 * sin_01;
                let sat = 1.0;
                let val = 1.0;

                let rgb = hsv2rgb_u8(hue, sat, val);
                *led = rgb.into();
            } else{
                let rgb = hsv2rgb_u8(0.0,0.0,0.0);
                *led = rgb.into();

            }            
        }
        
        {
        //    rainbow_driver.update(leds_port.as_mut_slice());
        }

        count += 1;
        if (count >= 20){
            chase_led_num+= 1;
            count = 0;

        }
        if (chase_led_num >= 255){
            chase_led_num = 0;
        }

        // Here the magic happens and the `leds` buffer is written to the
        // ws2812 LEDs:
        ws_stbd.write(brightness(leds_stbd.iter().copied(), strip_brightness))
            .unwrap();
        ws_port.write(brightness(leds_port.iter().copied(), strip_brightness))
            .unwrap();

        // Wait a bit until calculating the next frame:
        frame_delay.delay_ms(1); // ~60 FPS

        // Increase the time counter variable and make sure it
        // stays inbetween 0.0 to 1.0 range:
        t += (16.0 / 1000.0) * animation_speed;
        while t > 1.0 {
            t -= 1.0;
        }
    }
}


trait LedUpdate {
    fn update(&mut self, leds: &mut [RGB8]);
}


struct RainbowyDriver {
    hue_state: f32,
}

impl LedUpdate for RainbowyDriver {
    fn update(&mut self, leds: &mut [RGB8]) {
        for (idx, led_value) in leds.iter_mut().enumerate() {
            let hue = ((idx as f32 / 255.) + self.hue_state) % 1.0;
            let full_value = hsv2rgb_u8(hue, 1., 1.);
            
            *led_value = full_value.into();
        }
        self.hue_state = (self.hue_state + (1./255.)) % 1.0;
    }
}

pub fn hsv2rgb(hue: f32, sat: f32, val: f32) -> (f32, f32, f32) {
    let c = val * sat;
    let v = (hue / 60.0) % 2.0 - 1.0;
    let v = if v < 0.0 { -v } else { v };
    let x = c * (1.0 - v);
    let m = val - c;
    let (r, g, b) = if hue < 60.0 {
        (c, x, 0.0)
    } else if hue < 120.0 {
        (x, c, 0.0)
    } else if hue < 180.0 {
        (0.0, c, x)
    } else if hue < 240.0 {
        (0.0, x, c)
    } else if hue < 300.0 {
        (x, 0.0, c)
    } else {
        (c, 0.0, x)
    };
    (r + m, g + m, b + m)
}

pub fn hsv2rgb_u8(h: f32, s: f32, v: f32) -> (u8, u8, u8) {
    let r = hsv2rgb(h, s, v);

    (
        (r.0 * 255.0) as u8,
        (r.1 * 255.0) as u8,
        (r.2 * 255.0) as u8,
        )
}

pub fn in_chain(i: usize, num: usize) -> bool{
    	let i_signed = i as i32;
	let num_signed = num as i32;
	const CHAIN_LEN: i32 = 17;

	(i_signed - num_signed).abs() < CHAIN_LEN
}
