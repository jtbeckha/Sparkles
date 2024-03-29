use byteorder::{ByteOrder, NativeEndian};

const BYTES_PER_SAMPLE: usize = 4;

/// A frame of stereo audio data containing an f32 PCM sample for the left and right channel.
#[derive(PartialEq, Debug)]
pub struct Frame {
    pub left: f32,
    pub right: f32
}

/// Decode audio stream data into a vector of f32 PCM samples.
pub fn decode(input: &[u8]) -> Vec<f32> {
    // TODO make sure this will always be true (i.e. input should be constrained to a number divisible by BYTES_PER_SAMPLE)
    // so that we never mix up stereo channels which are interleaved in the resulting sample Vec.
    let vec_capacity = input.len() / BYTES_PER_SAMPLE;
    assert_eq!(0, input.len() / BYTES_PER_SAMPLE % 2);
    let mut output = Vec::with_capacity(vec_capacity);
    for chunk in input.chunks_exact(BYTES_PER_SAMPLE) {
        assert_eq!(BYTES_PER_SAMPLE, chunk.len());
        let sample = NativeEndian::read_f32(&chunk);
        output.push(sample);
    }
    return output;
}

/// Decode audio stream data into a vector of Frames.
pub fn decode_stereo(input: &[u8]) -> Vec<Frame> {
    // TODO make sure this will always be true (i.e. input should be constrained to a number divisible by BYTES_PER_SAMPLE)
    // so that we never mix up stereo channels which are interleaved in the resulting sample Vec.
    let vec_capacity = input.len() / BYTES_PER_SAMPLE;
    assert_eq!(0, input.len() / BYTES_PER_SAMPLE % 2);
    let mut output = Vec::with_capacity(vec_capacity);
    for chunk in input.chunks_exact(BYTES_PER_SAMPLE * 2) {
        assert_eq!(BYTES_PER_SAMPLE * 2, chunk.len());
        let (left_bytes, right_bytes) = chunk.split_at(BYTES_PER_SAMPLE);

        let left = NativeEndian::read_f32(left_bytes);
        let right = NativeEndian::read_f32(right_bytes);
        output.push(Frame {
            left, right
        });
    }
    return output;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_decode_stereo() {
        let input: [u8; 16] = [0x80, 0x80, 0x80, 0x80, 0x40, 0x40, 0x40, 0x40, 0x20, 0x20, 0x20, 0x20, 0x10, 0x10, 0x10, 0x10];

        let expected: Vec<Frame> = vec![
            Frame { left: -1.180104E-38, right: 3.0039215 },
            Frame { left: 1.3563156E-19, right: 2.8411367E-29 }
        ];

        let actual = decode_stereo(&input);

        assert_eq!(expected, actual);
    }
}
