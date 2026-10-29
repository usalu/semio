//! 👥️ Sourcing curation presence — shareable live ephemeral state + mutations.

use protocol::Mutation;
use pack::snapshot::ArtifactPack;

//#region 🔖️Presence
/// 👥️ Shareable live subset of sourcing curation view state (grid camera). Row selection now broadcasts
/// automatically through the framework's typed `PresenceInteraction` field for the "rows" interaction
/// domain (ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM) — no longer mirrored here.
#[derive(semio_framework_dsl_record_derive::DslRecord, Clone, Debug, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_os_kernel::DslArtifact, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone, semio_framework_value_derive::NativeReceive)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[value(rename_all = "camelCase", default)]
#[cfg_attr(test, serde(rename_all = "camelCase", default))]
#[artifact(extension = "sourcingcuration.presence")]
#[dsl(layout = "lines")]
pub struct SourcingCurationPresence {
    #[value(required)]
    pub world_camera_position: [f64; 3],
    #[value(required)]
    pub world_camera_target: [f64; 3],
    pub world_camera_fov: f64,
}

impl Default for SourcingCurationPresence {
    fn default() -> Self {
        Self { world_camera_position: [2.5, 2.0, 2.5], world_camera_target: [0.0, 0.0, 0.0], world_camera_fov: 50.0 }
    }
}

impl store::ArtifactPresenceSnapshot for SourcingCurationPresence {
    fn decode_presence_native(bytes: &[u8], owner: &mut store::io::control::NativeSnapshotDecodeOwner<'_, '_>) -> Result<Self, semio_framework_value::ValueError> {
        store::decode_presence_json(bytes, owner)
    }
}

/// 🔺️ Sparse field delta over [`SourcingCurationPresence`]: every present slot is the new value of exactly that field.
#[derive(Clone, Debug, Default, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue)]
#[value(rename_all = "camelCase", default)]
pub struct SourcingCurationPresenceDiff {
    #[value(skip_serializing_if = "Option::is_none")]
    pub world_camera_position: Option<[f64; 3]>,
    #[value(skip_serializing_if = "Option::is_none")]
    pub world_camera_target: Option<[f64; 3]>,
    #[value(skip_serializing_if = "Option::is_none")]
    pub world_camera_fov: Option<f64>,
}

impl protocol::MutationDiff<SourcingCurationPresence> for SourcingCurationPresenceDiff {}

impl protocol::ColdMutationDiff<SourcingCurationPresence> for SourcingCurationPresenceDiff {
    fn absorb(&mut self, other: Self) {
        if other.world_camera_position.is_some() {
            self.world_camera_position = other.world_camera_position;
        }
        if other.world_camera_target.is_some() {
            self.world_camera_target = other.world_camera_target;
        }
        if other.world_camera_fov.is_some() {
            self.world_camera_fov = other.world_camera_fov;
        }
    }

    fn apply(&self, unpublished: &mut SourcingCurationPresence, _capability: protocol::ApplyCapability) -> protocol::MutationApplyResult<()> {
        let base = &*unpublished;
        let mut next = base.clone();
        if let Some(value) = &self.world_camera_position {
            next.world_camera_position = value.clone();
        }
        if let Some(value) = &self.world_camera_target {
            next.world_camera_target = value.clone();
        }
        if let Some(value) = &self.world_camera_fov {
            next.world_camera_fov = value.clone();
        }
        *unpublished = next;
        Ok(())
    }
}

impl protocol::DiffAlgebra<SourcingCurationPresence> for SourcingCurationPresenceDiff {
    fn is_empty(&self) -> bool {
        self.world_camera_position.is_none() && self.world_camera_target.is_none() && self.world_camera_fov.is_none()
    }
}

impl protocol::ColdDiffAlgebra<SourcingCurationPresence> for SourcingCurationPresenceDiff {
    fn inverse(&self, base: &SourcingCurationPresence) -> Self {
        Self {
            world_camera_position: self.world_camera_position.as_ref().map(|_| base.world_camera_position.clone()),
            world_camera_target: self.world_camera_target.as_ref().map(|_| base.world_camera_target.clone()),
            world_camera_fov: self.world_camera_fov.as_ref().map(|_| base.world_camera_fov.clone()),
        }
    }
}

impl pack::snapshot::ArtifactDsl for SourcingCurationPresence {
    const EXTENSION: &'static str = Self::__DSL_EXTENSION;
    fn envelope_id() -> &'static str {
        Self::__DSL_ENVELOPE_ID
    }
    fn parse_dsl(text: &str) -> Result<Self, semio_framework_diagnostic::TextError> {
        let body = match store::semio_format::split_text_preamble(text) {
            Ok((_, rest)) => rest,
            Err(_) => text,
        };
        if body.trim().is_empty() {
            return Ok(Self::default());
        }
        let record = semio_framework_dsl_record::parse(body, &Self::__dsl_spec(), &semio_framework_dsl_record::ParseOptions { limits: semio_framework_diagnostic::Limits::default(), mode: semio_framework_dsl_record::SourceMode::Document })?;
        Self::__dsl_from_record(&record)
    }
    fn print_dsl(&self) -> String {
        let body = semio_framework_dsl_record::print(&self.__dsl_to_record(), &Self::__dsl_spec(), semio_framework_dsl_record::JoinMode::Document);
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(<Self as pack::snapshot::ArtifactDsl>::envelope_id(), store::semio_format::Component::Dsl, 1).expect("valid envelope_id");
        store::semio_format::wrap_text(&envelope, &body)
    }
}

impl ArtifactPack for SourcingCurationPresence {
    fn encode_pack_with(&self, options: &store::PackEncodeOptions) -> Result<Vec<u8>, store::PackError> {
        let inner = store::pack_rt::encode_document(&Self::__dsl_spec(), &self.__dsl_to_record(), options)?;
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(<Self as pack::snapshot::ArtifactDsl>::envelope_id(), store::semio_format::Component::Pack, 1).map_err(|e| store::PackError::from(e.into_value_error()))?;
        Ok(store::semio_format::wrap_binary(&envelope, &inner))
    }
    fn decode_pack_with(bytes: &[u8], options: &store::PackDecodeOptions) -> Result<Self, store::PackError> {
        if bytes.is_empty() {
            return Ok(Self::default());
        }
        let (envelope, inner) = store::semio_format::unwrap_binary(bytes).map_err(|e| store::PackError::from(e.into_value_error()))?;
        if !envelope.matches_identity(<Self as pack::snapshot::ArtifactDsl>::envelope_id(), store::semio_format::Component::Pack, 1) {
            return Err(store::PackError::from(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, format!("pack envelope mismatch: expected {}.pack v1, got {}", <Self as pack::snapshot::ArtifactDsl>::envelope_id(), envelope.binary_token()))));
        }
        let (record, _report) = store::pack_rt::decode_document(&inner, &Self::__dsl_spec(), options)?;
        Self::__dsl_from_record(&record).map_err(store::os_store::serialization::text_error_to_pack_error)
    }
    fn record_spec() -> Option<semio_framework_dsl_record::RecordSpec> {
        Some(Self::__dsl_spec())
    }
}
impl ::semio_framework_os_kernel::os_store::serialization::ArtifactStoreSnapshot for SourcingCurationPresence {
    fn reference_snapshot_codec()->Option<::semio_framework_os_kernel::os_store::serialization::ArtifactReferenceSnapshotCodec<Self>>{Some(::semio_framework_os_kernel::os_store::serialization::reference_snapshot_codec::<Self>())}


}
//#endregion 🔖️Presence

//#region 🔖️PresenceMutation
#[derive(Clone, Debug, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_dsl_record_derive::DslEnum, semio_framework_value::RetireOwned)]
#[value(rename_all = "camelCase")]
pub enum SourcingCurationPresenceMutation {
    #[dsl(key = "world-camera")]
    SetWorldCamera {
        #[dsl(coord)]
        position: [f64; 3],
        #[dsl(coord)]
        target: [f64; 3],
        fov: f64,
    },
}

impl Mutation<SourcingCurationPresence> for SourcingCurationPresenceMutation {
    type Diff = SourcingCurationPresenceDiff;

    /// 🧷️ Hand-written (not `#[derive(dsl::Mutations)]`: a single whole-value snapshot replace, not a
    /// `dsl::Mutations`-eligible semantic-document vocabulary). ⚠️ PROVISIONAL: the `owner` path below
    /// names no directory on disk — this enum has no `🧬️mutations/<slug>` leaf triad of its own, so the
    /// entry is a metadata placeholder to satisfy `protocol::Mutation`, matching the sibling
    /// `🎚️config` enum and puzzle's `🖐️5d` precedent.
    const DESCRIPTORS: &'static [protocol::MutationLeafDescriptor] = &[
        protocol::MutationLeafDescriptor { schema_version: 1, owner: "✏️s/🔌️plugins/🪵️sourcing/🗿️artifacts/🗂️curation/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/👥️presence/set-world-camera", semantic_kind: "set-world-camera", display_name: "Set World Camera", emoji: "📄", aggregate_variant: "SetWorldCamera", payload_schema: "🧬️schema/🔣️.json", text_opcode: None, binary_tag: None, invertibility: protocol::MutationInvertibility::ExplicitMutation, diff_participation: protocol::MutationDiffParticipation::Detect, outcome_classes: &[protocol::MutationOutcomeClass::Applied], composition: protocol::MutationComposition::Atomic, required_language_surfaces: &[protocol::MutationLanguageSurface::Rust, protocol::MutationLanguageSurface::JsonSchema] },
    ];

    fn descriptor(&self) -> &'static protocol::MutationLeafDescriptor {
        match self {
            Self::SetWorldCamera { .. } => &Self::DESCRIPTORS[0],
        }
    }
}

impl protocol::ColdMutation<SourcingCurationPresence> for SourcingCurationPresenceMutation {
    type ReferenceDiff = SourcingCurationPresenceDiff;
    fn diff(&self, base: &SourcingCurationPresence) -> protocol::MutationOutcome<SourcingCurationPresenceDiff> {
        protocol::MutationOutcome::new(match self {
            Self::SetWorldCamera { position, target, fov } => SourcingCurationPresenceDiff { world_camera_position: (base.world_camera_position != *position).then_some(*position), world_camera_target: (base.world_camera_target != *target).then_some(*target), world_camera_fov: (base.world_camera_fov != *fov).then_some(*fov), ..Default::default() },
        })
    }
    fn inverse(&self, base: &SourcingCurationPresence) -> Result<Vec<Self>, semio_framework_value::ValueError> {
        Ok(vec![match self {
            Self::SetWorldCamera { .. } => Self::SetWorldCamera { position: base.world_camera_position, target: base.world_camera_target, fov: base.world_camera_fov },
        }])
    }
}

impl protocol::OpText for SourcingCurationPresenceMutation {
    fn parse_op(line: &str) -> Result<Self, semio_framework_diagnostic::TextError> {
        let variants = <Self as semio_framework_dsl_record::DslVariants>::variants();
        for (keyword, spec_fn) in &variants {
            let probe = format!("{keyword} ");
            if line == keyword.as_str() || line.starts_with(&probe) {
                let body = if line.len() > keyword.len() { line[keyword.len()..].trim_start() } else { "" };
                let record = semio_framework_dsl_record::parse(body, &(spec_fn.ordinary)(), &semio_framework_dsl_record::ParseOptions { limits: semio_framework_diagnostic::Limits::default(), mode: semio_framework_dsl_record::SourceMode::Inline })?;
                return <Self as semio_framework_dsl_record::DslVariants>::from_named_record(keyword, &record);
            }
        }
        Err(semio_framework_diagnostic::TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue,(format!("unknown operation line '{line}'")).to_string(),semio_framework_diagnostic::TextSpan::at(1,1)))
    }
    fn print_op(&self) -> String {
        let (keyword, record) = <Self as semio_framework_dsl_record::DslVariants>::to_named_record(self);
        let variants = <Self as semio_framework_dsl_record::DslVariants>::variants();
        let spec_fn = variants.iter().find(|(k, _)| k == &keyword).map(|(_, s)| *s).expect("variant spec must exist for its own keyword");
        let body = semio_framework_dsl_record::print(&record, &(spec_fn.ordinary)(), semio_framework_dsl_record::JoinMode::Inline);
        if body.is_empty() {
            keyword
        } else {
            format!("{keyword} {body}")
        }
    }
}

impl protocol::OpBinary for SourcingCurationPresenceMutation {
    fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        dsl::variants_binary::encode_op(self)
    }
    fn decode_op(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        dsl::variants_binary::decode_op(bytes)
    }
}
//#endregion 🔖️PresenceMutation

#[cfg(test)]
mod law_tests {
    use super::*;

    /// ⚖️ The inverse diffs sum to the negative of the forward diff (L3).
    #[semio_framework_async_macros::async_test]
    async fn inverse_diffs_sum_to_the_negative_diff() {
        let base = SourcingCurationPresence::default();
        protocol::os_spr::protocol_laws::assert_mutation_inverse_sum_law(&SourcingCurationPresenceMutation::SetWorldCamera { position: [1.0, 2.0, 3.0], target: [0.5, 0.0, 0.0], fov: 35.0 }, &base).await;
    }
}

//#region 🧩️ArtifactItems
pub mod artifact_items {
pub const SOURCING_CURATION_SCHEMA: &str = "sourcing.curation/v1";
}
//#endregion 🧩️ArtifactItems

//#region 🧩️ArtifactItems3
pub mod artifact_items_3 {
//#region 🔖️Geometry
/// 📦️ A parametric geometry recipe an object kind is composed of — data describing shape, not a subclass.
#[derive(Clone, Debug, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_dsl_record_derive::DslEnum, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
#[value(tag = "kind", rename_all = "camelCase", deny_unknown_fields)]
pub enum GeometryRecipe {
    Box {
        #[dsl(unit = "m")]
        width: f64,
        #[dsl(unit = "m")]
        height: f64,
        #[dsl(unit = "m")]
        depth: f64,
    },
    Frame {
        #[dsl(unit = "m")]
        width: f64,
        #[dsl(unit = "m")]
        height: f64,
        #[dsl(unit = "m")]
        depth: f64,
        #[dsl(unit = "m")]
        profile: f64,
    },
    Slab {
        #[dsl(unit = "m")]
        width: f64,
        #[dsl(unit = "m")]
        depth: f64,
        #[dsl(unit = "m")]
        thickness: f64,
    },
    Mesh {
        positions: Vec<f32>,
        normals: Vec<f32>,
        indices: Vec<u32>,
    },
    Glb {
        url: String,
        #[dsl(unit = "m")]
        extent: f64,
    },
}
//#endregion 🔖️Geometry
}
//#endregion 🧩️ArtifactItems3

//#region 🧩️ArtifactItems4
pub mod artifact_items_4 {
use crate::{GeometryRecipe};

//#region 🔖️ObjectKind
/// 🧱️ A catalogue object KIND: identity ∘ typology reference ∘ availability ∘ geometry (composition, not subclassing).
///
/// `geometry` is `Box<GeometryRecipe>` (not a bare `GeometryRecipe`) because `#[dsl(statements)]`'s
/// `RequiredStatements` shape — the "exactly one required tagged value" slot a `DslEnum` sum type
/// needs to occupy a plain (non-`Option`, non-`Vec`) field — only recognizes a `Box<T>` inner type.
#[derive(Clone, Debug, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_dsl_record_derive::DslRecord)]
#[value(rename_all = "camelCase", deny_unknown_fields)]
pub struct ObjectKind {
    #[dsl(defines = "object")]
    pub id: String,
    pub name: String,
    pub module_id: String,
    pub typology_path: Vec<String>,
    pub availability: u32,
    #[dsl(statements)]
    pub geometry: Box<GeometryRecipe>,
}
//#endregion 🔖️ObjectKind
}
//#endregion 🧩️ArtifactItems4

//#region 🧩️ArtifactItems6
pub mod artifact_items_6 {
use crate::{GeometryRecipe};
use crate::{ObjectKind};
use crate::{CuratedItem};
use semio_s_artifact_stdio_semio_kit::schema::snapshot::{SemioKitSnapshot, SemioKitType};
pub use crate::schema::snapshot::CurationSnapshot;

//#region 🔖️CatalogComposition
/// 🧩️ Sourcing-owned per-kind metadata NOT representable in stdio's composed `s.stdio.semio.kit`
/// subset (`SemioKitType` carries only `id`/`name`/`category`) — typology classification,
/// availability, and procedural geometry. Id-joined 1:1 to a `SemioKitType` in the composed
/// `CurationSnapshot::catalog` child by `id` (see `stock_of`/`object_kind_from_parts`). Ticket
/// 26/08/12/UNIFIED-COMPOSABLE-ARTIFACT-SYSTEM: replaces the former inline `stock: Vec<ObjectKind>`
/// field, which duplicated the `kit.catalog`/type-registry vocabulary this ticket composes instead.
#[derive(Clone, Debug, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
#[value(rename_all = "camelCase", deny_unknown_fields)]
pub struct ObjectKindExtra {
    #[dsl(defines = "object")]
    pub id: String,
    pub name: String,
    pub module_id: String,
    pub typology_path: Vec<String>,
    pub availability: u32,
    #[dsl(statements)]
    pub geometry: Box<GeometryRecipe>,
}

/// 🔀️ `ObjectKind` → the shared `SemioKitType` half of the composed catalog child. `category` maps
/// from `module_id` — the closest existing kit vocabulary slot for a grouping label (`SemioKitType`
/// has no separate module concept).
pub fn kit_type_from_object_kind(kind: &ObjectKind) -> SemioKitType {
    SemioKitType { id: kind.id.clone(), name: kind.name.clone(), category: kind.module_id.clone() }
}

/// 🔀️ `ObjectKind` → the sourcing-owned overflow half (`stock_extra`) the composed kit type cannot
/// carry. Lossless together with `kit_type_from_object_kind`: every `ObjectKind` field lands in
/// exactly one of the two halves.
pub fn object_kind_extra_from_object_kind(kind: &ObjectKind) -> ObjectKindExtra {
    ObjectKindExtra { id: kind.id.clone(), name: kind.name.clone(), module_id: kind.module_id.clone(), typology_path: kind.typology_path.clone(), availability: kind.availability, geometry: kind.geometry.clone() }
}

/// 🔀️ Inverse of the split above — reassembles one full `ObjectKind` from its two composed halves.
pub fn object_kind_from_parts(kit_type: &SemioKitType, extra: &ObjectKindExtra) -> ObjectKind {
    ObjectKind { id: kit_type.id.clone(), name: kit_type.name.clone(), module_id: kit_type.category.clone(), typology_path: extra.typology_path.clone(), availability: extra.availability, geometry: extra.geometry.clone() }
}

/// 🔀️ The full stock list's shared half, as a fresh (design-less, link-less) `SemioKitSnapshot` —
/// content-addressed by `catalog_child_handle` below, never embedded inline in `CurationSnapshot`.
pub fn catalog_snapshot_from_stock(stock: &[ObjectKind]) -> SemioKitSnapshot {
    SemioKitSnapshot { types: stock.iter().map(kit_type_from_object_kind).collect(), ..SemioKitSnapshot::default() }
}

/// 🔀️ The full stock list's sourcing-owned overflow half.
pub fn stock_extra_from_stock(stock: &[ObjectKind]) -> Vec<ObjectKindExtra> {
    stock.iter().map(object_kind_extra_from_object_kind).collect()
}

/// 🔀️ Reassembles the full `Vec<ObjectKind>` catalog from its composed-child half and its
/// sourcing-owned overflow half, id-joined. A `SemioKitType` with no matching `ObjectKindExtra`
/// (composed-child content the working-scene cache hasn't seen yet — see `stock_of`'s doc comment) is
/// silently dropped rather than fabricated with placeholder geometry.
pub fn stock_from_catalog_and_extra(catalog: &SemioKitSnapshot, extra: &[ObjectKindExtra]) -> Vec<ObjectKind> {
    let extra_by_id: std::collections::HashMap<&str, &ObjectKindExtra> = extra.iter().map(|e| (e.id.as_str(), e)).collect();
    catalog.types.iter().filter_map(|kit_type| extra_by_id.get(kit_type.id.as_str()).map(|extra| object_kind_from_parts(kit_type, extra))).collect()
}

/// 🪪️ Content-addressed child handle for a stock list's shared catalog half — hashes the deterministic
/// JSON of the derived `SemioKitType` list so peers replaying the same stock converge on the same
/// `child_id` (never a random/incrementing id), mirroring `lowpoly`'s `mesh_child_handle`.
pub fn catalog_child_handle(stock: &[ObjectKind]) -> store::os_store::artifact_child::ArtifactChild<SemioKitSnapshot> {
    let catalog = catalog_snapshot_from_stock(stock);
    let canonical = semio_framework_pack_json::to_json_string(&catalog.types);
    let child_id = store::content_id("catalog", canonical.as_bytes());
    let dialect = semio_framework_artifact_reference::ArtifactDialect { artifact_kind: "s.stdio.semio".into(), standard: "v1".into(), subset: "kit".into() };
    let target = semio_framework_artifact_reference::ArtifactRef { artifact_id: child_id.clone(), dialect };
    store::os_store::artifact_child::ArtifactChild::new(child_id, target)
}

/// 🏗️ Builds a `CurationSnapshot` from a full stock list, minting its content-addressed `catalog`
/// handle, splitting the stock into its composed-child half and sourcing-owned overflow half, and
/// seeding the working-scene cache so this SAME call's render/export/inference paths can resolve the
/// handle immediately. The one sanctioned construction path for "real stock content" — every fixture,
/// test, and command that used to write `CurationSnapshot { stock, .. }` directly goes through this now.
pub fn curation_snapshot_from_stock(stock: &[ObjectKind], curated: Vec<CuratedItem>) -> CurationSnapshot {
    let handle = catalog_child_handle(stock);
    CurationSnapshot { catalog: handle, stock_extra: stock_extra_from_stock(stock), curated }
}

/// 🌱 Seeds the working-scene cache for `stock`'s deterministic `catalog_child_handle`, without
/// building a whole `CurationSnapshot` — for fixture loaders (`default_document`/`empty_document`) that
/// parse the persisted snapshot from DSL text (which never embeds child content) but still need the
/// SAME content-addressed handle's catalog resolvable immediately after loading.
pub fn validate_catalog_payload(stock: &[ObjectKind]) {
    let _ = catalog_child_handle(stock);
}

/// 👁️ The one accessor every render/export/inference call site funnels through to read the full
/// reassembled stock catalogue from snapshot-owned overflow records.
pub fn stock_of(document: &CurationSnapshot) -> Vec<ObjectKind> {
    let _ = &document.catalog;
    document.stock_extra.iter().map(|row| ObjectKind { id: row.id.clone(), name: row.name.clone(), module_id: row.module_id.clone(), typology_path: row.typology_path.clone(), availability: row.availability, geometry: row.geometry.clone() }).collect()
}

/// 🪆️ The persisted slot name of the composed kit catalogue child — `CurationSnapshot::catalog`'s
/// `#[child]` field, as the framework's child projection names it.
pub const CATALOG_CHILD_SLOT: &str = "catalog";

/// 🌱️ `ArtifactApp::genesis_child_pack` for the catalogue: the kit half is a pure function of the
/// stock (`catalog_snapshot_from_stock`), so a whole-document load or a fresh boot re-derives it here
/// instead of shipping it. Answers only when the handle derived from the snapshot's own stock is the
/// handle the snapshot declares — a document whose catalog id disagrees with its stock is left for
/// closure validation to reject as `Incomplete`, never silently repaired.
pub fn genesis_catalog_pack(document: &CurationSnapshot, slot: &str, child_id: &str) -> Option<Vec<u8>> {
    if slot != CATALOG_CHILD_SLOT || child_id != document.catalog.child_id {
        return None;
    }
    let stock = stock_of(document);
    (catalog_child_handle(&stock).child_id == child_id).then(|| <SemioKitSnapshot as pack::snapshot::ArtifactPack>::encode_pack(&catalog_snapshot_from_stock(&stock)))
}

/// 🧬️ The bounded projection of the curation's composed `catalog` handle, as both surfaces hand it to
/// the framework's replacement pump (`ArtifactEditor::child_restore_projection` /
/// `ArtifactViewer::child_restore_projection`). The trait default grants no restore authority, so
/// without it every live envelope load faults `editor did not declare a loaded-parent child projection`.
pub fn curation_child_restore_projection(document: &CurationSnapshot) -> Result<semio_framework_composition_admission::ChildRestoreProjection<'_>, semio_framework_plugin::Fault> {
    semio_framework_composition_admission::ChildRestoreProjection::from_snapshot(document).map_err(|error| semio_framework_plugin::Fault::from(format!("sourcing curation child projection failed: {error}")))
}
//#endregion 🔖️CatalogComposition
}
//#endregion 🧩️ArtifactItems6
