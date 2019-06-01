use byteorder::{ByteOrder, NativeEndian};

const BYTES_PER_SAMPLE: usize = 4;

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
