//! 🧬️ PlyArtifact schema — full artifact state.

use crate::schema::snapshot::{PlyElement, PlyFormat};
use crate::PlySnapshot;
use framework_schema::ArtifactSchema;

//#region 🔖️Artifact
/// 🧬️ Full `stdio.ply` artifact state — mirrors `PlySnapshot`'s persistent fields exactly.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, ArtifactSchema, semio_framework_value::RetireOwned)]
#[value(rename_all = "camelCase")]
#[artifact_schema(id = "s.stdio.ply")]
pub struct PlyArtifact {
    #[state(artifact)]
    pub schema: String,
    #[state(artifact)]
    #[value(default)]
    pub format: PlyFormat,
    #[state(artifact)]
    #[value(default)]
    pub comments: Vec<String>,
    #[state(artifact)]
    #[value(default)]
    pub elements: Vec<PlyElement>,
}
//#endregion 🔖️Artifact

//#region 🔖️Conversions
impl Default for PlyArtifact {
    fn default() -> Self {
        Self::from_snapshot(PlySnapshot::default())
    }
}

impl PlyArtifact {
    /// 📸️ Persisted subset.
    pub fn to_snapshot(&self) -> PlySnapshot {
        PlySnapshot { schema: self.schema.clone(), format: self.format, comments: self.comments.clone(), elements: self.elements.clone() }
    }

    /// 🧬️ Builds a full artifact from a snapshot.
    pub fn from_snapshot(snapshot: PlySnapshot) -> Self {
        Self { schema: snapshot.schema, format: snapshot.format, comments: snapshot.comments, elements: snapshot.elements }
    }

    /// 🔄 Writes persistent fields from a snapshot into this artifact.
    pub fn set_snapshot(&mut self, snapshot: PlySnapshot) {
        self.schema = snapshot.schema;
        self.format = snapshot.format;
        self.comments = snapshot.comments;
        self.elements = snapshot.elements;
    }
}
//#endregion 🔖️Conversions

//#region 🔖️Descriptor
/// 🧬️ Descriptor for `s.stdio.ply`.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn ply_artifact_schema_descriptor() -> semio_framework_schema_registry::ArtifactSchemaDescriptor {
    ::semio_framework_schema_registry::register_schema_leaf_documents(&[
        ::semio_framework_schema_registry::SchemaLeafDocument { scope: "s.stdio.ply", export: "artifact", format: ::semio_framework_schema_registry::SchemaFormat::Rust, path: "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧱️ply/🏅️standards/🔖️1.0/🪆️subsets/✳️any/🧬️schema/🦀️.rs" },
        ::semio_framework_schema_registry::SchemaLeafDocument { scope: "s.stdio.ply", export: "artifact", format: ::semio_framework_schema_registry::SchemaFormat::Typescript, path: "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧱️ply/🏅️standards/🔖️1.0/🪆️subsets/✳️any/🧬️schema/🟦️.ts" },
        ::semio_framework_schema_registry::SchemaLeafDocument { scope: "s.stdio.ply", export: "artifact", format: ::semio_framework_schema_registry::SchemaFormat::Graphql, path: "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧱️ply/🏅️standards/🔖️1.0/🪆️subsets/✳️any/🧬️schema/🔗️.graphql" },
        ::semio_framework_schema_registry::SchemaLeafDocument { scope: "s.stdio.ply", export: "artifact", format: ::semio_framework_schema_registry::SchemaFormat::Protobuf, path: "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧱️ply/🏅️standards/🔖️1.0/🪆️subsets/✳️any/🧬️schema/🛰️.proto" },
        ::semio_framework_schema_registry::SchemaLeafDocument { scope: "s.stdio.ply", export: "snapshot", format: ::semio_framework_schema_registry::SchemaFormat::Rust, path: "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧱️ply/🏅️standards/🔖️1.0/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🦀️.rs" },
        ::semio_framework_schema_registry::SchemaLeafDocument { scope: "s.stdio.ply", export: "snapshot", format: ::semio_framework_schema_registry::SchemaFormat::Typescript, path: "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧱️ply/🏅️standards/🔖️1.0/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🟦️.ts" },
        ::semio_framework_schema_registry::SchemaLeafDocument { scope: "s.stdio.ply", export: "snapshot", format: ::semio_framework_schema_registry::SchemaFormat::Graphql, path: "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧱️ply/🏅️standards/🔖️1.0/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🔗️.graphql" },
        ::semio_framework_schema_registry::SchemaLeafDocument { scope: "s.stdio.ply", export: "snapshot", format: ::semio_framework_schema_registry::SchemaFormat::Protobuf, path: "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧱️ply/🏅️standards/🔖️1.0/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🛰️.proto" },
        ::semio_framework_schema_registry::SchemaLeafDocument { scope: "s.stdio.ply", export: "diff", format: ::semio_framework_schema_registry::SchemaFormat::Rust, path: "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧱️ply/🏅️standards/🔖️1.0/🪆️subsets/✳️any/🧬️schema/🔺️diff/🦀️.rs" },
        ::semio_framework_schema_registry::SchemaLeafDocument { scope: "s.stdio.ply", export: "diff", format: ::semio_framework_schema_registry::SchemaFormat::Rust, path: "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧱️ply/🏅️standards/🔖️1.0/🪆️subsets/✳️any/🧬️schema/🔺️diff/🧮️indexed-triple-algebra/🦀️.rs" },
        ::semio_framework_schema_registry::SchemaLeafDocument { scope: "s.stdio.ply", export: "diff", format: ::semio_framework_schema_registry::SchemaFormat::Rust, path: "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧱️ply/🏅️standards/🔖️1.0/🪆️subsets/✳️any/🧬️schema/🔺️diff/📊️row-and-element-diffs/🦀️.rs" },
        ::semio_framework_schema_registry::SchemaLeafDocument { scope: "s.stdio.ply", export: "diff", format: ::semio_framework_schema_registry::SchemaFormat::Rust, path: "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧱️ply/🏅️standards/🔖️1.0/🪆️subsets/✳️any/🧬️schema/🔺️diff/🧊️ply-diff-aggregate/🦀️.rs" },
        ::semio_framework_schema_registry::SchemaLeafDocument { scope: "s.stdio.ply", export: "diff", format: ::semio_framework_schema_registry::SchemaFormat::Rust, path: "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧱️ply/🏅️standards/🔖️1.0/🪆️subsets/✳️any/🧬️schema/🔺️diff/🧵️codec-and-demo-cases/🦀️.rs" },
        ::semio_framework_schema_registry::SchemaLeafDocument { scope: "s.stdio.ply", export: "diff", format: ::semio_framework_schema_registry::SchemaFormat::Typescript, path: "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧱️ply/🏅️standards/🔖️1.0/🪆️subsets/✳️any/🧬️schema/🔺️diff/🟦️.ts" },
        ::semio_framework_schema_registry::SchemaLeafDocument { scope: "s.stdio.ply", export: "diff", format: ::semio_framework_schema_registry::SchemaFormat::Graphql, path: "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧱️ply/🏅️standards/🔖️1.0/🪆️subsets/✳️any/🧬️schema/🔺️diff/🔗️.graphql" },
        ::semio_framework_schema_registry::SchemaLeafDocument { scope: "s.stdio.ply", export: "diff", format: ::semio_framework_schema_registry::SchemaFormat::Protobuf, path: "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧱️ply/🏅️standards/🔖️1.0/🪆️subsets/✳️any/🧬️schema/🔺️diff/🛰️.proto" },
        ::semio_framework_schema_registry::SchemaLeafDocument { scope: "s.stdio.ply", export: "mutations", format: ::semio_framework_schema_registry::SchemaFormat::Rust, path: "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧱️ply/🏅️standards/🔖️1.0/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️.rs" },
        ::semio_framework_schema_registry::SchemaLeafDocument { scope: "s.stdio.ply", export: "mutations", format: ::semio_framework_schema_registry::SchemaFormat::Typescript, path: "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧱️ply/🏅️standards/🔖️1.0/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🟦️.ts" },
        ::semio_framework_schema_registry::SchemaLeafDocument { scope: "s.stdio.ply", export: "mutations", format: ::semio_framework_schema_registry::SchemaFormat::Graphql, path: "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧱️ply/🏅️standards/🔖️1.0/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔗️.graphql" },
        ::semio_framework_schema_registry::SchemaLeafDocument { scope: "s.stdio.ply", export: "mutations", format: ::semio_framework_schema_registry::SchemaFormat::Protobuf, path: "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧱️ply/🏅️standards/🔖️1.0/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🛰️.proto" },
    ]);
    #[cfg(target_arch = "wasm32")]
    semio_framework_schema_registry::install_default_schema_leaf_reader(semio_framework_plugin::reactor::package_asset, "");
    #[cfg(not(target_arch = "wasm32"))]
    semio_framework_schema_registry::install_default_schema_leaf_reader(semio_framework_plugin::reactor::package_asset, env!("CARGO_MANIFEST_DIR"));
    semio_framework_schema_registry::ArtifactSchemaDescriptor {
        id: "s.stdio.ply",
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
//#endregion 🔖️Descriptor
//#region 🏗️DerivedConstruction

//#endregion 🏗️DerivedConstruction

//#region 🧐️DerivedAnalysis

//#endregion 🧐️DerivedAnalysis

//#region 🔖️DocumentHelpers
/// 🌱 Empty persisted snapshot.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn empty_ply_snapshot() -> PlySnapshot {
    PlySnapshot::default()
}

/// ✅️ P2-FG3: the representative `PlySnapshot` every conformance law and the shipped
/// `📚️examples/🎬️demo/🖼️assets` fixtures are built from — a `vertex` element (2 rows, plain
/// scalar `float` properties) and a `face` element (1 row, a `list uchar int vertex_indices`
/// property, exercising the count-prefixed list-cell shape) plus one comment. `format:
/// PlyFormat::Ascii` selects representative format metadata. Logical Text and Pack retain
/// all formats, independent declarations, occurrence vectors and exact scalar words.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn demo_ply_snapshot() -> PlySnapshot {
    use crate::schema::snapshot::{PlyProperty, PlyRow, PlyScalarType, PlyValue};
    PlySnapshot {
        schema: crate::STDIO_PLY_DOCUMENT_SCHEMA.into(),
        format: PlyFormat::Ascii,
        comments: vec!["semio demo".into()],
        elements: vec![
            PlyElement {
                name: "vertex".into(),
                count: 2,
                properties: vec![PlyProperty::Scalar { name: "x".into(), kind: PlyScalarType::Float }, PlyProperty::Scalar { name: "y".into(), kind: PlyScalarType::Float }, PlyProperty::Scalar { name: "z".into(), kind: PlyScalarType::Float }],
                rows: vec![PlyRow { values: vec![PlyValue::Float(0.0), PlyValue::Float(0.0), PlyValue::Float(0.0)] }, PlyRow { values: vec![PlyValue::Float(1.0), PlyValue::Float(0.5), PlyValue::Float(-1.5)] }],
            },
            PlyElement {
                name: "face".into(),
                count: 1,
                properties: vec![PlyProperty::List { name: "vertex_indices".into(), count_kind: PlyScalarType::UChar, value_kind: PlyScalarType::Int }],
                rows: vec![PlyRow { values: vec![PlyValue::List(vec![PlyValue::Int(0), PlyValue::Int(1)])] }],
            },
        ],
    }
}
//#endregion 🔖️DocumentHelpers

//#region 🧬️DerivedArtifactFacets

//#endregion 🧬️DerivedArtifactFacets
