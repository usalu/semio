//! 🧬️ Rewrite snapshot schema — artifact-lane fields only.

use crate::artifacts::jack::PropertyValue;
use crate::artifacts::rewrite::LayoutPoint;
use schema::ArtifactSchema;
use std::collections::BTreeMap;

//#region 🔖️Snapshot
/// 📸️ Persisted rewrite-rule document snapshot (persistent fields of the artifact).
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::DslRecord, ArtifactSchema)]
#[value(rename_all = "camelCase")]
#[dsl(extension = "rewrite", layout = "lines")]
#[artifact_schema(id = "s.trinity.rewrite")]
pub struct RewriteSnapshot {
    #[state(artifact)]
    pub before_fixture_json: String,
    #[state(artifact)]
    pub lhs_json: String,
    #[state(artifact)]
    #[dsl(lang = "json")]
    pub rhs_json: String,
    #[state(artifact)]
    #[value(default)]
    pub parameter_bindings: BTreeMap<String, PropertyValue>,
    #[state(artifact)]
    #[value(default)]
    pub rule_layout: BTreeMap<String, LayoutPoint>,
}
//#region 🔖️HandcraftedArtifactCodecs
/// ✉️ P6 handcrafted ArtifactDsl/ArtifactPack (derive no longer emits these traits).
impl store::ArtifactDsl for RewriteSnapshot {
    const EXTENSION: &'static str = "rewrite";
    fn envelope_id() -> &'static str {
        "trinity.rewrite"
    }
    fn parse_dsl(text: &str) -> Result<Self, store::TextError> {
        let body = match store::semio_format::split_text_preamble(text) {
            Ok((_, rest)) => rest,
            Err(_) => text,
        };
        let record = dsl::parse(body, &Self::__dsl_spec(), &dsl::ParseOptions { limits: dsl::Limits::default(), mode: dsl::SourceMode::Document })?;
        Self::__dsl_from_record(&record)
    }
    fn print_dsl(&self) -> String {
        let body = dsl::print(&self.__dsl_to_record(), &Self::__dsl_spec(), dsl::JoinMode::Document);
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(<Self as store::ArtifactDsl>::envelope_id(), store::semio_format::Component::Dsl, 1).expect("valid envelope_id");
        store::semio_format::wrap_text(&envelope, &body)
    }
}

impl store::ArtifactPack for RewriteSnapshot {
    fn encode_pack_with(&self, options: &store::PackEncodeOptions) -> Result<Vec<u8>, store::PackError> {
        let inner = store::pack_rt::encode_document(&Self::__dsl_spec(), &self.__dsl_to_record(), options)?;
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(<Self as store::ArtifactDsl>::envelope_id(), store::semio_format::Component::Pack, 1).map_err(|e| store::PackError::Schema(e.to_string()))?;
        Ok(store::semio_format::wrap_binary(&envelope, &inner))
    }
    fn decode_pack_with(bytes: &[u8], options: &store::PackDecodeOptions) -> Result<Self, store::PackError> {
        let (envelope, inner) = store::semio_format::unwrap_binary(bytes).map_err(|e| store::PackError::Schema(e.to_string()))?;
        if envelope.envelope_id() != <Self as store::ArtifactDsl>::envelope_id() {
            return Err(store::PackError::Schema(format!("pack envelope mismatch: expected {}, got {}", <Self as store::ArtifactDsl>::envelope_id(), envelope.envelope_id())));
        }
        let (record, _report) = store::pack_rt::decode_document(&inner, &Self::__dsl_spec(), options)?;
        Self::__dsl_from_record(&record).map_err(store::text_error_to_pack_error)
    }
    fn record_spec() -> Option<dsl::RecordSpec> {
        Some(Self::__dsl_spec())
    }
}
//#endregion 🔖️HandcraftedArtifactCodecs
//#endregion 🔖️Snapshot

//#region 🌉️ExternalCodecBridge
/// 📤️ Renders a [`RewriteSnapshot`] as this facet's own camelCase JSON projection — the comparison
/// surface `mutate-rewrite-1`'s scenarios are measured through, and the shape the committed
/// `../🧬️mutations/<slug>/🧪️tests/<fixture>/📸️snapshot/{⬅️before,➡️after}/🔣️component.json`
/// specification vectors are written in. The three authored bodies travel as opaque JSON STRINGS,
/// so the projection is JSON containing JSON — which is exactly the shape a transcribed Rust
/// literal gets wrong silently.
///
/// A thin `serde_json` wrapper (already a direct dependency of this crate, used behind this
/// interface per CLAUDE.md's "external libraries behind an interface" rule, never a new one).
pub fn encode_rewrite_snapshot_json(snapshot: &RewriteSnapshot) -> String {
    pack::to_json_string(snapshot)
}

/// 📥️ The inverse of [`encode_rewrite_snapshot_json`] — decodes those committed specification
/// vectors into real [`RewriteSnapshot`] values, so `mutate-rewrite-1`'s adapter reads the committed
/// fixture rather than re-declaring it as a Rust literal beside it. Reaching `serde_json` from that
/// adapter is impossible: the generated test host links only this crate and `semio-repo-test-host`.
pub fn decode_rewrite_snapshot_json(text: &str) -> Result<RewriteSnapshot, String> {
    pack::from_json_str(text).map_err(|error| error.to_string())
}

/// 📝️ Parses `.rewrite.dsl.semio` text into a [`RewriteSnapshot`] — a named, non-async pass-through
/// of this type's own handcrafted `store::ArtifactDsl` impl above, whose trait and error type are
/// both unnameable outside this crate, so `mutate-rewrite-1`'s `identity-round-trip` scenario
/// reaches the real committed artifact
/// (`../../📚️examples/🎬️demo/🖼️assets/🗣️example.dsl.semio`) through this instead.
pub fn parse_rewrite_dsl(text: &str) -> Result<RewriteSnapshot, String> {
    <RewriteSnapshot as store::ArtifactDsl>::parse_dsl(text).map_err(|error| format!("{error:?}"))
}

/// 📝️ Renders a [`RewriteSnapshot`] back as `.rewrite.dsl.semio` text — the inverse of
/// [`parse_rewrite_dsl`], preamble, quoted body strings and fenced `rhs-json` block included.
pub fn print_rewrite_dsl(snapshot: &RewriteSnapshot) -> String {
    store::ArtifactDsl::print_dsl(snapshot)
}

/// 🔎️ The rule's two keyed maps as sorted key lists plus the byte lengths of its three authored
/// bodies — the readable half of a divergence message, so a failing scenario names WHICH axis moved
/// rather than only that two long JSON-in-JSON documents differ.
pub fn rewrite_rule_summary(snapshot: &RewriteSnapshot) -> String {
    format!(
        "bindings[{}] layout[{}] before={}B lhs={}B rhs={}B",
        snapshot.parameter_bindings.keys().cloned().collect::<Vec<_>>().join(" "),
        snapshot.rule_layout.keys().cloned().collect::<Vec<_>>().join(" "),
        snapshot.before_fixture_json.len(),
        snapshot.lhs_json.len(),
        snapshot.rhs_json.len()
    )
}
//#endregion 🌉️ExternalCodecBridge
