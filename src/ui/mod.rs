use anyhow::Result;
use itertools::Itertools;
use ratatui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout, Rect},
    style::Style,
    terminal::Terminal,
    widgets::{Block, BorderType, Borders},
};
use std::{mem::discriminant, sync::mpsc::Receiver};

use super::AppEvent;
use app_state::AppState;
use audio_graph::AudioGraph;
use instructions::Instruction;
use tuning_bar::TuningBar;
use tuning_notes::TuningNotes;
use tuning_pegs::TuningPegs;

mod app_color;
mod app_state;
mod audio_graph;
mod instructions;
mod loading_icon;
mod tuning_bar;
mod tuning_notes;
mod tuning_pegs;
mod utils;

pub static mut FRAME_COUNT: usize = 0;

// In cents. 100 cents is 1 semitone
pub const IN_TUNE_RANGE: f64 = 8.0;

pub fn render(event_stream: Receiver<AppEvent>) -> Result<()> {
    // startup: Enable raw mode for the terminal, giving us fine control over user input
    crossterm::terminal::enable_raw_mode()?;
    crossterm::execute!(std::io::stderr(), crossterm::terminal::EnterAlternateScreen)?;

    let mut terminal = Terminal::new(CrosstermBackend::new(std::io::stderr()))?;
    let mut app_state = AppState::new();

    // Main application loop
    loop {
        terminal.draw(|f| {
            unsafe {
                FRAME_COUNT += 1;
            }

            let [tuning_strings_rect, instructions_rect, tuning_bar_rect, graph_rect] =
                calculate_layout(f.size());

            // Background
            f.render_widget(
                Block::default().style(Style::default().bg(*app_color::BACKGROUND_DARK)),
                f.size(),
            );

            f.render_widget(
                Block::default()
                    .borders(Borders::ALL)
                    .title("Tuning strings")
                    .border_type(BorderType::Rounded)
                    .border_style(Style::default().fg(*app_color::BORDER)),
                tuning_strings_rect,
            );

            f.render_widget(
                Block::default()
                    .borders(Borders::ALL)
                    .title("Instructions")
                    .border_type(BorderType::Rounded)
                    .border_style(Style::default().fg(*app_color::BORDER)),
                instructions_rect,
            );

            f.render_widget(
                Block::default()
                    .borders(Borders::ALL)
                    .title("Tuning bar")
                    .border_type(BorderType::Rounded)
                    .border_style(Style::default().fg(*app_color::BORDER)),
                tuning_bar_rect,
            );

            f.render_widget(
                Block::default()
                    .borders(Borders::ALL)
                    .title("Audio graph")
                    .border_type(BorderType::Rounded)
                    .border_style(Style::default().fg(*app_color::BORDER)),
                graph_rect,
            );

            let layout = Layout::default()
                .direction(Direction::Horizontal)
                .constraints(vec![Constraint::Min(16), Constraint::Percentage(100)])
                .split(tuning_strings_rect);

            f.render_stateful_widget(
                TuningNotes::new(),
                utils::transform(layout[0], 2, 1),
                &mut app_state.tuning_notes,
            );

            f.render_stateful_widget(
                TuningPegs::new(),
                layout[1],
                &mut tuning_pegs::State {
                    focus_peg: current_peg_index(&app_state),
                },
            );
            f.render_stateful_widget(TuningBar::new(), tuning_bar_rect, &mut app_state.tuning_bar);

            f.render_widget(Instruction::new(), instructions_rect);
            f.render_stateful_widget(AudioGraph::new(), graph_rect, &mut app_state.audio_graph);
        })?;

        match poll_terminal_event()? {
            Some(AppEvent::Quit) => break,
            Some(event) => app_state.handle_event(&event),
            None => (),
        }

        // Drop all events except for the last one
        let events: Vec<AppEvent> = event_stream
            .try_iter()
            .unique_by(|x| discriminant(x))
            .collect();

        for event in events.iter() {
            app_state.handle_event(event);
        }
    }

    // shutdown down: reset terminal back to original state
    crossterm::execute!(std::io::stderr(), crossterm::terminal::LeaveAlternateScreen)?;
    crossterm::terminal::disable_raw_mode()?;

    Ok(())
}

fn current_peg_index(state: &AppState) -> Option<usize> {
    match state.tuning_notes.detecting_note {
        Some(note) => {
            let index = state
                .tuning_notes
                .notes
                .iter()
                .position(|item| *item == note)
                .unwrap();

            Some(index)
        }

        None => None,
    }
}

// Layout is as follow
//  ------------------------------------
// |                 |                  |
// |  tuning_strings |    instructions  |
// |       (0)       |       (1)        |
// |                 |                  |
// |------------------------------------|
// |              tuning_bar            |
// |                 (2)                |
//  ------------------------------------
fn calculate_layout(root_rect: Rect) -> [Rect; 4] {
    let total_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints(vec![
            Constraint::Max(24),
            Constraint::Max(16),
            Constraint::Min(0),
        ])
        .split(root_rect);

    let upper_half = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(vec![Constraint::Percentage(60), Constraint::Percentage(40)])
        .split(total_layout[0]);

    return [
        upper_half[0],
        upper_half[1],
        total_layout[1],
        total_layout[2],
    ];
}

fn poll_terminal_event() -> Result<Option<AppEvent>> {
    if crossterm::event::poll(std::time::Duration::from_millis(25))? {
        // If a key event occurs, handle it
        if let crossterm::event::Event::Key(key) = crossterm::event::read()? {
            if key.kind == crossterm::event::KeyEventKind::Press {
                let event = match key.code {
                    crossterm::event::KeyCode::Char('j') => Some(AppEvent::DownButtonPressed),
                    crossterm::event::KeyCode::Char('k') => Some(AppEvent::UpButtonPressed),
                    crossterm::event::KeyCode::Char('l') => Some(AppEvent::RightButtonPressed),
                    crossterm::event::KeyCode::Char('h') => Some(AppEvent::LeftButtonPressed),
                    crossterm::event::KeyCode::Esc => Some(AppEvent::EscButtonPressed),
                    crossterm::event::KeyCode::Char('q') => Some(AppEvent::Quit),
                    _ => None,
                };

                return Ok(event);
            }
        }
    }

    Ok(None)
}
