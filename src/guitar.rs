use anyhow::{anyhow, Error, Result};
use core::fmt;
use regex::Regex;
use std::str::FromStr;

#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq)]
enum Accidentals {
    Sharp,
    Flat,
}

impl fmt::Display for Accidentals {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let note_string = match self {
            Accidentals::Sharp => "♯",
            Accidentals::Flat => "♭",
        };

        write!(f, "{}", note_string)
    }
}

impl FromStr for Accidentals {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "#" => Ok(Accidentals::Sharp),
            "b" => Ok(Accidentals::Flat),
            _ => Err(anyhow!("Invalid accidentals")),
        }
    }
}

#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq)]
pub enum BaseNote {
    A,
    B,
    C,
    D,
    E,
    F,
    G,
}

impl fmt::Display for BaseNote {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let note_string = match self {
            BaseNote::A => "A",
            BaseNote::B => "B",
            BaseNote::C => "C",
            BaseNote::D => "D",
            BaseNote::E => "E",
            BaseNote::F => "F",
            BaseNote::G => "G",
        };

        write!(f, "{}", note_string)
    }
}

impl FromStr for BaseNote {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "A" => Ok(BaseNote::A),
            "B" => Ok(BaseNote::B),
            "C" => Ok(BaseNote::C),
            "D" => Ok(BaseNote::D),
            "E" => Ok(BaseNote::E),
            "F" => Ok(BaseNote::F),
            "G" => Ok(BaseNote::G),
            _ => Err(anyhow!("Invalid base note")),
        }
    }
}

#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq)]
pub struct Note {
    pub note: BaseNote,
    pub octave: u8,
    accidentals: Option<Accidentals>,
}

impl fmt::Display for Note {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let accidentals = match &self.accidentals {
            Some(a) => a.to_string(),
            None => String::new(),
        };

        write!(f, "{}{}({})", self.note, accidentals, self.octave)
    }
}

impl FromStr for Note {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let re = Regex::new(r"^([ABCDEFG])([#b]?)(\d)$").unwrap();
        let captures = re.captures(s).unwrap();

        let note: BaseNote = captures[1].parse()?;
        let accidentals = if captures[2].is_empty() {
            None
        } else {
            Some(captures[2].parse()?)
        };

        let octave: u8 = captures[3].parse()?;

        Ok(Self {
            note,
            accidentals,
            octave,
        })
    }
}

impl Note {
    pub fn new(string_representation: &str) -> Self {
        let note = string_representation.parse().unwrap();

        let valid = match note {
            Note {
                note: BaseNote::E,
                accidentals: Some(Accidentals::Sharp),
                ..
            } => false,

            Note {
                note: BaseNote::B,
                accidentals: Some(Accidentals::Sharp),
                ..
            } => false,

            Note {
                note: BaseNote::C,
                accidentals: Some(Accidentals::Flat),
                ..
            } => false,

            Note {
                note: BaseNote::F,
                accidentals: Some(Accidentals::Flat),
                ..
            } => false,

            _ => true,
        };

        if !valid {
            panic!("Invalid note {:?}", note)
        }

        note
    }
}

const A4_FREQUENCY: f64 = 440.0;

pub fn get_note_frequency(note: &Note) -> f64 {
    let difference = semi_tone_count(note) - semi_tone_count(&Note::new("A4"));

    let exponent = difference as f64 / 12.0;
    A4_FREQUENCY * 2_f64.powf(exponent)
}

fn semi_tone_count(note: &Note) -> i8 {
    let mut count: i8 = match note.note {
        BaseNote::C => 0,
        BaseNote::D => 2,
        BaseNote::E => 4,
        BaseNote::F => 5,
        BaseNote::G => 7,
        BaseNote::A => 9,
        BaseNote::B => 11,
    };

    match note.accidentals {
        Some(Accidentals::Sharp) => count += 1,
        Some(Accidentals::Flat) => count -= 1,
        None => (),
    }

    count + (note.octave as i8) * 12
}

pub fn semi_tone_up(note: &Note) -> Note {
    match note {
        // E -> F
        Note {
            accidentals: None,
            note: BaseNote::E,
            ..
        } => Note {
            note: BaseNote::F,
            accidentals: None,
            octave: note.octave,
        },

        // B -> C
        Note {
            accidentals: None,
            note: BaseNote::B,
            ..
        } => Note {
            note: BaseNote::C,
            accidentals: None,
            octave: note.octave + 1,
        },

        Note {
            accidentals: None, ..
        } => {
            let mut clone = note.clone();
            clone.accidentals = Some(Accidentals::Sharp);

            clone
        }

        Note {
            accidentals: Some(Accidentals::Sharp),
            note: base_note,
            octave,
        } => {
            let new_base_note = next_base_note(&base_note);
            let new_octave = if *base_note == BaseNote::B {
                *octave + 1
            } else {
                *octave
            };

            Note {
                note: new_base_note,
                octave: new_octave,
                accidentals: None,
            }
        }

        Note {
            accidentals: Some(Accidentals::Flat),
            ..
        } => {
            let mut clone = note.clone();
            clone.accidentals = None;

            clone
        }
    }
}

pub fn semi_tone_down(note: &Note) -> Note {
    match note {
        Note {
            accidentals: None,
            note: BaseNote::F,
            ..
        } => Note {
            note: BaseNote::E,
            accidentals: None,
            octave: note.octave,
        },

        Note {
            accidentals: None,
            note: BaseNote::C,
            ..
        } => Note {
            note: BaseNote::B,
            accidentals: None,
            octave: note.octave - 1,
        },

        Note {
            accidentals: None, ..
        } => {
            let mut clone = note.clone();
            clone.accidentals = Some(Accidentals::Flat);

            clone
        }

        Note {
            accidentals: Some(Accidentals::Flat),
            note: base_note,
            octave,
        } => {
            let new_base_note = prev_base_note(&base_note);
            let new_octave = if *base_note == BaseNote::C {
                *octave - 1
            } else {
                *octave
            };

            Note {
                note: new_base_note,
                octave: new_octave,
                accidentals: None,
            }
        }

        Note {
            accidentals: Some(Accidentals::Sharp),
            ..
        } => {
            let mut clone = note.clone();
            clone.accidentals = None;

            clone
        }
    }
}

fn next_base_note(base_note: &BaseNote) -> BaseNote {
    let notes = vec![
        BaseNote::A,
        BaseNote::B,
        BaseNote::C,
        BaseNote::D,
        BaseNote::E,
        BaseNote::F,
        BaseNote::G,
    ];

    let index = notes.iter().position(|n| n == base_note).unwrap();
    notes[(index + 1) % notes.len()].clone()
}

fn prev_base_note(base_note: &BaseNote) -> BaseNote {
    let notes = vec![
        BaseNote::A,
        BaseNote::B,
        BaseNote::C,
        BaseNote::D,
        BaseNote::E,
        BaseNote::F,
        BaseNote::G,
    ];

    let index = notes.iter().position(|n| n == base_note).unwrap();
    notes[(index + notes.len() - 1) % notes.len()].clone()
}

#[cfg(test)]
mod note_new {
    use super::*;

    #[test]
    fn a4() {
        assert_eq!(
            Note::new("A4"),
            Note {
                note: BaseNote::A,
                accidentals: None,
                octave: 4
            }
        );
    }

    #[test]
    fn a4_sharp() {
        assert_eq!(
            Note::new("A#4"),
            Note {
                note: BaseNote::A,
                accidentals: Some(Accidentals::Sharp),
                octave: 4
            }
        );
    }

    #[test]
    fn a4_flat() {
        assert_eq!(
            Note::new("Ab4"),
            Note {
                note: BaseNote::A,
                accidentals: Some(Accidentals::Flat),
                octave: 4
            }
        );
    }

    #[test]
    #[should_panic]
    fn c4_can_not_flat() {
        Note::new("Cb4");
    }

    #[test]
    #[should_panic]
    fn f4_can_not_flat() {
        Note::new("Fb4");
    }

    #[test]
    #[should_panic]
    fn e4_can_not_sharp() {
        Note::new("E#4");
    }

    #[test]
    #[should_panic]
    fn b4_can_not_sharp() {
        Note::new("B#4");
    }
}

#[cfg(test)]
mod get_note_frequency_tests {
    use super::*;

    #[test]
    fn a4() {
        let result = get_note_frequency(&Note::new("A4"));
        assert_eq!(result, A4_FREQUENCY);
    }

    #[test]
    fn a4_sharp() {
        let result = get_note_frequency(&Note::new("A#4"));
        assert_eq!(result, 466.1637615180899);
    }

    #[test]
    fn c4() {
        let result = get_note_frequency(&Note::new("C4"));
        assert_eq!(result, 261.6255653005986);
    }

    #[test]
    fn b3_flat() {
        let result = get_note_frequency(&Note::new("Bb3"));
        assert_eq!(result, 233.08188075904496);
    }

    #[test]
    fn e4() {
        let result = get_note_frequency(&Note::new("E4"));
        assert_eq!(result, 329.6275569128699);
    }

    #[test]
    fn g3() {
        let result = get_note_frequency(&Note::new("G3"));
        assert_eq!(result, 195.99771799087463);
    }

    #[test]
    fn d3() {
        let result = get_note_frequency(&Note::new("D3"));
        assert_eq!(result, 146.8323839587038);
    }

    #[test]
    fn a2() {
        let result = get_note_frequency(&Note::new("A2"));
        assert_eq!(result, 110.0);
    }

    #[test]
    fn e2() {
        let result = get_note_frequency(&Note::new("E2"));
        assert_eq!(result, 82.4068892282175);
    }
}

#[cfg(test)]
mod semi_tone_up_tests {
    use super::*;

    #[test]
    fn a4() {
        let result = semi_tone_up(&Note::new("A4"));
        assert_eq!(result, Note::new("A#4"));
    }

    #[test]
    fn a4_sharp() {
        let result = semi_tone_up(&Note::new("A#4"));
        assert_eq!(result, Note::new("B4"));
    }

    #[test]
    fn a4_flat() {
        let result = semi_tone_up(&Note::new("Ab4"));
        assert_eq!(result, Note::new("A4"));
    }

    #[test]
    fn e4() {
        let result = semi_tone_up(&Note::new("E4"));
        assert_eq!(result, Note::new("F4"));
    }

    #[test]
    fn b4_flat() {
        let result = semi_tone_up(&Note::new("B4"));
        assert_eq!(result, Note::new("C5"));
    }
}

#[cfg(test)]
mod semi_tone_down_tests {
    use super::*;

    #[test]
    fn a4() {
        let result = semi_tone_down(&Note::new("A4"));
        assert_eq!(result, Note::new("Ab4"));
    }

    #[test]
    fn a4_sharp() {
        let result = semi_tone_down(&Note::new("A#4"));
        assert_eq!(result, Note::new("A4"));
    }

    #[test]
    fn a4_flat() {
        let result = semi_tone_down(&Note::new("Ab4"));
        assert_eq!(result, Note::new("G4"));
    }

    #[test]
    fn c4() {
        let result = semi_tone_down(&Note::new("C4"));
        assert_eq!(result, Note::new("B3"));
    }
}
