use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    widgets::{Block, BorderType, Borders, Tabs},
};

use crate::app::{App, TabType, PopupType};

mod render_draw_tab;
mod render_graph_tab;
mod render_list_tab;
mod render_log_tab;
mod render_popup;

pub fn ui(f: &mut Frame, app: &mut App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(3), Constraint::Min(0)])
        .split(f.area());

    // Tabs
    let tab_titles = vec!["Fai una Prova", "Scheda pt.1", "Scheda pt.2", "Logs Prove"];
    let tabs = Tabs::new(tab_titles.clone())
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .title(" Menù (Tab per muoverti) "),
        )
        .select(app.current_tab.idx())
        .style(Style::default().fg(Color::White))
        .highlight_style(
            Style::default()
                .fg(Color::Yellow)
                .add_modifier(Modifier::BOLD),
        );

    f.render_widget(tabs, chunks[0]);

    // Calculate tab areas for mouse interaction
    let tab_bar_inner = Rect {
        x: chunks[0].x + 1,
        y: chunks[0].y + 1,
        width: chunks[0].width - 2,
        height: chunks[0].height - 2,
    };

    app.tab_areas.clear();
    let mut current_x = tab_bar_inner.x + 1; // Start with 1 space padding

    for title in &tab_titles {
        let tab_width = title.len() as u16 + 2; // Title + 1 space on each side
        app.tab_areas.push(Rect {
            x: current_x,
            y: tab_bar_inner.y,
            width: tab_width,
            height: tab_bar_inner.height,
        });
        current_x += tab_width + 1; // Move to next tab position (with separator)
    }

    // Content based on selected tab
    match app.current_tab {
        TabType::DrawTab => render_draw_tab::render_draw_tab(f, chunks[1], app),
        TabType::CharacterSheetTab => render_graph_tab::render_graph_tab(f, chunks[1], app),
        TabType::AdditionalInfoTab => {
            render_list_tab::render_list_tab(f, chunks[1], app);
            // force ui scrollbar to be visible if section has text
            if !app.list_data.notes.is_empty() {
                app.update_notes_vertical_scroll_state();
            }
            for i in 0..3 {
                if !app.list_data.lessons[i].is_empty() {
                    app.update_list_vertical_scroll_state(i);
                }
            }
        }
        TabType::LogTab => render_log_tab::render_history_tab(f, chunks[1], app),
        _ => {}
    }

    if app.popup != PopupType::None {
        // Draw popup
        render_popup::draw_popup(f, app);
    } else if app.editing_node || app.editing_character_info {
        // Node editing popup or Character info editing popup
        render_popup::draw_node_edit_popup(f, app);
    } else if app.editing_list_item {
        // List editing popup
        render_popup::draw_list_edit_popup(f, app);
    }
}
