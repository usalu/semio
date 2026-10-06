//! 🔌️ Jack native document preserves inline manifests and literal composed Graph handles.

//#region 📖️SemioGrammar
/// 📖️ Normative handcrafted text grammar for this facet (`dialect grammar`).
pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️.grammar.semio");
//#endregion 📖️SemioGrammar

use crate::JackSnapshot;
use semio_framework_diagnostic::TextError;
use semio_framework_diagnostic::TextSpan;
use store::ArtifactDsl;
use store::PackDecodeOptions;
use store::PackEncodeOptions;
use store::PackError;

#[path = "🧬️records/🦀️.rs"]
mod records;
pub(crate) use records::JackPackRecord;

/// 🖨️ The derived text body: the same `JackPackRecord` the pack encodes, printed by the spec-driven engine.
pub(crate) fn print_pack_record_text(snapshot: &JackSnapshot) -> String {
    semio_framework_dsl_record::print(&JackPackRecord::from_snapshot(snapshot).__dsl_to_record(), &JackPackRecord::__dsl_spec(), semio_framework_dsl_record::JoinMode::Document)
}

/// 📖️ Parses a derived text body back through `JackPackRecord`, with the same decode steps as the pack.
pub(crate) fn parse_pack_record_text(body: &str) -> Result<JackSnapshot, semio_framework_diagnostic::TextError> {
    let record = semio_framework_dsl_record::parse(body, &JackPackRecord::__dsl_spec(), &semio_framework_dsl_record::ParseOptions { limits: semio_framework_diagnostic::Limits::default(), mode: semio_framework_dsl_record::SourceMode::Document })?;
    let mut snapshot = JackPackRecord::__dsl_from_record(&record)?.into_snapshot().map_err(|error| semio_framework_diagnostic::TextError::from_value_error(error, semio_framework_diagnostic::TextSpan::at(1, 1)))?;
    crate::attach_bundled_content(&mut snapshot);
    Ok(snapshot)
}
//#endregion 🔖️PackRecord

//#region 🔖️HandcraftedArtifactCodecs
impl ArtifactDsl for JackSnapshot {
    const EXTENSION: &'static str = "trinity";
    fn envelope_id() -> &'static str {
        "trinity.jack"
    }

    fn parse_dsl(text: &str) -> Result<Self, TextError> {
        let body = match store::semio_format::split_text_preamble(text) {
            Ok((_, rest)) => rest,
            Err(_) => text,
        };
        parse_pack_record_text(body)
    }

    fn print_dsl(&self) -> String {
        let body = print_pack_record_text(self);
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(<Self as ArtifactDsl>::envelope_id(), store::semio_format::Component::Dsl, 1).expect("valid envelope_id");
        store::semio_format::wrap_text(&envelope, &body)
    }
}

impl store::ArtifactPack for JackSnapshot {
    fn encode_pack_with(&self, options: &PackEncodeOptions) -> Result<Vec<u8>, PackError> {
        let inner = store::pack_rt::encode_document(&JackPackRecord::__dsl_spec(), &JackPackRecord::from_snapshot(self).__dsl_to_record(), options)?;
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(<Self as ArtifactDsl>::envelope_id(), store::semio_format::Component::Pack, 1).map_err(|e| PackError::from(e.into_value_error()))?;
        Ok(store::semio_format::wrap_binary(&envelope, &inner))
    }

    fn decode_pack_with(bytes: &[u8], options: &PackDecodeOptions) -> Result<Self, PackError> {
        let (envelope, inner) = store::semio_format::unwrap_binary(bytes).map_err(|e| PackError::from(e.into_value_error()))?;
        if !envelope.matches_identity(<Self as ArtifactDsl>::envelope_id(), store::semio_format::Component::Pack, 1) {
            return Err(PackError::from(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue,format!("pack envelope mismatch: expected {}.pack v1, got {}", <Self as ArtifactDsl>::envelope_id(), envelope.binary_token()))));
        }
        let (record, _report) = store::pack_rt::decode_document(&inner, &JackPackRecord::__dsl_spec(), options)?;
        let mut snapshot = JackPackRecord::__dsl_from_record(&record).map_err(store::text_error_to_pack_error)?.into_snapshot().map_err(PackError::from)?;
        crate::attach_bundled_content(&mut snapshot);
        Ok(snapshot)
    }

    fn sqlite_snapshot_codec() -> Option<store::ArtifactSqliteSnapshotCodec> {
        Some(<Self as store::ArtifactSqliteSnapshot>::sqlite_codec())
    }

    fn record_spec() -> Option<semio_framework_dsl_record::RecordSpec> {
        Some(JackPackRecord::__dsl_spec())
    }
}
//#endregion 🔖️HandcraftedArtifactCodecs

/// 📄️ The Nakagin Capsule Tower example fixture, handcrafted in the `.trinity` DSL.
pub const NAKAGIN_EXAMPLE_TEXT: &str = include_str!("../../../🖼️assets/🎬️demo/🗣️.dsl.semio");

/// 📖️ Parses `.trinity` DSL text into a `JackSnapshot`.
pub fn parse_dsl(text: &str) -> Result<JackSnapshot, TextError> {
    <JackSnapshot as ArtifactDsl>::parse_dsl(text)
}

/// 🖨️ Prints a `JackSnapshot` back to `.trinity` DSL text.
pub fn print_dsl(document: &JackSnapshot) -> String {
    ArtifactDsl::print_dsl(document)
}

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests

//#region 🚚️Carrier
/// 🚚️ The carrier this facet's `parse`/`print` speak, named as the schema names the export.
pub type JackSnapshotText = String;
//#endregion 🚚️Carrier

#[allow(unused_imports)]
mod snapshot_codec {
use super::*;
use crate::standards::v1::subsets::any::schema::snapshot::*;
use crate::{Camera, JackContentChild, Manifest};
use ::semio_framework_schema::ArtifactSchema;

/// 📤️ Renders a [`JackSnapshot`] as this facet's own camelCase JSON projection — the comparison
/// surface `🔌️mutate-jack-1`'s scenarios are measured through, and the shape the committed
/// `../🧫️fixtures/🧬️mutations/<slug>/<fixture>/📸️snapshot/{⬅️before,➡️after}/🔣️.json`
/// specification vectors are written in. It carries `content` as a HANDLE, never as a scene, and
/// that handle's `childId` is a digest of the child — so it moves if and only if the working scene
/// moved, which is what makes it a usable observability surface here.
///
/// A thin `pack::json` wrapper over [`JackSnapshot`]'s own `ToValue`, bridged through
/// `pack::json_from_dsl_value` since `DslValue` and `pack::json::Value` are sibling trees (used
/// behind this interface per CLAUDE.md's "external libraries behind an interface" rule).
pub fn encode_jack_snapshot_json(snapshot: &JackSnapshot) -> Result<String, semio_framework_value::ValueError> {
    let value = crate::standards::v1::subsets::any::io::json_native::convert(semio_framework_value::ToValue::to_value(snapshot), false)?;
    Ok(semio_framework_pack_json::to_string(&semio_framework_pack_json::from_dsl_value(&value)))
}

/// 📥️ The inverse of [`encode_jack_snapshot_json`] — decodes those committed specification vectors
/// into real [`JackSnapshot`] values, so `🔌️mutate-jack-1`'s adapter reads the committed fixture
/// rather than re-declaring it as a Rust literal beside it.
pub fn decode_jack_snapshot_json(text: &str) -> Result<JackSnapshot, semio_framework_value::ValueError> {
    let parsed = semio_framework_pack_json::parse(text, semio_framework_pack_json::JsonMemberPolicy::Reject).map_err(|error| semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, error.to_string()))?;
    <JackSnapshot as semio_framework_value::FromValue>::from_value(crate::standards::v1::subsets::any::io::json_native::convert(semio_framework_pack_json::to_dsl_value(&parsed), true)?)
}

/// 📝️ Parses the literal Jack parent and its independent content-child address.
/// Child materialization belongs to the host's composed artifact boundary.
pub fn parse_jack_dsl(text: &str) -> Result<JackSnapshot, String> {
    <JackSnapshot as store::ArtifactDsl>::parse_dsl(text).map_err(|error| format!("{error:?}"))
}

/// 📝️ Renders the literal parent record with its native document preamble.
pub fn print_jack_dsl(snapshot: &JackSnapshot) -> String {
    store::ArtifactDsl::print_dsl(snapshot)
}
}
pub use snapshot_codec::*;

#[allow(unused_imports)]
/// ▶️ Parses and executes a text query, publishing its semantic graph effects.
pub fn run(graph: &mut crate::Graph, source: &str) -> Result<crate::ast::QueryResult, String> {
    let query = crate::language_service::parse(source)?;
    let (result, effects) = crate::executor::execute(graph, &query)?;
    crate::apply_graph_effects(graph, &effects).map_err(|error| error.to_string())?;
    Ok(result)
}

/// 📤️ Encodes a typed query result under caller-owned physical progress and byte admission.
pub fn encode_query_result_json(result: &crate::ast::QueryResult, control: &mut semio_framework_value::NativeEncodeControl<'_>) -> Result<String, semio_framework_value::ValueError> {
    semio_framework_pack_json::to_json_string_controlled(result, control)
}

/// ▶️ Runs a text query and emits its response under physical JSON byte admission.
pub fn run_json_controlled(graph: &mut crate::Graph, source: &str, control: &mut semio_framework_value::NativeEncodeControl<'_>) -> Result<String, String> {
    let result = run(graph, source)?;
    encode_query_result_json(&result, control).map_err(|error| error.to_string())
}

/// ▶️ Executes a text query with the default response byte grant.
pub fn run_json(graph: &mut crate::Graph, source: &str) -> Result<String, String> {
    let mut progress = |_| true;
    let mut control = semio_framework_value::NativeEncodeControl::new(1_048_576, &mut progress);
    run_json_controlled(graph, source, &mut control)
}


#[allow(unused_imports)]
/// 📤️ Encodes the typed language-service example at its physical text boundary.
pub fn example_graph_snapshot_json() -> String {
    semio_framework_pack_json::to_json_string(&crate::language_service::example_graph_snapshot())
}

