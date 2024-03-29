use std::ffi::c_void;

use embedded_gfx::framebuffer::DmaReadyFramebuffer;
use embedded_graphics::Drawable;
use embedded_graphics::{
    draw_target::DrawTarget,
    geometry::Point,
    mono_font::{ascii::FONT_8X13, MonoTextStyle},
    pixelcolor::{Rgb565, Rgb888},
    primitives::{Line, Primitive, PrimitiveStyle},
    text::Text,
};

use crate::display_driver::FramebufferTarget;

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

    fn print(&self, fbuf: &mut impl DrawTarget<Color = Rgb565>) {
        for (i, row) in self.rows.iter().enumerate() {
            let _ = Text::new(
                row,
                Point::new(3, 10 + i as i32 * 13),
                MonoTextStyle::new(&FONT_8X13, Rgb565::new(252, 252, 252)),
            )
            .draw(fbuf);
        }
    }
}

pub struct CommandLine {
    line: String,
    previous: String,
}

impl CommandLine {
    fn new() -> Self {
        Self {
            line: String::new(),
            previous: String::new(),
        }
    }

    pub fn push(&mut self, c: char) {
        self.line.push(c);
    }

    pub fn pop(&mut self) {
        self.line.pop();
    }

    fn enter(&mut self) {
        self.previous = self.line.clone();
        self.line.clear();
    }

    pub fn get(&self) -> &str {
        &self.line
    }

    pub fn arrow_up(&mut self) {
        self.line = self.previous.clone();
    }
}

pub struct FbTerminal<'a, const W: usize, const H: usize> {
    fbuf: DmaReadyFramebuffer<W, H>,
    rows: TerminalRows,
    display: &'a mut dyn FramebufferTarget,
    pub command_line: CommandLine,
    /// draw to framebuffer after push_line
    auto_draw: bool,
}

impl<'a, const W: usize, const H: usize> FbTerminal<'a, W, H> {
    pub fn new(fb: *mut u16, display: &'a mut impl FramebufferTarget) -> FbTerminal<'a, W, H> {
        let fbuf = DmaReadyFramebuffer::<W, H>::new(fb as *mut c_void, true);

        let rows = TerminalRows::new();

        FbTerminal {
            fbuf,
            rows,
            display,
            command_line: CommandLine::new(),
            auto_draw: false,
        }
    }

    pub fn auto_draw(&mut self, auto: bool) {
        self.auto_draw = auto;
    }

    pub fn println(&mut self, res: &str) {
        let max_width = 28;
        let mut line = String::new();
        for c in res.chars() {
            if line.len() > max_width {
                self.rows.push(line);
                line = String::new();
            }
            line.push(c);
        }
        self.rows.push(line);

        if self.auto_draw {
            self.draw();
        }
    }

    pub fn enter(&mut self) {
        self.println(&format!("> {}", self.command_line.line));
        self.command_line.enter();
    }

    pub fn draw(&mut self) {
        let text_style = MonoTextStyle::new(&FONT_8X13, Rgb565::new(252, 252, 252));
        self.fbuf.clear(Rgb565::new(0, 1, 0)).unwrap();

        Line::new(
            Point::new(0, H as i32 - 18),
            Point::new(W as i32, H as i32 - 18),
        )
        .into_styled(PrimitiveStyle::with_stroke(
            Rgb888::new(77 >> 3, 85 >> 2, 94 >> 3).into(),
            1,
        ))
        .draw(&mut self.fbuf)
        .unwrap();

        Text::new(
            &format!("> {}", self.command_line.get()),
            Point::new(3, H as i32 - 5),
            text_style,
        )
        .draw(&mut self.fbuf)
        .unwrap();

        self.rows.print(&mut self.fbuf);

        self.display.eat_framebuffer(self.fbuf.as_slice()).unwrap();
    }
}
