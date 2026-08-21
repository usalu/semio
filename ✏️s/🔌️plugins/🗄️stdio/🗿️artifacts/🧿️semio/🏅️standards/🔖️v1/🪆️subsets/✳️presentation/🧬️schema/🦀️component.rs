//! 🧬️ SemioPresentationArtifact schema — full artifact state, mirrors `SemioPresentationSnapshot`
//! field for field (see gif's `GifArtifact` for the precedent this follows).

use crate::artifacts::semio::standards::v1::subsets::presentation::schema::snapshot::{SemioPresentationSnapshot, Slide, SlideLayout, SlideMaster};
use schema::ArtifactSchema;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase")]
#[artifact_schema(id = "s.stdio.semio.presentation")]
pub struct SemioPresentationArtifact {
    #[state(artifact)]
    pub schema: String,
    #[state(artifact)]
    #[serde(default)]
    pub masters: Vec<SlideMaster>,
    #[state(artifact)]
    #[serde(default)]
    pub layouts: Vec<SlideLayout>,
    #[state(artifact)]
    #[serde(default)]
    pub slides: Vec<Slide>,
}

impl Default for SemioPresentationArtifact {
    fn default() -> Self {
        Self::from_snapshot(SemioPresentationSnapshot::default())
    }
}

impl SemioPresentationArtifact {
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn to_snapshot(&self) -> SemioPresentationSnapshot {
        SemioPresentationSnapshot { schema: self.schema.clone(), masters: self.masters.clone(), layouts: self.layouts.clone(), slides: self.slides.clone() }
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn from_snapshot(snapshot: SemioPresentationSnapshot) -> Self {
        Self { schema: snapshot.schema, masters: snapshot.masters, layouts: snapshot.layouts, slides: snapshot.slides }
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn set_snapshot(&mut self, snapshot: SemioPresentationSnapshot) {
        self.schema = snapshot.schema;
        self.masters = snapshot.masters;
        self.layouts = snapshot.layouts;
        self.slides = snapshot.slides;
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn semio_presentation_artifact_schema_descriptor() -> schema::ArtifactSchemaDescriptor {
    schema::ArtifactSchemaDescriptor {
        id: "s.stdio.semio.presentation",
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
    use crate::artifacts::semio::standards::v1::subsets::presentation::schema::diff::SemioPresentationDiff;
    use crate::artifacts::semio::standards::v1::subsets::presentation::schema::mutations::{apply_semio_presentation_mutation, SemioPresentationMutation};
    use crate::artifacts::semio::standards::v1::subsets::presentation::schema::snapshot::SemioPresentationSnapshot;
    use semio_framework_plugin::ArtifactBuilder;

    #[derive(Clone, Debug, Default)]
    pub struct SemioPresentationBuilderConstruction {
        snapshot: SemioPresentationSnapshot,
    }

    impl ArtifactBuilder for SemioPresentationBuilderConstruction {
        type Snapshot = SemioPresentationSnapshot;
        type Mutation = SemioPresentationMutation;
        type Diff = SemioPresentationDiff;
        async fn empty() -> Self {
            Self { snapshot: SemioPresentationSnapshot::default() }
        }
        async fn from_snapshot(snapshot: Self::Snapshot) -> Self {
            Self { snapshot }
        }
        async fn from_text(text: &str) -> Result<Self, store::TextError> {
            Ok(Self::from_snapshot(<SemioPresentationSnapshot as store::ArtifactDsl>::parse_dsl(text).await?).await)
        }
        async fn from_binary(bytes: &[u8]) -> Result<Self, store::PackError> {
            Ok(Self::from_snapshot(<SemioPresentationSnapshot as store::ArtifactPack>::decode_pack(bytes).await?).await)
        }
        async fn mutate(mut self, mutation: Self::Mutation) -> (Self, protocol::MutationOutcome<Self::Diff>) {
            let diff = apply_semio_presentation_mutation(&mut self.snapshot, &mutation);
            (self, diff)
        }
        async fn absorb(mut self, diff: Self::Diff) -> protocol::MutationApplyResult<Self> {
            self.snapshot = <SemioPresentationDiff as protocol::MutationDiff<SemioPresentationSnapshot>>::apply(&diff, &self.snapshot).await?;
            Ok(self)
        }
        async fn build(self) -> Result<Self::Snapshot, Vec<dsl::Diagnostic>> {
            Ok(self.snapshot)
        }
    }

    //#region 🧪️Tests
    #[cfg(test)]
    mod tests {
        use super::*;
        use crate::artifacts::semio::standards::v1::subsets::presentation::schema::snapshot::{Slide, SlideLayout, SlideMaster};

        #[semio_framework_async_macros::async_test]
        async fn empty_from_snapshot_and_build_round_trip() {
            let builder = SemioPresentationBuilderConstruction::empty();
            assert_eq!(builder.clone().build().unwrap(), SemioPresentationSnapshot::default());

            let populated = SemioPresentationSnapshot { masters: vec![SlideMaster { id: "m1".into(), shapes: Vec::new() }], ..Default::default() };
            let builder2 = SemioPresentationBuilderConstruction::from_snapshot(populated.clone());
            assert_eq!(builder2.build().unwrap(), populated);
        }

        #[semio_framework_async_macros::async_test]
        async fn from_text_and_from_binary_round_trip_through_a_mutated_snapshot() {
            let mut snap = SemioPresentationSnapshot::default();
            snap.masters.push(SlideMaster { id: "m1".into(), shapes: Vec::new() });
            snap.layouts.push(SlideLayout { id: "l1".into(), master_id: "m1".into(), shapes: Vec::new() });
            snap.slides.push(Slide { id: "s1".into(), layout_id: Some("l1".into()), shapes: Vec::new(), notes: Vec::new() });

            let text = <SemioPresentationSnapshot as store::ArtifactDsl>::print_dsl(&snap);
            let from_text = SemioPresentationBuilderConstruction::from_text(&text).unwrap().build().unwrap();
            assert_eq!(from_text, snap);

            let bytes = <SemioPresentationSnapshot as store::ArtifactPack>::encode_pack(&snap);
            let from_binary = SemioPresentationBuilderConstruction::from_binary(&bytes).unwrap().build().unwrap();
            assert_eq!(from_binary, snap);
        }

        #[semio_framework_async_macros::async_test]
        async fn mutate_then_absorb_matches_direct_apply() {
            let builder = SemioPresentationBuilderConstruction::empty();
            let mutation = SemioPresentationMutation::InsertMaster { master: SlideMaster { id: "m1".into(), shapes: Vec::new() } };
            let (builder, diff) = builder.mutate(mutation);
            let mutated_snapshot = builder.clone().build().unwrap();
            assert_eq!(mutated_snapshot.masters.len(), 1);

            let reabsorbed = SemioPresentationBuilderConstruction::empty().absorb(diff.diff().clone()).expect("absorb must succeed for a well-formed fixture");
            assert_eq!(reabsorbed.build().unwrap(), mutated_snapshot);
        }
    }
    //#endregion 🧪️Tests
}
pub use derived_construction::*;
//#endregion 🏗️DerivedConstruction

//#region 🧐️DerivedAnalysis
pub mod derived_analysis {
    use crate::artifacts::semio::standards::v1::subsets::presentation::schema::snapshot::{SemioPresentationSnapshot, STDIO_SEMIOPRESENTATION_DOCUMENT_SCHEMA};
    use semio_framework_plugin::{Analysis, AnalyzeSource, ArtifactAnalysis, Dialect, IoConfidence, StandardId, SubsetId};

    #[derive(Clone, Debug, Default)]
    pub struct SemioPresentationParts {
        pub snapshot: Option<SemioPresentationSnapshot>,
    }

    pub struct SemioPresentationAnalyzerAnalysis;

    impl ArtifactAnalysis for SemioPresentationAnalyzerAnalysis {
        type Parts = SemioPresentationParts;
        const DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.semio", standard: StandardId("v1"), subset: SubsetId("presentation") };

        async fn sniff(source: &AnalyzeSource<'_>) -> IoConfidence {
            match source {
                AnalyzeSource::Binary(bytes) => {
                    let marker = STDIO_SEMIOPRESENTATION_DOCUMENT_SCHEMA.as_bytes();
                    if bytes.windows(marker.len().max(1)).any(|w| w == marker) {
                        IoConfidence::High
                    } else {
                        IoConfidence::Low
                    }
                }
                AnalyzeSource::Text(text) => {
                    if text.contains(STDIO_SEMIOPRESENTATION_DOCUMENT_SCHEMA) {
                        IoConfidence::High
                    } else {
                        IoConfidence::Low
                    }
                }
            }
        }

        async fn analyze(sources: &[AnalyzeSource<'_>]) -> Analysis<Self::Parts> {
            let mut parts = SemioPresentationParts::default();
            let mut diagnostics = Vec::new();
            let mut confidence = IoConfidence::High;
            for source in sources {
                match source {
                    AnalyzeSource::Text(text) => match <SemioPresentationSnapshot as store::ArtifactDsl>::parse_dsl(text).await {
                        Ok(snapshot) => parts.snapshot = Some(snapshot),
                        Err(err) => {
                            confidence = IoConfidence::Low;
                            diagnostics.push(dsl::Diagnostic::error("stdio.analyze.text", dsl::TextSpan::at(1, 1), err.to_string()));
                        }
                    },
                    AnalyzeSource::Binary(bytes) => match <SemioPresentationSnapshot as store::ArtifactPack>::decode_pack(bytes).await {
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

    //#region 🧪️Tests
    #[cfg(test)]
    mod tests {
        use super::*;
        use crate::artifacts::semio::standards::v1::subsets::presentation::schema::snapshot::SlideMaster;

        // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
        fn sample() -> SemioPresentationSnapshot {
            SemioPresentationSnapshot { masters: vec![SlideMaster { id: "m1".into(), shapes: Vec::new() }], ..Default::default() }
        }

        #[semio_framework_async_macros::async_test]
        async fn sniff_reports_high_for_real_payloads_low_for_garbage() {
            let bytes = <SemioPresentationSnapshot as store::ArtifactPack>::encode_pack(&sample());
            assert_eq!(SemioPresentationAnalyzerAnalysis::sniff(&AnalyzeSource::Binary(&bytes)), IoConfidence::High);
            assert_eq!(SemioPresentationAnalyzerAnalysis::sniff(&AnalyzeSource::Binary(b"not a presentation")), IoConfidence::Low);

            let text = <SemioPresentationSnapshot as store::ArtifactDsl>::print_dsl(&sample());
            assert_eq!(SemioPresentationAnalyzerAnalysis::sniff(&AnalyzeSource::Text(&text)), IoConfidence::High);
            assert_eq!(SemioPresentationAnalyzerAnalysis::sniff(&AnalyzeSource::Text("garbage")), IoConfidence::Low);
        }

        #[semio_framework_async_macros::async_test]
        async fn analyze_decodes_binary_and_text_sources() {
            let snap = sample();
            let bytes = <SemioPresentationSnapshot as store::ArtifactPack>::encode_pack(&snap);
            let analysis = SemioPresentationAnalyzerAnalysis::analyze(&[AnalyzeSource::Binary(&bytes)]);
            assert_eq!(analysis.confidence, IoConfidence::High);
            assert_eq!(analysis.parts.snapshot, Some(snap.clone()));

            let text = <SemioPresentationSnapshot as store::ArtifactDsl>::print_dsl(&snap);
            let analysis2 = SemioPresentationAnalyzerAnalysis::analyze(&[AnalyzeSource::Text(&text)]);
            assert_eq!(analysis2.parts.snapshot, Some(snap));
        }

        #[semio_framework_async_macros::async_test]
        async fn analyze_flags_low_confidence_on_undecodable_source() {
            let analysis = SemioPresentationAnalyzerAnalysis::analyze(&[AnalyzeSource::Binary(b"garbage")]);
            assert_eq!(analysis.confidence, IoConfidence::Low);
            assert!(!analysis.diagnostics.is_empty());
        }
    }
    //#endregion 🧪️Tests
}
pub use derived_analysis::*;
//#endregion 🧐️DerivedAnalysis

//#region 🧬️DerivedArtifactFacets
semio_framework_plugin::derive_artifact_facets!(
    pub spec SemioPresentationBuilderFacets {
        construction: SemioPresentationBuilderConstruction,
        analysis: SemioPresentationAnalyzerAnalysis,
        composition: super::super::io::derived_composition::SemioPresentationComposerComposition,
    }
    builder: SemioPresentationBuilder,
    analyzer: SemioPresentationAnalyzer,
    composer: SemioPresentationComposer,
);
//#endregion 🧬️DerivedArtifactFacets
