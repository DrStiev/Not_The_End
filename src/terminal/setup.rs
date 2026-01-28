use crossterm::{
    event::EnableMouseCapture,
    execute,
    terminal::{EnterAlternateScreen, enable_raw_mode},
};
use std::io;

/// Inizializza il terminale per l'applicazione TUI
///
/// Abilita:
/// - Raw mode (input byte-by-byte, nessun echo)
/// - Schermo alternativo
/// - Cattura mouse
pub fn setup() -> io::Result<()> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    Ok(())
}

// #[cfg(test)]
// mod tests {
//     use super::*;

//     // Test terminal
//     #[test]
//     fn test_setup_cleanup() {
//         assert!(setup().is_ok());
//         // Manual cleanup needed in real terminal
//     }
// }
