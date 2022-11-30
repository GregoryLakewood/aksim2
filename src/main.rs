#![no_main]
#![no_std]

use cortex_m::delay::Delay;

use rust_aksim2 as _;

use stm32_hal2::{
    clocks::{Clocks, SpeedError},
    pac, gpio::{Pin, Port, PinMode, OutputSpeed},
};

trait ToggleableOutputPin {
    /// Error type
    type Error;

    /// Toggle pin output.
    fn toggle(&mut self) -> Result<(), Self::Error>;
}

#[cortex_m_rt::entry]
fn main() -> ! {
    // Set up ARM Cortex-M peripherals. These are common to many MCUs, including all STM32 ones.
    let cp = cortex_m::Peripherals::take().unwrap();
    // Set up peripherals specific to the microcontroller you're using.
    let _dp = pac::Peripherals::take().unwrap();

    // This line is required to prevent the debugger from disconnecting on entering WFI.
    // This appears to be a limitation of many STM32 families. Not required in production code,
    // and significantly increases power consumption in low-power modes.
    stm32_hal2::debug_workaround();

    // Create an initial clock configuration that uses the MCU's internal oscillator (HSI),
    // sets the MCU to its maximum system clock speed.

    let mut delay = system_clock_config(cp).unwrap();

    // LD2 pin on Nucleo Board
    let mut led2 = Pin::new(Port::A, 5, PinMode::Output);
    led2.set_low();
    led2.output_speed(OutputSpeed::High);

    loop {
        led2.toggle().unwrap();
        delay.delay_ms(1000);
    }
}

fn system_clock_config(cp: cortex_m::Peripherals) -> Result<Delay, SpeedError> {
    let clock_cfg = Clocks::default();
    clock_cfg.setup()?;
    // Setup a delay, based on the Cortex-m systick.
    Ok(Delay::new(cp.SYST, clock_cfg.systick()))
}

impl ToggleableOutputPin for Pin {
    type Error = core::convert::Infallible;

    fn toggle(&mut self) -> Result<(), Self::Error> {
        if self.is_high() {
            Pin::set_low(self);
        } else {
            Pin::set_high(self);
        }
        Ok(())
    }
}
