//! ??? Handcrafted retained-mode terminal UI: semio-styled scene, cell renderer, and ANSI backend.

#[cfg(all(feature = "tui-terminal", windows))]
#[path = "🪟️windows/🦀️.rs"]
mod windows_abi;

#[path = "📐️geometry/🦀️.rs"]
pub mod geometry;

#[path = "🎨️theme/🦀️.rs"]
pub mod theme;

#[path = "📝️text/🦀️.rs"]
pub mod text;

#[path = "🔲️cell/🦀️.rs"]
pub mod cell;

#[path = "🔡️ansi/🦀️.rs"]
pub mod ansi;

#[path = "📟️vt/🦀️.rs"]
pub mod vt;

#[path = "📡️event/🦀️.rs"]
pub mod event;

#[path = "🎬️scene/🦀️.rs"]
pub mod scene;

#[path = "📏️layout/🦀️.rs"]
pub mod layout;

#[path = "🪀️widget/🦀️.rs"]
pub mod widget;

#[path = "🖥️chrome/🦀️.rs"]
pub mod chrome;

#[path = "⚙️engine/🦀️.rs"]
pub mod engine;

#[path = "🔌️backend/🦀️.rs"]
pub mod backend;

/// ?? Pseudo-terminal child process spawn and byte I/O for the native TUI host.
#[cfg(feature = "tui-terminal")]
#[path = "🚇️pty/🦀️.rs"]
pub mod pty;

#[path = "🏃️host/🦀️.rs"]
pub mod host;

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
