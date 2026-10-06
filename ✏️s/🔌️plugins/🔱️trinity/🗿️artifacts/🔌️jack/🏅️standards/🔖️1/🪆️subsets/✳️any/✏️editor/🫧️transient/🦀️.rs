//! 🪟️ Concrete Jack window-transient owners.

#[path = "../🎭️modes/✏️edit/🪟️windows/📝️editor/🫧️transient/🦀️.rs"]
mod editor_window;
pub use editor_window::{JackEditorSelection, JackEditorWindowTransient, JackEditorWindowTransientMutation, JackEditorWindowTransientOwner, SetEditorSelection, WINDOW_KIND_ID as JACK_EDITOR_WINDOW_KIND_ID};
pub use crate::editor::jack::transient::component::editor_window::io::text::mutations::{jack_editor_window_transient_mutation_report_json};

#[path = "../🎭️modes/✏️edit/🪟️windows/📊️results/🫧️transient/🦀️.rs"]
mod results_window;
pub use results_window::{addressed as addressed_results_window, register as register_results_window, JackResultsWindowTransient, JackResultsWindowTransientMutation, JackResultsWindowTransientOwner, ReplaceQueryResult};
pub use crate::editor::jack::transient::component::results_window::io::text::mutations::{jack_results_window_transient_mutation_report_json};
