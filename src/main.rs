use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEventKind},
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};
use rand::prelude::IndexedRandom;
use ratatui::{
    Frame, Terminal,
    backend::CrosstermBackend,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Clear, Paragraph, Wrap},
};
use std::io;

#[derive(Debug, Clone, Copy, PartialEq)]
enum BallType {
    White,
    Red,
}

#[derive(Debug, PartialEq)]
enum PopupType {
    None,
    ConfirmDraw,
    ConfirmRisk,
}

#[derive(Debug, PartialEq)]
enum FocusedSection {
    WhiteBalls,
    RedBalls,
    DrawInput,
}

struct App {
    white_balls: usize,
    red_balls: usize,
    draw_count: usize,
    focused_section: FocusedSection,
    popup: PopupType,
    drawn_balls: Vec<BallType>,
    first_draw_complete: bool,
    pool: Vec<BallType>,
}

impl App {
    fn new() -> App {
        App {
            white_balls: 0,
            red_balls: 0,
            draw_count: 1,
            focused_section: FocusedSection::WhiteBalls,
            popup: PopupType::None,
            drawn_balls: Vec::new(),
            first_draw_complete: false,
            pool: Vec::new(),
        }
    }

    fn reset(&mut self) {
        self.white_balls = 0;
        self.red_balls = 0;
        self.draw_count = 1;
        self.drawn_balls.clear();
        self.first_draw_complete = false;
        self.pool.clear();
        self.popup = PopupType::None;
    }

    fn create_pool(&mut self) {
        self.pool.clear();
        for _ in 0..self.white_balls {
            self.pool.push(BallType::White);
        }
        for _ in 0..self.red_balls {
            self.pool.push(BallType::Red);
        }
    }

    fn draw_from_pool(&mut self, count: usize) -> Vec<BallType> {
        let mut rng = rand::rng();
        let mut drawn = Vec::new();

        for _ in 0..count.min(self.pool.len()) {
            if let Some(&ball) = self.pool.choose(&mut rng) {
                drawn.push(ball);
                if let Some(pos) = self.pool.iter().position(|&x| x == ball) {
                    self.pool.remove(pos);
                }
            }
        }

        drawn
    }

    fn perform_first_draw(&mut self) {
        self.create_pool();
        let drawn = self.draw_from_pool(self.draw_count);
        self.drawn_balls = drawn;
        self.first_draw_complete = true;

        if self.drawn_balls.len() < 5 {
            self.popup = PopupType::ConfirmRisk;
        } else {
            self.popup = PopupType::None;
        }
    }

    fn perform_risk_draw(&mut self) {
        let remaining = 5 - self.drawn_balls.len();
        if remaining > 0 {
            let additional = self.draw_from_pool(remaining);
            self.drawn_balls.extend(additional);
        }
        self.popup = PopupType::None;
    }
}

fn main() -> Result<(), io::Error> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut app = App::new();
    let res = run_app(&mut terminal, &mut app);

    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        println!("Error: {:?}", err);
    }

    Ok(())
}

fn run_app<B: ratatui::backend::Backend>(
    terminal: &mut Terminal<B>,
    app: &mut App,
) -> io::Result<()> {
    loop {
        terminal.draw(|f| ui(f, app))?;

        if let Event::Key(key) = event::read()? {
            // Controlla se evento e' un KeyDown non un KeyRelease
            if key.kind == KeyEventKind::Press {
                if app.popup != PopupType::None {
                    match key.code {
                        KeyCode::Enter => {
                            if app.popup == PopupType::ConfirmDraw {
                                app.perform_first_draw();
                            } else if app.popup == PopupType::ConfirmRisk {
                                app.perform_risk_draw();
                            }
                        }
                        KeyCode::Esc => {
                            app.popup = PopupType::None;
                        }
                        _ => {}
                    }
                } else {
                    match key.code {
                        KeyCode::Char('q') | KeyCode::Char('Q') => return Ok(()),
                        KeyCode::Char('r') | KeyCode::Char('R') => {
                            app.reset();
                        }
                        KeyCode::Tab => {
                            app.focused_section = match app.focused_section {
                                FocusedSection::WhiteBalls => FocusedSection::RedBalls,
                                FocusedSection::RedBalls => FocusedSection::DrawInput,
                                FocusedSection::DrawInput => FocusedSection::WhiteBalls,
                            };
                        }
                        KeyCode::Up | KeyCode::Right => match app.focused_section {
                            FocusedSection::WhiteBalls => {
                                if app.white_balls < 19 {
                                    app.white_balls += 1;
                                }
                            }
                            FocusedSection::RedBalls => {
                                if app.red_balls < 6 {
                                    app.red_balls += 1;
                                }
                            }
                            FocusedSection::DrawInput => {
                                if app.draw_count < 4 {
                                    app.draw_count += 1;
                                }
                            }
                        },
                        KeyCode::Down | KeyCode::Left => match app.focused_section {
                            FocusedSection::WhiteBalls => {
                                if app.white_balls > 0 {
                                    app.white_balls -= 1;
                                }
                            }
                            FocusedSection::RedBalls => {
                                if app.red_balls > 0 {
                                    app.red_balls -= 1;
                                }
                            }
                            FocusedSection::DrawInput => {
                                if app.draw_count > 1 {
                                    app.draw_count -= 1;
                                }
                            }
                        },
                        KeyCode::Enter => {
                            if app.focused_section == FocusedSection::DrawInput
                                && !app.first_draw_complete
                            {
                                app.popup = PopupType::ConfirmDraw;
                            }
                        }
                        _ => {}
                    }
                }
            }
        }
    }
}

fn ui(f: &mut Frame, app: &App) {
    let main_layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(f.area());

    let left_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(main_layout[0]);

    let right_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(85), Constraint::Percentage(15)])
        .split(main_layout[1]);

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

    // Popup
    if app.popup != PopupType::None {
        draw_popup(f, app);
    }
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
