use crossterm::{
    event::DisableMouseCapture,
    execute,
    terminal::{LeaveAlternateScreen, disable_raw_mode},
};
use ratatui::{Terminal, prelude::Backend};
use std::io;

/// Ripristina il terminale allo stato normale
///
/// Disabilita:
/// - Raw mode
/// - Schermo alternativo
/// - Cattura mouse
/// - Mostra il cursore
pub fn cleanup<B: ratatui::backend::Backend + std::io::Write>(
    terminal: &mut Terminal<B>,
) -> io::Result<()>
where
    std::io::Error: From<<B as Backend>::Error>,
{
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;
    Ok(())
}
