use crossterm::event::{KeyCode, KeyEvent};

use crate::app::{
    App, CharacterSection, FocusedSection, ListSection, MAX_DRAW, MIN_DRAW, PopupType, TabType,
};

/// Gestisce gli eventi della tastiera quando non si è in modalità editing/popup
pub fn handle_normal_mode(app: &mut App, key: KeyEvent) -> bool {
    match key.code {
        KeyCode::Char('q') | KeyCode::Char('Q') => true, // Quit
        KeyCode::Char('r') | KeyCode::Char('R') => {
            app.reset();
            false
        }
        KeyCode::Tab => {
            app.current_tab = app.current_tab.next();
            false
        }
        KeyCode::Enter => {
            handle_enter_key(app);
            false
        }
        KeyCode::Char('e') | KeyCode::Char('E') => {
            handle_enable_key(app);
            false
        }
        KeyCode::Right => {
            handle_right_arrow(app);
            false
        }
        KeyCode::Left => {
            handle_left_arrow(app);
            false
        }
        KeyCode::Up => {
            handle_up_arrow(app);
            false
        }
        KeyCode::Down => {
            handle_down_arrow(app);
            false
        }
        _ => false,
    }
}

/// Gestisce il tasto Enter in modalità normale
fn handle_enter_key(app: &mut App) {
    match app.current_tab {
        TabType::DrawTab => handle_enter_draw_tab(app),
        TabType::CharacterSheetTab => handle_enter_character_tab(app),
        TabType::AdditionalInfoTab => {
            if app.selected_list_item.is_some() {
                app.start_list_editing();
            }
        }
        _ => {}
    }
}

/// Gestisce Enter nel tab di estrazione
fn handle_enter_draw_tab(app: &mut App) {
    match app.focused_section {
        FocusedSection::DrawInput => {
            if app.white_balls > 0 && app.red_balls > 0 {
                app.popup = PopupType::ConfirmDraw;
            }
        }
        FocusedSection::ForcedFour => {
            app.forced_four_mode = !app.forced_four_mode;
            if app.forced_four_mode {
                app.draw_count = 4;
            } else {
                app.draw_count = 1;
            }
        }
        FocusedSection::RandomMode => {
            app.random_mode = !app.random_mode;
        }
        _ => {}
    }
}

/// Gestisce Enter nel tab personaggio
fn handle_enter_character_tab(app: &mut App) {
    if app.selected_character_info != CharacterSection::None && !app.editing_character_info {
        app.start_character_editing();
    } else if app.selected_node.is_some()
        && !app.editing_node
        && app.selected_character_info == CharacterSection::None
    {
        app.start_node_editing();
    }
}

/// Gestisce il tasto 'E' per abilitare tratti/sfortune
fn handle_enable_key(app: &mut App) {
    match app.current_tab {
        TabType::CharacterSheetTab => handle_enable_trait(app),
        TabType::AdditionalInfoTab => handle_enable_misfortune(app),
        TabType::DrawTab => handle_enable_status(app),
        _ => {}
    }
}

/// Abilita/disabilita un tratto
fn handle_enable_trait(app: &mut App) {
    if let Some(idx) = app.selected_node {
        // Ignora nodi vuoti
        if app.honeycomb_nodes[idx].text.is_empty() {
            return;
        }

        // Toggle del tratto
        if app.used_traits.contains(&idx) {
            let pos = app.used_traits.iter().position(|n| *n == idx).unwrap();
            app.used_traits.swap_remove(pos);
            app.white_balls -= 1;
        } else {
            app.used_traits.push(idx);
            app.white_balls += 1;
        }
    }
}

/// Abilita/disabilita una sfortunata
fn handle_enable_misfortune(app: &mut App) {
    if let Some((ListSection::Misfortunes | ListSection::MisfortunesDifficult, idx)) =
        app.selected_list_item
    {
        // Ignora sfortune vuote
        if app.list_data.misfortunes[idx].is_empty() {
            return;
        }

        let value = app.list_data.misfortunes_red_balls[idx]
            .trim()
            .parse::<usize>()
            .unwrap_or(0);

        // Toggle della sfortuna
        if app.additional_red_balls[idx] != 0 {
            app.red_balls -= app.additional_red_balls[idx];
            app.additional_red_balls[idx] = 0;
        } else {
            app.additional_red_balls[idx] = value;
            app.red_balls += value;
        }
    }
}

/// Abilita/disabilita uno status
fn handle_enable_status(app: &mut App) {
    if app.focused_section == FocusedSection::RandomMode {
        app.random_mode = !app.random_mode;
    } else if app.focused_section == FocusedSection::ForcedFour {
        app.forced_four_mode = !app.forced_four_mode;
        if app.forced_four_mode {
            app.draw_count = MAX_DRAW;
        } else {
            app.draw_count = MIN_DRAW;
        }
    }
}

/// Gestisce la freccia destra
fn handle_right_arrow(app: &mut App) {
    match app.current_tab {
        TabType::DrawTab => {
            app.focused_section = app.focused_section.next();
        }
        TabType::CharacterSheetTab => {
            if app.selected_character_info != CharacterSection::None {
                app.selected_character_info = app.selected_character_info.next();
            } else {
                app.next_hex();
            }
        }
        TabType::AdditionalInfoTab => {
            app.next_section();
        }
        _ => {}
    }
}

/// Gestisce la freccia sinistra
fn handle_left_arrow(app: &mut App) {
    match app.current_tab {
        TabType::DrawTab => {
            app.focused_section = app.focused_section.prev();
        }
        TabType::CharacterSheetTab => {
            if app.selected_character_info != CharacterSection::None {
                app.selected_character_info = app.selected_character_info.next();
            } else {
                app.prev_hex();
            }
        }
        TabType::AdditionalInfoTab => {
            app.prev_section();
        }
        _ => {}
    }
}

/// Gestisce la freccia su
fn handle_up_arrow(app: &mut App) {
    match app.current_tab {
        TabType::DrawTab => {
            app.increment_balls();
        }
        TabType::CharacterSheetTab => {
            if app.selected_character_info == CharacterSection::None {
                app.up_hex();
            }
        }
        TabType::AdditionalInfoTab => {
            app.up_section();
        }
        TabType::LogTab => {
            app.vertical_scroll = app.vertical_scroll.saturating_sub(1);
            app.vertical_scroll_state = app.vertical_scroll_state.position(app.vertical_scroll);
        }
        _ => {}
    }
}

/// Gestisce la freccia giù
fn handle_down_arrow(app: &mut App) {
    match app.current_tab {
        TabType::DrawTab => {
            app.decrement_balls();
        }
        TabType::CharacterSheetTab => {
            if app.selected_character_info == CharacterSection::None {
                app.down_hex();
            }
        }
        TabType::AdditionalInfoTab => {
            app.down_section();
        }
        TabType::LogTab => {
            if app.vertical_scroll < app.history.len() * 13 {
                app.vertical_scroll = app.vertical_scroll.saturating_add(1);
                app.vertical_scroll_state = app.vertical_scroll_state.position(app.vertical_scroll);
            }
        }
        _ => {}
    }
}

#[cfg(test)]
mod tests {
    use crossterm::event::KeyModifiers;

    use super::*;

    // Test keyboard
    #[test]
    fn test_quit_key() {
        let mut app = App::new();
        let key = KeyEvent::new(KeyCode::Char('q'), KeyModifiers::empty());
        assert!(handle_normal_mode(&mut app, key));
    }

    #[test]
    fn test_reset_key() {
        let mut app = App::new();
        app.white_balls = 5;
        let key = KeyEvent::new(KeyCode::Char('r'), KeyModifiers::empty());
        handle_normal_mode(&mut app, key);
        assert_eq!(app.white_balls, 0);
    }

    #[test]
    fn test_enter_draw_tab() {
        let mut app = App::new();
        app.current_tab = TabType::DrawTab;
        app.focused_section = FocusedSection::DrawInput;
        app.white_balls = 5;
        app.red_balls = 3;

        handle_enter_draw_tab(&mut app);

        assert_eq!(app.popup, PopupType::ConfirmDraw);
    }

    #[test]
    fn test_enable_trait() {
        let mut app = App::new();
        app.selected_node = Some(9);
        app.honeycomb_nodes[9].text = "Coraggioso".to_string();

        handle_enable_trait(&mut app);

        assert!(app.used_traits.contains(&9));
        assert_eq!(app.white_balls, 1);
    }
}
