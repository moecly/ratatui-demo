use ratatui::layout::{Constraint, Layout, Rect};
use ratatui::style::{Color, Style};
use ratatui::text::Span;
use ratatui::widgets::{Block, Gauge, Paragraph, Tabs};
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

    match app.tabs.index {
        0 => {
            draw_first_tab(app, frame);
        }
        _ => {}
    }
}

fn draw_first_tab(app: &mut App, frame: &mut Frame) {
    let chunks = Layout::vertical([
        Constraint::Length(9),
        Constraint::Min(8),
        Constraint::Length(7),
    ])
    .split(frame.area());
    draw_gauges(app, frame, chunks[0]);
}

fn draw_gauges(app: &mut App, frame: &mut Frame, area: Rect) {
    let chunks = Layout::vertical([
        Constraint::Length(2),
        Constraint::Length(3),
        Constraint::Length(2),
    ])
    .margin(1)
    .split(area);

    let label = format!("{:.2}%", app.progress * 100.0);
    let gauge = Gauge::default()
        .label(label)
        .block(Block::new());
    frame.render_widget(gauge, area);
}
