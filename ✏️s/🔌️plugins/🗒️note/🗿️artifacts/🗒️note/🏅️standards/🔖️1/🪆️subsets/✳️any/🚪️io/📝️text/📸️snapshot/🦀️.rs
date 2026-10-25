//! 📜️ Note artifact — textual document grammar surface + laws (constitutional: dsl).

use crate::NoteSnapshot;

//#region 📖️SemioGrammar
/// 📖️ Normative handcrafted text grammar for this facet (`dialect grammar`).
pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️.grammar.semio");
//#endregion 📖️SemioGrammar

/// 📄️ The `semio` example document, handcrafted in the `.note` DSL.
pub const SEMIO_NOTE_EXAMPLE_TEXT: &str = include_str!("../../../🖼️assets/🎬️demo/🗣️.dsl.semio");

/// 📖️ Parses `.note` DSL text into a `NoteSnapshot`.
pub fn parse_dsl(text: &str) -> Result<NoteSnapshot, semio_framework_diagnostic::TextError> {
    <NoteSnapshot as pack::snapshot::ArtifactDsl>::parse_dsl(text)
}

/// 🖨️ Prints a `NoteSnapshot` back to `.note` DSL text.
pub fn print_dsl(document: &NoteSnapshot) -> String {
    pack::snapshot::ArtifactDsl::print_dsl(document)
}

//#region 🧪️Tests
//#endregion 🧪️Tests



//#region 🔖️ExternalBridges
/// 📖️ Parses `.note` DSL text with a plain-`String` error, reachable from OUTSIDE this crate —
/// `store` is a private `extern crate` alias (`🦀️.rs`), so `store::TextError` cannot be named
/// by the exhaustive mutation case's test adapter that has to read the committed
/// `🗣️.dsl.semio` artifact.
// 🚫️async: E1 pure codec helper (file verified I/O-free) — see R9
pub fn parse_note_dsl(text: &str) -> Result<NoteSnapshot, String> {
    <NoteSnapshot as pack::snapshot::ArtifactDsl>::parse_dsl(text).map_err(|error| error.to_string())
}

/// 🖨️ Prints a [`NoteSnapshot`] back to `.note` DSL text under a name an external caller can reach, paired
/// with [`parse_note_dsl`].
// 🚫️async: E1 pure codec helper (file verified I/O-free) — see R9
pub fn print_note_dsl(snapshot: &NoteSnapshot) -> String {
    pack::snapshot::ArtifactDsl::print_dsl(snapshot)
}
//#endregion 🔖️ExternalBridges

#[allow(unused_imports)]
mod mutations_codec {
use super::*;
use crate::NoteSnapshot;
use protocol::{Mutation, MutationDiff};
use semio_framework_value_derive::{FromValue, ToValue};

/// 📥️ Decodes a committed `📸️snapshot/{⬅️before,➡️after}/🔣️.json` vector.
// 🚫️async: E1 pure codec helper (file verified I/O-free) — see R9
pub fn decode_note_snapshot_json(text: &str) -> Result<NoteSnapshot, String> {
    semio_framework_pack_json::from_json_str(text, semio_framework_pack_json::JsonMemberPolicy::Reject).map_err(|error| error.to_string())
}

/// 📤️ The snapshot as the same canonical JSON the committed vectors are written in — the
/// projection an external test host compares through.
// 🚫️async: E1 pure codec helper (file verified I/O-free) — see R9
pub fn encode_note_snapshot_json(snapshot: &NoteSnapshot) -> String {
    semio_framework_pack_json::to_json_string(snapshot)
}
}
pub use mutations_codec::*;


#[allow(unused_imports)]
mod snapshot_codec {
use super::*;
use crate::standards::v1::subsets::any::schema::snapshot::*;
use crate::{NoteBlockNode, NoteImageAsset, NOTE_DOCUMENT_SCHEMA};
use framework_schema::ArtifactSchema;
use semio_framework_value_derive::{FromValue, ToValue};
use std::collections::BTreeMap;

/// ✉️ P6 handcrafted ArtifactDsl/ArtifactPack (derive no longer emits these traits).
impl pack::snapshot::ArtifactDsl for NoteSnapshot {
    const EXTENSION: &'static str = "note";
    fn envelope_id() -> &'static str {
        "note.note"
    }
    fn parse_dsl(text: &str) -> Result<Self, semio_framework_diagnostic::TextError> {
        let body = match store::semio_format::split_text_preamble(text) {
            Ok((_, rest)) => rest,
            Err(_) => text,
        };
        let record = semio_framework_dsl_record::parse(body, &Self::__dsl_spec(), &semio_framework_dsl_record::ParseOptions { limits: semio_framework_diagnostic::Limits::default(), mode: semio_framework_dsl_record::SourceMode::Document })?;
        Self::__dsl_from_record(&record)
    }
    fn print_dsl(&self) -> String {
        let body = semio_framework_dsl_record::print(&self.__dsl_to_record(), &Self::__dsl_spec(), semio_framework_dsl_record::JoinMode::Document);
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(<Self as pack::snapshot::ArtifactDsl>::envelope_id(), store::semio_format::Component::Dsl, 1).expect("valid envelope_id");
        store::semio_format::wrap_text(&envelope, &body)
    }
}
}
#[cfg(test)] pub use snapshot_codec::*;

#[allow(unused_imports)]
mod snapshot_wire_codec {
use super::*;
use crate::standards::v1::subsets::any::schema::*;
use crate::{NoteTableCell, NoteTextParagraph, NoteTextRun, NOTE_DOCUMENT_SCHEMA};
use framework_schema::ArtifactSchema;
use semio_framework_value_derive::{FromValue, ToValue};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::BTreeMap;
/// 🔁️ Entities this module's schema exports and its crate declares elsewhere.
use crate::NoteBlockNode;
use crate::NoteImageAsset;

/// 📄️ The `semio` example, parsed once from {@link SEMIO_NOTE_EXAMPLE_TEXT} — the source of truth for
/// every "semio" example call site (`setActiveExample`, tests). Falls back to the empty document if the
/// fixture ever fails to parse, matching the old JSON fixture's failure behavior.
pub fn semio_example_snapshot() -> NoteSnapshot {
    <NoteSnapshot as pack::snapshot::ArtifactDsl>::parse_dsl(SEMIO_NOTE_EXAMPLE_TEXT).unwrap_or_else(|_| empty_note_snapshot())
}

/// 📄️ JSON re-serialization of {@link semio_example_snapshot}, for the framework-generic call sites that
/// contractually require JSON text (`PluginApp::render`'s `projection_override_json`, `App::example`'s
/// manifest `document_json`).
pub fn semio_example_json() -> String {
    semio_framework_pack_json::to_json_string(&semio_example_snapshot())
}

pub fn empty_note_snapshot() -> NoteSnapshot {
    NoteSnapshot {
        schema: NOTE_DOCUMENT_SCHEMA.into(),
        id: "empty".into(),
        title: None,
        blocks: Vec::new(),
        grid_visible: Some(true),
        grid_spacing: Some(32.0),
        grid_subdivisions: Some(4.0),
        grid_opacity: Some(0.35),
        snap_enabled: Some(false),
        snap_grid_spacing: Some(8.0),
        pencil_width: Some(3.0),
        eraser_radius: Some(12.0),
        assets: BTreeMap::new(),
        linked_artifact: None,
    }
}
}
pub use snapshot_wire_codec::*;
