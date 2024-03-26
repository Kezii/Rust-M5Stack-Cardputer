use cardputer::{
    hal::cardputer_peripherals,
    terminal::FbTerminal,
    typing::{KeyboardEvent, Typing},
    SCREEN_HEIGHT, SCREEN_WIDTH,
};
use esp_idf_hal::{io::Write, peripherals};

const SAMPLE_RATE: f64 = 48000.0;
const FREQUENCY: f64 = 440.0;
const AMPLITUDE: f64 = 127.0;

fn generate_sine_wave(duration_secs: f64) -> Vec<u8> {
    let num_samples = (duration_secs * SAMPLE_RATE) as usize;
    let mut samples = Vec::with_capacity(num_samples);

    let sample_period = 1.0 / SAMPLE_RATE;

    for i in 0..num_samples {
        let t = i as f64 * sample_period;
        let angular_freq = 2.0 * 3.141593 * FREQUENCY + t * 200.0;
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
    let (mut display, mut keyboard, mut speaker) = cardputer_peripherals(
        peripherals.pins,
        peripherals.spi2,
        peripherals.ledc,
        peripherals.i2s0,
    );

    let mut raw_fb = Box::new([0u16; SCREEN_WIDTH * SCREEN_HEIGHT]);
    let mut terminal =
        FbTerminal::<SCREEN_WIDTH, SCREEN_HEIGHT>::new(raw_fb.as_mut_ptr(), &mut display);

    let mut typing = Typing::new();

    loop {
        let evt = keyboard.read_events();
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
                                terminal.println("Beep");
                                speaker.tx_enable().unwrap();
                                speaker
                                    .write_all(
                                        &generate_sine_wave(1.0),
                                        esp_idf_hal::delay::TickType::new_millis(2000).into(),
                                    )
                                    .unwrap();
                                speaker.flush().unwrap();
                                speaker.tx_disable().unwrap();
                            }
                            _ => {
                                terminal.println("?");
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
