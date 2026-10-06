//! 📜️ Playbook artifact — textual document grammar surface + laws (constitutional: dsl).

//#region 📖️SemioGrammar
/// 📖️ Normative handcrafted text grammar for this facet (`dialect grammar`).
pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️.grammar.semio");
//#endregion 📖️SemioGrammar

use crate::PlaybookSnapshot;

/// 📄️ The `facade-generator` example spec, handcrafted in the `.playbook` DSL.
pub const FACADE_GENERATOR_EXAMPLE_TEXT: &str = include_str!("../../../🖼️assets/🎬️demo/🗣️.dsl.semio");

/// 📖️ Parses `.playbook` DSL text into a `PlaybookSnapshot`.
pub fn parse_dsl(text: &str) -> Result<PlaybookSnapshot, semio_framework_diagnostic::TextError> {
    <PlaybookSnapshot as store::ArtifactDsl>::parse_dsl(text)
}

/// 🖨️ Prints a `PlaybookSnapshot` back to `.playbook` DSL text.
pub fn print_dsl(document: &PlaybookSnapshot) -> String {
    store::ArtifactDsl::print_dsl(document)
}

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests

//#region 🚚️Carrier
/// 🚚️ The carrier this facet's `parse`/`print` speak, named as the schema names the export.
pub type PlaybookSnapshotText = String;
//#endregion 🚚️Carrier

#[allow(unused_imports)]
mod mutations_codec {
use super::*;
use crate::standards::v1::subsets::any::schema::mutations::*;
use crate::{PlaybookDiff, PlaybookSnapshot};
use semio_framework_value_derive::{FromValue, ToValue};
#[cfg(test)]
use serde::{Deserialize, Serialize};
use crate::standards::v1::subsets::any::schema::mutations::change_title::{change_title_operation, ChangeTitle};

/// 📥️ Decodes a committed `📸️snapshot/{⬅️before,➡️after}/🔣️.json` vector.
// 🚫️async: E1 pure codec helper (file verified I/O-free) — see R9
pub fn decode_playbook_snapshot_json(text: &str) -> Result<PlaybookSnapshot, String> {
    semio_framework_pack_json::from_json_str(text, semio_framework_pack_json::JsonMemberPolicy::Reject).map_err(|error| error.to_string())
}

/// 📤️ The snapshot as the same canonical JSON the committed vectors are written in — the
/// projection an external test host compares through.
// 🚫️async: E1 pure codec helper (file verified I/O-free) — see R9
pub fn encode_playbook_snapshot_json(snapshot: &PlaybookSnapshot) -> String {
    semio_framework_pack_json::to_json_string(snapshot)
}
}
pub use mutations_codec::*;


#[allow(unused_imports)]
mod snapshot_codec {
use super::*;
use crate::standards::v1::subsets::any::schema::snapshot::*;
use framework_schema::ArtifactSchema;

/// 📦️ Derived pack and text record of a `PlaybookSnapshot`: its scalars and the `flow` child's coordinate — no child content.
#[derive(semio_framework_dsl_record_derive::DslRecord)]
#[dsl(extension = "playbook")]
pub(crate) struct PlaybookPackRecord {
    schema: String,
    id: String,
    version: String,
    title: Option<String>,
    flow: crate::PlaybookFlowChild,
}

impl PlaybookPackRecord {
    pub(crate) fn from_snapshot(snapshot: &PlaybookSnapshot) -> Self {
        Self { schema: snapshot.schema.clone(), id: snapshot.id.clone(), version: snapshot.version.clone(), title: snapshot.title.clone(), flow: snapshot.flow.clone() }
    }

    pub(crate) fn into_snapshot(self) -> PlaybookSnapshot {
        PlaybookSnapshot { schema: self.schema, id: self.id, version: self.version, title: self.title, flow: self.flow }
    }
}

impl store::ArtifactDsl for PlaybookSnapshot {
    const EXTENSION: &'static str = "playbook";
    fn envelope_id() -> &'static str {
        "playbook.playbook"
    }
    fn parse_dsl(text: &str) -> Result<Self, semio_framework_diagnostic::TextError> {
        let body = match store::semio_format::split_text_preamble(text) {
            Ok((_, rest)) => rest,
            Err(_) => text,
        };
        let record = semio_framework_dsl_record::parse(body, &PlaybookPackRecord::__dsl_spec(), &semio_framework_dsl_record::ParseOptions { limits: semio_framework_diagnostic::Limits::default(), mode: semio_framework_dsl_record::SourceMode::Document })?;
        Ok(PlaybookPackRecord::__dsl_from_record(&record)?.into_snapshot())
    }
    fn print_dsl(&self) -> String {
        let body = semio_framework_dsl_record::print(&PlaybookPackRecord::from_snapshot(self).__dsl_to_record(), &PlaybookPackRecord::__dsl_spec(), semio_framework_dsl_record::JoinMode::Document);
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(<Self as store::ArtifactDsl>::envelope_id(), store::semio_format::Component::Dsl, 1).expect("valid envelope_id");
        store::semio_format::wrap_text(&envelope, &body)
    }
}

/// 📖️ Parses `.playbook` DSL text with a plain-`String` error, reachable from OUTSIDE this crate —
/// `store` is a private `extern crate` alias (`🦀️.rs`), so `store::TextError` cannot be named
/// by the exhaustive mutation case's test adapter that has to read the committed
/// `🗣️.dsl.semio` artifact.
// 🚫️async: E1 pure codec helper (file verified I/O-free) — see R9
pub fn parse_playbook_dsl(text: &str) -> Result<PlaybookSnapshot, String> {
    <PlaybookSnapshot as store::ArtifactDsl>::parse_dsl(text).map_err(|error| error.to_string())
}

/// 🖨️ Prints a [`PlaybookSnapshot`] back to `.playbook` DSL text under a name an external caller can reach, paired
/// with [`parse_playbook_dsl`].
// 🚫️async: E1 pure codec helper (file verified I/O-free) — see R9
pub fn print_playbook_dsl(snapshot: &PlaybookSnapshot) -> String {
    store::ArtifactDsl::print_dsl(snapshot)
}
}
pub use snapshot_codec::*;
