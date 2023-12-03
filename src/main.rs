mod lzss;
mod fileops;
mod bitwriter;
mod bitreader;
mod ui;
mod tests;
mod map;
mod heap;
mod huffman;
mod node;

use crossterm::{
    event::{self, KeyCode, KeyEventKind},
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use ratatui::{widgets::{Block, Borders, BorderType}, prelude::{Alignment, Constraint, Direction, Layout}, style::{Style, Color}};
use ratatui::{
    prelude::{CrosstermBackend, Stylize, Terminal},
    widgets::Paragraph,
};
use std::io::{stderr, Result};

fn main() -> Result<()> {
    stderr().execute(EnterAlternateScreen)?;
    enable_raw_mode()?;
    let mut terminal = Terminal::new(CrosstermBackend::new(stderr()))?;
    terminal.clear()?;

    loop {
        terminal.draw(|frame| {
            let area = frame.size();
            let outer_layout = Layout::default()
                .direction(Direction::Horizontal)
                .constraints(vec![
                    Constraint::Percentage(50),
                    Constraint::Percentage(10),
                ])
                .split(area);

            frame.render_widget(
                Paragraph::new(
                    format!("Welcome to my File Compression App")
                )
                .alignment(Alignment::Center)
                .block(
                    Block::default()
                      .title("File Compressor")
                      .title_alignment(Alignment::Center)
                      .borders(Borders::ALL)
                      .border_type(BorderType::Rounded),
                )
                .style(Style::default().fg(Color::Blue)),
                outer_layout[0],
            );

            frame.render_widget(
                Paragraph::new(
                    format!("Welcome to my File Compression App")
                )
                .alignment(Alignment::Center)
                .block(
                    Block::default()
                      .title("File Compressor")
                      .title_alignment(Alignment::Center)
                      .borders(Borders::ALL)
                      .border_type(BorderType::Rounded),
                )
                .style(Style::default().fg(Color::Blue)),
                outer_layout[1],
            );


        })?;

        if event::poll(std::time::Duration::from_millis(100))? {
            if let event::Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press && key.code == KeyCode::Char('q') {
                    break;
                }
            }
        }
    }

    stderr().execute(LeaveAlternateScreen)?;
    disable_raw_mode()?;
    Ok(())
}
