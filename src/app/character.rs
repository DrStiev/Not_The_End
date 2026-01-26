use serde::{Deserialize, Serialize};
use std::fs;

const DATA_FILE: &str = "character_sheet.toml";

/// Sezione del personaggio in fase di modifica
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum CharacterSection {
    None,
    CharacterName,
    CharacterObjective,
}

impl CharacterSection {
    pub fn next(&self) -> Self {
        use CharacterSection::*;
        match *self {
            CharacterName => CharacterObjective,
            CharacterObjective => CharacterName,
            None => None,
        }
    }
}

/// Informazioni base del personaggio
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CharacterBaseInformation {
    pub name: String,
    pub objective: String,
}

impl CharacterBaseInformation {
    pub fn length(&self) -> usize {
        50
    }

    pub(crate) fn load_character_base_info() -> Self {
        if let Ok(contents) = fs::read_to_string(DATA_FILE)
            && let Ok(data) = toml::from_str::<CharacterBaseInformation>(&contents)
        {
            return data;
        }
        CharacterBaseInformation::default()
    }
}
