use ratatui::{
    Frame,
    layout::Alignment,
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, BorderType, Borders, Clear, Paragraph, Wrap},
};

use super::super::utils::centered_rect;
use crate::app::App;

/// Renderizza il popup di editing per nodi e informazioni personaggio
pub fn render_node_edit_popup(f: &mut Frame, app: &App) {
    let area = centered_rect(50, 20, f.area());

    let popup_block = Block::default()
        .title(Line::from(" Modifica (Esc per confermare) ").alignment(Alignment::Center))
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .style(Style::default().bg(Color::Black).fg(Color::Yellow));

    let text = vec![
        Line::from(""),
        Line::from(vec![
            Span::styled(
                if app.editing_node {
                    &app.node_edit_buffer
                } else {
                    &app.character_edit_buffer
                },
                Style::default().add_modifier(Modifier::BOLD),
            ),
            Span::styled("▌", Style::default().fg(Color::LightYellow)),
        ]),
    ];

    let paragraph = Paragraph::new(text)
        .block(popup_block)
        .alignment(Alignment::Center);

    f.render_widget(Clear, area);
    f.render_widget(paragraph, area);
}

/// Renderizza il popup di editing per liste (con supporto multilinea)
pub fn render_list_edit_popup(f: &mut Frame, app: &App) {
    let area = centered_rect(70, 30, f.area());

    let popup_block = Block::default()
        .title(Line::from(" Modifica (Esc per confermare) ").alignment(Alignment::Center))
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .style(Style::default().bg(Color::Black).fg(Color::Yellow));

    let mut text = vec![Line::from("")];

    // Gestione carattere '\n'
    let temp = &app.list_edit_buffer;
    let mut curr = 0;
    for (i, c) in temp.char_indices() {
        if c == '\n' {
            if curr == i {
                text.push(Line::from(""));
            } else {
                text.push(Line::from(Span::styled(
                    &temp[curr..i],
                    Style::default().add_modifier(Modifier::BOLD),
                )));
            }
            curr = i + 1;
        }
    }
    text.push(Line::from(vec![
        Span::styled(&temp[curr..], Style::default().add_modifier(Modifier::BOLD)),
        Span::styled("▌", Style::default().fg(Color::LightYellow)),
    ]));

    let paragraph = Paragraph::new(text)
        .block(popup_block)
        .wrap(Wrap { trim: true })
        .alignment(Alignment::Center);

    f.render_widget(Clear, area);
    f.render_widget(paragraph, area);
}
