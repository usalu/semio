//! 🏛️ The embedded terminal: parser, screen with absolute-row history and reflow, input encoders and the pane widget state.

#[path = "🎨️palette/🦀️.rs"]
pub mod palette;

#[path = "🎹️encode/🦀️.rs"]
pub mod encode;

#[path = "📜️history/🦀️.rs"]
pub mod history;

#[path = "🧬️parser/🦀️.rs"]
mod parser;

#[path = "🧱️screen/🦀️.rs"]
mod screen;

#[path = "🖥️pane/🦀️.rs"]
pub mod pane;

pub use palette::{color_256, Palette};
pub use parser::VtParser;
pub use screen::{CellPoint, Match, Modes, VtScreen};

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
