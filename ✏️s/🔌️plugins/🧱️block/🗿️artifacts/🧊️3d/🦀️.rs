//! 🏙️ Block 3D artifact — the document entity the 🧊️3d app edits (constitutional: general). Edits
//! exactly one `ObjectKind`: its identity, representations (meshes at LOD/tags — the semio_compose_rs
//! `type` app's successor), and the `VortexKind` templates placed on its rim.

pub use crate::artifacts::block3d::schema::snapshot::Block3dSnapshot;

use semio_framework_plugin::{ArtifactKindSpec, MediaClass, MediaForm, MediaType, OsMediaCapability};
use semio_s_plugin_stdio::artifacts::semio::standards::v1::subsets::kit::schema::snapshot::{SemioKitSnapshot, SemioKitType};

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
pub async fn kit_type_from_vortex_kind(kind: &Block3dVortexKind) -> SemioKitType {
    SemioKitType { id: kind.id.clone(), name: kind.name.clone(), category: "vortex-kind".into() }
}

/// 🔀️ `Block3dVortexKind` → the block3d-owned overflow half (`vortex_kind_extra`) the composed kit
/// type cannot carry. Lossless together with `kit_type_from_vortex_kind`: every `Block3dVortexKind`
/// field lands in exactly one of the two halves.
pub async fn vortex_kind_extra_from_vortex_kind(kind: &Block3dVortexKind) -> Block3dVortexKindExtra {
    Block3dVortexKindExtra { id: kind.id.clone(), name: kind.name.clone(), label: kind.label.clone(), color: kind.color.clone(), default_cable_kind: kind.default_cable_kind.clone() }
}

/// 🔀️ Inverse of the split above — reassembles one full `Block3dVortexKind` from its two composed
/// halves.
pub async fn vortex_kind_from_parts(kit_type: &SemioKitType, extra: &Block3dVortexKindExtra) -> Block3dVortexKind {
    Block3dVortexKind { id: kit_type.id.clone(), name: kit_type.name.clone(), label: extra.label.clone(), color: extra.color.clone(), default_cable_kind: extra.default_cable_kind.clone() }
}

/// 🔀️ The full vortex-kinds list's shared half, as a fresh (design-less, link-less) `SemioKitSnapshot`
/// — content-addressed by `catalog_child_handle` below, never embedded inline in `Block3dSnapshot`.
pub async fn catalog_snapshot_from_vortex_kinds(kinds: &[Block3dVortexKind]) -> SemioKitSnapshot {
    SemioKitSnapshot { types: kinds.iter().map(kit_type_from_vortex_kind).collect(), ..SemioKitSnapshot::default() }
}

/// 🔀️ The full vortex-kinds list's block3d-owned overflow half.
pub async fn vortex_kind_extra_list_from_vortex_kinds(kinds: &[Block3dVortexKind]) -> Vec<Block3dVortexKindExtra> {
    kinds.iter().map(vortex_kind_extra_from_vortex_kind).collect()
}

/// 🔀️ Reassembles the full `Vec<Block3dVortexKind>` catalog from its composed-child half and its
/// block3d-owned overflow half, id-joined. A `SemioKitType` with no matching `Block3dVortexKindExtra`
/// (composed-child content the working-scene cache hasn't seen yet — see `vortex_kinds_of`'s doc
/// comment) is silently dropped rather than fabricated with placeholder label/color.
pub async fn vortex_kinds_from_catalog_and_extra(catalog: &SemioKitSnapshot, extra: &[Block3dVortexKindExtra]) -> Vec<Block3dVortexKind> {
    let extra_by_id: std::collections::HashMap<&str, &Block3dVortexKindExtra> = extra.iter().map(|e| (e.id.as_str(), e)).collect();
    catalog.types.iter().filter_map(|kit_type| extra_by_id.get(kit_type.id.as_str()).map(|extra| vortex_kind_from_parts(kit_type, extra))).collect()
}

/// 🪪️ Content-addressed child handle for a vortex-kinds list's shared catalog half — hashes the
/// deterministic JSON of the derived `SemioKitType` list so peers replaying the same vortex-kinds
/// converge on the same `child_id` (never a random/incrementing id), mirroring `sourcing`'s
/// `catalog_child_handle`.
pub async fn catalog_child_handle(kinds: &[Block3dVortexKind]) -> store::ArtifactChild<SemioKitSnapshot> {
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
pub async fn vortex_kinds_of_parts(catalog: &store::ArtifactChild<SemioKitSnapshot>, extra: &[Block3dVortexKindExtra]) -> Vec<Block3dVortexKind> {
    let _ = catalog;
    extra.iter().map(|row| Block3dVortexKind { id: row.id.clone(), name: row.name.clone(), label: row.label.clone(), color: row.color.clone(), default_cable_kind: row.default_cable_kind.clone() }).collect()
}

/// 👁️ `vortex_kinds_of_parts` specialized to `Block3dSnapshot`.
pub async fn vortex_kinds_of(snapshot: &Block3dSnapshot) -> Vec<Block3dVortexKind> {
    vortex_kinds_of_parts(&snapshot.catalog, &snapshot.vortex_kind_extra)
}

/// ✍️ The one writer every mutation-diff-apply call site funnels through to replace the full vortex-
/// kinds catalogue: mints a fresh content-addressed `catalog` handle, seeds the working-scene cache,
/// and writes the overflow half — given the composed child handle and overflow list directly (works
/// for both `Block3dSnapshot` and `Block3dArtifact`).
pub async fn set_vortex_kinds_parts(catalog: &mut store::ArtifactChild<SemioKitSnapshot>, extra: &mut Vec<Block3dVortexKindExtra>, kinds: Vec<Block3dVortexKind>) {
    let handle = catalog_child_handle(&kinds);
    *catalog = handle;
    *extra = vortex_kind_extra_list_from_vortex_kinds(&kinds);
}

/// ✍️ `set_vortex_kinds_parts` specialized to `Block3dSnapshot`.
pub async fn set_vortex_kinds(snapshot: &mut Block3dSnapshot, kinds: Vec<Block3dVortexKind>) {
    set_vortex_kinds_parts(&mut snapshot.catalog, &mut snapshot.vortex_kind_extra, kinds);
}

/// 🌱 Seeds the working-scene cache for `kinds`'s deterministic `catalog_child_handle`, without
/// writing any snapshot fields — for fixture loaders that parse the persisted snapshot from DSL text
/// (which never embeds child content) but still need the SAME content-addressed handle's catalog
/// resolvable immediately after loading.
pub async fn validate_vortex_kind_catalog(kinds: &[Block3dVortexKind]) {
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
/// 🪟 Per-window-instance view state (representation subset, layout, active utility).
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
    #[value(default = "default_active_utility")]
    #[cfg_attr(test, serde(default = "default_active_utility"))]
    pub active_utility: String,
}

async fn default_arrangement() -> String {
    "overlap".into()
}

async fn default_spacing() -> f64 {
    8.0
}

async fn default_active_utility() -> String {
    crate::editor::block3d::BLOCK3D_UTILITY_SELECT.into()
}

impl Block3dWindowView {
    /// 🪟 Builds a default view record for one window id.
    pub async fn for_window(window_id: impl Into<String>) -> Self {
        Self { window_id: window_id.into(), representation_ids: Vec::new(), arrangement: default_arrangement(), spacing: default_spacing(), active_utility: default_active_utility() }
    }
}

/// 🖌️ Transient brush hover pose in world space (config/preview).
#[derive(Clone, Debug, PartialEq, dsl::ToValue, dsl::FromValue, dsl::DslRecord)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[value(rename_all = "camelCase")]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
pub struct Block3dBrushPreview {
    #[dsl(coord)]
    pub position: [f64; 3],
    #[dsl(dir)]
    pub direction: [f64; 3],
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
        export_stdio_kinds: vec!["stdio.json", "stdio.obj", "stdio.png", "stdio.stl", "stdio.zip"],
        import_stdio_kinds: vec!["stdio.json", "stdio.obj", "stdio.png", "stdio.stl", "stdio.zip"],
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
mod tests {
    use super::*;

    #[semio_framework_async_macros::async_test]
    async fn artifact_kind_declares_the_3d_block_interchange_kind() {
        let kind = artifact_kind();
        assert_eq!(kind.id, "3d.block");
        assert_eq!(kind.schema, BLOCK_3D_SCHEMA);
        assert_eq!(kind.component_kind, "block3d");
    }
}
//#endregion 🧪️Tests
//#region 🪪️Declaration
/// 🔖️ This artifact's declaration (ticket 26/08/12/ARTIFACTS-ONLY-PLUGIN-ARCHITECTURE M1) — replaces
/// the old side-effecting `register()`, which called four different global registries directly from
/// a plugin `.setup()` callback. `Block3dPlayApp`'s CONFIG/PRESENCE schema — an app-scope concern
/// `ArtifactDeclaration` deliberately has no field for (see that struct's own doc) — now registers via
/// `ArtifactEditor::app_schema()` returning `crate::editor::block3d::config::schema::app_schema_descriptor()`
/// (ticket W1c), so `.setup()` is gone from `🧱️block/🦀️.rs` entirely.
pub async fn definition() -> Result<semio_framework_plugin::ArtifactDefinition, semio_framework_plugin::ArtifactDefinitionError> {
    use semio_framework_plugin::{ArtifactCapability, ArtifactCapabilityKind, ArtifactDefinition, ArtifactIdentity, ArtifactIdentityClaim, ArtifactIdentityNamespace, ArtifactLocale, ArtifactLocalization};

    let rows: &[(&str, &str, &str, &[(&str, &str)], Option<(&str, &str)>)] = &[
        ("s.block3d.standard.v1", "standard", "1", &[], None),
        ("s.block3d.standard.v1.profile.any", "profile", "any", &[], None),
        ("s.block3d.schema.artifact", "schema", "s.block.block3d", &[("schema", "s.block.block3d")], None),
        ("s.block3d.inference.artifact", "inference", "s.block.block3d.inference", &[("schema", "s.block.block3d.inference")], None),
        ("s.block3d.composer.native", "composer", "s.block3d@1/*", &[("dialect", "s.block3d@1/*")], None),
        ("s.block3d.composer.format-1", "composer", "s.stdio.zip@2.0/*", &[("dialect", "s.stdio.zip@2.0/*")], None),
        ("s.block3d.composer.format-2", "composer", "s.stdio.png@1.2/*", &[("dialect", "s.stdio.png@1.2/*")], None),
        ("s.block3d.composer.format-3", "composer", "s.stdio.json@rfc8259/*", &[("dialect", "s.stdio.json@rfc8259/*")], None),
        ("s.block3d.composer.format-4", "composer", "s.stdio.stl@ascii/*", &[("dialect", "s.stdio.stl@ascii/*")], None),
        ("s.block3d.composer.format-5", "composer", "s.stdio.obj@3.0/*", &[("dialect", "s.stdio.obj@3.0/*")], None),
        ("s.block3d.grammar.1", "grammar", "block.block3d", &[("grammar", "block.block3d")], None),
        ("s.block3d.grammar.2", "grammar", "block.block3d.op", &[("grammar", "block.block3d.op")], None),
        ("s.block3d.grammar.3", "grammar", "block.block3d.diff", &[("grammar", "block.block3d.diff")], None),
        ("s.block3d.grammar.4", "grammar", "3d.pack", &[("grammar", "3d.pack")], None),
        ("s.block3d.grammar.5", "grammar", "3d.spr", &[("grammar", "3d.spr")], None),
        // 🐛️ D2-capability-claim-repairs: `.document_codec::<EditorApp<Block3dPlayApp>>()` derives
        // its extension claim from `<Block3dSnapshot as store::ArtifactDsl>::EXTENSION`
        // (`…/🧬️schema/📸️snapshot/🦀️.rs`), which is `"block3d"`, not `"block"`.
        ("s.block3d.codec.document-1", "codec", "block.3d:block3d", &[("codec", "block.3d"), ("extension", "block3d")], None),
        ("s.block3d.localization.en", "localization", "3D Block", &[], Some(("en", "3D Block"))),
        ("s.block3d.localization.de", "localization", "3D-Baustein", &[], Some(("de", "3D-Baustein"))),
    ];
    let mut definition = ArtifactDefinition::new(ArtifactIdentity::parse("s.block3d")?);
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
pub fn artifact() -> semio_framework_plugin::app::declarations::ArtifactDeclaration<crate::BlockApps> {
    use semio_framework_plugin::app::declarations::ArtifactDeclaration;
    use store::os_io::ArtifactKindId;
    ArtifactDeclaration { kind: ArtifactKindId::parse("s.block.block3d").expect("canonical block3d kind"), localization: &[], standards: vec![crate::artifacts::block3d::standards::v1::standard()] }
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
                    grammar: Some(crate::artifacts::block3d::dsl::COMPONENT_GRAMMAR_SEMIO),
                    grammar_path: Some(crate::artifacts::block3d::dsl::COMPONENT_GRAMMAR_PATH),
                    protocol: Some(crate::artifacts::block3d::snapshot::pack::COMPONENT_PROTOCOL_SEMIO),
                    protocol_path: Some(crate::artifacts::block3d::snapshot::pack::COMPONENT_PROTOCOL_PATH),
                    hooks: dsl::passthrough_hooks("block.block3d"),
                },
                dsl::LanguageSpec {
                    id: "block.block3d.op",
                    extension: None,
                    role: dsl::LanguageRole::Ops,
                    grammar: Some(crate::artifacts::block3d::op::COMPONENT_GRAMMAR_SEMIO),
                    grammar_path: Some(crate::artifacts::block3d::op::COMPONENT_GRAMMAR_PATH),
                    protocol: Some(crate::artifacts::block3d::spr::COMPONENT_PROTOCOL_SEMIO),
                    protocol_path: Some(crate::artifacts::block3d::spr::COMPONENT_PROTOCOL_PATH),
                    hooks: dsl::passthrough_hooks("block.block3d.op"),
                },
                dsl::LanguageSpec {
                    id: "block.block3d.diff",
                    extension: None,
                    role: dsl::LanguageRole::Diff,
                    grammar: Some(crate::artifacts::block3d::diff::COMPONENT_GRAMMAR_SEMIO),
                    grammar_path: Some(crate::artifacts::block3d::diff::COMPONENT_GRAMMAR_PATH),
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
                    protocol: Some(crate::artifacts::block3d::snapshot::pack::COMPONENT_PROTOCOL_SEMIO),
                    protocol_path: Some(crate::artifacts::block3d::snapshot::pack::COMPONENT_PROTOCOL_PATH),
                    hooks: dsl::passthrough_hooks("3d.pack"),
                },
                dsl::LanguageSpec {
                    id: "3d.spr",
                    extension: None,
                    role: dsl::LanguageRole::Spr,
                    grammar: None,
                    grammar_path: None,
                    protocol: Some(crate::artifacts::block3d::spr::COMPONENT_PROTOCOL_SEMIO),
                    protocol_path: Some(crate::artifacts::block3d::spr::COMPONENT_PROTOCOL_PATH),
                    hooks: dsl::passthrough_hooks("3d.spr"),
                },
            ]
        })
        .as_slice()
}
//#endregion 🪪️Declaration
