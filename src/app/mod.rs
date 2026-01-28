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
#[allow(unused_imports)]
pub use app_state::{App, MAX_DRAW, MAX_TOKEN, MIN_DRAW};
pub use character::CharacterSection;
pub use list::{ListSection, get_section_type};
pub use types::{BallType, FocusedSection, PopupType, TabType};
