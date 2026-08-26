//! 🧬️ SemioDocumentArtifact schema — full artifact state, mirrors `SemioDocumentSnapshot` field for
//! field (see gif's `GifArtifact` for the precedent this follows).

use crate::artifacts::semio::standards::v1::subsets::document::schema::snapshot::{DocBlock, DocImage, DocStyle, SemioDocumentSnapshot};
use schema::ArtifactSchema;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase")]
#[artifact_schema(id = "s.stdio.semio.document")]
pub struct SemioDocumentArtifact {
    #[state(artifact)]
    pub schema: String,
    #[state(artifact)]
    #[serde(default)]
    pub styles: Vec<DocStyle>,
    #[state(artifact)]
    #[serde(default)]
    pub images: Vec<DocImage>,
    #[state(artifact)]
    #[serde(default)]
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
pub fn semio_document_artifact_schema_descriptor() -> schema::ArtifactSchemaDescriptor {
    schema::ArtifactSchemaDescriptor {
        id: "s.stdio.semio.document",
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
//#region 🏗️DerivedConstruction
pub mod derived_construction {
    use crate::artifacts::semio::standards::v1::subsets::document::schema::diff::SemioDocumentDiff;
    use crate::artifacts::semio::standards::v1::subsets::document::schema::mutations::{apply_semio_document_mutation, SemioDocumentMutation};
    use crate::artifacts::semio::standards::v1::subsets::document::schema::snapshot::{DocBlock, DocImage, DocStyle, SemioDocumentSnapshot};
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
    mod tests {
        use super::*;

        #[semio_framework_async_macros::async_test]
        async fn fluent_builder_round_trips_through_text_and_binary() {
            let built = SemioDocumentBuilderConstruction::empty()
                .with_style(DocStyle { id: "Normal".into(), name: "Normal".into(), based_on: None })
                .with_image(DocImage { id: "img1".into(), mime: "image/png".into(), bytes: vec![1, 2, 3] })
                .with_block(DocBlock::paragraph("hello"))
                .build()
                .expect("build");
            assert_eq!(built.styles.len(), 1);
            assert_eq!(built.images.len(), 1);
            assert_eq!(built.blocks.len(), 1);

            let text = <SemioDocumentSnapshot as store::ArtifactDsl>::print_dsl(&built);
            let from_text = SemioDocumentBuilderConstruction::from_text(&text).expect("from_text").build().expect("build");
            assert_eq!(from_text, built);

            let bytes = <SemioDocumentSnapshot as store::ArtifactPack>::encode_pack(&built);
            let from_binary = SemioDocumentBuilderConstruction::from_binary(&bytes).expect("from_binary").build().expect("build");
            assert_eq!(from_binary, built);
        }

        #[semio_framework_async_macros::async_test]
        async fn mutate_then_absorb_round_trips() {
            let (builder, diff) = SemioDocumentBuilderConstruction::empty().mutate(SemioDocumentMutation::InsertStyle { style: DocStyle { id: "s".into(), name: "S".into(), based_on: None } });
            let rebuilt = SemioDocumentBuilderConstruction::empty().absorb(diff.diff().clone()).expect("absorb must succeed for a well-formed fixture");
            assert_eq!(builder.build().unwrap(), rebuilt.build().unwrap());
        }
    }
    //#endregion 🔖️Tests
}
pub use derived_construction::*;
//#endregion 🏗️DerivedConstruction

//#region 🧐️DerivedAnalysis
pub mod derived_analysis {
    use crate::artifacts::semio::standards::v1::subsets::document::schema::snapshot::{SemioDocumentSnapshot, STDIO_SEMIODOCUMENT_DOCUMENT_SCHEMA};
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
    mod tests {
        use super::*;
        use crate::artifacts::semio::standards::v1::subsets::document::schema::snapshot::{DocBlock, DocStyle};

        // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
        fn rich_snapshot() -> SemioDocumentSnapshot {
            SemioDocumentSnapshot { schema: STDIO_SEMIODOCUMENT_DOCUMENT_SCHEMA.into(), styles: vec![DocStyle { id: "n".into(), name: "Normal".into(), based_on: None }], images: Vec::new(), blocks: vec![DocBlock::paragraph("hi")] }
        }

        #[semio_framework_async_macros::async_test]
        async fn sniff_detects_own_binary_and_text_payloads() {
            let snap = rich_snapshot();
            let bytes = store::ArtifactPack::encode_pack(&snap);
            assert_eq!(SemioDocumentAnalyzerAnalysis::sniff(&AnalyzeSource::Binary(&bytes)), IoConfidence::High);
            let text = <SemioDocumentSnapshot as store::ArtifactDsl>::print_dsl(&snap);
            assert_eq!(SemioDocumentAnalyzerAnalysis::sniff(&AnalyzeSource::Text(&text)), IoConfidence::High);
            assert_eq!(SemioDocumentAnalyzerAnalysis::sniff(&AnalyzeSource::Binary(b"not a semio document at all")), IoConfidence::Low);
        }

        #[semio_framework_async_macros::async_test]
        async fn analyze_decodes_binary_source_into_snapshot() {
            let snap = rich_snapshot();
            let bytes = store::ArtifactPack::encode_pack(&snap);
            let analysis = SemioDocumentAnalyzerAnalysis::analyze(&[AnalyzeSource::Binary(&bytes)]);
            assert_eq!(analysis.confidence, IoConfidence::High);
            assert_eq!(analysis.parts.snapshot, Some(snap));
        }

        #[semio_framework_async_macros::async_test]
        async fn analyze_reports_low_confidence_on_malformed_text() {
            let analysis = SemioDocumentAnalyzerAnalysis::analyze(&[AnalyzeSource::Text("not valid semio document dsl")]);
            assert_eq!(analysis.confidence, IoConfidence::Low);
            assert!(!analysis.diagnostics.is_empty());
        }
    }
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
