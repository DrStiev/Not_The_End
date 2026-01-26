use serde::{Deserialize, Serialize};
use std::fs;

const DATA_FILE: &str = "character_sheet.toml";

/// Sezione della lista attualmente selezionata
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ListSection {
    Misfortunes,
    MisfortunesDifficult,
    LxResources,
    Notes,
    Lessons,
}

impl ListSection {
    pub fn next(&self) -> Self {
        use ListSection::*;
        match *self {
            Misfortunes => MisfortunesDifficult,
            MisfortunesDifficult => LxResources,
            LxResources => Notes,
            Notes => Lessons,
            Lessons => Misfortunes,
        }
    }

    pub fn prev(&self) -> Self {
        use ListSection::*;
        match *self {
            Misfortunes => Lessons,
            MisfortunesDifficult => Misfortunes,
            LxResources => MisfortunesDifficult,
            Notes => LxResources,
            Lessons => Notes,
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

    #[allow(dead_code)]
    pub fn idx(&self) -> usize {
        use ListSection::*;
        match *self {
            Misfortunes => 0,
            MisfortunesDifficult => 1,
            LxResources => 2,
            Notes => 3,
            Lessons => 4,
        }
    }

    pub fn length(&self) -> usize {
        use ListSection::*;
        match *self {
            Misfortunes => 50,
            MisfortunesDifficult => 2,
            LxResources => 75,
            Notes => 1000,
            Lessons => 500,
        }
    }
}

/// Dati delle liste (sfortune, risorse, note, lezioni)
#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct ListData {
    pub misfortunes: [String; 4],
    pub misfortunes_red_balls: [String; 4],
    pub left_resources: [String; 10],
    pub notes: String,
    pub lessons: [String; 3],
}

impl ListData {
    pub(crate) fn load_list_data() -> Self {
        if let Ok(contents) = fs::read_to_string(DATA_FILE)
            && let Ok(data) = toml::from_str::<ListData>(&contents)
        {
            return data;
        }
        ListData::default()
    }
}

/// Funzione di utilità per la conversione di indici
pub fn get_section_type(idx: usize) -> ListSection {
    use ListSection::*;
    match idx {
        0 => Misfortunes,
        1 => MisfortunesDifficult,
        2 => LxResources,
        3 => Notes,
        4 => Lessons,
        _ => Misfortunes,
    }
}
