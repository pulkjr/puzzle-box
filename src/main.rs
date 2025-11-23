#![no_std]
#![no_main]

use bsp::entry;
use defmt_rtt as _;
use panic_probe as _;
use puzzle_box::hardware::HardwareBus;
use puzzle_box::PuzzleBox;
use rp_pico::hal::timer::Timer;

use rp_pico as bsp;

use bsp::hal::{
    clocks::{init_clocks_and_plls, Clock},
    pac,
    sio::Sio,
    watchdog::Watchdog,
};

#[entry]
fn main() -> ! {
    // Take ownership of the RP2040's peripheral access crate (PAC).
    // This gives access to all hardware peripherals (GPIO, ADC, clocks, etc.).
    let mut pac = pac::Peripherals::take().unwrap();

    // Take ownership of the Cortex-M0+ core peripherals (SysTick, NVIC, etc.).
    let core = pac::CorePeripherals::take().unwrap();

    // Create a watchdog timer instance. Used to reset the chip if it hangs.
    let mut watchdog = Watchdog::new(pac.WATCHDOG);

    // Create a SIO (Single-cycle I/O) instance. Provides fast access to GPIO pins.
    let sio = Sio::new(pac.SIO);

    // External high-speed crystal frequency on the Pico board (12 MHz).
    // This is used to configure the system clocks.
    let external_xtal_freq_hz = 12_000_000u32;

    // Initialize the system clocks and PLLs (Phase-Locked Loops).
    // This sets up the CPU, USB, and peripheral clocks based on the external crystal.
    let clocks = init_clocks_and_plls(
        external_xtal_freq_hz,
        pac.XOSC,        // External crystal oscillator
        pac.CLOCKS,      // Clock control registers
        pac.PLL_SYS,     // System PLL
        pac.PLL_USB,     // USB PLL
        &mut pac.RESETS, // Reset controller
        &mut watchdog,   // Watchdog for safe startup
    )
    .ok()
    .unwrap();

    // Create a delay abstraction using the SysTick timer.
    // Useful for blocking delays in milliseconds/microseconds.
    let mut delay = cortex_m::delay::Delay::new(core.SYST, clocks.system_clock.freq().to_Hz());

    // Initialize the Timer peripheral
    let timer = Timer::new(pac.TIMER, &mut pac.RESETS, &clocks);

    // Initialize all GPIO pins into a type-safe `Pins` struct.
    // This gives access to named pins like gpio0, gpio1, led, etc.
    let pins = bsp::Pins::new(
        pac.IO_BANK0,    // IO bank (controls pin functions)
        pac.PADS_BANK0,  // Pad controls (pull-ups, drive strength, etc.)
        sio.gpio_bank0,  // SIO GPIO bank
        &mut pac.RESETS, // Reset controller
    );

    // Initialize the ADC peripheral (Analog-to-Digital Converter).
    // This allows us to read analog voltages from pins GPIO26–28.
    let mut adc = rp_pico::hal::adc::Adc::new(pac.ADC, &mut pac.RESETS);

    // Create the HardwareBus abstraction, which owns all hardware peripherals.
    // - LED pin configured as push-pull output
    // - Buttons configured as pull-down inputs
    // - GPIO26 configured as an ADC-capable pin
    // - ADC peripheral passed in for analog reads
    let hardware = HardwareBus::new(
        pins.led.into_push_pull_output(),
        pins.gpio0.into_pull_up_input(),
        pins.gpio1.into_pull_up_input(),
        pins.gpio2.into_pull_up_input(),
        pins.gpio3.into_pull_up_input(),
        pins.gpio4.into_push_pull_output(),
        pins.gpio26.into_floating_disabled(),
        adc,
    );

    // Create your PuzzleBox abstraction, which uses the HardwareBus
    // to manage game logic and interact with hardware.
    let mut puzzle_box = PuzzleBox::new(hardware);

    // Super loop: runs forever, repeatedly ticking the puzzle box logic.
    loop {
        // Get current time in microseconds
        let now_us = timer.get_counter().ticks();

        // Convert to milliseconds
        let now_ms = now_us / 1000;

        puzzle_box.tick(now_ms);
    }
}
