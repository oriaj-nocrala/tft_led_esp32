#![no_std]
#![no_main]

use display_interface_spi::SPIInterface;
use embedded_graphics::image::{Image, ImageRaw};
use embedded_hal_bus::spi::ExclusiveDevice;
use esp_backtrace as _;
use esp_hal::clock::CpuClock;
use esp_hal::delay::Delay;
use esp_hal::gpio::{Level, Output, OutputConfig};
use esp_hal::main;
use esp_hal::spi::master::{Config as SpiConfig, Spi};
use esp_hal::spi::Mode;
use esp_hal::time::{Duration, Instant, Rate};
use ili9341::{Ili9341, Orientation};
use embedded_graphics::prelude::*;
use embedded_graphics::pixelcolor::Rgb565;
use embedded_graphics::primitives::{PrimitiveStyleBuilder, Rectangle};


use log::info;

extern crate alloc;

#[main]
fn main() -> ! {
    // generator version: 0.3.1

    esp_println::logger::init_logger_from_env();

    let config = esp_hal::Config::default().with_cpu_clock(CpuClock::max());
    let _peripherals = esp_hal::init(config);

    let sck = _peripherals.GPIO18;
    let mosi = _peripherals.GPIO23;
    let miso = _peripherals.GPIO19;

    let cs = Output::new(_peripherals.GPIO5, Level::Low, OutputConfig::default());
    let dc = Output::new(_peripherals.GPIO2, Level::Low, OutputConfig::default());
    let reset = Output::new(_peripherals.GPIO4, Level::Low, OutputConfig::default());
    let mut led = Output::new(_peripherals.GPIO21, Level::Low, OutputConfig::default());
    led.set_high();
    let mut delay = Delay::new();

    let spi_peripheral = _peripherals.SPI2;

    let spi_config = SpiConfig::default().with_frequency(Rate::from_mhz(26)).with_mode(Mode::_0);


    let spi = Spi::new(spi_peripheral, spi_config).unwrap()
        .with_sck(sck)
        .with_mosi(mosi)
        .with_miso(miso);

    let bus = ExclusiveDevice::new(spi, cs, delay).unwrap();

    // let mut buffer = [0_u8; 512];
    let iface = SPIInterface::new(bus, dc);

    let mut display = Ili9341::new(
    iface,
    reset,
    &mut delay,
    Orientation::Portrait,
    ili9341::DisplaySize240x320,
    ).unwrap();

    // Rellena toda la pantalla de color azul
    // let style = PrimitiveStyleBuilder::new()
    //     .fill_color(Rgb565::BLUE)
    //     .build();

    // Rectangle::new(
    //     Point::zero(),
    //     Size::new(240, 320), // Tamaño completo de la pantalla
    // )
    // .into_styled(style)
    // .draw(&mut display)
    // .unwrap();

    let raw_image = ImageRaw::<Rgb565>::new(include_bytes!("../../output.raw"), 240);
    Image::new(&raw_image, Point::zero()).draw(&mut display).unwrap();

    esp_alloc::heap_allocator!(size: 72 * 1024);

    loop {
        info!("Hello world!");
        let delay_start = Instant::now();
        while delay_start.elapsed() < Duration::from_millis(500) {}
    }

    // for inspiration have a look at the examples at https://github.com/esp-rs/esp-hal/tree/esp-hal-v1.0.0-beta.0/examples/src/bin
}
