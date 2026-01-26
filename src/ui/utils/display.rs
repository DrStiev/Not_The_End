use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    text::{Line, Span},
};

/// Crea una visualizzazione di palline piene colorate
pub fn create_filled_balls_display(count: usize, color: Color) -> Line<'static> {
    let mut spans = Vec::new();
    for _ in 0..count {
        spans.push(Span::styled("● ", Style::default().fg(color)));
    }
    Line::from(spans)
}

/// Crea una visualizzazione di palline vuote (grigie)
pub fn create_empty_balls_display(count: usize) -> Line<'static> {
    let mut spans = Vec::new();
    for _ in 0..count {
        spans.push(Span::styled("○ ", Style::default().fg(Color::Gray)));
    }
    Line::from(spans)
}

/// Crea un rettangolo centrato con le percentuali specificate
pub fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_filled_balls_display() {
        let line = create_filled_balls_display(3, Color::White);
        assert_eq!(line.spans.len(), 3);
    }

    #[test]
    fn test_centered_rect() {
        let rect = centered_rect(50, 50, Rect::new(0, 0, 100, 100));
        assert_eq!(rect.width, 50);
        assert_eq!(rect.height, 50);
    }
}
