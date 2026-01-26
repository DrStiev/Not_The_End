// Moduli interni
mod types;
mod character;
mod honeycomb;
mod list;
mod history;
mod app_state;

// Modulo implementazioni (suddiviso in sottomoduli)
#[path = "impl/mod.rs"]
mod app_impl;

// Re-export dei tipi pubblici
pub use types::{BallType, PopupType, TabType, FocusedSection, get_tab_type};
pub use character::{CharacterSection, CharacterBaseInformation};
pub use honeycomb::HoneycombNode;
pub use list::{ListSection, ListData, get_section_type};
pub use history::DrawHistory;
pub use app_state::{App, MAX_TOKEN, MAX_DRAW, MIN_DRAW};