//! 🗂️ Sourcing curate artifact — the document entities this plugin's curate app edits: a catalogue of
//! object kinds (parametric geometry + typology + availability) and a curated selection.

use semio_framework_plugin::{ArtifactKindSpec, MediaClass, MediaForm, MediaType, OsMediaCapability};
use serde::{Deserialize, Serialize};

pub use crate::artifacts::curate::schema::mutations::SourcingMutation;

pub use crate::artifacts::curate::schema::diff::CurateDiff;

pub const SOURCING_CURATE_SCHEMA: &str = "sourcing.curate/v1";
pub use crate::artifacts::curate::schema::snapshot::CurateSnapshot;

//#region 🔖️Geometry
/// 📦️ A parametric geometry recipe an object kind is composed of — data describing shape, not a subclass.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslEnum)]
#[serde(tag = "kind", rename_all = "camelCase")]
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
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
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

//#region 🔖️ArtifactKind
/// 🗂️ This artifact's `ArtifactKindSpec` — stitched into the app manifest by
/// `crate::apps::curate::create_sourcing_curate_app`'s `🔖️Manifest` region.
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

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    /// 🗂️ The manifest-facing `ArtifactKindSpec.schema` ("sourcing.curate") is deliberately NOT
    /// `SOURCING_CURATE_SCHEMA` ("sourcing.curate/v1") — the former names the artifact kind in the OS
    /// media catalogue, the latter keys the store envelope. Pinned so a future edit can't silently
    /// merge them (mirrors `flow`'s identical `artifact_kind` split-schema pin).
    #[test]
    fn artifact_kind_keeps_the_media_schema_distinct_from_the_store_schema() {
        assert_eq!(artifact_kind().schema, "sourcing.curate");
        assert_eq!(SOURCING_CURATE_SCHEMA, "sourcing.curate/v1");
    }
}
//#endregion 🧪️Tests
//#region 🚪️DerivedIoRegistry
pub mod io_registry {
    use std::sync::OnceLock;
    use semio_framework_plugin::{ComposerEntry, Dialect, ErasedComposeSource, ComposedArtifact, ComposeError, register_composer_entries};
    use crate::artifacts::curate::standards::v1::engine::io_registry as v1;

    static ENTRIES: OnceLock<Vec<&'static ComposerEntry>> = OnceLock::new();

    pub fn entries() -> &'static [&'static ComposerEntry] {
        ENTRIES.get_or_init(|| v1::entries().iter().collect()).as_slice()
    }

    pub fn compose(target: Dialect, sources: &[ErasedComposeSource]) -> Result<ComposedArtifact, ComposeError> {
        let entry = entries()
            .iter()
            .find(|e| e.writes == target)
            .ok_or_else(|| ComposeError { message: format!("CurateComposer: no entry writes {:?}", target), diagnostics: Vec::new() })?;
        (entry.compose)(sources)
    }

    pub fn register() {
        register_composer_entries(v1::entries());
    }
}
//#endregion 🚪️DerivedIoRegistry
