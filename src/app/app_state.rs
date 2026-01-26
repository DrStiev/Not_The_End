use ratatui::prelude::Rect;
use ratatui::widgets::ScrollbarState;

use super::character::{CharacterBaseInformation, CharacterSection};
use super::history::DrawHistory;
use super::honeycomb::HoneycombNode;
use super::list::{ListData, ListSection};
use super::types::{BallType, FocusedSection, PopupType, TabType};

pub const MAX_TOKEN: usize = 20;
pub const MAX_DRAW: usize = 4;
pub const MIN_DRAW: usize = 1;

/// Stato principale dell'applicazione
#[derive(Debug, Clone)]
pub struct App {
    // Draw state
    pub white_balls: usize,
    pub red_balls: usize,
    pub draw_count: usize,
    pub focused_section: FocusedSection,
    pub popup: PopupType,
    pub drawn_balls: Vec<BallType>,
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

    // Character data
    pub character_base_info: CharacterBaseInformation,
    pub editing_character_info: bool,
    pub character_edit_buffer: String,
    pub selected_character_info: CharacterSection,
    pub character_name_area: Rect,
    pub character_objective_area: Rect,

    // Honeycomb grid
    pub honeycomb_nodes: Vec<HoneycombNode>,
    pub selected_node: Option<usize>,
    pub editing_node: bool,
    pub node_edit_buffer: String,
    pub graph_area: Rect,
    pub used_traits: Vec<usize>,

    // New modes
    pub random_mode: bool,
    pub forced_four_mode: bool,

    // List tab data
    pub list_data: ListData,
    pub notes_vertical_scroll: usize,
    pub notes_vertical_scroll_state: ScrollbarState,
    pub list_vertical_scroll: [usize; 3],
    pub list_vertical_scroll_state: [ScrollbarState; 3],
    pub selected_list_item: Option<(ListSection, usize)>,
    pub editing_list_item: bool,
    pub list_edit_buffer: String,
    pub additional_red_balls: [usize; 4],
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
            // Character data
            character_base_info: CharacterBaseInformation::load_character_base_info(),
            editing_character_info: false,
            character_edit_buffer: String::new(),
            selected_character_info: CharacterSection::None,
            character_name_area: Rect::default(),
            character_objective_area: Rect::default(),
            // Honeycomb grid
            honeycomb_nodes: HoneycombNode::load_honeycomb_data(),
            selected_node: Some(9), // central node: archetipo
            editing_node: false,
            node_edit_buffer: String::new(),
            graph_area: Rect::default(),
            used_traits: Vec::new(),
            // New mode
            random_mode: false,
            forced_four_mode: false,
            // List tab data
            list_data: ListData::load_list_data(),
            notes_vertical_scroll: 0,
            notes_vertical_scroll_state: ScrollbarState::default(),
            list_vertical_scroll: [0, 0, 0],
            list_vertical_scroll_state: [
                ScrollbarState::default(),
                ScrollbarState::default(),
                ScrollbarState::default(),
            ],
            selected_list_item: Some((ListSection::Misfortunes, 0)),
            editing_list_item: false,
            list_edit_buffer: String::new(),
            additional_red_balls: [0, 0, 0, 0],
        }
    }
}
