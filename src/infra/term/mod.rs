use std::io;
use termion::{input::MouseTerminal, raw::IntoRawMode, screen::AlternateScreen};
use tui::{
    backend::TermionBackend,
    layout::{Constraint, Layout},
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, Cell, Row, Table},
    Terminal,
};

use crate::prelude::*;

pub struct StatefulTable<'a> {
    items: Vec<Vec<&'a str>>,
}

impl<'a> StatefulTable<'a> {
    pub fn new() -> StatefulTable<'a> {
        StatefulTable {
            items: vec![vec!["Row191", "Row192", "Row193"]],
        }
    }

    pub fn render(&self) -> Result<()> {
        dbg!("rendienrg");
        // Terminal initialization
        let stdout = io::stdout().into_raw_mode()?;
        let stdout = MouseTerminal::from(stdout);
        let stdout = AlternateScreen::from(stdout);
        let backend = TermionBackend::new(stdout);
        let mut terminal = Terminal::new(backend)?;

        loop {
            terminal.draw(|f| {
                let rects = Layout::default()
                    .constraints([Constraint::Percentage(100)].as_ref())
                    .margin(5)
                    .split(f.size());

                let selected_style = Style::default().add_modifier(Modifier::REVERSED);
                let normal_style = Style::default().bg(Color::Blue);
                let header_cells = ["Header1", "Header2", "Header3"]
                    .iter()
                    .map(|h| Cell::from(*h).style(Style::default().fg(Color::Red)));
                let header = Row::new(header_cells)
                    .style(normal_style)
                    .height(1)
                    .bottom_margin(1);
                let rows = self.items.iter().map(|item| {
                    let height = item
                        .iter()
                        .map(|content| content.chars().filter(|c| *c == '\n').count())
                        .max()
                        .unwrap_or(0)
                        + 1;
                    let cells = item.iter().map(|c| Cell::from(*c));
                    Row::new(cells).height(height as u16).bottom_margin(1)
                });
                let t = Table::new(rows)
                    .header(header)
                    .block(Block::default().borders(Borders::ALL).title("Table"))
                    .highlight_style(selected_style)
                    .highlight_symbol(">> ")
                    .widths(&[
                        Constraint::Percentage(50),
                        Constraint::Length(30),
                        Constraint::Max(10),
                    ]);
                f.render_widget(t, rects[0]);
            })?;
        }
        Ok(())
    }
}
