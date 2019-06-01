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
