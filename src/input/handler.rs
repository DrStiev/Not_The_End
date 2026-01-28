use crossterm::event::{self, Event, KeyEventKind};
use std::io;

use crate::app::{App, PopupType};

mod editing;
mod keyboard;
mod mouse;

/// Gestisce tutti gli eventi di input (tastiera, mouse)
/// Ritorna `true` se l'applicazione deve terminare
pub fn handle_input(app: &mut App) -> io::Result<bool> {
    match event::read()? {
        Event::Key(key) => {
            // Considera solo eventi di pressione (ignora rilascio)
            if key.kind != KeyEventKind::Press {
                return Ok(false);
            }

            // Determina se siamo in modalità editing/popup
            let is_editing = app.editing_node
                || app.editing_list_item
                || app.editing_character_info
                || app.popup != PopupType::None;

            if is_editing {
                editing::handle_editing_mode(app, key);
                Ok(false)
            } else {
                Ok(keyboard::handle_normal_mode(app, key))
            }
        }
        Event::Mouse(mouse) => {
            mouse::handle_mouse_event(app, mouse);
            Ok(false)
        }
        _ => Ok(false),
    }
}
