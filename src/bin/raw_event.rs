use anyhow::Result;
use std::thread;
use std::time::{Duration, Instant};

use terminal_guitar_tuner::guitar::{get_note_frequency, Note};
use terminal_guitar_tuner::pitch_detector;
use terminal_guitar_tuner::recorder::Recorder;
use terminal_guitar_tuner::{AppEvent, SAMPLE_RATE};

const FRAME_RATE_PER_SECOND: u64 = 2;

fn main() -> Result<()> {
    let join_handle = thread::spawn(move || {
        let tuning_notes = vec![
            Note::new("E4"),
            Note::new("B3"),
            Note::new("G3"),
            Note::new("D3"),
            Note::new("A2"),
            Note::new("E2"),
        ];

        let mut next_frame_deadline = Instant::now();
        let buffer_size = 1 << 11;
        let mut recorder = Recorder::new(SAMPLE_RATE, buffer_size);
        recorder.record().unwrap();

        // Loop until interrupted by user
        loop {
            next_frame_deadline += Duration::from_millis(1000 / FRAME_RATE_PER_SECOND);

            recorder.with_samples(|samples| {
                let result = pitch_detector::detect_note(&samples, SAMPLE_RATE, &tuning_notes);

                let event = match result {
                    Ok((note, frequency)) => AppEvent::PitchDetected(note, frequency),
                    Err(_) => AppEvent::NoPitchDetected,
                };

                println!("{:?}", event);

                if let AppEvent::PitchDetected(note, _) = event {
                    let perfect_pitch = get_note_frequency(&note);
                    println!("Perfect pitch {:?}", perfect_pitch);
                }

                thread::sleep(next_frame_deadline - Instant::now());
            });
        }
    });

    let _ = join_handle.join();
    Ok(())
}
