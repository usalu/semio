//! 🧬️ PlyArtifact schema — full artifact state.

use crate::artifacts::ply::schema::snapshot::{PlyElement, PlyFormat};
use crate::artifacts::ply::PlySnapshot;
use schema::ArtifactSchema;
use serde::{Deserialize, Serialize};

//#region 🔖️Artifact
/// 🧬️ Full `stdio.ply` artifact state — mirrors `PlySnapshot`'s persistent fields exactly.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase")]
#[artifact_schema(id = "s.stdio.ply")]
pub struct PlyArtifact {
    #[state(artifact)]
    pub schema: String,
    #[state(artifact)]
    #[serde(default)]
    pub format: PlyFormat,
    #[state(artifact)]
    #[serde(default)]
    pub comments: Vec<String>,
    #[state(artifact)]
    #[serde(default)]
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
        PlySnapshot {
            schema: self.schema.clone(),
            format: self.format,
            comments: self.comments.clone(),
            elements: self.elements.clone(),
        }
    }

    /// 🧬️ Builds a full artifact from a snapshot.
    pub fn from_snapshot(snapshot: PlySnapshot) -> Self {
        Self {
            schema: snapshot.schema,
            format: snapshot.format,
            comments: snapshot.comments,
            elements: snapshot.elements,
        }
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
pub fn ply_artifact_schema_descriptor() -> schema::ArtifactSchemaDescriptor {
    schema::ArtifactSchemaDescriptor {
        id: "s.stdio.ply",
        artifact: schema::FacetLeaves {
            rust: include_str!("🦀️component.rs"),
            typescript: include_str!("🟦️component.ts"),
            graphql: include_str!("🔗️component.graphql"),
            json_schema: include_str!("🔣️component.json"),
            proto: include_str!("🛰️component.proto"),
        },
        snapshot: schema::FacetLeaves {
            rust: include_str!("📸️snapshot/🦀️component.rs"),
            typescript: include_str!("📸️snapshot/🟦️component.ts"),
            graphql: include_str!("📸️snapshot/🔗️component.graphql"),
            json_schema: include_str!("📸️snapshot/🔣️component.json"),
            proto: include_str!("📸️snapshot/🛰️component.proto"),
        },
        diff: schema::FacetLeaves {
            rust: include_str!("🔺️diff/🦀️component.rs"),
            typescript: include_str!("🔺️diff/🟦️component.ts"),
            graphql: include_str!("🔺️diff/🔗️component.graphql"),
            json_schema: include_str!("🔺️diff/🔣️component.json"),
            proto: include_str!("🔺️diff/🛰️component.proto"),
        },
        mutations: schema::FacetLeaves {
            rust: include_str!("🧬️mutations/🦀️component.rs"),
            typescript: include_str!("🧬️mutations/🟦️component.ts"),
            graphql: include_str!("🧬️mutations/🔗️component.graphql"),
            json_schema: include_str!("🧬️mutations/🔣️component.json"),
            proto: include_str!("🧬️mutations/🛰️component.proto"),
        },
    }
}
//#endregion 🔖️Descriptor
//#region 🏗️DerivedConstruction
pub mod derived_construction {
    use semio_framework_plugin::ArtifactBuilder;
    use crate::artifacts::ply::{PlyDiff, PlyMutation, PlySnapshot};

    //#region 🔖️Builder
    /// 🏗️ Builds a `stdio.ply` snapshot.
    #[derive(Clone, Debug, Default)]
    pub struct PlyBuilderConstruction {
        snapshot: PlySnapshot,
        diagnostics: Vec<dsl::Diagnostic>,
    }

    impl ArtifactBuilder for PlyBuilderConstruction {
        type Snapshot = PlySnapshot;
        type Mutation = PlyMutation;
        type Diff = PlyDiff;
        fn empty() -> Self {
            Self { snapshot: PlySnapshot::default(), diagnostics: Vec::new() }
        }
        fn from_snapshot(snapshot: Self::Snapshot) -> Self {
            Self { snapshot, diagnostics: Vec::new() }
        }
        fn from_text(text: &str) -> Result<Self, store::TextError> {
            Ok(Self::from_snapshot(<PlySnapshot as store::ArtifactDsl>::parse_dsl(text)?))
        }
        fn from_binary(bytes: &[u8]) -> Result<Self, store::PackError> {
            Ok(Self::from_snapshot(<PlySnapshot as store::ArtifactPack>::decode_pack(bytes)?))
        }
        fn mutate(mut self, mutation: Self::Mutation) -> (Self, Self::Diff) {
            let diff = crate::artifacts::ply::schema::mutations::apply_ply_mutation(&mut self.snapshot, &mutation);
            (self, diff)
        }
        fn absorb(mut self, diff: Self::Diff) -> Self {
            self.snapshot = <PlyDiff as protocol::MutationDiff<PlySnapshot>>::apply(&diff, &self.snapshot);
            self
        }
        fn build(self) -> Result<Self::Snapshot, Vec<dsl::Diagnostic>> {
            if self.diagnostics.is_empty() { Ok(self.snapshot) } else { Err(self.diagnostics) }
        }
    }
    //#endregion 🔖️Builder
}
pub use derived_construction::*;
//#endregion 🏗️DerivedConstruction

//#region 🧐️DerivedAnalysis
pub mod derived_analysis {
    use semio_framework_plugin::{ArtifactAnalysis, Dialect, StandardId, SubsetId, IoConfidence, Analysis, AnalyzeSource};
    use crate::artifacts::ply::PlySnapshot;

    //#region 🔖️Parts
    /// 🧩 Analyzed `stdio.ply` parts.
    #[derive(Clone, Debug, Default)]
    pub struct PlyParts {
        pub snapshot: Option<PlySnapshot>,
    }
    //#endregion 🔖️Parts

    //#region 🔖️Analyzer
    /// 🧐️ Analyzes `stdio.ply` (1.0/✳️any) sources.
    pub struct PlyAnalyzerAnalysis;

    impl ArtifactAnalysis for PlyAnalyzerAnalysis {
        type Parts = PlyParts;
        const DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.ply", standard: StandardId("1.0"), subset: SubsetId("*") };

        fn sniff(source: &AnalyzeSource<'_>) -> IoConfidence {
            // 🔍 PLY files (ascii or either binary variant) always start with a literal ASCII
            // "ply" magic line — `ply\n` or `ply\r\n` — per the format spec. Unlike png/las,
            // stdio.ply's text envelope embeds the raw ply bytes directly (no hex dump), so both
            // sources are checked against the same literal prefix.
            const MAGIC_LF: &[u8] = b"ply\n";
            const MAGIC_CRLF: &[u8] = b"ply\r\n";
            let starts_with_magic = |bytes: &[u8]| bytes.starts_with(MAGIC_LF) || bytes.starts_with(MAGIC_CRLF);
            match source {
                AnalyzeSource::Binary(bytes) => {
                    if starts_with_magic(bytes) { IoConfidence::High } else { IoConfidence::Low }
                }
                AnalyzeSource::Text(text) => {
                    let body = match store::semio_format::split_text_preamble(text) {
                        Ok((_, rest)) => rest,
                        Err(_) => text,
                    };
                    if starts_with_magic(body.as_bytes()) { IoConfidence::High } else { IoConfidence::Low }
                }
            }
        }

        fn analyze(sources: &[AnalyzeSource<'_>]) -> Analysis<Self::Parts> {
            let mut parts = PlyParts::default();
            let mut diagnostics = Vec::new();
            let mut confidence = IoConfidence::High;
            for source in sources {
                match source {
                    AnalyzeSource::Text(text) => match <PlySnapshot as store::ArtifactDsl>::parse_dsl(text) {
                        Ok(snapshot) => parts.snapshot = Some(snapshot),
                        Err(err) => {
                            confidence = IoConfidence::Low;
                            diagnostics.push(dsl::Diagnostic::error(
                                "stdio.analyze.text",
                                dsl::TextSpan::at(1, 1),
                                err.to_string(),
                            ));
                        }
                    },
                    AnalyzeSource::Binary(bytes) => match <PlySnapshot as store::ArtifactPack>::decode_pack(bytes) {
                        Ok(snapshot) => parts.snapshot = Some(snapshot),
                        Err(err) => {
                            confidence = IoConfidence::Low;
                            diagnostics.push(dsl::Diagnostic::error(
                                "stdio.analyze.binary",
                                dsl::TextSpan::at(1, 1),
                                err.to_string(),
                            ));
                        }
                    },
                }
            }
            Analysis { parts, dialect: Self::DIALECT, confidence, diagnostics }
        }
    }
    //#endregion 🔖️Analyzer
}
pub use derived_analysis::*;
//#endregion 🧐️DerivedAnalysis

//#region 🔖️DocumentHelpers
/// 🌱 Empty persisted snapshot.
pub fn empty_ply_snapshot() -> PlySnapshot {
    PlySnapshot::default()
}

/// ✅️ P2-FG3: the representative `PlySnapshot` every conformance law and the shipped
/// `📚️examples/🎬️demo/🖼️assets` fixtures are built from — a `vertex` element (2 rows, plain
/// scalar `float` properties) and a `face` element (1 row, a `list uchar int vertex_indices`
/// property, exercising the count-prefixed list-cell shape) plus one comment. `format:
/// PlyFormat::Ascii` deliberately — `print_dsl`/`parse_dsl` always render/read the CANONICAL
/// ascii encoding regardless of `format` (see `📸️snapshot/🦀️component.rs`'s own
/// `HandcraftedArtifactCodecs` doc comment), so a demo snapshot whose OWN `format` field isn't
/// `Ascii` would make `fixture_honesty_law`'s `parse_dsl(print_dsl(demo)) == demo` fail — the
/// DSL/text facet's own format-normalization would silently overwrite it. The Pack facet (which
/// DOES respect `self.format`) is exercised against genuine BINARY bytes separately, by
/// `protocol_walk_law` calling `encode_ply_with_format` directly with a non-ascii format.
pub fn demo_ply_snapshot() -> PlySnapshot {
    use crate::artifacts::ply::schema::snapshot::{PlyProperty, PlyRow, PlyScalarType, PlyValue};
    PlySnapshot {
        schema: crate::artifacts::ply::STDIO_PLY_DOCUMENT_SCHEMA.into(),
        format: PlyFormat::Ascii,
        comments: vec!["semio demo".into()],
        elements: vec![
            PlyElement {
                name: "vertex".into(),
                count: 2,
                properties: vec![
                    PlyProperty::Scalar { name: "x".into(), kind: PlyScalarType::Float },
                    PlyProperty::Scalar { name: "y".into(), kind: PlyScalarType::Float },
                    PlyProperty::Scalar { name: "z".into(), kind: PlyScalarType::Float },
                ],
                rows: vec![
                    PlyRow { values: vec![PlyValue::Float(0.0), PlyValue::Float(0.0), PlyValue::Float(0.0)] },
                    PlyRow { values: vec![PlyValue::Float(1.0), PlyValue::Float(0.5), PlyValue::Float(-1.5)] },
                ],
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
semio_framework_plugin::derive_artifact_facets!(
    pub spec PlyBuilderFacets {
        construction: derived_construction::PlyBuilderConstruction,
        analysis: derived_analysis::PlyAnalyzerAnalysis,
        composition: super::super::io::derived_composition::PlyComposerComposition,
    }
    builder: PlyBuilder,
    analyzer: PlyAnalyzer,
    composer: PlyComposer,
);
//#endregion 🧬️DerivedArtifactFacets
