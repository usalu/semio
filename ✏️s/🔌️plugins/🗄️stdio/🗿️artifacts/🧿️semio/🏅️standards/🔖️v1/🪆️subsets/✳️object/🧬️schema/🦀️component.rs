//! 🧬️ SemioObjectArtifact schema — full artifact state, mirrors `SemioObjectSnapshot` field for
//! field (see gif's `GifArtifact` for the precedent this follows).

use crate::artifacts::semio::standards::v1::subsets::object::schema::snapshot::{SemioObjectNode, SemioObjectSnapshot, SemioValue};
use schema::ArtifactSchema;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase")]
#[artifact_schema(id = "s.stdio.semio.object")]
pub struct SemioObjectArtifact {
    #[state(persistent)]
    pub schema: String,
    #[state(persistent)]
    pub root: SemioValue,
    #[state(persistent)]
    pub objects: Vec<SemioObjectNode>,
}

impl Default for SemioObjectArtifact {
    fn default() -> Self { Self::from_snapshot(SemioObjectSnapshot::default()) }
}

impl SemioObjectArtifact {
    pub fn to_snapshot(&self) -> SemioObjectSnapshot {
        SemioObjectSnapshot {
            schema: self.schema.clone(),
            root: self.root.clone(),
            objects: self.objects.clone(),
        }
    }
    pub fn from_snapshot(snapshot: SemioObjectSnapshot) -> Self {
        Self {
            schema: snapshot.schema,
            root: snapshot.root,
            objects: snapshot.objects,
        }
    }
    pub fn set_snapshot(&mut self, snapshot: SemioObjectSnapshot) {
        self.schema = snapshot.schema;
        self.root = snapshot.root;
        self.objects = snapshot.objects;
    }
}

pub fn semio_object_artifact_schema_descriptor() -> schema::ArtifactSchemaDescriptor {
    schema::ArtifactSchemaDescriptor {
        id: "s.stdio.semio.object",
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
    use crate::artifacts::semio::standards::v1::subsets::object::schema::diff::SemioObjectDiff;
    use crate::artifacts::semio::standards::v1::subsets::object::schema::mutations::{SemioObjectMutation, apply_semio_object_mutation};
    use crate::artifacts::semio::standards::v1::subsets::object::schema::snapshot::SemioObjectSnapshot;

    #[derive(Clone, Debug, Default)]
    pub struct SemioObjectBuilderConstruction { snapshot: SemioObjectSnapshot }

    impl ArtifactBuilder for SemioObjectBuilderConstruction {
        type Snapshot = SemioObjectSnapshot;
        type Mutation = SemioObjectMutation;
        type Diff = SemioObjectDiff;
        fn empty() -> Self { Self { snapshot: SemioObjectSnapshot::default() } }
        fn from_snapshot(snapshot: Self::Snapshot) -> Self { Self { snapshot } }
        fn from_text(text: &str) -> Result<Self, store::TextError> {
            Ok(Self::from_snapshot(<SemioObjectSnapshot as store::ArtifactDsl>::parse_dsl(text)?))
        }
        fn from_binary(bytes: &[u8]) -> Result<Self, store::PackError> {
            Ok(Self::from_snapshot(<SemioObjectSnapshot as store::ArtifactPack>::decode_pack(bytes)?))
        }
        fn mutate(mut self, mutation: Self::Mutation) -> (Self, Self::Diff) {
            let diff = apply_semio_object_mutation(&mut self.snapshot, &mutation);
            (self, diff)
        }
        fn absorb(mut self, diff: Self::Diff) -> Self {
            self.snapshot = <SemioObjectDiff as protocol::MutationDiff<SemioObjectSnapshot>>::apply(&diff, &self.snapshot);
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
    use crate::artifacts::semio::standards::v1::subsets::object::schema::snapshot::{SemioObjectSnapshot, STDIO_SEMIOOBJECT_DOCUMENT_SCHEMA};

    #[derive(Clone, Debug, Default)]
    pub struct SemioObjectParts { pub snapshot: Option<SemioObjectSnapshot> }

    pub struct SemioObjectAnalyzerAnalysis;

    impl ArtifactAnalysis for SemioObjectAnalyzerAnalysis {
        type Parts = SemioObjectParts;
        const DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.semio", standard: StandardId("v1"), subset: SubsetId("object") };

        fn sniff(source: &AnalyzeSource<'_>) -> IoConfidence {
            match source {
                AnalyzeSource::Binary(bytes) => {
                    let marker = STDIO_SEMIOOBJECT_DOCUMENT_SCHEMA.as_bytes();
                    if bytes.windows(marker.len().max(1)).any(|w| w == marker) { IoConfidence::High } else { IoConfidence::Low }
                }
                AnalyzeSource::Text(text) => {
                    if text.contains(STDIO_SEMIOOBJECT_DOCUMENT_SCHEMA) { IoConfidence::High } else { IoConfidence::Low }
                }
            }
        }

        fn analyze(sources: &[AnalyzeSource<'_>]) -> Analysis<Self::Parts> {
            let mut parts = SemioObjectParts::default();
            let mut diagnostics = Vec::new();
            let mut confidence = IoConfidence::High;
            for source in sources {
                match source {
                    AnalyzeSource::Text(text) => match <SemioObjectSnapshot as store::ArtifactDsl>::parse_dsl(text) {
                        Ok(snapshot) => parts.snapshot = Some(snapshot),
                        Err(err) => {
                            confidence = IoConfidence::Low;
                            diagnostics.push(dsl::Diagnostic::error("stdio.analyze.text", dsl::TextSpan::at(1, 1), err.to_string()));
                        }
                    },
                    AnalyzeSource::Binary(bytes) => match <SemioObjectSnapshot as store::ArtifactPack>::decode_pack(bytes) {
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
    pub spec SemioObjectBuilderFacets {
        construction: derived_construction::SemioObjectBuilderConstruction,
        analysis: derived_analysis::SemioObjectAnalyzerAnalysis,
        composition: super::super::io::derived_composition::SemioObjectComposerComposition,
    }
    builder: SemioObjectBuilder,
    analyzer: SemioObjectAnalyzer,
    composer: SemioObjectComposer,
);
//#endregion 🧬️DerivedArtifactFacets
