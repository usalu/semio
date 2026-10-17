//! 🧬️ Rust window-owner schema surface.
pub use crate::editor::note::window::{NoteCompositeWindowConfig, NoteCompositeWindowTransient};
use semio_framework_value_derive::{FromValue, ToValue};

/// 🧷️ One provisional entry of a persisted ink tool transaction, its mutation in value form so the window transient
/// retires it like any other value.
#[derive(Clone, Debug, PartialEq, ToValue, FromValue)]
#[value(rename_all = "camelCase")]
pub struct NoteInkToolEntry {
    pub key: String,
    pub mutation: semio_framework_value::DslValue,
}

/// 💾️ A composite window's in-flight ink gesture, persisted in its window transient between dispatches — ephemeral
/// local tool state, never history: the statechart configuration by stable ids, the verb, the admission and document
/// revision it opened on, and the open transaction's provisional entries.
#[derive(Clone, Debug, PartialEq, ToValue, FromValue)]
#[value(rename_all = "camelCase")]
pub struct NoteInkToolState {
    pub states: Vec<String>,
    pub verb: String,
    pub authoring_seed: String,
    pub base_revision: String,
    pub transaction: semio_framework_os_kernel::TransactionRef,
    pub entries: Vec<NoteInkToolEntry>,
}

semio_framework_value::artifact_retire_struct!(NoteInkToolEntry { key, mutation });
semio_framework_value::artifact_retire_struct!(NoteInkToolState { states, verb, authoring_seed, base_revision, transaction, entries });
