//! 🧬️ Curation snapshot schema — artifact-lane fields only.

use crate::artifacts::curation::{CuratedItem, ObjectKindExtra};
use schema::ArtifactSchema;
use semio_s_plugin_stdio::artifacts::semio::standards::v1::subsets::kit::schema::snapshot::SemioKitSnapshot;

//#region 🔖️Snapshot
/// 📸️ Persisted curation document snapshot (persistent fields of the artifact). `catalog`/`stock_extra`
/// together replace the former inline `stock: Vec<ObjectKind>` field: `catalog` composes stdio's
/// `s.stdio.semio.kit` subset as an owned child (the shared `id`/`name`/`category` type-registry
/// vocabulary), `stock_extra` carries the sourcing-owned overflow (`typologyPath`/`availability`/
/// `geometry`) that subset can't represent — see `crate::artifacts::curation::stock_of` for the
/// reassembly accessor every reader funnels through.
#[derive(Clone, Debug, PartialEq, dsl::ToValue, dsl::FromValue, dsl::DslRecord, ArtifactSchema)]
#[value(rename_all = "camelCase")]
#[dsl(id = "curation.curation", layout = "lines")]
#[artifact_schema(id = "s.sourcing.curation")]
pub struct CurationSnapshot {
    #[state(artifact)]
    #[child(kind = "s.stdio.semio.kit")]
    pub catalog: store::ArtifactChild<SemioKitSnapshot>,
    #[state(artifact)]
    #[value(default)]
    pub stock_extra: Vec<ObjectKindExtra>,
    #[state(artifact)]
    #[value(default)]
    #[dsl(table)]
    pub curated: Vec<CuratedItem>,
}

impl Default for CurationSnapshot {
    /// 🌱 `ArtifactChild<S>` has no blanket `Default` (its target is content-addressed, never
    /// arbitrary), so this is hand-written rather than derived — mints the same empty-stock handle
    /// `catalog_child_handle(&[])` would, matching an explicitly-built empty document.
    fn default() -> Self {
        Self { catalog: crate::artifacts::curation::catalog_child_handle(&[]), stock_extra: Vec::new(), curated: Vec::new() }
    }
}
//#region 🔖️HandcraftedArtifactCodecs
/// ✉️ P6 handcrafted ArtifactDsl/ArtifactPack (derive no longer emits these traits).
impl store::ArtifactDsl for CurationSnapshot {
    const EXTENSION: &'static str = "curation";
    fn envelope_id() -> &'static str {
        "curation.curation"
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

impl store::ArtifactPack for CurationSnapshot {
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
/// 📤️ Renders a [`CurationSnapshot`] as this facet's own camelCase JSON projection — the comparison
/// surface `mutate-curation-1`'s scenarios are measured through, and the shape the committed
/// `../🧬️mutations/<slug>/🧪️tests/<fixture>/📸️snapshot/{⬅️before,➡️after}/🔣️.json`
/// specification vectors are written in. `curated` travels as an ORDERED list, which is what makes
/// the append-at-the-end and restore-in-place claims below checkable at all.
///
/// A thin `dsl::json` wrapper over this type's own `ToValue` codec — ticket
/// `26/09/01/RUNTIME-DEPENDENCY-ELIMINATION-FOR-S-PLUGINS-AND-ARTIFACTS` retired the `serde_json`
/// bridge this used to be, since `ArtifactChild<SemioKitSnapshot>` no longer implements `Serialize`.
pub fn encode_curation_snapshot_json(snapshot: &CurationSnapshot) -> String {
    dsl::json::to_json_string(snapshot)
}

/// 📥️ The inverse of [`encode_curation_snapshot_json`] — decodes those committed specification
/// vectors into real [`CurationSnapshot`] values, so `mutate-curation-1`'s adapter reads the committed
/// fixture rather than re-declaring it as a Rust literal beside it. Reaching `serde_json` from that
/// adapter is impossible: the generated test host links only this crate and `semio-repo-test-host`.
pub fn decode_curation_snapshot_json(text: &str) -> Result<CurationSnapshot, String> {
    dsl::json::from_json_str(text).map_err(|error| error.to_string())
}

/// 📝️ Parses `.curation.dsl.semio` text into a [`CurationSnapshot`] — a named, non-async pass-through of
/// this type's own `store::ArtifactDsl` impl above, whose trait and error type are both unnameable
/// outside this crate, so `mutate-curation-1`'s `identity-round-trip` scenario reaches the real
/// committed artifact (`../../📚️examples/🎬️demo/🖼️assets/🗣️.dsl.semio`) through this instead.
pub fn parse_curation_dsl(text: &str) -> Result<CurationSnapshot, String> {
    <CurationSnapshot as store::ArtifactDsl>::parse_dsl(text).map_err(|error| format!("{error:?}"))
}

/// 📝️ Renders a [`CurationSnapshot`] back as `.curation.dsl.semio` text — the inverse of
/// [`parse_curation_dsl`], preamble, catalog handle, stock table and curated table included.
pub fn print_curation_dsl(snapshot: &CurationSnapshot) -> String {
    store::ArtifactDsl::print_dsl(snapshot)
}

/// 🔎️ The curation as `objectId x count` pairs in list order — the readable half of a divergence
/// message, so a failing scenario names WHICH entry moved rather than only that two documents differ.
pub fn curation_selection_summary(snapshot: &CurationSnapshot) -> String {
    snapshot.curated.iter().map(|item| format!("{}x{}", item.object_id, item.count)).collect::<Vec<_>>().join(" ")
}
//#endregion 🌉️ExternalCodecBridge
