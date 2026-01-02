use crossterm::{
    event::{DisableMouseCapture, EnableMouseCapture},
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};
use std::io;

// include module ui.rs
mod app;
use crate::app::App;

mod ui;
use crate::ui::ui;

mod key_handler;
use crate::key_handler::handle_key_press;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // when enablebled raw mode:
    // Input will not be forwarded to screen
    // Input will not be processed on enter press
    // Input will not be line buffered (input sent byte-by-byte to input buffer)
    // Special keys like backspace and CTRL+C will not be processed by terminal driver
    // New line character will not be processed therefore println! can’t be used, use write! instead
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let mut app = App::new();

    ratatui::run(|terminal| {
        loop {
            let _ = terminal.draw(|frame| ui(frame, &mut app));
            // should be handled with a '?' operator
            if handle_key_press(&mut app).unwrap() {
                // disable raw mode and make console print normally
                disable_raw_mode()?;
                execute!(
                    terminal.backend_mut(),
                    LeaveAlternateScreen,
                    DisableMouseCapture
                )?;
                terminal.show_cursor()?;
                // if return value is true then 'Q' or 'q' key has been pressed and application need to quit
                break Ok(());
            }
        }
    })

    // disable_raw_mode()?;
    // Ok(())
}

#[cfg(test)]
mod tests {
    // cargo insta test --review
    use crate::App;
    use crate::ui::ui;
    use insta::assert_snapshot;
    use ratatui::{Terminal, backend::TestBackend};

    #[test]
    fn test_render_app() {
        let mut app = App::new();
        let mut terminal = Terminal::new(TestBackend::new(100, 40)).unwrap();
        // create and run your app/widget here
        terminal.draw(|frame| ui(frame, &mut app)).unwrap();
        assert_snapshot!(terminal.backend());
    }
}
