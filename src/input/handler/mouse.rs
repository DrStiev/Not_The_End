use crossterm::event::{MouseButton, MouseEvent, MouseEventKind};

use crate::app::{App, PopupType, TabType};

/// Gestisce gli eventi del mouse
pub fn handle_mouse_event(app: &mut App, mouse: MouseEvent) {
    // Ignora eventi mouse se siamo in editing o popup
    if app.popup != PopupType::None
        || app.editing_node
        || app.editing_list_item
        || app.editing_character_info
    {
        return;
    }

    // Gestisci solo click sinistro
    if let MouseEventKind::Down(MouseButton::Left) = mouse.kind {
        handle_left_click(app, mouse.column, mouse.row);
    }
}

/// Gestisce il click sinistro del mouse
fn handle_left_click(app: &mut App, x: u16, y: u16) {
    // Gestione click generale (tab, sezioni comuni)
    app.handle_mouse_click(x, y);

    // Gestione specifica per tab personaggio
    if app.current_tab == TabType::CharacterSheetTab {
        app.handle_character_click(x, y);
        app.handle_node_click(x, y);
    }
}
