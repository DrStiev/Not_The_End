use chrono::prelude::*;
use rand::prelude::IndexedRandom;
use ratatui::prelude::Rect;
use std::fs;

use super::app_state::{App, MAX_DRAW, MAX_TOKEN, MIN_DRAW};
use super::character::CharacterSection;
use super::history::DrawHistory;
use super::honeycomb::HoneycombData;
use super::list::ListSection;
use super::types::{BallType, FocusedSection, PopupType, TabType};

const DATA_FILE: &str = "character_sheet.toml";

impl App {
    /// Salva i dati su file TOML
    fn save_data(&self) {
        let data = HoneycombData {
            nodes: self
                .honeycomb_nodes
                .iter()
                .map(|n| n.text.clone())
                .collect(),
        };

        let mut string = String::new();
        if let Ok(toml_string) = toml::to_string_pretty(&self.character_base_info) {
            string.push_str(&toml_string);
        }
        if let Ok(toml_string) = toml::to_string_pretty(&data) {
            string.push_str(&toml_string);
        }
        if let Ok(toml_string) = toml::to_string_pretty(&self.list_data) {
            string.push_str(&toml_string);
        }
        let _ = fs::write(DATA_FILE, string);
    }

    /// Gestisce il click del mouse sulle informazioni del personaggio
    pub fn handle_character_click(&mut self, x: u16, y: u16) {
        use CharacterSection::*;
        if self.current_tab != TabType::CharacterSheetTab || self.editing_character_info {
            return;
        }

        if is_inside(x, y, &self.character_name_area) {
            self.selected_character_info = CharacterName;
        } else if is_inside(x, y, &self.character_objective_area) {
            self.selected_character_info = CharacterObjective;
        } else {
            self.selected_character_info = None;
        }
    }

    /// Gestisce il click del mouse sui nodi della griglia esagonale
    pub fn handle_node_click(&mut self, x: u16, y: u16) {
        if self.current_tab != TabType::CharacterSheetTab || self.editing_node {
            return;
        }

        let inner_area = self.graph_area;

        // Check if area is too small
        if inner_area.width < 20 || inner_area.height < 10 {
            return;
        }

        // Calculate center offset
        let center_x = (inner_area.x / 2 + inner_area.width) / 2;
        let center_y = (inner_area.y / 2 + inner_area.height) / 2;

        for (i, node) in self.honeycomb_nodes.iter().enumerate() {
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

            if x >= node_x && x < node_x + node.width && y >= node_y && y < node_y + node.height {
                self.selected_node = Some(i);
                return;
            }
        }
    }

    /// Inizia la modifica delle informazioni del personaggio
    pub fn start_character_editing(&mut self) {
        if self.selected_character_info != CharacterSection::None {
            if self.selected_character_info == CharacterSection::CharacterName {
                self.character_edit_buffer = self.character_base_info.name.clone();
            } else {
                self.character_edit_buffer = self.character_base_info.objective.clone();
            }
            self.editing_character_info = true;
        }
    }

    /// Termina la modifica delle informazioni del personaggio
    pub fn finish_character_editing(&mut self) {
        if self.selected_character_info != CharacterSection::None {
            if self.selected_character_info == CharacterSection::CharacterName {
                self.character_base_info.name = self.character_edit_buffer.clone();
            } else {
                self.character_base_info.objective = self.character_edit_buffer.clone();
            }
            self.save_data();
            self.editing_character_info = false;
            self.character_edit_buffer.clear();
        }
    }

    /// Inizia la modifica di un nodo della griglia esagonale
    pub fn start_node_editing(&mut self) {
        if let Some(idx) = self.selected_node {
            self.editing_node = true;
            self.node_edit_buffer = self.honeycomb_nodes[idx].text.clone();
        }
    }

    /// Termina la modifica di un nodo della griglia esagonale
    pub fn finish_node_editing(&mut self) {
        if let Some(idx) = self.selected_node {
            self.honeycomb_nodes[idx].text = self.node_edit_buffer.clone().trim().to_string();
            self.save_data();
        }
        self.editing_node = false;
        self.node_edit_buffer.clear();
    }

    /// Inizia la modifica di un elemento della lista
    pub fn start_list_editing(&mut self) {
        use ListSection::*;
        if let Some((section, idx)) = self.selected_list_item {
            self.editing_list_item = true;
            self.list_edit_buffer = match section {
                Misfortunes => self.list_data.misfortunes[idx].clone(),
                MisfortunesDifficult => self.list_data.misfortunes_red_balls[idx].clone(),
                LxResources => self.list_data.left_resources[idx].clone(),
                Notes => self.list_data.notes.clone(),
                Lessons => self.list_data.lessons[idx].clone(),
            };
        }
    }

    /// Termina la modifica di un elemento della lista
    pub fn finish_list_editing(&mut self) {
        use ListSection::*;
        if let Some((section, idx)) = self.selected_list_item {
            match section {
                Misfortunes => {
                    self.list_data.misfortunes[idx] =
                        self.list_edit_buffer.clone().trim().to_string()
                }
                MisfortunesDifficult => {
                    self.list_data.misfortunes_red_balls[idx] =
                        self.list_edit_buffer.clone().trim().to_string()
                }
                LxResources => {
                    self.list_data.left_resources[idx] =
                        self.list_edit_buffer.clone().trim().to_string()
                }
                Notes => {
                    self.list_data.notes = self.list_edit_buffer.clone().trim().to_string();
                    self.update_notes_vertical_scroll_state();
                }
                Lessons => {
                    self.list_data.lessons[idx] = self.list_edit_buffer.clone().trim().to_string();
                    self.update_list_vertical_scroll_state(idx)
                }
            }
            self.save_data();
        }
        self.editing_list_item = false;
        self.list_edit_buffer.clear();
    }

    /// Reset dello stato dell'applicazione
    pub fn reset(&mut self) {
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

    /// Aggiorna lo stato della scrollbar verticale per le lezioni
    pub fn update_list_vertical_scroll_state(&mut self, idx: usize) {
        let width = self.lections_area[idx % 3].width;
        let content_height = self.list_data.lessons[idx % 3].len() / width as usize;
        self.list_vertical_scroll_state[idx % 3] =
            self.list_vertical_scroll_state[idx % 3].content_length(content_height);
    }

    /// Aggiorna lo stato della scrollbar verticale per le note
    pub fn update_notes_vertical_scroll_state(&mut self) {
        let width = self.resources_area[1].width;
        let content_height = self.list_data.notes.len() / width as usize;
        self.notes_vertical_scroll_state = self
            .notes_vertical_scroll_state
            .content_length(content_height);
    }

    /// Gestisce il click del mouse
    pub fn handle_mouse_click(&mut self, x: u16, y: u16) {
        // Check tab clicks
        for (i, area) in self.tab_areas.iter().enumerate() {
            if is_inside(x, y, area) {
                use super::types::get_tab_type;
                self.current_tab = get_tab_type(i);
                return;
            }
        }

        match self.current_tab {
            TabType::DrawTab => {
                use FocusedSection::*;
                if is_inside(x, y, &self.white_balls_area) {
                    self.focused_section = WhiteBalls;
                } else if is_inside(x, y, &self.red_balls_area) {
                    self.focused_section = RedBalls;
                } else if is_inside(x, y, &self.draw_input_area) {
                    self.focused_section = DrawInput;
                } else if is_inside(x, y, &self.random_mode_area) {
                    self.random_mode = !self.random_mode;
                } else if is_inside(x, y, &self.forced_four_area) {
                    self.forced_four_mode = !self.forced_four_mode;
                    if self.forced_four_mode {
                        self.draw_count = MAX_DRAW;
                    } else {
                        self.draw_count = MIN_DRAW;
                    }
                }
            }
            TabType::AdditionalInfoTab => {
                use super::list::get_section_type;
                use ListSection::*;

                for idx in 0..4 {
                    if !self.editing_list_item {
                        if idx < 2 && is_inside(x, y, &self.resources_area[idx]) {
                            self.selected_list_item = Some((get_section_type(idx + 2), 0));
                            if idx == 1 {
                                self.update_notes_vertical_scroll_state();
                            }
                        } else if idx < 3 && is_inside(x, y, &self.lections_area[idx]) {
                            self.selected_list_item = Some((Lessons, idx));
                            self.update_list_vertical_scroll_state(idx);
                        } else if is_inside(x, y, &self.misfortunes_area[idx]) {
                            self.selected_list_item = Some((Misfortunes, idx));
                        } else if is_inside(x, y, &self.misfortunes_red_balls_area[idx]) {
                            self.selected_list_item = Some((MisfortunesDifficult, idx));
                        }
                    }
                }
            }
            _ => {}
        }
    }

    /// Incrementa il valore delle palline/estrazioni
    pub fn increment_balls(&mut self) {
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

    /// Passa alla sezione successiva nelle liste
    pub fn next_section(&mut self) {
        use ListSection::*;
        if let Some((section, idx)) = self.selected_list_item {
            match section {
                Misfortunes | MisfortunesDifficult => {
                    if idx < 3 {
                        self.selected_list_item = Some((section, idx + 1));
                    } else {
                        self.selected_list_item = Some((section.next(), 0));
                    }
                }
                LxResources | Notes => {
                    self.selected_list_item = Some((section.next(), 0));
                }
                Lessons => {
                    if idx < 2 {
                        self.selected_list_item = Some((section, idx + 1));
                    } else {
                        self.selected_list_item = Some((section.next(), 0));
                    }
                }
            }
        }
    }

    /// Passa alla sezione precedente nelle liste
    pub fn prev_section(&mut self) {
        use ListSection::*;
        if let Some((section, idx)) = self.selected_list_item {
            match section {
                Misfortunes => {
                    if idx > 0 {
                        self.selected_list_item = Some((section, idx - 1));
                    } else {
                        self.selected_list_item = Some((section.prev(), 2));
                    }
                }
                MisfortunesDifficult => {
                    if idx > 0 {
                        self.selected_list_item = Some((section, idx - 1));
                    } else {
                        self.selected_list_item = Some((section.prev(), 3));
                    }
                }
                LxResources => {
                    self.selected_list_item = Some((section.prev(), 3));
                }
                Notes => {
                    self.selected_list_item = Some((section.prev(), 0));
                }
                Lessons => {
                    if idx > 0 {
                        self.selected_list_item = Some((section, idx - 1));
                    } else {
                        self.selected_list_item = Some((section.prev(), 0));
                    }
                }
            }
        }
    }

    /// Muove la selezione verso l'alto nelle liste
    pub fn up_section(&mut self) {
        use ListSection::*;
        if let Some((section, idx)) = self.selected_list_item {
            match section {
                Misfortunes | MisfortunesDifficult => {
                    self.selected_list_item = Some((section.vertical(), idx));
                }
                LxResources => {
                    if idx > 0 {
                        self.selected_list_item = Some((section, idx - 1));
                    } else {
                        self.selected_list_item = Some((section, 9 - idx));
                    }
                }
                Notes => {
                    self.notes_vertical_scroll = self.notes_vertical_scroll.saturating_sub(1);
                    self.notes_vertical_scroll_state = self
                        .notes_vertical_scroll_state
                        .position(self.notes_vertical_scroll);
                }
                Lessons => {
                    self.list_vertical_scroll[idx] =
                        self.list_vertical_scroll[idx].saturating_sub(1);
                    self.list_vertical_scroll_state[idx] = self.list_vertical_scroll_state[idx]
                        .position(self.list_vertical_scroll[idx]);
                }
            }
        }
    }

    /// Muove la selezione verso il basso nelle liste
    pub fn down_section(&mut self) {
        use ListSection::*;
        if let Some((section, idx)) = self.selected_list_item {
            match section {
                Misfortunes | MisfortunesDifficult => {
                    self.selected_list_item = Some((section.vertical(), idx))
                }
                LxResources => {
                    self.selected_list_item = Some((section, (idx + 1) % 10));
                }
                Notes => {
                    if self.notes_vertical_scroll
                        < self.list_data.notes.len() / self.resources_area[1].width as usize
                    {
                        self.notes_vertical_scroll = self.notes_vertical_scroll.saturating_add(1);
                        self.notes_vertical_scroll_state = self
                            .notes_vertical_scroll_state
                            .position(self.notes_vertical_scroll);
                    }
                }
                Lessons => {
                    if self.list_vertical_scroll[idx]
                        < self.list_data.lessons[idx].len() / self.lections_area[idx].width as usize
                    {
                        self.list_vertical_scroll[idx] =
                            self.list_vertical_scroll[idx].saturating_add(1);
                        self.list_vertical_scroll_state[idx] = self.list_vertical_scroll_state[idx]
                            .position(self.list_vertical_scroll[idx]);
                    }
                }
            }
        }
    }

    /// Passa al nodo esagonale successivo (destra)
    pub fn next_hex(&mut self) {
        if let Some(idx) = self.selected_node {
            match idx {
                0..7 | 11..13 => {
                    self.selected_node = Some(idx + 4);
                }
                7..11 => {
                    self.selected_node = Some(idx + 5);
                }
                13..16 => {
                    self.selected_node = Some(idx + 3);
                }
                16..19 => {
                    self.selected_node = Some(idx - 16);
                }
                _ => {}
            }
        }
    }

    /// Passa al nodo esagonale precedente (sinistra)
    pub fn prev_hex(&mut self) {
        if let Some(idx) = self.selected_node {
            match idx {
                6..8 | 12..19 => {
                    self.selected_node = Some(idx - 4);
                }
                8..12 => {
                    self.selected_node = Some(idx - 5);
                }
                3..6 => {
                    self.selected_node = Some(idx - 3);
                }
                0..3 => {
                    self.selected_node = Some(idx + 16);
                }
                _ => {}
            }
        }
    }

    /// Muove la selezione verso l'alto nella griglia esagonale
    pub fn up_hex(&mut self) {
        if let Some(idx) = self.selected_node {
            match idx {
                0 => {
                    self.selected_node = Some(2);
                }
                3 => {
                    self.selected_node = Some(6);
                }
                7 => {
                    self.selected_node = Some(11);
                }
                12 => {
                    self.selected_node = Some(15);
                }
                16 => {
                    self.selected_node = Some(18);
                }
                _ => {
                    self.selected_node = Some(idx - 1);
                }
            }
        }
    }

    /// Muove la selezione verso il basso nella griglia esagonale
    pub fn down_hex(&mut self) {
        if let Some(idx) = self.selected_node {
            match idx {
                2 => {
                    self.selected_node = Some(0);
                }
                6 => {
                    self.selected_node = Some(3);
                }
                11 => {
                    self.selected_node = Some(7);
                }
                15 => {
                    self.selected_node = Some(12);
                }
                18 => {
                    self.selected_node = Some(16);
                }
                _ => {
                    self.selected_node = Some(idx + 1);
                }
            }
        }
    }
}

/// Verifica se un punto (x, y) è all'interno di un'area
fn is_inside(x: u16, y: u16, area: &Rect) -> bool {
    x >= area.x && x < area.x + area.width && y >= area.y && y < area.y + area.height
}
