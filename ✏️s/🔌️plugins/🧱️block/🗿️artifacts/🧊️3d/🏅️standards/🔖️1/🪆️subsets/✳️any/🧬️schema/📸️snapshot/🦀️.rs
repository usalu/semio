//! 🧬️ Block3d snapshot schema — artifact-lane fields only.

#![allow(async_fn_in_trait)]

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;

use semio_s_plugin_block::{BlockAttribute, BlockAuthor, BlockCamera3d, BlockCompatibilityRule, BlockKindIdentity, BlockMeta, BlockRepresentation};
use ::semio_framework_schema::ArtifactSchema;
use semio_s_artifact_stdio_semio_kit::schema::snapshot::SemioKitSnapshot;

//#region 🔖️Snapshot
/// 📸️ Persisted block3d document snapshot (persistent fields of the artifact).
#[derive(Clone, Debug, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_dsl_record_derive::DslRecord, ArtifactSchema)]
#[value(rename_all = "camelCase")]
#[dsl(id = "block.block3d", layout = "lines")]
#[artifact_schema(id = "s.block.block3d")]
pub struct Block3dSnapshot {
    #[state(artifact)]
    pub schema: String,
    #[dsl(block)]
    #[state(artifact)]
    pub object_kind: BlockKindIdentity,
    #[value(default)]
    #[dsl(table)]
    #[state(artifact)]
    pub representations: Vec<BlockRepresentation>,
    #[state(artifact)]
    #[child(kind = "s.stdio.semio")]
    pub catalog: store::os_store::artifact_child::ArtifactChild<SemioKitSnapshot>,
    #[value(default)]
    #[dsl(table)]
    #[state(artifact)]
    pub vortex_kind_extra: Vec<Block3dVortexKindExtra>,
    #[value(default)]
    #[dsl(table)]
    #[state(artifact)]
    pub vortices: Vec<Block3dVortexTemplate>,
    #[value(default)]
    #[dsl(table)]
    #[state(artifact)]
    pub compatibility: Vec<BlockCompatibilityRule>,
    #[value(default)]
    #[dsl(table)]
    #[state(artifact)]
    pub attributes: Vec<BlockAttribute>,
    #[value(default)]
    #[dsl(table)]
    #[state(artifact)]
    pub authors: Vec<BlockAuthor>,
    #[dsl(block)]
    #[value(default)]
    #[state(artifact)]
    pub camera3d: BlockCamera3d,
    #[dsl(block)]
    #[value(default)]
    #[state(artifact)]
    pub meta: BlockMeta,
}
//#endregion 🔖️Snapshot

impl Default for Block3dSnapshot {
    fn default() -> Self {
        Self {
            schema: BLOCK_3D_SCHEMA.to_string(),
            object_kind: BlockKindIdentity::default(),
            representations: Vec::new(),
            catalog: crate::catalog_child_handle(&[]),
            vortex_kind_extra: Vec::new(),
            vortices: Vec::new(),
            compatibility: Vec::new(),
            attributes: Vec::new(),
            authors: Vec::new(),
            camera3d: BlockCamera3d::default(),
            meta: BlockMeta::default(),
        }
    }
}

#[path = "🔢️transport/🦀️.rs"]
pub mod json_transport;

//#region 🍃️FacetLeaves
/// 🍃️ The five handcrafted leaves of the `snapshot` schema facet.
pub const SNAPSHOT_LEAVES: semio_framework_schema_registry::FacetLeaves = semio_framework_schema_registry::FacetLeaves { rust: include_str!("🦀️.rs"), typescript: include_str!("🟦️.ts"), graphql: include_str!("🔗️.graphql"), json_schema: include_str!("🔣️.json"), proto: include_str!("🛰️.proto") };
//#endregion 🍃️FacetLeaves

use semio_framework_plugin::ArtifactKindSpec;

use semio_framework_plugin::MediaClass;
use semio_framework_plugin::MediaForm;
use semio_framework_plugin::MediaType;
use semio_framework_plugin::OsMediaCapability;
use semio_s_artifact_stdio_semio_kit::schema::snapshot::SemioKitType;

pub const BLOCK_3D_SCHEMA: &str = "block.3d";

/// 🔘️ One vortex-kind catalog row this object kind ships with — the LOGICAL, in-memory shape every
/// app/command/test call site still uses. Persisted storage on `Block3dSnapshot` composes stdio's
/// `s.stdio.semio.kit` subset (`catalog`, the `id`/`name` half) plus `vortex_kind_extra` (the
/// `label`/`color`/`defaultCableKind` overflow that subset can't represent) — see
/// `🔖️VortexKindCatalogComposition` below for the split/join and `vortex_kinds_of`/`set_vortex_kinds`
/// for the one accessor pair every reader/writer funnels through.
#[derive(Clone, Debug, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[value(rename_all = "camelCase")]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
pub struct Block3dVortexKind {
    #[dsl(defines = "vortex_kind")]
    pub id: String,
    pub name: String,
    pub label: String,
    pub color: String,
    pub default_cable_kind: String,
}

/// 🧩️ Block3d-owned per-vortex-kind overflow NOT representable in stdio's composed `s.stdio.semio.kit`
/// subset (`SemioKitType` carries only `id`/`name`/`category`) — label, color, default cable kind.
/// Id-joined 1:1 to a `SemioKitType` in the composed `Block3dSnapshot::catalog` child by `id` (see
/// `vortex_kinds_of`/`vortex_kind_from_parts`). Ticket 26/08/12/UNIFIED-COMPOSABLE-ARTIFACT-SYSTEM:
/// replaces the former inline `vortex_kinds: Vec<Block3dVortexKind>` field, which duplicated the
/// `kit.catalog`/type-registry vocabulary this ticket composes instead.
#[derive(Clone, Debug, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[value(rename_all = "camelCase")]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
pub struct Block3dVortexKindExtra {
    #[dsl(defines = "vortex_kind")]
    pub id: String,
    #[value(default)]
    #[cfg_attr(test, serde(default))]
    pub name: String,
    pub label: String,
    pub color: String,
    pub default_cable_kind: String,
}

/// 🔀️ `Block3dVortexKind` → the shared `SemioKitType` half of the composed catalog child. `category`
/// is a fixed constant (`Block3dVortexKind` has no grouping concept of its own) — never round-tripped
/// through `vortex_kind_extra`, so this stays lossless without needing a shadow field.
pub fn kit_type_from_vortex_kind(kind: &Block3dVortexKind) -> SemioKitType {
    SemioKitType { id: kind.id.clone(), name: kind.name.clone(), category: "vortex-kind".into() }
}

/// 🔀️ `Block3dVortexKind` → the block3d-owned overflow half (`vortex_kind_extra`) the composed kit
/// type cannot carry. Lossless together with `kit_type_from_vortex_kind`: every `Block3dVortexKind`
/// field lands in exactly one of the two halves.
pub fn vortex_kind_extra_from_vortex_kind(kind: &Block3dVortexKind) -> Block3dVortexKindExtra {
    Block3dVortexKindExtra { id: kind.id.clone(), name: kind.name.clone(), label: kind.label.clone(), color: kind.color.clone(), default_cable_kind: kind.default_cable_kind.clone() }
}

/// 🔀️ The full vortex-kinds list's shared half, as a fresh (design-less, link-less) `SemioKitSnapshot`
/// — content-addressed by `catalog_child_handle` below, never embedded inline in `Block3dSnapshot`.
pub fn catalog_snapshot_from_vortex_kinds(kinds: &[Block3dVortexKind]) -> SemioKitSnapshot {
    SemioKitSnapshot { types: kinds.iter().map(kit_type_from_vortex_kind).collect(), ..SemioKitSnapshot::default() }
}

/// 🔀️ The full vortex-kinds list's block3d-owned overflow half.
pub fn vortex_kind_extra_list_from_vortex_kinds(kinds: &[Block3dVortexKind]) -> Vec<Block3dVortexKindExtra> {
    kinds.iter().map(vortex_kind_extra_from_vortex_kind).collect()
}

/// 🪪️ Content-addressed child handle for a vortex-kinds list's shared catalog half — hashes the
/// deterministic JSON of the derived `SemioKitType` list so peers replaying the same vortex-kinds
/// converge on the same `child_id` (never a random/incrementing id), mirroring `sourcing`'s
/// `catalog_child_handle`.
pub fn catalog_child_handle(kinds: &[Block3dVortexKind]) -> store::os_store::artifact_child::ArtifactChild<SemioKitSnapshot> {
    let catalog = catalog_snapshot_from_vortex_kinds(kinds);
    let canonical = semio_framework_pack_json::to_json_string(&catalog.types);
    let child_id = store::content_id("catalog", canonical.as_bytes());
    let dialect = semio_framework_artifact_reference::ArtifactDialect { artifact_kind: "s.stdio.semio".into(), standard: "v1".into(), subset: "kit".into() };
    let target = semio_framework_artifact_reference::ArtifactRef { artifact_id: child_id.clone(), dialect };
    store::os_store::artifact_child::ArtifactChild::new(child_id, target)
}

/// 👁️ The one accessor every render/export/inference/mutation-diff call site funnels through to read
/// the full reassembled vortex-kinds catalogue, given the composed child handle and the overflow list
/// directly (works for both `Block3dSnapshot` and `Block3dArtifact`, which mirror these two fields).
pub fn vortex_kinds_of_parts(catalog: &store::os_store::artifact_child::ArtifactChild<SemioKitSnapshot>, extra: &[Block3dVortexKindExtra]) -> Vec<Block3dVortexKind> {
    let _ = catalog;
    extra.iter().map(|row| Block3dVortexKind { id: row.id.clone(), name: row.name.clone(), label: row.label.clone(), color: row.color.clone(), default_cable_kind: row.default_cable_kind.clone() }).collect()
}

/// 👁️ `vortex_kinds_of_parts` specialized to `Block3dSnapshot`.
pub fn vortex_kinds_of(snapshot: &Block3dSnapshot) -> Vec<Block3dVortexKind> {
    vortex_kinds_of_parts(&snapshot.catalog, &snapshot.vortex_kind_extra)
}

/// 🎯️ Padding the world `fit` lane frames a document with — a quarter of the bounding sphere's
/// radius of air around it, the render host's own default (twin of puzzle3d's `PUZZLE3D_FIT_PADDING`).
pub const BLOCK3D_FIT_PADDING: f64 = 1.25;

/// 🎯️ Document IDENTITY for the world `fit` lane: WHICH object kind this document is and WHICH
/// representation meshes it delivers — never where its vortices currently sit. `WorldAutoFit` refits
/// once per revision, so this is the difference between "frame the example that was just loaded" and
/// "yank the camera on every vortex drag".
///
/// Without the lane a world window is framed solely by the document's authored `camera3d`, and a
/// document whose representation mesh lies outside that camera's frustum renders a viewport with no
/// visible geometry at all — measured on the curated `hexagonal-cut-concrete-forest-left` example
/// (its camera is authored around the 0.36 m rim ring at the origin, while the representation GLB is
/// an ~11 m × 4.7 m slab reaching away from it).
///
/// The bounds themselves stay unpublished (`world3d_fit_json(..., None)`, as puzzle3d does): a
/// representation is a mesh URL, so this crate knows what it asked for but never how big the
/// delivered geometry is — only the render host, which loaded it, can measure that.
pub fn block3d_world_fit_revision(snapshot: &Block3dSnapshot) -> u32 {
    use std::hash::{Hash, Hasher};
    let mut hasher = std::collections::hash_map::DefaultHasher::new();
    snapshot.object_kind.id.hash(&mut hasher);
    for representation in &snapshot.representations {
        representation.id.hash(&mut hasher);
        representation.mesh_url.hash(&mut hasher);
    }
    (hasher.finish() >> 32) as u32
}

/// ✍️ The one writer every mutation-diff-apply call site funnels through to replace the full vortex-
/// kinds catalogue: mints a fresh content-addressed `catalog` handle, seeds the working-scene cache,
/// and writes the overflow half — given the composed child handle and overflow list directly (works
/// for both `Block3dSnapshot` and `Block3dArtifact`).
pub fn set_vortex_kinds_parts(catalog: &mut store::os_store::artifact_child::ArtifactChild<SemioKitSnapshot>, extra: &mut Vec<Block3dVortexKindExtra>, kinds: &[Block3dVortexKind]) {
    let handle = catalog_child_handle(kinds);
    *catalog = handle;
    *extra = vortex_kind_extra_list_from_vortex_kinds(kinds);
}

/// ✍️ `set_vortex_kinds_parts` specialized to `Block3dSnapshot`.
pub fn set_vortex_kinds(snapshot: &mut Block3dSnapshot, kinds: &[Block3dVortexKind]) {
    set_vortex_kinds_parts(&mut snapshot.catalog, &mut snapshot.vortex_kind_extra, kinds);
}

/// 🌱 Seeds the working-scene cache for `kinds`'s deterministic `catalog_child_handle`, without
/// writing any snapshot fields — for fixture loaders that parse the persisted snapshot from DSL text
/// (which never embeds child content) but still need the SAME content-addressed handle's catalog
/// resolvable immediately after loading.
pub fn validate_vortex_kind_catalog(kinds: &[Block3dVortexKind]) {
    let _ = catalog_child_handle(kinds);
}

/// 🌱️ One rim-vortex template — where a vortex of `vortex_kind` sits on the object's surface.
#[derive(Clone, Debug, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[value(rename_all = "camelCase")]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
pub struct Block3dVortexTemplate {
    pub id: String,
    #[dsl(refs = "vortex_kind")]
    pub vortex_kind: String,
    #[value(default)]
    #[cfg_attr(test, serde(default))]
    #[dsl(coord)]
    pub position: [f64; 3],
    #[value(default)]
    #[cfg_attr(test, serde(default))]
    #[dsl(dir)]
    pub direction: [f64; 3],
    #[value(default)]
    #[cfg_attr(test, serde(default))]
    pub radius: f64,
    #[value(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(test, serde(default, skip_serializing_if = "Option::is_none"))]
    pub label: Option<String>,
}

/// 🪟 Per-window-instance view state for representation subset and layout.
#[derive(Clone, Debug, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[value(rename_all = "camelCase")]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
pub struct Block3dWindowView {
    pub window_id: String,
    #[value(default)]
    #[cfg_attr(test, serde(default))]
    pub representation_ids: Vec<String>,
    #[value(default = "default_arrangement")]
    #[cfg_attr(test, serde(default = "default_arrangement"))]
    pub arrangement: String,
    #[value(default = "default_spacing")]
    #[cfg_attr(test, serde(default = "default_spacing"))]
    pub spacing: f64,
}

impl Block3dWindowView {
    /// 🪟 Builds a default view record for one window id.
    pub fn for_window(window_id: impl Into<String>) -> Self {
        Self { window_id: window_id.into(), representation_ids: Vec::new(), arrangement: default_arrangement(), spacing: default_spacing() }
    }
}

/// 🗂️ This artifact's `ArtifactKindSpec` — the canonical `3d.block` declaration, stitched into
/// `crate::editor::block3d::create_block3d_app`.
pub fn artifact_kind() -> ArtifactKindSpec {
    ArtifactKindSpec {
        id: "3d.block".into(),
        label: semio_framework_ui_locale::LocalizedLabel::native("Object Kind", "Objekttyp"),
        source_format: BLOCK_3D_SCHEMA.into(),
        component_kind: "block3d".into(),
        dimension: "3d".into(),
        media_capability: OsMediaCapability::MeshOnly,
        media_type: MediaType { class: MediaClass::Kit, form: MediaForm::Type },
        schema: BLOCK_3D_SCHEMA.into(),
        export_formats: vec![],
        import_formats: vec![],
        export_stdio_kinds: vec!["stdio.json".into(), "stdio.txt".into(), "stdio.zip".into()],
        import_stdio_kinds: vec!["stdio.json".into(), "stdio.txt".into(), "stdio.zip".into()],
    }
}

/// 🎯️ Fully-qualified dialect coordinate for `s.block.block3d@1/*` (ticket
/// 26/08/16/ARTIFACT-VIEWERS-AND-EDITORS-PER-SUBSET contract §2.1/§2.4) — lives at the ARTIFACT level
/// (not under `editor`/`viewer`) specifically so a viewer file can read it without ever importing
/// through the sibling editor module. `artifact_kind` matches the literal 3rd-column descriptor of
/// this file's own `("s.block3d.schema.artifact", "schema", "s.block.block3d", …)` row in
/// `definition()` above; `standard`/`subset` match this file's own
/// `🏅️standards/🔖️1/🪆️subsets/✳️any` location — i.e. the canonical surface id is
/// `s.block.block3d@1/*#editor` / `s.block.block3d@1/*#viewer`, exactly the contract §1 grammar.
pub const BLOCK3D_DIALECT: semio_framework_plugin::app::Dialect = semio_framework_plugin::app::Dialect { artifact_kind: "s.block.block3d", standard: semio_framework_plugin::app::StandardId("1"), subset: semio_framework_plugin::app::SubsetId::ANY };

extern crate self as semio_s_artifact_block_3d_model;
extern crate semio_framework_pack as pack;
#[path = "../../🚪️io/💾️binary/📸️snapshot/🦀️.rs"]
pub mod binary_snapshot;
#[path = "../../🚪️io/📝️text/📸️snapshot/🦀️.rs"]
pub mod text_snapshot;
#[path = "../../🚪️io/🪶️sqlite/📸️snapshot/🦀️.rs"]
pub mod sqlite_snapshot;
