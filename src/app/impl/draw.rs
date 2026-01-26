use chrono::prelude::*;
use rand::prelude::IndexedRandom;

use super::super::app_state::{App, MAX_DRAW, MIN_DRAW};
use super::super::history::DrawHistory;
use super::super::types::{BallType, PopupType};

impl App {
    /// Reset dello stato dell'applicazione
    pub fn reset(&mut self) {
        use super::super::list::ListSection;
        use super::super::types::FocusedSection;
        
        self.white_balls = 0;
        self.red_balls = 0;
        self.draw_count = 1;
        self.drawn_balls.clear();
        self.pool.clear();
        self.popup = PopupType::None;
        self.current_first_draw.clear();
        self.forced_four_mode = false;
        self.random_mode = false;
        self.focused_section = FocusedSection::WhiteBalls;
        self.used_traits.clear();
        self.selected_node = Some(9); // set selection over archetype
        self.additional_red_balls = [0, 0, 0, 0];
        self.selected_list_item = Some((ListSection::Misfortunes, 0));
    }

    /// Crea il pool di palline per l'estrazione
    pub fn create_pool(&mut self) {
        use BallType::*;
        self.pool.clear();
        if self.random_mode {
            // Random mode: replace white balls with random mix of red and white
            for _ in 0..self.white_balls {
                if rand::random() {
                    self.pool.push(White);
                } else {
                    self.pool.push(Red);
                }
            }
        } else {
            // Normal mode
            for _ in 0..self.white_balls {
                self.pool.push(White);
            }
        }
        for _ in 0..self.red_balls {
            self.pool.push(Red);
        }
    }

    /// Estrae un certo numero di palline dal pool
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

    /// Aggiunge un'estrazione alla cronologia
    fn add_to_log(&mut self, risk: bool, risk_ball: Vec<BallType>) {
        let local: DateTime<Local> = Local::now();
        self.history.push(DrawHistory {
            time: local.format("%A %e %B %Y, %T").to_string(),
            white_balls: self.white_balls,
            traits: self.used_traits.clone(),
            red_balls: self.red_balls,
            misfortunes: self.additional_red_balls,
            first_draw: self.current_first_draw.clone(),
            risked: risk,
            risk_draw: risk_ball,
            confused: self.random_mode,
            adrenalined: self.forced_four_mode,
        });

        self.random_mode = false;
        self.forced_four_mode = false;
        self.used_traits.clear();
        self.additional_red_balls = [0, 0, 0, 0];
    }

    /// Esegue la prima estrazione
    pub fn perform_first_draw(&mut self) {
        use PopupType::*;
        self.create_pool();
        let drawn = self.draw_from_pool(self.draw_count);
        self.drawn_balls = drawn.clone();
        self.current_first_draw = drawn;

        if self.drawn_balls.len() < 5 {
            self.popup = ConfirmRisk;
        } else {
            self.add_to_log(false, Vec::new());
            self.update_vertical_scroll_state();
            self.popup = None;
        }
    }

    /// Esegue l'estrazione di rischio
    pub fn perform_risk_draw(&mut self) {
        use PopupType::*;
        let remaining = 5 - self.drawn_balls.len();
        let mut risk_balls = Vec::new();
        if remaining > 0 {
            let additional = self.draw_from_pool(remaining);
            risk_balls = additional.clone();
            self.drawn_balls.extend(additional);
        }
        self.add_to_log(true, risk_balls);
        self.update_vertical_scroll_state();
        self.popup = None;
    }

    /// Annulla l'estrazione di rischio
    pub fn cancel_draw(&mut self) {
        self.add_to_log(false, Vec::new());
        self.update_vertical_scroll_state();
    }

    /// Aggiorna lo stato della scrollbar verticale per la cronologia
    fn update_vertical_scroll_state(&mut self) {
        let content_height = self.history.len() * 13;
        self.vertical_scroll_state = self.vertical_scroll_state.content_length(content_height);
    }

    /// Incrementa il valore delle palline/estrazioni
    pub fn increment_balls(&mut self) {
        use super::super::app_state::MAX_TOKEN;
        use super::super::types::FocusedSection;
        
        match self.focused_section {
            FocusedSection::WhiteBalls => {
                if self.white_balls < MAX_TOKEN {
                    self.white_balls += 1;
                }
            }
            FocusedSection::RedBalls => {
                if self.red_balls < MAX_TOKEN {
                    self.red_balls += 1;
                }
            }
            FocusedSection::DrawInput => {
                if self.draw_count < MAX_DRAW && !self.forced_four_mode {
                    self.draw_count += 1;
                }
            }
            _ => {}
        }
    }

    /// Decrementa il valore delle palline/estrazioni
    pub fn decrement_balls(&mut self) {
        use super::super::types::FocusedSection;
        
        match self.focused_section {
            FocusedSection::WhiteBalls => {
                if self.white_balls > 0 {
                    self.white_balls -= 1;
                    if !self.used_traits.is_empty() {
                        let _ = self.used_traits.pop();
                    }
                }
            }
            FocusedSection::RedBalls => {
                if self.red_balls > 0 {
                    if self.red_balls > self.additional_red_balls.iter().sum() {
                        self.red_balls -= 1;
                    } else {
                        for (i, d) in self.additional_red_balls.clone().iter().enumerate() {
                            if *d > 0 {
                                self.red_balls -= *d;
                                self.additional_red_balls[i] = 0;
                                break;
                            }
                        }
                    }
                }
            }
            FocusedSection::DrawInput => {
                if self.draw_count > MIN_DRAW && !self.forced_four_mode {
                    self.draw_count -= 1;
                }
            }
            _ => {}
        }
    }
}

