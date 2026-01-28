use crossterm::event::{KeyCode, KeyEvent};

use crate::app::{App, PopupType};

/// Gestisce gli eventi della tastiera in modalità editing/popup
pub fn handle_editing_mode(app: &mut App, key: KeyEvent) {
    match key.code {
        KeyCode::Esc => handle_escape(app),
        KeyCode::Char(c) => handle_char_input(app, c),
        KeyCode::Backspace => handle_backspace(app),
        KeyCode::Enter => handle_enter_editing(app),
        _ => {}
    }
}

/// Gestisce il tasto Escape in modalità editing
fn handle_escape(app: &mut App) {
    if app.editing_node {
        app.finish_node_editing();
    } else if app.editing_list_item {
        app.finish_list_editing();
    } else if app.editing_character_info {
        app.finish_character_editing();
    } else if app.popup == PopupType::ConfirmRisk {
        app.cancel_draw();
    }
    app.popup = PopupType::None;
}

/// Gestisce l'input di caratteri
fn handle_char_input(app: &mut App, c: char) {
    if app.editing_node && app.node_edit_buffer.len() < 35 {
        app.node_edit_buffer.push(c);
    } else if app.editing_character_info
        && app.character_edit_buffer.len() < app.character_base_info.length()
    {
        app.character_edit_buffer.push(c);
    } else if app.editing_list_item
        && app.list_edit_buffer.len() < app.selected_list_item.unwrap().0.length()
    {
        app.list_edit_buffer.push(c);
    }
}

/// Gestisce il backspace
fn handle_backspace(app: &mut App) {
    if app.editing_node {
        app.node_edit_buffer.pop();
    } else if app.editing_character_info {
        app.character_edit_buffer.pop();
    } else if app.editing_list_item {
        app.list_edit_buffer.pop();
    }
}

/// Gestisce Enter in modalità editing
fn handle_enter_editing(app: &mut App) {
    match app.popup {
        PopupType::ConfirmDraw => {
            app.perform_first_draw();
        }
        PopupType::ConfirmRisk => {
            app.perform_risk_draw();
        }
        PopupType::None => {
            // In editing liste, permetti newline se c'è spazio
            if app.editing_list_item
                && app.list_edit_buffer.len() < app.selected_list_item.unwrap().0.length()
            {
                app.list_edit_buffer.push('\n');
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crossterm::event::KeyModifiers;

    use super::*;

    // Test editing
    #[test]
    fn test_char_input_node() {
        let mut app = App::new();
        app.editing_node = true;
        let key = KeyEvent::new(KeyCode::Char('a'), KeyModifiers::empty());
        handle_editing_mode(&mut app, key);
        assert_eq!(app.node_edit_buffer, "a");
    }
}
