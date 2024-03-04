use std::ffi::c_void;

use cardputer::{
    hal::cardputer_peripherals,
    typing::{KeyboardEvent, Typing},
};
use embedded_gfx::framebuffer::DmaReadyFramebuffer;
use embedded_graphics::{
    draw_target::DrawTarget,
    geometry::Point,
    mono_font::{ascii::FONT_8X13, MonoTextStyle},
    pixelcolor::{Rgb565, Rgb888},
    primitives::{Line, Primitive, PrimitiveStyle},
    text::Text,
    Drawable,
};
use esp_idf_hal::peripherals;
use log::info;

const WIDTH: usize = 240;
const HEIGHT: usize = 135;

struct TerminalRows {
    rows: [String; 9],
}

impl TerminalRows {
    fn new() -> Self {
        Self {
            rows: Default::default(),
        }
    }

    fn push(&mut self, line: String) {
        for i in 0..self.rows.len() - 1 {
            self.rows[i] = self.rows[i + 1].clone();
        }
        self.rows[self.rows.len() - 1] = line;
    }

    fn print(&self, fbuf: &mut DmaReadyFramebuffer<WIDTH, HEIGHT>) {
        for (i, row) in self.rows.iter().enumerate() {
            Text::new(
                row,
                Point::new(3, 10 + i as i32 * 13),
                MonoTextStyle::new(&FONT_8X13, Rgb565::new(252, 252, 252)),
            )
            .draw(fbuf)
            .unwrap();
        }
    }
}

#[allow(clippy::approx_constant)]
fn main() {
    esp_idf_svc::sys::link_patches();

    esp_idf_svc::log::EspLogger::initialize_default();

    let peripherals = peripherals::Peripherals::take().unwrap();

    let (mut display, mut keyboard) = cardputer_peripherals(peripherals);

    let mut raw_framebuffer_0 = Box::new([0u16; WIDTH * HEIGHT]);

    let mut fbuf = DmaReadyFramebuffer::<WIDTH, HEIGHT>::new(
        raw_framebuffer_0.as_mut_ptr() as *mut c_void,
        true,
    );

    let text_style = MonoTextStyle::new(&FONT_8X13, Rgb565::new(252, 252, 252));
    let mut typing = Typing::new();

    let mut command_line = String::new();
    let mut previous_command_line = String::new();
    let mut rows = TerminalRows::new();
    let mut ctx = simple_context_().unwrap();

    loop {
        let evt = keyboard.read_events();
        if let Some(evt) = evt {
            if let Some(evts) = typing.eat_keyboard_events(evt) {
                match evts {
                    KeyboardEvent::Ascii(c) => {
                        command_line.push(c);
                    }
                    KeyboardEvent::Backspace => {
                        command_line.pop();
                    }
                    KeyboardEvent::Enter => {
                        info!("Command: {}", command_line);
                        rows.push(format!("> {}", command_line));
                        let res = execute_command(command_line.as_str(), &mut ctx);
                        previous_command_line = command_line.clone();
                        command_line.clear();

                        let max_width = 28;
                        let mut line = String::new();
                        for c in res.chars() {
                            if line.len() > max_width {
                                rows.push(line);
                                line = String::new();
                            }
                            line.push(c);
                        }
                        rows.push(line);
                    }
                    KeyboardEvent::ArrowUp => {
                        command_line = previous_command_line.clone();
                    }
                    _ => {}
                }
            }
        }

        Line::new(
            Point::new(0, HEIGHT as i32 - 18),
            Point::new(WIDTH as i32, HEIGHT as i32 - 18),
        )
        .into_styled(PrimitiveStyle::with_stroke(
            Rgb888::new(77 >> 3, 85 >> 2, 94 >> 3).into(),
            1,
        ))
        .draw(&mut fbuf)
        .unwrap();
        Text::new(
            &format!("> {}", command_line),
            Point::new(3, HEIGHT as i32 - 5),
            text_style,
        )
        .draw(&mut fbuf)
        .unwrap();

        rows.print(&mut fbuf);

        display.eat_framebuffer(fbuf.as_slice()).unwrap();

        //let bg = rgb565::Rgb565::from_rgb888_components(35,38,39);
        //let bg = bg.to_rgb565_components();

        fbuf.clear(Rgb565::new(0, 1, 0)).unwrap();
    }
}

fn execute_command(command: &str, ctx: &mut rink_core::Context) -> String {
    use rink_core::*;

    //let mut ctx = Context::new();

    let result = one_line(ctx, command);

    match result {
        Ok(r) => r,
        Err(r) => r,
    }
}

pub fn simple_context_() -> Result<rink_core::Context, String> {
    use rink_core::*;

    use rink_core::loader::gnu_units;

    let units = include_str!("definitions.units");

    let mut iter = gnu_units::TokenIterator::new(units).peekable();
    let units = gnu_units::parse(&mut iter);

    //let dates = parsing::datetime::parse_datefile(DATES_FILE);

    let mut ctx = Context::new();
    ctx.load(units)?;
    //ctx.load_dates(dates);

    Ok(ctx)
}
