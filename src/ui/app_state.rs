use std::collections::HashSet;

use super::{audio_graph, tuning_bar, tuning_notes};
use crate::guitar::Note;
use crate::AppEvent;

#[derive(Clone, Debug, PartialEq)]
pub struct AppState {
    pub tuning_notes: tuning_notes::State,
    pub tuning_bar: tuning_bar::State,
    pub audio_graph: audio_graph::State,
}

impl AppState {
    pub fn new() -> Self {
        let tuning_notes_state = tuning_notes::State {
            notes: vec![
                Note::new("E4"),
                Note::new("B3"),
                Note::new("G3"),
                Note::new("D3"),
                Note::new("A2"),
                Note::new("E2"),
            ],
            tuned_notes: HashSet::new(),
            selected_note_index: None,
            detecting_note: None,
        };

        let tuning_bar_state = tuning_bar::State::new(&tuning_notes_state.notes[0]);
        let audio_graph_state = audio_graph::State::new();

        AppState {
            tuning_notes: tuning_notes_state,
            tuning_bar: tuning_bar_state,
            audio_graph: audio_graph_state,
        }
    }

    pub fn handle_event(&mut self, event: &AppEvent) {
        match event {
            AppEvent::UpButtonPressed => self.tuning_notes.prev_string(),
            AppEvent::DownButtonPressed => self.tuning_notes.next_string(),
            AppEvent::RightButtonPressed => self.tuning_notes.next_note(),
            AppEvent::LeftButtonPressed => self.tuning_notes.prev_note(),
            AppEvent::EscButtonPressed => self.tuning_notes.selected_note_index = None,

            // To protect against accidental noise, a string is considered in tune if
            // the detected pitch stays in the accept_range two times in a row
            AppEvent::PitchDetected(note, frequency) => {
                if !self.tuning_notes.notes.contains(&note) {
                    return;
                }

                if self.tuning_notes.detecting_note != Some(*note) {
                    self.tuning_notes.detecting_note = Some(*note);
                    self.tuning_bar = tuning_bar::State::new(note);
                }

                self.tuning_bar.current_pitch = Some(*frequency);

                if self.tuning_bar.in_tune_range(*frequency) {
                    if self.tuning_bar.pitch_in_accept_range_once {
                        self.tuning_notes.tuned_notes.insert(*note);
                    } else {
                        self.tuning_bar.pitch_in_accept_range_once = true;
                    }
                } else {
                    self.tuning_bar.pitch_in_accept_range_once = false;
                }
            }

            AppEvent::NoPitchDetected => {
                self.tuning_notes.detecting_note = None;
                self.tuning_bar = tuning_bar::State::new(&self.tuning_notes.notes[0]);
            }

            AppEvent::AudioRecorded(data) => self.audio_graph.dataset = data.clone(),
            AppEvent::Quit => (),
        }
    }
}

#[cfg(test)]
mod handle_event_tests {
    use super::*;

    #[test]
    fn up_button_pressed_no_selected_string() {
        let mut state = AppState::new();
        state.tuning_notes.notes = vec![Note::new("E4"), Note::new("F4")];
        state.tuning_notes.selected_note_index = None;

        state.handle_event(&AppEvent::UpButtonPressed);

        assert_eq!(state.tuning_notes.selected_note_index, Some(1));
    }

    #[test]
    fn up_button_pressed_with_selected_string() {
        let mut state = AppState::new();
        state.tuning_notes.notes = vec![Note::new("E4"), Note::new("F4")];
        state.tuning_notes.selected_note_index = Some(1);

        state.handle_event(&AppEvent::UpButtonPressed);

        assert_eq!(state.tuning_notes.selected_note_index, Some(0));
    }

    #[test]
    fn down_button_pressed_no_selected_string() {
        let mut state = AppState::new();
        state.tuning_notes.notes = vec![Note::new("E4"), Note::new("F4")];
        state.tuning_notes.selected_note_index = None;

        state.handle_event(&AppEvent::DownButtonPressed);

        assert_eq!(state.tuning_notes.selected_note_index, Some(0));
    }

    #[test]
    fn down_button_pressed_with_selected_string() {
        let mut state = AppState::new();
        state.tuning_notes.notes = vec![Note::new("E4"), Note::new("F4")];
        state.tuning_notes.selected_note_index = Some(0);

        state.handle_event(&AppEvent::DownButtonPressed);

        assert_eq!(state.tuning_notes.selected_note_index, Some(1));
    }

    #[test]
    fn left_button_pressed_no_selected_string() {
        let mut state = AppState::new();
        state.tuning_notes.notes = vec![Note::new("E4"), Note::new("F4")];
        state.tuning_notes.selected_note_index = None;
        let clone = state.clone();

        state.handle_event(&AppEvent::LeftButtonPressed);

        assert_eq!(state, clone);
    }

    #[test]
    fn left_button_pressed_with_selected_string() {
        let mut state = AppState::new();
        state.tuning_notes.notes = vec![Note::new("E4"), Note::new("F4")];
        state.tuning_notes.selected_note_index = Some(0);

        state.handle_event(&AppEvent::LeftButtonPressed);

        assert_eq!(
            state.tuning_notes.notes,
            vec![Note::new("Eb4"), Note::new("F4")]
        );
    }

    #[test]
    fn right_button_pressed_no_selected_string() {
        let mut state = AppState::new();
        state.tuning_notes.notes = vec![Note::new("E4"), Note::new("F4")];
        state.tuning_notes.selected_note_index = None;
        let clone = state.clone();

        state.handle_event(&AppEvent::RightButtonPressed);

        assert_eq!(state, clone);
    }

    #[test]
    fn right_button_pressed_with_selected_string() {
        let mut state = AppState::new();
        state.tuning_notes.notes = vec![Note::new("E4"), Note::new("F4")];
        state.tuning_notes.selected_note_index = Some(0);

        state.handle_event(&AppEvent::RightButtonPressed);

        assert_eq!(
            state.tuning_notes.notes,
            vec![Note::new("F4"), Note::new("F4")]
        );
    }

    #[test]
    fn pitch_detected_note_exists() {
        let mut state = AppState::new();
        state.tuning_notes.notes = vec![Note::new("E4"), Note::new("F4")];
        state.tuning_notes.selected_note_index = Some(0);

        let note = Note::new("E4");
        let pitch = 329.0;
        state.handle_event(&AppEvent::PitchDetected(note, pitch));

        assert_eq!(state.tuning_notes.detecting_note, Some(note));
        assert_eq!(state.tuning_bar.current_pitch, Some(pitch));
    }

    #[test]
    fn pitch_detected_note_does_not_exist() {
        let mut state = AppState::new();
        state.tuning_notes.notes = vec![Note::new("E4"), Note::new("F4")];
        state.tuning_notes.selected_note_index = Some(0);

        let clone = state.clone();
        let note = Note::new("A4");
        state.handle_event(&AppEvent::PitchDetected(note, 440.0));

        assert_eq!(state, clone);
    }

    #[test]
    fn pitch_detected_note_once() {
        let mut state = AppState::new();
        state.tuning_notes.notes = vec![Note::new("E4"), Note::new("F4")];
        state.tuning_notes.selected_note_index = Some(0);

        let note = Note::new("E4");
        let pitch = 329.0;
        state.handle_event(&AppEvent::PitchDetected(note, pitch));

        assert_eq!(state.tuning_notes.tuned_notes.contains(&note), false);
        assert_eq!(state.tuning_bar.current_pitch, Some(pitch));
    }

    #[test]
    fn pitch_detected_note_twice() {
        let mut state = AppState::new();
        state.tuning_notes.notes = vec![Note::new("E4"), Note::new("F4")];
        state.tuning_notes.selected_note_index = Some(0);

        let note = Note::new("E4");
        let pitch = 329.0;
        state.handle_event(&AppEvent::PitchDetected(note, pitch));
        state.handle_event(&AppEvent::PitchDetected(note, pitch));

        assert_eq!(state.tuning_notes.tuned_notes.contains(&note), true);
        assert_eq!(state.tuning_bar.current_pitch, Some(pitch));
    }

    #[test]
    fn pitch_detected_note_switch_between_notes() {
        let mut state = AppState::new();
        state.tuning_notes.notes = vec![Note::new("E4"), Note::new("F4")];
        state.tuning_notes.selected_note_index = Some(0);

        let note = Note::new("E4");
        let pitch = 329.0;
        state.handle_event(&AppEvent::PitchDetected(note, pitch));
        state.handle_event(&AppEvent::PitchDetected(Note::new("F4"), 349.0));
        state.handle_event(&AppEvent::PitchDetected(note, pitch));

        assert_eq!(state.tuning_notes.tuned_notes.contains(&note), false);
        assert_eq!(state.tuning_bar.current_pitch, Some(pitch));
    }

    #[test]
    fn pitch_detected_note_interrupt_by_no_pitch() {
        let mut state = AppState::new();
        state.tuning_notes.notes = vec![Note::new("E4"), Note::new("F4")];
        state.tuning_notes.selected_note_index = Some(0);

        let note = Note::new("E4");
        let pitch = 329.0;
        state.handle_event(&AppEvent::PitchDetected(note, pitch));
        state.handle_event(&AppEvent::NoPitchDetected);
        state.handle_event(&AppEvent::PitchDetected(note, pitch));

        assert_eq!(state.tuning_notes.tuned_notes.contains(&note), false);
        assert_eq!(state.tuning_bar.current_pitch, Some(pitch));
    }

    #[test]
    fn no_pitch_detected() {
        let mut state = AppState::new();
        state.tuning_notes.notes = vec![Note::new("E4"), Note::new("F4")];
        state.tuning_notes.selected_note_index = Some(0);
        state.tuning_notes.detecting_note = Some(state.tuning_notes.notes[0].clone());
        state.tuning_bar.current_pitch = Some(100.0);

        state.handle_event(&AppEvent::NoPitchDetected);

        assert_eq!(state.tuning_notes.detecting_note, None);
        assert_eq!(state.tuning_bar.current_pitch, None);
    }
}
