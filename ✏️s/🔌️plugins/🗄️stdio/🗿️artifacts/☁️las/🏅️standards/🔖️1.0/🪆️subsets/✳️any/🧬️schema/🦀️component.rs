//! 🧬️ LasArtifact schema — full artifact state.

use crate::artifacts::las::LasSnapshot;
use schema::ArtifactSchema;
use serde::{Deserialize, Serialize};

//#region 🔖️Artifact
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase")]
#[artifact_schema(id = "s.stdio.las")]
pub struct LasArtifact {
    #[state(persistent)]
    pub schema: String,
    #[state(persistent)]
    #[serde(default)]
    pub header: crate::artifacts::las::schema::snapshot::LasHeader,
    #[state(persistent)]
    #[serde(default)]
    pub vlrs: Vec<crate::artifacts::las::schema::snapshot::LasVlr>,
    #[state(persistent)]
    #[serde(default)]
    pub points: Vec<crate::artifacts::las::schema::snapshot::LasPoint>,
}
//#endregion 🔖️Artifact

//#region 🔖️Conversions
impl Default for LasArtifact {
    fn default() -> Self {
        Self::from_snapshot(LasSnapshot::default())
    }
}

impl LasArtifact {
    pub fn to_snapshot(&self) -> LasSnapshot {
        LasSnapshot {
            schema: self.schema.clone(),
            header: self.header.clone(),
            vlrs: self.vlrs.clone(),
            points: self.points.clone(),
        }
    }

    pub fn from_snapshot(snapshot: LasSnapshot) -> Self {
        Self {
            schema: snapshot.schema,
            header: snapshot.header,
            vlrs: snapshot.vlrs,
            points: snapshot.points,
        }
    }

    pub fn set_snapshot(&mut self, snapshot: LasSnapshot) {
        self.schema = snapshot.schema;
        self.header = snapshot.header;
        self.vlrs = snapshot.vlrs;
        self.points = snapshot.points;
    }
}
//#endregion 🔖️Conversions

//#region 🔖️Descriptor
pub fn las_artifact_schema_descriptor() -> schema::ArtifactSchemaDescriptor {
    schema::ArtifactSchemaDescriptor {
        id: "s.stdio.las",
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
    use crate::artifacts::las::{LasDiff, LasMutation, LasSnapshot};

    //#region 🔖️Builder
    /// 🏗️ Builds a `stdio.las` snapshot.
    #[derive(Clone, Debug, Default)]
    pub struct LasBuilderConstruction {
        snapshot: LasSnapshot,
        diagnostics: Vec<dsl::Diagnostic>,
    }

    impl ArtifactBuilder for LasBuilderConstruction {
        type Snapshot = LasSnapshot;
        type Mutation = LasMutation;
        type Diff = LasDiff;
        fn empty() -> Self {
            Self { snapshot: LasSnapshot::default(), diagnostics: Vec::new() }
        }
        fn from_snapshot(snapshot: Self::Snapshot) -> Self {
            Self { snapshot, diagnostics: Vec::new() }
        }
        fn from_text(text: &str) -> Result<Self, store::TextError> {
            Ok(Self::from_snapshot(<LasSnapshot as store::ArtifactDsl>::parse_dsl(text)?))
        }
        fn from_binary(bytes: &[u8]) -> Result<Self, store::PackError> {
            Ok(Self::from_snapshot(<LasSnapshot as store::ArtifactPack>::decode_pack(bytes)?))
        }
        fn mutate(mut self, mutation: Self::Mutation) -> (Self, Self::Diff) {
            let diff = crate::artifacts::las::schema::mutations::apply_las_mutation(&mut self.snapshot, &mutation);
            (self, diff)
        }
        fn absorb(mut self, diff: Self::Diff) -> Self {
            self.snapshot = <LasDiff as protocol::MutationDiff<LasSnapshot>>::apply(&diff, &self.snapshot);
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
    use crate::artifacts::las::LasSnapshot;

    //#region 🔖️Parts
    /// 🧩 Analyzed `stdio.las` parts.
    #[derive(Clone, Debug, Default)]
    pub struct LasParts {
        pub snapshot: Option<LasSnapshot>,
    }
    //#endregion 🔖️Parts

    //#region 🔖️Analyzer
    /// 🧐️ Analyzes `stdio.las` (1.0/✳️any) sources.
    pub struct LasAnalyzerAnalysis;

    impl ArtifactAnalysis for LasAnalyzerAnalysis {
        type Parts = LasParts;
        const DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.las", standard: StandardId("1.0"), subset: SubsetId("*") };

        fn sniff(source: &AnalyzeSource<'_>) -> IoConfidence {
            const SIG: [u8; 4] = *b"LASF";
            match source {
                AnalyzeSource::Binary(bytes) => {
                    if bytes.len() >= 4 && bytes[0..4] == SIG { IoConfidence::High } else { IoConfidence::Low }
                }
                AnalyzeSource::Text(text) => {
                    // 🔍 stdio.las's text envelope is a hex dump of the raw bytes after the
                    // `semio ...` preamble line — decode the first 4 bytes to sniff the real signature.
                    let body = match store::semio_format::split_text_preamble(text) {
                        Ok((_, rest)) => rest,
                        Err(_) => text,
                    };
                    let hex: String = body.chars().filter(|c| !c.is_whitespace()).take(8).collect();
                    if hex.len() < 8 {
                        return IoConfidence::Low;
                    }
                    let mut decoded = [0u8; 4];
                    for (i, byte) in decoded.iter_mut().enumerate() {
                        match u8::from_str_radix(&hex[i * 2..i * 2 + 2], 16) {
                            Ok(b) => *byte = b,
                            Err(_) => return IoConfidence::Low,
                        }
                    }
                    if decoded == SIG { IoConfidence::High } else { IoConfidence::Low }
                }
            }
        }

        fn analyze(sources: &[AnalyzeSource<'_>]) -> Analysis<Self::Parts> {
            let mut parts = LasParts::default();
            let mut diagnostics = Vec::new();
            let mut confidence = IoConfidence::High;
            for source in sources {
                match source {
                    AnalyzeSource::Text(text) => match <LasSnapshot as store::ArtifactDsl>::parse_dsl(text) {
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
                    AnalyzeSource::Binary(bytes) => match <LasSnapshot as store::ArtifactPack>::decode_pack(bytes) {
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
    pub spec LasBuilderFacets {
        construction: derived_construction::LasBuilderConstruction,
        analysis: derived_analysis::LasAnalyzerAnalysis,
        composition: super::super::io::derived_composition::LasComposerComposition,
    }
    builder: LasBuilder,
    analyzer: LasAnalyzer,
    composer: LasComposer,
);
//#endregion 🧬️DerivedArtifactFacets
