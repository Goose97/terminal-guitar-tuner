use anyhow::Result;
use std::env;
use std::fs::OpenOptions;
use std::io::Write;
use std::sync::mpsc;
use std::thread;
use std::time::{Duration, Instant};

use terminal_guitar_tuner::guitar::Note;
use terminal_guitar_tuner::pitch_detector;
use terminal_guitar_tuner::recorder::Recorder;
use terminal_guitar_tuner::ui;
use terminal_guitar_tuner::AppEvent;

const FRAME_RATE_PER_SECOND: u64 = 2;

fn main() -> Result<()> {
    let (send, recv) = mpsc::channel::<AppEvent>();
    let debug = env::var("DEBUG").is_ok();

    thread::spawn(move || {
        let tuning_notes = vec![
            Note::new("E4"),
            Note::new("B3"),
            Note::new("G3"),
            Note::new("D3"),
            Note::new("A2"),
            Note::new("E2"),
        ];

        let mut next_frame_deadline = Instant::now();
        let buffer_size = 1 << 12;
        let mut recorder = Recorder::new(buffer_size);
        let sample_rate = recorder.record().unwrap();

        // Open a file with append option
        let mut debug_log_file = if debug {
            let file = OpenOptions::new()
                .append(true)
                .create(true)
                .open("debug.log")
                .unwrap();

            Some(file)
        } else {
            None
        };

        // Loop until interrupted by user
        loop {
            next_frame_deadline += Duration::from_millis(1000 / FRAME_RATE_PER_SECOND);

            recorder.with_samples(|samples| {
                let result = pitch_detector::detect_note(&samples, sample_rate.0, &tuning_notes);

                let event = match result {
                    Ok((note, frequency)) => AppEvent::PitchDetected(note, frequency),
                    Err(_) => AppEvent::NoPitchDetected,
                };

                // Write to a file
                match debug_log_file.as_mut() {
                    Some(file) => {
                        file.write(format!("{:?}\n", event).as_bytes()).unwrap();
                    }

                    None => (),
                }

                let _ = send.send(event);
                let _ = send.send(AppEvent::AudioRecorded(samples));
                thread::sleep(next_frame_deadline - Instant::now());
            });
        }
    });

    ui::render(recv)
}
