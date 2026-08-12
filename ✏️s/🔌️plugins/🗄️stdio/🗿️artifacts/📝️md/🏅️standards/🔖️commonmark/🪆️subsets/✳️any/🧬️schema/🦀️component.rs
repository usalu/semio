//! 🧬️ MdArtifact schema — full artifact state.

use crate::artifacts::md::schema::snapshot::MdBlock;
use crate::artifacts::md::MdSnapshot;
use schema::ArtifactSchema;
use serde::{Deserialize, Serialize};

//#region 🔖️Artifact
/// 🧬️ Full `stdio.md` artifact state.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase")]
#[artifact_schema(id = "s.stdio.md")]
pub struct MdArtifact {
    #[state(persistent)]
    pub schema: String,
    #[state(persistent)]
    #[serde(default)]
    pub blocks: Vec<MdBlock>,
}
//#endregion 🔖️Artifact

//#region 🔖️Conversions
impl Default for MdArtifact {
    fn default() -> Self {
        Self::from_snapshot(MdSnapshot::default())
    }
}

impl MdArtifact {
    /// 📸️ Persisted subset.
    pub fn to_snapshot(&self) -> MdSnapshot {
        MdSnapshot {
            schema: self.schema.clone(),
            blocks: self.blocks.clone(),
        }
    }

    /// 🧬️ Builds a full artifact from a snapshot.
    pub fn from_snapshot(snapshot: MdSnapshot) -> Self {
        Self {
            schema: snapshot.schema,
            blocks: snapshot.blocks,
        }
    }

    /// 🔄 Writes persistent fields from a snapshot into this artifact.
    pub fn set_snapshot(&mut self, snapshot: MdSnapshot) {
        self.schema = snapshot.schema;
        self.blocks = snapshot.blocks;
    }
}
//#endregion 🔖️Conversions

//#region 🔖️Descriptor
/// 🧬️ Descriptor for `s.stdio.md`.
pub fn md_artifact_schema_descriptor() -> schema::ArtifactSchemaDescriptor {
    schema::ArtifactSchemaDescriptor {
        id: "s.stdio.md",
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
    use crate::artifacts::md::{MdDiff, MdMutation, MdSnapshot};

    //#region 🔖️Builder
    /// 🏗️ Builds a `stdio.md` snapshot.
    #[derive(Clone, Debug, Default)]
    pub struct MdBuilderConstruction {
        snapshot: MdSnapshot,
        diagnostics: Vec<dsl::Diagnostic>,
    }

    impl ArtifactBuilder for MdBuilderConstruction {
        type Snapshot = MdSnapshot;
        type Mutation = MdMutation;
        type Diff = MdDiff;
        fn empty() -> Self {
            Self { snapshot: MdSnapshot::default(), diagnostics: Vec::new() }
        }
        fn from_snapshot(snapshot: Self::Snapshot) -> Self {
            Self { snapshot, diagnostics: Vec::new() }
        }
        fn from_text(text: &str) -> Result<Self, store::TextError> {
            Ok(Self::from_snapshot(<MdSnapshot as store::ArtifactDsl>::parse_dsl(text)?))
        }
        fn from_binary(bytes: &[u8]) -> Result<Self, store::PackError> {
            Ok(Self::from_snapshot(<MdSnapshot as store::ArtifactPack>::decode_pack(bytes)?))
        }
        fn mutate(mut self, mutation: Self::Mutation) -> (Self, Self::Diff) {
            let diff = crate::artifacts::md::schema::mutations::apply_md_mutation(&mut self.snapshot, &mutation);
            (self, diff)
        }
        fn absorb(mut self, diff: Self::Diff) -> Self {
            self.snapshot = <MdDiff as protocol::MutationDiff<MdSnapshot>>::apply(&diff, &self.snapshot);
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
    use crate::artifacts::md::MdSnapshot;

    //#region 🔖️Parts
    /// 🧩 Analyzed `stdio.md` parts.
    #[derive(Clone, Debug, Default)]
    pub struct MdParts {
        pub snapshot: Option<MdSnapshot>,
    }
    //#endregion 🔖️Parts

    //#region 🔖️Analyzer
    /// 🧐️ Analyzes `stdio.md` (commonmark/✳️any) sources.
    pub struct MdAnalyzerAnalysis;

    /// 🔍 Markdown has no magic bytes — sniff by actually running the real block parser
    /// and checking for structural (non-paragraph) blocks, which plain text never produces.
    fn looks_like_markdown(text: &str) -> IoConfidence {
        if text.trim().is_empty() {
            return IoConfidence::Low;
        }
        let blocks = crate::artifacts::md::engine::parse_markdown_blocks(text);
        if blocks.is_empty() {
            return IoConfidence::Low;
        }
        let has_structure = blocks.iter().any(|b| {
            !matches!(
                b,
                crate::artifacts::md::schema::snapshot::MdBlock::Paragraph { inlines }
                    if inlines.iter().all(|n| matches!(n, crate::artifacts::md::schema::snapshot::MdInline::Text { .. }))
            )
        });
        if has_structure { IoConfidence::High } else { IoConfidence::Medium }
    }

    impl ArtifactAnalysis for MdAnalyzerAnalysis {
        type Parts = MdParts;
        const DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.md", standard: StandardId("commonmark"), subset: SubsetId("*") };

        fn sniff(source: &AnalyzeSource<'_>) -> IoConfidence {
            match source {
                AnalyzeSource::Text(text) => {
                    let body = match store::semio_format::split_text_preamble(text) {
                        Ok((_, rest)) => rest,
                        Err(_) => text,
                    };
                    looks_like_markdown(body)
                }
                AnalyzeSource::Binary(bytes) => match store::semio_format::unwrap_binary(bytes) {
                    Ok((_, inner)) => match String::from_utf8(inner) {
                        Ok(text) => looks_like_markdown(&text),
                        Err(_) => IoConfidence::Low,
                    },
                    Err(_) => match std::str::from_utf8(bytes) {
                        Ok(text) => looks_like_markdown(text),
                        Err(_) => IoConfidence::Low,
                    },
                },
            }
        }

        fn analyze(sources: &[AnalyzeSource<'_>]) -> Analysis<Self::Parts> {
            let mut parts = MdParts::default();
            let mut diagnostics = Vec::new();
            let mut confidence = IoConfidence::High;
            for source in sources {
                match source {
                    AnalyzeSource::Text(text) => match <MdSnapshot as store::ArtifactDsl>::parse_dsl(text) {
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
                    AnalyzeSource::Binary(bytes) => match <MdSnapshot as store::ArtifactPack>::decode_pack(bytes) {
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

    //#region 🧪️Tests
    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn sniff_real_markdown_structure_is_high() {
            let text = "# Title\n\n- one\n- two\n";
            assert_eq!(MdAnalyzerAnalysis::sniff(&AnalyzeSource::Text(text)), IoConfidence::High);
        }

        #[test]
        fn sniff_plain_paragraph_text_is_medium() {
            assert_eq!(MdAnalyzerAnalysis::sniff(&AnalyzeSource::Text("just a plain sentence.")), IoConfidence::Medium);
        }

        #[test]
        fn sniff_empty_is_low() {
            assert_eq!(MdAnalyzerAnalysis::sniff(&AnalyzeSource::Text("")), IoConfidence::Low);
        }
    }
    //#endregion 🧪️Tests
}
pub use derived_analysis::*;
//#endregion 🧐️DerivedAnalysis

//#region 🧬️DerivedArtifactFacets
semio_framework_plugin::derive_artifact_facets!(
    pub spec MdBuilderFacets {
        construction: derived_construction::MdBuilderConstruction,
        analysis: derived_analysis::MdAnalyzerAnalysis,
        composition: super::super::io::derived_composition::MdComposerComposition,
    }
    builder: MdBuilder,
    analyzer: MdAnalyzer,
    composer: MdComposer,
);
//#endregion 🧬️DerivedArtifactFacets
