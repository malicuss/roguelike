//! Игровой мир: состояние и правила.
//! Модули отсюда НЕ должны импортировать crate::ui.

pub mod action_timer;
pub mod app;
pub mod helpers;
pub mod man;
pub mod monsters;
pub mod room;

// публичный API слоя — то, чем пользуются снаружи (main, ui)
pub use action_timer::ActionTimer;
pub use app::App;
pub use man::*;
pub use monsters::{Monster, MonsterCtx};
pub use room::Room;
