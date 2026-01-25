use ratatui::{
    Frame,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, BorderType, Borders, Paragraph, Scrollbar, ScrollbarOrientation, Wrap},
};

use crate::app::{App, ListSection, get_section_type};

pub fn create_filled_balls_display(count: usize, color: Color) -> Line<'static> {
    let mut spans = Vec::new();
    for _ in 0..count {
        spans.push(Span::styled("● ", Style::default().fg(color)));
    }
    Line::from(spans)
}

fn render_list_items<'a>(
    list_idx: usize,
    item_idx: usize,
    text: &'a String,
    selected_list_item: Option<(ListSection, usize)>,
) -> Line<'a> {
    let is_selected = selected_list_item == Some((get_section_type(list_idx), item_idx));
    let style = if is_selected {
        Style::default()
            .fg(Color::Yellow)
            .add_modifier(Modifier::BOLD)
    } else {
        Style::default()
    };
    let content = if text.is_empty() { "[Vuoto]" } else { text };
    Line::from(Span::styled(content, style))
}

fn style(
    section: ListSection,
    idx: usize,
    selected_list_item: Option<(ListSection, usize)>,
    additional: Option<bool>,
) -> Style {
    let is_selected = selected_list_item == Some((section, idx));

    if is_selected {
        Style::default()
            .fg(Color::Yellow)
            .add_modifier(Modifier::BOLD)
    } else if additional.is_some() && additional.unwrap() {
        Style::default().fg(Color::Red).add_modifier(Modifier::BOLD)
    } else {
        Style::default()
    }
}

pub fn render_list_tab(f: &mut Frame, area: Rect, app: &mut App) {
    let main_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(8),  // Misfortunes
            Constraint::Length(4),  // Misfortunes Red Balls
            Constraint::Length(12), // Resources
            Constraint::Min(8),     // Lessons
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
        let style = style(
            ListSection::Misfortunes,
            i,
            app.selected_list_item,
            Some(app.additional_red_balls[i] != 0),
        );

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

    // Misfortunes Red Balls section (4 zones)
    let misfortunes_red_balls_layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(25),
            Constraint::Percentage(25),
            Constraint::Percentage(25),
            Constraint::Percentage(25),
        ])
        .split(main_layout[1]);

    for i in 0..4 {
        let style = style(
            ListSection::MisfortunesDifficult,
            i,
            app.selected_list_item,
            Some(app.additional_red_balls[i] != 0),
        );

        let block = Block::default()
            .title(" DIFFICOLTÀ ")
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .style(style);

        let n = app.list_data.misfortunes_red_balls[i]
            .trim()
            .parse::<usize>()
            .unwrap_or(0); // obtain 0 if NaN
        let text = if app.list_data.misfortunes_red_balls[i].is_empty() || n == 0 {
            Line::from("[Vuoto]")
        } else {
            create_filled_balls_display(n, Color::Red)
        };

        let paragraph = Paragraph::new(text)
            .block(block)
            .wrap(Wrap { trim: true })
            .alignment(Alignment::Center);

        app.misfortunes_red_balls_area[i] = misfortunes_red_balls_layout[i];
        f.render_widget(paragraph, misfortunes_red_balls_layout[i]);
    }

    // Resources section
    #[allow(unused_assignments)]
    let mut items: Vec<Line> = vec![
        Line::from(Span::styled("[Vuoto]", Style::default())),
        Line::from(Span::styled("[Vuoto]", Style::default())),
        Line::from(Span::styled("[Vuoto]", Style::default())),
        Line::from(Span::styled("[Vuoto]", Style::default())),
        Line::from(Span::styled("[Vuoto]", Style::default())),
        Line::from(Span::styled("[Vuoto]", Style::default())),
        Line::from(Span::styled("[Vuoto]", Style::default())),
        Line::from(Span::styled("[Vuoto]", Style::default())),
        Line::from(Span::styled("[Vuoto]", Style::default())),
        Line::from(Span::styled("[Vuoto]", Style::default())),
    ];
    let mut style_v = Style::default();
    let resources_layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(main_layout[2]);

    if let Some((ListSection::LxResources, _)) = app.selected_list_item {
        style_v = Style::default()
            .fg(Color::Yellow)
            .add_modifier(Modifier::BOLD);
    }

    // modify resource list
    let block = Block::default()
        .title(" Di quali RISORSE dispongo? (↑/↓ per scorrere) ")
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(style_v);

    items = app
        .list_data
        .left_resources
        .iter()
        .enumerate()
        .map(|(i, text)| render_list_items(2, i, text, app.selected_list_item))
        .collect();

    let paragraph = Paragraph::new(items).block(block).wrap(Wrap { trim: true });
    app.resources_area[0] = resources_layout[0];
    f.render_widget(paragraph, resources_layout[0]);

    // Notes section
    let style_v = style(ListSection::Notes, 0, app.selected_list_item, None);

    let block = Block::default()
        .title(" NOTE (↑/↓ per scorrere) ")
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .style(style_v);

    let text = if app.list_data.notes.is_empty() {
        "[Vuoto]"
    } else {
        &app.list_data.notes
    };

    let paragraph = Paragraph::new(text)
        .block(block)
        .scroll((app.notes_vertical_scroll as u16, 0))
        .wrap(Wrap { trim: true });

    app.resources_area[1] = resources_layout[1];
    f.render_widget(paragraph, resources_layout[1]);

    // Render scrollbar
    let scrollbar = Scrollbar::new(ScrollbarOrientation::VerticalRight)
        .begin_symbol(Some("↑"))
        .end_symbol(Some("↓"));

    f.render_stateful_widget(
        scrollbar,
        resources_layout[1],
        &mut app.notes_vertical_scroll_state,
    );

    // Lessons section (3 rectangles)
    let lessons_layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(33),
            Constraint::Percentage(34),
            Constraint::Percentage(33),
        ])
        .split(main_layout[3]);

    for i in 0..3 {
        let style = style(ListSection::Lessons, i, app.selected_list_item, None);

        let block = Block::default()
            .title(" LEZIONE (↑/↓ per scorrere) ")
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .style(style);

        let text = if app.list_data.lessons[i].is_empty() {
            "[Vuoto]"
        } else {
            &app.list_data.lessons[i]
        };

        let paragraph = Paragraph::new(text)
            .block(block)
            .scroll((app.list_vertical_scroll[i] as u16, 0))
            .wrap(Wrap { trim: true });

        app.lections_area[i] = lessons_layout[i];
        f.render_widget(paragraph, lessons_layout[i]);

        // Render scrollbar
        let scrollbar = Scrollbar::new(ScrollbarOrientation::VerticalRight)
            .begin_symbol(Some("↑"))
            .end_symbol(Some("↓"));

        f.render_stateful_widget(
            scrollbar,
            lessons_layout[i],
            &mut app.list_vertical_scroll_state[i],
        );
    }
}
