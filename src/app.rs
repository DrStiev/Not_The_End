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

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum PopupType {
    None,
    ConfirmDraw,
    ConfirmRisk,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TabType {
    DrawTab,
    CharacterSheetTab,
    AdditionalInfoTab,
    LogTab,
    None, // default
}

impl TabType {
    pub fn next(&self) -> Self {
        use TabType::*;
        match *self {
            DrawTab => CharacterSheetTab,
            CharacterSheetTab => AdditionalInfoTab,
            AdditionalInfoTab => LogTab,
            LogTab => DrawTab,
            _ => DrawTab,
        }
    }

    pub fn idx(&self) -> usize {
        use TabType::*;
        match *self {
            DrawTab => 0,
            CharacterSheetTab => 1,
            AdditionalInfoTab => 2,
            LogTab => 3,
            None => 0, // if not valid return 0 as default
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum FocusedSection {
    WhiteBalls,
    RedBalls,
    DrawInput,
    RandomMode,
    ForcedFour,
}

impl FocusedSection {
    pub fn next(&self) -> Self {
        use FocusedSection::*;
        match *self {
            WhiteBalls => RedBalls,
            RedBalls => RandomMode,
            DrawInput => WhiteBalls,
            RandomMode => ForcedFour,
            ForcedFour => DrawInput,
        }
    }
    pub fn prev(&self) -> Self {
        use FocusedSection::*;
        match *self {
            WhiteBalls => DrawInput,
            RedBalls => WhiteBalls,
            DrawInput => ForcedFour,
            RandomMode => RedBalls,
            ForcedFour => RandomMode,
        }
    }
}

#[derive(Debug, Clone)]
pub struct DrawHistory {
    pub time: String,
    pub white_balls: usize,
    pub traits: Vec<usize>,
    pub red_balls: usize,
    pub misfortunes: [usize; 4],
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
    pub misfortunes_red_balls: [String; 4],
    pub left_resources: [String; 5],
    pub right_resources: [String; 5],
    pub lessons: [String; 3],
}

impl Default for ListData {
    fn default() -> Self {
        ListData {
            misfortunes: [String::new(), String::new(), String::new(), String::new()],
            misfortunes_red_balls: [String::new(), String::new(), String::new(), String::new()],
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

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ListSection {
    Misfortunes,
    MisfortunesDifficult,
    LxResources,
    RxResources,
    Lessons,
}

impl ListSection {
    pub fn next(&self) -> Self {
        use ListSection::*;
        match *self {
            Misfortunes => MisfortunesDifficult,
            MisfortunesDifficult => LxResources,
            LxResources => RxResources,
            RxResources => Lessons,
            Lessons => Misfortunes,
        }
    }
    pub fn prev(&self) -> Self {
        use ListSection::*;
        match *self {
            Misfortunes => Lessons,
            MisfortunesDifficult => Misfortunes,
            LxResources => MisfortunesDifficult,
            RxResources => LxResources,
            Lessons => RxResources,
        }
    }
    pub fn vertical(&self) -> Self {
        use ListSection::*;
        match *self {
            Misfortunes => MisfortunesDifficult,
            MisfortunesDifficult => Misfortunes,
            _ => *self,
        }
    }

    pub fn idx(&self) -> usize {
        use ListSection::*;
        match *self {
            Misfortunes => 0,
            MisfortunesDifficult => 1,
            LxResources => 2,
            RxResources => 3,
            Lessons => 4,
        }
    }

    pub fn item_length(&self) -> usize {
        use ListSection::*;
        match *self {
            Misfortunes => 50,
            MisfortunesDifficult => 2,
            LxResources | RxResources => 75,
            Lessons => 500,
        }
    }
}

#[derive(Debug, Clone)]
pub struct App {
    pub white_balls: usize,
    pub red_balls: usize,
    pub draw_count: usize,
    pub focused_section: FocusedSection,
    pub popup: PopupType,
    pub drawn_balls: Vec<BallType>,
    // pub first_draw_complete: bool,
    pub pool: Vec<BallType>,
    pub current_tab: TabType,
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
    pub misfortunes_red_balls_area: [Rect; 4],
    pub resources_area: [Rect; 2],
    pub lections_area: [Rect; 3],
    // Honeycomb grid
    pub honeycomb_nodes: Vec<HoneycombNode>,
    pub selected_node: Option<usize>,
    pub editing_node: bool,
    pub node_edit_buffer: String,
    pub graph_area: Rect,        // memorizzo area grafo per rendering
    pub used_traits: Vec<usize>, // vector to store honeycomb idx when decide to enable trait for draw
    // New modes
    pub random_mode: bool,
    pub forced_four_mode: bool,
    // List tab data
    pub list_data: ListData,
    pub list_vertical_scroll: [usize; 3], // each lesson have teir own scrollbar
    pub list_vertical_scroll_state: [ScrollbarState; 3],
    pub selected_list_item: Option<(ListSection, usize)>, // (section Enum, item idx)
    pub editing_list_item: bool,
    pub list_edit_buffer: String,
    pub additional_red_balls: [usize; 4], // vector to store additional difficulties associated to active misfortune
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
            // first_draw_complete: false,
            pool: Vec::new(),
            current_tab: TabType::DrawTab,
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
            misfortunes_red_balls_area: [
                Rect::default(),
                Rect::default(),
                Rect::default(),
                Rect::default(),
            ],
            resources_area: [Rect::default(), Rect::default()],
            lections_area: [Rect::default(), Rect::default(), Rect::default()],
            // Honeycomb grid
            honeycomb_nodes,
            selected_node: Some(9), // central node: archetipo
            editing_node: false,
            node_edit_buffer: String::new(),
            graph_area: Rect::default(), // memorizzo area grafo per rendering
            used_traits: Vec::new(), // vector to store honeycomb idx when decide to enable trait for draw
            // New mode
            random_mode: false,
            forced_four_mode: false,
            // List tab data
            list_data,
            list_vertical_scroll: [0, 0, 0], // each lesson have teir own scrollbar
            list_vertical_scroll_state: [
                ScrollbarState::default(),
                ScrollbarState::default(),
                ScrollbarState::default(),
            ],
            selected_list_item: Some((ListSection::Misfortunes, 0)),
            editing_list_item: false,
            list_edit_buffer: String::new(),
            additional_red_balls: [0, 0, 0, 0], // vector to store additional difficulties associated to active misfortune
        }
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
        let node_width = 14; // 12
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
        //   -2   -1    0    1    2      //    -2    -1     0     1     2
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
        if self.current_tab != TabType::CharacterSheetTab || self.editing_node {
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
        let center_x = (inner_area.x + inner_area.width) / 2;
        let center_y = (inner_area.y + inner_area.height) / 2;

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
            self.honeycomb_nodes[idx].text = self.node_edit_buffer.clone().trim().to_string();
            self.save_data();
        }
        self.editing_node = false;
        self.node_edit_buffer.clear();
    }

    pub fn start_list_editing(&mut self) {
        use ListSection::*;
        if let Some((section, idx)) = self.selected_list_item {
            self.editing_list_item = true;
            self.list_edit_buffer = match section {
                Misfortunes => self.list_data.misfortunes[idx].clone(),
                MisfortunesDifficult => self.list_data.misfortunes_red_balls[idx].clone(),
                LxResources => self.list_data.left_resources[idx].clone(),
                RxResources => self.list_data.right_resources[idx].clone(),
                Lessons => self.list_data.lessons[idx].clone(),
            };
        }
    }

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
                RxResources => {
                    self.list_data.right_resources[idx] =
                        self.list_edit_buffer.clone().trim().to_string()
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

    pub fn reset(&mut self) {
        self.white_balls = 0;
        self.red_balls = 0;
        self.draw_count = 1;
        self.drawn_balls.clear();
        // self.first_draw_complete = false;
        self.pool.clear();
        self.popup = PopupType::None;
        self.current_first_draw.clear();
        self.forced_four_mode = false;
        self.random_mode = false;
        self.focused_section = FocusedSection::WhiteBalls;
        // clear array of used traits
        self.used_traits.clear();
        self.selected_node = Some(9); // set selection over archetype
        self.additional_red_balls = [0, 0, 0, 0];
        self.selected_list_item = Some((ListSection::Misfortunes, 0)); // set selection over element [0,0]
    }

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

    fn add_to_log(&mut self, risk: bool, risk_ball: Vec<BallType>) {
        let local: DateTime<Local> = Local::now();
        // Aggiungi alla cronologia
        self.history.push(DrawHistory {
            time: local.format("%A %e %B %Y, %T").to_string(),
            white_balls: self.white_balls,
            traits: self.used_traits.clone(),
            red_balls: self.red_balls,
            misfortunes: self.additional_red_balls.clone(),
            first_draw: self.current_first_draw.clone(),
            risked: risk,
            risk_draw: risk_ball,
            confused: self.random_mode,
            adrenalined: self.forced_four_mode,
        });

        // after add result to logs clear status modifiers
        self.random_mode = false;
        self.forced_four_mode = false;
        // clear array of used traits
        self.used_traits.clear();
        self.additional_red_balls = [0, 0, 0, 0];
    }

    pub fn perform_first_draw(&mut self) {
        use PopupType::*;
        self.create_pool();
        let drawn = self.draw_from_pool(self.draw_count);
        self.drawn_balls = drawn.clone();
        self.current_first_draw = drawn;
        // self.first_draw_complete = true;

        if self.drawn_balls.len() < 5 {
            self.popup = ConfirmRisk;
        } else {
            self.add_to_log(false, Vec::new());
            self.update_vertical_scroll_state();
            self.popup = None;
        }
    }

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

    pub fn cancel_draw(&mut self) {
        self.add_to_log(false, Vec::new());
        self.update_vertical_scroll_state();
    }

    fn update_vertical_scroll_state(&mut self) {
        // Calculate total content height (approximately 13 lines per entry)
        let content_height = self.history.len() * 13;
        self.vertical_scroll_state = self.vertical_scroll_state.content_length(content_height);
    }

    fn update_list_vertical_scroll_state(&mut self, idx: usize) {
        // use mod (%) operator to ensure that idx stay between 0..2
        let width = self.lections_area[idx % 3].width; // get length of displayed area
        let content_height = self.list_data.lessons[idx % 3].len() / width as usize; // calculate amount of scroll available
        self.list_vertical_scroll_state[idx % 3] =
            self.list_vertical_scroll_state[idx % 3].content_length(content_height);
    }

    pub fn handle_mouse_click(&mut self, x: u16, y: u16) {
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
                    self.random_mode = !self.random_mode;
                } else if is_inside(x, y, &self.forced_four_area) {
                    self.forced_four_mode = !self.forced_four_mode;
                    if self.forced_four_mode {
                        self.draw_count = 4;
                    } else {
                        self.draw_count = 1;
                    }
                }
            }
            TabType::AdditionalInfoTab => {
                use ListSection::*;
                // Tab 2 specific areas
                for idx in 0..4 {
                    // ignore mouse click if I'm in editing mode
                    if !self.editing_list_item {
                        if idx < 2 && is_inside(x, y, &self.resources_area[idx]) {
                            self.selected_list_item = Some((get_section_type(idx + 2), 0));
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

    pub fn increment_balls(&mut self) {
        match self.focused_section {
            FocusedSection::WhiteBalls => {
                // 20 token as hard cap
                if self.white_balls < 20 {
                    self.white_balls += 1;
                }
            }
            FocusedSection::RedBalls => {
                // 20 token as hard cap
                if self.red_balls < 20 {
                    self.red_balls += 1;
                }
            }
            FocusedSection::DrawInput => {
                if self.draw_count < 4 && !self.forced_four_mode {
                    self.draw_count += 1;
                }
            }
            _ => {}
        }
    }

    pub fn decrement_balls(&mut self) {
        match self.focused_section {
            FocusedSection::WhiteBalls => {
                if self.white_balls > 0 {
                    self.white_balls -= 1;
                    // pop first trait if present. don't care which one
                    if !self.used_traits.is_empty() {
                        let _ = self.used_traits.pop();
                    }
                }
            }
            FocusedSection::RedBalls => {
                if self.red_balls > 0 {
                    // first remove normal difficult
                    if self.red_balls > self.additional_red_balls.iter().sum() {
                        self.red_balls -= 1;
                    } else {
                        // remove first (in order) additional difficult from misfortunes
                        for (i,d) in self.additional_red_balls.clone().iter().enumerate() {
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
                if self.draw_count > 1 && !self.forced_four_mode {
                    self.draw_count -= 1;
                }
            }
            _ => {}
        }
    }

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
                LxResources | RxResources => {
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
                RxResources => {
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

    pub fn up_section(&mut self) {
        use ListSection::*;
        if let Some((section, idx)) = self.selected_list_item {
            match section {
                Misfortunes | MisfortunesDifficult => {
                    self.selected_list_item = Some((section.vertical(), idx));
                }
                LxResources | RxResources => {
                    if idx > 0 {
                        self.selected_list_item = Some((section, idx - 1));
                    } else {
                        self.selected_list_item = Some((section, 4 - idx));
                    }
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

    pub fn down_section(&mut self) {
        use ListSection::*;
        if let Some((section, idx)) = self.selected_list_item {
            match section {
                Misfortunes | MisfortunesDifficult => {
                    self.selected_list_item = Some((section.vertical(), idx))
                }
                LxResources | RxResources => {
                    self.selected_list_item = Some((section, (idx + 1) % 5));
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
                _ => {} // default value
            }
        }
    }

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
                _ => {} // default value
            }
        }
    }

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

pub fn get_section_type(idx: usize) -> ListSection {
    use ListSection::*;
    match idx {
        0 => Misfortunes,
        1 => MisfortunesDifficult,
        2 => LxResources,
        3 => RxResources,
        4 => Lessons,
        _ => Misfortunes,
    }
}

pub fn get_tab_type(idx: usize) -> TabType {
    use TabType::*;
    match idx {
        0 => DrawTab,
        1 => CharacterSheetTab,
        2 => AdditionalInfoTab,
        3 => LogTab,
        _ => None,
    }
}

fn is_inside(x: u16, y: u16, area: &Rect) -> bool {
    x >= area.x && x < area.x + area.width && y >= area.y && y < area.y + area.height
}
