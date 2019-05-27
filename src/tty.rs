use std::io;

use std::io::{Write, Stdout};
use termion::input::MouseTerminal;
use termion::raw::{IntoRawMode, RawTerminal};
use termion::screen::AlternateScreen;
use termion::cursor::HideCursor;

const FULL_BLOCK_CHARACTER: char = '\u{2588}';

pub struct Tty<W: Write> {
    /// The output target.
    pub stdout: W
}

// FIXME how do I generics?
impl Tty<AlternateScreen<HideCursor<MouseTerminal<RawTerminal<Stdout>>>>> {
    pub fn init() -> Self {
        let stdout = io::stdout().into_raw_mode().unwrap();
        let stdout = MouseTerminal::from(stdout);
        let stdout = HideCursor::from(stdout);
        return Tty { stdout: AlternateScreen::from(stdout) };
    }

    /// Clear the screen and reset cursor position.
    pub fn clear(&mut self) {
        write!(self.stdout, "{}", termion::clear::All).unwrap();
        write!(self.stdout, "{}", termion::cursor::Goto(1, 1)).unwrap();
        self.stdout.flush().unwrap();
    }

    pub fn draw(&mut self, meter: Meter) {
        //TODO cache this
        let row = std::iter::repeat(FULL_BLOCK_CHARACTER).take(meter.width as usize).collect::<String>();

        let y_start: u16;
        if meter.height >= meter.y {
            y_start = 1;
        } else {
            y_start = meter.y - meter.height;
        }

        for y in y_start..y_start + meter.height {
            write!(self.stdout, "{}", termion::cursor::Goto(meter.x, y)).unwrap();
            write!(self.stdout, "{}", row).unwrap();
        }
    }
}

/// Represents a drawable vertical meter
pub struct Meter {
    /// Left-most horizontal position of the meter
    pub x: u16,
    /// Bottom-most vertical position of the meter
    pub y: u16,
    /// Horizontal size of the meter
    pub width: u16,
    /// Vertical size of the meter
    pub height: u16
}
