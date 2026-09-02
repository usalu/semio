//! 🧬️ SemioTextArtifact schema — full artifact state, mirrors `SemioTextSnapshot` field for
//! field (see `✳️image`'s `SemioImageArtifact` for the precedent this follows).

use crate::artifacts::semio::standards::v1::subsets::text::schema::snapshot::{SemioTextRun, SemioTextSnapshot};
use schema::ArtifactSchema;

#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, ArtifactSchema)]
#[value(rename_all = "camelCase")]
#[artifact_schema(id = "s.stdio.semio.text")]
pub struct SemioTextArtifact {
    #[state(artifact)]
    pub schema: String,
    #[state(artifact)]
    #[value(default)]
    pub runs: Vec<SemioTextRun>,
}

impl Default for SemioTextArtifact {
    fn default() -> Self {
        Self::from_snapshot(SemioTextSnapshot::default())
    }
}

impl SemioTextArtifact {
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn to_snapshot(&self) -> SemioTextSnapshot {
        SemioTextSnapshot { schema: self.schema.clone(), runs: self.runs.clone() }
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn from_snapshot(snapshot: SemioTextSnapshot) -> Self {
        Self { schema: snapshot.schema, runs: snapshot.runs }
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn set_snapshot(&mut self, snapshot: SemioTextSnapshot) {
        self.schema = snapshot.schema;
        self.runs = snapshot.runs;
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn semio_text_artifact_schema_descriptor() -> schema::ArtifactSchemaDescriptor {
    schema::ArtifactSchemaDescriptor {
        id: "s.stdio.semio.text",
        artifact: schema::FacetLeaves {
            rust: include_str!("🦀️.rs"),
            typescript: include_str!("🟦️.ts"),
            graphql: include_str!("🔗️.graphql"),
            json_schema: include_str!("🔣️.json"),
            proto: include_str!("🛰️.proto"),
        },
        snapshot: schema::FacetLeaves {
            rust: include_str!("📸️snapshot/🦀️.rs"),
            typescript: include_str!("📸️snapshot/🟦️.ts"),
            graphql: include_str!("📸️snapshot/🔗️.graphql"),
            json_schema: include_str!("📸️snapshot/🔣️.json"),
            proto: include_str!("📸️snapshot/🛰️.proto"),
        },
        diff: schema::FacetLeaves {
            rust: include_str!("🔺️diff/🦀️.rs"),
            typescript: include_str!("🔺️diff/🟦️.ts"),
            graphql: include_str!("🔺️diff/🔗️.graphql"),
            json_schema: include_str!("🔺️diff/🔣️.json"),
            proto: include_str!("🔺️diff/🛰️.proto"),
        },
        mutations: schema::FacetLeaves {
            rust: include_str!("🧬️mutations/🦀️.rs"),
            typescript: include_str!("🧬️mutations/🟦️.ts"),
            graphql: include_str!("🧬️mutations/🔗️.graphql"),
            json_schema: include_str!("🧬️mutations/🔣️.json"),
            proto: include_str!("🧬️mutations/🛰️.proto"),
        },
    }
}
//#region 🏗️DerivedConstruction
pub mod derived_construction {
    use crate::artifacts::semio::standards::v1::subsets::text::schema::diff::SemioTextDiff;
    use crate::artifacts::semio::standards::v1::subsets::text::schema::mutations::{apply_semio_text_mutation, SemioTextMutation};
    use crate::artifacts::semio::standards::v1::subsets::text::schema::snapshot::{SemioTextMark, SemioTextRun, SemioTextSnapshot};
    use semio_framework_plugin::ArtifactBuilder;

    #[derive(Clone, Debug, Default)]
    pub struct SemioTextBuilderConstruction {
        snapshot: SemioTextSnapshot,
    }

    //#region 🔖️TypedConstructors
    impl SemioTextBuilderConstruction {
        /// 🏗️ Starts a fresh, empty text document.
        // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
        pub fn new() -> Self {
            Self { snapshot: SemioTextSnapshot::default() }
        }
        /// 🏗️ Appends one run, in order.
        // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
        pub fn add_run(mut self, language: impl Into<String>, content: impl Into<String>, marks: Vec<SemioTextMark>) -> Self {
            self.snapshot.runs.push(SemioTextRun { language: language.into(), content: content.into(), marks });
            self
        }
    }
    //#endregion 🔖️TypedConstructors

    impl ArtifactBuilder for SemioTextBuilderConstruction {
        type Snapshot = SemioTextSnapshot;
        type Mutation = SemioTextMutation;
        type Diff = SemioTextDiff;
        fn empty() -> Self {
            Self { snapshot: SemioTextSnapshot::default() }
        }
        fn from_snapshot(snapshot: Self::Snapshot) -> Self {
            Self { snapshot }
        }
        fn from_text(text: &str) -> Result<Self, store::TextError> {
            Ok(Self::from_snapshot(<SemioTextSnapshot as store::ArtifactDsl>::parse_dsl(text)?))
        }
        fn from_binary(bytes: &[u8]) -> Result<Self, store::PackError> {
            Ok(Self::from_snapshot(<SemioTextSnapshot as store::ArtifactPack>::decode_pack(bytes)?))
        }
        fn mutate(mut self, mutation: Self::Mutation) -> (Self, protocol::MutationOutcome<Self::Diff>) {
            let diff = apply_semio_text_mutation(&mut self.snapshot, &mutation);
            (self, diff)
        }
        fn absorb(mut self, diff: Self::Diff) -> protocol::MutationApplyResult<Self> {
            self.snapshot = <SemioTextDiff as protocol::MutationDiff<SemioTextSnapshot>>::apply(&diff, &self.snapshot)?;
            Ok(self)
        }
        fn build(self) -> Result<Self::Snapshot, Vec<dsl::Diagnostic>> {
            Ok(self.snapshot)
        }
    }

    //#region 🔖️Tests
    #[cfg(test)]
    mod tests {
        use super::*;
        use crate::artifacts::semio::standards::v1::subsets::text::schema::snapshot::SemioTextMarkKind;

        #[semio_framework_async_macros::async_test]
        async fn typed_constructors_build_a_populated_snapshot() {
            let snapshot = SemioTextBuilderConstruction::new().add_run("en", "hello", vec![]).add_run("en", "world", vec![SemioTextMark { kind: SemioTextMarkKind::Bold, href: String::new() }]).build().expect("build");
            assert_eq!(snapshot.runs.len(), 2);
            assert_eq!(snapshot.runs[1].marks.len(), 1);
        }
    }
    //#endregion 🔖️Tests
}
pub use derived_construction::*;
//#endregion 🏗️DerivedConstruction

//#region 🧐️DerivedAnalysis
pub mod derived_analysis {
    use crate::artifacts::semio::standards::v1::subsets::text::schema::snapshot::{SemioTextSnapshot, STDIO_SEMIOTEXT_DOCUMENT_SCHEMA};
    use semio_framework_plugin::{Analysis, AnalyzeSource, ArtifactAnalysis, Dialect, IoConfidence, StandardId, SubsetId};

    #[derive(Clone, Debug, Default)]
    pub struct SemioTextParts {
        pub snapshot: Option<SemioTextSnapshot>,
    }

    pub struct SemioTextAnalyzerAnalysis;

    impl ArtifactAnalysis for SemioTextAnalyzerAnalysis {
        type Parts = SemioTextParts;
        const DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.semio", standard: StandardId("v1"), subset: SubsetId("text") };

        fn sniff(source: &AnalyzeSource<'_>) -> IoConfidence {
            match source {
                AnalyzeSource::Binary(bytes) => {
                    let marker = STDIO_SEMIOTEXT_DOCUMENT_SCHEMA.as_bytes();
                    if bytes.windows(marker.len().max(1)).any(|w| w == marker) {
                        IoConfidence::High
                    } else {
                        IoConfidence::Low
                    }
                }
                AnalyzeSource::Text(text) => {
                    if text.contains(STDIO_SEMIOTEXT_DOCUMENT_SCHEMA) {
                        IoConfidence::High
                    } else {
                        IoConfidence::Low
                    }
                }
            }
        }

        fn analyze(sources: &[AnalyzeSource<'_>]) -> Analysis<Self::Parts> {
            let mut parts = SemioTextParts::default();
            let mut diagnostics = Vec::new();
            let mut confidence = IoConfidence::High;
            for source in sources {
                match source {
                    AnalyzeSource::Text(text) => match <SemioTextSnapshot as store::ArtifactDsl>::parse_dsl(text) {
                        Ok(snapshot) => parts.snapshot = Some(snapshot),
                        Err(err) => {
                            confidence = IoConfidence::Low;
                            diagnostics.push(dsl::Diagnostic::error("stdio.analyze.text", dsl::TextSpan::at(1, 1), err.to_string()));
                        }
                    },
                    AnalyzeSource::Binary(bytes) => match <SemioTextSnapshot as store::ArtifactPack>::decode_pack(bytes) {
                        Ok(snapshot) => parts.snapshot = Some(snapshot),
                        Err(err) => {
                            confidence = IoConfidence::Low;
                            diagnostics.push(dsl::Diagnostic::error("stdio.analyze.binary", dsl::TextSpan::at(1, 1), err.to_string()));
                        }
                    },
                }
            }
            Analysis { parts, dialect: Self::DIALECT, confidence, diagnostics }
        }
    }
}
pub use derived_analysis::*;
//#endregion 🧐️DerivedAnalysis

//#region 🧬️DerivedArtifactFacets
semio_framework_plugin::derive_artifact_facets!(
    pub spec SemioTextBuilderFacets {
        construction: SemioTextBuilderConstruction,
        analysis: SemioTextAnalyzerAnalysis,
        composition: super::super::io::derived_composition::SemioTextComposerComposition,
    }
    builder: SemioTextBuilder,
    analyzer: SemioTextAnalyzer,
    composer: SemioTextComposer,
);
//#endregion 🧬️DerivedArtifactFacets
