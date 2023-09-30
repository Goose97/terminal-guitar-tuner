use super::FRAME_COUNT;
use ratatui::text::Span;

#[derive(Clone, Copy)]
pub struct LoadingIcon {
    char: char,
}

// Higher is slower
const SPINNING_SPEED: usize = 4;

impl LoadingIcon {
    pub fn new() -> Self {
        let char = unsafe {
            match FRAME_COUNT % (8 * SPINNING_SPEED) / SPINNING_SPEED {
                0 => '⣷',
                1 => '⣯',
                2 => '⣟',
                3 => '⡿',
                4 => '⢿',
                5 => '⣻',
                6 => '⣽',
                7 => '⣾',
                _ => unreachable!(),
            }
        };

        Self { char }
    }
}

impl<'a> Into<Span<'a>> for LoadingIcon {
    fn into(self) -> Span<'a> {
        Span::from(self.char.to_string())
    }
}
