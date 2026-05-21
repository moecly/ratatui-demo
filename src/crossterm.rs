use std::time::Duration;
use std::time::Instant;

use color_eyre::eyre::Context;
use color_eyre::eyre::Ok;
use color_eyre::eyre::Result;
use crossterm::event;
use crossterm::event::KeyCode;
use ratatui::DefaultTerminal;

use crate::app::App;
use crate::ui;

pub fn run(terminal: &mut DefaultTerminal) -> Result<()> {
    let mut app = App::new();
    let tick_rate = Duration::from_millis(1);

    let mut last_tick = Instant::now();
    loop {
        terminal.draw(|frame| ui::render(frame, &mut app))?;

        let timeout = tick_rate.saturating_sub(last_tick.elapsed());
        if !event::poll(timeout)? {
            app.on_tick();
            last_tick = Instant::now();
            continue;;
        }

        if should_quit()? {
            break;
        }
    }
    Ok(())
}

fn should_quit() -> Result<bool> {
    if event::poll(Duration::from_millis(250)).context("event poll failed")? {
        let q_pressed = event::read()
            .context("event read failed")?
            .as_key_press_event()
            .is_some_and(|key| key.code == KeyCode::Char('q'));
        return Ok(q_pressed);
    }
    Ok(false)
}
