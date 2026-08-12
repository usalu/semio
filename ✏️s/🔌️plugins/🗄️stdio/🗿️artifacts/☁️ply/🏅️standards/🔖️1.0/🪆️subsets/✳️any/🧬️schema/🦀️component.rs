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
    #[state(persistent)]
    pub schema: String,
    #[state(persistent)]
    #[serde(default)]
    pub format: PlyFormat,
    #[state(persistent)]
    #[serde(default)]
    pub comments: Vec<String>,
    #[state(persistent)]
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
