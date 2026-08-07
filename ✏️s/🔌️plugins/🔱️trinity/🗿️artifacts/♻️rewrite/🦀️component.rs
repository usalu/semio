//! ♻️ `trinity.rewrite.rule` artifact — document entities (constitutional: general).

use crate::artifacts::jack::PropertyValue;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

//#region ⚠️ Errors
/// ⚠️ Trinity rewrite-engine errors.
#[derive(Debug, thiserror::Error)]
pub enum TrinityRewriteError {
    /// 🧩️ Trinity graph fixture load/validation/mutation failure.
    #[error(transparent)]
    Graph(#[from] crate::artifacts::jack::TrinityRamError),
    /// 🧭️ VCS store/dispatch failure.
    #[error(transparent)]
    Vcs(#[from] vcs::VcsError),
    /// 🧬️ JSON (de)serialization failure.
    #[error(transparent)]
    Json(#[from] serde_json::Error),
    /// 🔤️ Jack query parse/execute failure (the shared `🫀️core` jack-query kernel's own API is not
    /// yet thiserror-migrated).
    #[error("{0}")]
    Jack(String),
    /// 📐️ Force-directed layout failure (`infinite_board_port_directed`'s own API is not yet
    /// thiserror-migrated).
    #[error("{0}")]
    Layout(String),
    /// 🎨️ Canvas theme merge failure (`infinite_board_port_directed`'s own API is not yet
    /// thiserror-migrated).
    #[error("{0}")]
    CanvasTheme(String),
    #[error("force layout fixture missing nodes")]
    ForceLayoutFixtureMissingNodes,
}
//#endregion ⚠️ Errors

//#region 🔖️Types
/// 📍️ Local `{x, y}` twin for a bare `(f64, f64)` tuple — the DSL engine's `DslField` binding has no
/// impl for raw Rust tuples (only named `DslRecord`/`DslScalar` types can bind), so `rule_layout`'s
/// value type is this named record instead, with `From`/`Into` conversions at this crate's own
/// remaining `(f64, f64)` call sites (tests only — no production logic reads `rule_layout` today).
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
pub struct LayoutPoint {
    pub x: f64,
    pub y: f64,
}

impl From<(f64, f64)> for LayoutPoint {
    fn from((x, y): (f64, f64)) -> Self {
        Self { x, y }
    }
}

impl From<LayoutPoint> for (f64, f64) {
    fn from(point: LayoutPoint) -> Self {
        (point.x, point.y)
    }
}

/// 📐️ The full rewrite-rule document: before fixture, LHS/RHS patterns, parameter bindings, and
/// rule-graph layout overrides. Every field binds directly through the `dsl::` engine: `rhs_json` is
/// `#[dsl(lang = "json")]` so its pretty-printed JSON blob prints as a fenced verbatim block instead
/// of an escaped quoted string — `before_fixture_json`/`lhs_json` deliberately stay plain
/// `Shape::Text` (bare `String`, no attribute): annotating more than one `_json` field here breaks the
/// fence lexer's "closing ``` must be alone on its line" rule (confirmed by a failing round-trip
/// test, reverted to only the last field). `parameter_bindings` is `BTreeMap<String, PropertyValue>`
/// (bare `HashMap` has no blanket `DslField` impl, only `BTreeMap` does), and `rule_layout` uses the
/// `LayoutPoint` twin above in place of a bare tuple.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
#[dsl(extension = "rewrite", layout = "lines")]
pub struct RewriteRuleModel {
    pub before_fixture_json: String,
    pub lhs_json: String,
    #[dsl(lang = "json")]
    pub rhs_json: String,
    #[serde(default)]
    pub parameter_bindings: BTreeMap<String, PropertyValue>,
    #[serde(default)]
    pub rule_layout: BTreeMap<String, LayoutPoint>,
}
//#region 🔖️HandcraftedDocumentCodecs
/// ✉️ P6 handcrafted DocumentDsl/DocumentPack (derive no longer emits these traits).
impl store::DocumentDsl for RewriteRuleModel {
    const EXTENSION: &'static str = "rewrite";
    fn envelope_id() -> &'static str { "rewrite" }
    fn parse_dsl(text: &str) -> Result<Self, store::TextError> {
        let body = match store::semio_format::split_text_preamble(text) {
            Ok((_, rest)) => rest,
            Err(_) => text,
        };
        let record = dsl::parse(
            body,
            &Self::__dsl_spec(),
            &dsl::ParseOptions { limits: dsl::Limits::default(), mode: dsl::SourceMode::Document },
        )?;
        Self::__dsl_from_record(&record)
    }
    fn print_dsl(&self) -> String {
        let body = dsl::print(&self.__dsl_to_record(), &Self::__dsl_spec(), dsl::JoinMode::Document);
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(
            <Self as store::DocumentDsl>::envelope_id(),
            store::semio_format::Component::Dsl,
            1,
        ).expect("valid envelope_id");
        store::semio_format::wrap_text(&envelope, &body)
    }
}

impl store::DocumentPack for RewriteRuleModel {
    fn encode_pack_with(&self, options: &store::PackEncodeOptions) -> Result<Vec<u8>, store::PackError> {
        let inner = store::pack_rt::encode_document(&Self::__dsl_spec(), &self.__dsl_to_record(), options)?;
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(
            <Self as store::DocumentDsl>::envelope_id(),
            store::semio_format::Component::Pack,
            1,
        ).map_err(|e| store::PackError::Schema(e.to_string()))?;
        Ok(store::semio_format::wrap_binary(&envelope, &inner))
    }
    fn decode_pack_with(bytes: &[u8], options: &store::PackDecodeOptions) -> Result<Self, store::PackError> {
        let (envelope, inner) = store::semio_format::unwrap_binary(bytes)
            .map_err(|e| store::PackError::Schema(e.to_string()))?;
        if envelope.envelope_id() != <Self as store::DocumentDsl>::envelope_id() {
            return Err(store::PackError::Schema(format!(
                "pack envelope mismatch: expected {}, got {}",
                <Self as store::DocumentDsl>::envelope_id(),
                envelope.envelope_id()
            )));
        }
        let (record, _report) = store::pack_rt::decode_document(&inner, &Self::__dsl_spec(), options)?;
        Self::__dsl_from_record(&record).map_err(store::text_error_to_pack_error)
    }
    fn record_spec() -> Option<dsl::RecordSpec> { Some(Self::__dsl_spec()) }
}
//#endregion 🔖️HandcraftedDocumentCodecs




pub const REWRITE_RULE_SCHEMA: &str = "trinity.rewrite.rule";
//#endregion 🔖️Types

// 📌️ Unlike `jack`, `rewrite`'s own `create_rewrite_app()` never calls `.artifact_kind(...)` in the
// old code (verified by reading the old `trinity_rewrite_ui` bundle chain) — rewrite has no
// `ArtifactKindSpec` of its own; it only declares `.io(rewrite_io())`, whose `graph:in`/`graph:out`
// ports reuse jack's `"graph.trinity"` kind id. Not a gap — preserved verbatim.

// 📜️ `RewriteRuleModel`/`RewriteRuleOperation` derive their `store::DocumentDsl`/`protocol::OpText`
// impls directly (see `#[derive(dsl::DslRecord)]` above and `#[derive(dsl::DslEnum)]` in `🔧️op`) —
// every field already binds through the `dsl::` engine with no foreign types, so no hand-written
// parser/printer or twin type is needed anywhere in this artifact (unlike `jack`'s `GraphFixture`).
