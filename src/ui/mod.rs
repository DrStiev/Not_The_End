use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout},
};

use crate::app::{App, PopupType, TabType};

// Moduli UI
mod components;
mod tabs;
mod utils;

/// Funzione principale di rendering dell'interfaccia
pub fn ui(f: &mut Frame, app: &mut App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(3), Constraint::Min(0)])
        .split(f.area());

    // Renderizza la barra dei tab
    components::render_tabs_bar(f, chunks[0], app);

    // Renderizza il contenuto in base al tab selezionato
    render_tab_content(f, chunks[1], app);

    // Renderizza popup se presenti
    render_popups(f, app);
}

/// Renderizza il contenuto del tab corrente
fn render_tab_content(f: &mut Frame, area: ratatui::prelude::Rect, app: &mut App) {
    match app.current_tab {
        TabType::DrawTab => tabs::tab_draw::render(f, area, app),
        TabType::CharacterSheetTab => tabs::tab_character::render(f, area, app),
        TabType::AdditionalInfoTab => {
            tabs::tab_list::render(f, area, app);
            // Forza la scrollbar a essere visibile se la sezione ha testo
            ensure_scrollbars_visible(app);
        }
        TabType::LogTab => tabs::tab_log::render(f, area, app),
        _ => {}
    }
}

/// Assicura che le scrollbar siano visibili quando necessario
fn ensure_scrollbars_visible(app: &mut App) {
    if !app.list_data.notes.is_empty() {
        app.update_notes_vertical_scroll_state();
    }
    for i in 0..3 {
        if !app.list_data.lessons[i].is_empty() {
            app.update_list_vertical_scroll_state(i);
        }
    }
}

/// Renderizza i popup sovrapposti al contenuto
fn render_popups(f: &mut Frame, app: &App) {
    if app.popup != PopupType::None {
        // Popup di conferma estrazione/rischio
        components::render_draw_popup(f, app);
    } else if app.editing_node || app.editing_character_info {
        // Popup editing nodi o info personaggio
        components::render_node_edit_popup(f, app);
    } else if app.editing_list_item {
        // Popup editing liste
        components::render_list_edit_popup(f, app);
    }
}
