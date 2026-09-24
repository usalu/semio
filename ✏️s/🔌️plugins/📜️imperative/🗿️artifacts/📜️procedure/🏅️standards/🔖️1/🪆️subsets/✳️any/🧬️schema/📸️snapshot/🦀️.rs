//! 🧬️ Imperative snapshot schema — artifact-lane fields only.

use crate::{ProcedureFlowChild, ProcedureTextChild};
use framework_schema::ArtifactSchema;

//#region 🔖️Snapshot
/// 📸️ Persisted imperative document snapshot (persistent fields of the artifact). Ticket
/// `26/08/12/UNIFIED-COMPOSABLE-ARTIFACT-SYSTEM` (`imperative→C:text,flow`): the inline `path:
/// Path` (the ordered/nested `Step` control-flow tree) and `seed: BTreeMap<String, Value>` (the
/// initial variable dictionary) content fields are replaced by two fixed composed CHILD slots —
/// this plugin no longer defines its own program-graph or seed-content model, it composes stdio's
/// `flow` and `text` subsets instead. `#[child(...)]` drives `#[derive(ArtifactSchema)]`'s
/// slot-table emission; never hand-written.
#[derive(Clone, Debug, PartialEq, dsl::ToValue, dsl::FromValue, ArtifactSchema)]
#[value(rename_all = "camelCase", deny_unknown_fields)]
#[artifact_schema(id = "s.imperative.procedure")]
pub struct ProcedureSnapshot {
    #[state(artifact)]
    pub schema: String,
    #[state(artifact)]
    #[child(kind = "s.stdio.semio")]
    pub flow: ProcedureFlowChild,
    #[state(artifact)]
    #[child(kind = "s.stdio.semio")]
    pub text: ProcedureTextChild,
}

impl Default for ProcedureSnapshot {
    fn default() -> Self {
        crate::procedure_snapshot_with_content("procedure.document", &crate::Path::new(), &std::collections::BTreeMap::new())
    }
}
//#endregion 🔖️Snapshot

//#region 🔖️PackRecord
/// 🛤️ Derived pack record of a `ProcedureSnapshot`: both composed-child handles plus the content their
/// local owners hold (the flow child's `path`, the text child's `seed`), which a bare-handle pack would
/// lose. Text and pack are both derived from this record.
#[derive(dsl::DslRecord)]
#[dsl(extension = "imperative")]
struct ProcedurePackRecord {
    schema: String,
    flow: ProcedureFlowChild,
    text: ProcedureTextChild,
    path: dsl::DslValue,
    seed: dsl::DslValue,
}

impl ProcedurePackRecord {
    fn from_snapshot(snapshot: &ProcedureSnapshot) -> Self {
        let scene = crate::procedure_working_scene(snapshot);
        Self { schema: snapshot.schema.clone(), flow: snapshot.flow.clone(), text: snapshot.text.clone(), path: dsl::ToValue::to_value(&scene.path), seed: dsl::ToValue::to_value(&scene.seed) }
    }

    fn into_snapshot(self) -> Result<ProcedureSnapshot, String> {
        let (mut flow, mut text) = (self.flow, self.text);
        let seed: std::collections::BTreeMap<String, crate::Value> = dsl::FromValue::from_value(self.seed).map_err(|error| error.to_string())?;
        text.set_local_owner(std::sync::Arc::new(crate::ProcedureTextWorkingData { seed }));
        let path: crate::Path = dsl::FromValue::from_value(self.path).map_err(|error| error.to_string())?;
        flow.set_local_owner(std::sync::Arc::new(crate::ProcedureFlowWorkingData { path }));
        Ok(ProcedureSnapshot { schema: self.schema, flow, text })
    }
}

/// 🖨️ The derived text body: the same `ProcedurePackRecord` the pack encodes, printed by the spec-driven engine.
pub(crate) fn print_pack_record_text(snapshot: &ProcedureSnapshot) -> String {
    dsl::print(&ProcedurePackRecord::from_snapshot(snapshot).__dsl_to_record(), &ProcedurePackRecord::__dsl_spec(), dsl::JoinMode::Document)
}

/// 📖️ Parses a derived text body back through `ProcedurePackRecord`, with the same decode steps as the pack.
pub(crate) fn parse_pack_record_text(body: &str) -> Result<ProcedureSnapshot, store::TextError> {
    let record = dsl::parse(body, &ProcedurePackRecord::__dsl_spec(), &dsl::ParseOptions { limits: dsl::Limits::default(), mode: dsl::SourceMode::Document })?;
    ProcedurePackRecord::__dsl_from_record(&record)?.into_snapshot().map_err(|error| store::TextError::new(error, dsl::TextSpan::at(1, 1)))
}
//#endregion 🔖️PackRecord

//#region 🔖️HandcraftedArtifactCodecs
/// 🎁 `ArtifactDsl` and `ArtifactPack` are the derived text and pack of `ProcedurePackRecord`.
impl store::ArtifactDsl for ProcedureSnapshot {
    const EXTENSION: &'static str = "imperative";
    fn envelope_id() -> &'static str {
        "imperative.imperative"
    }
    fn parse_dsl(text: &str) -> Result<Self, store::TextError> {
        let body = match store::semio_format::split_text_preamble(text) {
            Ok((_, rest)) => rest,
            Err(_) => text,
        };
        parse_pack_record_text(body)
    }
    fn print_dsl(&self) -> String {
        let body = print_pack_record_text(self);
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(<Self as store::ArtifactDsl>::envelope_id(), store::semio_format::Component::Dsl, 1).expect("valid envelope_id");
        store::semio_format::wrap_text(&envelope, &body)
    }
}

impl store::ArtifactPack for ProcedureSnapshot {
    fn encode_pack_with(&self, options: &store::PackEncodeOptions) -> Result<Vec<u8>, store::PackError> {
        let inner = store::pack_rt::encode_document(&ProcedurePackRecord::__dsl_spec(), &ProcedurePackRecord::from_snapshot(self).__dsl_to_record(), options)?;
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(<Self as store::ArtifactDsl>::envelope_id(), store::semio_format::Component::Pack, 1).map_err(|e| store::PackError::Schema(e.to_string()))?;
        Ok(store::semio_format::wrap_binary(&envelope, &inner))
    }
    fn decode_pack_with(bytes: &[u8], options: &store::PackDecodeOptions) -> Result<Self, store::PackError> {
        let (envelope, inner) = store::semio_format::unwrap_binary(bytes).map_err(|e| store::PackError::Schema(e.to_string()))?;
        if !envelope.matches_identity(<Self as store::ArtifactDsl>::envelope_id(), store::semio_format::Component::Pack, 1) {
            return Err(store::PackError::Schema(format!("pack envelope mismatch: expected {}.pack v1, got {}", <Self as store::ArtifactDsl>::envelope_id(), envelope.binary_token())));
        }
        let (record, _report) = store::pack_rt::decode_document(&inner, &ProcedurePackRecord::__dsl_spec(), options)?;
        ProcedurePackRecord::__dsl_from_record(&record).map_err(store::text_error_to_pack_error)?.into_snapshot().map_err(store::PackError::Schema)
    }
    fn record_spec() -> Option<dsl::RecordSpec> {
        Some(ProcedurePackRecord::__dsl_spec())
    }
}
//#endregion 🔖️HandcraftedArtifactCodecs

//#region 🌉️ExternalCodecBridge
/// 📤️ Renders an [`ProcedureSnapshot`] as this facet's own camelCase JSON projection — the
/// comparison surface `🛟️mutate-procedure-1`'s scenarios are measured through, and the shape the
/// committed `../🧫️fixtures/🧬️mutations/<slug>/<fixture>/📸️snapshot/{⬅️before,➡️after}/🔣️.json`
/// specification vectors are written in. It carries `flow` and `text` as content-addressed HANDLES,
/// never as content, which is what makes it a usable observability surface here: the `flow` handle
/// moves if and only if the program moved.
///
/// A thin `dsl::os_pack::json` wrapper over `ProcedureSnapshot`'s own `ToValue` impl — first-party,
/// infallible (RUNTIME-DEPENDENCY-ELIMINATION-FOR-S-PLUGINS-AND-ARTIFACTS).
pub fn encode_procedure_snapshot_json(snapshot: &ProcedureSnapshot) -> String {
    dsl::os_pack::json::to_json_string(snapshot)
}

/// 📥️ The inverse of [`encode_procedure_snapshot_json`] — decodes those committed specification
/// vectors into real [`ProcedureSnapshot`] values, so `🛟️mutate-procedure-1`'s adapter reads the
/// committed fixture rather than re-declaring it as a Rust literal beside it.
pub fn decode_procedure_snapshot_json(text: &str) -> Result<ProcedureSnapshot, String> {
    dsl::os_pack::json::from_json_str(text).map_err(|error| error.to_string())
}

/// 📝️ Parses `.imperative.dsl.semio` text into an [`ProcedureSnapshot`] — a named, non-async
/// pass-through of this type's own `store::ArtifactDsl` impl above, whose trait and error type are
/// both unnameable outside this crate, so `🛟️mutate-procedure-1`'s `identity-round-trip` scenario
/// reaches the real committed artifact (`../../🖼️assets/🎬️demo/🗣️.dsl.semio`)
/// through this instead.
pub fn parse_procedure_dsl(text: &str) -> Result<ProcedureSnapshot, String> {
    <ProcedureSnapshot as store::ArtifactDsl>::parse_dsl(text).map_err(|error| format!("{error:?}"))
}

/// 📝️ Renders an [`ProcedureSnapshot`] back as `.imperative.dsl.semio` text — the inverse of
/// [`parse_procedure_dsl`], preamble and both composed child handles included.
pub fn print_procedure_dsl(snapshot: &ProcedureSnapshot) -> String {
    store::ArtifactDsl::print_dsl(snapshot)
}
//#endregion 🌉️ExternalCodecBridge

