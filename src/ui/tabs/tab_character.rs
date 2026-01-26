use ratatui::{
    Frame,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::Line,
    widgets::{Block, BorderType, Borders, Paragraph, Wrap},
};

use crate::app::{App, CharacterSection};

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

pub fn render(f: &mut Frame, area: Rect, app: &mut App) {
    let main_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3), // Name and Objective part
            Constraint::Fill(1),   // Honeycomb grid
        ])
        .split(area);

    // Name and Objective section
    let upper_layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Length(30), // name section
            Constraint::Fill(1),    // blank
            Constraint::Length(45), // objective section
        ])
        .split(main_layout[0]);

    // Name section
    let draw_style = if app.selected_character_info == CharacterSection::CharacterName {
        Style::default()
            .fg(Color::Yellow)
            .add_modifier(Modifier::BOLD)
    } else {
        Style::default()
    };
    let block = Block::default()
        .title(" Come mi chiamo? ")
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .style(draw_style);
    let text = if app.character_base_info.name.is_empty() {
        "[Vuoto]"
    } else {
        &app.character_base_info.name
    };
    let paragraph = Paragraph::new(text)
        .block(block)
        .wrap(Wrap { trim: true })
        .alignment(Alignment::Center);

    app.character_name_area = upper_layout[0];
    f.render_widget(paragraph, app.character_name_area);

    // Objective section
    let draw_style = if app.selected_character_info == CharacterSection::CharacterObjective {
        Style::default()
            .fg(Color::Yellow)
            .add_modifier(Modifier::BOLD)
    } else {
        Style::default()
    };
    let block = Block::default()
        .title(" Per cosa sono disposto a RISCHIARE? ")
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .style(draw_style);
    let text = if app.character_base_info.objective.is_empty() {
        "[Vuoto]"
    } else {
        &app.character_base_info.objective
    };
    let paragraph = Paragraph::new(text)
        .block(block)
        .wrap(Wrap { trim: true })
        .alignment(Alignment::Center);

    app.character_objective_area = upper_layout[2];
    f.render_widget(paragraph, app.character_objective_area);

    // Honeycomb section
    let block = Block::default()
        .title(
            " Scheda HexSys (Click per Selezionare, Enter per Modificare, E per Attivare Tratto) ",
        )
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded);

    let inner_area = block.inner(main_layout[1]);
    f.render_widget(block, main_layout[1]);

    // Store area for click detection
    app.graph_area = inner_area;

    // Check if area is too small
    if inner_area.width < 20 || inner_area.height < 10 {
        let warning =
            Paragraph::new("Finestra troppo piccola!\nIngrandire per visualizzare la scheda.")
                .alignment(Alignment::Center)
                .style(Style::default().fg(Color::Red));
        f.render_widget(warning, inner_area);
        return;
    }

    // Calculate center of the area
    let center_x = (inner_area.x / 2 + inner_area.width) / 2;
    let center_y = (inner_area.y / 2 + inner_area.height) / 2;

    // Render each node
    for (i, node) in app.honeycomb_nodes.iter().enumerate() {
        // Calculate node position with proper bounds checking
        let node_x_calc = (center_x - node.width / 2) as i32 + node.x as i32;
        let node_y_calc = (center_y + node.height / 2) as i32 + node.y as i32;

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
        let is_trait_used = app.used_traits.contains(&i);

        let node_style = if is_selected {
            Style::default()
                .fg(Color::Yellow)
                .add_modifier(Modifier::BOLD)
        } else if is_trait_used {
            Style::default()
                .fg(Color::Green)
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
