use std::fs;

use super::super::app_state::App;
use super::super::character::CharacterSection;
use super::super::honeycomb::HoneycombData;
use super::super::list::ListSection;

const DATA_FILE: &str = "character_sheet.toml";

impl App {
    /// Salva i dati su file TOML
    pub(crate) fn save_data(&self) {
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
}