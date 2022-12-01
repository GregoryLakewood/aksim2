#![no_main]
#![no_std]

use cortex_m::delay::Delay;

use aksim2 as _;

use stm32_hal2::{
    clocks::{Clocks, SpeedError},
    gpio::{OutputSpeed, Pin, PinMode, Port},
    pac,
    spi::{BaudRate, Spi, SpiConfig, SpiMode},
};

trait ToggleableOutputPin {
    /// Error type
    type Error;

    /// Toggle pin output.
    fn toggle(&mut self) -> Result<(), Self::Error>;
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

struct Aksim2<R> {
    spi: Spi<R>,
    cs: Pin,
}

impl<R> Aksim2<R> {
    fn new(spi: Spi<R>, mut cs: Pin) -> Self{
        // Configure CS pin
        cs.mode(PinMode::Output);
        cs.set_high();
        cs.output_speed(OutputSpeed::High);
        Aksim2 { spi: spi, cs: cs }
    }

    fn read() -> Result<u8, stm32_hal2::spi::Error> {
        Ok(1)
    }
}

#[cortex_m_rt::entry]
fn main() -> ! {
    let cp = cortex_m::Peripherals::take().unwrap();
    let dp = pac::Peripherals::take().unwrap();
    stm32_hal2::debug_workaround();

    let mut delay = system_clock_config(cp).unwrap();

    // LD2 pin on Nucleo Board
    let mut led2 = Pin::new(Port::A, 5, PinMode::Output);
    led2.set_low();
    led2.output_speed(OutputSpeed::High);

    // Configure pins for SPI3
    let _sck = Pin::new(Port::C, 10, PinMode::Alt(6));
    let _miso = Pin::new(Port::C, 11, PinMode::Alt(6));
    let _mosi = Pin::new(Port::C, 12, PinMode::Alt(6));

    // Configure CS pin for SPI3
    let mut cs = Pin::new(Port::A, 15, PinMode::Output);

    let spi_cfg = SpiConfig {
        mode: SpiMode::mode3(), // SpiConfig::default() uses mode 0.
        ..Default::default()
    };

    // Set up an SPI peripheral, running at 4Mhz, in SPI mode 0.
    let mut spi = Spi::new(
        dp.SPI1,
        spi_cfg,
        BaudRate::Div32, // Eg 80Mhz apb clock / 32 = 2.5Mhz SPI clock.
    );

    spi.read();

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
