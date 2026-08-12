//! 🧬️ ZipArtifact schema — full artifact state.

use crate::artifacts::zip::schema::snapshot::ZipEntry;
use crate::artifacts::zip::ZipSnapshot;
use schema::ArtifactSchema;
use serde::{Deserialize, Serialize};

//#region Artifact
/// 🧬️ Full `stdio.zip` artifact state.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase")]
#[artifact_schema(id = "s.stdio.zip")]
pub struct ZipArtifact {
    #[state(persistent)]
    pub schema: String,
    #[state(persistent)]
    #[serde(default)]
    pub entries: Vec<ZipEntry>,
    #[state(persistent)]
    #[serde(default)]
    pub comment: String,
}
//#endregion Artifact

//#region Conversions
impl Default for ZipArtifact {
    fn default() -> Self {
        Self::from_snapshot(ZipSnapshot::default())
    }
}

impl ZipArtifact {
    /// 📸️ Persisted subset.
    pub fn to_snapshot(&self) -> ZipSnapshot {
        ZipSnapshot {
            schema: self.schema.clone(),
            entries: self.entries.clone(),
            comment: self.comment.clone(),
        }
    }

    /// 🧬️ Builds a full artifact from a snapshot.
    pub fn from_snapshot(snapshot: ZipSnapshot) -> Self {
        Self {
            schema: snapshot.schema,
            entries: snapshot.entries,
            comment: snapshot.comment,
        }
    }

    /// 🔄 Writes persistent fields from a snapshot into this artifact.
    pub fn set_snapshot(&mut self, snapshot: ZipSnapshot) {
        self.schema = snapshot.schema;
        self.entries = snapshot.entries;
        self.comment = snapshot.comment;
    }
}
//#endregion Conversions

//#region Descriptor
/// 🧬️ Descriptor for `s.stdio.zip`.
pub fn zip_artifact_schema_descriptor() -> schema::ArtifactSchemaDescriptor {
    schema::ArtifactSchemaDescriptor {
        id: "s.stdio.zip",
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
    use semio_framework_plugin::ArtifactBuilder;
    use crate::artifacts::zip::{ZipDiff, ZipMutation, ZipSnapshot};
    use crate::artifacts::zip::schema::snapshot::{ZipCompressionMethod, ZipEntry};

    //#region 🔖️Builder
    /// 🏗️ Builds a `stdio.zip` snapshot.
    #[derive(Clone, Debug, Default)]
    pub struct ZipBuilderConstruction {
        snapshot: ZipSnapshot,
        diagnostics: Vec<dsl::Diagnostic>,
    }

    impl ArtifactBuilder for ZipBuilderConstruction {
        type Snapshot = ZipSnapshot;
        type Mutation = ZipMutation;
        type Diff = ZipDiff;
        fn empty() -> Self {
            Self { snapshot: ZipSnapshot::default(), diagnostics: Vec::new() }
        }
        fn from_snapshot(snapshot: Self::Snapshot) -> Self {
            Self { snapshot, diagnostics: Vec::new() }
        }
        fn from_text(text: &str) -> Result<Self, store::TextError> {
            Ok(Self::from_snapshot(<ZipSnapshot as store::ArtifactDsl>::parse_dsl(text)?))
        }
        fn from_binary(bytes: &[u8]) -> Result<Self, store::PackError> {
            Ok(Self::from_snapshot(<ZipSnapshot as store::ArtifactPack>::decode_pack(bytes)?))
        }
        fn mutate(mut self, mutation: Self::Mutation) -> (Self, Self::Diff) {
            let diff = crate::artifacts::zip::schema::mutations::apply_zip_mutation(&mut self.snapshot, &mutation);
            (self, diff)
        }
        fn absorb(mut self, diff: Self::Diff) -> Self {
            self.snapshot = <ZipDiff as protocol::MutationDiff<ZipSnapshot>>::apply(&diff, &self.snapshot);
            self
        }
        fn build(self) -> Result<Self::Snapshot, Vec<dsl::Diagnostic>> {
            if self.diagnostics.is_empty() { Ok(self.snapshot) } else { Err(self.diagnostics) }
        }
    }
    //#endregion 🔖️Builder

    //#region 🔖️TypedConstructors
    impl ZipBuilderConstruction {
        /// ➕️ Adds a member stored with no compression (method 0).
        pub fn with_stored_entry(mut self, name: impl Into<String>, data: Vec<u8>) -> Self {
            self.snapshot.entries.push(ZipEntry {
                name: name.into(),
                data,
                method: ZipCompressionMethod::Stored,
                ..Default::default()
            });
            self
        }

        /// ➕️ Adds a member compressed via the real deflate codec (method 8).
        pub fn with_deflate_entry(mut self, name: impl Into<String>, data: Vec<u8>) -> Self {
            self.snapshot.entries.push(ZipEntry {
                name: name.into(),
                data,
                method: ZipCompressionMethod::Deflate,
                ..Default::default()
            });
            self
        }

        /// ➕️ Adds a fully-specified member (metadata-faithful construction path).
        pub fn with_entry(mut self, entry: ZipEntry) -> Self {
            self.snapshot.entries.push(entry);
            self
        }

        /// 💬️ Sets the archive-level (EOCD) comment.
        pub fn with_comment(mut self, comment: impl Into<String>) -> Self {
            self.snapshot.comment = comment.into();
            self
        }
    }
    //#endregion 🔖️TypedConstructors
}
pub use derived_construction::*;
//#endregion 🏗️DerivedConstruction

//#region 🧐️DerivedAnalysis
pub mod derived_analysis {
    use semio_framework_plugin::{ArtifactAnalysis, Dialect, StandardId, SubsetId, IoConfidence, Analysis, AnalyzeSource};
    use crate::artifacts::zip::ZipSnapshot;

    //#region 🔖️Parts
    /// 🧩 Analyzed `stdio.zip` parts.
    #[derive(Clone, Debug, Default)]
    pub struct ZipParts {
        pub snapshot: Option<ZipSnapshot>,
    }
    //#endregion 🔖️Parts

    //#region 🔖️Analyzer
    /// 🧐️ Analyzes `stdio.zip` (2.0/✳️any) sources.
    pub struct ZipAnalyzerAnalysis;

    impl ArtifactAnalysis for ZipAnalyzerAnalysis {
        type Parts = ZipParts;
        const DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.zip", standard: StandardId("2.0"), subset: SubsetId("*") };

        fn sniff(source: &AnalyzeSource<'_>) -> IoConfidence {
            // 🕵️ Real sniff: inspects the argument's bytes (magic + a well-formed EOCD), never a
            // constant. `AnalyzeSource::Text` is the hex-envelope DSL form, not raw container bytes,
            // so it can't be magic-sniffed the same way — treated as low confidence here (the DSL
            // envelope preamble, not this sniff, is what actually recognizes it).
            use crate::artifacts::zip::engine::{sniff_zip_bytes, SniffConfidence};
            match source {
                AnalyzeSource::Binary(bytes) => match sniff_zip_bytes(bytes) {
                    SniffConfidence::High => IoConfidence::High,
                    SniffConfidence::Medium => IoConfidence::Medium,
                    SniffConfidence::Low => IoConfidence::Low,
                },
                AnalyzeSource::Text(_) => IoConfidence::Low,
            }
        }

        fn analyze(sources: &[AnalyzeSource<'_>]) -> Analysis<Self::Parts> {
            let mut parts = ZipParts::default();
            let mut diagnostics = Vec::new();
            let mut confidence = IoConfidence::High;
            for source in sources {
                match source {
                    AnalyzeSource::Text(text) => match <ZipSnapshot as store::ArtifactDsl>::parse_dsl(text) {
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
                    AnalyzeSource::Binary(bytes) => match <ZipSnapshot as store::ArtifactPack>::decode_pack(bytes) {
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
    pub spec ZipBuilderFacets {
        construction: derived_construction::ZipBuilderConstruction,
        analysis: derived_analysis::ZipAnalyzerAnalysis,
        composition: super::super::io::derived_composition::ZipComposerComposition,
    }
    builder: ZipBuilder,
    analyzer: ZipAnalyzer,
    composer: ZipComposer,
);
//#endregion 🧬️DerivedArtifactFacets
