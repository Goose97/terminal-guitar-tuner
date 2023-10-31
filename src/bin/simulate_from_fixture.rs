use anyhow::Result;
use std::fs;
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

use terminal_guitar_tuner::guitar::Note;
use terminal_guitar_tuner::pitch_detector;
use terminal_guitar_tuner::ui;
use terminal_guitar_tuner::{AppEvent, SAMPLE_RATE};

// Simulate from fixture
fn main() -> Result<()> {
    let (send, recv) = mpsc::channel::<AppEvent>();
    let tuning_notes = vec![
        Note::new("E4"),
        Note::new("B3"),
        Note::new("G3"),
        Note::new("D3"),
        Note::new("A2"),
        Note::new("E2"),
    ];

    let samples: Vec<f64> = fs::read_to_string("test/fixtures/G3_pcm")
        .unwrap()
        .lines()
        .map(|x| x.parse().unwrap())
        .collect();

    let chunk_size = 1 << 10;

    thread::spawn(move || {
        for mut chunk in samples.chunks(chunk_size).into_iter() {
            let result = pitch_detector::detect_note(&mut chunk, SAMPLE_RATE, &tuning_notes);
            let event = match result {
                Ok((note, frequency)) => AppEvent::PitchDetected(note, frequency),
                Err(_) => AppEvent::NoPitchDetected,
            };

            thread::sleep(Duration::from_millis(1000));
            let _ = send.send(event);
        }
    });

    ui::render(recv)
}
