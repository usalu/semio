//! 🗂️ Sourcing curate artifact — the document entities this plugin's curate app edits: a catalogue of
//! object kinds (parametric geometry + typology + availability) and a curated selection.

use semio_framework_plugin::{ArtifactKindSpec, Dialect, MediaClass, MediaForm, MediaType, OsMediaCapability, StandardId, SubsetId};
use semio_framework_value_derive::{FromValue, ToValue};
use semio_s_plugin_stdio::artifacts::semio::standards::v1::subsets::kit::schema::snapshot::{SemioKitSnapshot, SemioKitType};
use serde::{Deserialize, Serialize};

pub use crate::artifacts::curate::schema::mutations::SourcingMutation;

pub use crate::artifacts::curate::schema::diff::CurateDiff;

pub const SOURCING_CURATE_SCHEMA: &str = "sourcing.curate/v1";
pub use crate::artifacts::curate::schema::snapshot::CurateSnapshot;

/// 🪪️ This artifact's canonical `Dialect` (ticket 26/08/16/ARTIFACT-VIEWERS-AND-EDITORS-PER-SUBSET,
/// contract §1/§7.4) — lives at the ARTIFACT level (not under `editor`/`viewer`) specifically so a
/// viewer file can read it without ever importing through the sibling `editor` module. `artifact_kind
/// = "s.sourcing.curate"` matches the id `definition()`'s own `"s.curate.schema.artifact"` capability
/// row descriptor below; `standard`/`subset` match this file's own
/// `🏅️standards/🔖️1/🪆️subsets/✳️any` location — the canonical surface id is
/// `s.sourcing.curate@1/*#editor` / `s.sourcing.curate@1/*#viewer`, exactly the contract §1 grammar.
pub const SOURCING_DIALECT: Dialect = Dialect { artifact_kind: "s.sourcing.curate", standard: StandardId("1"), subset: SubsetId::ANY };

//#region 🔖️Geometry
/// 📦️ A parametric geometry recipe an object kind is composed of — data describing shape, not a subclass.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ToValue, FromValue, dsl::DslEnum)]
#[serde(tag = "kind", rename_all = "camelCase")]
#[value(tag = "kind", rename_all = "camelCase")]
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
}
//#endregion 🔖️Geometry

//#region 🔖️ObjectKind
/// 🧱️ A catalogue object KIND: identity ∘ typology reference ∘ availability ∘ geometry (composition, not subclassing).
///
/// `geometry` is `Box<GeometryRecipe>` (not a bare `GeometryRecipe`) because `#[dsl(statements)]`'s
/// `RequiredStatements` shape — the "exactly one required tagged value" slot a `DslEnum` sum type
/// needs to occupy a plain (non-`Option`, non-`Vec`) field — only recognizes a `Box<T>` inner type.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ToValue, FromValue, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
#[value(rename_all = "camelCase")]
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

//#region 🔖️Document
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize, dsl::DslScalar)]
#[serde(rename_all = "camelCase")]
pub enum SortDirection {
    Asc,
    Desc,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
pub struct TableSort {
    pub column_id: String,
    pub direction: SortDirection,
}

/// 🔍️ The pool table's active filter set — narrows `CurateSnapshot::stock` down to `filtered_stock()`.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
pub struct Filters {
    #[serde(default)]
    pub query: String,
    #[serde(default)]
    pub module_ids: Vec<String>,
    #[serde(default)]
    pub typology_path: Vec<String>,
    #[serde(default)]
    pub min_availability: u32,
    #[serde(default)]
    #[dsl(block)]
    pub sort: Option<TableSort>,
}

/// 🧺️ One curated object kind and how many units of it have been picked.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
pub struct CuratedItem {
    #[dsl(refs = "object")]
    pub object_id: String,
    pub count: u32,
}

//#endregion 🔖️Document

//#region 🔖️CatalogComposition
/// 🧩️ Sourcing-owned per-kind metadata NOT representable in stdio's composed `s.stdio.semio.kit`
/// subset (`SemioKitType` carries only `id`/`name`/`category`) — typology classification,
/// availability, and procedural geometry. Id-joined 1:1 to a `SemioKitType` in the composed
/// `CurateSnapshot::catalog` child by `id` (see `stock_of`/`object_kind_from_parts`). Ticket
/// 26/08/12/UNIFIED-COMPOSABLE-ARTIFACT-SYSTEM: replaces the former inline `stock: Vec<ObjectKind>`
/// field, which duplicated the `kit.catalog`/type-registry vocabulary this ticket composes instead.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
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
/// content-addressed by `catalog_child_handle` below, never embedded inline in `CurateSnapshot`.
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
pub fn catalog_child_handle(stock: &[ObjectKind]) -> store::ArtifactChild<SemioKitSnapshot> {
    use std::hash::{Hash, Hasher};
    let catalog = catalog_snapshot_from_stock(stock);
    let canonical = serde_json::to_string(&catalog.types).unwrap_or_default();
    let mut hasher = std::collections::hash_map::DefaultHasher::new();
    canonical.hash(&mut hasher);
    let child_id = format!("catalog-{:016x}", hasher.finish());
    let dialect = store::os_io::ArtifactDialect { artifact_kind: "s.stdio.semio".into(), standard: "v1".into(), subset: "kit".into() };
    let target = store::os_io::ArtifactRef { artifact_id: child_id.clone(), dialect };
    store::ArtifactChild::new(child_id, target)
}

/// 🏗️ Builds a `CurateSnapshot` from a full stock list, minting its content-addressed `catalog`
/// handle, splitting the stock into its composed-child half and sourcing-owned overflow half, and
/// seeding the working-scene cache so this SAME call's render/export/inference paths can resolve the
/// handle immediately. The one sanctioned construction path for "real stock content" — every fixture,
/// test, and command that used to write `CurateSnapshot { stock, .. }` directly goes through this now.
pub fn curate_snapshot_from_stock(stock: Vec<ObjectKind>, curated: Vec<CuratedItem>) -> CurateSnapshot {
    let handle = catalog_child_handle(&stock);
    CurateSnapshot { catalog: handle, stock_extra: stock_extra_from_stock(&stock), curated }
}

/// 🌱 Seeds the working-scene cache for `stock`'s deterministic `catalog_child_handle`, without
/// building a whole `CurateSnapshot` — for fixture loaders (`default_document`/`empty_document`) that
/// parse the persisted snapshot from DSL text (which never embeds child content) but still need the
/// SAME content-addressed handle's catalog resolvable immediately after loading.
pub fn validate_catalog_payload(stock: &[ObjectKind]) {
    let _ = catalog_child_handle(stock);
}

/// 👁️ The one accessor every render/export/inference call site funnels through to read the full
/// reassembled stock catalogue from snapshot-owned overflow records.
pub fn stock_of(document: &CurateSnapshot) -> Vec<ObjectKind> {
    let _ = &document.catalog;
    document.stock_extra.iter().map(|row| ObjectKind { id: row.id.clone(), name: row.name.clone(), module_id: row.module_id.clone(), typology_path: row.typology_path.clone(), availability: row.availability, geometry: row.geometry.clone() }).collect()
}
//#endregion 🔖️CatalogComposition

//#region 🔖️ArtifactKind
/// 🗂️ This artifact's `ArtifactKindSpec` — stitched into the app manifest by
/// `crate::editor::sourcing::create_sourcing_curate_app`'s `🔖️Manifest` region.
pub fn artifact_kind() -> ArtifactKindSpec {
    ArtifactKindSpec {
        id: "catalogue.sourcing".into(),
        name: "Sourcing Curation".into(),
        source_format: "sourcing.curate".into(),
        component_kind: "catalogue".into(),
        dimension: "data".into(),
        media_capability: OsMediaCapability::MeshOnly,
        media_type: MediaType { class: MediaClass::Kit, form: MediaForm::Kit },
        schema: "sourcing.curate".into(),
        export_formats: vec![],
        import_formats: vec![],
        export_stdio_kinds: vec!["stdio.json", "stdio.obj", "stdio.png", "stdio.stl", "stdio.zip"],
        import_stdio_kinds: vec!["stdio.json", "stdio.obj", "stdio.png", "stdio.stl", "stdio.zip"],
    }
}
//#endregion 🔖️ArtifactKind

//#region 🔖️Register
/// 🔖️ This artifact's declaration (ticket 26/08/12/ARTIFACTS-ONLY-PLUGIN-ARCHITECTURE M1) — replaces
/// the old side-effecting `register()`, which called four different global registries directly from
/// a plugin `.setup()` callback (a fifth, `crate::artifacts::curate::io_registry::register()`, was a
/// pure duplicate of what `.composers(...)` now does and was deleted rather than ported — see the
/// mechanism report's `register_all` composer-registration step). `crate::editor::sourcing::config::
/// schema::register_app_schema()` is the one exception, still called from `🪵️sourcing/🦀️.rs`'s
/// own `.setup()`: it registers the `SourcingCurateApp` CONFIG/PRESENCE schema, an app-scope concern
/// `ArtifactDeclaration` deliberately has no field for (see that struct's own doc). Relocated from
/// `⚙️engine` (ticket 26/08/12/ARTIFACTS-ONLY-PLUGIN-ARCHITECTURE reloc-g2): `declaration()` describes
/// the artifact (kind, schema, io ports, ownership), which is not engine behaviour.
pub fn definition() -> Result<semio_framework_plugin::ArtifactDefinition, semio_framework_plugin::ArtifactDefinitionError> {
    use semio_framework_plugin::{ArtifactCapability, ArtifactCapabilityKind, ArtifactDefinition, ArtifactIdentity, ArtifactIdentityClaim, ArtifactIdentityNamespace, ArtifactLocale, ArtifactLocalization};
    let rows: &[(&str, &str, &str, &[(&str, &str)], Option<(&str, &str)>)] = &[
        ("s.curate.standard.v1", "standard", "1", &[], None),
        ("s.curate.standard.v1.profile.any", "profile", "any", &[], None),
        ("s.curate.schema.artifact", "schema", "s.sourcing.curate", &[("schema", "s.sourcing.curate")], None),
        ("s.curate.inference.artifact", "inference", "s.sourcing.curate.inference", &[("schema", "s.sourcing.curate.inference")], None),
        ("s.curate.composer.zip", "composer", "s.stdio.zip@2.0/*", &[("dialect", "s.stdio.zip@2.0/*")], None),
        ("s.curate.composer.png", "composer", "s.stdio.png@1.2/*", &[("dialect", "s.stdio.png@1.2/*")], None),
        ("s.curate.composer.json", "composer", "s.stdio.json@rfc8259/*", &[("dialect", "s.stdio.json@rfc8259/*")], None),
        ("s.curate.composer.stl", "composer", "s.stdio.stl@ascii/*", &[("dialect", "s.stdio.stl@ascii/*")], None),
        ("s.curate.composer.obj", "composer", "s.stdio.obj@3.0/*", &[("dialect", "s.stdio.obj@3.0/*")], None),
        ("s.curate.grammar.document", "grammar", "sourcing.curate", &[("grammar", "sourcing.curate")], None),
        ("s.curate.grammar.op", "grammar", "sourcing.curate.op", &[("grammar", "sourcing.curate.op")], None),
        ("s.curate.grammar.diff", "grammar", "sourcing.curate.diff", &[("grammar", "sourcing.curate.diff")], None),
        ("s.curate.grammar.pack", "grammar", "curate.pack", &[("grammar", "curate.pack")], None),
        ("s.curate.grammar.spr", "grammar", "curate.spr", &[("grammar", "curate.spr")], None),
        ("s.curate.codec.document.v1", "codec", "sourcing.curate/v1:curate", &[("codec", "sourcing.curate/v1"), ("extension", "curate")], None),
        ("s.curate.localization.en", "localization", "Sourcing", &[], Some(("en", "Sourcing"))),
        ("s.curate.localization.de", "localization", "Beschaffung", &[], Some(("de", "Beschaffung"))),
    ];
    let mut definition = ArtifactDefinition::new(ArtifactIdentity::parse("s.curate")?);
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

/// 🌳️ This artifact's declaration tree root (ticket 26/08/17/CLEAN-ARTIFACT-STANDARD-SUBSET-
/// MECHANISM, design.md §2) — one standard (`1`), one subset (`any`). Replaces the OLD
/// `declaration()`/`ArtifactDeclaration::builder(...)` channel outright (atomic cutover with the
/// plugin root edit — no dual registration). `localization: &[]` is a documented shortfall: the
/// real en/de localized names still live on `definition()`'s kept capability rows (debt D1).
pub fn artifact() -> semio_framework_plugin::app::declarations::ArtifactDeclaration<crate::SourcingApps> {
    use semio_framework_plugin::app::declarations::ArtifactDeclaration;
    use store::os_io::ArtifactKindId;
    ArtifactDeclaration { kind: ArtifactKindId::parse("s.sourcing.curate").expect("canonical sourcing.curate kind"), localization: &[], standards: vec![crate::artifacts::curate::standards::v1::standard()] }
}

/// 📌️ Handcrafted facet grammars (text) and protocols (binary) — built once and leaked to a
/// `&'static` slice since `dsl::passthrough_hooks` isn't `const fn`. Consumed by
/// `🚪️io/🦀️.rs`'s `io()` (via `language_spec`) to populate `NativeCodecs`'s
/// `LanguagePair`s — the new declaration tree's home for what the OLD `declaration()`'s
/// `.languages(...)` call used to register.
pub(crate) fn pilot_languages() -> &'static [dsl::LanguageSpec] {
    static LANGUAGES: std::sync::OnceLock<Vec<dsl::LanguageSpec>> = std::sync::OnceLock::new();
    LANGUAGES
        .get_or_init(|| {
            vec![
                dsl::LanguageSpec {
                    id: "sourcing.curate",
                    extension: Some("curate"),
                    role: dsl::LanguageRole::Document,
                    grammar: Some(crate::artifacts::curate::dsl::COMPONENT_GRAMMAR_SEMIO),
                    grammar_path: Some(crate::artifacts::curate::dsl::COMPONENT_GRAMMAR_PATH),
                    protocol: Some(crate::artifacts::curate::snapshot::pack::COMPONENT_PROTOCOL_SEMIO),
                    protocol_path: Some(crate::artifacts::curate::snapshot::pack::COMPONENT_PROTOCOL_PATH),
                    hooks: dsl::passthrough_hooks("sourcing.curate"),
                },
                dsl::LanguageSpec {
                    id: "sourcing.curate.op",
                    extension: None,
                    role: dsl::LanguageRole::Ops,
                    grammar: Some(crate::artifacts::curate::op::COMPONENT_GRAMMAR_SEMIO),
                    grammar_path: Some(crate::artifacts::curate::op::COMPONENT_GRAMMAR_PATH),
                    protocol: Some(crate::artifacts::curate::spr::COMPONENT_PROTOCOL_SEMIO),
                    protocol_path: Some(crate::artifacts::curate::spr::COMPONENT_PROTOCOL_PATH),
                    hooks: dsl::passthrough_hooks("sourcing.curate.op"),
                },
                dsl::LanguageSpec {
                    id: "sourcing.curate.diff",
                    extension: None,
                    role: dsl::LanguageRole::Diff,
                    grammar: Some(crate::artifacts::curate::diff::text::COMPONENT_GRAMMAR_SEMIO),
                    grammar_path: Some(crate::artifacts::curate::diff::text::COMPONENT_GRAMMAR_PATH),
                    protocol: None,
                    protocol_path: None,
                    hooks: dsl::passthrough_hooks("sourcing.curate.diff"),
                },
                dsl::LanguageSpec {
                    id: "curate.pack",
                    extension: None,
                    role: dsl::LanguageRole::Pack,
                    grammar: None,
                    grammar_path: None,
                    protocol: Some(crate::artifacts::curate::snapshot::pack::COMPONENT_PROTOCOL_SEMIO),
                    protocol_path: Some(crate::artifacts::curate::snapshot::pack::COMPONENT_PROTOCOL_PATH),
                    hooks: dsl::passthrough_hooks("curate.pack"),
                },
                dsl::LanguageSpec {
                    id: "curate.spr",
                    extension: None,
                    role: dsl::LanguageRole::Spr,
                    grammar: None,
                    grammar_path: None,
                    protocol: Some(crate::artifacts::curate::spr::COMPONENT_PROTOCOL_SEMIO),
                    protocol_path: Some(crate::artifacts::curate::spr::COMPONENT_PROTOCOL_PATH),
                    hooks: dsl::passthrough_hooks("curate.spr"),
                },
            ]
        })
        .as_slice()
}

/// 🔎️ Finds `pilot_languages()`'s entry for one `dsl::LanguageRole` — the lookup `io()` uses to
/// populate each `NativeCodecs` facet's `LanguagePair`.
pub(crate) fn language_spec(role: dsl::LanguageRole) -> Option<&'static dsl::LanguageSpec> {
    pilot_languages().iter().find(|spec| spec.role == role)
}
//#endregion 🔖️Register

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    /// 🗂️ The manifest-facing `ArtifactKindSpec.schema` ("sourcing.curate") is deliberately NOT
    /// `SOURCING_CURATE_SCHEMA` ("sourcing.curate/v1") — the former names the artifact kind in the OS
    /// media catalogue, the latter keys the store envelope. Pinned so a future edit can't silently
    /// merge them (mirrors `flow`'s identical `artifact_kind` split-schema pin).
    #[semio_framework_async_macros::async_test]
    async fn artifact_kind_keeps_the_media_schema_distinct_from_the_store_schema() {
        assert_eq!(artifact_kind().schema, "sourcing.curate");
        assert_eq!(SOURCING_CURATE_SCHEMA, "sourcing.curate/v1");
    }
}
//#endregion 🧪️Tests
