use crate::guitar::{get_note_frequency, Note};
use anyhow::{anyhow, Ok, Result};
use std::f64::consts::PI;
use std::fs::File;
use std::io::Write;

const MAX_FREQUENCY: f64 = 1325.0;

// Using auto-correlation method
// This function does the following:
// 1. Calculate the normalize square difference of the samples
// 2. Extracts peaks from the graph
// 3. Pick to correct peak
// 4. Infer the closest note from the peak
pub fn detect_note(
    samples: &[f64],
    sampling_rate: usize,
    tuning_notes: &[Note],
) -> Result<(Note, f64)> {
    // Guitar notes have range of 75Hz - 1320Hz (accounted for overtones)
    // Filter out frequencies that aren't in this range
    // For now, I just know how to implements a low-pass filter
    // TODO: filter <75Hz range
    let filter = low_pass_filter(MAX_FREQUENCY / sampling_rate as f64, 256);
    let samples = apply_filter(&samples, &filter);

    let nsd: Vec<f64> = normalized_square_difference(&samples);
    let frequency: f64 = infer_fundamental_frequency(&nsd, sampling_rate)?;

    for harmonic_degree in 1..=5 {
        let harmonic_frequency = frequency / harmonic_degree as f64;
        if let Some(note) = infer_note(harmonic_frequency, tuning_notes) {
            return Ok((note, harmonic_frequency));
        }
    }

    Err(anyhow!("Fail to detect note"))
}

fn normalized_square_difference(samples: &[f64]) -> Vec<f64> {
    (0..samples.len())
        .map(|lag| {
            let numerator: f64 = (0..(samples.len() - lag))
                .map(|i| samples[i] * samples[i + lag])
                .sum();

            let denominator: f64 = (0..(samples.len() - lag))
                .map(|i| samples[i].powi(2) + samples[i + lag].powi(2))
                .sum();

            2.0 * numerator / denominator
        })
        .collect()
}

enum YAxis {
    Positive,
    Negative,
}

#[derive(Debug, Clone)]
struct KeyMaxima {
    index: usize,
    max: f64,
    left_neighbor: Option<f64>,
    right_neighbor: Option<f64>,
}

fn infer_fundamental_frequency(samples: &[f64], sampling_rate: usize) -> Result<f64> {
    let maximas = key_local_maximas(&samples);
    let best_maxima = pick_maxima(&maximas).ok_or(anyhow!("Can't the best local maxima"))?;
    let interpolated_index = parabolic_interpolation(&best_maxima);

    Ok(sampling_rate as f64 / interpolated_index)
}

fn key_local_maximas(samples: &[f64]) -> Vec<KeyMaxima> {
    // It always starts with a negative slope
    let mut state = YAxis::Positive;
    let mut prev: f64 = samples[0];
    let mut max: Option<(usize, f64)> = None;
    let mut maximas: Vec<KeyMaxima> = vec![];

    let get_neighbor = |index: usize| -> KeyMaxima {
        let left = if index > 0 {
            Some(samples[index - 1])
        } else {
            None
        };
        let right = if index < samples.len() - 1 {
            Some(samples[index + 1])
        } else {
            None
        };

        KeyMaxima {
            index,
            max: samples[index],
            left_neighbor: left,
            right_neighbor: right,
        }
    };

    for (index, &sample) in samples.iter().enumerate() {
        if prev > 0.0 && sample <= 0.0 || prev <= 0.0 && sample > 0.0 {
            // Zero crossing
            match state {
                YAxis::Positive => {
                    if let Some((index, _)) = max {
                        maximas.push(get_neighbor(index));
                    }

                    max = None;
                    state = YAxis::Negative;
                }

                YAxis::Negative => {
                    state = YAxis::Positive;
                }
            }
        }

        if matches!(state, YAxis::Positive) {
            match max {
                Some((_, current_max)) if current_max < sample => max = Some((index, sample)),
                Some(_) => (),
                None => max = Some((index, sample)),
            }
        }

        prev = sample;
    }

    if let Some((index, _)) = max {
        maximas.push(get_neighbor(index));
    }

    return maximas;
}

const MAXIMA_THRESHOLD: f64 = 0.85;

fn pick_maxima(maximas: &[KeyMaxima]) -> Option<KeyMaxima> {
    let max = maximas
        .iter()
        .skip(1)
        .max_by(|m1, m2| f64::total_cmp(&m1.max, &m2.max));

    if let Some(KeyMaxima { max: max_value, .. }) = max {
        maximas
            .iter()
            .skip(1)
            .find(|m| m.max >= max_value * MAXIMA_THRESHOLD)
            .cloned()
    } else {
        None
    }
}

// Given three points, plot a parabole through 3 points and interpolate the maximum/minimum
fn parabolic_interpolation(points: &KeyMaxima) -> f64 {
    match points {
        KeyMaxima {
            index,
            left_neighbor: None,
            ..
        } => *index as f64,

        KeyMaxima {
            index,
            right_neighbor: None,
            ..
        } => *index as f64,

        KeyMaxima {
            index,
            max,
            left_neighbor: Some(left),
            right_neighbor: Some(right),
        } => {
            // Parabolic formula with 3 coefficients: ax^2 + bx + c = y
            // With three point (0, left), (1, mid) and (2, right), we have 3 equations
            // and can find all coefficients
            // The maximum of parabola is at x = -b / 2a
            let a = (left + right) / 2.0 - max;
            let b = 2.0 * max - 0.5 * right - 1.5 * left;
            let interpolated_max = -b / (2.0 * a);

            (*index as f64) - 1.0 + interpolated_max
        }
    }
}

const FREQUENCY_MAX_DIFFERENCE: f64 = 5.0;

// Infer which note is playing based on proximity of frequency
// If the difference in frequency is bigger than FREQUENCY_MAX_DIFFERENCE, we don't
// consider that note
// Returns None if we can't infer any note
fn infer_note(frequency: f64, tuning_notes: &[Note]) -> Option<Note> {
    tuning_notes
        .iter()
        .filter_map(|note| {
            let note_frequency = get_note_frequency(note);
            let diff = (frequency - note_frequency).abs();
            if diff <= FREQUENCY_MAX_DIFFERENCE {
                Some((note, diff))
            } else {
                None
            }
        })
        .max_by(|x1, x2| f64::total_cmp(&x1.1, &x2.1))
        .map(|(note, _)| note.clone())
}

#[allow(dead_code)]
fn plot_graph(samples: &[f64], index: usize) {
    let filename = format!("debug/frequencies_{}.json", index);
    let mut file = File::create(filename).unwrap();

    // let payload: String = samples[..2048]
    let payload: String = samples
        .iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join(",");

    file.write_all(format!("[{}]", payload).as_bytes()).unwrap();
}

// cutoff_frequency is specified as a fraction of sampling rate, from 0-0.5
// order must be an odd number so the filter is symmetric, makes it easier for calculation
fn low_pass_filter(cutoff_frequency: f64, order: usize) -> Vec<f64> {
    let sinc = |x: f64| -> f64 {
        if x == 0.0 {
            1.0
        } else {
            (PI * x).sin() / (PI * x)
        }
    };

    let bound: isize = (order as isize - 1) / 2;
    let signal: Vec<f64> = (-bound..=bound)
        .map(|x| 2.0 * cutoff_frequency * sinc(2.0 * cutoff_frequency * x as f64))
        .collect();

    // Applying Hamming window
    let window: Vec<f64> = apodize::hamming_iter(order).collect();

    // Linear convolve
    let coefficients: Vec<f64> = signal.iter().zip(window).map(|(s, w)| s * w).collect();

    // Normalize: make sure the sum of all coefficients is one
    let sum: f64 = coefficients.iter().sum();
    coefficients.iter().map(|x| x / sum).collect()
}

fn apply_filter(samples: &[f64], filter: &[f64]) -> Vec<f64> {
    let filter_order = filter.len();

    (0..samples.len())
        .map(|i| {
            let lower_bound = if i < filter_order {
                0
            } else {
                i - filter_order + 1
            };

            (lower_bound..=i)
                .rev()
                .zip(filter)
                .map(|(index, coefficient)| samples[index] * coefficient)
                .sum()
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::SAMPLE_RATE;
    use std::fs;

    fn overlap_chunks(samples: &[f64], chunk_size: usize, move_index: usize) -> Vec<Vec<f64>> {
        let mut index = 0;
        let mut chunks: Vec<Vec<f64>> = vec![];

        while index + chunk_size <= samples.len() {
            chunks.push(samples[index..index + chunk_size].to_vec());
            index += move_index;
        }

        chunks
    }

    fn detect_fixture_pitch(note: &Note) -> Vec<(Note, f64)> {
        let fixture = format!("test/fixtures/{}{}_pcm", note.note, note.octave);
        let tuning_notes = vec![
            Note::new("E4"),
            Note::new("B3"),
            Note::new("G3"),
            Note::new("D3"),
            Note::new("A2"),
            Note::new("E2"),
        ];

        let samples: Vec<f64> = fs::read_to_string(fixture)
            .unwrap()
            .lines()
            .map(|x| x.parse().unwrap())
            .collect();

        let chunk_size = 8192;

        overlap_chunks(&samples, chunk_size, chunk_size / 2)
            .into_iter()
            .take(5)
            .filter(|chunk| chunk.len() == chunk_size)
            .map(|mut chunk| detect_note(&mut chunk, SAMPLE_RATE, &tuning_notes).unwrap())
            .collect()
    }

    #[test]
    fn it_can_detect_e4() {
        let result = detect_fixture_pitch(&Note::new("E4"));

        let all_match = result.iter().all(|(n, _)| *n == Note::new("E4"));
        assert_eq!(all_match, true);

        let target_frequency = get_note_frequency(&Note::new("E4"));
        let detected_frequencies = result.iter().map(|&(_, f)| f).collect::<Vec<f64>>();

        assert_eq!(
            detected_frequencies
                .iter()
                .all(|&f| (f - target_frequency).abs() < 1.0),
            true
        );
    }

    #[test]
    fn it_can_detect_b3() {
        let result = detect_fixture_pitch(&Note::new("B3"));

        let all_match = result.iter().all(|(n, _)| *n == Note::new("B3"));
        assert_eq!(all_match, true);

        let target_frequency = get_note_frequency(&Note::new("B3"));
        let detected_frequencies = result.iter().map(|&(_, f)| f).collect::<Vec<f64>>();

        assert_eq!(
            detected_frequencies
                .iter()
                .all(|&f| (f - target_frequency).abs() < 1.0),
            true
        );
    }

    #[test]
    fn it_can_detect_g3() {
        let result = detect_fixture_pitch(&Note::new("G3"));

        let all_match = result.iter().all(|(n, _)| *n == Note::new("G3"));
        assert_eq!(all_match, true);

        let target_frequency = get_note_frequency(&Note::new("G3"));
        let detected_frequencies = result.iter().map(|&(_, f)| f).collect::<Vec<f64>>();

        assert_eq!(
            detected_frequencies
                .iter()
                .all(|&f| (f - target_frequency).abs() < 1.0),
            true
        );
    }

    #[test]
    fn it_can_detect_d3() {
        let result = detect_fixture_pitch(&Note::new("D3"));

        let all_match = result.iter().all(|(n, _)| *n == Note::new("D3"));
        assert_eq!(all_match, true);

        let target_frequency = get_note_frequency(&Note::new("D3"));
        let detected_frequencies = result.iter().map(|&(_, f)| f).collect::<Vec<f64>>();

        assert_eq!(
            detected_frequencies
                .iter()
                .all(|&f| (f - target_frequency).abs() < 1.0),
            true
        );
    }

    #[test]
    fn it_can_detect_a2() {
        let result = detect_fixture_pitch(&Note::new("A2"));

        let all_match = result.iter().all(|(n, _)| *n == Note::new("A2"));
        assert_eq!(all_match, true);

        let target_frequency = get_note_frequency(&Note::new("A2"));
        let detected_frequencies = result.iter().map(|&(_, f)| f).collect::<Vec<f64>>();

        assert_eq!(
            detected_frequencies
                .iter()
                .all(|&f| (f - target_frequency).abs() < 1.0),
            true
        );
    }

    #[test]
    fn it_can_detect_e2() {
        let result = detect_fixture_pitch(&Note::new("E2"));

        let all_match = result.iter().all(|(n, _)| *n == Note::new("E2"));
        assert_eq!(all_match, true);

        let target_frequency = get_note_frequency(&Note::new("E2"));
        let detected_frequencies = result.iter().map(|&(_, f)| f).collect::<Vec<f64>>();

        assert_eq!(
            detected_frequencies
                .iter()
                .all(|&f| (f - target_frequency).abs() < 1.0),
            true
        );
    }
}
