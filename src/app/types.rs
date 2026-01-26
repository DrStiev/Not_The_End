use std::fmt;

/// Tipo di pallina estratta dal sacchetto
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

/// Tipo di popup visualizzato
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum PopupType {
    None,
    ConfirmDraw,
    ConfirmRisk,
}

/// Tab attivo nell'interfaccia
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

/// Sezione attualmente focalizzata nel tab di estrazione
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

/// Funzioni di utilità per la conversione di indici
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
