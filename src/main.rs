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
use crate::decode::{decode, decode_stereo};
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

    let buffer = &mut [0u8; (DEFAULT_SAMPLE_RATE / DEFAULT_FPS) as usize];
    let mut should_exit = false;

    let (terminal_size_x, terminal_size_y) = terminal_size().unwrap();
    let meter_width: u16 = 10;
    // Spacing between the two meters
    let meter_spacing: u16 = 5;
    // X-coordinate of the center of the terminal
    let terminal_center_x = terminal_size_x / 2;

    let mut l_meter: Meter = Meter {
        // x-coordinate is the leftmost position of the meter. We want the rightmost position of
        // this meter to be shifted to the left by meter_spacing / 2 from the center of the terminal.
        x: terminal_center_x - (meter_width + (meter_spacing / 2)),
        y: terminal_size_y,
        width: meter_width,
        height: 0,
    };
    let mut r_meter: Meter = Meter {
        // x-coordinate is the leftmost position of the meter. We want the leftmost position of
        // this meter to be shifted to the right by meter_spacing / 2 from the center of the terminal.
        x: terminal_center_x + (meter_spacing / 2),
        y: terminal_size_y,
        width: meter_width,
        height: 0,
    };



    // Start streaming the audio buffer and visualizing it
    while !should_exit {
        // Read from audio buffer.
        stream.stream(buffer);

        let mut samples = decode_stereo(buffer);

        // FIXME is it ok to declare new floats every time?
        let (l_amp, r_amp) = analyze::rms_amplitude_stereo(&mut samples);

        let l_decibel: f32;
        if l_amp <= 0f32 {
            l_decibel = MIN_DECIBEL_LEVEL;
        } else {
            l_decibel = 20f32 * l_amp.log10();
        }

        let r_decibel: f32;
        if r_amp <= 0f32 {
            r_decibel = MIN_DECIBEL_LEVEL;
        } else {
            r_decibel = 20f32 * r_amp.log10();
        }

        let mut l_meter_height = ((l_decibel + MIN_DECIBEL_MAGNITUDE) / MIN_DECIBEL_MAGNITUDE) * terminal_size_y as f32;
        // fp precision errors (?) can lead this to be negative, leading to an overflow when converting to u16 below
        if l_meter_height < 0f32 {
            l_meter_height = 0f32;
        }
        let mut r_meter_height = ((r_decibel + MIN_DECIBEL_MAGNITUDE) / MIN_DECIBEL_MAGNITUDE) * terminal_size_y as f32;
        // fp precision errors (?) can lead this to be negative, leading to an overflow when converting to u16 below
        if r_meter_height < 0f32 {
            r_meter_height = 0f32;
        }

        l_meter.update_and_draw(l_meter_height as u16, &mut writer);
        r_meter.update_and_draw(r_meter_height as u16, &mut writer);
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
