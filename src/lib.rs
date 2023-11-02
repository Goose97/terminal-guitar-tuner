#[macro_use]
extern crate lazy_static;

use crate::guitar::Note;

pub mod guitar;
pub mod pitch_detector;
pub mod recorder;
pub mod ui;

#[derive(Debug)]
pub enum AppEvent {
    PitchDetected(Note, f64),
    NoPitchDetected,
    AudioRecorded(Vec<f64>),
    DownButtonPressed,
    UpButtonPressed,
    LeftButtonPressed,
    RightButtonPressed,
    EscButtonPressed,
    Quit,
}

pub const SAMPLE_RATE: usize = 44100;
