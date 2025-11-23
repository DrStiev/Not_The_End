use rand::prelude::IndexedRandom;
use ratatui::widgets::ScrollbarState;
use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum BallType {
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
pub enum PopupType {
    None,
    ConfirmDraw,
    ConfirmRisk,
}

#[derive(Debug, PartialEq)]
pub enum FocusedSection {
    WhiteBalls,
    RedBalls,
    DrawInput,
}

#[derive(Debug, Clone)]
pub struct DrawHistory {
    pub white_balls: usize,
    pub red_balls: usize,
    pub first_draw: Vec<BallType>,
    pub risked: bool,
    pub risk_draw: Vec<BallType>,
}

impl DrawHistory {
    pub fn format_balls(&self, balls: &[BallType]) -> String {
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

pub struct App {
    pub white_balls: usize,
    pub red_balls: usize,
    pub draw_count: usize,
    pub focused_section: FocusedSection,
    pub popup: PopupType,
    pub drawn_balls: Vec<BallType>,
    pub first_draw_complete: bool,
    pub pool: Vec<BallType>,
    pub current_tab: usize,
    pub history: Vec<DrawHistory>,
    pub current_first_draw: Vec<BallType>,
    pub vertical_scroll: usize,
    pub vertical_scroll_state: ScrollbarState,
}

impl App {
    pub fn new() -> App {
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
            vertical_scroll: 0,
            vertical_scroll_state: ScrollbarState::default(),
        }
    }

    pub fn reset(&mut self) {
        self.white_balls = 0;
        self.red_balls = 0;
        self.draw_count = 1;
        self.drawn_balls.clear();
        self.first_draw_complete = false;
        self.pool.clear();
        self.popup = PopupType::None;
        self.current_first_draw.clear();
    }

    pub fn create_pool(&mut self) {
        self.pool.clear();
        for _ in 0..self.white_balls {
            self.pool.push(BallType::White);
        }
        for _ in 0..self.red_balls {
            self.pool.push(BallType::Red);
        }
    }

    pub fn draw_from_pool(&mut self, count: usize) -> Vec<BallType> {
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

    pub fn perform_first_draw(&mut self) {
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
            self.update_vertical_scroll_state();
            self.popup = PopupType::None;
        }
    }

    pub fn perform_risk_draw(&mut self) {
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
        self.update_vertical_scroll_state();
        self.popup = PopupType::None;
    }

    pub fn cancel_draw(&mut self) {
        // Aggiungi alla cronologia senza rischio
        self.history.push(DrawHistory {
            white_balls: self.white_balls,
            red_balls: self.red_balls,
            first_draw: self.current_first_draw.clone(),
            risked: false,
            risk_draw: Vec::new(),
        });
        self.update_vertical_scroll_state();
    }

    fn update_vertical_scroll_state(&mut self) {
        // Calculate total content height (approximately 13 lines per entry)
        let content_height = self.history.len() * 13;
        self.vertical_scroll_state = self.vertical_scroll_state.content_length(content_height);
    }
}
