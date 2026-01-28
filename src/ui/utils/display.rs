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
mod display_tests {
    use crate::ui::utils::*;
    use ratatui::prelude::Rect;
    use ratatui::style::Color;

    #[test]
    fn test_create_filled_balls_display_empty() {
        let line = create_filled_balls_display(0, Color::White);
        assert_eq!(line.spans.len(), 0);
    }

    #[test]
    fn test_create_filled_balls_display_single() {
        let line = create_filled_balls_display(1, Color::White);
        assert_eq!(line.spans.len(), 1);
    }

    #[test]
    fn test_create_filled_balls_display_multiple() {
        let line = create_filled_balls_display(5, Color::White);
        assert_eq!(line.spans.len(), 5);
    }

    #[test]
    fn test_create_filled_balls_display_white() {
        let line = create_filled_balls_display(3, Color::White);
        for span in &line.spans {
            assert_eq!(span.style.fg, Some(Color::White));
        }
    }

    #[test]
    fn test_create_filled_balls_display_red() {
        let line = create_filled_balls_display(3, Color::Red);
        for span in &line.spans {
            assert_eq!(span.style.fg, Some(Color::Red));
        }
    }

    #[test]
    fn test_create_empty_balls_display_empty() {
        let line = create_empty_balls_display(0);
        assert_eq!(line.spans.len(), 0);
    }

    #[test]
    fn test_create_empty_balls_display_multiple() {
        let line = create_empty_balls_display(4);
        assert_eq!(line.spans.len(), 4);
    }

    #[test]
    fn test_create_empty_balls_display_color() {
        let line = create_empty_balls_display(2);
        for span in &line.spans {
            assert_eq!(span.style.fg, Some(Color::Gray));
        }
    }

    #[test]
    fn test_centered_rect_50_50() {
        let parent = Rect::new(0, 0, 100, 100);
        let centered = centered_rect(50, 50, parent);

        assert_eq!(centered.width, 50);
        assert_eq!(centered.height, 50);
        assert_eq!(centered.x, 25);
        assert_eq!(centered.y, 25);
    }

    #[test]
    fn test_centered_rect_30_30() {
        let parent = Rect::new(0, 0, 100, 100);
        let centered = centered_rect(30, 30, parent);

        assert_eq!(centered.width, 30);
        assert_eq!(centered.height, 30);
        assert_eq!(centered.x, 35);
        assert_eq!(centered.y, 35);
    }

    #[test]
    fn test_centered_rect_100_100() {
        let parent = Rect::new(0, 0, 100, 100);
        let centered = centered_rect(100, 100, parent);

        assert_eq!(centered.width, 100);
        assert_eq!(centered.height, 100);
        assert_eq!(centered.x, 0);
        assert_eq!(centered.y, 0);
    }

    #[test]
    fn test_centered_rect_small_parent() {
        let parent = Rect::new(0, 0, 20, 20);
        let centered = centered_rect(50, 50, parent);

        assert_eq!(centered.width, 10);
        assert_eq!(centered.height, 10);
    }

    #[test]
    fn test_centered_rect_with_offset() {
        let parent = Rect::new(10, 10, 100, 100);
        let centered = centered_rect(50, 50, parent);

        // Should be centered within the parent area
        assert_eq!(centered.width, 50);
        assert_eq!(centered.height, 50);
    }
}
