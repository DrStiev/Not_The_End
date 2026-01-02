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
                // differentiate from input popup key press and normal key press
                if !app.editing_node && !app.editing_list_item && app.popup == PopupType::None {
                    match key.code {
                        // if not inside an input popup then activate (R)eset and (Q)uit action
                        KeyCode::Char('q') | KeyCode::Char('Q') => return Ok(true),
                        KeyCode::Char('r') | KeyCode::Char('R') => {
                            app.reset();
                        }
                        // if not inside an input popup then activate (Tab) movement
                        KeyCode::Tab => {
                            // total number of tabs = 4
                            app.current_tab = app.current_tab.next();
                        }
                        // if not inside an input popup then (Enter) input popup
                        KeyCode::Enter => {
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
                        // if not inside an input popup then activate (E)nable selection action
                        KeyCode::Char('e') | KeyCode::Char('E') => {
                            match app.current_tab {
                                // (E)nable trait
                                TabType::CharacterSheetTab => {
                                    if let Some(idx) = app.selected_node {
                                        // ignore empty node
                                        if app.honeycomb_nodes[idx].text.is_empty() {
                                            return Ok(false);
                                        }
                                        // check if not used then push and add token, otherwise remove and remove token
                                        if app.used_traits.contains(&idx) {
                                            let _ = app.used_traits.swap_remove(
                                                app.used_traits
                                                    .iter()
                                                    .position(|n| *n == idx)
                                                    .unwrap(),
                                            );
                                            app.white_balls -= 1;
                                        } else {
                                            app.used_traits.push(idx);
                                            app.white_balls += 1;
                                        }
                                    }
                                }
                                // (E)nable misfortune
                                TabType::AdditionalInfoTab => {
                                    if let Some((section, idx)) = app.selected_list_item {
                                        match section {
                                            ListSection::Misfortunes
                                            | ListSection::MisfortunesDifficult => {
                                                // if misfortune is empty, ignore it
                                                if app.list_data.misfortunes[idx].is_empty() {
                                                    return Ok(false);
                                                }
                                                let value = &app.list_data.misfortunes_red_balls
                                                    [idx]
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
                        // if not inside popup input then activate arrows navigation (↑/↓ and ←/→)
                        KeyCode::Right => match app.current_tab {
                            TabType::DrawTab => {
                                app.focused_section = app.focused_section.next();
                            }
                            TabType::CharacterSheetTab => {
                                app.next_hex();
                            }
                            TabType::AdditionalInfoTab => {
                                app.next_section();
                            }
                            _ => {}
                        },
                        // moving through element of tab
                        KeyCode::Left => match app.current_tab {
                            TabType::DrawTab => {
                                app.focused_section = app.focused_section.prev();
                            }
                            TabType::CharacterSheetTab => {
                                app.prev_hex();
                            }
                            TabType::AdditionalInfoTab => {
                                app.prev_section();
                            }
                            _ => {}
                        },
                        // editing first tab tokens
                        KeyCode::Up => match app.current_tab {
                            TabType::DrawTab => {
                                app.increment_balls();
                            }
                            TabType::CharacterSheetTab => {
                                app.up_hex();
                            }
                            TabType::AdditionalInfoTab => {
                                app.up_section();
                            }
                            TabType::LogTab => {
                                app.vertical_scroll = app.vertical_scroll.saturating_sub(1);
                                app.vertical_scroll_state =
                                    app.vertical_scroll_state.position(app.vertical_scroll);
                            }
                            _ => {}
                        },
                        // editing first tab tokens
                        KeyCode::Down => match app.current_tab {
                            TabType::DrawTab => {
                                app.decrement_balls();
                            }
                            TabType::CharacterSheetTab => {
                                app.down_hex();
                            }
                            TabType::AdditionalInfoTab => {
                                app.down_section();
                            }
                            TabType::LogTab => {
                                if app.vertical_scroll < app.history.len() * 13 {
                                    app.vertical_scroll = app.vertical_scroll.saturating_add(1);
                                    app.vertical_scroll_state =
                                        app.vertical_scroll_state.position(app.vertical_scroll);
                                }
                            }
                            _ => {}
                        },
                        _ => {}
                    }
                } else {
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
                        KeyCode::Enter => match app.popup {
                            PopupType::ConfirmDraw => {
                                app.perform_first_draw();
                            }
                            PopupType::ConfirmRisk => {
                                app.perform_risk_draw();
                            }
                            _ => {}
                        },

                        _ => {}
                    }
                }
            }
        }

        // handle mouse click event
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
