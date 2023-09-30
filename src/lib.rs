pub mod guitar;
pub mod pitch_detector;
pub mod recorder;
pub mod ui;

use crate::guitar::Note;

#[derive(Debug)]
pub enum AppEvent {
    PitchDetected(Note, f64),
    NoPitchDetected,
    DownButtonPressed,
    UpButtonPressed,
    LeftButtonPressed,
    RightButtonPressed,
    EscButtonPressed,
    Quit,
}

pub const SAMPLE_RATE: usize = 44100;
