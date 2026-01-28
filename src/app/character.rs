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

#[cfg(test)]
mod character_tests {
    use crate::app::character::*;

    #[test]
    fn test_character_section_next() {
        assert_eq!(
            CharacterSection::CharacterName.next(),
            CharacterSection::CharacterObjective
        );
        assert_eq!(
            CharacterSection::CharacterObjective.next(),
            CharacterSection::CharacterName
        );
        assert_eq!(CharacterSection::None.next(), CharacterSection::None);
    }

    #[test]
    fn test_character_section_equality() {
        assert_eq!(CharacterSection::None, CharacterSection::None);
        assert_ne!(CharacterSection::CharacterName, CharacterSection::None);
        assert_ne!(
            CharacterSection::CharacterName,
            CharacterSection::CharacterObjective
        );
    }

    #[test]
    fn test_character_base_info_default() {
        let info = CharacterBaseInformation::default();
        assert_eq!(info.name, "");
        assert_eq!(info.objective, "");
    }

    #[test]
    fn test_character_base_info_length() {
        let info = CharacterBaseInformation::default();
        assert_eq!(info.length(), 50);
    }

    #[test]
    fn test_character_base_info_with_data() {
        let info = CharacterBaseInformation {
            name: "Eroe Coraggioso".to_string(),
            objective: "Salvare il regno".to_string(),
        };
        assert_eq!(info.name, "Eroe Coraggioso");
        assert_eq!(info.objective, "Salvare il regno");
    }

    #[test]
    fn test_character_name_max_length() {
        let long_name = "a".repeat(100);
        let info = CharacterBaseInformation {
            name: long_name.clone(),
            objective: "Test".to_string(),
        };
        assert_eq!(info.name.len(), 100);
        assert!(info.name.len() > info.length());
    }

    #[test]
    fn test_character_objective_empty() {
        let info = CharacterBaseInformation {
            name: "Test".to_string(),
            objective: "".to_string(),
        };
        assert!(info.objective.is_empty());
    }

    #[test]
    fn test_character_section_cycle() {
        let mut section = CharacterSection::CharacterName;
        section = section.next();
        section = section.next();
        assert_eq!(section, CharacterSection::CharacterName);
    }
}
