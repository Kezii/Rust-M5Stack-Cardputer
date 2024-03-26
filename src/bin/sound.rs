use std::f32::consts::PI;

use cardputer::{
    hal::cardputer_peripherals,
    terminal::FbTerminal,
    typing::{KeyboardEvent, Typing},
    SCREEN_HEIGHT, SCREEN_WIDTH,
};
use esp_idf_hal::{io::Write, peripherals};

const SAMPLE_RATE: f32 = 48000.0;
const FREQUENCY: f32 = 440.0;
const AMPLITUDE: f32 = 127.0;

fn generate_sine_wave(duration_secs: f32) -> Vec<u8> {
    let num_samples = (duration_secs * SAMPLE_RATE) as usize;
    let mut samples = Vec::with_capacity(num_samples);

    let sample_period = 1.0 / SAMPLE_RATE;

    for i in 0..num_samples {
        let t = i as f32 * sample_period;
        let angular_freq = 2.0 * PI * FREQUENCY + t * 200.0;
        let sample_value = (AMPLITUDE * (angular_freq * t).sin()) as u8;
        samples.push(sample_value);
    }

    samples
}

#[allow(clippy::approx_constant)]
fn main() {
    esp_idf_svc::sys::link_patches();

    // esp_idf_hal::i2s::I2sDriver::new_std_tx(i2s, config, bclk, dout, mclk, ws)
    let peripherals = peripherals::Peripherals::take().unwrap();
    let mut p = cardputer_peripherals(
        peripherals.pins,
        peripherals.spi2,
        peripherals.ledc,
        peripherals.i2s0,
    );

    let mut raw_fb = Box::new([0u16; SCREEN_WIDTH * SCREEN_HEIGHT]);
    let mut terminal =
        FbTerminal::<SCREEN_WIDTH, SCREEN_HEIGHT>::new(raw_fb.as_mut_ptr(), &mut p.display);

    let mut typing = Typing::new();

    loop {
        let evt = p.keyboard.read_events();
        if let Some(evt) = evt {
            if let Some(evts) = typing.eat_keyboard_events(evt) {
                match evts {
                    KeyboardEvent::Ascii(c) => {
                        terminal.command_line.push(c);
                    }
                    KeyboardEvent::Backspace => {
                        terminal.command_line.pop();
                    }
                    KeyboardEvent::Enter => {
                        terminal.enter();

                        let text = terminal.command_line.get();

                        match text {
                            "b" => {
                                terminal.println("Beep");
                                p.speaker.tx_enable().unwrap();
                                p.speaker
                                    .write_all(
                                        &generate_sine_wave(1.0),
                                        esp_idf_hal::delay::TickType::new_millis(2000).into(),
                                    )
                                    .unwrap();
                                p.speaker.flush().unwrap();
                                p.speaker.tx_disable().unwrap();
                            }
                            _ => {
                                terminal.println("?");
                            }
                        }
                    }
                    KeyboardEvent::ArrowUp => {
                        terminal.command_line.arrow_up();
                    }
                    _ => {}
                }
            }
        }

        terminal.draw();
    }
}
