use serde::{Deserialize, Serialize};
use std::fs;

const DATA_FILE: &str = "character_sheet.toml";

/// Nodo della griglia esagonale (tratti del personaggio)
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

impl HoneycombNode {
    pub fn create_honeycomb_layout_with_data(texts: Vec<String>) -> Vec<Self> {
        let mut nodes = Vec::new();
        let node_width = 14;
        let node_height = 6;
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

    fn create_honeycomb_layout() -> Vec<Self> {
        let texts = vec![String::new(); 19];
        Self::create_honeycomb_layout_with_data(texts)
    }

    pub(crate) fn load_honeycomb_data() -> Vec<Self> {
        if let Ok(contents) = fs::read_to_string(DATA_FILE)
            && let Ok(data) = toml::from_str::<HoneycombData>(&contents)
        {
            return Self::create_honeycomb_layout_with_data(data.nodes);
        }
        Self::create_honeycomb_layout()
    }
}

/// Struttura per serializzazione/deserializzazione dei nodi
#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct HoneycombData {
    pub nodes: Vec<String>,
}
