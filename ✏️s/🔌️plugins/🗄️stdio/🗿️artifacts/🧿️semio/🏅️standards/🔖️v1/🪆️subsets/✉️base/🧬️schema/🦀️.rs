//! 🧬️ SemioArtifact schema — full artifact state, mirrors `SemioSnapshot` field for
//! field (see gif's `GifArtifact` for the precedent this follows). 🚧 scaffolded by W1b.

use crate::schema::snapshot::{SemioSnapshot, SemioSubsetSnapshot};
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
    pub fn to_snapshot(&self) -> SemioSnapshot {
        SemioSnapshot { schema: self.schema.clone(), subset: self.subset.clone() }
    }
    pub fn from_snapshot(snapshot: SemioSnapshot) -> Self {
        Self { schema: snapshot.schema, subset: snapshot.subset }
    }
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
    ::semio_framework_schema_registry::register_schema_leaf_documents(&[
        ::semio_framework_schema_registry::SchemaLeafDocument { scope: "s.stdio.semio", export: "artifact", format: ::semio_framework_schema_registry::SchemaFormat::Typescript, path: "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✉️base/🧬️schema/🟦️.ts" },
        ::semio_framework_schema_registry::SchemaLeafDocument { scope: "s.stdio.semio", export: "artifact", format: ::semio_framework_schema_registry::SchemaFormat::Graphql, path: "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✉️base/🧬️schema/🔗️.graphql" },
        ::semio_framework_schema_registry::SchemaLeafDocument { scope: "s.stdio.semio", export: "artifact", format: ::semio_framework_schema_registry::SchemaFormat::Protobuf, path: "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✉️base/🧬️schema/🛰️.proto" },
        ::semio_framework_schema_registry::SchemaLeafDocument { scope: "s.stdio.semio", export: "snapshot", format: ::semio_framework_schema_registry::SchemaFormat::Typescript, path: "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✉️base/🧬️schema/📸️snapshot/🟦️.ts" },
        ::semio_framework_schema_registry::SchemaLeafDocument { scope: "s.stdio.semio", export: "snapshot", format: ::semio_framework_schema_registry::SchemaFormat::Graphql, path: "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✉️base/🧬️schema/📸️snapshot/🔗️.graphql" },
        ::semio_framework_schema_registry::SchemaLeafDocument { scope: "s.stdio.semio", export: "snapshot", format: ::semio_framework_schema_registry::SchemaFormat::Protobuf, path: "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✉️base/🧬️schema/📸️snapshot/🛰️.proto" },
        ::semio_framework_schema_registry::SchemaLeafDocument { scope: "s.stdio.semio", export: "diff", format: ::semio_framework_schema_registry::SchemaFormat::Typescript, path: "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✉️base/🧬️schema/🔺️diff/🟦️.ts" },
        ::semio_framework_schema_registry::SchemaLeafDocument { scope: "s.stdio.semio", export: "diff", format: ::semio_framework_schema_registry::SchemaFormat::Graphql, path: "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✉️base/🧬️schema/🔺️diff/🔗️.graphql" },
        ::semio_framework_schema_registry::SchemaLeafDocument { scope: "s.stdio.semio", export: "diff", format: ::semio_framework_schema_registry::SchemaFormat::Protobuf, path: "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✉️base/🧬️schema/🔺️diff/🛰️.proto" },
        ::semio_framework_schema_registry::SchemaLeafDocument { scope: "s.stdio.semio", export: "mutations", format: ::semio_framework_schema_registry::SchemaFormat::Typescript, path: "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✉️base/🧬️schema/🧬️mutations/🟦️.ts" },
        ::semio_framework_schema_registry::SchemaLeafDocument { scope: "s.stdio.semio", export: "mutations", format: ::semio_framework_schema_registry::SchemaFormat::Graphql, path: "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✉️base/🧬️schema/🧬️mutations/🔗️.graphql" },
        ::semio_framework_schema_registry::SchemaLeafDocument { scope: "s.stdio.semio", export: "mutations", format: ::semio_framework_schema_registry::SchemaFormat::Protobuf, path: "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✉️base/🧬️schema/🧬️mutations/🛰️.proto" },
    ]);
    semio_framework_schema_registry::ArtifactSchemaDescriptor {
        id: "s.stdio.semio",
        artifact: semio_framework_schema_registry::FacetLeaves { rust: include_str!("🦀️.rs"), typescript: "", graphql: "", json_schema: include_str!("🔣️.json"), proto: "" },
        snapshot: semio_framework_schema_registry::FacetLeaves {
            rust: include_str!("📸️snapshot/🦀️.rs"),
            typescript: "",
            graphql: "",
            json_schema: include_str!("📸️snapshot/🔣️.json"),
            proto: "",
        },
        diff: semio_framework_schema_registry::FacetLeaves {
            rust: include_str!("🔺️diff/🦀️.rs"),
            typescript: "",
            graphql: "",
            json_schema: include_str!("🔺️diff/🔣️.json"),
            proto: "",
        },
        mutations: semio_framework_schema_registry::FacetLeaves {
            rust: include_str!("🧬️mutations/🦀️.rs"),
            typescript: "",
            graphql: "",
            json_schema: include_str!("🧬️mutations/🔣️.json"),
            proto: "",
        },
    }
}
//#region 🏗️DerivedConstruction

//#endregion 🏗️DerivedConstruction

//#region 🧐️DerivedAnalysis

//#endregion 🧐️DerivedAnalysis

//#region 🧬️DerivedArtifactFacets

//#endregion 🧬️DerivedArtifactFacets
