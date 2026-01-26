use ratatui::{
    Frame,
    layout::Rect,
    style::{Color, Modifier, Style},
    widgets::{Block, BorderType, Borders, Tabs},
};

use crate::app::App;

/// Titoli dei tab
pub const TAB_TITLES: [&str; 4] = ["Fai una Prova", "Scheda pt.1", "Scheda pt.2", "Logs Prove"];

/// Renderizza la barra dei tab e calcola le aree per l'interazione mouse
pub fn render_tabs_bar(f: &mut Frame, area: Rect, app: &mut App) {
    let tabs = Tabs::new(TAB_TITLES.to_vec())
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

    f.render_widget(tabs, area);

    // Calcola le aree dei tab per l'interazione mouse
    calculate_tab_areas(area, app);
}

/// Calcola le aree interattive dei singoli tab
fn calculate_tab_areas(tabs_area: Rect, app: &mut App) {
    let tab_bar_inner = Rect {
        x: tabs_area.x + 1,
        y: tabs_area.y + 1,
        width: tabs_area.width - 2,
        height: tabs_area.height - 2,
    };

    app.tab_areas.clear();
    let mut current_x = tab_bar_inner.x + 1; // Inizia con 1 spazio di padding

    for title in &TAB_TITLES {
        let tab_width = title.len() as u16 + 2; // Titolo + 1 spazio per lato
        app.tab_areas.push(Rect {
            x: current_x,
            y: tab_bar_inner.y,
            width: tab_width,
            height: tab_bar_inner.height,
        });
        current_x += tab_width + 1; // Passa al prossimo tab (con separatore)
    }
}
