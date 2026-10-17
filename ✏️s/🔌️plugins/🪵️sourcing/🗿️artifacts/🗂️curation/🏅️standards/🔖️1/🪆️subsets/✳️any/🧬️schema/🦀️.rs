//! 🧬️ Curation artifact schema — every field of the artifact with its state class.

use crate::{CurationSnapshot, SourcingMutation};
use framework_schema::ArtifactSchema;
use semio_framework_plugin::world3d_mesh_id_from_url;
use semio_framework_dispatch_macros::{dyn_enum, dyn_enum_close};
use semio_s_artifact_stdio_semio_kit::schema::snapshot::SemioKitSnapshot;

//#region 🔖️Artifact
/// 🧬️ curation document artifact state. `catalog`/
/// `stock_extra` mirror `CurationSnapshot`'s own composed-child split (see that struct's doc comment).
#[derive(Clone, Debug, PartialEq, semio_framework_value::ToValue, ArtifactSchema, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
#[value(rename_all = "camelCase")]
#[artifact_schema(id = "s.sourcing.curation")]
pub struct CurationArtifact {
    #[state(artifact)]
    #[child(kind = "s.stdio.semio")]
    pub catalog: store::os_store::artifact_child::ArtifactChild<SemioKitSnapshot>,
    #[state(artifact)]
    pub stock_extra: Vec<ObjectKindExtra>,
    #[state(artifact)]
    pub curated: Vec<CuratedItem>,
}
impl semio_framework_value::FromValue for CurationArtifact {
    fn from_value(value: semio_framework_value::DslValue) -> Result<Self, semio_framework_value::ValueError> {
        <CurationSnapshot as semio_framework_value::FromValue>::from_value(value).map(Self::from_snapshot)
    }
}
//#endregion 🔖️Artifact

//#region 🔖️Conversions
impl Default for CurationArtifact {
    fn default() -> Self {
        Self { catalog: crate::catalog_child_handle(&[]), stock_extra: Vec::new(), curated: Vec::new() }
    }
}

impl CurationArtifact {
    /// 📸️ Persisted subset.
    pub fn to_snapshot(&self) -> CurationSnapshot {
        CurationSnapshot { catalog: self.catalog.clone(), stock_extra: self.stock_extra.clone(), curated: self.curated.clone() }
    }

    /// 🧬️ Builds the document artifact from its snapshot.
    pub fn from_snapshot(snapshot: CurationSnapshot) -> Self {
        Self { catalog: snapshot.catalog, stock_extra: snapshot.stock_extra, curated: snapshot.curated }
    }

    /// 🔄 Writes persistent fields from a snapshot into this artifact.
    pub fn set_snapshot(&mut self, snapshot: CurationSnapshot) {
        self.catalog = snapshot.catalog;
        self.stock_extra = snapshot.stock_extra;
        self.curated = snapshot.curated;
    }
}
//#endregion 🔖️Conversions

//#region 🔖️Descriptor
/// 🧬️ Descriptor for `s.sourcing.curation` — twenty handcrafted schema leaves.
pub fn curation_artifact_schema_descriptor() -> semio_framework_schema_registry::ArtifactSchemaDescriptor {
    semio_framework_schema_registry::ArtifactSchemaDescriptor {
        id: "s.sourcing.curation",
        artifact: semio_framework_schema_registry::FacetLeaves {
            rust: include_str!("🦀️.rs"),
            typescript: include_str!("🟦️.ts"),
            graphql: include_str!("🔗️.graphql"),
            json_schema: include_str!("🔣️.json"),
            proto: include_str!("🛰️.proto"),
        },
        snapshot: semio_framework_schema_registry::FacetLeaves {
            rust: include_str!("📸️snapshot/🦀️.rs"),
            typescript: include_str!("📸️snapshot/🟦️.ts"),
            graphql: include_str!("📸️snapshot/🔗️.graphql"),
            json_schema: include_str!("📸️snapshot/🔣️.json"),
            proto: include_str!("📸️snapshot/🛰️.proto"),
        },
        diff: semio_framework_schema_registry::FacetLeaves {
            rust: include_str!("🔺️diff/🦀️.rs"),
            typescript: include_str!("🔺️diff/🟦️.ts"),
            graphql: include_str!("🔺️diff/🔗️.graphql"),
            json_schema: include_str!("🔺️diff/🔣️.json"),
            proto: include_str!("🔺️diff/🛰️.proto"),
        },
        mutations: semio_framework_schema_registry::FacetLeaves {
            rust: include_str!("🧬️mutations/🦀️.rs"),
            typescript: include_str!("🧬️mutations/🟦️.ts"),
            graphql: include_str!("🧬️mutations/🔗️.graphql"),
            json_schema: include_str!("🧬️mutations/🔣️.json"),
            proto: include_str!("🧬️mutations/🛰️.proto"),
        },
    }
}
//#endregion 🔖️Descriptor


//#region 🏗️Construction
/// 🏗️ W1-C's generic `SnapshotBuilder<Snapshot, Mutation>` (design.md §5 step 3) — replaces the
/// deleted `derive_artifact_facets!`-generated `CurationBuilder`/`CurationAnalyzer`/`CurationComposer`
/// cluster (and the hand-rolled `CurationBuilderConstruction`/`CurationAnalyzerAnalysis` it wrapped)
/// outright: construction is a plain snapshot+mutation build (no custom analysis/composition logic
/// this subset needs beyond the ordinary `Mutation`/`MutationDiff` algebra), so the trivial-subset
/// shape applies verbatim.
pub type Construction = semio_framework_plugin::app::SnapshotBuilder<CurationSnapshot, SourcingMutation>;
//#endregion 🏗️Construction

//#region 🔁️Domain
/// 🔁️ The domain module of this schema, exported at the schema path.
pub use crate::standards::v1::subsets::any::schema::domain::*;
//#endregion 🔁️Domain

//#region 🔁️Re-exports
/// 🔁️ Entities this module's schema exports and its crate declares elsewhere.
pub use crate::GeometryRecipe;
pub use crate::ObjectKind;
pub use crate::ObjectKindExtra;
pub use crate::CuratedItem;
pub use crate::Filters;
//#endregion 🔁️Re-exports
