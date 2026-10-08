//! 🫧️ Jack editor-window transient mutation aggregate.

use super::{JackEditorWindowTransient, JackEditorWindowTransientDiff};

#[path = "🔤️set-editor-selection/🦀️.rs"]
mod set_editor_selection;
pub use set_editor_selection::SetEditorSelection;

#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslEnum, dsl::Mutations)]
#[mutations(snapshot = JackEditorWindowTransient, diff = JackEditorWindowTransientDiff, schema = "trinity.jackeditorwindowtransient")]
pub enum JackEditorWindowTransientMutation {
    #[dsl(key = "set-editor-selection")]
    SetEditorSelection(SetEditorSelection),
}





//#region 🌉️TestBridge

//#endregion 🌉️TestBridge
