extern crate libpulse_binding as pulse;
extern crate libpulse_simple_binding as psimple;
mod util;

use std::io;

use psimple::Simple;
use pulse::stream::Direction;
use pulse::error::PAErr;
use std::io::stdout;
use termion::event::Key;
use termion::input::MouseTerminal;
use termion::raw::IntoRawMode;
use termion::screen::AlternateScreen;
use tui::backend::TermionBackend;
use tui::Terminal;
use tui::layout::{Layout, Constraint};
use tui::widgets::{BarChart, Block, Borders, Widget, Gauge};
use tui::style::{Style, Color};

use crate::util::event::{Event, Events};

const APP_NAME: &str = "sparkles";
const CHUNK_SIZE: usize = 128;

struct App {
    data: u16
}

impl App {
    fn new() -> App {
        App {
            data: 0
        }
    }

    fn update(&mut self, value: u16) {
        if (value > 100) {
            self.data = 100;
        } else {
            self.data = value;
        }
    }
}

fn main() {
    // Connect to PulseAudio server
    let spec = pulse::sample::Spec {
        format: pulse::sample::SAMPLE_FLOAT32,
        channels: 1,
        rate: 48000
    };
    assert!(spec.is_valid());

    let stream = Simple::new(
        None,               // Use default server
        APP_NAME,
        Direction::Record,
        None,                // Use default device
        "visualizer",
        &spec,
        None,               // Use default channel map
        None                // Use default buffering attributes
    ).unwrap();

    // Initialize UI
    let stdout = io::stdout().into_raw_mode().unwrap();
    let stdout = MouseTerminal::from(stdout);
    let stdout = AlternateScreen::from(stdout);
    let backend = TermionBackend::new(stdout);
    let mut terminal = Terminal::new(backend).unwrap();
    terminal.hide_cursor();

    let mut app = App::new();

    // Setup event handlers
    let events = Events::new();

    // Start streaming the audio buffer and updating UI
    let mut average_volume;
    let buffer = &mut [0u8; CHUNK_SIZE];
    let mut should_exit = false;

    while !should_exit {
        // Read from PA buffer.
        if let Err(PAErr(err)) = stream.read(buffer) {
            dbg!(err);
            break;
        }

        average_volume = compute_average_volume(buffer);

        terminal.draw(|mut f| {
            let chunks = Layout::default()
                .direction(tui::layout::Direction::Vertical)
                .margin(2)
                .constraints([Constraint::Percentage(100)].as_ref())
                .split(f.size());
            Gauge::default()
                .block(Block::default().title(APP_NAME).borders(Borders::ALL))
                .style(Style::default().fg(Color::Yellow))
                .percent(app.data)
                .render(&mut f, chunks[0]);

            match events.next().unwrap() {
                Event::Input(input) => {
                    if input == Key::Char('q') {
                        should_exit = true;
                    }
                }
                Event::Tick => {
                    app.update(average_volume as u16)
                }
            }
        });
    }
}

fn compute_average_volume(buffer: &mut [u8]) -> i32 {
    let mut sum = 0;
    for (_, elem) in buffer.iter().enumerate() {
        sum += *elem as i32;
    }

    return sum / buffer.len() as i32;
}
