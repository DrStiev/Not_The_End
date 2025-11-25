use ratatui::{
    Frame,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Clear, Paragraph, Scrollbar, ScrollbarOrientation, Tabs, Wrap},
};

use crate::app::{App, BallType, FocusedSection, PopupType};

pub fn ui(f: &mut Frame, app: &mut App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(3), Constraint::Min(0)])
        .split(f.area());

    // Tabs
    let tab_titles = vec!["Fai una Prova", "Tab 2", "Tab 3", "Log"];
    let tabs = Tabs::new(tab_titles.clone())
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title(" Naviga (Tab) "),
        )
        .select(app.current_tab)
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
        0 => render_draw_tab(f, chunks[1], app),
        1 => render_empty_tab(f, chunks[1], "Tab 2"),
        2 => render_empty_tab(f, chunks[1], "Tab 3"),
        3 => render_history_tab(f, chunks[1], app),
        _ => {}
    }

    // Popup
    if app.popup != PopupType::None {
        draw_popup(f, app);
    }
}

fn render_draw_tab(f: &mut Frame, area: Rect, app: &mut App) {
    let main_layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(area);

    let left_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(main_layout[0]);

    let right_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(85), Constraint::Percentage(15)])
        .split(main_layout[1]);

    // Store areas for mouse interaction (entire widget including borders)
    app.white_balls_area = left_layout[0];
    app.red_balls_area = left_layout[1];
    app.draw_input_area = right_layout[0];

    // Sezione pallini bianchi
    let white_style = if app.focused_section == FocusedSection::WhiteBalls {
        Style::default()
            .fg(Color::Yellow)
            .add_modifier(Modifier::BOLD)
    } else {
        Style::default()
    };

    let white_block = Block::default()
        .title(" Quanti TRATTI vuoi usare? (↑/↓) ")
        .borders(Borders::ALL)
        .style(white_style);

    let white_balls_text = create_filled_balls_display(app.white_balls, Color::White);
    let white_paragraph = Paragraph::new(white_balls_text)
        .block(white_block)
        .alignment(Alignment::Center);

    f.render_widget(white_paragraph, left_layout[0]);

    // Sezione pallini rossi
    let red_style = if app.focused_section == FocusedSection::RedBalls {
        Style::default()
            .fg(Color::Yellow)
            .add_modifier(Modifier::BOLD)
    } else {
        Style::default()
    };

    let red_block = Block::default()
        .title(" Quanto è DIFFICILE la prova? (↑/↓) ")
        .borders(Borders::ALL)
        .style(red_style);

    let red_balls_text = create_filled_balls_display(app.red_balls, Color::Red);
    let red_paragraph = Paragraph::new(red_balls_text)
        .block(red_block)
        .alignment(Alignment::Center);

    f.render_widget(red_paragraph, left_layout[1]);

    // Sezione pescata
    let draw_style = if app.focused_section == FocusedSection::DrawInput {
        Style::default()
            .fg(Color::Yellow)
            .add_modifier(Modifier::BOLD)
    } else {
        Style::default()
    };

    let draw_block = Block::default()
        .title(" Effettua una PROVA (↑/↓ poi Enter) ")
        .borders(Borders::ALL)
        .style(draw_style);

    let draw_content = create_draw_section_content(app);
    let draw_paragraph = Paragraph::new(draw_content)
        .block(draw_block)
        .alignment(Alignment::Center)
        .wrap(Wrap { trim: true });

    f.render_widget(draw_paragraph, right_layout[0]);

    // Bottone reset
    let reset_block = Block::default().title(" Reset (R) ").borders(Borders::ALL);

    let reset_text = Line::from(vec![
        Span::styled("Premi ", Style::default()),
        Span::styled(
            "R",
            Style::default()
                .fg(Color::Green)
                .add_modifier(Modifier::BOLD),
        ),
        Span::styled(" per resettare", Style::default()),
    ]);

    let reset_paragraph = Paragraph::new(reset_text)
        .block(reset_block)
        .alignment(Alignment::Center);

    f.render_widget(reset_paragraph, right_layout[1]);
}

fn render_empty_tab(f: &mut Frame, area: Rect, title: &str) {
    let block = Block::default().title(title).borders(Borders::ALL);

    let text = Paragraph::new("Contenuto in arrivo...")
        .block(block)
        .alignment(Alignment::Center);

    f.render_widget(text, area);
}

fn render_history_tab(f: &mut Frame, area: Rect, app: &mut App) {
    let block = Block::default()
        .title(" Log - Cronologia Prove (↑/↓ per scorrere) ")
        .borders(Borders::ALL);

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
            format!("═══ Prova #{} ═══", i + 1),
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
        )));
        lines.push(Line::from(""));

        // Pallini bianchi usati
        lines.push(Line::from(vec![
            Span::styled("Traits: ", Style::default().add_modifier(Modifier::BOLD)),
            Span::raw(format!("{}", entry.white_balls)),
        ]));

        // Pallini rossi usati
        lines.push(Line::from(vec![
            Span::styled(
                "Difficoltà: ",
                Style::default().add_modifier(Modifier::BOLD),
            ),
            Span::raw(format!("{}", entry.red_balls)),
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

fn create_filled_balls_display(count: usize, color: Color) -> Line<'static> {
    let mut spans = Vec::new();
    for _ in 0..count {
        spans.push(Span::styled("● ", Style::default().fg(color)));
    }
    Line::from(spans)
}

fn create_empty_balls_display(count: usize) -> Line<'static> {
    let mut spans = Vec::new();
    for _ in 0..count {
        spans.push(Span::styled("○ ", Style::default().fg(Color::White)));
    }
    Line::from(spans)
}

fn create_draw_section_content(app: &App) -> Vec<Line<'static>> {
    let mut lines = Vec::new();

    lines.push(Line::from(""));
    lines.push(Line::from(Span::styled(
        " Quanti TOKEN vuoi ESTRARRE? ",
        Style::default().add_modifier(Modifier::BOLD),
    )));
    lines.push(create_empty_balls_display(app.draw_count));
    lines.push(Line::from(""));

    if !app.drawn_balls.is_empty() {
        lines.push(Line::from(""));
        lines.push(Line::from(Span::styled(
            "Token estratti:",
            Style::default().add_modifier(Modifier::BOLD),
        )));

        let mut ball_spans = Vec::new();
        for ball in &app.drawn_balls {
            let (symbol, color) = match ball {
                BallType::White => ("● ", Color::White),
                BallType::Red => ("● ", Color::Red),
            };
            ball_spans.push(Span::styled(symbol, Style::default().fg(color)));
        }
        lines.push(Line::from(ball_spans));
    }

    lines
}

fn draw_popup(f: &mut Frame, app: &App) {
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
