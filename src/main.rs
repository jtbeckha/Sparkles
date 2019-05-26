extern crate libpulse_binding as pulse;
extern crate libpulse_simple_binding as psimple;

use std::io;

use psimple::Simple;
use pulse::stream::Direction;
use pulse::error::PAErr;
use std::io::{stdout, stdin, Read, Write};
use termion::event::{Event, Key};
use termion::input::{MouseTerminal, TermRead};
use termion::raw::IntoRawMode;
use termion::screen::AlternateScreen;
use termion::async_stdin;

const APP_NAME: &str = "sparkles";
const DEFAULT_SAMPLE_RATE: u16 = 48000;
const DEFAULT_FPS: u16 = 60;

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
    let mut stdout = AlternateScreen::from(stdout);

    let mut stdin = async_stdin().bytes();

    // Start streaming the audio buffer and updating UI
    let mut average_volume;
    let buffer = &mut [0u8; (DEFAULT_SAMPLE_RATE / DEFAULT_FPS) as usize];
    let mut should_exit = false;

    while !should_exit {
        // Read from PA buffer.
        if let Err(PAErr(err)) = stream.read(buffer) {
            dbg!(err);
            break;
        }

        average_volume = compute_average_volume(buffer);

        write!(stdout, "{}\r\n", average_volume);
        stdout.flush();

        while let b = stdin.next() {
            if b.is_none() {
                break;
            }

            if let Some(Ok(b'q')) = b {
                should_exit = true;
            }
        }
    }
}

fn compute_average_volume(buffer: &mut [u8]) -> i32 {
    let mut sum = 0;
    for (_, elem) in buffer.iter().enumerate() {
        sum += *elem as i32;
    }

    return sum / buffer.len() as i32;
}
