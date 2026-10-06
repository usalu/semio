//! 📝️ Text representation codec surface for `stdio.dwg` (snapshot).

/// 📖️ Grammar include.
pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️.grammar.semio");

//#region 🚚️Carrier
/// 🚚️ The carrier this facet's `parse`/`print` speak, named as the schema names the export.
pub type DwgSnapshotText = String;
//#endregion 🚚️Carrier

#[cfg(test)]
#[path = "🧪️tests/🏗️grammar/🦀️.rs"]
mod grammar_shape_tests;

#[allow(unused_imports)]
mod snapshot_codec {
use super::*;
use crate::standards::v_ac1024::subsets::any::schema::snapshot::*;
use crate::standards::v_ac1024::engine as dwg_engine;
use crate::STDIO_DWG_DOCUMENT_SCHEMA;
use framework_schema::ArtifactSchema;
use std::fmt;
use semio_framework_value::{ValueError, ValueRefusalKind};

impl store::ArtifactDsl for DwgSnapshot {
    const EXTENSION: &'static str = "dwg";
    fn envelope_id() -> &'static str {
        "stdio.dwg"
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
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(<Self as store::ArtifactDsl>::envelope_id(), store::semio_format::Component::Dsl, 1).expect("valid envelope_id");
        store::semio_format::wrap_text(&envelope, &body)
    }
}
}
pub use snapshot_codec::*;

#[allow(unused_imports)]
mod snapshot_wire_codec {
use super::*;
use crate::standards::v_ac1024::subsets::any::schema::snapshot::*;
use crate::standards::v_ac1024::engine as dwg_engine;
use crate::STDIO_DWG_DOCUMENT_SCHEMA;
use framework_schema::ArtifactSchema;
use std::fmt;
use semio_framework_value::{ValueError, ValueRefusalKind};















/// 🚫️ Why this writer cannot emit `snapshot`'s stamp, as the `x-semio-invariant` id it breaks and a sentence naming the
/// value: `version-sentinel` when the stamp is not `AC` + four digits, `written-as-ac1024` when a drawing that carries
/// content is stamped anything but [`WRITTEN_VERSION`]. AC1018 (R2004) frames objects without a handle-stream size,
/// AC1027/AC1032 (R2013+) add `has_ds_data` to every object, so a foreign stamp over these object streams would be a file
/// no reader decodes as written. Only the empty preamble-only document carries no object stream a stamp could
/// contradict, so it may carry any sentinel.
///
/// @see https://www.opendesign.com/files/guestdownloads/OpenDesign_Specification_for_.dwg_files.pdf — §20 object layouts
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn unwritable_version(snapshot: &DwgSnapshot) -> Option<(&'static str, String)> {
    if snapshot.version.len() != 6 || dwg_version_sentinel(snapshot.version.as_bytes()).is_err() {
        return Some(("version-sentinel", format!("{:?} is not an AC10xx DWG version sentinel", snapshot.version)));
    }
    (snapshot.version != WRITTEN_VERSION && !is_preamble_only_document(snapshot)).then(|| ("written-as-ac1024", format!("a drawing that carries content is written as {WRITTEN_VERSION}; this writer cannot lay out {} object streams", snapshot.version)))
}


}
pub use snapshot_wire_codec::*;
