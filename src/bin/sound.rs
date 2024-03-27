use std::f32::consts::PI;

use cardputer::{
    hal::cardputer_peripherals,
    terminal::FbTerminal,
    typing::{KeyboardEvent, Typing},
    SCREEN_HEIGHT, SCREEN_WIDTH,
};
use esp_idf_hal::{io::Write, peripherals};

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

    // Enable the speaker,
    // TODO: is there reason to not do this in hal.rs?
    p.speaker.tx_enable().unwrap();

    let wav = generate_sine_wave(1.0, 880.0);

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
                        let text = terminal.command_line.get();

                        match text {
                            "b" => {
                                p.speaker
                                    .write_all(
                                        &wav,
                                        esp_idf_hal::delay::TickType::new_millis(100).into(),
                                    )
                                    .unwrap();
                            }
                            _ => {
                                terminal.println("Commands: b to Beep");
                            }
                        }

                        terminal.enter();
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

fn generate_sine_wave(duration_secs: f32, frequency: f32) -> Vec<u8> {
    const SAMPLE_RATE: f32 = 48000.0;
    const AMPLITUDE: f32 = 127.0;

    let num_samples = (duration_secs * SAMPLE_RATE) as usize;
    let mut samples = Vec::with_capacity(num_samples);

    let sample_period = 1.0 / SAMPLE_RATE;

    for i in 0..num_samples {
        let t = i as f32 * sample_period;
        let angular_freq = 2.0 * PI * frequency;
        let sample_value = (AMPLITUDE * (angular_freq * t).sin()) as u8;
        samples.push(sample_value);
    }

    samples
}
