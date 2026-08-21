//! 🧬️ BcfArtifact schema — full artifact state.

use crate::artifacts::bcf::schema::snapshot::{BcfRawPart, BcfTopic};
use crate::artifacts::bcf::BcfSnapshot;
use schema::ArtifactSchema;
use serde::{Deserialize, Serialize};

//#region Artifact
/// 🧬️ Full `stdio.bcf` artifact state.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase")]
#[artifact_schema(id = "s.stdio.bcf")]
pub struct BcfArtifact {
    #[state(artifact)]
    pub schema: String,
    #[state(artifact)]
    #[serde(default)]
    pub version: String,
    #[state(artifact)]
    #[serde(default)]
    pub topics: Vec<BcfTopic>,
    #[state(artifact)]
    #[serde(default)]
    pub parts: Vec<BcfRawPart>,
}
//#endregion Artifact

//#region Conversions
impl Default for BcfArtifact {
    fn default() -> Self {
        Self::from_snapshot(BcfSnapshot::default())
    }
}

impl BcfArtifact {
    /// 📸️ Persisted subset.
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn to_snapshot(&self) -> BcfSnapshot {
        BcfSnapshot { schema: self.schema.clone(), version: self.version.clone(), topics: self.topics.clone(), parts: self.parts.clone() }
    }

    /// 🧬️ Builds a full artifact from a snapshot.
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn from_snapshot(snapshot: BcfSnapshot) -> Self {
        Self { schema: snapshot.schema, version: snapshot.version, topics: snapshot.topics, parts: snapshot.parts }
    }

    /// 🔄 Writes persistent fields from a snapshot into this artifact.
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn set_snapshot(&mut self, snapshot: BcfSnapshot) {
        self.schema = snapshot.schema;
        self.version = snapshot.version;
        self.topics = snapshot.topics;
        self.parts = snapshot.parts;
    }
}
//#endregion Conversions

//#region Descriptor
/// 🧬️ Descriptor for `s.stdio.bcf`.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn bcf_artifact_schema_descriptor() -> schema::ArtifactSchemaDescriptor {
    schema::ArtifactSchemaDescriptor {
        id: "s.stdio.bcf",
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
//#endregion Descriptor
//#region 🏗️DerivedConstruction
pub mod derived_construction {
    use crate::artifacts::bcf::{BcfDiff, BcfMutation, BcfSnapshot};
    use semio_framework_plugin::ArtifactBuilder;

    //#region 🔖️Builder
    /// 🏗️ Builds a `stdio.bcf` snapshot.
    #[derive(Clone, Debug, Default)]
    pub struct BcfBuilderConstruction {
        snapshot: BcfSnapshot,
        diagnostics: Vec<dsl::Diagnostic>,
    }

    impl ArtifactBuilder for BcfBuilderConstruction {
        type Snapshot = BcfSnapshot;
        type Mutation = BcfMutation;
        type Diff = BcfDiff;
        async fn empty() -> Self {
            Self { snapshot: BcfSnapshot::default(), diagnostics: Vec::new() }
        }
        async fn from_snapshot(snapshot: Self::Snapshot) -> Self {
            Self { snapshot, diagnostics: Vec::new() }
        }
        async fn from_text(text: &str) -> Result<Self, store::TextError> {
            Ok(Self::from_snapshot(<BcfSnapshot as store::ArtifactDsl>::parse_dsl(text)?).await)
        }
        async fn from_binary(bytes: &[u8]) -> Result<Self, store::PackError> {
            Ok(Self::from_snapshot(<BcfSnapshot as store::ArtifactPack>::decode_pack(bytes)?).await)
        }
        async fn mutate(mut self, mutation: Self::Mutation) -> (Self, protocol::MutationOutcome<Self::Diff>) {
            let diff = crate::artifacts::bcf::schema::mutations::apply_bcf_mutation(&mut self.snapshot, &mutation);
            (self, diff)
        }
        async fn absorb(mut self, diff: Self::Diff) -> protocol::MutationApplyResult<Self> {
            self.snapshot = <BcfDiff as protocol::MutationDiff<BcfSnapshot>>::apply(&diff, &self.snapshot)?;
            Ok(self)
        }
        async fn build(self) -> Result<Self::Snapshot, Vec<dsl::Diagnostic>> {
            if self.diagnostics.is_empty() {
                Ok(self.snapshot)
            } else {
                Err(self.diagnostics)
            }
        }
    }
    //#endregion 🔖️Builder
}
pub use derived_construction::*;
//#endregion 🏗️DerivedConstruction

//#region 🧐️DerivedAnalysis
pub mod derived_analysis {
    use crate::artifacts::bcf::BcfSnapshot;
    use semio_framework_plugin::{Analysis, AnalyzeSource, ArtifactAnalysis, Dialect, IoConfidence, StandardId, SubsetId};

    //#region 🔖️Parts
    /// 🧩 Analyzed `stdio.bcf` parts.
    #[derive(Clone, Debug, Default)]
    pub struct BcfParts {
        pub snapshot: Option<BcfSnapshot>,
    }
    //#endregion 🔖️Parts

    //#region 🔖️Analyzer
    /// 🧐️ Analyzes `stdio.bcf` (2.1/✳️any) sources.
    pub struct BcfAnalyzerAnalysis;

    impl ArtifactAnalysis for BcfAnalyzerAnalysis {
        type Parts = BcfParts;
        const DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.bcf", standard: StandardId("2.1"), subset: SubsetId("*") };

        async fn sniff(source: &AnalyzeSource<'_>) -> IoConfidence {
            // 🕵️ Real sniff: BCF is a zip container that additionally carries a root `bcf.version`
            // entry. Reuses the zip artifact's own byte-level magic+EOCD check (never reimplemented
            // here) for the base confidence, then cheaply corroborates the `bcf.version` entry name
            // via a substring scan of the raw bytes -- filenames are stored as literal bytes in both
            // the local and central-directory headers, so this finds a real entry name without
            // paying for a full `decode_zip` (which would also inflate every snapshot PNG payload
            // just to read names -- the same cost tradeoff the zip analyzer's own sniff makes by
            // stopping at "does a well-formed EOCD exist" rather than parsing every entry).
            use crate::artifacts::zip::standards::v2_0::subsets::any::io::{sniff_zip_bytes, SniffConfidence};
            match source {
                AnalyzeSource::Binary(bytes) => match sniff_zip_bytes(bytes).await {
                    SniffConfidence::Low => IoConfidence::Low,
                    zip_confidence => {
                        let needle = b"bcf.version";
                        let has_bcf_version_name = bytes.len() >= needle.len() && bytes.windows(needle.len()).any(|w| w == needle);
                        match (zip_confidence, has_bcf_version_name) {
                            (SniffConfidence::High, true) => IoConfidence::High,
                            (SniffConfidence::High, false) => IoConfidence::Medium,
                            (SniffConfidence::Medium, _) => IoConfidence::Medium,
                            (SniffConfidence::Low, _) => unreachable!("Low was matched above"),
                        }
                    }
                },
                // The DSL envelope (hex-wrapped text) preamble is what actually recognizes the text
                // form, not this byte-magic sniff.
                AnalyzeSource::Text(_) => IoConfidence::Low,
            }
        }

        async fn analyze(sources: &[AnalyzeSource<'_>]) -> Analysis<Self::Parts> {
            let mut parts = BcfParts::default();
            let mut diagnostics = Vec::new();
            let mut confidence = IoConfidence::High;
            for source in sources {
                match source {
                    AnalyzeSource::Text(text) => match <BcfSnapshot as store::ArtifactDsl>::parse_dsl(text) {
                        Ok(snapshot) => parts.snapshot = Some(snapshot),
                        Err(err) => {
                            confidence = IoConfidence::Low;
                            diagnostics.push(dsl::Diagnostic::error("stdio.analyze.text", dsl::TextSpan::at(1, 1), err.to_string()));
                        }
                    },
                    AnalyzeSource::Binary(bytes) => match <BcfSnapshot as store::ArtifactPack>::decode_pack(bytes) {
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
    //#endregion 🔖️Analyzer

    //#region 🧪️Tests
    #[cfg(test)]
    mod tests {
        use super::*;
        #[semio_framework_async_macros::async_test]
        async fn sniff_bumps_to_high_when_bcf_version_entry_name_is_present() {
            let snap = BcfSnapshot { schema: "stdio.bcf".into(), version: "2.1".into(), topics: Vec::new(), parts: Vec::new() };
            let bytes = crate::artifacts::bcf::io::encode_bcf(&snap).expect("encode");
            assert_eq!(BcfAnalyzerAnalysis::sniff(&AnalyzeSource::Binary(&bytes)), IoConfidence::High);
        }

        #[semio_framework_async_macros::async_test]
        async fn sniff_stays_medium_for_a_real_zip_without_bcf_version() {
            let zip_snap = crate::artifacts::zip::ZipSnapshot {
                schema: crate::artifacts::zip::STDIO_ZIP_DOCUMENT_SCHEMA.into(),
                entries: vec![crate::artifacts::zip::schema::snapshot::ZipEntry { name: "unrelated.txt".into(), data: b"not a bcf archive".to_vec(), ..Default::default() }],
                comment: String::new(),
            };
            let bytes = crate::artifacts::zip::standards::v2_0::subsets::any::io::encode_zip(&zip_snap).expect("encode plain zip");
            assert_eq!(BcfAnalyzerAnalysis::sniff(&AnalyzeSource::Binary(&bytes)), IoConfidence::Medium);
        }

        #[semio_framework_async_macros::async_test]
        async fn sniff_rejects_non_zip_garbage() {
            assert_eq!(BcfAnalyzerAnalysis::sniff(&AnalyzeSource::Binary(b"not a zip at all")), IoConfidence::Low);
        }

        #[semio_framework_async_macros::async_test]
        async fn sniff_treats_text_source_as_low() {
            assert_eq!(BcfAnalyzerAnalysis::sniff(&AnalyzeSource::Text("deadbeef")), IoConfidence::Low);
        }
    }
    //#endregion 🧪️Tests
}
pub use derived_analysis::*;
//#endregion 🧐️DerivedAnalysis

//#region 🧬️DerivedArtifactFacets
semio_framework_plugin::derive_artifact_facets!(
    pub spec BcfBuilderFacets {
        construction: BcfBuilderConstruction,
        analysis: BcfAnalyzerAnalysis,
        composition: super::super::io::derived_composition::BcfComposerComposition,
    }
    builder: BcfBuilder,
    analyzer: BcfAnalyzer,
    composer: BcfComposer,
);
//#endregion 🧬️DerivedArtifactFacets
