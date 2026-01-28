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

#[cfg(test)]
mod navigation_tests {
    use crate::app::{App, ListSection};

    #[test]
    fn test_next_section_misfortunes() {
        let mut app = App::new();
        app.selected_list_item = Some((ListSection::Misfortunes, 0));

        app.next_section();

        assert_eq!(app.selected_list_item, Some((ListSection::Misfortunes, 1)));
    }

    #[test]
    fn test_next_section_misfortunes_wrap() {
        let mut app = App::new();
        app.selected_list_item = Some((ListSection::Misfortunes, 3));

        app.next_section();

        assert_eq!(
            app.selected_list_item,
            Some((ListSection::MisfortunesDifficult, 0))
        );
    }

    #[test]
    fn test_next_section_lessons() {
        let mut app = App::new();
        app.selected_list_item = Some((ListSection::Lessons, 0));

        app.next_section();

        assert_eq!(app.selected_list_item, Some((ListSection::Lessons, 1)));
    }

    #[test]
    fn test_next_section_lessons_wrap() {
        let mut app = App::new();
        app.selected_list_item = Some((ListSection::Lessons, 2));

        app.next_section();

        assert_eq!(app.selected_list_item, Some((ListSection::Misfortunes, 0)));
    }

    #[test]
    fn test_prev_section_misfortunes() {
        let mut app = App::new();
        app.selected_list_item = Some((ListSection::Misfortunes, 1));

        app.prev_section();

        assert_eq!(app.selected_list_item, Some((ListSection::Misfortunes, 0)));
    }

    #[test]
    fn test_prev_section_misfortunes_wrap() {
        let mut app = App::new();
        app.selected_list_item = Some((ListSection::Misfortunes, 0));

        app.prev_section();

        assert_eq!(app.selected_list_item, Some((ListSection::Lessons, 2)));
    }

    #[test]
    fn test_next_hex() {
        let mut app = App::new();
        app.selected_node = Some(0);

        app.next_hex();

        assert_eq!(app.selected_node, Some(4));
    }

    #[test]
    fn test_next_hex_wrap() {
        let mut app = App::new();
        app.selected_node = Some(18);

        app.next_hex();

        assert_eq!(app.selected_node, Some(2));
    }

    #[test]
    fn test_prev_hex() {
        let mut app = App::new();
        app.selected_node = Some(4);

        app.prev_hex();

        assert_eq!(app.selected_node, Some(1));
    }

    #[test]
    fn test_prev_hex_wrap() {
        let mut app = App::new();
        app.selected_node = Some(0);

        app.prev_hex();

        assert_eq!(app.selected_node, Some(16));
    }

    #[test]
    fn test_up_hex() {
        let mut app = App::new();
        app.selected_node = Some(5);

        app.up_hex();

        assert_eq!(app.selected_node, Some(4));
    }

    #[test]
    fn test_up_hex_wrap() {
        let mut app = App::new();
        app.selected_node = Some(0);

        app.up_hex();

        assert_eq!(app.selected_node, Some(2));
    }

    #[test]
    fn test_down_hex() {
        let mut app = App::new();
        app.selected_node = Some(4);

        app.down_hex();

        assert_eq!(app.selected_node, Some(5));
    }

    #[test]
    fn test_down_hex_wrap() {
        let mut app = App::new();
        app.selected_node = Some(2);

        app.down_hex();

        assert_eq!(app.selected_node, Some(0));
    }

    #[test]
    fn test_up_section_resources() {
        let mut app = App::new();
        app.selected_list_item = Some((ListSection::LxResources, 5));

        app.up_section();

        assert_eq!(app.selected_list_item, Some((ListSection::LxResources, 4)));
    }

    #[test]
    fn test_up_section_resources_wrap() {
        let mut app = App::new();
        app.selected_list_item = Some((ListSection::LxResources, 0));

        app.up_section();

        assert_eq!(app.selected_list_item, Some((ListSection::LxResources, 9)));
    }

    #[test]
    fn test_down_section_resources() {
        let mut app = App::new();
        app.selected_list_item = Some((ListSection::LxResources, 4));

        app.down_section();

        assert_eq!(app.selected_list_item, Some((ListSection::LxResources, 5)));
    }

    #[test]
    fn test_down_section_resources_wrap() {
        let mut app = App::new();
        app.selected_list_item = Some((ListSection::LxResources, 9));

        app.down_section();

        assert_eq!(app.selected_list_item, Some((ListSection::LxResources, 0)));
    }

    #[test]
    fn test_hex_navigation_cycle() {
        let mut app = App::new();
        app.selected_node = Some(9);

        // Test full cycle
        for _ in 0..19 {
            app.next_hex();
        }
        // Should eventually come back to a valid position
        assert!(app.selected_node.is_some());
    }
}
