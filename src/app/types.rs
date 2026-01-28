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

#[cfg(test)]
mod types_tests {

    use crate::app::types::*;

    #[test]
    fn test_ball_type_display() {
        assert_eq!(BallType::White.to_string(), "Successo");
        assert_eq!(BallType::Red.to_string(), "Complicazione");
    }

    #[test]
    fn test_ball_type_equality() {
        assert_eq!(BallType::White, BallType::White);
        assert_eq!(BallType::Red, BallType::Red);
        assert_ne!(BallType::White, BallType::Red);
    }

    #[test]
    fn test_popup_type_none() {
        let popup = PopupType::None;
        assert_eq!(popup, PopupType::None);
    }

    #[test]
    fn test_popup_type_variants() {
        assert_ne!(PopupType::ConfirmDraw, PopupType::ConfirmRisk);
        assert_ne!(PopupType::None, PopupType::ConfirmDraw);
    }

    #[test]
    fn test_tab_type_next() {
        assert_eq!(TabType::DrawTab.next(), TabType::CharacterSheetTab);
        assert_eq!(
            TabType::CharacterSheetTab.next(),
            TabType::AdditionalInfoTab
        );
        assert_eq!(TabType::AdditionalInfoTab.next(), TabType::LogTab);
        assert_eq!(TabType::LogTab.next(), TabType::DrawTab);
    }

    #[test]
    fn test_tab_type_next_wraps() {
        let mut tab = TabType::DrawTab;
        for _ in 0..4 {
            tab = tab.next();
        }
        assert_eq!(tab, TabType::DrawTab);
    }

    #[test]
    fn test_tab_type_idx() {
        assert_eq!(TabType::DrawTab.idx(), 0);
        assert_eq!(TabType::CharacterSheetTab.idx(), 1);
        assert_eq!(TabType::AdditionalInfoTab.idx(), 2);
        assert_eq!(TabType::LogTab.idx(), 3);
        assert_eq!(TabType::None.idx(), 0);
    }

    #[test]
    fn test_focused_section_next() {
        assert_eq!(FocusedSection::WhiteBalls.next(), FocusedSection::RedBalls);
        assert_eq!(FocusedSection::RedBalls.next(), FocusedSection::RandomMode);
        assert_eq!(
            FocusedSection::RandomMode.next(),
            FocusedSection::ForcedFour
        );
        assert_eq!(FocusedSection::ForcedFour.next(), FocusedSection::DrawInput);
        assert_eq!(FocusedSection::DrawInput.next(), FocusedSection::WhiteBalls);
    }

    #[test]
    fn test_focused_section_prev() {
        assert_eq!(FocusedSection::WhiteBalls.prev(), FocusedSection::DrawInput);
        assert_eq!(FocusedSection::DrawInput.prev(), FocusedSection::ForcedFour);
        assert_eq!(
            FocusedSection::ForcedFour.prev(),
            FocusedSection::RandomMode
        );
        assert_eq!(FocusedSection::RandomMode.prev(), FocusedSection::RedBalls);
        assert_eq!(FocusedSection::RedBalls.prev(), FocusedSection::WhiteBalls);
    }

    #[test]
    fn test_focused_section_cycle() {
        let mut section = FocusedSection::WhiteBalls;
        for _ in 0..5 {
            section = section.next();
        }
        assert_eq!(section, FocusedSection::WhiteBalls);
    }

    #[test]
    fn test_get_tab_type() {
        assert_eq!(get_tab_type(0), TabType::DrawTab);
        assert_eq!(get_tab_type(1), TabType::CharacterSheetTab);
        assert_eq!(get_tab_type(2), TabType::AdditionalInfoTab);
        assert_eq!(get_tab_type(3), TabType::LogTab);
        assert_eq!(get_tab_type(99), TabType::None);
    }
}
