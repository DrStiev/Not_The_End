use ratatui::{
    Frame,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{
        Block, BorderType, Borders, Clear, Paragraph, Scrollbar, ScrollbarOrientation, Tabs, Wrap,
    },
};

use super::app::{App, BallType, FocusedSection, PopupType};

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
                .title(" Menù (Tab) "),
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
        1 => render_graph_tab(f, chunks[1], app),
        2 => render_list_tab(f, chunks[1], app),
        3 => render_history_tab(f, chunks[1], app),
        _ => {}
    }

    // Popup
    if app.popup != PopupType::None {
        draw_popup(f, app);
    }

    // Node editing popup
    if app.editing_node {
        draw_node_edit_popup(f, app);
    }

    // List editing popup
    if app.editing_list_item {
        draw_list_edit_popup(f, app);
    }
}

fn render_draw_tab(f: &mut Frame, area: Rect, app: &mut App) {
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
        .title(" Quanti TRATTI vuoi usare? (↑/↓) ")
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
        .title(" Quanto è DIFFICILE la prova? (↑/↓) ")
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
        .title(" Effettua una PROVA (↑/↓ poi Enter) ")
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

fn graph_node_title(idx: usize) -> String {
    // let s = idx.to_string();
    match idx {
        // hardcoded position of cell inside hexgrid graph
        // counting left to right, top to bottom (0 to 18)
        9 => " Archetipo ".to_string(),                      //+ &s,
        4 | 5 | 8 | 10 | 13 | 14 => " Qualità ".to_string(), //+ &s,
        _ => " Abilità ".to_string(),                        //+ &s,
    }
}

fn render_graph_tab(f: &mut Frame, area: Rect, app: &mut App) {
    let block = Block::default()
        .title(" Scheda HexSys (Click per selezionare, Enter per modificare) ")
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded);

    let inner_area = block.inner(area);
    f.render_widget(block, area);

    // Store area for click detection
    app.graph_area = area;

    // Check if area is too small
    if inner_area.width < 30 || inner_area.height < 15 {
        let warning =
            Paragraph::new("Finestra troppo piccola!\nIngrandire per visualizzare la scheda.")
                .alignment(Alignment::Center)
                .style(Style::default().fg(Color::Red));
        f.render_widget(warning, inner_area);
        return;
    }

    // Calculate center of the area
    let center_x = (inner_area.x + inner_area.width) / 2;
    let center_y = (inner_area.y + inner_area.height) / 2;

    // Render each node
    for (i, node) in app.honeycomb_nodes.iter().enumerate() {
        // Calculate node position with proper bounds checking
        let node_x_calc = center_x as i32 + node.x as i32;
        let node_y_calc = center_y as i32 + node.y as i32;

        // Skip if node would be outside bounds
        if node_x_calc < inner_area.x as i32
            || node_y_calc < inner_area.y as i32
            || node_x_calc + node.width as i32 > (inner_area.x + inner_area.width) as i32
            || node_y_calc + node.height as i32 > (inner_area.y + inner_area.height) as i32
        {
            continue;
        }

        let node_x = node_x_calc as u16;
        let node_y = node_y_calc as u16;

        let is_selected = app.selected_node == Some(i);

        let node_style = if is_selected {
            Style::default()
                .fg(Color::Yellow)
                .add_modifier(Modifier::BOLD)
        } else {
            Style::default()
        };

        let node_block = Block::default()
            .title(Line::from(graph_node_title(i)).centered())
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .style(node_style);

        let node_rect = Rect {
            x: node_x,
            y: node_y,
            width: node.width,
            height: node.height,
        };

        let node_text = if node.text.is_empty() {
            "[Vuoto]"
        } else {
            &node.text
        };

        let paragraph = Paragraph::new("\n".to_owned() + node_text)
            .block(node_block)
            .alignment(Alignment::Center)
            .wrap(Wrap { trim: true });

        f.render_widget(paragraph, node_rect);
    }
}

fn render_list_tab(f: &mut Frame, area: Rect, app: &mut App) {
    let main_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(8), // Misfortunes
            Constraint::Length(8), // Resources
            Constraint::Min(8),    // Lessons
        ])
        .split(area);

    // Misfortunes section (4 zones)
    let misfortunes_layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(25),
            Constraint::Percentage(25),
            Constraint::Percentage(25),
            Constraint::Percentage(25),
        ])
        .split(main_layout[0]);

    for i in 0..4 {
        let is_selected = app.selected_list_item == Some((0, i));
        let style = if is_selected {
            Style::default()
                .fg(Color::Yellow)
                .add_modifier(Modifier::BOLD)
        } else {
            Style::default()
        };

        let block = Block::default()
            .title(" SVENTURA ")
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .style(style);

        let text = if app.list_data.misfortunes[i].is_empty() {
            "[Vuoto]"
        } else {
            &app.list_data.misfortunes[i]
        };

        let paragraph = Paragraph::new(text)
            .block(block)
            .wrap(Wrap { trim: true })
            .alignment(Alignment::Center);

        app.misfortunes_area[i] = misfortunes_layout[i];
        f.render_widget(paragraph, misfortunes_layout[i]);
    }

    // Resources section (2 lists of 5)
    let mut style_left = Style::default();
    let mut style_right = Style::default();
    let resources_layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(main_layout[1]);

    if let Some((section, _)) = app.selected_list_item {
        match section {
            1 => {
                style_left = Style::default()
                    .fg(Color::Yellow)
                    .add_modifier(Modifier::BOLD);
            }
            2 => {
                style_right = Style::default()
                    .fg(Color::Yellow)
                    .add_modifier(Modifier::BOLD);
            }
            _ => {}
        }
    }

    // Left resources
    let left_block = Block::default()
        .title(" Di quali RISORSE dispongo? ")
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(style_left);

    let left_items: Vec<Line> = app
        .list_data
        .left_resources
        .iter()
        .enumerate()
        .map(|(i, text)| {
            let is_selected = app.selected_list_item == Some((1, i));
            let style = if is_selected {
                Style::default()
                    .fg(Color::Yellow)
                    .add_modifier(Modifier::BOLD)
            } else {
                Style::default()
            };
            let content = if text.is_empty() { "[Vuoto]" } else { text };
            Line::from(Span::styled(content, style))
        })
        .collect();

    let left_paragraph = Paragraph::new(left_items).block(left_block);
    app.resources_area[0] = resources_layout[0];
    f.render_widget(left_paragraph, resources_layout[0]);

    // Right resources
    let right_block = Block::default()
        .title(" Di quali RISORSE dispongo? ")
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(style_right);

    let right_items: Vec<Line> = app
        .list_data
        .right_resources
        .iter()
        .enumerate()
        .map(|(i, text)| {
            let is_selected = app.selected_list_item == Some((2, i));
            let style = if is_selected {
                Style::default()
                    .fg(Color::Yellow)
                    .add_modifier(Modifier::BOLD)
            } else {
                Style::default()
            };
            let content = if text.is_empty() { "[Vuoto]" } else { text };
            Line::from(Span::styled(content, style))
        })
        .collect();

    let right_paragraph = Paragraph::new(right_items).block(right_block);
    app.resources_area[1] = resources_layout[1];
    f.render_widget(right_paragraph, resources_layout[1]);

    // Lessons section (3 rectangles)
    let lessons_layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(33),
            Constraint::Percentage(34),
            Constraint::Percentage(33),
        ])
        .split(main_layout[2]);

    for i in 0..3 {
        let is_selected = app.selected_list_item == Some((3, i));
        let style = if is_selected {
            Style::default()
                .fg(Color::Yellow)
                .add_modifier(Modifier::BOLD)
        } else {
            Style::default()
        };

        let block = Block::default()
            .title(" LEZIONE ")
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .style(style);

        let text = if app.list_data.lessons[i].is_empty() {
            "[Vuoto]"
        } else {
            &app.list_data.lessons[i]
        };

        let paragraph = Paragraph::new(text).block(block).wrap(Wrap { trim: true });

        app.lections_area[i] = lessons_layout[i];
        f.render_widget(paragraph, lessons_layout[i]);
    }
}

fn render_empty_tab(f: &mut Frame, area: Rect, title: &str) {
    let block = Block::default()
        .title(title)
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded);

    let text = Paragraph::new("Contenuto in arrivo...")
        .block(block)
        .alignment(Alignment::Center);

    f.render_widget(text, area);
}

fn render_history_tab(f: &mut Frame, area: Rect, app: &mut App) {
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
        spans.push(Span::styled("○ ", Style::default().fg(Color::Gray)));
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

fn draw_node_edit_popup(f: &mut Frame, app: &App) {
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
                &app.node_edit_buffer,
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

fn draw_list_edit_popup(f: &mut Frame, app: &App) {
    let area = centered_rect(70, 30, f.area());

    let popup_block = Block::default()
        .title(Line::from(" Modifica (Esc per confermare) ").alignment(Alignment::Center))
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .style(Style::default().bg(Color::Black).fg(Color::Yellow));

    let mut text = vec![Line::from("")];

    // handle \n character
    let temp = &app.list_edit_buffer;
    let mut curr = 0;
    for (i, c) in temp.chars().enumerate() {
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
