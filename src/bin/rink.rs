use cardputer::{
    hal::cardputer_peripherals,
    terminal::Terminal,
    typing::{KeyboardEvent, Typing},
    SCREEN_HEIGHT, SCREEN_WIDTH,
};

use esp_idf_hal::peripherals;
use log::info;

#[allow(clippy::approx_constant)]
fn main() {
    esp_idf_svc::sys::link_patches();

    esp_idf_svc::log::EspLogger::initialize_default();

    let peripherals = peripherals::Peripherals::take().unwrap();

    let (mut display, mut keyboard) =
        cardputer_peripherals(peripherals.pins, peripherals.spi2, peripherals.ledc);

    let mut raw_fb = Box::new([0u16; SCREEN_WIDTH * SCREEN_HEIGHT]);
    let mut terminal = Terminal::<SCREEN_WIDTH, SCREEN_HEIGHT>::new(raw_fb.as_mut_ptr());

    let mut typing = Typing::new();

    let mut command_line = String::new();
    let mut previous_command_line = String::new();
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
                        terminal.push_line(&format!("> {}", command_line));
                        let res = execute_command(command_line.as_str(), &mut ctx);
                        previous_command_line = command_line.clone();
                        command_line.clear();

                        terminal.push_line(&res);
                    }
                    KeyboardEvent::ArrowUp => {
                        command_line = previous_command_line.clone();
                    }
                    _ => {}
                }
            }
        }

        display
            .eat_framebuffer(terminal.print(&command_line))
            .unwrap();
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
