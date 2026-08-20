//! 🧬️ PdfArtifact schema (1.7) — full artifact state.

use crate::artifacts::pdf::standards::v1_7::subsets::any::schema::snapshot::{PdfDictEntry, PdfIndirectObject, PdfInfo, PdfPage, PdfSnapshot};
use schema::ArtifactSchema;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase")]
#[artifact_schema(id = "s.stdio.pdf.1.7")]
pub struct PdfArtifact {
    #[state(artifact)]
    pub schema: String,
    #[state(artifact)]
    #[serde(default)]
    pub declared_version: String,
    #[state(artifact)]
    #[serde(default)]
    pub pages: Vec<PdfPage>,
    #[state(artifact)]
    #[serde(default)]
    pub info: PdfInfo,
    #[state(artifact)]
    #[serde(default)]
    pub objects: Vec<PdfIndirectObject>,
    #[state(artifact)]
    #[serde(default)]
    pub trailer: Vec<PdfDictEntry>,
}

impl Default for PdfArtifact {
    fn default() -> Self {
        Self::from_snapshot(PdfSnapshot::default())
    }
}

impl PdfArtifact {
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn to_snapshot(&self) -> PdfSnapshot {
        PdfSnapshot { schema: self.schema.clone(), declared_version: self.declared_version.clone(), pages: self.pages.clone(), info: self.info.clone(), objects: self.objects.clone(), trailer: self.trailer.clone() }
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn from_snapshot(snapshot: PdfSnapshot) -> Self {
        Self { schema: snapshot.schema, declared_version: snapshot.declared_version, pages: snapshot.pages, info: snapshot.info, objects: snapshot.objects, trailer: snapshot.trailer }
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn set_snapshot(&mut self, snapshot: PdfSnapshot) {
        self.schema = snapshot.schema;
        self.declared_version = snapshot.declared_version;
        self.pages = snapshot.pages;
        self.info = snapshot.info;
        self.objects = snapshot.objects;
        self.trailer = snapshot.trailer;
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn pdf_artifact_schema_descriptor() -> schema::ArtifactSchemaDescriptor {
    schema::ArtifactSchemaDescriptor {
        id: "s.stdio.pdf.1.7",
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
    use crate::artifacts::pdf::standards::v1_7::subsets::any::schema::diff::PdfDiff;
    use crate::artifacts::pdf::standards::v1_7::subsets::any::schema::mutations::{apply_pdf_mutation, PdfMutation};
    use crate::artifacts::pdf::standards::v1_7::subsets::any::schema::snapshot::{PdfInfo, PdfPage, PdfSnapshot};
    use semio_framework_plugin::ArtifactBuilder;

    //#region 🔖️Builder
    /// 🏗️ Builds a `stdio.pdf.1.7` snapshot.
    #[derive(Clone, Debug, Default)]
    pub struct PdfBuilderConstruction {
        snapshot: PdfSnapshot,
        diagnostics: Vec<dsl::Diagnostic>,
    }

    impl PdfBuilderConstruction {
        /// ➕ Typed construction: appends a page (the analyzer→builder round-trip acceptance test's
        /// primary entry point -- requirement #8's `InsertPage`, exposed ergonomically).
        // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
        pub fn add_page(self, page: PdfPage) -> Self {
            let index = self.snapshot.pages.len();
            let (next, _diff) = self.mutate(PdfMutation::InsertPage { index, page });
            next
        }
        // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
        pub fn set_info(self, info: PdfInfo) -> Self {
            let (next, _diff) = self.mutate(PdfMutation::SetInfo { info });
            next
        }
    }

    impl ArtifactBuilder for PdfBuilderConstruction {
        type Snapshot = PdfSnapshot;
        type Mutation = PdfMutation;
        type Diff = PdfDiff;
        async fn empty() -> Self {
            Self { snapshot: PdfSnapshot::default(), diagnostics: Vec::new() }
        }
        async fn from_snapshot(snapshot: Self::Snapshot) -> Self {
            Self { snapshot, diagnostics: Vec::new() }
        }
        async fn from_text(text: &str) -> Result<Self, store::TextError> {
            Ok(Self::from_snapshot(<PdfSnapshot as store::ArtifactDsl>::parse_dsl(text).await?).await)
        }
        async fn from_binary(bytes: &[u8]) -> Result<Self, store::PackError> {
            Ok(Self::from_snapshot(<PdfSnapshot as store::ArtifactPack>::decode_pack(bytes).await?).await)
        }
        async fn mutate(mut self, mutation: Self::Mutation) -> (Self, protocol::MutationOutcome<Self::Diff>) {
            let diff = apply_pdf_mutation(&mut self.snapshot, &mutation);
            (self, diff)
        }
        async fn absorb(mut self, diff: Self::Diff) -> protocol::MutationApplyResult<Self> {
            self.snapshot = <PdfDiff as protocol::MutationDiff<PdfSnapshot>>::apply(&diff, &self.snapshot).await?;
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
    use crate::artifacts::pdf::standards::v1_7::subsets::any::schema::snapshot::PdfSnapshot;
    use semio_framework_plugin::{Analysis, AnalyzeSource, ArtifactAnalysis, Dialect, IoConfidence, StandardId, SubsetId};

    //#region 🔖️Parts
    /// 🧩 Analyzed `stdio.pdf.1.7` parts.
    #[derive(Clone, Debug, Default)]
    pub struct PdfParts {
        pub snapshot: Option<PdfSnapshot>,
    }
    //#endregion 🔖️Parts

    //#region 🔖️Analyzer
    /// 🧐️ Analyzes `stdio.pdf` (1.7/✳️any) sources.
    pub struct PdfAnalyzerAnalysis;

    impl ArtifactAnalysis for PdfAnalyzerAnalysis {
        type Parts = PdfParts;
        const DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.pdf", standard: StandardId("1.7"), subset: SubsetId("*") };

        /// 🔍️ Real sniff (requirement #9): inspects `%PDF-` magic + version probe via
        /// `engine::sniff_pdf`, does not discard its argument.
        async fn sniff(source: &AnalyzeSource<'_>) -> IoConfidence {
            match source {
                AnalyzeSource::Binary(bytes) => match crate::artifacts::pdf::standards::v1_7::subsets::any::io::sniff_pdf(bytes).await {
                    Some(_version) => IoConfidence::High,
                    None => IoConfidence::Low,
                },
                AnalyzeSource::Text(text) => {
                    let body = match store::semio_format::split_text_preamble(text) {
                        Ok((_, rest)) => rest,
                        Err(_) => text,
                    };
                    let hex: String = body.chars().filter(|c| !c.is_whitespace()).take(10).collect();
                    let magic: Vec<u8> = (0..hex.len().min(10)).step_by(2).filter_map(|i| hex.get(i..i + 2)).filter_map(|h| u8::from_str_radix(h, 16).ok()).collect();
                    match crate::artifacts::pdf::standards::v1_7::subsets::any::io::sniff_pdf(&magic).await {
                        Some(_) => IoConfidence::Medium,
                        None => IoConfidence::Low,
                    }
                }
            }
        }

        async fn analyze(sources: &[AnalyzeSource<'_>]) -> Analysis<Self::Parts> {
            let mut parts = PdfParts::default();
            let mut diagnostics = Vec::new();
            let mut confidence = IoConfidence::High;
            for source in sources {
                match source {
                    AnalyzeSource::Text(text) => match if text.as_bytes().starts_with(b"%PDF-") {
                        crate::artifacts::pdf::standards::v1_7::subsets::any::io::decode_pdf(text.as_bytes()).await.map_err(|error| format!("{error:?}"))
                    } else {
                        <PdfSnapshot as store::ArtifactDsl>::parse_dsl(text).await.map_err(|error| error.to_string())
                    } {
                        Ok(snapshot) => parts.snapshot = Some(snapshot),
                        Err(err) => {
                            confidence = IoConfidence::Low;
                            diagnostics.push(dsl::Diagnostic::error("stdio.analyze.text", dsl::TextSpan::at(1, 1), err));
                        }
                    },
                    AnalyzeSource::Binary(bytes) => match if crate::artifacts::pdf::standards::v1_7::subsets::any::io::sniff_pdf(bytes).await.is_some() {
                        crate::artifacts::pdf::standards::v1_7::subsets::any::io::decode_pdf(bytes).await.map_err(|error| format!("{error:?}"))
                    } else {
                        <PdfSnapshot as store::ArtifactPack>::decode_pack(bytes).await.map_err(|error| error.to_string())
                    } {
                        Ok(snapshot) => parts.snapshot = Some(snapshot),
                        Err(err) => {
                            confidence = IoConfidence::Low;
                            diagnostics.push(dsl::Diagnostic::error("stdio.analyze.binary", dsl::TextSpan::at(1, 1), err));
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
    pub spec PdfBuilderFacets {
        construction: PdfBuilderConstruction,
        analysis: PdfAnalyzerAnalysis,
        composition: super::super::io::derived_composition::PdfComposerComposition,
    }
    builder: PdfBuilder,
    analyzer: PdfAnalyzer,
    composer: PdfComposer,
);
//#endregion 🧬️DerivedArtifactFacets
