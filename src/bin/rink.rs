use cardputer::{
    hal::cardputer_peripherals,
    terminal::FbTerminal,
    typing::{KeyboardEvent, Typing},
    SCREEN_HEIGHT, SCREEN_WIDTH,
};

use esp_idf_hal::peripherals;

#[allow(clippy::approx_constant)]
fn main() {
    esp_idf_svc::sys::link_patches();

    esp_idf_svc::log::EspLogger::initialize_default();

    let peripherals = peripherals::Peripherals::take().unwrap();

    let (mut display, mut keyboard) =
        cardputer_peripherals(peripherals.pins, peripherals.spi2, peripherals.ledc);

    let mut raw_fb = Box::new([0u16; SCREEN_WIDTH * SCREEN_HEIGHT]);
    let mut terminal =
        FbTerminal::<SCREEN_WIDTH, SCREEN_HEIGHT>::new(raw_fb.as_mut_ptr(), &mut display);

    let mut typing = Typing::new();

    let mut ctx = simple_context_().unwrap();

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
                        let res = execute_command(terminal.command_line.get(), &mut ctx);
                        terminal.enter();
                        terminal.println(&res);
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
