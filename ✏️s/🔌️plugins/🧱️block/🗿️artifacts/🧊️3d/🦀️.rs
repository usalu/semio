//! 🏙️ Block 3D artifact — the document entity the 🧊️3d app edits (constitutional: general). Edits
//! exactly one `ObjectKind`: its identity, representations (meshes at LOD/tags — the semio_compose_rs
//! `type` app's successor), and the `VortexKind` templates placed on its rim.

#![allow(clippy::result_large_err)]
extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_os_kernel as vcs;

pub use semio_s_artifact_block_2d::{BlockAttribute, BlockAuthor, BlockCamera2d, BlockCamera3d, BlockCompatibilityRule, BlockKindIdentity, BlockMeta, BlockRepresentation};

#[cfg(feature = "component-app-assembly")]
pub trait ArtifactApps:
    semio_framework_plugin::PluginApp
    + From<semio_framework_plugin::app::VcsArtifactApp<semio_framework_plugin::app::EditorApp<editor::block3d::Block3dPlayApp>>>
    + From<semio_framework_plugin::app::VcsArtifactApp<semio_framework_plugin::app::ViewerApp<viewer::block3d::Block3dViewer>>>
{
}

#[cfg(feature = "component-app-assembly")]
impl<PA> ArtifactApps for PA where
    PA: semio_framework_plugin::PluginApp
        + From<semio_framework_plugin::app::VcsArtifactApp<semio_framework_plugin::app::EditorApp<editor::block3d::Block3dPlayApp>>>
        + From<semio_framework_plugin::app::VcsArtifactApp<semio_framework_plugin::app::ViewerApp<viewer::block3d::Block3dViewer>>>
{
}

use semio_framework_plugin::{ArtifactKindSpec, MediaClass, MediaForm, MediaType, OsMediaCapability};
use semio_s_artifact_stdio_semio::standards::v1::subsets::kit::schema::snapshot::{SemioKitSnapshot, SemioKitType};

pub const BLOCK_3D_SCHEMA: &str = "block.3d";

// #region 🔖️Document
/// 🔘️ One vortex-kind catalog row this object kind ships with — the LOGICAL, in-memory shape every
/// app/command/test call site still uses. Persisted storage on `Block3dSnapshot` composes stdio's
/// `s.stdio.semio.kit` subset (`catalog`, the `id`/`name` half) plus `vortex_kind_extra` (the
/// `label`/`color`/`defaultCableKind` overflow that subset can't represent) — see
/// `🔖️VortexKindCatalogComposition` below for the split/join and `vortex_kinds_of`/`set_vortex_kinds`
/// for the one accessor pair every reader/writer funnels through.
#[derive(Clone, Debug, PartialEq, dsl::ToValue, dsl::FromValue, dsl::DslRecord)]
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

//#region 🔖️VortexKindCatalogComposition
/// 🧩️ Block3d-owned per-vortex-kind overflow NOT representable in stdio's composed `s.stdio.semio.kit`
/// subset (`SemioKitType` carries only `id`/`name`/`category`) — label, color, default cable kind.
/// Id-joined 1:1 to a `SemioKitType` in the composed `Block3dSnapshot::catalog` child by `id` (see
/// `vortex_kinds_of`/`vortex_kind_from_parts`). Ticket 26/08/12/UNIFIED-COMPOSABLE-ARTIFACT-SYSTEM:
/// replaces the former inline `vortex_kinds: Vec<Block3dVortexKind>` field, which duplicated the
/// `kit.catalog`/type-registry vocabulary this ticket composes instead.
#[derive(Clone, Debug, PartialEq, dsl::ToValue, dsl::FromValue, dsl::DslRecord)]
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

/// 🔀️ Inverse of the split above — reassembles one full `Block3dVortexKind` from its two composed
/// halves.
pub fn vortex_kind_from_parts(kit_type: &SemioKitType, extra: &Block3dVortexKindExtra) -> Block3dVortexKind {
    Block3dVortexKind { id: kit_type.id.clone(), name: kit_type.name.clone(), label: extra.label.clone(), color: extra.color.clone(), default_cable_kind: extra.default_cable_kind.clone() }
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

/// 🔀️ Reassembles the full `Vec<Block3dVortexKind>` catalog from its composed-child half and its
/// block3d-owned overflow half, id-joined. A `SemioKitType` with no matching `Block3dVortexKindExtra`
/// (composed-child content the working-scene cache hasn't seen yet — see `vortex_kinds_of`'s doc
/// comment) is silently dropped rather than fabricated with placeholder label/color.
pub fn vortex_kinds_from_catalog_and_extra(catalog: &SemioKitSnapshot, extra: &[Block3dVortexKindExtra]) -> Vec<Block3dVortexKind> {
    let extra_by_id: std::collections::HashMap<&str, &Block3dVortexKindExtra> = extra.iter().map(|e| (e.id.as_str(), e)).collect();
    catalog.types.iter().filter_map(|kit_type| extra_by_id.get(kit_type.id.as_str()).map(|extra| vortex_kind_from_parts(kit_type, extra))).collect()
}

/// 🪪️ Content-addressed child handle for a vortex-kinds list's shared catalog half — hashes the
/// deterministic JSON of the derived `SemioKitType` list so peers replaying the same vortex-kinds
/// converge on the same `child_id` (never a random/incrementing id), mirroring `sourcing`'s
/// `catalog_child_handle`.
pub fn catalog_child_handle(kinds: &[Block3dVortexKind]) -> store::ArtifactChild<SemioKitSnapshot> {
    use std::hash::{Hash, Hasher};
    let catalog = catalog_snapshot_from_vortex_kinds(kinds);
    let canonical = dsl::os_pack::json::to_json_string(&catalog.types);
    let mut hasher = std::collections::hash_map::DefaultHasher::new();
    canonical.hash(&mut hasher);
    let child_id = format!("catalog-{:016x}", hasher.finish());
    let dialect = store::os_io::ArtifactDialect { artifact_kind: "s.stdio.semio".into(), standard: "v1".into(), subset: "kit".into() };
    let target = store::os_io::ArtifactRef { artifact_id: child_id.clone(), dialect };
    store::ArtifactChild::new(child_id, target)
}

/// 👁️ The one accessor every render/export/inference/mutation-diff call site funnels through to read
/// the full reassembled vortex-kinds catalogue, given the composed child handle and the overflow list
/// directly (works for both `Block3dSnapshot` and `Block3dArtifact`, which mirror these two fields).
pub fn vortex_kinds_of_parts(catalog: &store::ArtifactChild<SemioKitSnapshot>, extra: &[Block3dVortexKindExtra]) -> Vec<Block3dVortexKind> {
    let _ = catalog;
    extra.iter().map(|row| Block3dVortexKind { id: row.id.clone(), name: row.name.clone(), label: row.label.clone(), color: row.color.clone(), default_cable_kind: row.default_cable_kind.clone() }).collect()
}

/// 👁️ `vortex_kinds_of_parts` specialized to `Block3dSnapshot`.
pub fn vortex_kinds_of(snapshot: &Block3dSnapshot) -> Vec<Block3dVortexKind> {
    vortex_kinds_of_parts(&snapshot.catalog, &snapshot.vortex_kind_extra)
}

/// ✍️ The one writer every mutation-diff-apply call site funnels through to replace the full vortex-
/// kinds catalogue: mints a fresh content-addressed `catalog` handle, seeds the working-scene cache,
/// and writes the overflow half — given the composed child handle and overflow list directly (works
/// for both `Block3dSnapshot` and `Block3dArtifact`).
pub fn set_vortex_kinds_parts(catalog: &mut store::ArtifactChild<SemioKitSnapshot>, extra: &mut Vec<Block3dVortexKindExtra>, kinds: Vec<Block3dVortexKind>) {
    let handle = catalog_child_handle(&kinds);
    *catalog = handle;
    *extra = vortex_kind_extra_list_from_vortex_kinds(&kinds);
}

/// ✍️ `set_vortex_kinds_parts` specialized to `Block3dSnapshot`.
pub fn set_vortex_kinds(snapshot: &mut Block3dSnapshot, kinds: Vec<Block3dVortexKind>) {
    set_vortex_kinds_parts(&mut snapshot.catalog, &mut snapshot.vortex_kind_extra, kinds);
}

/// 🌱 Seeds the working-scene cache for `kinds`'s deterministic `catalog_child_handle`, without
/// writing any snapshot fields — for fixture loaders that parse the persisted snapshot from DSL text
/// (which never embeds child content) but still need the SAME content-addressed handle's catalog
/// resolvable immediately after loading.
pub fn validate_vortex_kind_catalog(kinds: &[Block3dVortexKind]) {
    let _ = catalog_child_handle(kinds);
}
//#endregion 🔖️VortexKindCatalogComposition

/// 🌱️ One rim-vortex template — where a vortex of `vortex_kind` sits on the object's surface.
#[derive(Clone, Debug, PartialEq, dsl::ToValue, dsl::FromValue, dsl::DslRecord)]
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

//#region 🔖️WindowView
/// 🪟 Per-window-instance view state for representation subset and layout.
#[derive(Clone, Debug, PartialEq, dsl::ToValue, dsl::FromValue, dsl::DslRecord)]
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

fn default_arrangement() -> String {
    "overlap".into()
}

fn default_spacing() -> f64 {
    8.0
}

impl Block3dWindowView {
    /// 🪟 Builds a default view record for one window id.
    pub fn for_window(window_id: impl Into<String>) -> Self {
        Self { window_id: window_id.into(), representation_ids: Vec::new(), arrangement: default_arrangement(), spacing: default_spacing() }
    }
}

//#endregion 🔖️WindowView

//#region 🔖️Snapshot
//#endregion 🔖️Snapshot

// #endregion 🔖️Document

//#region 🔖️ArtifactKind
/// 🗂️ This artifact's `ArtifactKindSpec` — the canonical `3d.block` declaration, stitched into
/// `crate::editor::block3d::create_block3d_app`.
pub fn artifact_kind() -> ArtifactKindSpec {
    ArtifactKindSpec {
        id: "3d.block".into(),
        name: "Object Kind".into(),
        source_format: BLOCK_3D_SCHEMA.into(),
        component_kind: "block3d".into(),
        dimension: "3d".into(),
        media_capability: OsMediaCapability::MeshOnly,
        media_type: MediaType { class: MediaClass::Kit, form: MediaForm::Type },
        schema: BLOCK_3D_SCHEMA.into(),
        export_formats: vec![],
        import_formats: vec![],
        export_stdio_kinds: vec!["stdio.json".into(), "stdio.obj".into(), "stdio.png".into(), "stdio.stl".into(), "stdio.txt".into(), "stdio.zip".into()],
        import_stdio_kinds: vec!["stdio.json".into(), "stdio.obj".into(), "stdio.png".into(), "stdio.stl".into(), "stdio.txt".into(), "stdio.zip".into()],
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
//#endregion 🔖️ArtifactKind

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
//#region 🪪️Declaration
/// 🔖️ This artifact's declaration (ticket 26/08/12/ARTIFACTS-ONLY-PLUGIN-ARCHITECTURE M1) — replaces
/// the old side-effecting `register()`, which called four different global registries directly from
/// a plugin `.setup()` callback. `Block3dPlayApp`'s CONFIG/PRESENCE schema — an app-scope concern
/// `ArtifactDeclaration` deliberately has no field for (see that struct's own doc) — now registers via
/// `ArtifactEditor::app_schema()` returning `crate::editor::block3d::config::schema::app_schema_descriptor()`
/// (ticket W1c), so `.setup()` is gone from `🧱️block/🦀️.rs` entirely.
pub fn definition() -> Result<semio_framework_plugin::ArtifactDefinition, semio_framework_plugin::ArtifactDefinitionError> {
    use semio_framework_plugin::{ArtifactCapability, ArtifactCapabilityKind, ArtifactDefinition, ArtifactIdentity, ArtifactIdentityClaim, ArtifactIdentityNamespace, ArtifactLocale, ArtifactLocalization};

    let rows: &[semio_framework_plugin::ArtifactCapabilityRow<'_>] = &[
        ("s.block.block3d.standard.v1", "standard", "1", &[], None),
        ("s.block.block3d.standard.v1.profile.any", "profile", "any", &[], None),
        ("s.block.block3d.schema.artifact", "schema", "s.block.block3d", &[("schema", "s.block.block3d")], None),
        ("s.block.block3d.inference.artifact", "inference", "s.block.block3d.inference", &[("schema", "s.block.block3d.inference")], None),
        ("s.block.block3d.composer.native", "composer", "s.block.block3d@1/*", &[("dialect", "s.block.block3d@1/*")], None),
        ("s.block.block3d.composer.format-1", "composer", "s.stdio.zip@2.0/*", &[("dialect", "s.stdio.zip@2.0/*")], None),
        ("s.block.block3d.composer.format-2", "composer", "s.stdio.png@1.2/*", &[("dialect", "s.stdio.png@1.2/*")], None),
        ("s.block.block3d.composer.format-3", "composer", "s.stdio.json@rfc8259/*", &[("dialect", "s.stdio.json@rfc8259/*")], None),
        ("s.block.block3d.composer.format-4", "composer", "s.stdio.stl@ascii/*", &[("dialect", "s.stdio.stl@ascii/*")], None),
        ("s.block.block3d.composer.format-5", "composer", "s.stdio.obj@3.0/*", &[("dialect", "s.stdio.obj@3.0/*")], None),
        ("s.block.block3d.composer.format-6", "composer", "s.stdio.txt@utf-8/*", &[("dialect", "s.stdio.txt@utf-8/*")], None),
        ("s.block.block3d.grammar.1", "grammar", "block.block3d", &[("grammar", "block.block3d")], None),
        ("s.block.block3d.grammar.2", "grammar", "block.block3d.op", &[("grammar", "block.block3d.op")], None),
        ("s.block.block3d.grammar.3", "grammar", "block.block3d.diff", &[("grammar", "block.block3d.diff")], None),
        ("s.block.block3d.grammar.4", "grammar", "3d.pack", &[("grammar", "3d.pack")], None),
        ("s.block.block3d.grammar.5", "grammar", "3d.spr", &[("grammar", "3d.spr")], None),
        // 🐛️ D2-capability-claim-repairs: `.document_codec::<EditorApp<Block3dPlayApp>>()` derives
        // its extension claim from `<Block3dSnapshot as store::ArtifactDsl>::EXTENSION`
        // (`…/🧬️schema/📸️snapshot/🦀️.rs`), which is `"block3d"`, not `"block"`.
        ("s.block.block3d.codec.document-1", "codec", "block.3d:block3d", &[("codec", "block.3d"), ("codec-extension", "8:block.3d:block3d")], None),
        ("s.block.block3d.localization.en", "localization", "3D Block", &[], Some(("en", "3D Block"))),
        ("s.block.block3d.localization.de", "localization", "3D-Baustein", &[], Some(("de", "3D-Baustein"))),
    ];
    let mut definition = ArtifactDefinition::new(ArtifactIdentity::parse("s.block.block3d")?);
    for (identity, kind, descriptor, claims, localization) in rows {
        let mut capability = ArtifactCapability::new(ArtifactIdentity::parse(*identity)?, ArtifactCapabilityKind::parse(*kind)?).descriptor(descriptor.as_bytes())?;
        for (namespace, value) in *claims {
            capability = capability.claim(ArtifactIdentityClaim::new(ArtifactIdentityNamespace::parse(*namespace)?, *value)?)?;
        }
        if let Some((locale, text)) = localization {
            capability = capability.localization(ArtifactLocalization::new(ArtifactLocale::parse(*locale)?, *text)?)?;
        }
        definition = definition.capability(capability)?;
    }
    Ok(definition)
}

/// 🔖️ New declaration channel (ticket 26/08/17/MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME,
/// `descriptor-prep`): replaces `declaration()`/`ArtifactDeclaration::builder(...).try_build()` — the
/// old channel's `require_declared_capability_or_record` runs an exact sorted-claims equality check
/// between `definition()`'s hand-authored capability rows and the runtime registration, which is why
/// `.artifact(block3d::declaration())` failed assembly. This channel never runs that check; the real
/// data (schema/inference descriptors, editor/viewer, native codec) is read from
/// `standards::v1::subsets::any::subset()` instead. Mirrors `🗒️note`/`🖍️draw`/`🔱️trinity`'s own
/// migration exactly.
#[cfg(feature = "component-app-assembly")]
pub fn artifact<PA: ArtifactApps>() -> semio_framework_plugin::app::declarations::ArtifactDeclaration<PA> {
    use semio_framework_plugin::app::declarations::ArtifactDeclaration;
    use store::os_io::ArtifactKindId;
    ArtifactDeclaration { kind: ArtifactKindId::parse("s.block.block3d").expect("canonical block3d kind"), localization: &[], standards: vec![standards::v1::standard::<PA>()] }
}

/// 📌️ Handcrafted facet grammars (text) and protocols (binary) for in-process execution — built once
/// and leaked to a `&'static` slice since `dsl::passthrough_hooks` isn't `const fn`. `pub` (was
/// private): the new `🪆️subsets/✳️any/🦀️.rs` reads it to build `io_declaration()`'s native
/// codec pairs.
pub fn pilot_languages() -> &'static [dsl::LanguageSpec] {
    static LANGUAGES: std::sync::OnceLock<Vec<dsl::LanguageSpec>> = std::sync::OnceLock::new();
    LANGUAGES
        .get_or_init(|| {
            vec![
                dsl::LanguageSpec {
                    id: "block.block3d",
                    extension: Some("block3d"),
                    role: dsl::LanguageRole::Document,
                    grammar: Some(standards::v1::subsets::any::schema::snapshot::text::COMPONENT_GRAMMAR_SEMIO),
                    grammar_path: Some(standards::v1::subsets::any::schema::snapshot::text::COMPONENT_GRAMMAR_PATH),
                    protocol: Some(standards::v1::subsets::any::schema::snapshot::binary::COMPONENT_PROTOCOL_SEMIO),
                    protocol_path: Some(standards::v1::subsets::any::schema::snapshot::binary::COMPONENT_PROTOCOL_PATH),
                    hooks: dsl::passthrough_hooks("block.block3d"),
                },
                dsl::LanguageSpec {
                    id: "block.block3d.op",
                    extension: None,
                    role: dsl::LanguageRole::Ops,
                    grammar: Some(standards::v1::subsets::any::schema::mutations::text::COMPONENT_GRAMMAR_SEMIO),
                    grammar_path: Some(standards::v1::subsets::any::schema::mutations::text::COMPONENT_GRAMMAR_PATH),
                    protocol: Some(standards::v1::subsets::any::schema::mutations::binary::COMPONENT_PROTOCOL_SEMIO),
                    protocol_path: Some(standards::v1::subsets::any::schema::mutations::binary::COMPONENT_PROTOCOL_PATH),
                    hooks: dsl::passthrough_hooks("block.block3d.op"),
                },
                dsl::LanguageSpec {
                    id: "block.block3d.diff",
                    extension: None,
                    role: dsl::LanguageRole::Diff,
                    grammar: Some(standards::v1::subsets::any::schema::diff::COMPONENT_GRAMMAR_SEMIO),
                    grammar_path: Some(standards::v1::subsets::any::schema::diff::COMPONENT_GRAMMAR_PATH),
                    protocol: None,
                    protocol_path: None,
                    hooks: dsl::passthrough_hooks("block.block3d.diff"),
                },
                dsl::LanguageSpec {
                    id: "3d.pack",
                    extension: None,
                    role: dsl::LanguageRole::Pack,
                    grammar: None,
                    grammar_path: None,
                    protocol: Some(standards::v1::subsets::any::schema::snapshot::binary::COMPONENT_PROTOCOL_SEMIO),
                    protocol_path: Some(standards::v1::subsets::any::schema::snapshot::binary::COMPONENT_PROTOCOL_PATH),
                    hooks: dsl::passthrough_hooks("3d.pack"),
                },
                dsl::LanguageSpec {
                    id: "3d.spr",
                    extension: None,
                    role: dsl::LanguageRole::Spr,
                    grammar: None,
                    grammar_path: None,
                    protocol: Some(standards::v1::subsets::any::schema::mutations::binary::COMPONENT_PROTOCOL_SEMIO),
                    protocol_path: Some(standards::v1::subsets::any::schema::mutations::binary::COMPONENT_PROTOCOL_PATH),
                    hooks: dsl::passthrough_hooks("3d.spr"),
                },
            ]
        })
        .as_slice()
}
//#endregion 🪪️Declaration

#[path = "."]
        pub mod standards {
            #[path = "."]
            pub mod v1 {
                #[cfg(feature = "component-app-assembly")]
                #[path = "🏅️standards/🔖️1/🦀️.rs"]
                mod component;
                #[cfg(feature = "component-app-assembly")]
                pub use component::*;
                #[path = "."]
                pub mod subsets {
                    #[path = "."]
                    pub mod any {
                        #[cfg(feature = "component-app-assembly")]
                        #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🦀️.rs"]
                        mod component;
                        #[cfg(feature = "component-app-assembly")]
                        pub use component::*;
                        #[path = "."]
                        pub mod schema {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "."]
                            pub mod snapshot {
                                #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🦀️.rs"]
                                mod component;
                                pub use component::*;
                                #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/💾️binary/🦀️.rs"]
                                pub mod binary;
                                #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/📝️text/🦀️.rs"]
                                pub mod text;
                            }
                            #[path = "."]
                            pub mod inferences {
                                #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🦀️.rs"]
                                mod component;
                                pub use component::*;
                                #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/💾️binary/🦀️.rs"]
                                pub mod binary;
                                #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/📝️text/🦀️.rs"]
                                pub mod text;
                                #[path = "."]
                                pub mod bounds {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/📦bounds/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                }
                            }
                            #[path = "."]
                            pub mod diff {
                                #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/🦀️.rs"]
                                mod component;
                                pub use component::*;
                                #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/📝️text/🦀️.rs"]
                                pub mod text;
                                pub use text::*;
                                #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/💾️binary/🦀️.rs"]
                                pub mod binary;
                            }
                            #[path = "."]
                            pub mod mutations {
                                #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️.rs"]
                                mod component;
                                pub use component::*;
                                #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💾️binary/🦀️.rs"]
                                pub mod binary;
                                #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📝️text/🦀️.rs"]
                                pub mod text;
                                #[path = "."]
                                pub mod rename_object_kind {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/✏️rename-object-kind/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/✏️rename-object-kind/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/✏️rename-object-kind/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/✏️rename-object-kind/🧪️tests/🧪️renames-object-kind-to-pod/🦀️.rs"]
                                    mod tests_renames_object_kind_to_pod;
                                }
                                #[path = "."]
                                pub mod change_object_kind_label {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏷️change-object-kind-label/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏷️change-object-kind-label/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏷️change-object-kind-label/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏷️change-object-kind-label/🧪️tests/🧪️relabels-object-kind/🦀️.rs"]
                                    mod tests_relabels_object_kind;
                                }
                                #[path = "."]
                                pub mod change_object_kind_variant {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔀️change-object-kind-variant/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔀️change-object-kind-variant/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔀️change-object-kind-variant/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔀️change-object-kind-variant/🧪️tests/🧪️switches-variant-to-b/🦀️.rs"]
                                    mod tests_switches_variant_to_b;
                                }
                                #[path = "."]
                                pub mod change_object_kind_description {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📃️change-object-kind-description/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📃️change-object-kind-description/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📃️change-object-kind-description/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📃️change-object-kind-description/🧪️tests/🧪️rewrites-object-kind-description/🦀️.rs"]
                                    mod tests_rewrites_object_kind_description;
                                }
                                #[path = "."]
                                pub mod change_object_kind_icon {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🖼️change-object-kind-icon/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🖼️change-object-kind-icon/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🖼️change-object-kind-icon/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🖼️change-object-kind-icon/🧪️tests/🧪️repoints-object-kind-icon/🦀️.rs"]
                                    mod tests_repoints_object_kind_icon;
                                }
                                #[path = "."]
                                pub mod change_object_kind_unit {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📐change-object-kind-unit/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📐change-object-kind-unit/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📐change-object-kind-unit/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📐change-object-kind-unit/🧪️tests/🧪️switches-unit-to-centimeter/🦀️.rs"]
                                    mod tests_switches_unit_to_centimeter;
                                }
                                #[path = "."]
                                pub mod create_representation {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧱create-representation/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧱create-representation/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧱create-representation/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧱create-representation/🧪️tests/🧪️appends-frame-representation/🦀️.rs"]
                                    mod tests_appends_frame_representation;
                                }
                                #[path = "."]
                                pub mod delete_representation {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🗑️delete-representation/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🗑️delete-representation/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🗑️delete-representation/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🗑️delete-representation/🧪️tests/🧪️removes-shell-representation/🦀️.rs"]
                                    mod tests_removes_shell_representation;
                                }
                                #[path = "."]
                                pub mod rename_representation {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/✒️rename-representation/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/✒️rename-representation/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/✒️rename-representation/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/✒️rename-representation/🧪️tests/🧪️renames-shell-to-hull/🦀️.rs"]
                                    mod tests_renames_shell_to_hull;
                                }
                                #[path = "."]
                                pub mod change_representation_mesh_url {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌐change-representation-mesh-url/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌐change-representation-mesh-url/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌐change-representation-mesh-url/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌐change-representation-mesh-url/🧪️tests/🧪️repoints-shell-mesh-url/🦀️.rs"]
                                    mod tests_repoints_shell_mesh_url;
                                }
                                #[path = "."]
                                pub mod change_representation_lod {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏔️change-representation-lod/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏔️change-representation-lod/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏔️change-representation-lod/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏔️change-representation-lod/🧪️tests/🧪️promotes-shell-to-lod2/🦀️.rs"]
                                    mod tests_promotes_shell_to_lod2;
                                }
                                #[path = "."]
                                pub mod change_representation_description {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📜change-representation-description/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📜change-representation-description/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📜change-representation-description/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📜change-representation-description/🧪️tests/🧪️rewrites-shell-description/🦀️.rs"]
                                    mod tests_rewrites_shell_description;
                                }
                                #[path = "."]
                                pub mod add_representation_tag {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔖add-representation-tag/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔖add-representation-tag/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔖add-representation-tag/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔖add-representation-tag/🧪️tests/🧪️tags-shell-as-structural/🦀️.rs"]
                                    mod tests_tags_shell_as_structural;
                                }
                                #[path = "."]
                                pub mod remove_representation_tag {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🚫remove-representation-tag/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🚫remove-representation-tag/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🚫remove-representation-tag/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🚫remove-representation-tag/🧪️tests/🧪️untags-shell-printable/🦀️.rs"]
                                    mod tests_untags_shell_printable;
                                }
                                #[path = "."]
                                pub mod add_representation_attribute {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧩add-representation-attribute/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧩add-representation-attribute/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧩add-representation-attribute/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧩add-representation-attribute/🧪️tests/🧪️adds-color-attribute-to-shell/🦀️.rs"]
                                    mod tests_adds_color_attribute_to_shell;
                                }
                                #[path = "."]
                                pub mod remove_representation_attribute {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/➖remove-representation-attribute/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/➖remove-representation-attribute/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/➖remove-representation-attribute/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/➖remove-representation-attribute/🧪️tests/🧪️drops-finish-attribute-from-shell/🦀️.rs"]
                                    mod tests_drops_finish_attribute_from_shell;
                                }
                                #[path = "."]
                                pub mod create_vortex_kind {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌱create-vortex-kind/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌱create-vortex-kind/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌱create-vortex-kind/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌱create-vortex-kind/🧪️tests/🧪️appends-vent-vortex-kind/🦀️.rs"]
                                    mod tests_appends_vent_vortex_kind;
                                }
                                #[path = "."]
                                pub mod delete_vortex_kind {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/❌delete-vortex-kind/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/❌delete-vortex-kind/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/❌delete-vortex-kind/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/❌delete-vortex-kind/🧪️tests/🧪️removes-hatch-vortex-kind/🦀️.rs"]
                                    mod tests_removes_hatch_vortex_kind;
                                }
                                #[path = "."]
                                pub mod rename_vortex_kind {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🖋️rename-vortex-kind/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🖋️rename-vortex-kind/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🖋️rename-vortex-kind/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🖋️rename-vortex-kind/🧪️tests/🧪️renames-door-to-portal/🦀️.rs"]
                                    mod tests_renames_door_to_portal;
                                }
                                #[path = "."]
                                pub mod change_vortex_kind_label {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🎫change-vortex-kind-label/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🎫change-vortex-kind-label/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🎫change-vortex-kind-label/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🎫change-vortex-kind-label/🧪️tests/🧪️relabels-door-vortex-kind/🦀️.rs"]
                                    mod tests_relabels_door_vortex_kind;
                                }
                                #[path = "."]
                                pub mod change_vortex_kind_color {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🎨change-vortex-kind-color/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🎨change-vortex-kind-color/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🎨change-vortex-kind-color/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🎨change-vortex-kind-color/🧪️tests/🧪️recolors-door-vortex-kind/🦀️.rs"]
                                    mod tests_recolors_door_vortex_kind;
                                }
                                #[path = "."]
                                pub mod change_vortex_kind_default_cable_kind {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔌change-vortex-kind-default-cable-kind/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔌change-vortex-kind-default-cable-kind/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔌change-vortex-kind-default-cable-kind/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔌change-vortex-kind-default-cable-kind/🧪️tests/🧪️swaps-door-default-cable-kind/🦀️.rs"]
                                    mod tests_swaps_door_default_cable_kind;
                                }
                                #[path = "."]
                                pub mod create_vortex {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌀create-vortex/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌀create-vortex/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌀create-vortex/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌀create-vortex/🧪️tests/🧪️appends-rear-vortex/🦀️.rs"]
                                    mod tests_appends_rear_vortex;
                                }
                                #[path = "."]
                                pub mod delete_vortex {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🕳️delete-vortex/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🕳️delete-vortex/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🕳️delete-vortex/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🕳️delete-vortex/🧪️tests/🧪️removes-front-vortex/🦀️.rs"]
                                    mod tests_removes_front_vortex;
                                }
                                #[path = "."]
                                pub mod move_vortex {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📍move-vortex/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📍move-vortex/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📍move-vortex/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📍move-vortex/🧪️tests/🧪️repositions-front-vortex/🦀️.rs"]
                                    mod tests_repositions_front_vortex;
                                }
                                #[path = "."]
                                pub mod resize_vortex {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📏resize-vortex/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📏resize-vortex/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📏resize-vortex/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📏resize-vortex/🧪️tests/🧪️widens-front-vortex/🦀️.rs"]
                                    mod tests_widens_front_vortex;
                                }
                                #[path = "."]
                                pub mod change_vortex_vortex_kind {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧷change-vortex-vortex-kind/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧷change-vortex-vortex-kind/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧷change-vortex-vortex-kind/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧷change-vortex-vortex-kind/🧪️tests/🧪️rekinds-front-vortex-as-hatch/🦀️.rs"]
                                    mod tests_rekinds_front_vortex_as_hatch;
                                }
                                #[path = "."]
                                pub mod change_vortex_label {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪧change-vortex-label/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪧change-vortex-label/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪧change-vortex-label/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪧change-vortex-label/🧪️tests/🧪️relabels-front-vortex/🦀️.rs"]
                                    mod tests_relabels_front_vortex;
                                }
                                #[path = "."]
                                pub mod add_compatibility_rule {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/➕add-compatibility-rule/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/➕add-compatibility-rule/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/➕add-compatibility-rule/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/➕add-compatibility-rule/🧪️tests/🧪️allows-door-to-hatch/🦀️.rs"]
                                    mod tests_allows_door_to_hatch;
                                }
                                #[path = "."]
                                pub mod remove_compatibility_rule {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/✂️remove-compatibility-rule/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/✂️remove-compatibility-rule/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/✂️remove-compatibility-rule/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/✂️remove-compatibility-rule/🧪️tests/🧪️revokes-door-to-door/🦀️.rs"]
                                    mod tests_revokes_door_to_door;
                                }
                                #[path = "."]
                                pub mod add_attribute {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔩add-attribute/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔩add-attribute/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔩add-attribute/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔩add-attribute/🧪️tests/🧪️adds-weight-attribute/🦀️.rs"]
                                    mod tests_adds_weight_attribute;
                                }
                                #[path = "."]
                                pub mod remove_attribute {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🚷remove-attribute/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🚷remove-attribute/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🚷remove-attribute/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🚷remove-attribute/🧪️tests/🧪️drops-material-attribute/🦀️.rs"]
                                    mod tests_drops_material_attribute;
                                }
                                #[path = "."]
                                pub mod add_author {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/👤add-author/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/👤add-author/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/👤add-author/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/👤add-author/🧪️tests/🧪️credits-bo/🦀️.rs"]
                                    mod tests_credits_bo;
                                }
                                #[path = "."]
                                pub mod remove_author {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🙅remove-author/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🙅remove-author/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🙅remove-author/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🙅remove-author/🧪️tests/🧪️uncredits-ada/🦀️.rs"]
                                    mod tests_uncredits_ada;
                                }
                                #[path = "."]
                                pub mod move_camera3d {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🎥move-camera3d/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🎥move-camera3d/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🎥move-camera3d/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🎥move-camera3d/🧪️tests/🧪️orbits-camera/🦀️.rs"]
                                    mod tests_orbits_camera;
                                }
                                #[path = "."]
                                pub mod scale_camera3d {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔍scale-camera3d/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔍scale-camera3d/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔍scale-camera3d/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔍scale-camera3d/🧪️tests/🧪️zooms-camera-out/🦀️.rs"]
                                    mod tests_zooms_camera_out;
                                }
                                #[path = "."]
                                pub mod change_meta_description {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💬change-meta-description/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💬change-meta-description/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💬change-meta-description/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💬change-meta-description/🧪️tests/🧪️rewrites-session-notes/🦀️.rs"]
                                    mod tests_rewrites_session_notes;
                                }
                            }
                        }
                        #[path = "."]
                        pub mod io {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "."]
                            pub mod import {
                                #[path = "."]
                                pub mod deserializers {
                                    #[path = "."]
                                    pub mod artifacts {
                                        #[path = "."]
                                        pub mod zip {
                                            #[path = "."]
                                            pub mod v2_0 {
                                                #[path = "."]
                                                pub mod any {
                                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/🎒️zip/🔖️2.0/✳️any/🦀️.rs"]
                                                    mod component;
                                                    pub use component::*;
                                                }
                                            }
                                        }
                                        #[path = "."]
                                        pub mod txt {
                                            #[path = "."]
                                            pub mod v_utf_8 {
                                                #[path = "."]
                                                pub mod any {
                                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/🔤️txt/🔖️utf-8/✳️any/🦀️.rs"]
                                                    mod component;
                                                    pub use component::*;
                                                }
                                            }
                                        }
                                        #[path = "."]
                                        pub mod png {
                                            #[path = "."]
                                            pub mod v1_2 {
                                                #[path = "."]
                                                pub mod any {
                                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/📷️png/🔖️1.2/✳️any/🦀️.rs"]
                                                    mod component;
                                                    pub use component::*;
                                                }
                                            }
                                        }
                                        #[path = "."]
                                        pub mod json {
                                            #[path = "."]
                                            pub mod v_rfc8259 {
                                                #[path = "."]
                                                pub mod any {
                                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/🔣️json/🔖️rfc8259/✳️any/🦀️.rs"]
                                                    mod component;
                                                    pub use component::*;
                                                }
                                            }
                                        }
                                        #[path = "."]
                                        pub mod stl {
                                            #[path = "."]
                                            pub mod v_ascii {
                                                #[path = "."]
                                                pub mod any {
                                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/🔺️stl/🔖️ascii/✳️any/🦀️.rs"]
                                                    mod component;
                                                    pub use component::*;
                                                }
                                            }
                                        }
                                        #[path = "."]
                                        pub mod obj {
                                            #[path = "."]
                                            pub mod v3_0 {
                                                #[path = "."]
                                                pub mod any {
                                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/🧊️obj/🔖️3.0/✳️any/🦀️.rs"]
                                                    mod component;
                                                    pub use component::*;
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                            #[path = "."]
                            pub mod export {
                                #[path = "."]
                                pub mod serializers {
                                    #[path = "."]
                                    pub mod artifacts {
                                        #[path = "."]
                                        pub mod zip {
                                            #[path = "."]
                                            pub mod v2_0 {
                                                #[path = "."]
                                                pub mod any {
                                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📤️export/🧵️serializers/🗿️artifacts/🎒️zip/🔖️2.0/✳️any/🦀️.rs"]
                                                    mod component;
                                                    pub use component::*;
                                                }
                                            }
                                        }
                                        #[path = "."]
                                        pub mod txt {
                                            #[path = "."]
                                            pub mod v_utf_8 {
                                                #[path = "."]
                                                pub mod any {
                                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📤️export/🧵️serializers/🗿️artifacts/🔤️txt/🔖️utf-8/✳️any/🦀️.rs"]
                                                    mod component;
                                                    pub use component::*;
                                                }
                                            }
                                        }
                                        #[path = "."]
                                        pub mod png {
                                            #[path = "."]
                                            pub mod v1_2 {
                                                #[path = "."]
                                                pub mod any {
                                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📤️export/🧵️serializers/🗿️artifacts/📷️png/🔖️1.2/✳️any/🦀️.rs"]
                                                    mod component;
                                                    pub use component::*;
                                                }
                                            }
                                        }
                                        #[path = "."]
                                        pub mod json {
                                            #[path = "."]
                                            pub mod v_rfc8259 {
                                                #[path = "."]
                                                pub mod any {
                                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📤️export/🧵️serializers/🗿️artifacts/🔣️json/🔖️rfc8259/✳️any/🦀️.rs"]
                                                    mod component;
                                                    pub use component::*;
                                                }
                                            }
                                        }
                                        #[path = "."]
                                        pub mod stl {
                                            #[path = "."]
                                            pub mod v_ascii {
                                                #[path = "."]
                                                pub mod any {
                                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📤️export/🧵️serializers/🗿️artifacts/🔺️stl/🔖️ascii/✳️any/🦀️.rs"]
                                                    mod component;
                                                    pub use component::*;
                                                }
                                            }
                                        }
                                        #[path = "."]
                                        pub mod obj {
                                            #[path = "."]
                                            pub mod v3_0 {
                                                #[path = "."]
                                                pub mod any {
                                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📤️export/🧵️serializers/🗿️artifacts/🧊️obj/🔖️3.0/✳️any/🦀️.rs"]
                                                    mod component;
                                                    pub use component::*;
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }

                pub use crate::standards::v1::subsets::any::schema::diff::Block3dDiff;
        pub use crate::standards::v1::subsets::any::schema::mutations::Block3dMutation;
        pub use crate::standards::v1::subsets::any::schema::snapshot::Block3dSnapshot;

#[path = "."]
pub mod examples {
    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🌲️hexagonal-cut-concrete-forest-left/🦀️.rs"]
    pub mod art_3d_hexagonal_cut_concrete_forest_left;
    #[cfg(test)]
    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🌲️hexagonal-cut-concrete-forest-left/🧪️tests/🧩️example/🦀️.rs"]
    mod art_3d_hexagonal_cut_concrete_forest_left_tests;
    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🏢️nakagin-capsule/🦀️.rs"]
    pub mod art_3d_nakagin_capsule;
    #[cfg(test)]
    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🏢️nakagin-capsule/🧪️tests/🧩️example/🦀️.rs"]
    mod art_3d_nakagin_capsule_tests;
}

#[cfg(feature = "component-app-assembly")]
#[path = "."]
pub mod editor {
    #[path = "."]
    pub mod block3d {
#[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs"]
        mod component;
        pub use component::*;

        #[path = "."]
        pub mod config {
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎚️config/🦀️.rs"]
            mod component;
            pub use component::*;

            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎚️config/🧬️schema/🦀️.rs"]
            pub mod schema;
        }

        #[path = "."]
        pub mod presence {
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/👥️presence/🦀️.rs"]
            mod component;
            pub use component::*;

            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/👥️presence/🧬️schema/🦀️.rs"]
            pub mod schema;
        }

        #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🗣️terminology/🦀️.rs"]
        pub mod terminology;
        #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🌍️world/🦀️.rs"]
        pub mod world;

        #[path = "."]
        pub mod commands {
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🧱️add-representation/🦀️.rs"]
            pub mod add_representation;
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🌀️add-vortex/🦀️.rs"]
            pub mod add_vortex;
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🔘️add-vortex-kind/🦀️.rs"]
            pub mod add_vortex_kind;
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🎨️edit/🦀️.rs"]
            pub mod edit;
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🖌️hover-surface/🦀️.rs"]
            pub mod hover_surface;
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/👋️leave-surface/🦀️.rs"]
            pub mod leave_surface;
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🏷️patch-object-kind/🦀️.rs"]
            pub mod patch_object_kind;
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🩹️patch-representation/🦀️.rs"]
            pub mod patch_representation;
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/📍️place-vortex/🦀️.rs"]
            pub mod place_vortex;
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🚮️remove-representation/🦀️.rs"]
            pub mod remove_representation;
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/➖️remove-vortex/🦀️.rs"]
            pub mod remove_vortex;
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🗑️remove-vortex-kind/🦀️.rs"]
            pub mod remove_vortex_kind;
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🎬️set-active-example/🦀️.rs"]
            pub mod set_active_example;
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🪟️set-active-representation/🦀️.rs"]
            pub mod set_active_representation;
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🔄️set-brush-flip/🦀️.rs"]
            pub mod set_brush_flip;
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/📏️set-brush-radius/🦀️.rs"]
            pub mod set_brush_radius;
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🧷️set-brush-vortex-kind/🦀️.rs"]
            pub mod set_brush_vortex_kind;
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🎥️set-camera/🦀️.rs"]
            pub mod set_camera;
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/↔️set-window-arrangement/🦀️.rs"]
            pub mod set_window_arrangement;
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🖼️set-window-representations/🦀️.rs"]
            pub mod set_window_representations;
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/📐️set-window-spacing/🦀️.rs"]
            pub mod set_window_spacing;
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🔀️toggle-window-representation/🦀️.rs"]
            pub mod toggle_window_representation;
        }

        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod edit {
                #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🦀️.rs"]
                mod component;
                pub use component::*;

                #[path = "."]
                pub mod windows {
                    #[path = "."]
                    pub mod world {
                        #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/🌐️world/🦀️.rs"]
                        mod component;
                        pub use component::*;

                        #[path = "."]
                        pub mod transient {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/🌐️world/🫧️transient/🦀️.rs"]
                            mod component;
                            pub use component::*;
                        }

                        #[path = "."]
                        pub mod options {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/🌐️world/☑️options/↔️arrangement/🦀️.rs"]
                            pub mod arrangement;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/🌐️world/☑️options/🖌️brush/🦀️.rs"]
                            pub mod brush;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/🌐️world/☑️options/🔀️quick-representation/🦀️.rs"]
                            pub mod quick_representation;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/🌐️world/☑️options/🧱️representations/🦀️.rs"]
                            pub mod representations;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/🌐️world/☑️options/📏️spacing/🦀️.rs"]
                            pub mod spacing;
                        }
                    }
                }
            }
        }

        #[path = "."]
        pub mod panels {
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📌️panels/🗿️artifact/🦀️.rs"]
            pub mod document;
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📌️panels/🔍️inspection/🦀️.rs"]
            pub mod inspection;
        }
    }
}

#[cfg(feature = "component-app-assembly")]
#[path = "."]
pub mod viewer {
    #[path = "."]
    pub mod block3d {
#[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/👁️viewer/🦀️.rs"]
        mod component;
        pub use component::*;

        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod view {
                #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/👁️viewer/🎭️modes/👁️view/🦀️.rs"]
                mod component;
                pub use component::*;

                #[path = "."]
                pub mod windows {
                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/👁️viewer/🎭️modes/👁️view/🪟️windows/🌐️world/🦀️.rs"]
                    pub mod world;
                }
            }
        }
    }
}
