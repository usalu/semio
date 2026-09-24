//! 🧬️ Playbook snapshot schema — artifact-lane fields only.
//!
//! Ticket `26/08/12/UNIFIED-COMPOSABLE-ARTIFACT-SYSTEM` (`playbook→C:document,flow`): `steps:
//! Vec<PlaybookStep>` is replaced by the composed `document`/`flow` child slots (see the artifact
//! root's `🔖️ContentBridge` region). This struct no longer maps 1:1 onto the kernel `PlaybookSpec`:
//! `store::ArtifactDsl` and `ArtifactPack` are the derived text and pack of `PlaybookPackRecord`.
//! Both carry the steps the `flow` handle's local owner holds, because the
//! parent is the state every composed child is derived from (`genesis_playbook_child_pack`).

use crate::PlaybookStep;
use framework_schema::ArtifactSchema;

//#region 🔖️Snapshot
/// 📸️ Persisted playbook document snapshot (persistent fields of the artifact). `#[child(...)]`
/// drives `#[derive(ArtifactSchema)]`'s slot-table emission; never hand-written.
#[derive(Clone, Debug, PartialEq, ArtifactSchema)]
#[artifact_schema(id = "s.playbook.playbook")]
pub struct PlaybookSnapshot {
    #[state(artifact)]
    pub schema: String,
    #[state(artifact)]
    pub id: String,
    #[state(artifact)]
    pub version: String,
    #[state(artifact)]
    pub title: Option<String>,
    #[state(artifact)]
    #[child(kind = "s.stdio.semio")]
    pub document: crate::PlaybookDocumentChild,
    #[state(artifact)]
    #[child(kind = "s.stdio.semio")]
    pub flow: crate::PlaybookFlowChild,
}

impl Default for PlaybookSnapshot {
    fn default() -> Self {
        let kernel = crate::playbook::empty_playbook_snapshot();
        Self::from_kernel(crate::playbook::PlaybookSpec { schema: kernel.schema, id: kernel.id, version: kernel.version, title: kernel.title, steps: kernel.steps })
    }
}

impl PlaybookSnapshot {
    /// 🌉️ Builds a plugin snapshot from the shared kernel `PlaybookSpec`, minting/caching the
    /// composed `document`/`flow` children from its `steps`.
    pub fn from_kernel(spec: crate::playbook::PlaybookSpec) -> Self {
        crate::playbook_snapshot_with_steps(&spec.schema, &spec.id, &spec.version, spec.title, spec.steps)
    }

    /// 🌉️ Lowers this snapshot into the kernel `PlaybookSpec` for shared domain helpers — reads
    /// steps off the `flow` child's working-scene cache (see the artifact root's `🔖️WorkingScene`).
    pub fn to_kernel(self) -> crate::playbook::PlaybookSpec {
        self.as_kernel()
    }

    /// 🌉️ Borrows as kernel spec without consuming `self`.
    pub fn as_kernel(&self) -> crate::playbook::PlaybookSpec {
        crate::playbook::PlaybookSpec { schema: self.schema.clone(), id: self.id.clone(), version: self.version.clone(), title: self.title.clone(), steps: crate::playbook_steps(self) }
    }

    /// 🔎️ The current steps, read through the composed `flow` child's working-scene cache — the
    /// single call site every render/inference/export path in this plugin uses instead of the old
    /// `.steps` field access.
    pub fn steps(&self) -> Vec<PlaybookStep> {
        crate::playbook_steps(self)
    }
}
//#endregion 🔖️Snapshot

//#region 🔖️ValueCodec
/// 🔀️ Hand-written, not derived — mirrors the sibling `PlaybookArtifact` impl one region up
/// (`../🦀️.rs`'s `🔖️ValueCodec`): `document`/`flow` are `store::ArtifactChild<S>` composed-artifact
/// handles, bridged per-field through the pre-existing `to_dsl_value`/`from_dsl_value` seam instead
/// of widening the derive macro to understand child-slot handles.
impl ::semio_framework_os_kernel::ToValue for PlaybookSnapshot {
    fn to_value(&self) -> ::semio_framework_os_kernel::DslValue {
        ::semio_framework_os_kernel::DslValue::object([
            ("schema".to_string(), ::semio_framework_os_kernel::ToValue::to_value(&self.schema)),
            ("id".to_string(), ::semio_framework_os_kernel::ToValue::to_value(&self.id)),
            ("version".to_string(), ::semio_framework_os_kernel::ToValue::to_value(&self.version)),
            ("title".to_string(), ::semio_framework_os_kernel::ToValue::to_value(&self.title)),
            ("document".to_string(), ::semio_framework_os_kernel::to_dsl_value(&self.document).expect("ArtifactChild serializes")),
            ("flow".to_string(), ::semio_framework_os_kernel::to_dsl_value(&self.flow).expect("ArtifactChild serializes")),
        ])
    }
}
impl ::semio_framework_os_kernel::FromValue for PlaybookSnapshot {
    fn from_value(value: ::semio_framework_os_kernel::DslValue) -> Result<Self, ::semio_framework_os_kernel::ValueError> {
        let entries = value.into_object()?;
        let get = |key: &str| entries.iter().find(|(k, _)| k == key).map(|(_, v)| v.clone());
        let field = |key: &str| get(key).ok_or_else(|| ::semio_framework_os_kernel::ValueError::new(format!("missing field `{key}`")));
        Ok(Self {
            schema: ::semio_framework_os_kernel::FromValue::from_value(field("schema")?)?,
            id: ::semio_framework_os_kernel::FromValue::from_value(field("id")?)?,
            version: ::semio_framework_os_kernel::FromValue::from_value(field("version")?)?,
            title: ::semio_framework_os_kernel::FromValue::from_value(field("title")?)?,
            document: ::semio_framework_os_kernel::from_dsl_value(field("document")?).map_err(::semio_framework_os_kernel::ValueError::new)?,
            flow: ::semio_framework_os_kernel::from_dsl_value(field("flow")?).map_err(::semio_framework_os_kernel::ValueError::new)?,
        })
    }
}
//#endregion 🔖️ValueCodec

//#region 🔖️PackRecord
/// 📦️ Derived pack record of a `PlaybookSnapshot`: both composed-child handles plus the steps the
/// `flow` handle's local owner holds, which a bare-handle pack would lose. The steps travel as their
/// first-party JSON text because block `default`/`params` are free-form `DslValue`s whose key order is
/// significant, while pack canonicalises map keys into sorted order.
#[derive(dsl::DslRecord)]
#[dsl(extension = "playbook")]
struct PlaybookPackRecord {
    schema: String,
    id: String,
    version: String,
    title: Option<String>,
    document: crate::PlaybookDocumentChild,
    flow: crate::PlaybookFlowChild,
    steps: String,
}

impl PlaybookPackRecord {
    fn from_snapshot(snapshot: &PlaybookSnapshot) -> Self {
        Self {
            schema: snapshot.schema.clone(),
            id: snapshot.id.clone(),
            version: snapshot.version.clone(),
            title: snapshot.title.clone(),
            document: snapshot.document.clone(),
            flow: snapshot.flow.clone(),
            steps: protocol::json::to_json_string(&crate::playbook_steps(snapshot)),
        }
    }

    fn into_snapshot(self) -> Result<PlaybookSnapshot, String> {
        let mut flow = self.flow;
        let steps: Vec<PlaybookStep> = protocol::json::from_json_str(&self.steps).map_err(|error| error.to_string())?;
        crate::attach_playbook_steps(&mut flow, steps);
        Ok(PlaybookSnapshot { schema: self.schema, id: self.id, version: self.version, title: self.title, document: self.document, flow })
    }
}
//#endregion 🔖️PackRecord

//#region 🔖️HandcraftedArtifactCodecs
impl store::ArtifactDsl for PlaybookSnapshot {
    const EXTENSION: &'static str = "playbook";
    fn envelope_id() -> &'static str {
        "playbook.playbook"
    }
    fn parse_dsl(text: &str) -> Result<Self, store::TextError> {
        let body = match store::semio_format::split_text_preamble(text) {
            Ok((_, rest)) => rest,
            Err(_) => text,
        };
        let record = dsl::parse(body, &PlaybookPackRecord::__dsl_spec(), &dsl::ParseOptions { limits: dsl::Limits::default(), mode: dsl::SourceMode::Document })?;
        PlaybookPackRecord::__dsl_from_record(&record)?.into_snapshot().map_err(|error| store::TextError::new(error, dsl::TextSpan::at(1, 1)))
    }
    fn print_dsl(&self) -> String {
        let body = dsl::print(&PlaybookPackRecord::from_snapshot(self).__dsl_to_record(), &PlaybookPackRecord::__dsl_spec(), dsl::JoinMode::Document);
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(<Self as store::ArtifactDsl>::envelope_id(), store::semio_format::Component::Dsl, 1).expect("valid envelope_id");
        store::semio_format::wrap_text(&envelope, &body)
    }
}

impl store::ArtifactPack for PlaybookSnapshot {
    fn encode_pack_with(&self, options: &store::PackEncodeOptions) -> Result<Vec<u8>, store::PackError> {
        let inner = store::pack_rt::encode_document(&PlaybookPackRecord::__dsl_spec(), &PlaybookPackRecord::from_snapshot(self).__dsl_to_record(), options)?;
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(<Self as store::ArtifactDsl>::envelope_id(), store::semio_format::Component::Pack, 1).map_err(|e| store::PackError::Schema(e.to_string()))?;
        Ok(store::semio_format::wrap_binary(&envelope, &inner))
    }
    fn decode_pack_with(bytes: &[u8], options: &store::PackDecodeOptions) -> Result<Self, store::PackError> {
        let (envelope, inner) = store::semio_format::unwrap_binary(bytes).map_err(|e| store::PackError::Schema(e.to_string()))?;
        if !envelope.matches_identity(<Self as store::ArtifactDsl>::envelope_id(), store::semio_format::Component::Pack, 1) {
            return Err(store::PackError::Schema(format!("pack envelope mismatch: expected {}.pack v1, got {}", <Self as store::ArtifactDsl>::envelope_id(), envelope.binary_token())));
        }
        let (record, _report) = store::pack_rt::decode_document(&inner, &PlaybookPackRecord::__dsl_spec(), options)?;
        PlaybookPackRecord::__dsl_from_record(&record).map_err(store::text_error_to_pack_error)?.into_snapshot().map_err(store::PackError::Schema)
    }
    fn record_spec() -> Option<dsl::RecordSpec> {
        Some(PlaybookPackRecord::__dsl_spec())
    }
}
//#endregion 🔖️HandcraftedArtifactCodecs

//#region 🔖️ExternalBridges
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
//#endregion 🔖️ExternalBridges
