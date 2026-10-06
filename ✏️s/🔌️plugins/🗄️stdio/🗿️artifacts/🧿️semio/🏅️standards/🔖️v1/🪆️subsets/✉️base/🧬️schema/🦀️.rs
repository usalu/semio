//! 🧬️ SemioArtifact schema — full artifact state, mirrors `SemioSnapshot` field for
//! field (see gif's `GifArtifact` for the precedent this follows). 🚧 scaffolded by W1b.

use crate::standards::v1::subsets::base::schema::snapshot::{SemioSnapshot, SemioSubsetSnapshot};
use framework_schema::ArtifactSchema;

#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, ArtifactSchema)]
#[value(rename_all = "camelCase")]
#[artifact_schema(id = "s.stdio.semio")]
pub struct SemioArtifact {
    #[state(artifact)]
    pub schema: String,
    #[state(artifact)]
    pub subset: SemioSubsetSnapshot,
}

impl Default for SemioArtifact {
    fn default() -> Self {
        Self::from_snapshot(SemioSnapshot::default())
    }
}

impl SemioArtifact {
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn to_snapshot(&self) -> SemioSnapshot {
        SemioSnapshot { schema: self.schema.clone(), subset: self.subset.clone() }
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn from_snapshot(snapshot: SemioSnapshot) -> Self {
        Self { schema: snapshot.schema, subset: snapshot.subset }
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn set_snapshot(&mut self, snapshot: SemioSnapshot) {
        self.schema = snapshot.schema;
        self.subset = snapshot.subset;
    }
}

/// 📚️ semio's shared schema documents — named exports of the `s.stdio.semio` scope that the subsets' facets `$ref`
/// (`base/geometry.json`, `base/child.json`), declared with the artifact (`ArtifactDeclarationBuilder::schema_documents`).
pub const SEMIO_SHARED_SCHEMA_DOCUMENTS: semio_framework_schema_registry::ScopeSchemaExports = semio_framework_schema_registry::ScopeSchemaExports {
    scope: "s.stdio.semio",
    exports: &[semio_framework_schema_registry::SchemaExport { id: "brep-inference", leaves: semio_framework_schema_registry::FacetLeaves { rust: include_str!("../../🧊️brep/🧬️schema/💡️inferences/🦀️.rs"), typescript: include_str!("../../🧊️brep/🧬️schema/💡️inferences/🟦️.ts"), graphql: include_str!("../../🧊️brep/🧬️schema/💡️inferences/🔗️.graphql"), json_schema: include_str!("../../🧊️brep/🧬️schema/💡️inferences/🔣️.json"), proto: include_str!("../../🧊️brep/🧬️schema/💡️inferences/🛰️.proto") } }, semio_framework_schema_registry::SchemaExport { id: "geometry", leaves: semio_framework_schema_registry::FacetLeaves { rust: include_str!("🧮️geometry/🦀️.rs"), typescript: include_str!("🧮️geometry/🟦️.ts"), graphql: include_str!("🧮️geometry/🔗️.graphql"), json_schema: include_str!("🧮️geometry/🔣️.json"), proto: include_str!("🧮️geometry/🛰️.proto") } }, semio_framework_schema_registry::SchemaExport { id: "child", leaves: semio_framework_schema_registry::FacetLeaves { rust: include_str!("🪆️child/🦀️.rs"), typescript: include_str!("🪆️child/🟦️.ts"), graphql: "", json_schema: include_str!("🪆️child/🔣️.json"), proto: "" } }],
};

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn semio_artifact_schema_descriptor() -> semio_framework_schema_registry::ArtifactSchemaDescriptor {
    semio_framework_schema_registry::ArtifactSchemaDescriptor {
        id: "s.stdio.semio",
        artifact: semio_framework_schema_registry::FacetLeaves { rust: include_str!("🦀️.rs"), typescript: include_str!("🟦️.ts"), graphql: include_str!("🔗️.graphql"), json_schema: include_str!("🔣️.json"), proto: include_str!("🛰️.proto") },
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
//#region 🏗️DerivedConstruction

//#endregion 🏗️DerivedConstruction

//#region 🧐️DerivedAnalysis

//#endregion 🧐️DerivedAnalysis

//#region 🧬️DerivedArtifactFacets

//#endregion 🧬️DerivedArtifactFacets
