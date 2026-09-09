//! 🪟️ Concrete Jack window-transient owners.

#[path = "../🎭️modes/✏️edit/🪟️windows/📝️editor/🫧️transient/🦀️.rs"]
mod editor_window;
pub use editor_window::{JackEditorSelection, JackEditorWindowTransient, JackEditorWindowTransientMutation, JackEditorWindowTransientOwner, SetEditorSelection, WINDOW_KIND_ID as JACK_EDITOR_WINDOW_KIND_ID};

#[path = "../🎭️modes/✏️edit/🪟️windows/📊️results/🫧️transient/🦀️.rs"]
mod results_window;
pub use results_window::{addressed as addressed_results_window, register as register_results_window, JackResultsWindowTransient, JackResultsWindowTransientMutation, JackResultsWindowTransientOwner, ReplaceQueryResult};
