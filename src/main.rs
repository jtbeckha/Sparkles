mod analyze;
mod audio_stream;
mod decode;
mod tty;

#[macro_use] extern crate log;
extern crate simplelog;

use byteorder::{ByteOrder, NativeEndian};
use simplelog::{CombinedLogger, LevelFilter, Config, WriteLogger};
use std::fs::File;
use std::io::{Read, Write};
use termion::{async_stdin, terminal_size};

use crate::analyze::rms_amplitude;
use crate::audio_stream::Type;
use crate::decode::decode;
use crate::tty::Tty;
use crate::tty::Meter;

const DEFAULT_SAMPLE_RATE: u16 = 48000;
const DEFAULT_FPS: u16 = 20;
// Log10(0) is -inf (or undefined) so set a reasonable min decibel level
const MIN_DECIBEL_LEVEL: f32 = -30f32;
const MIN_DECIBEL_MAGNITUDE: f32 = 30f32;

fn main() {
    // Initialize logging
    CombinedLogger::init(
        vec![
            WriteLogger::new(LevelFilter::Info, Config::default(), File::create("sparkles.log").unwrap()),
        ]
    ).unwrap();

    // Establish audio stream
    let mut stream = match audio_stream::build_audio_stream(Type::PulseSimple) {
        Some(stream) => stream,
        None => panic!("Unable to build audio_stream")
    };

    // Initialize UI
    let mut writer = Tty::init();
    let mut stdin = async_stdin().bytes();
    writer.clear();

    let mut amp;
    let buffer = &mut [0u8; (DEFAULT_SAMPLE_RATE / DEFAULT_FPS) as usize];
    let mut should_exit = false;

    let (terminal_size_x, terminal_size_y) = terminal_size().unwrap();
    let mut meter: Meter = Meter {
        x: terminal_size_x / 2,
        y: terminal_size_y,
        width: 10,
        height: 0,
    };

    // Start streaming the audio buffer and visualizing it
    while !should_exit {
        // Read from audio buffer.
        stream.stream(buffer);

        let mut samples = decode::decode(buffer);

        amp = analyze::rms_amplitude(&mut samples);

        let decibel: f32;
        if amp <= 0f32 {
            decibel = MIN_DECIBEL_LEVEL;
        } else {
            decibel = 20f32 * amp.log10();
        }

        let mut meter_height = ((decibel + MIN_DECIBEL_MAGNITUDE) / MIN_DECIBEL_MAGNITUDE) * terminal_size_y as f32;
        // fp precision errors (?) can lead this to be negative, leading to an overflow when converting to u16 below
        if meter_height < 0f32 {
            meter_height = 0f32;
        }
        meter.update_and_draw(meter_height as u16, &mut writer);
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
