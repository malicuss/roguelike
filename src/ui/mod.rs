pub mod event;
pub mod input;
pub mod render; // бывший ui.rs
pub mod tui; // бывший update.rs

pub use event::{Event, EventHandler};
pub use render::render;
pub use tui::Tui;
