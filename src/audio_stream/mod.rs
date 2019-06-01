mod pulse_simple;

/// Trait indicating PCM audio data may be streamed via this type.
pub trait Stream: {
    /// Stream PCM data, filling the provided buffer.
    fn stream(&mut self, buffer: &mut [u8]);
}

/// Types of supported audio streams.
#[derive(PartialEq)]
pub enum Type {
    PulseSimple
}

/// Build an audio stream of the provided Type.
pub fn build_audio_stream(stream_type: Type) -> Option<Box<dyn Stream>> {
    if stream_type == Type::PulseSimple {
        return Option::Some(pulse_simple::build());
    }

    panic!("Invalid audio stream_type");
    return Option::None;
}
