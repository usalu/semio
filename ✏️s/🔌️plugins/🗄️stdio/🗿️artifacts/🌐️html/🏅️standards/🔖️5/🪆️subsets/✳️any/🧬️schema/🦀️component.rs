//! 🧬️ HtmlArtifact schema — full artifact state, mirrors `HtmlSnapshot` field for field (see
//! svg's `SvgArtifact` for the precedent this follows).

use crate::artifacts::html::standards::v5::subsets::any::schema::snapshot::{HtmlNode, HtmlSnapshot};
use schema::ArtifactSchema;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase")]
#[artifact_schema(id = "s.stdio.html")]
pub struct HtmlArtifact {
    #[state(persistent)]
    pub schema: String,
    #[state(persistent)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub doctype: Option<String>,
    #[state(persistent)]
    pub root: HtmlNode,
}

impl Default for HtmlArtifact {
    fn default() -> Self { Self::from_snapshot(HtmlSnapshot::default()) }
}

impl HtmlArtifact {
    pub fn to_snapshot(&self) -> HtmlSnapshot {
        HtmlSnapshot {
            schema: self.schema.clone(),
            doctype: self.doctype.clone(),
            root: self.root.clone(),
        }
    }
    pub fn from_snapshot(snapshot: HtmlSnapshot) -> Self {
        Self {
            schema: snapshot.schema,
            doctype: snapshot.doctype,
            root: snapshot.root,
        }
    }
    pub fn set_snapshot(&mut self, snapshot: HtmlSnapshot) {
        self.schema = snapshot.schema;
        self.doctype = snapshot.doctype;
        self.root = snapshot.root;
    }
}

pub fn html_artifact_schema_descriptor() -> schema::ArtifactSchemaDescriptor {
    schema::ArtifactSchemaDescriptor {
        id: "s.stdio.html",
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
    use semio_framework_plugin::ArtifactBuilder;
    use crate::artifacts::html::standards::v5::subsets::any::schema::diff::HtmlDiff;
    use crate::artifacts::html::standards::v5::subsets::any::schema::mutations::{HtmlMutation, apply_html_mutation};
    use crate::artifacts::html::standards::v5::subsets::any::schema::snapshot::HtmlSnapshot;

    #[derive(Clone, Debug, Default)]
    pub struct HtmlBuilderConstruction { snapshot: HtmlSnapshot }

    impl ArtifactBuilder for HtmlBuilderConstruction {
        type Snapshot = HtmlSnapshot;
        type Mutation = HtmlMutation;
        type Diff = HtmlDiff;
        fn empty() -> Self { Self { snapshot: HtmlSnapshot::default() } }
        fn from_snapshot(snapshot: Self::Snapshot) -> Self { Self { snapshot } }
        fn from_text(text: &str) -> Result<Self, store::TextError> {
            Ok(Self::from_snapshot(<HtmlSnapshot as store::ArtifactDsl>::parse_dsl(text)?))
        }
        fn from_binary(bytes: &[u8]) -> Result<Self, store::PackError> {
            Ok(Self::from_snapshot(<HtmlSnapshot as store::ArtifactPack>::decode_pack(bytes)?))
        }
        fn mutate(mut self, mutation: Self::Mutation) -> (Self, Self::Diff) {
            let diff = apply_html_mutation(&mut self.snapshot, &mutation);
            (self, diff)
        }
        fn absorb(mut self, diff: Self::Diff) -> Self {
            self.snapshot = <HtmlDiff as protocol::MutationDiff<HtmlSnapshot>>::apply(&diff, &self.snapshot);
            self
        }
        fn build(self) -> Result<Self::Snapshot, Vec<dsl::Diagnostic>> { Ok(self.snapshot) }
    }
}
pub use derived_construction::*;
//#endregion 🏗️DerivedConstruction

//#region 🧐️DerivedAnalysis
pub mod derived_analysis {
    use semio_framework_plugin::{ArtifactAnalysis, Dialect, StandardId, SubsetId, IoConfidence, Analysis, AnalyzeSource};
    use crate::artifacts::html::standards::v5::subsets::any::schema::snapshot::{HtmlSnapshot, STDIO_HTML_DOCUMENT_SCHEMA};
    use crate::artifacts::html::standards::v5::subsets::any::io::import::deserializers as engine;

    #[derive(Clone, Debug, Default)]
    pub struct HtmlParts { pub snapshot: Option<HtmlSnapshot> }

    pub struct HtmlAnalyzerAnalysis;

    impl ArtifactAnalysis for HtmlAnalyzerAnalysis {
        type Parts = HtmlParts;
        const DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.html", standard: StandardId("5"), subset: SubsetId("*") };

        fn sniff(source: &AnalyzeSource<'_>) -> IoConfidence {
            match source {
                AnalyzeSource::Binary(bytes) => {
                    if engine::sniff_real_bytes(bytes) {
                        return IoConfidence::High;
                    }
                    let marker = STDIO_HTML_DOCUMENT_SCHEMA.as_bytes();
                    if bytes.windows(marker.len().max(1)).any(|w| w == marker) { IoConfidence::High } else { IoConfidence::Low }
                }
                AnalyzeSource::Text(text) => {
                    if engine::sniff_real_bytes(text.as_bytes()) || text.contains(STDIO_HTML_DOCUMENT_SCHEMA) {
                        IoConfidence::High
                    } else {
                        IoConfidence::Low
                    }
                }
            }
        }

        fn analyze(sources: &[AnalyzeSource<'_>]) -> Analysis<Self::Parts> {
            let mut parts = HtmlParts::default();
            let mut diagnostics = Vec::new();
            let mut confidence = IoConfidence::High;
            for source in sources {
                match source {
                    AnalyzeSource::Text(text) => match <HtmlSnapshot as store::ArtifactDsl>::parse_dsl(text) {
                        Ok(snapshot) => parts.snapshot = Some(snapshot),
                        Err(err) => {
                            confidence = IoConfidence::Low;
                            diagnostics.push(dsl::Diagnostic::error("stdio.analyze.text", dsl::TextSpan::at(1, 1), err.to_string()));
                        }
                    },
                    AnalyzeSource::Binary(bytes) => match <HtmlSnapshot as store::ArtifactPack>::decode_pack(bytes) {
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
    pub spec HtmlBuilderFacets {
        construction: derived_construction::HtmlBuilderConstruction,
        analysis: derived_analysis::HtmlAnalyzerAnalysis,
        composition: super::super::io::derived_composition::HtmlComposerComposition,
    }
    builder: HtmlBuilder,
    analyzer: HtmlAnalyzer,
    composer: HtmlComposer,
);
//#endregion 🧬️DerivedArtifactFacets
