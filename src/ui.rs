use ratatui::Frame;
use ratatui::widgets::Paragraph;

pub fn render(frame: &mut Frame) {
    let greeting = Paragraph::new("Hello World!");
    frame.render_widget(greeting, frame.area());
}
