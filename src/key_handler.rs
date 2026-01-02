use crossterm::event::{self, Event, KeyCode, KeyEventKind, MouseButton, MouseEventKind};

// include module ui.rs
use crate::app::{App, FocusedSection, ListSection, PopupType, TabType};
use std::io;

// return
pub fn handle_key_press(app: &mut App) -> io::Result<bool> {
    match event::read()? {
        // Controlla se evento e' un KeyDown non un KeyRelease
        Event::Key(key) => {
            if key.kind == KeyEventKind::Press {
                match key.code {
                    KeyCode::Esc => {
                        if app.editing_node {
                            app.finish_node_editing();
                        } else if app.editing_list_item {
                            app.finish_list_editing();
                        } else if app.popup == PopupType::ConfirmRisk {
                            app.cancel_draw();
                        }
                        app.popup = PopupType::None;
                    }
                    // quit or reset
                    KeyCode::Char('q') | KeyCode::Char('Q') => return Ok(true),
                    KeyCode::Char('r') | KeyCode::Char('R') => {
                        app.reset();
                    }
                    KeyCode::Char('e') | KeyCode::Char('E') => {
                        // select trait to use as token for next draw
                        match app.current_tab {
                            TabType::CharacterSheetTab => {
                                if let Some(idx) = app.selected_node {
                                    // ignore empty node
                                    if app.honeycomb_nodes[idx].text.is_empty() {
                                        return Ok(false);
                                    }
                                    // check if not used then push and add token, otherwise remove and remove token
                                    if app.used_traits.contains(&idx) {
                                        let _ = app.used_traits.swap_remove(
                                            app.used_traits.iter().position(|n| *n == idx).unwrap(),
                                        );
                                        app.white_balls -= 1;
                                    } else {
                                        app.used_traits.push(idx);
                                        app.white_balls += 1;
                                    }
                                }
                            }
                            TabType::AdditionalInfoTab => {
                                if let Some((section, idx)) = app.selected_list_item {
                                    match section {
                                        ListSection::Misfortunes
                                        | ListSection::MisfortunesDifficult => {
                                            // if misfortune is empty, ignore it
                                            if app.list_data.misfortunes[idx].is_empty() {
                                                return Ok(false);
                                            }
                                            let value = &app.list_data.misfortunes_red_balls[idx]
                                                .trim()
                                                .parse::<usize>()
                                                .unwrap_or(0); // obtain 0 if NaN
                                            // check if not used then push and add token, otherwise remove token
                                            if app.additional_red_balls[idx] != 0 {
                                                app.red_balls -= app.additional_red_balls[idx];
                                                app.additional_red_balls[idx] = 0;
                                            } else {
                                                app.additional_red_balls[idx] = *value;
                                                app.red_balls += app.additional_red_balls[idx];
                                            }
                                        }
                                        _ => {}
                                    }
                                }
                            }
                            _ => {}
                        }
                    }
                    KeyCode::Char(c) => {
                        if app.editing_node && app.node_edit_buffer.len() < 35 {
                            app.node_edit_buffer.push(c);
                        } else if app.editing_list_item
                            && app.list_edit_buffer.len()
                                < app.selected_list_item.unwrap().0.item_length()
                        {
                            app.list_edit_buffer.push(c);
                        }
                    }
                    KeyCode::Backspace => {
                        if app.editing_node {
                            app.node_edit_buffer.pop();
                        } else if app.editing_list_item {
                            app.list_edit_buffer.pop();
                        }
                    }
                    KeyCode::Enter => {
                        match app.popup {
                            PopupType::ConfirmDraw => {
                                app.perform_first_draw();
                            }
                            PopupType::ConfirmRisk => {
                                app.perform_risk_draw();
                            }
                            PopupType::None => {
                                match app.current_tab {
                                    TabType::DrawTab => {
                                        match app.focused_section {
                                            FocusedSection::DrawInput => {
                                                // perform draw iff there are tokens to be drawn
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
                                    TabType::CharacterSheetTab => {
                                        // this section do not need to handle '\n' character
                                        if app.selected_node.is_some() && !app.editing_node {
                                            app.start_node_editing();
                                        }
                                    }
                                    TabType::AdditionalInfoTab => {
                                        if app.editing_list_item {
                                            if app.list_edit_buffer.len()
                                                < app.selected_list_item.unwrap().0.item_length()
                                            {
                                                app.list_edit_buffer.push('\n');
                                            }
                                        } else if app.selected_list_item.is_some() {
                                            app.start_list_editing();
                                        }
                                    }
                                    _ => {}
                                }
                            }
                        }
                    }
                    // moving through tab
                    KeyCode::Tab => {
                        // total number of tabs = 4
                        app.current_tab = app.current_tab.next();
                    }
                    // moving through element of tab
                    KeyCode::Right => match app.current_tab {
                        TabType::DrawTab => {
                            app.focused_section = app.focused_section.next();
                        }
                        TabType::CharacterSheetTab => {
                            if let Some(idx) = app.selected_node {
                                match idx {
                                    0..7 | 11..13 => {
                                        app.selected_node = Some(idx + 4);
                                    }
                                    7..11 => {
                                        app.selected_node = Some(idx + 5);
                                    }
                                    13..16 => {
                                        app.selected_node = Some(idx + 3);
                                    }
                                    16..19 => {
                                        app.selected_node = Some(idx - 16);
                                    }
                                    _ => {}
                                }
                            }
                        }
                        TabType::AdditionalInfoTab => {
                            if let Some((section, idx)) = app.selected_list_item {
                                match section {
                                    ListSection::Misfortunes
                                    | ListSection::MisfortunesDifficult => {
                                        if idx < 3 {
                                            app.selected_list_item = Some((section, idx + 1));
                                        } else {
                                            app.selected_list_item = Some((section.next(), 0));
                                        }
                                    }
                                    ListSection::LxResources | ListSection::RxResources => {
                                        app.selected_list_item = Some((section.next(), 0));
                                    }
                                    ListSection::Lessons => {
                                        if idx < 2 {
                                            app.selected_list_item = Some((section, idx + 1));
                                        } else {
                                            app.selected_list_item = Some((section.next(), 0));
                                        }
                                    }
                                }
                            }
                        }
                        _ => {}
                    },
                    // moving through element of tab
                    KeyCode::Left => match app.current_tab {
                        TabType::DrawTab => {
                            app.focused_section = app.focused_section.prev();
                        }
                        TabType::CharacterSheetTab => {
                            if let Some(idx) = app.selected_node {
                                match idx {
                                    6..8 | 12..19 => {
                                        app.selected_node = Some(idx - 4);
                                    }
                                    8..12 => {
                                        app.selected_node = Some(idx - 5);
                                    }
                                    3..6 => {
                                        app.selected_node = Some(idx - 3);
                                    }
                                    0..3 => {
                                        app.selected_node = Some(idx + 16);
                                    }
                                    _ => {}
                                }
                            }
                        }
                        TabType::AdditionalInfoTab => {
                            if let Some((section, idx)) = app.selected_list_item {
                                match section {
                                    ListSection::Misfortunes => {
                                        if idx > 0 {
                                            app.selected_list_item = Some((section, idx - 1));
                                        } else {
                                            app.selected_list_item = Some((section.prev(), 2));
                                        }
                                    }
                                    ListSection::MisfortunesDifficult => {
                                        if idx > 0 {
                                            app.selected_list_item = Some((section, idx - 1));
                                        } else {
                                            app.selected_list_item = Some((section.prev(), 3));
                                        }
                                    }
                                    ListSection::LxResources => {
                                        app.selected_list_item = Some((section.prev(), 3));
                                    }
                                    ListSection::RxResources => {
                                        app.selected_list_item = Some((section.prev(), 0));
                                    }
                                    ListSection::Lessons => {
                                        if idx > 0 {
                                            app.selected_list_item = Some((section, idx - 1));
                                        } else {
                                            app.selected_list_item = Some((section.prev(), 0));
                                        }
                                    }
                                }
                            }
                        }
                        _ => {}
                    },
                    // editing first tab tokens
                    KeyCode::Up => {
                        match app.current_tab {
                            TabType::DrawTab => {
                                match app.focused_section {
                                    FocusedSection::WhiteBalls => {
                                        // 20 token as hard cap
                                        if app.white_balls < 20 {
                                            app.white_balls += 1;
                                        }
                                    }
                                    FocusedSection::RedBalls => {
                                        // 20 token as hard cap
                                        if app.red_balls < 20 {
                                            app.red_balls += 1;
                                        }
                                    }
                                    FocusedSection::DrawInput => {
                                        if app.draw_count < 4 && !app.forced_four_mode {
                                            app.draw_count += 1;
                                        }
                                    }
                                    _ => {}
                                }
                            }
                            TabType::CharacterSheetTab => {
                                if let Some(idx) = app.selected_node {
                                    match idx {
                                        // first column
                                        0..3 => {
                                            if idx == 0 {
                                                app.selected_node = Some(2);
                                            } else {
                                                app.selected_node = Some(idx - 1);
                                            }
                                        }
                                        // second column
                                        3..7 => {
                                            if idx == 3 {
                                                app.selected_node = Some(6);
                                            } else {
                                                app.selected_node = Some(idx - 1);
                                            }
                                        }
                                        // third column
                                        7..12 => {
                                            if idx == 7 {
                                                app.selected_node = Some(11);
                                            } else {
                                                app.selected_node = Some(idx - 1);
                                            }
                                        }
                                        // fourth column
                                        12..16 => {
                                            if idx == 12 {
                                                app.selected_node = Some(15);
                                            } else {
                                                app.selected_node = Some(idx - 1);
                                            }
                                        }
                                        // fifth column
                                        16..19 => {
                                            if idx == 16 {
                                                app.selected_node = Some(18);
                                            } else {
                                                app.selected_node = Some(idx - 1);
                                            }
                                        }
                                        _ => {}
                                    }
                                }
                            }
                            TabType::AdditionalInfoTab => {
                                if let Some((section, idx)) = app.selected_list_item {
                                    match section {
                                        ListSection::Misfortunes
                                        | ListSection::MisfortunesDifficult => {
                                            app.selected_list_item = Some((section.vertical(), idx))
                                        }
                                        ListSection::LxResources | ListSection::RxResources => {
                                            if idx > 0 {
                                                app.selected_list_item = Some((section, idx - 1));
                                            } else {
                                                app.selected_list_item = Some((section, 4 - idx));
                                            }
                                        }
                                        ListSection::Lessons => {
                                            app.list_vertical_scroll[idx] =
                                                app.list_vertical_scroll[idx].saturating_sub(1);
                                            app.list_vertical_scroll_state[idx] = app
                                                .list_vertical_scroll_state[idx]
                                                .position(app.list_vertical_scroll[idx]);
                                        }
                                    }
                                }
                            }
                            TabType::LogTab => {
                                app.vertical_scroll = app.vertical_scroll.saturating_sub(1);
                                app.vertical_scroll_state =
                                    app.vertical_scroll_state.position(app.vertical_scroll);
                            }
                            _ => {}
                        }
                    }
                    // editing first tab tokens
                    KeyCode::Down => {
                        match app.current_tab {
                            TabType::DrawTab => {
                                match app.focused_section {
                                    FocusedSection::WhiteBalls => {
                                        if app.white_balls > 0 {
                                            app.white_balls -= 1;
                                            // pop first trait if present. do't care which one
                                            if !app.used_traits.is_empty() {
                                                let _ = app.used_traits.pop();
                                            }
                                        }
                                    }
                                    FocusedSection::RedBalls => {
                                        if app.red_balls > 0 {
                                            if app.red_balls > app.additional_red_balls.iter().sum()
                                            {
                                                app.red_balls -= 1;
                                            }
                                        }
                                    }
                                    FocusedSection::DrawInput => {
                                        if app.draw_count > 1 && !app.forced_four_mode {
                                            app.draw_count -= 1;
                                        }
                                    }
                                    _ => {}
                                }
                            }
                            TabType::CharacterSheetTab => {
                                if let Some(idx) = app.selected_node {
                                    match idx {
                                        // first column
                                        0..3 => {
                                            app.selected_node = Some((idx + 1) % 3);
                                        }
                                        // second column
                                        3..7 => {
                                            if idx == 6 {
                                                app.selected_node = Some(3);
                                            } else {
                                                app.selected_node = Some(idx + 1);
                                            }
                                        }
                                        // third column
                                        7..12 => {
                                            if idx == 11 {
                                                app.selected_node = Some(7);
                                            } else {
                                                app.selected_node = Some(idx + 1);
                                            }
                                        }
                                        // fourth column
                                        12..16 => {
                                            if idx == 15 {
                                                app.selected_node = Some(12);
                                            } else {
                                                app.selected_node = Some(idx + 1);
                                            }
                                        }
                                        // fifth column
                                        16..19 => {
                                            if idx == 18 {
                                                app.selected_node = Some(16);
                                            } else {
                                                app.selected_node = Some(idx + 1);
                                            }
                                        }
                                        _ => {}
                                    }
                                }
                            }
                            TabType::AdditionalInfoTab => {
                                if let Some((section, idx)) = app.selected_list_item {
                                    match section {
                                        ListSection::Misfortunes
                                        | ListSection::MisfortunesDifficult => {
                                            app.selected_list_item = Some((section.vertical(), idx))
                                        }
                                        ListSection::LxResources | ListSection::RxResources => {
                                            app.selected_list_item = Some((section, (idx + 1) % 5));
                                        }
                                        ListSection::Lessons => {
                                            if app.list_vertical_scroll[idx]
                                                < app.list_data.lessons[idx].len()
                                                    / app.lections_area[idx].width as usize
                                            {
                                                app.list_vertical_scroll[idx] =
                                                    app.list_vertical_scroll[idx].saturating_add(1);
                                                app.list_vertical_scroll_state[idx] = app
                                                    .list_vertical_scroll_state[idx]
                                                    .position(app.list_vertical_scroll[idx]);
                                            }
                                        }
                                    }
                                }
                            }
                            TabType::LogTab => {
                                if app.vertical_scroll < app.history.len() * 13 {
                                    app.vertical_scroll = app.vertical_scroll.saturating_add(1);
                                    app.vertical_scroll_state =
                                        app.vertical_scroll_state.position(app.vertical_scroll);
                                }
                            }
                            _ => {}
                        }
                    }
                    _ => {}
                }
            }
        }

        Event::Mouse(mouse) => {
            if app.popup != PopupType::None || app.editing_node || app.editing_list_item {
                return Ok(false);
            }

            match mouse.kind {
                MouseEventKind::Down(MouseButton::Left) => {
                    app.handle_mouse_click(mouse.column, mouse.row);
                    // Also check for node clicks in graph tab
                    if app.current_tab == TabType::CharacterSheetTab {
                        app.handle_node_click(mouse.column, mouse.row, &app.graph_area.clone());
                    }
                }
                _ => {}
            }
        }
        _ => {}
    }

    Ok(false)
}
