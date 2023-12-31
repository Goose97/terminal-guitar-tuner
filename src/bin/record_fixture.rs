use anyhow::Result;
use std::fs::File;
use std::io::Write;
use std::thread;
use std::time::Duration;
use terminal_guitar_tuner::FIXTURE_SAMPLE_RATE;

use terminal_guitar_tuner::recorder::Recorder;

// Record into fixtures
fn main() -> Result<()> {
    let mut recorder = Recorder::new(FIXTURE_SAMPLE_RATE as usize * 2);
    recorder.record()?;

    thread::sleep(Duration::from_millis(2500));

    recorder.with_samples(|samples| {
        let payload: String = samples
            .iter()
            .map(|&x| x.to_string())
            .collect::<Vec<String>>()
            .join("\n");

        let mut file = File::options()
            .write(true)
            .truncate(true)
            .open("test/fixtures/D3_pcm")
            .unwrap();

        file.write_all(format!("{}", payload).as_bytes()).unwrap();
    });

    Ok(())
}
