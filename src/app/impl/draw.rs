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
        use PopupType::*;
        self.add_to_log(false, Vec::new());
        self.update_vertical_scroll_state();
        self.popup = None;
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

#[cfg(test)]
mod draw_tests {
    use crate::app::{App, BallType, FocusedSection, MAX_DRAW, MAX_TOKEN, MIN_DRAW, PopupType};

    #[test]
    fn test_reset() {
        let mut app = App::new();
        app.white_balls = 5;
        app.red_balls = 3;
        app.drawn_balls = vec![BallType::White, BallType::Red];
        app.random_mode = true;
        app.forced_four_mode = true;

        app.reset();

        assert_eq!(app.white_balls, 0);
        assert_eq!(app.red_balls, 0);
        assert_eq!(app.draw_count, 1);
        assert!(app.drawn_balls.is_empty());
        assert!(app.pool.is_empty());
        assert!(!app.random_mode);
        assert!(!app.forced_four_mode);
    }

    #[test]
    fn test_create_pool_normal_mode() {
        let mut app = App::new();
        app.white_balls = 5;
        app.red_balls = 3;
        app.random_mode = false;

        app.create_pool();

        assert_eq!(app.pool.len(), 8);
        let white_count = app.pool.iter().filter(|&&b| b == BallType::White).count();
        let red_count = app.pool.iter().filter(|&&b| b == BallType::Red).count();
        assert_eq!(white_count, 5);
        assert_eq!(red_count, 3);
    }

    #[test]
    fn test_create_pool_random_mode() {
        let mut app = App::new();
        app.white_balls = 10;
        app.red_balls = 5;
        app.random_mode = true;

        app.create_pool();

        assert_eq!(app.pool.len(), 15);
        // In random mode, white balls become random mix of white/red
    }

    #[test]
    fn test_draw_from_pool() {
        let mut app = App::new();
        app.white_balls = 10;
        app.red_balls = 5;
        app.create_pool();

        let initial_pool_size = app.pool.len();
        let drawn = app.draw_from_pool(3);

        assert_eq!(drawn.len(), 3);
        assert_eq!(app.pool.len(), initial_pool_size - 3);
    }

    #[test]
    fn test_draw_from_pool_more_than_available() {
        let mut app = App::new();
        app.white_balls = 2;
        app.red_balls = 1;
        app.create_pool();

        let drawn = app.draw_from_pool(10);

        assert_eq!(drawn.len(), 3); // Can only draw what's available
        assert!(app.pool.is_empty());
    }

    #[test]
    fn test_draw_from_empty_pool() {
        let mut app = App::new();
        app.create_pool();

        let drawn = app.draw_from_pool(5);

        assert!(drawn.is_empty());
    }

    #[test]
    fn test_perform_first_draw_with_risk() {
        let mut app = App::new();
        app.white_balls = 5;
        app.red_balls = 3;
        app.draw_count = 2;

        app.perform_first_draw();

        assert_eq!(app.drawn_balls.len(), 2);
        assert_eq!(app.popup, PopupType::ConfirmRisk);

        app.perform_risk_draw();
        assert_eq!(app.drawn_balls.len(), 5);
        assert_eq!(app.popup, PopupType::None);
    }

    #[test]
    fn test_perform_first_draw_without_risk() {
        let mut app = App::new();
        app.white_balls = 10;
        app.red_balls = 5;
        app.draw_count = 4;
        app.forced_four_mode = true;

        app.perform_first_draw();

        assert_eq!(app.drawn_balls.len(), 4);
        assert_eq!(app.popup, PopupType::ConfirmRisk);

        app.cancel_draw();
        assert_eq!(app.popup, PopupType::None);
    }

    #[test]
    fn test_increment_balls_white() {
        let mut app = App::new();
        app.focused_section = FocusedSection::WhiteBalls;
        app.white_balls = 5;

        app.increment_balls();

        assert_eq!(app.white_balls, 6);
    }

    #[test]
    fn test_increment_balls_white_max() {
        let mut app = App::new();
        app.focused_section = FocusedSection::WhiteBalls;
        app.white_balls = MAX_TOKEN;

        app.increment_balls();

        assert_eq!(app.white_balls, MAX_TOKEN);
    }

    #[test]
    fn test_increment_balls_red() {
        let mut app = App::new();
        app.focused_section = FocusedSection::RedBalls;
        app.red_balls = 2;

        app.increment_balls();

        assert_eq!(app.red_balls, 3);
    }

    #[test]
    fn test_increment_draw_count() {
        let mut app = App::new();
        app.focused_section = FocusedSection::DrawInput;
        app.draw_count = 2;
        app.forced_four_mode = false;

        app.increment_balls();

        assert_eq!(app.draw_count, 3);
    }

    #[test]
    fn test_increment_draw_count_max() {
        let mut app = App::new();
        app.focused_section = FocusedSection::DrawInput;
        app.draw_count = MAX_DRAW;
        app.forced_four_mode = false;

        app.increment_balls();

        assert_eq!(app.draw_count, MAX_DRAW);
    }

    #[test]
    fn test_increment_draw_count_forced_four() {
        let mut app = App::new();
        app.focused_section = FocusedSection::DrawInput;
        app.draw_count = 4;
        app.forced_four_mode = true;

        app.increment_balls();

        assert_eq!(app.draw_count, 4); // Should not change in forced four mode
    }

    #[test]
    fn test_decrement_balls_white() {
        let mut app = App::new();
        app.focused_section = FocusedSection::WhiteBalls;
        app.white_balls = 5;

        app.decrement_balls();

        assert_eq!(app.white_balls, 4);
    }

    #[test]
    fn test_decrement_balls_white_min() {
        let mut app = App::new();
        app.focused_section = FocusedSection::WhiteBalls;
        app.white_balls = 0;

        app.decrement_balls();

        assert_eq!(app.white_balls, 0);
    }

    #[test]
    fn test_decrement_balls_red() {
        let mut app = App::new();
        app.focused_section = FocusedSection::RedBalls;
        app.red_balls = 5;

        app.decrement_balls();

        assert_eq!(app.red_balls, 4);
    }

    #[test]
    fn test_decrement_draw_count() {
        let mut app = App::new();
        app.focused_section = FocusedSection::DrawInput;
        app.draw_count = 3;
        app.forced_four_mode = false;

        app.decrement_balls();

        assert_eq!(app.draw_count, 2);
    }

    #[test]
    fn test_decrement_draw_count_min() {
        let mut app = App::new();
        app.focused_section = FocusedSection::DrawInput;
        app.draw_count = MIN_DRAW;
        app.forced_four_mode = false;

        app.decrement_balls();

        assert_eq!(app.draw_count, MIN_DRAW);
    }

    #[test]
    fn test_perform_risk_draw() {
        let mut app = App::new();
        app.white_balls = 10;
        app.red_balls = 5;
        app.draw_count = 2;
        app.perform_first_draw();

        assert_eq!(app.drawn_balls.len(), 2);
        assert_eq!(app.popup, PopupType::ConfirmRisk);

        app.perform_risk_draw();

        assert_eq!(app.drawn_balls.len(), 5);
        assert_eq!(app.popup, PopupType::None);
    }

    #[test]
    fn test_cancel_draw() {
        let mut app = App::new();
        app.white_balls = 5;
        app.red_balls = 3;
        app.draw_count = 2;
        app.perform_first_draw();

        let history_len = app.history.len();
        app.cancel_draw();

        assert_eq!(app.history.len(), history_len + 1);
        assert_eq!(app.popup, PopupType::None);
    }
}
