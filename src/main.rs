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
                    if app.editing_node {
                        match key.code {
                            KeyCode::Esc => {
                                app.finish_node_editing();
                            }
                            KeyCode::Char(c) => {
                                if app.node_edit_buffer.len() < 25 {
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
                            KeyCode::Char('q') | KeyCode::Char('Q') => return Ok(()),
                            KeyCode::Char('r') | KeyCode::Char('R') => {
                                app.reset();
                            }
                            KeyCode::Tab => {
                                // total number of tabs = 4
                                app.current_tab = (app.current_tab + 1) % 4;
                            }
                            KeyCode::Right => {
                                if app.current_tab == 0 {
                                    app.focused_section = match app.focused_section {
                                        FocusedSection::WhiteBalls => FocusedSection::RedBalls,
                                        FocusedSection::RedBalls => FocusedSection::RandomMode,
                                        FocusedSection::RandomMode => FocusedSection::ForcedFour,
                                        FocusedSection::ForcedFour => FocusedSection::DrawInput,
                                        FocusedSection::DrawInput => FocusedSection::WhiteBalls,
                                    };
                                }
                            }
                            KeyCode::Left => {
                                if app.current_tab == 0 {
                                    app.focused_section = match app.focused_section {
                                        FocusedSection::WhiteBalls => FocusedSection::DrawInput,
                                        FocusedSection::RedBalls => FocusedSection::WhiteBalls,
                                        FocusedSection::RandomMode => FocusedSection::RedBalls,
                                        FocusedSection::ForcedFour => FocusedSection::RandomMode,
                                        FocusedSection::DrawInput => FocusedSection::ForcedFour,
                                    };
                                }
                            }
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
                                            if app.draw_count < 4 {
                                                app.draw_count += 1;
                                            }
                                        }
                                        _ => {}
                                    }
                                } else if app.current_tab == 3 {
                                    if app.vertical_scroll > 0 {
                                        app.vertical_scroll -= 1;
                                        app.vertical_scroll_state =
                                            app.vertical_scroll_state.position(app.vertical_scroll);
                                    }
                                } else if app.current_tab == 1 {
                                    if app.v_scroll_graph > 0 {
                                        app.v_scroll_graph -= 1;
                                        app.v_scroll_graph_state =
                                            app.v_scroll_graph_state.position(app.v_scroll_graph);
                                    }
                                }
                            }
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
                                            if app.draw_count > 1 {
                                                app.draw_count -= 1;
                                            }
                                        }
                                        _ => {}
                                    }
                                } else if app.current_tab == 3 {
                                    let max_scroll = (app.history.len() * 13).saturating_sub(10);
                                    if app.vertical_scroll < max_scroll {
                                        app.vertical_scroll += 1;
                                        app.vertical_scroll_state =
                                            app.vertical_scroll_state.position(app.vertical_scroll);
                                    }
                                } else if app.current_tab == 1 {
                                    let max_scroll = (app.v_scroll_graph).saturating_sub(10);
                                    if app.v_scroll_graph < max_scroll {
                                        app.v_scroll_graph += 1;
                                        app.v_scroll_graph_state =
                                            app.v_scroll_graph_state.position(app.v_scroll_graph);
                                    }
                                }
                            }
                            KeyCode::Enter => {
                                if app.current_tab == 0 {
                                    if app.focused_section == FocusedSection::DrawInput
                                        && !app.first_draw_complete
                                    {
                                        app.popup = PopupType::ConfirmDraw;
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
                                }
                                if app.current_tab == 1 && app.selected_node.is_some() {
                                    app.start_node_editing();
                                }
                            }
                            _ => {}
                        }
                    }
                }
            }
            Event::Mouse(mouse) => {
                if app.popup != PopupType::None {
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
