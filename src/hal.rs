use display_interface_spi::SPIInterface;

use esp_idf_hal::{
    delay::Ets,
    gpio::{IOPin, InputPin, Output, OutputPin, PinDriver},
    ledc::{LedcChannel, LedcTimer},
    peripheral::Peripheral,
    peripherals,
    prelude::*,
    spi::{Dma, SpiAnyPins, SpiDeviceDriver, SpiDriver, SpiDriverConfig},
};

use crate::{display_driver, keyboard::CardputerKeyboard};

#[allow(clippy::too_many_arguments)]
pub fn prepare_display<SPI: SpiAnyPins>(
    spi: impl Peripheral<P = SPI> + 'static,
    sdo: impl Peripheral<P = impl OutputPin> + 'static,
    sdi: Option<impl Peripheral<P = impl InputPin> + 'static>,
    sclk: impl Peripheral<P = impl OutputPin> + 'static,
    cs: Option<impl Peripheral<P = impl OutputPin> + 'static>,
    rst: impl Peripheral<P = impl OutputPin> + 'static,
    dc: impl Peripheral<P = impl OutputPin> + 'static,
    bl: impl Peripheral<P = impl OutputPin> + 'static,
    ledc_timer: impl Peripheral<P = impl LedcTimer> + 'static,
    ledc_channel: impl Peripheral<P = impl LedcChannel> + 'static,
) -> display_driver::ST7789<
    SPIInterface<
        SpiDeviceDriver<'static, SpiDriver<'static>>,
        PinDriver<'static, impl OutputPin, Output>,
    >,
    esp_idf_hal::gpio::PinDriver<'static, impl OutputPin, esp_idf_hal::gpio::Output>,
    esp_idf_hal::gpio::PinDriver<'static, impl OutputPin, esp_idf_hal::gpio::Output>,
> {
    let config = esp_idf_hal::spi::config::Config::new()
        .baudrate(80.MHz().into())
        .data_mode(esp_idf_hal::spi::config::MODE_0)
        .queue_size(1);
    let device = SpiDeviceDriver::new_single(
        spi,
        sclk,
        sdo,
        sdi,
        cs,
        &SpiDriverConfig::new().dma(Dma::Auto(4096)),
        &config,
    )
    .unwrap();

    let pin_dc = PinDriver::output(dc).unwrap();

    let spi_interface = SPIInterface::new(device, pin_dc);

    // let ledc_config = esp_idf_svc::hal::ledc::config::TimerConfig::new().frequency(25.kHz().into());
    // let timer = LedcTimerDriver::new(ledc_timer, &ledc_config).unwrap();

    // let backlight_pwm = LedcDriver::new(ledc_channel, timer, bl).unwrap();
    // backlight_pwm.set_duty(backlight_pwm.get_max_duty()).unwrap();

    let rst_pin = PinDriver::output(rst).unwrap();
    let bl_pin = PinDriver::output(bl).unwrap();

    display_driver::ST7789::new(spi_interface, Some(rst_pin), Some(bl_pin), 240, 135)
}

pub fn cardputer_peripherals<'a>(
    peripherals: peripherals::Peripherals,
) -> (
    display_driver::ST7789<
        SPIInterface<
            SpiDeviceDriver<'static, SpiDriver<'static>>,
            PinDriver<'static, impl OutputPin, Output>,
        >,
        esp_idf_hal::gpio::PinDriver<'static, impl OutputPin, esp_idf_hal::gpio::Output>,
        esp_idf_hal::gpio::PinDriver<'static, impl OutputPin, esp_idf_hal::gpio::Output>,
    >,
    CardputerKeyboard<'a>,
) {
    // display

    let mut display = prepare_display(
        peripherals.spi2,
        peripherals.pins.gpio35,
        None as Option<esp_idf_hal::gpio::Gpio37>, //not true but we need to make the compiler happy
        peripherals.pins.gpio36,
        Some(peripherals.pins.gpio37),
        peripherals.pins.gpio33,
        peripherals.pins.gpio34,
        peripherals.pins.gpio38,
        peripherals.ledc.timer0,
        peripherals.ledc.channel0,
    );

    let mut delay = Ets;

    display.hard_reset(&mut delay).unwrap();
    display.init(&mut delay).unwrap();
    display
        .set_orientation(display_driver::Orientation::Landscape)
        .unwrap();
    display
        .set_address_window(0 + 40, 0 + 53, 240 - 1 + 40, 135 + 53)
        .unwrap();

    // keyboard

    let mux_pins = [
        PinDriver::output(peripherals.pins.gpio8.downgrade_output()).unwrap(),
        PinDriver::output(peripherals.pins.gpio9.downgrade_output()).unwrap(),
        PinDriver::output(peripherals.pins.gpio11.downgrade_output()).unwrap(),
    ];

    let column_pins = [
        PinDriver::input(peripherals.pins.gpio13.downgrade()).unwrap(),
        PinDriver::input(peripherals.pins.gpio15.downgrade()).unwrap(),
        PinDriver::input(peripherals.pins.gpio3.downgrade()).unwrap(),
        PinDriver::input(peripherals.pins.gpio4.downgrade()).unwrap(),
        PinDriver::input(peripherals.pins.gpio5.downgrade()).unwrap(),
        PinDriver::input(peripherals.pins.gpio6.downgrade()).unwrap(),
        PinDriver::input(peripherals.pins.gpio7.downgrade()).unwrap(),
    ];

    let mut keyboard = CardputerKeyboard::new(mux_pins, column_pins);
    keyboard.init();

    (display, keyboard)
}
