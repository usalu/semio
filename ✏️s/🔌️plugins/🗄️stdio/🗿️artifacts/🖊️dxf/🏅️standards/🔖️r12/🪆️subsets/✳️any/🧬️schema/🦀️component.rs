//! 🧬️ DxfArtifact schema — full artifact state (mirrors `DxfSnapshot`'s persisted fields
//! one-for-one; see `📸️snapshot/🦀️component.rs` module docs for the full typed-model rationale).

use crate::artifacts::dxf::schema::snapshot::{DxfBlock, DxfEntity, DxfHeaderVar, DxfOtherTable, DxfTables};
use crate::artifacts::dxf::DxfSnapshot;
use schema::ArtifactSchema;
use serde::{Deserialize, Serialize};

//#region 🔖️Artifact
/// 🧬️ Full `stdio.dxf` artifact state.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase")]
#[artifact_schema(id = "s.stdio.dxf")]
pub struct DxfArtifact {
    #[state(persistent)]
    pub schema: String,
    #[state(persistent)]
    #[serde(default)]
    pub header_vars: Vec<DxfHeaderVar>,
    #[state(persistent)]
    #[serde(default)]
    pub tables: DxfTables,
    #[state(persistent)]
    #[serde(default)]
    pub other_tables: Vec<DxfOtherTable>,
    #[state(persistent)]
    #[serde(default)]
    pub blocks: Vec<DxfBlock>,
    #[state(persistent)]
    #[serde(default)]
    pub entities: Vec<DxfEntity>,
}
//#endregion 🔖️Artifact

//#region 🔖️Conversions
impl Default for DxfArtifact {
    fn default() -> Self {
        Self::from_snapshot(DxfSnapshot::default())
    }
}

impl DxfArtifact {
    /// 📸️ Persisted subset.
    pub fn to_snapshot(&self) -> DxfSnapshot {
        DxfSnapshot {
            schema: self.schema.clone(),
            header_vars: self.header_vars.clone(),
            tables: self.tables.clone(),
            other_tables: self.other_tables.clone(),
            blocks: self.blocks.clone(),
            entities: self.entities.clone(),
        }
    }

    /// 🧬️ Builds a full artifact from a snapshot.
    pub fn from_snapshot(snapshot: DxfSnapshot) -> Self {
        Self {
            schema: snapshot.schema,
            header_vars: snapshot.header_vars,
            tables: snapshot.tables,
            other_tables: snapshot.other_tables,
            blocks: snapshot.blocks,
            entities: snapshot.entities,
        }
    }

    /// 🔄 Writes persistent fields from a snapshot into this artifact.
    pub fn set_snapshot(&mut self, snapshot: DxfSnapshot) {
        self.schema = snapshot.schema;
        self.header_vars = snapshot.header_vars;
        self.tables = snapshot.tables;
        self.other_tables = snapshot.other_tables;
        self.blocks = snapshot.blocks;
        self.entities = snapshot.entities;
    }
}
//#endregion 🔖️Conversions

//#region 🔖️Descriptor
/// 🧬️ Descriptor for `s.stdio.dxf`.
pub fn dxf_artifact_schema_descriptor() -> schema::ArtifactSchemaDescriptor {
    schema::ArtifactSchemaDescriptor {
        id: "s.stdio.dxf",
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
    use crate::artifacts::dxf::{DxfDiff, DxfMutation, DxfSnapshot};

    //#region 🔖️Builder
    /// 🏗️ Builds a `stdio.dxf` snapshot.
    #[derive(Clone, Debug, Default)]
    pub struct DxfBuilderConstruction {
        snapshot: DxfSnapshot,
        diagnostics: Vec<dsl::Diagnostic>,
    }

    impl ArtifactBuilder for DxfBuilderConstruction {
        type Snapshot = DxfSnapshot;
        type Mutation = DxfMutation;
        type Diff = DxfDiff;
        fn empty() -> Self {
            Self { snapshot: DxfSnapshot::default(), diagnostics: Vec::new() }
        }
        fn from_snapshot(snapshot: Self::Snapshot) -> Self {
            Self { snapshot, diagnostics: Vec::new() }
        }
        fn from_text(text: &str) -> Result<Self, store::TextError> {
            Ok(Self::from_snapshot(<DxfSnapshot as store::ArtifactDsl>::parse_dsl(text)?))
        }
        fn from_binary(bytes: &[u8]) -> Result<Self, store::PackError> {
            Ok(Self::from_snapshot(<DxfSnapshot as store::ArtifactPack>::decode_pack(bytes)?))
        }
        fn mutate(mut self, mutation: Self::Mutation) -> (Self, Self::Diff) {
            let diff = crate::artifacts::dxf::schema::mutations::apply_dxf_mutation(&mut self.snapshot, &mutation);
            (self, diff)
        }
        fn absorb(mut self, diff: Self::Diff) -> Self {
            self.snapshot = <DxfDiff as protocol::MutationDiff<DxfSnapshot>>::apply(&diff, &self.snapshot);
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
    use crate::artifacts::dxf::DxfSnapshot;

    //#region 🔖️Parts
    /// 🧩 Analyzed `stdio.dxf` parts.
    #[derive(Clone, Debug, Default)]
    pub struct DxfParts {
        pub snapshot: Option<DxfSnapshot>,
    }
    //#endregion 🔖️Parts

    //#region 🔖️Analyzer
    /// 🧐️ Analyzes `stdio.dxf` (r12/✳️any) sources.
    pub struct DxfAnalyzerAnalysis;

    impl ArtifactAnalysis for DxfAnalyzerAnalysis {
        type Parts = DxfParts;
        const DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.dxf", standard: StandardId("r12"), subset: SubsetId("*") };

        /// 🧭️ DXF ASCII has no fixed magic byte (unlike binary formats), so this is a structural
        /// heuristic rather than an exact match: the first non-blank line must trim to a valid
        /// integer group code, and one of the DXF section/version markers (`SECTION`, `HEADER`,
        /// `ENTITIES`, or an `AC10xx`-style version string) must appear among the first tags.
        fn sniff(source: &AnalyzeSource<'_>) -> IoConfidence {
            let text = match source {
                AnalyzeSource::Text(text) => Some(*text),
                AnalyzeSource::Binary(_) => None,
            };
            let Some(text) = text else { return IoConfidence::Low };
            let body = match store::semio_format::split_text_preamble(text) {
                Ok((_, rest)) => rest,
                Err(_) => text,
            };
            let lines: Vec<&str> = body.lines().map(str::trim).filter(|l| !l.is_empty()).collect();
            let Some(first) = lines.first() else { return IoConfidence::Low };
            if first.parse::<i32>().is_err() {
                return IoConfidence::Low;
            }
            let has_marker = lines.iter().take(64).any(|l| {
                matches!(*l, "SECTION" | "HEADER" | "ENTITIES" | "EOF")
                    || (l.len() == 6 && l.starts_with("AC") && l[2..].chars().all(|c| c.is_ascii_digit()))
            });
            if has_marker {
                IoConfidence::High
            } else {
                IoConfidence::Medium
            }
        }

        fn analyze(sources: &[AnalyzeSource<'_>]) -> Analysis<Self::Parts> {
            let mut parts = DxfParts::default();
            let mut diagnostics = Vec::new();
            let mut confidence = IoConfidence::High;
            for source in sources {
                match source {
                    AnalyzeSource::Text(text) => match <DxfSnapshot as store::ArtifactDsl>::parse_dsl(text) {
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
                    AnalyzeSource::Binary(bytes) => match <DxfSnapshot as store::ArtifactPack>::decode_pack(bytes) {
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
    pub spec DxfBuilderFacets {
        construction: derived_construction::DxfBuilderConstruction,
        analysis: derived_analysis::DxfAnalyzerAnalysis,
        composition: super::super::io::derived_composition::DxfComposerComposition,
    }
    builder: DxfBuilder,
    analyzer: DxfAnalyzer,
    composer: DxfComposer,
);
//#endregion 🧬️DerivedArtifactFacets
