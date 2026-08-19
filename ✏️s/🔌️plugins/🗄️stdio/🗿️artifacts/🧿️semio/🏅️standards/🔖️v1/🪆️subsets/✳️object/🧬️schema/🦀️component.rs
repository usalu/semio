//! 🧬️ SemioObjectArtifact schema — full artifact state, mirrors `SemioObjectSnapshot` field for
//! field (see `✳️text`'s `SemioTextArtifact` for the precedent this follows).

use crate::artifacts::semio::standards::v1::subsets::any::schema::geometry::SemioTransform;
use crate::artifacts::semio::standards::v1::subsets::brep::schema::snapshot::SemioBrepSnapshot;
use crate::artifacts::semio::standards::v1::subsets::mesh::schema::snapshot::SemioMeshSnapshot;
use crate::artifacts::semio::standards::v1::subsets::object::schema::snapshot::SemioObjectSnapshot;
use crate::artifacts::semio::standards::v1::subsets::value::schema::snapshot::SemioValueSnapshot;
use schema::ArtifactSchema;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase")]
#[artifact_schema(id = "s.stdio.semio.object")]
pub struct SemioObjectArtifact {
    #[state(artifact)]
    pub schema: String,
    #[state(artifact)]
    pub transform: SemioTransform,
    #[state(artifact)]
    #[child(kind = "s.stdio.semio.brep")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub brep: Option<store::ArtifactChild<SemioBrepSnapshot>>,
    #[state(artifact)]
    #[child(kind = "s.stdio.semio.mesh")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mesh: Option<store::ArtifactChild<SemioMeshSnapshot>>,
    #[state(artifact)]
    #[child(kind = "s.stdio.semio.value")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<store::ArtifactChild<SemioValueSnapshot>>,
}

impl Default for SemioObjectArtifact {
    fn default() -> Self {
        Self::from_snapshot(SemioObjectSnapshot::default())
    }
}

impl SemioObjectArtifact {
    pub async fn to_snapshot(&self) -> SemioObjectSnapshot {
        SemioObjectSnapshot { schema: self.schema.clone(), transform: self.transform.clone(), brep: self.brep.clone(), mesh: self.mesh.clone(), properties: self.properties.clone() }
    }
    pub async fn from_snapshot(snapshot: SemioObjectSnapshot) -> Self {
        Self { schema: snapshot.schema, transform: snapshot.transform, brep: snapshot.brep, mesh: snapshot.mesh, properties: snapshot.properties }
    }
    pub async fn set_snapshot(&mut self, snapshot: SemioObjectSnapshot) {
        self.schema = snapshot.schema;
        self.transform = snapshot.transform;
        self.brep = snapshot.brep;
        self.mesh = snapshot.mesh;
        self.properties = snapshot.properties;
    }
}

pub async fn semio_object_artifact_schema_descriptor() -> schema::ArtifactSchemaDescriptor {
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
    use crate::artifacts::semio::standards::v1::subsets::any::schema::geometry::SemioTransform;
    use crate::artifacts::semio::standards::v1::subsets::object::schema::diff::SemioObjectDiff;
    use crate::artifacts::semio::standards::v1::subsets::object::schema::mutations::SemioObjectMutation;
    use crate::artifacts::semio::standards::v1::subsets::object::schema::snapshot::SemioObjectSnapshot;
    use semio_framework_plugin::ArtifactBuilder;

    #[derive(Clone, Debug, Default)]
    pub struct SemioObjectBuilderConstruction {
        snapshot: SemioObjectSnapshot,
    }

    //#region 🔖️TypedConstructors
    impl SemioObjectBuilderConstruction {
        /// 🏗️ Starts a fresh object at the identity transform, no geometry/properties children.
        pub async fn new() -> Self {
            Self { snapshot: SemioObjectSnapshot::default() }
        }
        /// 🧭️ Overrides the object's placement.
        pub async fn with_transform(mut self, transform: SemioTransform) -> Self {
            self.snapshot.transform = transform;
            self
        }
        /// 🧱️ Attaches an owned brep CHILD handle (never embedded content).
        pub async fn with_brep(mut self, child_id: impl Into<String>, target: store::os_io::ArtifactRef) -> Self {
            self.snapshot.brep = Some(store::ArtifactChild::new(child_id.into(), target));
            self
        }
    }
    //#endregion 🔖️TypedConstructors

    impl ArtifactBuilder for SemioObjectBuilderConstruction {
        type Snapshot = SemioObjectSnapshot;
        type Mutation = SemioObjectMutation;
        type Diff = SemioObjectDiff;
        async fn empty() -> Self {
            Self { snapshot: SemioObjectSnapshot::default() }
        }
        async fn from_snapshot(snapshot: Self::Snapshot) -> Self {
            Self { snapshot }
        }
        async fn from_text(text: &str) -> Result<Self, store::TextError> {
            Ok(Self::from_snapshot(<SemioObjectSnapshot as store::ArtifactDsl>::parse_dsl(text)?))
        }
        async fn from_binary(bytes: &[u8]) -> Result<Self, store::PackError> {
            Ok(Self::from_snapshot(<SemioObjectSnapshot as store::ArtifactPack>::decode_pack(bytes)?))
        }
        async fn mutate(mut self, mutation: Self::Mutation) -> (Self, protocol::MutationOutcome<Self::Diff>) {
            let diff = <Self::Mutation as protocol::Mutation<SemioObjectSnapshot>>::diff(&mutation, &self.snapshot);
            let diff = diff.apply_to(&mut self.snapshot);
            (self, diff)
        }
        async fn absorb(mut self, diff: Self::Diff) -> protocol::MutationApplyResult<Self> {
            self.snapshot = <SemioObjectDiff as protocol::MutationDiff<SemioObjectSnapshot>>::apply(&diff, &self.snapshot)?;
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
            let snapshot = SemioObjectBuilderConstruction::new()
                .with_transform(SemioTransform { translation: crate::artifacts::semio::standards::v1::subsets::any::schema::geometry::SemioPoint3 { x: 1.0, y: 0.0, z: 0.0 }, ..SemioTransform::identity() })
                .with_brep("b1", store::os_io::ArtifactRef { artifact_id: "brep-a".into(), dialect: store::os_io::ArtifactDialect { artifact_kind: "s.stdio.semio".into(), standard: "v1".into(), subset: "brep".into() } })
                .build()
                .expect("build");
            assert_eq!(snapshot.transform.translation.x, 1.0);
            assert_eq!(snapshot.brep.unwrap().child_id, "b1");
        }
    }
    //#endregion 🔖️Tests
}
pub use derived_construction::*;
//#endregion 🏗️DerivedConstruction

//#region 🧐️DerivedAnalysis
pub mod derived_analysis {
    use crate::artifacts::semio::standards::v1::subsets::object::schema::snapshot::{SemioObjectSnapshot, STDIO_SEMIOOBJECT_DOCUMENT_SCHEMA};
    use semio_framework_plugin::{Analysis, AnalyzeSource, ArtifactAnalysis, Dialect, IoConfidence, StandardId, SubsetId};

    #[derive(Clone, Debug, Default)]
    pub struct SemioObjectParts {
        pub snapshot: Option<SemioObjectSnapshot>,
    }

    pub struct SemioObjectAnalyzerAnalysis;

    impl ArtifactAnalysis for SemioObjectAnalyzerAnalysis {
        type Parts = SemioObjectParts;
        const DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.semio", standard: StandardId("v1"), subset: SubsetId("object") };

        async fn sniff(source: &AnalyzeSource<'_>) -> IoConfidence {
            match source {
                AnalyzeSource::Binary(bytes) => {
                    let marker = STDIO_SEMIOOBJECT_DOCUMENT_SCHEMA.as_bytes();
                    if bytes.windows(marker.len().max(1)).any(|w| w == marker) {
                        IoConfidence::High
                    } else {
                        IoConfidence::Low
                    }
                }
                AnalyzeSource::Text(text) => {
                    if text.contains(STDIO_SEMIOOBJECT_DOCUMENT_SCHEMA) {
                        IoConfidence::High
                    } else {
                        IoConfidence::Low
                    }
                }
            }
        }

        async fn analyze(sources: &[AnalyzeSource<'_>]) -> Analysis<Self::Parts> {
            let mut parts = SemioObjectParts::default();
            let mut diagnostics = Vec::new();
            let mut confidence = IoConfidence::High;
            for source in sources {
                match source {
                    AnalyzeSource::Text(text) => match <SemioObjectSnapshot as store::ArtifactDsl>::parse_dsl(text) {
                        Ok(snapshot) => parts.snapshot = Some(snapshot),
                        Err(err) => {
                            confidence = IoConfidence::Low;
                            diagnostics.push(dsl::Diagnostic::error("stdio.analyze.object.text", dsl::TextSpan::at(1, 1), err.to_string()));
                        }
                    },
                    AnalyzeSource::Binary(bytes) => match <SemioObjectSnapshot as store::ArtifactPack>::decode_pack(bytes) {
                        Ok(snapshot) => parts.snapshot = Some(snapshot),
                        Err(err) => {
                            confidence = IoConfidence::Low;
                            diagnostics.push(dsl::Diagnostic::error("stdio.analyze.object.binary", dsl::TextSpan::at(1, 1), err.to_string()));
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
        construction: SemioObjectBuilderConstruction,
        analysis: SemioObjectAnalyzerAnalysis,
        composition: super::super::io::derived_composition::SemioObjectComposerComposition,
    }
    builder: SemioObjectBuilder,
    analyzer: SemioObjectAnalyzer,
    composer: SemioObjectComposer,
);
//#endregion 🧬️DerivedArtifactFacets
