//! 🧬️ SemioDocumentArtifact schema — full artifact state, mirrors `SemioDocumentSnapshot` field for
//! field (see gif's `GifArtifact` for the precedent this follows).

use crate::standards::v1::subsets::document::schema::snapshot::{DocBlock, DocImage, DocStyle, SemioDocumentSnapshot};
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
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn to_snapshot(&self) -> SemioDocumentSnapshot {
        SemioDocumentSnapshot { schema: self.schema.clone(), styles: self.styles.clone(), images: self.images.clone(), blocks: self.blocks.clone() }
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn from_snapshot(snapshot: SemioDocumentSnapshot) -> Self {
        Self { schema: snapshot.schema, styles: snapshot.styles, images: snapshot.images, blocks: snapshot.blocks }
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn set_snapshot(&mut self, snapshot: SemioDocumentSnapshot) {
        self.schema = snapshot.schema;
        self.styles = snapshot.styles;
        self.images = snapshot.images;
        self.blocks = snapshot.blocks;
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn semio_document_artifact_schema_descriptor() -> framework_schema::ArtifactSchemaDescriptor {
    framework_schema::ArtifactSchemaDescriptor {
        id: "s.stdio.semio.document",
        artifact: framework_schema::FacetLeaves {
            rust: include_str!("🦀️.rs"),
            typescript: include_str!("🟦️.ts"),
            graphql: include_str!("🔗️.graphql"),
            json_schema: include_str!("🔣️.json"),
            proto: include_str!("🛰️.proto"),
        },
        snapshot: framework_schema::FacetLeaves {
            rust: include_str!("📸️snapshot/🦀️.rs"),
            typescript: include_str!("📸️snapshot/🟦️.ts"),
            graphql: include_str!("📸️snapshot/🔗️.graphql"),
            json_schema: include_str!("📸️snapshot/🔣️.json"),
            proto: include_str!("📸️snapshot/🛰️.proto"),
        },
        diff: framework_schema::FacetLeaves {
            rust: include_str!("🔺️diff/🦀️.rs"),
            typescript: include_str!("🔺️diff/🟦️.ts"),
            graphql: include_str!("🔺️diff/🔗️.graphql"),
            json_schema: include_str!("🔺️diff/🔣️.json"),
            proto: include_str!("🔺️diff/🛰️.proto"),
        },
        mutations: framework_schema::FacetLeaves {
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
    use crate::standards::v1::subsets::document::schema::diff::SemioDocumentDiff;
    use crate::standards::v1::subsets::document::schema::mutations::{apply_semio_document_mutation, SemioDocumentMutation};
    #[cfg(test)]
    use crate::standards::v1::subsets::document::schema::mutations::{insert_style};
    use crate::standards::v1::subsets::document::schema::snapshot::{DocBlock, DocImage, DocStyle, SemioDocumentSnapshot};
    use semio_framework_plugin::ArtifactBuilder;

    #[derive(Clone, Debug, Default)]
    pub struct SemioDocumentBuilderConstruction {
        snapshot: SemioDocumentSnapshot,
    }

    impl SemioDocumentBuilderConstruction {
        /// 🎨️ Fluent: appends a named style.
        // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
        pub fn with_style(mut self, style: DocStyle) -> Self {
            self.snapshot.styles.push(style);
            self
        }
        /// 🖼️ Fluent: appends a named image.
        // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
        pub fn with_image(mut self, image: DocImage) -> Self {
            self.snapshot.images.push(image);
            self
        }
        /// 🧱️ Fluent: appends a top-level block.
        // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
        pub fn with_block(mut self, block: DocBlock) -> Self {
            self.snapshot.blocks.push(block);
            self
        }
    }

    impl ArtifactBuilder for SemioDocumentBuilderConstruction {
        type Snapshot = SemioDocumentSnapshot;
        type Mutation = SemioDocumentMutation;
        type Diff = SemioDocumentDiff;
        fn empty() -> Self {
            Self { snapshot: SemioDocumentSnapshot::default() }
        }
        fn from_snapshot(snapshot: Self::Snapshot) -> Self {
            Self { snapshot }
        }
        fn from_text(text: &str) -> Result<Self, store::TextError> {
            Ok(Self::from_snapshot(<SemioDocumentSnapshot as store::ArtifactDsl>::parse_dsl(text)?))
        }
        fn from_binary(bytes: &[u8]) -> Result<Self, store::PackError> {
            Ok(Self::from_snapshot(<SemioDocumentSnapshot as store::ArtifactPack>::decode_pack(bytes)?))
        }
        fn mutate(mut self, mutation: Self::Mutation) -> (Self, protocol::MutationOutcome<Self::Diff>) {
            let diff = apply_semio_document_mutation(&mut self.snapshot, &mutation);
            (self, diff)
        }
        fn absorb(mut self, diff: Self::Diff) -> protocol::MutationApplyResult<Self> {
            self.snapshot = <SemioDocumentDiff as protocol::MutationDiff<SemioDocumentSnapshot>>::apply(&diff, &self.snapshot)?;
            Ok(self)
        }
        fn build(self) -> Result<Self::Snapshot, Vec<dsl::Diagnostic>> {
            Ok(self.snapshot)
        }
    }

    //#region 🔖️Tests
    #[cfg(test)]
    include!("🧪️tests/🔬️derived-construction-unit/🦀️.rs");
    //#endregion 🔖️Tests
}
pub use derived_construction::*;
//#endregion 🏗️DerivedConstruction

//#region 🧐️DerivedAnalysis
pub mod derived_analysis {
    use crate::standards::v1::subsets::document::schema::snapshot::{SemioDocumentSnapshot, STDIO_SEMIODOCUMENT_DOCUMENT_SCHEMA};
    use semio_framework_plugin::{Analysis, AnalyzeSource, ArtifactAnalysis, Dialect, IoConfidence, StandardId, SubsetId};

    #[derive(Clone, Debug, Default)]
    pub struct SemioDocumentParts {
        pub snapshot: Option<SemioDocumentSnapshot>,
    }

    pub struct SemioDocumentAnalyzerAnalysis;

    impl ArtifactAnalysis for SemioDocumentAnalyzerAnalysis {
        type Parts = SemioDocumentParts;
        const DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.semio", standard: StandardId("v1"), subset: SubsetId("document") };

        fn sniff(source: &AnalyzeSource<'_>) -> IoConfidence {
            match source {
                AnalyzeSource::Binary(bytes) => {
                    let marker = STDIO_SEMIODOCUMENT_DOCUMENT_SCHEMA.as_bytes();
                    if bytes.windows(marker.len().max(1)).any(|w| w == marker) {
                        IoConfidence::High
                    } else {
                        IoConfidence::Low
                    }
                }
                AnalyzeSource::Text(text) => {
                    if text.contains(STDIO_SEMIODOCUMENT_DOCUMENT_SCHEMA) {
                        IoConfidence::High
                    } else {
                        IoConfidence::Low
                    }
                }
            }
        }

        fn analyze(sources: &[AnalyzeSource<'_>]) -> Analysis<Self::Parts> {
            let mut parts = SemioDocumentParts::default();
            let mut diagnostics = Vec::new();
            let mut confidence = IoConfidence::High;
            for source in sources {
                match source {
                    AnalyzeSource::Text(text) => match <SemioDocumentSnapshot as store::ArtifactDsl>::parse_dsl(text) {
                        Ok(snapshot) => parts.snapshot = Some(snapshot),
                        Err(err) => {
                            confidence = IoConfidence::Low;
                            diagnostics.push(dsl::Diagnostic::error("stdio.analyze.text", dsl::TextSpan::at(1, 1), err.to_string()));
                        }
                    },
                    AnalyzeSource::Binary(bytes) => match <SemioDocumentSnapshot as store::ArtifactPack>::decode_pack(bytes) {
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

    //#region 🔖️Tests
    #[cfg(test)]
    include!("🧪️tests/🔬️derived-analysis-unit/🦀️.rs");
    //#endregion 🔖️Tests
}
pub use derived_analysis::*;
//#endregion 🧐️DerivedAnalysis

//#region 🧬️DerivedArtifactFacets
semio_framework_plugin::derive_artifact_facets!(
    pub spec SemioDocumentBuilderFacets {
        construction: SemioDocumentBuilderConstruction,
        analysis: SemioDocumentAnalyzerAnalysis,
        composition: super::super::io::derived_composition::SemioDocumentComposerComposition,
    }
    builder: SemioDocumentBuilder,
    analyzer: SemioDocumentAnalyzer,
    composer: SemioDocumentComposer,
);
//#endregion 🧬️DerivedArtifactFacets
