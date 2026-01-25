use ratatui::{
    Frame,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, BorderType, Borders, Clear, Paragraph, Wrap},
};

use crate::app::{App, PopupType};

pub fn draw_popup(f: &mut Frame, app: &App) {
    // Posiziona il popup in basso per non coprire i risultati della pescata
    let area = centered_rect(30, 25, f.area());

    let title = match app.popup {
        PopupType::ConfirmDraw => " Conferma Pescata? ",
        PopupType::ConfirmRisk => " Vuoi Rischiare? ",
        _ => "",
    };

    let popup_block = Block::default()
        .title(Line::from(title).alignment(Alignment::Center))
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .style(Style::default().bg(Color::Black));

    let text = match app.popup {
        PopupType::ConfirmDraw => vec![
            Line::from(""),
            Line::from(vec![
                Span::styled("Enter", Style::default().fg(Color::Green)),
                Span::raw(" per confermare"),
            ]),
            Line::from(vec![
                Span::styled("Esc", Style::default().fg(Color::Red)),
                Span::raw(" per annullare"),
            ]),
        ],
        PopupType::ConfirmRisk => vec![
            Line::from(""),
            Line::from(format!(
                "Pescherai altri {} pallini",
                5 - app.drawn_balls.len()
            )),
            Line::from(""),
            Line::from(vec![
                Span::styled("Enter", Style::default().fg(Color::Green)),
                Span::raw(" per confermare"),
            ]),
            Line::from(vec![
                Span::styled("Esc", Style::default().fg(Color::Red)),
                Span::raw(" per annullare"),
            ]),
        ],
        _ => vec![],
    };

    let paragraph = Paragraph::new(text)
        .block(popup_block)
        .alignment(Alignment::Center)
        .wrap(Wrap { trim: true });

    f.render_widget(Clear, area);
    f.render_widget(paragraph, area);
}

fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage((100 - percent_y) / 2),
            Constraint::Percentage(percent_y),
            Constraint::Percentage((100 - percent_y) / 2),
        ])
        .split(r);

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - percent_x) / 2),
            Constraint::Percentage(percent_x),
            Constraint::Percentage((100 - percent_x) / 2),
        ])
        .split(popup_layout[1])[1]
}

pub fn draw_node_edit_popup(f: &mut Frame, app: &App) {
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
                // function is called only if in node editing or character editing
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

pub fn draw_list_edit_popup(f: &mut Frame, app: &App) {
    let area = centered_rect(70, 30, f.area());

    let popup_block = Block::default()
        .title(Line::from(" Modifica (Esc per confermare) ").alignment(Alignment::Center))
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .style(Style::default().bg(Color::Black).fg(Color::Yellow));

    let mut text = vec![Line::from("")];

    // handle '\n' character
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
