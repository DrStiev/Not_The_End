use super::super::app_state::App;
use super::super::list::ListSection;

impl App {
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
