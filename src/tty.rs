use std::io;

use std::io::{Write, Stdout};
use termion::input::MouseTerminal;
use termion::raw::{IntoRawMode, RawTerminal};
use termion::screen::AlternateScreen;
use termion::cursor::HideCursor;

const FULL_BLOCK_CHARACTER: char = '\u{2588}';
const WHITESPACE_CHARACTER: char = ' ';

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
        write!(self.stdout, "{}", termion::cursor::Hide).unwrap();
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
    pub height: u16,
}

impl Meter {
    /// Updates parameters of the meter and redraws it to the tty. This tries to be efficient and
    /// only draw or clear what it needs to display the new height. i.e. it doesn't redraw the
    /// entire Meter every time. This allows us to update the display without clearing the screen
    /// first which causes an annoying flicker effect.
    //TODO would scrolling down allow us to redraw everything on every frame without the flicker effect?
    //Ideally we could use the draw method above and pass in an array of Meters/some generic Drawable type
    //and just let it draw those. That would also make it much to do fancy color stuff (shifting gradients, etc).
    pub fn update_and_draw(&mut self, new_height: u16, tty: &mut Tty<AlternateScreen<HideCursor<MouseTerminal<RawTerminal<Stdout>>>>>) {
        //TODO cache these
        let draw_row = std::iter::repeat(FULL_BLOCK_CHARACTER).take(self.width as usize).collect::<String>();
        let clear_row = std::iter::repeat(WHITESPACE_CHARACTER).take(self.width as usize).collect::<String>();

        let y_start: u16;
        if new_height > self.height {
            // Add rows to extend the meter to new_height
            y_start = self.y - new_height;
            let height_diff = new_height - self.height;
            for y in y_start..(y_start + height_diff) {
                write!(tty.stdout, "{}", termion::cursor::Goto(self.x, y)).unwrap();
                write!(tty.stdout, "{}", draw_row).unwrap();
            }
        } else if new_height < self.height {
            // Clear rows to shorten the meter to new_height
            y_start = self.y - self.height;
            let height_diff = self.height - new_height;
            for y in y_start..(y_start + height_diff) {
                write!(tty.stdout, "{}", termion::cursor::Goto(self.x, y)).unwrap();
                write!(tty.stdout, "{}", clear_row).unwrap();
            }
        }

        self.height = new_height;
    }
}
