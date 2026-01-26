// Moduli interni
mod app_state;
mod character;
mod history;
mod honeycomb;
mod list;
mod types;

// Modulo implementazioni (suddiviso in sottomoduli)
#[path = "impl/mod.rs"]
mod app_impl;

// Re-export dei tipi pubblici
pub use app_state::App;
pub use character::CharacterSection;
pub use list::{ListSection, get_section_type};
pub use types::{BallType, FocusedSection, PopupType, TabType};
