//! 🧬️ Puzzle5d artifact schema — every field of the artifact with its state class.

use crate::{Puzzle5dKindCatalogsExtra, Puzzle5dSnapshot, Puzzle5dTargetVolume};
use ::semio_framework_schema::ArtifactSchema;
use semio_s_artifact_stdio_semio::standards::v1::subsets::kit::schema::snapshot::SemioKitSnapshot;
use std::collections::HashSet;

//#region 🔖️Artifact
/// 🧬️ puzzle5d document artifact state.
#[derive(Clone, Debug, PartialEq, ArtifactSchema, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
#[artifact_schema(id = "s.puzzle.puzzle5d")]
pub struct Puzzle5dArtifact {
    #[state(artifact)]
    pub schema: String,
    #[state(artifact)]
    pub domain: String,
    #[state(artifact)]
    pub label: Option<String>,
    #[state(artifact)]
    pub meta: Puzzle5dMeta,
    #[child(kind = "s.stdio.semio")]
    #[state(artifact)]
    pub kind_catalogs: Option<store::ArtifactChild<SemioKitSnapshot>>,
    #[state(artifact)]
    pub kind_catalogs_extra: Option<Puzzle5dKindCatalogsExtra>,
    #[state(artifact)]
    pub kind_compatibility: Vec<Puzzle5dKindCompatibility>,
    #[state(artifact)]
    pub parts: Vec<Puzzle5dPart>,
    #[state(artifact)]
    pub fasteners: Vec<Puzzle5dFastener>,
    #[state(artifact)]
    pub target_volumes: Vec<Puzzle5dTargetVolume>,
}
//#endregion 🔖️Artifact

//#region 🔖️Conversions
impl Default for Puzzle5dArtifact {
    fn default() -> Self {
        Self::from_snapshot(Puzzle5dSnapshot::default())
    }
}

impl Puzzle5dArtifact {
    /// 📸️ Persisted subset.
    pub fn to_snapshot(&self) -> Puzzle5dSnapshot {
        Puzzle5dSnapshot {
            schema: self.schema.clone(),
            domain: self.domain.clone(),
            label: self.label.clone(),
            meta: self.meta.clone(),
            kind_catalogs: self.kind_catalogs.clone(),
            kind_catalogs_extra: self.kind_catalogs_extra.clone(),
            kind_compatibility: self.kind_compatibility.clone(),
            parts: self.parts.clone(),
            fasteners: self.fasteners.clone(),
            target_volumes: self.target_volumes.clone(),
        }
    }

    /// 🧬️ Builds the document artifact from its snapshot.
    pub fn from_snapshot(snapshot: Puzzle5dSnapshot) -> Self {
        Self {
            schema: snapshot.schema,
            domain: snapshot.domain,
            label: snapshot.label,
            meta: snapshot.meta,
            kind_catalogs: snapshot.kind_catalogs,
            kind_catalogs_extra: snapshot.kind_catalogs_extra,
            kind_compatibility: snapshot.kind_compatibility,
            parts: snapshot.parts,
            fasteners: snapshot.fasteners,
            target_volumes: snapshot.target_volumes,
        }
    }

    /// 🔄 Writes persistent fields from a snapshot into this artifact.
    pub fn set_snapshot(&mut self, snapshot: Puzzle5dSnapshot) {
        self.schema = snapshot.schema;
        self.domain = snapshot.domain;
        self.label = snapshot.label;
        self.meta = snapshot.meta;
        self.kind_catalogs = snapshot.kind_catalogs;
        self.kind_catalogs_extra = snapshot.kind_catalogs_extra;
        self.kind_compatibility = snapshot.kind_compatibility;
        self.parts = snapshot.parts;
        self.fasteners = snapshot.fasteners;
        self.target_volumes = snapshot.target_volumes;
    }
}
//#endregion 🔖️Conversions

//#region 🔖️Descriptor
/// 🧬️ Descriptor for `s.puzzle.puzzle5d` — twenty handcrafted schema leaves.
pub fn puzzle5d_artifact_schema_descriptor() -> ::semio_framework_schema_registry::ArtifactSchemaDescriptor {
    ::semio_framework_schema_registry::ArtifactSchemaDescriptor {
        id: "s.puzzle.puzzle5d",
        artifact: ::semio_framework_schema_registry::FacetLeaves {
            rust: include_str!("🦀️.rs"),
            typescript: include_str!("🟦️.ts"),
            graphql: include_str!("🔗️.graphql"),
            json_schema: include_str!("🔣️.json"),
            proto: include_str!("🛰️.proto"),
        },
        snapshot: ::semio_framework_schema_registry::FacetLeaves {
            rust: include_str!("📸️snapshot/🦀️.rs"),
            typescript: include_str!("📸️snapshot/🟦️.ts"),
            graphql: include_str!("📸️snapshot/🔗️.graphql"),
            json_schema: include_str!("📸️snapshot/🔣️.json"),
            proto: include_str!("📸️snapshot/🛰️.proto"),
        },
        diff: ::semio_framework_schema_registry::FacetLeaves {
            rust: include_str!("🔺️diff/🦀️.rs"),
            typescript: include_str!("🔺️diff/🟦️.ts"),
            graphql: include_str!("🔺️diff/🔗️.graphql"),
            json_schema: include_str!("🔺️diff/🔣️.json"),
            proto: include_str!("🔺️diff/🛰️.proto"),
        },
        mutations: ::semio_framework_schema_registry::FacetLeaves {
            rust: include_str!("🧬️mutations/🦀️.rs"),
            typescript: include_str!("🧬️mutations/🟦️.ts"),
            graphql: include_str!("🧬️mutations/🔗️.graphql"),
            json_schema: include_str!("🧬️mutations/🔣️.json"),
            proto: include_str!("🧬️mutations/🛰️.proto"),
        },
    }
}
//#endregion 🔖️Descriptor
//#region 🏗️DerivedConstruction

//#endregion 🏗️DerivedConstruction

//#region 🧐️DerivedAnalysis

//#endregion 🧐️DerivedAnalysis

//#region 🧬️DerivedArtifactFacets

//#endregion 🧬️DerivedArtifactFacets

//#region 🔖️KindCompatibility
// 🚚️ Relocated from the deleted `⚙️engine` (ticket 26/08/12/ENGINELESS-ARTIFACTS-AND-APP-STATE-MACHINES):
// a pure fn over document/domain data, no app/wasm dependency — a schema-side helper, not engine behaviour.
pub const PUZZLE5D_DEFAULT_MANIFEST_ID: &str = "puzzle5d-default";

/// 🧲️ Looks up whether two grip kinds are compatible per the `puzzle5d-default` manifest's
/// `kindCompatibility` rows — the single shared table both the 2D board and 3D world honor so
/// brush/fill suggestions agree across projections.
pub fn puzzle5d_grip_kinds_compatible(source_kind: &str, target_kind: &str) -> bool {
    let Some(manifest) = crate::graph_manifest::manifest_by_id(PUZZLE5D_DEFAULT_MANIFEST_ID) else {
        return false;
    };
    manifest.kind_compatibility.iter().any(|row| {
        let source = row.get("source").and_then(|value| value.as_str());
        let target = row.get("target").and_then(|value| value.as_str());
        let bidirectional = row.get("bidirectional").and_then(|value| value.as_bool()).unwrap_or(false);
        (source == Some(source_kind) && target == Some(target_kind)) || (bidirectional && source == Some(target_kind) && target == Some(source_kind))
    })
}
//#endregion 🔖️KindCompatibility

//#region 🔖️DerivedDocumentHelpers
// 🚚️ Relocated from the deleted `⚙️engine` — pure document constructors/helpers with no app/wasm
// dependency, consumed by both the app's wasm bridge (`empty_puzzle5d_snapshot`) and the mutations
// binary facet, and by the transfer helpers (`next_id`).
pub fn empty_puzzle5d_snapshot() -> Puzzle5dSnapshot {
    Puzzle5dSnapshot::default()
}

/// 🪪️ Finds the smallest `"{prefix}{n}"` id not already present in `existing`.
pub fn next_id<'a>(existing: impl Iterator<Item = &'a str>, prefix: &str) -> String {
    let ids: HashSet<&str> = existing.collect();
    let mut i = ids.len();
    loop {
        let candidate = format!("{prefix}{i}");
        if !ids.iter().any(|id| *id == candidate) {
            return candidate;
        }
        i += 1;
    }
}
//#endregion 🔖️DerivedDocumentHelpers

//#region 🧪️EngineRelocationTests
#[cfg(test)]
#[path = "🧪️tests/🔬️engine-relocation/🦀️.rs"]
mod engine_relocation_tests;
//#endregion 🧪️EngineRelocationTests

//#region 🔁️Re-exports
/// 🔁️ Entities this module's schema exports and its crate declares elsewhere.
pub use crate::Puzzle5dMeta;
pub use crate::Puzzle5dKindCompatibility;
pub use crate::Puzzle5dPart;
pub use crate::Puzzle5dFastener;
//#endregion 🔁️Re-exports
