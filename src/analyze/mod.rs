use crate::decode::Frame;

///! The analyze module contains functions for analysis of PCM data to convert it into something
///! interesting to visualize e.g. RMS amplitude, frequency spectrum, etc.

/// Compute the RMS amplitude of samples.
pub fn rms_amplitude(samples: &mut Vec<f32>) -> f32 {
    let mut square_sum = 0f32;
    for (_, elem) in samples.iter().enumerate() {
        square_sum = square_sum + elem.powf(2f32);
    }

    let rms = (square_sum / samples.len() as f32).sqrt();
    return rms
}

/// Compute RMS amplitude of a set of stereo samples and returns the value for the left and
/// right channels, respectively.
pub fn rms_amplitude_stereo(samples: &mut Vec<Frame>) -> (f32, f32) {
    let mut left_square_sum = 0f32;
    let mut right_square_sum = 0f32;
    for (_, elem) in samples.iter().enumerate() {
        left_square_sum = left_square_sum + elem.left.powf(2f32);
        right_square_sum = right_square_sum + elem.right.powf(2f32);
    }

    let left_rms = (left_square_sum / samples.len() as f32).sqrt();
    let right_rms = (right_square_sum / samples.len() as f32).sqrt();

    return (left_rms, right_rms);
}
