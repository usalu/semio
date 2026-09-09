//! 🗂️ Sourcing curation artifact — the document entities this plugin's curation app edits: a catalogue of
//! object kinds (parametric geometry + typology + availability) and a curated selection.

// 🔓️ R7 — `SourcingModule` (schema/🦀️.rs) declares `async fn` methods and is `#[dyn_enum]`-closed
// into `SourcingModules`; Send comes structurally from that concrete enum (R3), so this lint's suggested
// `-> impl Future + Send` fix is never taken, and the method is never made sync to silence it.
#![allow(async_fn_in_trait)]

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
#[cfg(test)]
extern crate semio_framework_os_kernel as vcs;
extern crate semio_framework_schema as framework_schema;

use semio_framework_plugin::{ArtifactKindSpec, Dialect, MediaClass, MediaForm, MediaType, OsMediaCapability, StandardId, SubsetId};
use semio_s_artifact_stdio_semio::standards::v1::subsets::kit::schema::snapshot::{SemioKitSnapshot, SemioKitType};

pub use crate::schema::mutations::SourcingMutation;

pub use crate::schema::diff::CurationDiff;

pub const SOURCING_CURATION_SCHEMA: &str = "sourcing.curation/v1";
pub use crate::schema::snapshot::CurationSnapshot;

/// 🪪️ This artifact's canonical `Dialect` (ticket 26/08/16/ARTIFACT-VIEWERS-AND-EDITORS-PER-SUBSET,
/// contract §1/§7.4) — lives at the ARTIFACT level (not under `editor`/`viewer`) specifically so a
/// viewer file can read it without ever importing through the sibling `editor` module. `artifact_kind
/// = "s.sourcing.curation"` matches the id `definition()`'s own `"s.curation.schema.artifact"` capability
/// row descriptor below; `standard`/`subset` match this file's own
/// `🏅️standards/🔖️1/🪆️subsets/✳️any` location — the canonical surface id is
/// `s.sourcing.curation@1/*#editor` / `s.sourcing.curation@1/*#viewer`, exactly the contract §1 grammar.
pub const SOURCING_DIALECT: Dialect = Dialect { artifact_kind: "s.sourcing.curation", standard: StandardId("1"), subset: SubsetId::ANY };

//#region 🔖️Geometry
/// 📦️ A parametric geometry recipe an object kind is composed of — data describing shape, not a subclass.
#[derive(Clone, Debug, PartialEq, dsl::ToValue, dsl::FromValue, dsl::DslEnum)]
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
#[derive(Clone, Debug, PartialEq, dsl::ToValue, dsl::FromValue, dsl::DslRecord)]
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
#[derive(Clone, Copy, Debug, PartialEq, Eq, dsl::ToValue, dsl::FromValue, dsl::DslScalar)]
#[value(rename_all = "camelCase")]
pub enum SortDirection {
    Asc,
    Desc,
}

#[derive(Clone, Debug, PartialEq, dsl::ToValue, dsl::FromValue, dsl::DslRecord)]
#[value(rename_all = "camelCase")]
pub struct TableSort {
    pub column_id: String,
    pub direction: SortDirection,
}

/// 🔍️ The pool table's active filter set — narrows `CurationSnapshot::stock` down to `filtered_stock()`.
#[derive(Clone, Debug, Default, PartialEq, dsl::ToValue, dsl::FromValue, dsl::DslRecord)]
#[value(rename_all = "camelCase")]
pub struct Filters {
    #[value(default)]
    pub query: String,
    #[value(default)]
    pub module_ids: Vec<String>,
    #[value(default)]
    pub typology_path: Vec<String>,
    #[value(default)]
    pub min_availability: u32,
    #[value(default)]
    #[dsl(block)]
    pub sort: Option<TableSort>,
}

/// 🧺️ One curated object kind and how many units of it have been picked.
#[derive(Clone, Debug, PartialEq, dsl::ToValue, dsl::FromValue, dsl::DslRecord)]
#[value(rename_all = "camelCase")]
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
/// `CurationSnapshot::catalog` child by `id` (see `stock_of`/`object_kind_from_parts`). Ticket
/// 26/08/12/UNIFIED-COMPOSABLE-ARTIFACT-SYSTEM: replaces the former inline `stock: Vec<ObjectKind>`
/// field, which duplicated the `kit.catalog`/type-registry vocabulary this ticket composes instead.
#[derive(Clone, Debug, PartialEq, dsl::ToValue, dsl::FromValue, dsl::DslRecord)]
#[value(rename_all = "camelCase")]
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
pub fn catalog_child_handle(stock: &[ObjectKind]) -> store::ArtifactChild<SemioKitSnapshot> {
    use std::hash::{Hash, Hasher};
    let catalog = catalog_snapshot_from_stock(stock);
    let canonical = dsl::json::to_json_string(&catalog.types);
    let mut hasher = std::collections::hash_map::DefaultHasher::new();
    canonical.hash(&mut hasher);
    let child_id = format!("catalog-{:016x}", hasher.finish());
    let dialect = store::os_io::ArtifactDialect { artifact_kind: "s.stdio.semio".into(), standard: "v1".into(), subset: "kit".into() };
    let target = store::os_io::ArtifactRef { artifact_id: child_id.clone(), dialect };
    store::ArtifactChild::new(child_id, target)
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
//#endregion 🔖️CatalogComposition

//#region 🔖️ArtifactKind
/// 🗂️ This artifact's `ArtifactKindSpec` — stitched into the app manifest by
/// `crate::editor::sourcing::create_sourcing_curation_app`'s `🔖️Manifest` region.
pub fn artifact_kind() -> ArtifactKindSpec {
    ArtifactKindSpec {
        id: "catalogue.sourcing".into(),
        name: "Sourcing Curation".into(),
        source_format: "sourcing.curation".into(),
        component_kind: "catalogue".into(),
        dimension: "data".into(),
        media_capability: OsMediaCapability::MeshOnly,
        media_type: MediaType { class: MediaClass::Kit, form: MediaForm::Kit },
        schema: "sourcing.curation".into(),
        export_formats: vec![],
        import_formats: vec![],
        export_stdio_kinds: vec!["stdio.json".into(), "stdio.obj".into(), "stdio.png".into(), "stdio.stl".into(), "stdio.zip".into()],
        import_stdio_kinds: vec!["stdio.json".into(), "stdio.obj".into(), "stdio.png".into(), "stdio.stl".into(), "stdio.zip".into()],
    }
}
//#endregion 🔖️ArtifactKind

//#region 🔖️Register
/// 🔖️ This artifact's declaration (ticket 26/08/12/ARTIFACTS-ONLY-PLUGIN-ARCHITECTURE M1) — replaces
/// the old side-effecting `register()`, which called four different global registries directly from
/// a plugin `.setup()` callback (a fifth, `crate::io_registry::register()`, was a
/// pure duplicate of what `.composers(...)` now does and was deleted rather than ported — see the
/// mechanism report's `register_all` composer-registration step). `crate::editor::sourcing::config::
/// schema::register_app_schema()` is the one exception, still called from `🪵️sourcing/🦀️.rs`'s
/// own `.setup()`: it registers the `SourcingCurationApp` CONFIG/PRESENCE schema, an app-scope concern
/// `ArtifactDeclaration` deliberately has no field for (see that struct's own doc). Relocated from
/// `⚙️engine` (ticket 26/08/12/ARTIFACTS-ONLY-PLUGIN-ARCHITECTURE reloc-g2): `declaration()` describes
/// the artifact (kind, schema, io ports, ownership), which is not engine behaviour.
pub fn definition() -> Result<semio_framework_plugin::ArtifactDefinition, semio_framework_plugin::ArtifactDefinitionError> {
    use semio_framework_plugin::{ArtifactCapability, ArtifactCapabilityKind, ArtifactDefinition, ArtifactIdentity, ArtifactIdentityClaim, ArtifactIdentityNamespace, ArtifactLocale, ArtifactLocalization};
    let rows: &[semio_framework_plugin::ArtifactCapabilityRow<'_>] = &[
        ("s.sourcing.curation.standard.v1", "standard", "1", &[], None),
        ("s.sourcing.curation.standard.v1.profile.any", "profile", "any", &[], None),
        ("s.sourcing.curation.schema.artifact", "schema", "s.sourcing.curation", &[("schema", "s.sourcing.curation")], None),
        ("s.sourcing.curation.inference.artifact", "inference", "s.sourcing.curation.inference", &[("schema", "s.sourcing.curation.inference")], None),
        ("s.sourcing.curation.composer.zip", "composer", "s.stdio.zip@2.0/*", &[("dialect", "s.stdio.zip@2.0/*")], None),
        ("s.sourcing.curation.composer.png", "composer", "s.stdio.png@1.2/*", &[("dialect", "s.stdio.png@1.2/*")], None),
        ("s.sourcing.curation.composer.json", "composer", "s.stdio.json@rfc8259/*", &[("dialect", "s.stdio.json@rfc8259/*")], None),
        ("s.sourcing.curation.composer.stl", "composer", "s.stdio.stl@ascii/*", &[("dialect", "s.stdio.stl@ascii/*")], None),
        ("s.sourcing.curation.composer.obj", "composer", "s.stdio.obj@3.0/*", &[("dialect", "s.stdio.obj@3.0/*")], None),
        ("s.sourcing.curation.grammar.document", "grammar", "sourcing.curation", &[("grammar", "sourcing.curation")], None),
        ("s.sourcing.curation.grammar.op", "grammar", "sourcing.curation.op", &[("grammar", "sourcing.curation.op")], None),
        ("s.sourcing.curation.grammar.diff", "grammar", "sourcing.curation.diff", &[("grammar", "sourcing.curation.diff")], None),
        ("s.sourcing.curation.grammar.pack", "grammar", "curation.pack", &[("grammar", "curation.pack")], None),
        ("s.sourcing.curation.grammar.spr", "grammar", "curation.spr", &[("grammar", "curation.spr")], None),
        ("s.sourcing.curation.codec.document.v1", "codec", "sourcing.curation/v1:curation", &[("codec", "sourcing.curation/v1"), ("codec-extension", "20:sourcing.curation/v1:curation")], None),
        ("s.sourcing.curation.localization.en", "localization", "Sourcing", &[], Some(("en", "Sourcing"))),
        ("s.sourcing.curation.localization.de", "localization", "Beschaffung", &[], Some(("de", "Beschaffung"))),
    ];
    let mut definition = ArtifactDefinition::new(ArtifactIdentity::parse("s.sourcing.curation")?);
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
pub fn artifact<A: SourcingApplication>() -> semio_framework_plugin::app::declarations::ArtifactDeclaration<A> {
    use semio_framework_plugin::app::declarations::ArtifactDeclaration;
    use store::os_io::ArtifactKindId;
    ArtifactDeclaration { kind: ArtifactKindId::parse("s.sourcing.curation").expect("canonical sourcing.curation kind"), localization: &[], standards: vec![standards::v1::standard()] }
}

/// 🧩️ App fleet capable of hosting this artifact's editor and viewer.
pub trait SourcingApplication:
    semio_framework_plugin::PluginApp
    + From<semio_framework_plugin::VcsArtifactApp<semio_framework_plugin::EditorApp<editor::sourcing::SourcingCurationApp>>>
    + From<semio_framework_plugin::VcsArtifactApp<semio_framework_plugin::ViewerApp<viewer::sourcing::SourcingViewer>>>
{
}

impl<A> SourcingApplication for A where
    A: semio_framework_plugin::PluginApp
        + From<semio_framework_plugin::VcsArtifactApp<semio_framework_plugin::EditorApp<editor::sourcing::SourcingCurationApp>>>
        + From<semio_framework_plugin::VcsArtifactApp<semio_framework_plugin::ViewerApp<viewer::sourcing::SourcingViewer>>>
{
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
                    id: "sourcing.curation",
                    extension: Some("curation"),
                    role: dsl::LanguageRole::Document,
                    grammar: Some(document_dsl::COMPONENT_GRAMMAR_SEMIO),
                    grammar_path: Some(document_dsl::COMPONENT_GRAMMAR_PATH),
                    protocol: Some(snapshot::pack::COMPONENT_PROTOCOL_SEMIO),
                    protocol_path: Some(snapshot::pack::COMPONENT_PROTOCOL_PATH),
                    hooks: dsl::passthrough_hooks("sourcing.curation"),
                },
                dsl::LanguageSpec {
                    id: "sourcing.curation.op",
                    extension: None,
                    role: dsl::LanguageRole::Ops,
                    grammar: Some(op::COMPONENT_GRAMMAR_SEMIO),
                    grammar_path: Some(op::COMPONENT_GRAMMAR_PATH),
                    protocol: Some(spr::COMPONENT_PROTOCOL_SEMIO),
                    protocol_path: Some(spr::COMPONENT_PROTOCOL_PATH),
                    hooks: dsl::passthrough_hooks("sourcing.curation.op"),
                },
                dsl::LanguageSpec {
                    id: "sourcing.curation.diff",
                    extension: None,
                    role: dsl::LanguageRole::Diff,
                    grammar: Some(diff::text::COMPONENT_GRAMMAR_SEMIO),
                    grammar_path: Some(diff::text::COMPONENT_GRAMMAR_PATH),
                    protocol: None,
                    protocol_path: None,
                    hooks: dsl::passthrough_hooks("sourcing.curation.diff"),
                },
                dsl::LanguageSpec {
                    id: "curation.pack",
                    extension: None,
                    role: dsl::LanguageRole::Pack,
                    grammar: None,
                    grammar_path: None,
                    protocol: Some(snapshot::pack::COMPONENT_PROTOCOL_SEMIO),
                    protocol_path: Some(snapshot::pack::COMPONENT_PROTOCOL_PATH),
                    hooks: dsl::passthrough_hooks("curation.pack"),
                },
                dsl::LanguageSpec {
                    id: "curation.spr",
                    extension: None,
                    role: dsl::LanguageRole::Spr,
                    grammar: None,
                    grammar_path: None,
                    protocol: Some(spr::COMPONENT_PROTOCOL_SEMIO),
                    protocol_path: Some(spr::COMPONENT_PROTOCOL_PATH),
                    hooks: dsl::passthrough_hooks("curation.spr"),
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
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests

#[path = "."]
        pub mod standards {
            #[path = "."]
            pub mod v1 {
                #[path = "🏅️standards/🔖️1/🦀️.rs"]
                mod component;
                pub use component::*;
                #[path = "."]
                pub mod subsets {
                    #[path = "."]
                    pub mod any {
                        #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🦀️.rs"]
                        mod component;
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
                            }
                            #[path = "."]
                            pub mod inferences {
                                #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🦀️.rs"]
                                mod component;
                                pub use component::*;
                                #[path = "."]
                                pub mod entries {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🗃️entries/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                }
                            }
                            #[path = "."]
                            pub mod diff {
                                #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/🦀️.rs"]
                                mod component;
                                pub use component::*;
                            }
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/⚙️operations/🦀️.rs"]
                            pub mod operations;
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
                                pub mod create_curated_item {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌱create-curated-item/🦀️.rs"]
                                    mod component;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌱create-curated-item/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌱create-curated-item/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    pub use component::*;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌱create-curated-item/💾️binary/🦀️.rs"]
                                    pub mod binary;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌱create-curated-item/🧪️tests/🧲️appends-a-steel-plate-to-the-curation/🦀️.rs"]
                                    mod tests_appends_a_steel_plate_to_the_curation;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌱create-curated-item/📝️text/🦀️.rs"]
                                    pub mod text;
                                }
                                #[path = "."]
                                pub mod delete_curated_item {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🗑️delete-curated-item/🦀️.rs"]
                                    mod component;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🗑️delete-curated-item/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🗑️delete-curated-item/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    pub use component::*;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🗑️delete-curated-item/💾️binary/🦀️.rs"]
                                    pub mod binary;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🗑️delete-curated-item/🧪️tests/🚫️removes-the-clt-panel-from-the-curation/🦀️.rs"]
                                    mod tests_removes_the_clt_panel_from_the_curation;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🗑️delete-curated-item/📝️text/🦀️.rs"]
                                    pub mod text;
                                }
                                #[path = "."]
                                pub mod change_curated_item_count {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔢change-curated-item-count/🦀️.rs"]
                                    mod component;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔢change-curated-item-count/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔢change-curated-item-count/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    pub use component::*;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔢change-curated-item-count/💾️binary/🦀️.rs"]
                                    pub mod binary;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔢change-curated-item-count/🧪️tests/🔢️raises-the-glulam-beam-count-to-20/🦀️.rs"]
                                    mod tests_raises_the_glulam_beam_count_to_20;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔢change-curated-item-count/📝️text/🦀️.rs"]
                                    pub mod text;
                                }
                            }
                        }
                        #[path = "."]
                        pub mod io {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "."]
                            pub mod snapshot {
                                #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📸️snapshot/💾️binary/🦀️.rs"]
                                pub mod binary;
                                #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📸️snapshot/📝️text/🦀️.rs"]
                                pub mod text;
                            }
                            #[path = "."]
                            pub mod diff {
                                #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🔺️diff/💾️binary/🦀️.rs"]
                                pub mod binary;
                                #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🔺️diff/📝️text/🦀️.rs"]
                                pub mod text;
                            }
                            #[path = "."]
                            pub mod mutations {
                                #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🧬️mutations/💾️binary/🦀️.rs"]
                                pub mod binary;
                                #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🧬️mutations/📝️text/🦀️.rs"]
                                pub mod text;
                            }
                            #[path = "."]
                            pub mod inferences {
                                #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/💡️inferences/💾️binary/🦀️.rs"]
                                pub mod binary;
                                #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/💡️inferences/📝️text/🦀️.rs"]
                                pub mod text;
                            }
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

        // ---- Shims: keep pre-migration module paths resolving for external callers ----
        pub mod schema {
            pub use super::standards::v1::subsets::any::schema::*;
        }
        pub mod io {
            pub use super::standards::v1::subsets::any::io::*;
        }
        pub mod op {
            pub use crate::standards::v1::subsets::any::io::mutations::text::*;
        }
        pub mod document_dsl {
            pub use crate::standards::v1::subsets::any::io::snapshot::text::*;
        }
        pub mod spr {
            pub use crate::standards::v1::subsets::any::io::mutations::binary::*;
        }
        pub mod diff {
            pub use crate::standards::v1::subsets::any::schema::diff::*;
            pub mod schema {
                pub use crate::standards::v1::subsets::any::schema::diff::*;
            }
            pub mod text {
                pub use crate::standards::v1::subsets::any::io::diff::text::*;
            }
            pub mod pack {
                pub use crate::standards::v1::subsets::any::io::diff::binary::*;
            }
            pub mod binary {
                pub use crate::standards::v1::subsets::any::io::diff::binary::*;
            }
        }
        pub mod mutations {
            pub use crate::standards::v1::subsets::any::schema::mutations::*;
            pub mod schema {
                pub use crate::standards::v1::subsets::any::schema::mutations::*;
            }
            pub mod text {
                pub use crate::standards::v1::subsets::any::io::mutations::text::*;
            }
            pub mod pack {
                pub use crate::standards::v1::subsets::any::io::mutations::binary::*;
            }
            pub mod binary {
                pub use crate::standards::v1::subsets::any::io::mutations::binary::*;
            }
        }
        pub mod snapshot {
            pub use crate::standards::v1::subsets::any::schema::snapshot::*;
            pub mod schema {
                pub use crate::standards::v1::subsets::any::schema::snapshot::*;
            }
            pub mod text {
                pub use crate::standards::v1::subsets::any::io::snapshot::text::*;
            }
            pub mod pack {
                pub use crate::standards::v1::subsets::any::io::snapshot::binary::*;
            }
            pub mod binary {
                pub use crate::standards::v1::subsets::any::io::snapshot::binary::*;
            }
        }

        #[path = "."]
        pub mod examples {
            #[path = "."]
            pub mod demo {
                #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🎬️demo/🦀️.rs"]
                mod component;
                pub use component::*;
            }
        }

#[path = "."]
pub mod editor {
    #[path = "."]
    pub mod sourcing {
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

        #[path = "."]
        pub mod commands {
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/➕️curation-add/🦀️.rs"]
            pub mod curation_add;
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/➖️curation-remove/🦀️.rs"]
            pub mod curation_remove;
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🔢️curation-set-count/🦀️.rs"]
            pub mod curation_set_count;
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🧺️drop-on-curated/🦀️.rs"]
            pub mod drop_on_curated;
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🏊️drop-on-pool/🦀️.rs"]
            pub mod drop_on_pool;
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🎬️set-active-example/🦀️.rs"]
            pub mod set_active_example;
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🗿️set-artifact-json/🦀️.rs"]
            pub mod set_artifact_json;
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🧩️set-contributions/🦀️.rs"]
            pub mod set_contributions;
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/📉️set-filter-min-availability/🦀️.rs"]
            pub mod set_filter_min_availability;
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🧱️set-filter-module/🦀️.rs"]
            pub mod set_filter_module;
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🔎️set-filter-query/🦀️.rs"]
            pub mod set_filter_query;
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🏛️set-filter-typology/🦀️.rs"]
            pub mod set_filter_typology;
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/↕️sort-table/🦀️.rs"]
            pub mod sort_table;
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/📇️stock-from-catalogue/🦀️.rs"]
            pub mod stock_from_catalogue;
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
                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/🧺️curated/🦀️.rs"]
                    pub mod curated;
                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/🔢️grid/🦀️.rs"]
                    pub mod grid;
                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/🏊️pool/🦀️.rs"]
                    pub mod pool;
                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/👁️preview/🦀️.rs"]
                    pub mod preview;
                }
            }
        }
    }
}

#[path = "."]
pub mod viewer {
    #[path = "."]
    pub mod sourcing {
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
                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/👁️viewer/🎭️modes/👁️view/🪟️windows/🏊️pool/🦀️.rs"]
                    pub mod pool;
                }
            }
        }
    }
}
