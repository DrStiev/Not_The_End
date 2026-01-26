// Moduli interni
mod app_impl;
mod app_state;
mod character;
mod history;
mod honeycomb;
mod list;
mod types;

// Re-export dei tipi pubblici
pub use app_state::App;
pub use character::CharacterSection;
pub use list::{ListSection, get_section_type};
pub use types::{BallType, FocusedSection, PopupType, TabType};
