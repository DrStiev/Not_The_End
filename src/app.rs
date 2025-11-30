use chrono::prelude::*;
use rand::prelude::IndexedRandom;
use ratatui::prelude::Rect;
use ratatui::widgets::ScrollbarState;
use serde::{Deserialize, Serialize};
use std::{fmt, fs};

const DATA_FILE: &str = "character_sheet.toml";

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
    RandomMode,
    ForcedFour,
}

#[derive(Debug, Clone)]
pub struct DrawHistory {
    pub time: String,
    pub white_balls: usize,
    pub red_balls: usize,
    pub first_draw: Vec<BallType>,
    pub risked: bool,
    pub risk_draw: Vec<BallType>,
    pub confused: bool,
    pub adrenalined: bool,
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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HoneycombNode {
    pub text: String,
    #[serde(skip)]
    pub x: i16,
    #[serde(skip)]
    pub y: i16,
    #[serde(skip)]
    pub width: u16,
    #[serde(skip)]
    pub height: u16,
}

#[derive(Debug, Serialize, Deserialize)]
struct HoneycombData {
    nodes: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ListData {
    pub misfortunes: [String; 4],
    pub left_resources: [String; 5],
    pub right_resources: [String; 5],
    pub lessons: [String; 3],
}

impl Default for ListData {
    fn default() -> Self {
        ListData {
            misfortunes: [String::new(), String::new(), String::new(), String::new()],
            left_resources: [
                String::new(),
                String::new(),
                String::new(),
                String::new(),
                String::new(),
            ],
            right_resources: [
                String::new(),
                String::new(),
                String::new(),
                String::new(),
                String::new(),
            ],
            lessons: [String::new(), String::new(), String::new()],
        }
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
    // Log data
    pub history: Vec<DrawHistory>,
    pub current_first_draw: Vec<BallType>,
    pub vertical_scroll: usize,
    pub vertical_scroll_state: ScrollbarState,
    // Areas for mouse interaction
    pub tab_areas: Vec<Rect>,
    pub white_balls_area: Rect,
    pub red_balls_area: Rect,
    pub draw_input_area: Rect,
    pub random_mode_area: Rect,
    pub forced_four_area: Rect,
    pub misfortunes_area: [Rect; 4],
    pub resources_area: [Rect; 2],
    pub lections_area: [Rect; 3],
    // Honeycomb grid
    pub honeycomb_nodes: Vec<HoneycombNode>,
    pub selected_node: Option<usize>,
    pub editing_node: bool,
    pub node_edit_buffer: String,
    pub graph_area: Rect, // memorizzo area grafo per rendering
    // New modes
    pub random_mode: bool,
    pub forced_four_mode: bool,
    // List tab data
    pub list_data: ListData,
    pub selected_list_item: Option<(usize, usize)>, // (section, index)
    pub editing_list_item: bool,
    pub list_edit_buffer: String,
    // Position of cursor in the editor area
    pub cursor_index: usize,
}

impl App {
    pub fn new() -> App {
        let honeycomb_nodes = Self::load_honeycomb_data();
        let list_data = Self::load_list_data();

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
            // Log data
            history: Vec::new(),
            current_first_draw: Vec::new(),
            vertical_scroll: 0,
            vertical_scroll_state: ScrollbarState::default(),
            // Areas for mouse interaction
            tab_areas: Vec::new(),
            white_balls_area: Rect::default(),
            red_balls_area: Rect::default(),
            draw_input_area: Rect::default(),
            random_mode_area: Rect::default(),
            forced_four_area: Rect::default(),
            misfortunes_area: [
                Rect::default(),
                Rect::default(),
                Rect::default(),
                Rect::default(),
            ],
            resources_area: [Rect::default(), Rect::default()],
            lections_area: [Rect::default(), Rect::default(), Rect::default()],
            // Honeycomb grid
            honeycomb_nodes,
            selected_node: None,
            editing_node: false,
            node_edit_buffer: String::new(),
            graph_area: Rect::default(), // memorizzo area grafo per rendering
            // New mode
            random_mode: false,
            forced_four_mode: false,
            // List tab data
            list_data,
            selected_list_item: None,
            editing_list_item: false,
            list_edit_buffer: String::new(),
            // Position of cursor in the editor area
            cursor_index: 0,
        }
    }

    // Returns the byte index based on the character position.
    //
    // Since each character in a string can be contain multiple bytes, it's necessary to calculate
    // the byte index based on the index of the character.
    fn byte_index(&self, input: String) -> usize {
        input
            .char_indices()
            .map(|(i, _)| i)
            .nth(self.cursor_index)
            .unwrap_or(input.len())
    }

    fn clamp_cursor(&self, new_cursor_pos: usize, input: String) -> usize {
        new_cursor_pos.clamp(0, input.chars().count())
    }

    pub fn move_cursor_left(&mut self, input: String) {
        let cursor_moved_left = self.cursor_index.saturating_sub(1);
        self.cursor_index = self.clamp_cursor(cursor_moved_left, input);
    }

    pub fn move_cursor_right(&mut self, input: String) {
        let cursor_moved_right = self.cursor_index.saturating_add(1);
        self.cursor_index = self.clamp_cursor(cursor_moved_right, input);
    }

    pub fn reset_cursor(&mut self) {
        self.cursor_index = 0;
    }

    fn load_honeycomb_data() -> Vec<HoneycombNode> {
        if let Ok(contents) = fs::read_to_string(DATA_FILE) {
            if let Ok(data) = toml::from_str::<HoneycombData>(&contents) {
                return Self::create_honeycomb_layout_with_data(data.nodes);
            }
        }
        Self::create_honeycomb_layout()
    }

    fn load_list_data() -> ListData {
        if let Ok(contents) = fs::read_to_string(DATA_FILE) {
            if let Ok(data) = toml::from_str::<ListData>(&contents) {
                return data;
            }
        }
        ListData::default()
    }

    fn save_data(&self) {
        let data = HoneycombData {
            nodes: self
                .honeycomb_nodes
                .iter()
                .map(|n| n.text.clone())
                .collect(),
        };

        let mut string = String::new();
        if let Ok(toml_string) = toml::to_string_pretty(&data) {
            string.push_str(&toml_string);
        }
        if let Ok(toml_string) = toml::to_string_pretty(&self.list_data) {
            string.push_str(&toml_string);
        }
        let _ = fs::write(DATA_FILE, string);
    }

    fn create_honeycomb_layout() -> Vec<HoneycombNode> {
        let texts = vec![String::new(); 19];
        Self::create_honeycomb_layout_with_data(texts)
    }

    fn create_honeycomb_layout_with_data(texts: Vec<String>) -> Vec<HoneycombNode> {
        let mut nodes = Vec::new();
        let node_width = 12; // 12
        let node_height = 6; // 6
        let spacing_x = 0;
        let spacing_y = 0;
        let total_width = node_width + spacing_x;
        let total_height = node_height + spacing_y;

        //            /‾‾‾\             5 //              |‾‾‾‾|
        //       /‾‾‾\\___//‾‾‾\        4 //        |‾‾‾‾||____||‾‾‾‾|
        //  /‾‾‾\\___//‾‾‾\\___//‾‾‾\   3 //  |‾‾‾‾||____||‾‾‾‾||____||‾‾‾‾|
        //  \___//‾‾‾\\___//‾‾‾\\___/   2 //  |____||‾‾‾‾||____||‾‾‾‾||____|
        //  /‾‾‾\\___//‾‾‾\\___//‾‾‾\   1 //  |‾‾‾‾||____||‾‾‾‾||____||‾‾‾‾|
        //  \___//‾‾‾\\___//‾‾‾\\___/   0 //  |____||‾‾‾‾||____||‾‾‾‾||____|
        //  /‾‾‾\\___//‾‾‾\\___//‾‾‾\  -1 //  |‾‾‾‾||____||‾‾‾‾||____||‾‾‾‾|
        //  \___//‾‾‾\\___//‾‾‾\\___/  -2 //  |____||‾‾‾‾||____||‾‾‾‾||____|
        //       \___//‾‾‾\\___/       -3 //        |____||‾‾‾‾||____|
        //            \___/            -4 //              |____|
        //                                //    -2    -1     0     1     2
        let positions = [
            // column -2
            (-2, -2),
            (-2, 0),
            (-2, 2), // 0,1,2
            // column -1
            (-1, -3),
            (-1, -1),
            (-1, 1),
            (-1, 3), // 3,4,5,6
            // column 0
            (0, -4),
            (0, -2),
            (0, 0),
            (0, 2),
            (0, 4), // 7,8,9,10,11
            // column 1
            (1, -3),
            (1, -1),
            (1, 1),
            (1, 3), // 12,13,14,15
            // column 2
            (2, -2),
            (2, 0),
            (2, 2), // 16,17,18
        ];

        for (i, &(col, row)) in positions.iter().enumerate() {
            let text = if i < texts.len() {
                texts[i].clone()
            } else {
                String::new()
            };

            nodes.push(HoneycombNode {
                text,
                x: col * total_width as i16,
                y: (row * total_height as i16) / 2,
                width: node_width,
                height: node_height,
            });
        }

        nodes
    }

    pub fn handle_node_click(&mut self, x: u16, y: u16, graph_area: &Rect) {
        if self.current_tab != 1 || self.editing_node {
            return;
        }

        let inner_area = Rect {
            x: graph_area.x + 1,
            y: graph_area.y + 1,
            width: graph_area.width.saturating_sub(2),
            height: graph_area.height.saturating_sub(2),
        };

        // Check if area is too small
        if inner_area.width < 30 || inner_area.height < 20 {
            return;
        }

        // Calculate center offset
        let center_x = inner_area.x + inner_area.width / 2;
        let center_y = inner_area.y + inner_area.height / 2;

        for (i, node) in self.honeycomb_nodes.iter().enumerate() {
            let node_x_calc = center_x as i32 + node.x as i32;
            let node_y_calc = center_y as i32 + node.y as i32;

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

    pub fn start_node_editing(&mut self) {
        if let Some(idx) = self.selected_node {
            self.editing_node = true;
            self.node_edit_buffer = self.honeycomb_nodes[idx].text.clone();
        }
    }

    pub fn finish_node_editing(&mut self) {
        if let Some(idx) = self.selected_node {
            self.honeycomb_nodes[idx].text = self.node_edit_buffer.clone();
            self.save_data();
        }
        self.editing_node = false;
        self.node_edit_buffer.clear();
    }

    pub fn start_list_editing(&mut self) {
        if let Some((section, idx)) = self.selected_list_item {
            self.editing_list_item = true;
            self.list_edit_buffer = match section {
                0 => self.list_data.misfortunes[idx].clone(),
                1 => self.list_data.left_resources[idx].clone(),
                2 => self.list_data.right_resources[idx].clone(),
                3 => self.list_data.lessons[idx].clone(),
                _ => String::new(),
            };
        }
    }

    pub fn finish_list_editing(&mut self) {
        if let Some((section, idx)) = self.selected_list_item {
            match section {
                0 => self.list_data.misfortunes[idx] = self.list_edit_buffer.clone(),
                1 => self.list_data.left_resources[idx] = self.list_edit_buffer.clone(),
                2 => self.list_data.right_resources[idx] = self.list_edit_buffer.clone(),
                3 => self.list_data.lessons[idx] = self.list_edit_buffer.clone(),
                _ => {}
            }
            self.save_data();
        }
        self.editing_list_item = false;
        self.list_edit_buffer.clear();
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
        self.forced_four_mode = false;
        self.random_mode = false;
    }

    pub fn create_pool(&mut self) {
        self.pool.clear();
        if self.random_mode {
            // Random mode: replace white balls with random mix of red and white
            for _ in 0..self.white_balls {
                if rand::random() {
                    self.pool.push(BallType::White);
                } else {
                    self.pool.push(BallType::Red);
                }
            }
        } else {
            // Normal mode
            for _ in 0..self.white_balls {
                self.pool.push(BallType::White);
            }
            for _ in 0..self.red_balls {
                self.pool.push(BallType::Red);
            }
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
            let local: DateTime<Local> = Local::now();
            // Aggiungi alla cronologia senza rischio
            self.history.push(DrawHistory {
                time: local.format("%A %e %B %Y, %T").to_string(),
                white_balls: self.white_balls,
                red_balls: self.red_balls,
                first_draw: self.current_first_draw.clone(),
                risked: false,
                risk_draw: Vec::new(),
                confused: self.random_mode,
                adrenalined: self.forced_four_mode,
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
        let local: DateTime<Local> = Local::now();
        // Aggiungi alla cronologia con rischio
        self.history.push(DrawHistory {
            time: local.format("%A %e %B %Y, %T").to_string(),
            white_balls: self.white_balls,
            red_balls: self.red_balls,
            first_draw: self.current_first_draw.clone(),
            risked: true,
            risk_draw: risk_balls,
            confused: self.random_mode,
            adrenalined: self.forced_four_mode,
        });
        self.update_vertical_scroll_state();
        self.popup = PopupType::None;
    }

    pub fn cancel_draw(&mut self) {
        let local: DateTime<Local> = Local::now();
        // Aggiungi alla cronologia senza rischio
        self.history.push(DrawHistory {
            time: local.format("%A %e %B %Y, %T").to_string(),
            white_balls: self.white_balls,
            red_balls: self.red_balls,
            first_draw: self.current_first_draw.clone(),
            risked: false,
            risk_draw: Vec::new(),
            confused: self.random_mode,
            adrenalined: self.forced_four_mode,
        });
        self.update_vertical_scroll_state();
    }

    fn update_vertical_scroll_state(&mut self) {
        // Calculate total content height (approximately 13 lines per entry)
        let content_height = self.history.len() * 13;
        self.vertical_scroll_state = self.vertical_scroll_state.content_length(content_height);
    }

    pub fn handle_mouse_click(&mut self, x: u16, y: u16) {
        // Check tab clicks
        for (i, area) in self.tab_areas.iter().enumerate() {
            if x >= area.x && x < area.x + area.width && y >= area.y && y < area.y + area.height {
                self.current_tab = i;
                return;
            }
        }

        // Tab 0 specific areas
        if self.current_tab == 0 {
            if is_inside(x, y, &self.white_balls_area) {
                self.focused_section = FocusedSection::WhiteBalls;
            } else if is_inside(x, y, &self.red_balls_area) {
                self.focused_section = FocusedSection::RedBalls;
            } else if is_inside(x, y, &self.draw_input_area) {
                self.focused_section = FocusedSection::DrawInput;
            } else if is_inside(x, y, &self.random_mode_area) {
                self.random_mode = !self.random_mode;
            } else if is_inside(x, y, &self.forced_four_area) {
                self.forced_four_mode = !self.forced_four_mode;
                if self.forced_four_mode {
                    self.draw_count = 4;
                } else {
                    self.draw_count = 1;
                }
            }
        } else if self.current_tab == 2 {
            // Tab 2 specific areas
            for idx in 0..4 {
                if is_inside(x, y, &self.misfortunes_area[idx]) && !self.editing_list_item {
                    self.selected_list_item = Some((0, idx));
                }
            }
            for idx in 0..2 {
                if is_inside(x, y, &self.resources_area[idx]) && !self.editing_list_item {
                    self.selected_list_item = Some((idx + 1, 0));
                }
            }
            for idx in 0..3 {
                if is_inside(x, y, &self.lections_area[idx]) && !self.editing_list_item {
                    self.selected_list_item = Some((3, idx));
                }
            }
        }
    }
}

fn is_inside(x: u16, y: u16, area: &Rect) -> bool {
    x >= area.x && x < area.x + area.width && y >= area.y && y < area.y + area.height
}
