//! 🧬️ Rewriting snapshot schema — artifact-lane fields only.

use crate::LayoutPoint;
use ::semio_framework_schema::ArtifactSchema;
use std::collections::BTreeMap;

//#region 🔖️Snapshot
/// 📸️ Persisted rewrite-rule document snapshot (persistent fields of the artifact).
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord, ArtifactSchema)]
#[value(rename_all = "camelCase")]
#[dsl(extension = "rewriting", layout = "lines")]
#[artifact_schema(id = "s.trinity.rewriting")]
pub struct RewritingSnapshot {
    #[state(artifact)]
    #[child(kind="s.stdio.semio")]
    pub working_graph: semio_s_artifact_trinity_jack::JackSnapshot,
    #[state(artifact)]
    pub lhs: crate::standards::v1::subsets::any::schema::Lhs,
    #[state(artifact)]
    pub rhs: crate::standards::v1::subsets::any::schema::Rhs,
    #[state(artifact)]
    #[value(default)]
    pub parameter_bindings: semio_framework_graph::manifest::PropertyBag,
    #[state(artifact)]
    #[value(default)]
    pub rule_layout: crate::standards::v1::subsets::any::schema::RuleLayout,
}
//#region 🔖️HandcraftedArtifactCodecs
/// ✉️ P6 handcrafted ArtifactDsl/ArtifactPack (derive no longer emits these traits).
impl store::ArtifactDsl for RewritingSnapshot {
    const EXTENSION: &'static str = "rewriting";
    fn envelope_id() -> &'static str {
        "trinity.rewriting"
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

impl store::ArtifactPack for RewritingSnapshot {
    fn encode_pack_with(&self, options: &store::PackEncodeOptions) -> Result<Vec<u8>, store::PackError> {
        let inner = store::pack_rt::encode_document(&Self::__dsl_spec(), &self.__dsl_to_record(), options)?;
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(<Self as store::ArtifactDsl>::envelope_id(), store::semio_format::Component::Pack, 1).map_err(|e| store::PackError::from(e.into_value_error()))?;
        Ok(store::semio_format::wrap_binary(&envelope, &inner))
    }
    fn decode_pack_with(bytes: &[u8], options: &store::PackDecodeOptions) -> Result<Self, store::PackError> {
        let (envelope, inner) = store::semio_format::unwrap_binary(bytes).map_err(|e| store::PackError::from(e.into_value_error()))?;
        if !envelope.matches_identity(<Self as store::ArtifactDsl>::envelope_id(), store::semio_format::Component::Pack, 1) {
            return Err(store::PackError::from(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue,format!("pack envelope mismatch: expected {}.pack v1, got {}", <Self as store::ArtifactDsl>::envelope_id(), envelope.binary_token()))));
        }
        let (record, _report) = store::pack_rt::decode_document(&inner, &Self::__dsl_spec(), options)?;
        Self::__dsl_from_record(&record).map_err(store::text_error_to_pack_error)
    }
    fn record_spec() -> Option<semio_framework_dsl_record::RecordSpec> {
        Some(Self::__dsl_spec())
    }
    fn sqlite_snapshot_codec() -> Option<store::ArtifactSqliteSnapshotCodec> {
        Some(<Self as store::ArtifactSqliteSnapshot>::sqlite_codec())
    }
}
//#endregion 🔖️HandcraftedArtifactCodecs
//#endregion 🔖️Snapshot

//#region 🔣️DeclaredJsonCodec
/// 📤️ Renders the declared typed rule, Jack child handle, exact words and keyed maps as JSON.
pub fn encode_rewriting_snapshot_json(snapshot: &RewritingSnapshot) -> Result<String, semio_framework_value::ValueError> {
    let value = json::convert(semio_framework_value::ToValue::to_value(snapshot), false)?;
    Ok(semio_framework_pack_json::to_string(&semio_framework_pack_json::from_dsl_value(&value)))
}

/// 📥️ The inverse of [`encode_rewriting_snapshot_json`] — decodes those committed specification
/// vectors into real [`RewritingSnapshot`] values, so `mutate-rewriting-1`'s adapter reads the committed
/// fixture rather than re-declaring it as a Rust literal beside it. Reaching a JSON library from that
/// adapter is impossible: the generated test host links only this crate and `semio-repo-test-host`.
pub fn decode_rewriting_snapshot_json(text: &str) -> Result<RewritingSnapshot, semio_framework_value::ValueError> {
    let value = semio_framework_pack_json::parse(text, semio_framework_pack_json::JsonMemberPolicy::Reject).map_err(|error| semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, error.to_string()))?;
    let value = json::convert(semio_framework_pack_json::to_dsl_value(&value), true)?;
    <RewritingSnapshot as semio_framework_value::FromValue>::from_value(value)
}

/// 📝️ Parses `.rewriting.dsl.semio` text into a [`RewritingSnapshot`] — a named, non-async pass-through
/// of this type's own handcrafted `store::ArtifactDsl` impl above, whose trait and error type are
/// both unnameable outside this crate, so `mutate-rewriting-1`'s `identity-round-trip` scenario
/// reaches the real committed artifact
/// (`../../🖼️assets/🎬️demo/🗣️.dsl.semio`) through this instead.
pub fn parse_rewriting_dsl(text: &str) -> Result<RewritingSnapshot, String> {
    <RewritingSnapshot as store::ArtifactDsl>::parse_dsl(text).map_err(|error| format!("{error:?}"))
}

/// 📝️ Renders the declared typed Rewriting document through its owned DSL facet.
pub fn print_rewriting_dsl(snapshot: &RewritingSnapshot) -> String {
    store::ArtifactDsl::print_dsl(snapshot)
}

/// 🔎️ Lists typed rule cardinalities, retained child identity and literal sorted map keys.
pub fn rewrite_rule_summary(snapshot: &RewritingSnapshot) -> String {
    format!("bindings[{}] layout[{}] graph={} lhs={}:{} rhs={}/{}/{}/{}/{}",snapshot.parameter_bindings.keys().cloned().collect::<Vec<_>>().join(" "),snapshot.rule_layout.keys().cloned().collect::<Vec<_>>().join(" "),snapshot.working_graph.content.child_id,snapshot.lhs.pattern.left_var,snapshot.lhs.pattern.left_kind,snapshot.rhs.create.len(),snapshot.rhs.delete.len(),snapshot.rhs.set.len(),snapshot.rhs.merge.len(),snapshot.rhs.parameters.len())
}
//#endregion 🌉️ExternalCodecBridge

//#region 🔁️Re-exports
/// 🔁️ Entities this module's schema exports and its crate declares elsewhere.
pub use semio_framework_graph::manifest::PropertyValue;
//#endregion 🔁️Re-exports

#[cfg(test)]
#[path = "🧪️tests/🪶️sqlite/🦀️.rs"]
mod sqlite_snapshot_tests;

#[path="🔣️json/🦀️.rs"]
pub mod json;

#[path="🪶️sqlite/🦀️.rs"]
mod sqlite;
