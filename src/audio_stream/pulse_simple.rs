use crate::audio_stream::Stream;

extern crate libpulse_binding as pulse;
extern crate libpulse_simple_binding as psimple;

use pulse::error::PAErr;
use pulse::sample::SAMPLE_FLOAT32;
use pulse::sample::Spec;
use psimple::Simple;
use pulse::stream::Direction;

const APP_NAME: &str = "sparkles";

/// Construct a Stream to stream audio data from the PulseAudio buffer.
pub fn build() -> Box<Stream> {
    // Connect to PulseAudio server
    let spec = Spec {
        format: SAMPLE_FLOAT32,
        channels: 2,
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

    return Box::new(stream);
}

impl Stream for Simple {
    fn stream(&mut self, buffer: &mut [u8]) {
        if let Err(PAErr(err)) = self.read(buffer) {
            dbg!(err);
            panic!("Failed to read from PulseAudio buffer");
        }
    }
}
