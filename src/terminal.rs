use std::{
    io,
    time::{Duration, Instant},
};

use crossterm::event::{self, Event, KeyEventKind};

use ratatui::DefaultTerminal;

use crate::{app::App, ui};

const FRAME_DURATION: Duration = Duration::from_millis(16);

pub fn run() -> io::Result<()> {
    let mut terminal = ratatui::init();

    let result = run_loop(&mut terminal);

    ratatui::restore();

    result
}

fn run_loop(terminal: &mut DefaultTerminal) -> io::Result<()> {
    let mut app = App::new();

    let mut last_frame = Instant::now();

    terminal.draw(|frame| {
        ui::draw(frame, &app);
    })?;

    while app.running {
        let timeout = FRAME_DURATION.saturating_sub(last_frame.elapsed());

        if event::poll(timeout)? {
            if let Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press || key.kind == KeyEventKind::Repeat {
                    app.handle_key(key);
                }
            }
        }

        if !app.running {
            break;
        }

        if last_frame.elapsed() >= FRAME_DURATION {
            app.update();

            terminal.draw(|frame| {
                ui::draw(frame, &app);
            })?;

            last_frame = Instant::now();
        }
    }

    Ok(())
}
