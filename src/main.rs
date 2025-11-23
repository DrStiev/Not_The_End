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
    widgets::{Block, Borders, Clear, Paragraph, Tabs, Wrap},
};
use std::fmt;
use std::io;

#[derive(Debug, Clone, Copy, PartialEq)]
enum BallType {
    White,
    Red,
}

impl fmt::Display for BallType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            BallType::White => write!(f, "Successo"),
            BallType::Red => write!(f, "Complicazione"),
        }
    }
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

#[derive(Debug, Clone)]
struct DrawHistory {
    white_balls: usize,
    red_balls: usize,
    first_draw: Vec<BallType>,
    risked: bool,
    risk_draw: Vec<BallType>,
}

impl DrawHistory {
    fn format_balls(&self, balls: &[BallType]) -> String {
        if balls.is_empty() {
            return String::from("-");
        }

        balls
            .iter()
            .map(|b| b.to_string())
            .collect::<Vec<_>>()
            .join(", ")
    }
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
    current_tab: usize,
    history: Vec<DrawHistory>,
    current_first_draw: Vec<BallType>,
    history_scroll: usize,
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
            current_tab: 0,
            history: Vec::new(),
            current_first_draw: Vec::new(),
            history_scroll: 0,
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
        self.current_first_draw.clear();
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
        self.drawn_balls = drawn.clone();
        self.current_first_draw = drawn;
        self.first_draw_complete = true;

        if self.drawn_balls.len() < 5 {
            self.popup = PopupType::ConfirmRisk;
        } else {
            // Aggiungi alla cronologia senza rischio
            self.history.push(DrawHistory {
                white_balls: self.white_balls,
                red_balls: self.red_balls,
                first_draw: self.current_first_draw.clone(),
                risked: false,
                risk_draw: Vec::new(),
            });
            self.popup = PopupType::None;
        }
    }

    fn perform_risk_draw(&mut self) {
        let remaining = 5 - self.drawn_balls.len();
        let mut risk_balls = Vec::new();
        if remaining > 0 {
            let additional = self.draw_from_pool(remaining);
            risk_balls = additional.clone();
            self.drawn_balls.extend(additional);
        }
        // Aggiungi alla cronologia con rischio
        self.history.push(DrawHistory {
            white_balls: self.white_balls,
            red_balls: self.red_balls,
            first_draw: self.current_first_draw.clone(),
            risked: true,
            risk_draw: risk_balls,
        });
        self.popup = PopupType::None;
    }

    fn cancel_draw(&mut self) {
        // Aggiungi alla cronologia senza rischio
        self.history.push(DrawHistory {
            white_balls: self.white_balls,
            red_balls: self.red_balls,
            first_draw: self.current_first_draw.clone(),
            risked: false,
            risk_draw: Vec::new(),
        });
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
                            if app.popup == PopupType::ConfirmRisk {
                                app.cancel_draw();
                            }
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
                        KeyCode::Left => {
                            if app.current_tab > 0 {
                                app.current_tab -= 1;
                            }
                        }
                        KeyCode::Right => {
                            // total number of tabs
                            if app.current_tab < 3 {
                                app.current_tab += 1;
                            }
                        }
                        KeyCode::Tab => {
                            if app.current_tab == 0 {
                                app.focused_section = match app.focused_section {
                                    FocusedSection::WhiteBalls => FocusedSection::RedBalls,
                                    FocusedSection::RedBalls => FocusedSection::DrawInput,
                                    FocusedSection::DrawInput => FocusedSection::WhiteBalls,
                                };
                            }
                        }
                        KeyCode::Up => {
                            if app.current_tab == 0 {
                                match app.focused_section {
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
                                }
                            } else if app.current_tab == 3 {
                                if app.history_scroll > 0 {
                                    app.history_scroll -= 1;
                                }
                            }
                        }
                        KeyCode::Down => {
                            if app.current_tab == 0 {
                                match app.focused_section {
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
                                }
                            } else if app.current_tab == 3 {
                                app.history_scroll += 1;
                            }
                        }
                        KeyCode::Enter => {
                            if app.current_tab == 0
                                && app.focused_section == FocusedSection::DrawInput
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
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(3), Constraint::Min(0)])
        .split(f.area());

    // Tabs
    let tab_titles = vec!["Pesca", "Tab 2", "Tab 3", "Log"];
    let tabs = Tabs::new(tab_titles)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title(" Naviga (←/→) "),
        )
        .select(app.current_tab)
        .style(Style::default().fg(Color::White))
        .highlight_style(
            Style::default()
                .fg(Color::Yellow)
                .add_modifier(Modifier::BOLD),
        );

    f.render_widget(tabs, chunks[0]);

    // Content based on selected tab
    match app.current_tab {
        0 => render_draw_tab(f, chunks[1], app),
        1 => render_empty_tab(f, chunks[1]),
        2 => render_empty_tab(f,chunks[1]),
        3 => render_history_tab(f, chunks[1], app),
        _ => {}
    }

    // Popup
    if app.popup != PopupType::None {
        draw_popup(f, app);
    }
}

fn render_draw_tab(f: &mut Frame, area: Rect, app: &App) {
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

fn render_empty_tab(f: &mut Frame, area: Rect) {
    let block = Block::default().title(" Tab WIP ").borders(Borders::ALL);

    let text = Paragraph::new("Contenuto in arrivo...")
        .block(block)
        .alignment(Alignment::Center);

    f.render_widget(text, area);
}

fn render_history_tab(f: &mut Frame, area: Rect, app: &App) {
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
        .scroll((app.history_scroll as u16, 0))
        .wrap(Wrap { trim: true });

    f.render_widget(paragraph, area);
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
