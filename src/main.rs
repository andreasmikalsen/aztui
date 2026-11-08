use crate::app::app::App;
use crossterm::event;
use crossterm::event::{Event, KeyEventKind};
use crate::app::worker::process_messages;

mod app;
mod azure;
mod ui;
mod domain;

fn main() -> std::io::Result<()> {
    ratatui::run(|terminal| {
        let mut app = App::new().map_err(|e| std::io::Error::other(e.to_string()))?;

        while app.running {
            process_messages(&mut app);
            terminal.draw(|frame| app.render(frame))?;

            if event::poll(std::time::Duration::from_millis(100))? {
                if let Event::Key(key) = event::read()? {
                    if key.kind == KeyEventKind::Press {
                        app.handle_key(key)
                    }
                }
            }
        }

        Ok(())
    })
}
