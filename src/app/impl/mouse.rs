use ratatui::prelude::Rect;

use super::super::app_state::App;
use super::super::character::CharacterSection;
use super::super::list::ListSection;
use super::super::types::{FocusedSection, TabType};

impl App {
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

    /// Gestisce il click del mouse
    pub fn handle_mouse_click(&mut self, x: u16, y: u16) {
        use super::super::list::get_section_type;
        use super::super::types::get_tab_type;

        // Check tab clicks
        for (i, area) in self.tab_areas.iter().enumerate() {
            if is_inside(x, y, area) {
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
                    self.focused_section = RandomMode;
                } else if is_inside(x, y, &self.forced_four_area) {
                    self.focused_section = ForcedFour;
                }
            }
            TabType::AdditionalInfoTab => {
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
}

/// Verifica se un punto (x, y) è all'interno di un'area
fn is_inside(x: u16, y: u16, area: &Rect) -> bool {
    x >= area.x && x < area.x + area.width && y >= area.y && y < area.y + area.height
}
