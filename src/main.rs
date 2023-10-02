#[macro_use]
extern crate log;

use std::io;
use tosbuilder::app::{App, AppResult, Routes};
use tosbuilder::event::{Event, EventHandler};
use tosbuilder::handler::handle_key_events;
use tosbuilder::tui::Tui;
use tui::backend::CrosstermBackend;
use tui::Terminal;
use tosbuilder::utils::logger;

fn main() -> AppResult<()> {
    logger::init_logger();
    info!("[TOS Builder] Starting TOS Builder...");

    // Create an application.
    let mut app = App::new();
    // Set the application routes.
    app.set_routes(Routes::new(
        vec![
            "Project Select".to_string(),
            "TOS Download".to_string(),
            "TOS Config".to_string(),
            "AT Config".to_string(),
            "Make Config".to_string(),
        ],
        0,
    ));

    // Initialize the terminal user interface.
    let backend = CrosstermBackend::new(io::stderr());
    let terminal = Terminal::new(backend)?;
    let events = EventHandler::new(250);
    let mut tui = Tui::new(terminal, events);
    tui.init()?;

    // Start the main loop.
    while app.running {
        // Render the user interface.
        tui.draw(&mut app)?;
        // Handle events.
        match tui.events.next()? {
            Event::Tick => app.tick(),
            Event::Key(key_event) => handle_key_events(key_event, &mut app)?,
            Event::Mouse(_) => {}
            Event::Resize(_, _) => {}
        }
    }

    // Exit the user interface.
    tui.exit()?;
    Ok(())
}
