use ratatui::{
    Frame,
    layout::{Alignment, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, BorderType, Borders, Paragraph, Scrollbar, ScrollbarOrientation, Wrap},
};

use crate::app::{App, BallType};

pub fn render_history_tab(f: &mut Frame, area: Rect, app: &mut App) {
    let block = Block::default()
        .title(" Log - Cronologia Prove (↑/↓ per scorrere) ")
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded);

    if app.history.is_empty() {
        let text = Paragraph::new("Nessuna prova effettuata")
            .block(block)
            .alignment(Alignment::Center);
        f.render_widget(text, area);
        return;
    }

    let mut lines = Vec::new();
    lines.push(Line::from(""));

    for (i, entry) in app.history.iter().enumerate().rev() {
        lines.push(Line::from(Span::styled(
            format!("{} - Prova #{}: ", entry.time, i + 1),
            Style::default()
                .fg(Color::LightYellow)
                .add_modifier(Modifier::BOLD),
        )));
        lines.push(Line::from(""));

        let mut s: String = String::new();
        if !entry.traits.is_empty() {
            for idx in &entry.traits {
                s.push_str(&format!("{}, ", app.honeycomb_nodes[*idx].text));
            }
        } else {
            s.push_str("Nessuno");
        }

        // Pallini bianchi usati
        lines.push(Line::from(vec![
            Span::styled(
                "Totale Token messi in gioco: ",
                Style::default().add_modifier(Modifier::BOLD),
            ),
            Span::raw(format!("{}", entry.white_balls)),
        ]));
        lines.push(Line::from(vec![
            Span::styled(
                "Tratti della scheda utilizzati: ",
                Style::default().add_modifier(Modifier::BOLD),
            ),
            Span::raw(s.to_string()),
        ]));

        lines.push(Line::from(""));

        let mut s: String = String::new();
        let mut b: bool = false;
        // idx contains the number of additional red balls where I need the position in the array
        for (i, idx) in entry.misfortunes.iter().enumerate() {
            if *idx != 0 {
                b = true;
                s.push_str(&format!("{}, ", app.list_data.misfortunes[i]));
            }
        }
        if !b {
            s.push_str("Nessuna");
        }

        // Pallini rossi usati
        lines.push(Line::from(vec![
            Span::styled(
                "Difficoltà: ",
                Style::default().add_modifier(Modifier::BOLD),
            ),
            Span::raw(format!("{}", entry.red_balls)),
        ]));
        lines.push(Line::from(vec![
            Span::styled(
                "Sventure messe in gioco: ",
                Style::default().add_modifier(Modifier::BOLD),
            ),
            Span::raw(s.to_string()),
        ]));

        lines.push(Line::from(""));

        // Risultato prima pescata
        let first_draw_str = entry.format_balls(&entry.first_draw);
        lines.push(Line::from(vec![
            Span::styled(
                "Token pescati: ",
                Style::default().add_modifier(Modifier::BOLD),
            ),
            Span::raw(format!("{} ({})", entry.first_draw.len(), first_draw_str)),
        ]));

        // Visualizza i pallini della prima pescata
        let mut first_draw_spans = vec![Span::raw("  ")];
        for ball in &entry.first_draw {
            let (symbol, color) = match ball {
                BallType::White => ("● ", Color::White),
                BallType::Red => ("● ", Color::Red),
            };
            first_draw_spans.push(Span::styled(symbol, Style::default().fg(color)));
        }
        lines.push(Line::from(first_draw_spans));

        lines.push(Line::from(""));

        // Rischio
        if entry.risked {
            let risk_draw_str = entry.format_balls(&entry.risk_draw);
            lines.push(Line::from(vec![
                Span::styled("Rischiato: ", Style::default().add_modifier(Modifier::BOLD)),
                Span::styled("Sì", Style::default().fg(Color::Green)),
            ]));

            lines.push(Line::from(vec![
                Span::styled(
                    "  Risultato rischio: ",
                    Style::default().add_modifier(Modifier::BOLD),
                ),
                Span::raw(format!("{} ({})", entry.risk_draw.len(), risk_draw_str)),
            ]));

            // Visualizza i pallini del rischio
            let mut risk_draw_spans = vec![Span::raw("    ")];
            for ball in &entry.risk_draw {
                let (symbol, color) = match ball {
                    BallType::White => ("● ", Color::White),
                    BallType::Red => ("● ", Color::Red),
                };
                risk_draw_spans.push(Span::styled(symbol, Style::default().fg(color)));
            }
            lines.push(Line::from(risk_draw_spans));
        } else {
            lines.push(Line::from(vec![
                Span::styled("Rischiato: ", Style::default().add_modifier(Modifier::BOLD)),
                Span::styled("No", Style::default().fg(Color::Red)),
            ]));
        }

        if entry.confused {
            lines.push(Line::from(""));
            lines.push(Line::from(vec![Span::styled(
                "Sotto effetto di Confusione",
                Style::default().add_modifier(Modifier::BOLD),
            )]));
        }

        if entry.adrenalined {
            lines.push(Line::from(""));
            lines.push(Line::from(vec![Span::styled(
                "Sotto effetto di Adrenalina",
                Style::default().add_modifier(Modifier::BOLD),
            )]));
        }

        lines.push(Line::from(""));
    }

    let paragraph = Paragraph::new(lines)
        .block(block)
        .scroll((app.vertical_scroll as u16, 0))
        .wrap(Wrap { trim: true });

    f.render_widget(paragraph, area);

    // Render scrollbar
    let scrollbar = Scrollbar::new(ScrollbarOrientation::VerticalRight)
        .begin_symbol(Some("↑"))
        .end_symbol(Some("↓"));

    f.render_stateful_widget(scrollbar, area, &mut app.vertical_scroll_state);
}
