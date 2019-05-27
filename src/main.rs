mod tty;

extern crate libpulse_binding as pulse;
extern crate libpulse_simple_binding as psimple;

use psimple::Simple;
use pulse::stream::Direction;
use pulse::error::PAErr;
use std::io::{Read, Write};
use termion::{async_stdin, terminal_size};

use tty::Tty;
use tty::Meter;

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
    let mut writer = Tty::init();
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

        writer.clear();
        let terminal_size = terminal_size().unwrap();

        let meter: Meter = Meter {
            x: terminal_size.0 / 2,
            y: terminal_size.1,
            width: 10,
            height: 10,
        };
        writer.draw(meter);
        writer.stdout.flush().unwrap();

        loop {
            let b = stdin.next();
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
