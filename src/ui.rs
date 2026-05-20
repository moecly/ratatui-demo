use ratatui::layout::{Constraint, Layout};
use ratatui::style::{Color, Style};
use ratatui::text::Span;
use ratatui::widgets::{Block, Paragraph, Tabs};
use ratatui::{Frame, text};

use crate::app::App;

pub fn render(frame: &mut Frame, app: &mut App) {
    let chunks = Layout::vertical([Constraint::Length(3), Constraint::Min(0)]).split(frame.area());
    let tabs = app
        .tabs
        .titles
        .iter()
        .map(|t| text::Line::from(Span::styled(*t, Style::default().fg(Color::Green))))
        .collect::<Tabs>()
        .block(Block::bordered().title(app.title))
        .highlight_style(Style::default().fg(Color::Yellow))
        .select(app.tabs.index);

    frame.render_widget(tabs, chunks[0]);
}

fn draw_first_tab(frame: &mut Frame) {
    let chunks = Layout::vertical([
        Constraint::Length(9),
        Constraint::Min(8),
        Constraint::Length(7),
    ]).split(frame.area());
}
