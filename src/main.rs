use crossterm::{
    event::{
        self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEventKind, MouseButton,
        MouseEventKind,
    },
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};
use ratatui::{Terminal, backend::CrosstermBackend};
use std::io;

// include module ui.rs
mod app;
use crate::app::{App, FocusedSection, PopupType};

mod ui;
use crate::ui::ui;

fn main() -> Result<(), io::Error> {
    // when enablebled raw mode:
    // Input will not be forwarded to screen
    // Input will not be processed on enter press
    // Input will not be line buffered (input sent byte-by-byte to input buffer)
    // Special keys like backspace and CTRL+C will not be processed by terminal driver
    // New line character will not be processed therefore println! can’t be used, use write! instead
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut app = App::new();
    let res = run_app(&mut terminal, &mut app);

    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        println!("Error: {:?}", err);
    }

    Ok(())
}

fn run_app<B: ratatui::backend::Backend>(
    terminal: &mut Terminal<B>,
    app: &mut App,
) -> io::Result<()> {
    loop {
        terminal.draw(|f| ui(f, app))?;

        match event::read()? {
            // Controlla se evento e' un KeyDown non un KeyRelease
            Event::Key(key) => {
                if key.kind == KeyEventKind::Press {
                    // editing honeycomb node
                    if app.editing_node {
                        match key.code {
                            KeyCode::Esc => {
                                app.finish_node_editing();
                            }
                            KeyCode::Char(c) => {
                                if app.node_edit_buffer.len() < 35 {
                                    app.node_edit_buffer.push(c);
                                }
                            }
                            KeyCode::Backspace => {
                                app.node_edit_buffer.pop();
                            }
                            _ => {}
                        }
                        continue;
                    }
                    if app.editing_list_item {
                        // editing 3rd tab
                        match key.code {
                            KeyCode::Esc => {
                                app.finish_list_editing();
                            }
                            KeyCode::Char(c) => {
                                let max_len = match app.selected_list_item {
                                    Some((0, _)) => 75,                 // Misfortunes
                                    Some((1, _)) | Some((2, _)) => 120, // Resources
                                    Some((3, _)) => 500,                // Lessons
                                    _ => 0,                             // others
                                };
                                if app.list_edit_buffer.len() < max_len {
                                    app.list_edit_buffer.push(c);
                                }
                            }
                            KeyCode::Enter => {
                                let max_len = match app.selected_list_item {
                                    Some((0, _)) => 75,                 // Misfortunes
                                    Some((1, _)) | Some((2, _)) => 120, // Resources
                                    Some((3, _)) => 500,                // Lessons
                                    _ => 0,                             // others
                                };
                                if app.list_edit_buffer.len() < max_len {
                                    app.list_edit_buffer.push('\n');
                                }
                            }
                            KeyCode::Backspace => {
                                app.list_edit_buffer.pop();
                            }
                            _ => {}
                        }
                        continue;
                    }
                    if app.popup != PopupType::None {
                        match key.code {
                            KeyCode::Enter => {
                                if app.popup == PopupType::ConfirmDraw {
                                    app.perform_first_draw();
                                } else if app.popup == PopupType::ConfirmRisk {
                                    app.perform_risk_draw();
                                }
                            }
                            KeyCode::Esc => {
                                if app.popup == PopupType::ConfirmRisk {
                                    app.cancel_draw();
                                }
                                app.popup = PopupType::None;
                            }
                            _ => {}
                        }
                    } else {
                        match key.code {
                            // quit or reset
                            KeyCode::Char('q') | KeyCode::Char('Q') => return Ok(()),
                            KeyCode::Char('r') | KeyCode::Char('R') => {
                                app.reset();
                            }
                            // moving through tab
                            KeyCode::Tab => {
                                // total number of tabs = 4
                                app.current_tab = (app.current_tab + 1) % 4;
                            }
                            // moving through element of tab
                            KeyCode::Right => {
                                if app.current_tab == 0 {
                                    app.focused_section = match app.focused_section {
                                        FocusedSection::WhiteBalls => FocusedSection::RedBalls,
                                        FocusedSection::RedBalls => FocusedSection::RandomMode,
                                        FocusedSection::RandomMode => FocusedSection::ForcedFour,
                                        FocusedSection::ForcedFour => FocusedSection::DrawInput,
                                        FocusedSection::DrawInput => FocusedSection::WhiteBalls,
                                    };
                                } else if app.current_tab == 2 {
                                    if let Some((section, idx)) = app.selected_list_item {
                                        match section {
                                            0 => {
                                                if idx < 3 {
                                                    app.selected_list_item =
                                                        Some((section, idx + 1));
                                                } else {
                                                    app.selected_list_item = Some((section + 1, 0));
                                                }
                                            }
                                            1 => {
                                                app.selected_list_item = Some((2, 0));
                                            }
                                            2 => {
                                                app.selected_list_item = Some((3, 0));
                                            }
                                            3 => {
                                                if idx < 2 {
                                                    app.selected_list_item =
                                                        Some((section, idx + 1));
                                                } else {
                                                    app.selected_list_item = Some((0, 0));
                                                }
                                            }
                                            _ => {}
                                        }
                                    }
                                } else if app.current_tab == 1 {
                                    if let Some(idx) = app.selected_node {
                                        match idx {
                                            0 | 1 | 2 | 4 | 5 | 6 | 11 | 12 | 3 => {
                                                app.selected_node = Some(idx + 4);
                                            }
                                            8 | 9 | 10 | 7 => {
                                                app.selected_node = Some(idx + 5);
                                            }
                                            13 | 14 | 15 => {
                                                app.selected_node = Some(idx + 3);
                                            }
                                            16 => {
                                                app.selected_node = Some(0);
                                            }
                                            17 => {
                                                app.selected_node = Some(1);
                                            }
                                            18 => {
                                                app.selected_node = Some(2);
                                            }
                                            _ => {}
                                        }
                                    }
                                }
                            }
                            // moving through element of tab
                            KeyCode::Left => {
                                if app.current_tab == 0 {
                                    app.focused_section = match app.focused_section {
                                        FocusedSection::WhiteBalls => FocusedSection::DrawInput,
                                        FocusedSection::RedBalls => FocusedSection::WhiteBalls,
                                        FocusedSection::RandomMode => FocusedSection::RedBalls,
                                        FocusedSection::ForcedFour => FocusedSection::RandomMode,
                                        FocusedSection::DrawInput => FocusedSection::ForcedFour,
                                    };
                                } else if app.current_tab == 2 {
                                    if let Some((section, idx)) = app.selected_list_item {
                                        match section {
                                            0 => {
                                                if idx > 0 {
                                                    app.selected_list_item =
                                                        Some((section, idx - 1));
                                                } else {
                                                    app.selected_list_item = Some((3, 2));
                                                }
                                            }
                                            1 => {
                                                app.selected_list_item = Some((0, 3));
                                            }
                                            2 => {
                                                app.selected_list_item = Some((1, 0));
                                            }
                                            3 => {
                                                if idx > 0 {
                                                    app.selected_list_item =
                                                        Some((section, idx - 1));
                                                } else {
                                                    app.selected_list_item = Some((2, 0));
                                                }
                                            }
                                            _ => {}
                                        }
                                    }
                                } else if app.current_tab == 1 {
                                    if let Some(idx) = app.selected_node {
                                        match idx {
                                            16 | 17 | 18 | 12 | 13 | 14 | 7 | 6 | 15 => {
                                                app.selected_node = Some(idx - 4);
                                            }
                                            8 | 9 | 10 | 11 => {
                                                app.selected_node = Some(idx - 5);
                                            }
                                            3 | 4 | 5 => {
                                                app.selected_node = Some(idx - 3);
                                            }
                                            0 => {
                                                app.selected_node = Some(16);
                                            }
                                            1 => {
                                                app.selected_node = Some(17);
                                            }
                                            2 => {
                                                app.selected_node = Some(18);
                                            }
                                            _ => {}
                                        }
                                    }
                                }
                            }
                            // editing first tab tokens
                            KeyCode::Up => {
                                if app.current_tab == 0 {
                                    match app.focused_section {
                                        FocusedSection::WhiteBalls => {
                                            if app.white_balls < 10 {
                                                app.white_balls += 1;
                                            }
                                        }
                                        FocusedSection::RedBalls => {
                                            if app.red_balls < 10 {
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
                                    // scroll logs
                                } else if app.current_tab == 3 {
                                    if app.vertical_scroll > 0 {
                                        app.vertical_scroll -= 1;
                                        app.vertical_scroll_state =
                                            app.vertical_scroll_state.position(app.vertical_scroll);
                                    }
                                    // move through list item of 3rd tab
                                } else if app.current_tab == 2 {
                                    if let Some((section, idx)) = app.selected_list_item {
                                        match section {
                                            1 | 2 => {
                                                if idx > 0 {
                                                    app.selected_list_item =
                                                        Some((section, idx - 1));
                                                } else {
                                                    app.selected_list_item =
                                                        Some((section, 4 - idx));
                                                }
                                            }
                                            _ => {}
                                        }
                                    }
                                } else if app.current_tab == 1 {
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
                            }
                            // editing first tab tokens
                            KeyCode::Down => {
                                if app.current_tab == 0 {
                                    match app.focused_section {
                                        FocusedSection::WhiteBalls => {
                                            if app.white_balls > 0 {
                                                app.white_balls -= 1;
                                            }
                                        }
                                        FocusedSection::RedBalls => {
                                            if app.red_balls > 0 {
                                                app.red_balls -= 1;
                                            }
                                        }
                                        FocusedSection::DrawInput => {
                                            if app.draw_count > 1 && !app.forced_four_mode {
                                                app.draw_count -= 1;
                                            }
                                        }
                                        _ => {}
                                    }
                                    // scroll logs
                                } else if app.current_tab == 3 {
                                    let max_scroll = (app.history.len() * 13).saturating_sub(10);
                                    if app.vertical_scroll < max_scroll {
                                        app.vertical_scroll += 1;
                                        app.vertical_scroll_state =
                                            app.vertical_scroll_state.position(app.vertical_scroll);
                                    }
                                    // move through list item of 3rd tab
                                } else if app.current_tab == 2 {
                                    if let Some((section, idx)) = app.selected_list_item {
                                        match section {
                                            1 | 2 => {
                                                app.selected_list_item =
                                                    Some((section, (idx + 1) % 5));
                                            }
                                            _ => {}
                                        }
                                    }
                                } else if app.current_tab == 1 {
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
                            }
                            KeyCode::Enter => {
                                if app.current_tab == 0 {
                                    if app.focused_section == FocusedSection::DrawInput
                                    // && !app.first_draw_complete
                                    {
                                        // perform draw iff there are tokens to be drawn
                                        if app.white_balls > 0 && app.red_balls > 0 {
                                            app.popup = PopupType::ConfirmDraw;
                                        }
                                    } else if app.focused_section == FocusedSection::ForcedFour {
                                        app.forced_four_mode = !app.forced_four_mode;
                                        if app.forced_four_mode {
                                            app.draw_count = 4;
                                        } else {
                                            app.draw_count = 1;
                                        }
                                    } else if app.focused_section == FocusedSection::RandomMode {
                                        app.random_mode = !app.random_mode;
                                    }
                                } else if app.current_tab == 1 && app.selected_node.is_some() {
                                    app.start_node_editing();
                                } else if app.current_tab == 2 && app.selected_list_item.is_some() {
                                    app.start_list_editing();
                                }
                            }
                            _ => {}
                        }
                    }
                }
            }
            Event::Mouse(mouse) => {
                if app.popup != PopupType::None || app.editing_node || app.editing_list_item {
                    continue;
                }

                match mouse.kind {
                    MouseEventKind::Down(MouseButton::Left) => {
                        app.handle_mouse_click(mouse.column, mouse.row);
                        // Also check for node clicks in graph tab
                        if app.current_tab == 1 {
                            app.handle_node_click(mouse.column, mouse.row, &app.graph_area.clone());
                        }
                    }
                    _ => {}
                }
            }
            _ => {}
        }
    }
}
