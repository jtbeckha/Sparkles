mod tty;

#[macro_use] extern crate log;
extern crate simplelog;
extern crate libpulse_binding as pulse;
extern crate libpulse_simple_binding as psimple;

use byteorder::{ByteOrder, NativeEndian};
use psimple::Simple;
use pulse::stream::Direction;
use pulse::error::PAErr;
use std::io::{Read, Write};
use termion::{async_stdin, terminal_size};

use tty::Tty;
use tty::Meter;
use std::collections::VecDeque;

use simplelog::*;

use std::fs::File;
use pulse::sample::SAMPLE_FLOAT32;

const APP_NAME: &str = "sparkles";
const DEFAULT_SAMPLE_RATE: u16 = 48000;
const DEFAULT_FPS: u16 = 30;
// Log10(0) is -inf (or undefined) so set a reasonable min decibel level
const MIN_DECIBEL_LEVEL: f32 = -30f32;
const MIN_DECIBEL_MAGNITUDE: f32 = 30f32;

fn main() {
    CombinedLogger::init(
        vec![
            WriteLogger::new(LevelFilter::Info, Config::default(), File::create("sparkles.log").unwrap()),
        ]
    ).unwrap();

    // Connect to PulseAudio server
    let spec = pulse::sample::Spec {
        format: SAMPLE_FLOAT32,
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
    let mut amp;
    let buffer = &mut [0u8; (DEFAULT_SAMPLE_RATE / DEFAULT_FPS) as usize];
    let mut should_exit = false;

    while !should_exit {
        // Read from PA buffer.
        if let Err(PAErr(err)) = stream.read(buffer) {
            dbg!(err);
            break;
        }

        amp = compute_rms_amplitude(buffer);
        let decibel: f32;
        if amp == 0f32 {
            decibel = MIN_DECIBEL_LEVEL;
        } else {
            decibel = 20f32 * amp.log10();
        }

        let (terminal_size_x, terminal_size_y) = terminal_size().unwrap();

        let mut meter_height = ((decibel + MIN_DECIBEL_MAGNITUDE) / MIN_DECIBEL_MAGNITUDE) * terminal_size_y as f32;
        // fp precision errors (?) can lead this to be negative, leading to an overflow when converting to u16 below
        if meter_height < 0f32 {
            meter_height = 0f32;
        }
        let meter: Meter = Meter {
            x: terminal_size_x / 2,
            y: terminal_size_y,
            width: 10,
            height: meter_height as u16,
        };
        // Clear out any previous renderings to ensure they don't overlap
        writer.clear();
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

    writer.stdout.flush().ok();
}

fn compute_rms_amplitude(buffer: &mut [u8]) -> f32 {
    let decoded = decode(buffer);

    let mut square_sum = 0f32;
    for (_, elem) in decoded.iter().enumerate() {
        square_sum = square_sum + elem.powf(2f32);
    }

    let rms = (square_sum / decoded.len() as f32).sqrt();
    return rms
}

/// Decode audio stream data to an f32 vector.
pub fn decode(input: &[u8]) -> Vec<f32> {
    let mut output = Vec::with_capacity(input.len() / 4);
    for chunk in input.chunks_exact(4) {
        assert_eq!(4, chunk.len());
        let sample = NativeEndian::read_f32(&chunk);
        output.push(sample);
    }
    return output;
}
