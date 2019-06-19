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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rms_amplitude_stereo() {
        let mut samples: Vec<Frame> = vec![
            Frame { left: 2.0, right: 4.0 },
            Frame { left: 4.0, right: 8.0 }
        ];

        let (left_rms, right_rms) = rms_amplitude_stereo(&mut samples);

        // sqrt((2^2 + 4^2) / 2)
        assert_eq!((10.0 as f32).sqrt(), left_rms);
        // sqrt((4^2 + 8^2) / 2)
        assert_eq!((40.0 as f32).sqrt(), right_rms);
    }
}
