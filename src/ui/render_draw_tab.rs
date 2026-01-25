use ratatui::{
    Frame,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, BorderType, Borders, Paragraph, Wrap},
};

use crate::app::{App, BallType, FocusedSection};

pub fn create_filled_balls_display(count: usize, color: Color) -> Line<'static> {
    let mut spans = Vec::new();
    for _ in 0..count {
        spans.push(Span::styled("● ", Style::default().fg(color)));
    }
    Line::from(spans)
}

pub fn create_empty_balls_display(count: usize) -> Line<'static> {
    let mut spans = Vec::new();
    for _ in 0..count {
        spans.push(Span::styled("○ ", Style::default().fg(Color::Gray)));
    }
    Line::from(spans)
}

fn create_draw_section_content(app: &App) -> Vec<Line<'static>> {
    let mut lines = vec![
        Line::from(""),
        Line::from(Span::styled(
            " Quanti TOKEN vuoi ESTRARRE? ",
            Style::default().add_modifier(Modifier::BOLD),
        )),
        create_empty_balls_display(app.draw_count),
        Line::from(""),
    ];

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

pub fn render_draw_tab(f: &mut Frame, area: Rect, app: &mut App) {
    let main_layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(area);

    let left_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage(20), // white balls
            Constraint::Percentage(20), // red balls
            Constraint::Percentage(10), // reset
            Constraint::Percentage(50), // text
        ])
        .split(main_layout[0]);

    let right_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage(30), // draw balls
            Constraint::Percentage(20), // status
            Constraint::Percentage(50), // text
        ])
        .split(main_layout[1]);

    let right_middle_layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(right_layout[1]);

    // Store areas for mouse interaction (entire widget including borders)
    app.white_balls_area = left_layout[0];
    app.red_balls_area = left_layout[1];
    app.random_mode_area = right_middle_layout[0];
    app.forced_four_area = right_middle_layout[1];
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
        .title(" Quanti TRATTI vuoi usare? (↑/↓ per selezionare) ")
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .style(white_style);

    let white_balls_text = create_filled_balls_display(app.white_balls, Color::White);
    let white_paragraph = Paragraph::new(white_balls_text)
        .block(white_block)
        .alignment(Alignment::Center);

    f.render_widget(white_paragraph, app.white_balls_area);

    // Sezione pallini rossi
    let red_style = if app.focused_section == FocusedSection::RedBalls {
        Style::default()
            .fg(Color::Yellow)
            .add_modifier(Modifier::BOLD)
    } else {
        Style::default()
    };

    let red_block = Block::default()
        .title(" Quanto è DIFFICILE la prova? (↑/↓ per selezionare) ")
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .style(red_style);

    let red_balls_text = create_filled_balls_display(app.red_balls, Color::Red);
    let red_paragraph = Paragraph::new(red_balls_text)
        .block(red_block)
        .alignment(Alignment::Center);

    f.render_widget(red_paragraph, app.red_balls_area);

    // Sezione pescata
    let draw_style = if app.focused_section == FocusedSection::DrawInput {
        Style::default()
            .fg(Color::Yellow)
            .add_modifier(Modifier::BOLD)
    } else {
        Style::default()
    };

    let draw_block = Block::default()
        .title(" Effettua una PROVA (↑/↓ per selezionare, poi Enter) ")
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .style(draw_style);

    let draw_content = create_draw_section_content(app);
    let draw_paragraph = Paragraph::new(draw_content)
        .block(draw_block)
        .alignment(Alignment::Center)
        .wrap(Wrap { trim: true });

    f.render_widget(draw_paragraph, app.draw_input_area);

    // Bottone Confusione
    let confusion_style = if app.focused_section == FocusedSection::RandomMode {
        Style::default()
            .fg(Color::Yellow)
            .add_modifier(Modifier::BOLD)
    } else if app.random_mode {
        Style::default()
            .fg(Color::Green)
            .add_modifier(Modifier::BOLD)
    } else {
        Style::default()
    };

    let confusion_block = Block::default()
        .title(" Confusione ")
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .style(confusion_style);

    let confusion_text = Line::from(vec![
        Span::styled("Nella ", Style::default()),
        Span::styled("prossima ", Style::default()),
        Span::styled("PROVA", Style::default()),
        Span::styled(" aggiungi ", Style::default()),
        Span::styled("al ", Style::default()),
        Span::styled("POOL ", Style::default()),
        Span::styled("○", Style::default().fg(Color::Gray)),
        Span::styled(" invece ", Style::default()),
        Span::styled("di ", Style::default()),
        Span::styled("●", Style::default().fg(Color::White)),
        Span::styled(" .", Style::default()),
    ]);

    let confusion_paragraph = Paragraph::new(confusion_text)
        .block(confusion_block)
        .alignment(Alignment::Center)
        .wrap(Wrap { trim: true });

    f.render_widget(confusion_paragraph, right_middle_layout[0]);

    // Bottone Adrenalina
    let adrenalin_style = if app.focused_section == FocusedSection::ForcedFour {
        Style::default()
            .fg(Color::Yellow)
            .add_modifier(Modifier::BOLD)
    } else if app.forced_four_mode {
        Style::default()
            .fg(Color::Green)
            .add_modifier(Modifier::BOLD)
    } else {
        Style::default()
    };
    let adrenalin_block = Block::default()
        .title(" Adrenalina ")
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .style(adrenalin_style);

    let adrenalin_text = Line::from(vec![
        Span::styled("Nella ", Style::default()),
        Span::styled("prossima ", Style::default()),
        Span::styled("PROVA", Style::default()),
        Span::styled(" dovrai ", Style::default()),
        Span::styled("ESTRARRE", Style::default()),
        Span::styled(" almeno ", Style::default()),
        Span::styled("4 ", Style::default()),
        Span::styled("○", Style::default().fg(Color::Gray)),
        Span::styled(" .", Style::default()),
    ]);

    let adrenalin_paragraph = Paragraph::new(adrenalin_text)
        .block(adrenalin_block)
        .alignment(Alignment::Center)
        .wrap(Wrap { trim: true });

    f.render_widget(adrenalin_paragraph, right_middle_layout[1]);

    // Bottone reset / quit
    let reset_block = Block::default()
        .title(" Reset (R) / Quit (Q) ")
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded);

    let reset_text = Line::from(vec![
        Span::styled("Premi ", Style::default()),
        Span::styled(
            "R",
            Style::default()
                .fg(Color::Green)
                .add_modifier(Modifier::BOLD),
        ),
        Span::styled(" per resettare.", Style::default()),
        Span::styled(" Premi ", Style::default()),
        Span::styled(
            "Q",
            Style::default().fg(Color::Red).add_modifier(Modifier::BOLD),
        ),
        Span::styled(" per uscire.", Style::default()),
    ]);

    let reset_paragraph = Paragraph::new(reset_text)
        .block(reset_block)
        .alignment(Alignment::Center);

    f.render_widget(reset_paragraph, left_layout[2]);

    // text area left
    let text_block = Block::default()
        .title(" Ricorda... ")
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded);

    let text = vec![
        Line::from(vec![
            Span::styled("Affronti ", Style::default()),
            Span::styled("una ", Style::default()),
            Span::styled("PROVA", Style::default().fg(Color::LightYellow)),
            Span::styled(" quando ", Style::default()),
            Span::styled("ciò ", Style::default()),
            Span::styled("che ", Style::default()),
            Span::styled("stai ", Style::default()),
            Span::styled("tentando ", Style::default()),
            Span::styled("di ", Style::default()),
            Span::styled("fare ", Style::default()),
            Span::styled("potrebbe ", Style::default()),
            Span::styled("avere ", Style::default()),
            Span::styled("conseguenze ", Style::default()),
            Span::styled("NEGATIVE", Style::default()),
            Span::styled(".", Style::default()),
        ]),
        Line::from(""),
        Line::from(vec![
            Span::styled("Spendi ", Style::default()),
            Span::styled("il ", Style::default()),
            Span::styled("primo ", Style::default()),
            Span::styled("●", Style::default().fg(Color::White)),
            Span::styled(" per ", Style::default()),
            Span::styled("SUPERARE", Style::default().fg(Color::LightYellow)),
            Span::styled(" la ", Style::default()),
            Span::styled("PROVA", Style::default()),
            Span::styled(" e ", Style::default()),
            Span::styled("i ", Style::default()),
            Span::styled("restanti ", Style::default()),
            Span::styled("per ", Style::default()),
            Span::styled("MIGLIORARNE", Style::default().fg(Color::LightYellow)),
            Span::styled(" l'esito.", Style::default()),
        ]),
        Line::from(""),
        Line::from(vec![
            Span::styled("Spendi ", Style::default()),
            Span::styled("1 ", Style::default()),
            Span::styled("●", Style::default().fg(Color::Red)),
            Span::styled(" per ", Style::default()),
            Span::styled("accumulare ", Style::default()),
            Span::styled("ADRENALINA", Style::default().fg(Color::LightYellow)),
            Span::styled(" o ", Style::default()),
            Span::styled("CONFUSIONE", Style::default().fg(Color::LightYellow)),
            Span::styled(".", Style::default()),
        ]),
        Line::from(vec![
            Span::styled("Spendi ", Style::default()),
            Span::styled("1 ", Style::default()),
            Span::styled("●", Style::default().fg(Color::Red)),
            Span::styled(" come ", Style::default()),
            Span::styled("SVENTURA", Style::default().fg(Color::LightYellow)),
            Span::styled(" per ", Style::default()),
            Span::styled("fartene ", Style::default()),
            Span::styled("infliggere ", Style::default()),
            Span::styled("una ", Style::default()),
            Span::styled("dal ", Style::default()),
            Span::styled("NARRATORE.", Style::default()),
        ]),
        Line::from(vec![
            Span::styled("Spendi ", Style::default()),
            Span::styled("1 ", Style::default()),
            Span::styled("●", Style::default().fg(Color::Red)),
            Span::styled(" come ", Style::default()),
            Span::styled("COMPLICAZIONE", Style::default().fg(Color::LightYellow)),
            Span::styled(" per ", Style::default()),
            Span::styled("far ", Style::default()),
            Span::styled("raccontare ", Style::default()),
            Span::styled("dal ", Style::default()),
            Span::styled("NARRATORE ", Style::default()),
            Span::styled("un ", Style::default()),
            Span::styled("esito ", Style::default()),
            Span::styled("IMPREVISTO ", Style::default()),
            Span::styled("della ", Style::default()),
            Span::styled("SCENA. ", Style::default()),
        ]),
    ];

    let text_paragraph = Paragraph::new(text)
        .block(text_block)
        .alignment(Alignment::Left)
        .wrap(Wrap { trim: true });

    f.render_widget(text_paragraph, left_layout[3]);

    // text area right
    let text_block = Block::default()
        .title(" Ricorda... ")
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded);

    let text = vec![
        Line::from(vec![
            Span::styled("RISCHI", Style::default().fg(Color::LightYellow)),
            Span::styled(" quando ", Style::default()),
            Span::styled("vuoi ", Style::default()),
            Span::styled("ESTRARRE ", Style::default()),
            Span::styled("altri ", Style::default()),
            Span::styled("TOKEN ", Style::default()),
            Span::styled("oltre ", Style::default()),
            Span::styled("a ", Style::default()),
            Span::styled("quelli ", Style::default()),
            Span::styled("che ", Style::default()),
            Span::styled("hai ", Style::default()),
            Span::styled("già ", Style::default()),
            Span::styled("estratto ", Style::default()),
            Span::styled("durante ", Style::default()),
            Span::styled("una ", Style::default()),
            Span::styled("PROVA", Style::default()),
            Span::styled(".", Style::default()),
        ]),
        Line::from(""),
        Line::from(vec![
            Span::styled("Affronti ", Style::default()),
            Span::styled("una ", Style::default()),
            Span::styled("PROVA CRUCIALE", Style::default().fg(Color::LightYellow)),
            Span::styled(" quando ", Style::default()),
            Span::styled("la ", Style::default()),
            Span::styled("consideri ", Style::default()),
            Span::styled("DETERMINANTE ", Style::default()),
            Span::styled("per ", Style::default()),
            Span::styled("lo ", Style::default()),
            Span::styled("sviluppo ", Style::default()),
            Span::styled("dell'", Style::default()),
            Span::styled("eroe.", Style::default()),
            Span::styled(" Dichiara ", Style::default()),
            Span::styled("la ", Style::default()),
            Span::styled("PROVA CRUCIALE", Style::default().fg(Color::LightYellow)),
            Span::styled(" prima ", Style::default()),
            Span::styled("di ", Style::default()),
            Span::styled("ESTRARRE", Style::default().fg(Color::LightYellow)),
            Span::styled(".", Style::default()),
            Span::styled(" Affronta ", Style::default()),
            Span::styled("la ", Style::default()),
            Span::styled("PROVA", Style::default().fg(Color::LightYellow)),
            Span::styled(" normalmente.", Style::default()),
        ]),
        Line::from(""),
        Line::from(vec![
            Span::styled("Scegli ", Style::default()),
            Span::styled("un ", Style::default()),
            Span::styled("risultato ", Style::default()),
            Span::styled("in ", Style::default()),
            Span::styled("base ", Style::default()),
            Span::styled("all'", Style::default()),
            Span::styled("esito ", Style::default()),
            Span::styled("della ", Style::default()),
            Span::styled("prova:", Style::default()),
        ]),
        Line::from(vec![
            Span::styled("1. Guadagni ", Style::default()),
            Span::styled("o ", Style::default()),
            Span::styled("cambi ", Style::default()),
            Span::styled("un ", Style::default()),
            Span::styled("TRATTO", Style::default().fg(Color::LightYellow)),
            Span::styled(".", Style::default()),
        ]),
        Line::from(vec![
            Span::styled("2. Impari ", Style::default()),
            Span::styled("una ", Style::default()),
            Span::styled("LEZIONE", Style::default().fg(Color::LightYellow)),
            Span::styled(".", Style::default()),
        ]),
        Line::from(vec![
            Span::styled("3. Vieni ", Style::default()),
            Span::styled("segnato ", Style::default()),
            Span::styled("da ", Style::default()),
            Span::styled("una ", Style::default()),
            Span::styled("CICATRICE", Style::default().fg(Color::LightYellow)),
            Span::styled(".", Style::default()),
        ]),
    ];

    let text_paragraph = Paragraph::new(text)
        .block(text_block)
        .alignment(Alignment::Left)
        .wrap(Wrap { trim: true });

    f.render_widget(text_paragraph, right_layout[2]);
}
