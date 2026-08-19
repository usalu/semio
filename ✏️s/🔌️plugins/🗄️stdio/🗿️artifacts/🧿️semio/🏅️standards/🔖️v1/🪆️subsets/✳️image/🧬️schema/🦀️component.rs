//! 🧬️ SemioImageArtifact schema — full artifact state, mirrors `SemioImageSnapshot` field for
//! field (see gif's `GifArtifact` for the precedent this follows).

use crate::artifacts::semio::standards::v1::subsets::image::schema::snapshot::{SemioColorspace, SemioImageFrame, SemioImageMetadataEntry, SemioImageSnapshot};
use schema::ArtifactSchema;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase")]
#[artifact_schema(id = "s.stdio.semio.image")]
pub struct SemioImageArtifact {
    #[state(artifact)]
    pub schema: String,
    #[state(artifact)]
    pub width: u32,
    #[state(artifact)]
    pub height: u32,
    #[state(artifact)]
    #[serde(default)]
    pub colorspace: SemioColorspace,
    #[state(artifact)]
    #[serde(default)]
    pub bit_depth: u8,
    #[state(artifact)]
    #[serde(default)]
    pub frames: Vec<SemioImageFrame>,
    #[state(artifact)]
    #[serde(default)]
    pub icc: Option<Vec<u8>>,
    #[state(artifact)]
    #[serde(default)]
    pub metadata: Vec<SemioImageMetadataEntry>,
}

impl Default for SemioImageArtifact {
    fn default() -> Self {
        Self::from_snapshot(SemioImageSnapshot::default())
    }
}

impl SemioImageArtifact {
    pub async fn to_snapshot(&self) -> SemioImageSnapshot {
        SemioImageSnapshot { schema: self.schema.clone(), width: self.width, height: self.height, colorspace: self.colorspace, bit_depth: self.bit_depth, frames: self.frames.clone(), icc: self.icc.clone(), metadata: self.metadata.clone() }
    }
    pub async fn from_snapshot(snapshot: SemioImageSnapshot) -> Self {
        Self { schema: snapshot.schema, width: snapshot.width, height: snapshot.height, colorspace: snapshot.colorspace, bit_depth: snapshot.bit_depth, frames: snapshot.frames, icc: snapshot.icc, metadata: snapshot.metadata }
    }
    pub async fn set_snapshot(&mut self, snapshot: SemioImageSnapshot) {
        self.schema = snapshot.schema;
        self.width = snapshot.width;
        self.height = snapshot.height;
        self.colorspace = snapshot.colorspace;
        self.bit_depth = snapshot.bit_depth;
        self.frames = snapshot.frames;
        self.icc = snapshot.icc;
        self.metadata = snapshot.metadata;
    }
}

pub async fn semio_image_artifact_schema_descriptor() -> schema::ArtifactSchemaDescriptor {
    schema::ArtifactSchemaDescriptor {
        id: "s.stdio.semio.image",
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
    use crate::artifacts::semio::standards::v1::subsets::image::schema::diff::SemioImageDiff;
    use crate::artifacts::semio::standards::v1::subsets::image::schema::mutations::{apply_semio_image_mutation, SemioImageMutation};
    use crate::artifacts::semio::standards::v1::subsets::image::schema::snapshot::{SemioColorspace, SemioImageFrame, SemioImageMetadataEntry, SemioImageSnapshot};
    use semio_framework_plugin::ArtifactBuilder;

    #[derive(Clone, Debug, Default)]
    pub struct SemioImageBuilderConstruction {
        snapshot: SemioImageSnapshot,
    }

    //#region 🔖️TypedConstructors
    impl SemioImageBuilderConstruction {
        /// 🏗️ Starts a fresh image at the given pixel dimensions.
        pub async fn new(width: u32, height: u32) -> Self {
            Self { snapshot: SemioImageSnapshot { width, height, ..SemioImageSnapshot::default() } }
        }
        /// 🏗️ Sets the source colorspace.
        pub async fn set_colorspace(mut self, colorspace: SemioColorspace) -> Self {
            self.snapshot.colorspace = colorspace;
            self
        }
        /// 🏗️ Sets the bit depth.
        pub async fn set_bit_depth(mut self, bit_depth: u8) -> Self {
            self.snapshot.bit_depth = bit_depth;
            self
        }
        /// 🏗️ Appends one frame, in order.
        pub async fn add_frame(mut self, frame: SemioImageFrame) -> Self {
            self.snapshot.frames.push(frame);
            self
        }
        /// 🏗️ Sets the embedded ICC profile (`None` clears it).
        pub async fn set_icc(mut self, icc: Option<Vec<u8>>) -> Self {
            self.snapshot.icc = icc;
            self
        }
        /// 🏗️ Appends one metadata entry.
        pub async fn add_metadata(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
            self.snapshot.metadata.push(SemioImageMetadataEntry { key: key.into(), value: value.into() });
            self
        }
    }
    //#endregion 🔖️TypedConstructors

    impl ArtifactBuilder for SemioImageBuilderConstruction {
        type Snapshot = SemioImageSnapshot;
        type Mutation = SemioImageMutation;
        type Diff = SemioImageDiff;
        async fn empty() -> Self {
            Self { snapshot: SemioImageSnapshot::default() }
        }
        async fn from_snapshot(snapshot: Self::Snapshot) -> Self {
            Self { snapshot }
        }
        async fn from_text(text: &str) -> Result<Self, store::TextError> {
            Ok(Self::from_snapshot(<SemioImageSnapshot as store::ArtifactDsl>::parse_dsl(text)?))
        }
        async fn from_binary(bytes: &[u8]) -> Result<Self, store::PackError> {
            Ok(Self::from_snapshot(<SemioImageSnapshot as store::ArtifactPack>::decode_pack(bytes)?))
        }
        async fn mutate(mut self, mutation: Self::Mutation) -> (Self, protocol::MutationOutcome<Self::Diff>) {
            let diff = apply_semio_image_mutation(&mut self.snapshot, &mutation);
            (self, diff)
        }
        async fn absorb(mut self, diff: Self::Diff) -> protocol::MutationApplyResult<Self> {
            self.snapshot = <SemioImageDiff as protocol::MutationDiff<SemioImageSnapshot>>::apply(&diff, &self.snapshot)?;
            Ok(self)
        }
        async fn build(self) -> Result<Self::Snapshot, Vec<dsl::Diagnostic>> {
            Ok(self.snapshot)
        }
    }

    //#region 🔖️Tests
    #[cfg(test)]
    mod tests {
        use super::*;

        #[semio_framework_async_macros::async_test]
        async fn typed_constructors_build_a_populated_snapshot() {
            let snapshot = SemioImageBuilderConstruction::new(2, 2)
                .set_colorspace(SemioColorspace::Rgba)
                .set_bit_depth(8)
                .add_frame(SemioImageFrame { delay_ms: 0, rgba8: vec![255; 16] })
                .set_icc(Some(vec![1, 2, 3]))
                .add_metadata("Title", "test")
                .build()
                .expect("build");
            assert_eq!(snapshot.width, 2);
            assert_eq!(snapshot.height, 2);
            assert_eq!(snapshot.colorspace, SemioColorspace::Rgba);
            assert_eq!(snapshot.frames.len(), 1);
            assert_eq!(snapshot.icc, Some(vec![1, 2, 3]));
            assert_eq!(snapshot.metadata.len(), 1);
        }
    }
    //#endregion 🔖️Tests
}
pub use derived_construction::*;
//#endregion 🏗️DerivedConstruction

//#region 🧐️DerivedAnalysis
pub mod derived_analysis {
    use crate::artifacts::semio::standards::v1::subsets::image::schema::snapshot::{SemioImageSnapshot, STDIO_SEMIOIMAGE_DOCUMENT_SCHEMA};
    use semio_framework_plugin::{Analysis, AnalyzeSource, ArtifactAnalysis, Dialect, IoConfidence, StandardId, SubsetId};

    #[derive(Clone, Debug, Default)]
    pub struct SemioImageParts {
        pub snapshot: Option<SemioImageSnapshot>,
    }

    pub struct SemioImageAnalyzerAnalysis;

    impl ArtifactAnalysis for SemioImageAnalyzerAnalysis {
        type Parts = SemioImageParts;
        const DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.semio", standard: StandardId("v1"), subset: SubsetId("image") };

        async fn sniff(source: &AnalyzeSource<'_>) -> IoConfidence {
            match source {
                AnalyzeSource::Binary(bytes) => {
                    let marker = STDIO_SEMIOIMAGE_DOCUMENT_SCHEMA.as_bytes();
                    if bytes.windows(marker.len().max(1)).any(|w| w == marker) {
                        IoConfidence::High
                    } else {
                        IoConfidence::Low
                    }
                }
                AnalyzeSource::Text(text) => {
                    if text.contains(STDIO_SEMIOIMAGE_DOCUMENT_SCHEMA) {
                        IoConfidence::High
                    } else {
                        IoConfidence::Low
                    }
                }
            }
        }

        async fn analyze(sources: &[AnalyzeSource<'_>]) -> Analysis<Self::Parts> {
            let mut parts = SemioImageParts::default();
            let mut diagnostics = Vec::new();
            let mut confidence = IoConfidence::High;
            for source in sources {
                match source {
                    AnalyzeSource::Text(text) => match <SemioImageSnapshot as store::ArtifactDsl>::parse_dsl(text) {
                        Ok(snapshot) => parts.snapshot = Some(snapshot),
                        Err(err) => {
                            confidence = IoConfidence::Low;
                            diagnostics.push(dsl::Diagnostic::error("stdio.analyze.text", dsl::TextSpan::at(1, 1), err.to_string()));
                        }
                    },
                    AnalyzeSource::Binary(bytes) => match <SemioImageSnapshot as store::ArtifactPack>::decode_pack(bytes) {
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
    pub spec SemioImageBuilderFacets {
        construction: SemioImageBuilderConstruction,
        analysis: SemioImageAnalyzerAnalysis,
        composition: super::super::io::derived_composition::SemioImageComposerComposition,
    }
    builder: SemioImageBuilder,
    analyzer: SemioImageAnalyzer,
    composer: SemioImageComposer,
);
//#endregion 🧬️DerivedArtifactFacets
