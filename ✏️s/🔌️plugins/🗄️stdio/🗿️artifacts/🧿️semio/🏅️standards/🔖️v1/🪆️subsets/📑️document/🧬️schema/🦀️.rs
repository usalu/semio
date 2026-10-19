//! 🧬️ SemioDocumentArtifact schema — full artifact state, mirrors `SemioDocumentSnapshot` field for
//! field (see gif's `GifArtifact` for the precedent this follows).

use crate::schema::snapshot::{DocBlock, DocImage, DocStyle, SemioDocumentSnapshot};
use framework_schema::ArtifactSchema;

#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, ArtifactSchema)]
#[value(rename_all = "camelCase")]
#[artifact_schema(id = "s.stdio.semio.document")]
pub struct SemioDocumentArtifact {
    #[state(artifact)]
    pub schema: String,
    #[state(artifact)]
    #[value(default)]
    pub styles: Vec<DocStyle>,
    #[state(artifact)]
    #[value(default)]
    pub images: Vec<DocImage>,
    #[state(artifact)]
    #[value(default)]
    pub blocks: Vec<DocBlock>,
}

impl Default for SemioDocumentArtifact {
    fn default() -> Self {
        Self::from_snapshot(SemioDocumentSnapshot::default())
    }
}

impl SemioDocumentArtifact {
    pub fn to_snapshot(&self) -> SemioDocumentSnapshot {
        SemioDocumentSnapshot { schema: self.schema.clone(), styles: self.styles.clone(), images: self.images.clone(), blocks: self.blocks.clone() }
    }
    pub fn from_snapshot(snapshot: SemioDocumentSnapshot) -> Self {
        Self { schema: snapshot.schema, styles: snapshot.styles, images: snapshot.images, blocks: snapshot.blocks }
    }
    pub fn set_snapshot(&mut self, snapshot: SemioDocumentSnapshot) {
        self.schema = snapshot.schema;
        self.styles = snapshot.styles;
        self.images = snapshot.images;
        self.blocks = snapshot.blocks;
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn semio_document_artifact_schema_descriptor() -> semio_framework_schema_registry::ArtifactSchemaDescriptor {
    ::semio_framework_schema_registry::register_schema_leaf_documents(&[
        ::semio_framework_schema_registry::SchemaLeafDocument { scope: "s.stdio.semio.document", export: "artifact", format: ::semio_framework_schema_registry::SchemaFormat::Rust, path: "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/📑️document/🧬️schema/🦀️.rs" },
        ::semio_framework_schema_registry::SchemaLeafDocument { scope: "s.stdio.semio.document", export: "artifact", format: ::semio_framework_schema_registry::SchemaFormat::Typescript, path: "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/📑️document/🧬️schema/🟦️.ts" },
        ::semio_framework_schema_registry::SchemaLeafDocument { scope: "s.stdio.semio.document", export: "artifact", format: ::semio_framework_schema_registry::SchemaFormat::Graphql, path: "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/📑️document/🧬️schema/🔗️.graphql" },
        ::semio_framework_schema_registry::SchemaLeafDocument { scope: "s.stdio.semio.document", export: "artifact", format: ::semio_framework_schema_registry::SchemaFormat::Protobuf, path: "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/📑️document/🧬️schema/🛰️.proto" },
        ::semio_framework_schema_registry::SchemaLeafDocument { scope: "s.stdio.semio.document", export: "snapshot", format: ::semio_framework_schema_registry::SchemaFormat::Rust, path: "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/📑️document/🧬️schema/📸️snapshot/🦀️.rs" },
        ::semio_framework_schema_registry::SchemaLeafDocument { scope: "s.stdio.semio.document", export: "snapshot", format: ::semio_framework_schema_registry::SchemaFormat::Typescript, path: "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/📑️document/🧬️schema/📸️snapshot/🟦️.ts" },
        ::semio_framework_schema_registry::SchemaLeafDocument { scope: "s.stdio.semio.document", export: "snapshot", format: ::semio_framework_schema_registry::SchemaFormat::Graphql, path: "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/📑️document/🧬️schema/📸️snapshot/🔗️.graphql" },
        ::semio_framework_schema_registry::SchemaLeafDocument { scope: "s.stdio.semio.document", export: "snapshot", format: ::semio_framework_schema_registry::SchemaFormat::Protobuf, path: "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/📑️document/🧬️schema/📸️snapshot/🛰️.proto" },
        ::semio_framework_schema_registry::SchemaLeafDocument { scope: "s.stdio.semio.document", export: "diff", format: ::semio_framework_schema_registry::SchemaFormat::Rust, path: "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/📑️document/🧬️schema/🔺️diff/🦀️.rs" },
        ::semio_framework_schema_registry::SchemaLeafDocument { scope: "s.stdio.semio.document", export: "diff", format: ::semio_framework_schema_registry::SchemaFormat::Rust, path: "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/📑️document/🧬️schema/🔺️diff/🧬️diff-types/🦀️.rs" },
        ::semio_framework_schema_registry::SchemaLeafDocument { scope: "s.stdio.semio.document", export: "diff", format: ::semio_framework_schema_registry::SchemaFormat::Rust, path: "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/📑️document/🧬️schema/🔺️diff/⚙️indexed-and-named-engine/🦀️.rs" },
        ::semio_framework_schema_registry::SchemaLeafDocument { scope: "s.stdio.semio.document", export: "diff", format: ::semio_framework_schema_registry::SchemaFormat::Rust, path: "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/📑️document/🧬️schema/🔺️diff/🖊️diff-logic-and-algebra/🦀️.rs" },
        ::semio_framework_schema_registry::SchemaLeafDocument { scope: "s.stdio.semio.document", export: "diff", format: ::semio_framework_schema_registry::SchemaFormat::Typescript, path: "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/📑️document/🧬️schema/🔺️diff/🟦️.ts" },
        ::semio_framework_schema_registry::SchemaLeafDocument { scope: "s.stdio.semio.document", export: "diff", format: ::semio_framework_schema_registry::SchemaFormat::Graphql, path: "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/📑️document/🧬️schema/🔺️diff/🔗️.graphql" },
        ::semio_framework_schema_registry::SchemaLeafDocument { scope: "s.stdio.semio.document", export: "diff", format: ::semio_framework_schema_registry::SchemaFormat::Protobuf, path: "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/📑️document/🧬️schema/🔺️diff/🛰️.proto" },
        ::semio_framework_schema_registry::SchemaLeafDocument { scope: "s.stdio.semio.document", export: "mutations", format: ::semio_framework_schema_registry::SchemaFormat::Rust, path: "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/📑️document/🧬️schema/🧬️mutations/🦀️.rs" },
        ::semio_framework_schema_registry::SchemaLeafDocument { scope: "s.stdio.semio.document", export: "mutations", format: ::semio_framework_schema_registry::SchemaFormat::Typescript, path: "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/📑️document/🧬️schema/🧬️mutations/🟦️.ts" },
        ::semio_framework_schema_registry::SchemaLeafDocument { scope: "s.stdio.semio.document", export: "mutations", format: ::semio_framework_schema_registry::SchemaFormat::Graphql, path: "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/📑️document/🧬️schema/🧬️mutations/🔗️.graphql" },
        ::semio_framework_schema_registry::SchemaLeafDocument { scope: "s.stdio.semio.document", export: "mutations", format: ::semio_framework_schema_registry::SchemaFormat::Protobuf, path: "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/📑️document/🧬️schema/🧬️mutations/🛰️.proto" },
    ]);
    #[cfg(target_arch = "wasm32")]
    semio_framework_schema_registry::install_default_schema_leaf_reader(semio_framework_plugin::reactor::package_asset, "");
    #[cfg(not(target_arch = "wasm32"))]
    semio_framework_schema_registry::install_default_schema_leaf_reader(semio_framework_plugin::reactor::package_asset, env!("CARGO_MANIFEST_DIR"));
    semio_framework_schema_registry::ArtifactSchemaDescriptor {
        id: "s.stdio.semio.document",
        artifact: semio_framework_schema_registry::FacetLeaves { rust: "", typescript: "", graphql: "", json_schema: include_str!("🔣️.json"), proto: "" },
        snapshot: semio_framework_schema_registry::FacetLeaves {
            rust: "",
            typescript: "",
            graphql: "",
            json_schema: include_str!("📸️snapshot/🔣️.json"),
            proto: "",
        },
        diff: semio_framework_schema_registry::FacetLeaves {
            rust: "",
            typescript: "",
            graphql: "",
            json_schema: include_str!("🔺️diff/🔣️.json"),
            proto: "",
        },
        mutations: semio_framework_schema_registry::FacetLeaves {
            rust: "",
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
