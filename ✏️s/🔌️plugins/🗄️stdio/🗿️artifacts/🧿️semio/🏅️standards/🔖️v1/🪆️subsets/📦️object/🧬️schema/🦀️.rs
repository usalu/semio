//! 🧬️ SemioObjectArtifact schema — full artifact state, mirrors `SemioObjectSnapshot` field for
//! field (see `🔤️text`'s `SemioTextArtifact` for the precedent this follows).

use crate::standards::v1::subsets::base::schema::geometry::SemioTransform;
use crate::standards::v1::subsets::brep::schema::snapshot::SemioBrepSnapshot;
use crate::standards::v1::subsets::mesh::schema::snapshot::SemioMeshSnapshot;
use crate::standards::v1::subsets::object::schema::snapshot::SemioObjectSnapshot;
use crate::standards::v1::subsets::value::schema::snapshot::SemioValueSnapshot;
use framework_schema::ArtifactSchema;

#[derive(Clone, Debug, PartialEq, ArtifactSchema)]
#[artifact_schema(id = "s.stdio.semio.object")]
pub struct SemioObjectArtifact {
    #[state(artifact)]
    pub schema: String,
    #[state(artifact)]
    pub transform: SemioTransform,
    #[state(artifact)]
    #[child(kind = "s.stdio.semio.brep")]
    pub brep: Option<store::ArtifactChild<SemioBrepSnapshot>>,
    #[state(artifact)]
    #[child(kind = "s.stdio.semio.mesh")]
    pub mesh: Option<store::ArtifactChild<SemioMeshSnapshot>>,
    #[state(artifact)]
    #[child(kind = "s.stdio.semio.value")]
    pub properties: Option<store::ArtifactChild<SemioValueSnapshot>>,
}

impl Default for SemioObjectArtifact {
    fn default() -> Self {
        Self::from_snapshot(SemioObjectSnapshot::default())
    }
}

//#region 🔖️ValueCodec
/// 🔀️ Encodes composite child and link fields through their first-party value contracts.
impl dsl::ToValue for SemioObjectArtifact {
    fn to_value(&self) -> dsl::DslValue {
        let mut entries = vec![("schema".to_string(), dsl::ToValue::to_value(&self.schema)), ("transform".to_string(), dsl::ToValue::to_value(&self.transform))];
        if let Some(brep) = &self.brep {
            entries.push(("brep".to_string(), dsl::to_dsl_value(brep).expect("ArtifactChild serializes")));
        }
        if let Some(mesh) = &self.mesh {
            entries.push(("mesh".to_string(), dsl::to_dsl_value(mesh).expect("ArtifactChild serializes")));
        }
        if let Some(properties) = &self.properties {
            entries.push(("properties".to_string(), dsl::to_dsl_value(properties).expect("ArtifactChild serializes")));
        }
        dsl::DslValue::object(entries)
    }
}
impl dsl::FromValue for SemioObjectArtifact {
    fn from_value(value: dsl::DslValue) -> Result<Self, dsl::ValueError> {
        let entries = dsl::DslValue::into_object(value)?;
        let get = |key: &str| entries.iter().find(|(k, _)| k == key).map(|(_, v)| v.clone());
        let field = |key: &str| get(key).ok_or_else(|| dsl::ValueError::new(format!("missing field `{key}`")));
        Ok(Self {
            schema: dsl::FromValue::from_value(field("schema")?)?,
            transform: dsl::FromValue::from_value(field("transform")?)?,
            brep: get("brep").map(dsl::from_dsl_value).transpose().map_err(dsl::ValueError::new)?,
            mesh: get("mesh").map(dsl::from_dsl_value).transpose().map_err(dsl::ValueError::new)?,
            properties: get("properties").map(dsl::from_dsl_value).transpose().map_err(dsl::ValueError::new)?,
        })
    }
}
//#endregion 🔖️ValueCodec

impl SemioObjectArtifact {
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn to_snapshot(&self) -> SemioObjectSnapshot {
        SemioObjectSnapshot { schema: self.schema.clone(), transform: self.transform, brep: self.brep.clone(), mesh: self.mesh.clone(), properties: self.properties.clone() }
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn from_snapshot(snapshot: SemioObjectSnapshot) -> Self {
        Self { schema: snapshot.schema, transform: snapshot.transform, brep: snapshot.brep, mesh: snapshot.mesh, properties: snapshot.properties }
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn set_snapshot(&mut self, snapshot: SemioObjectSnapshot) {
        self.schema = snapshot.schema;
        self.transform = snapshot.transform;
        self.brep = snapshot.brep;
        self.mesh = snapshot.mesh;
        self.properties = snapshot.properties;
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn semio_object_artifact_schema_descriptor() -> framework_schema::ArtifactSchemaDescriptor {
    framework_schema::ArtifactSchemaDescriptor {
        id: "s.stdio.semio.object",
        artifact: framework_schema::FacetLeaves { rust: include_str!("🦀️.rs"), typescript: include_str!("🟦️.ts"), graphql: include_str!("🔗️.graphql"), json_schema: include_str!("🔣️.json"), proto: include_str!("🛰️.proto") },
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
    use crate::standards::v1::subsets::base::schema::geometry::SemioTransform;
    use crate::standards::v1::subsets::object::schema::diff::SemioObjectDiff;
    use crate::standards::v1::subsets::object::schema::mutations::{apply_semio_object_mutation, SemioObjectMutation};
    use crate::standards::v1::subsets::object::schema::snapshot::SemioObjectSnapshot;
    use semio_framework_plugin::ArtifactBuilder;

    #[derive(Clone, Debug, Default)]
    pub struct SemioObjectBuilderConstruction {
        snapshot: SemioObjectSnapshot,
    }

    //#region 🔖️TypedConstructors
    impl SemioObjectBuilderConstruction {
        /// 🏗️ Starts a fresh object at the identity transform, no geometry/properties children.
        // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
        pub fn new() -> Self {
            Self { snapshot: SemioObjectSnapshot::default() }
        }
        /// 🧭️ Overrides the object's placement.
        // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
        pub fn with_transform(mut self, transform: SemioTransform) -> Self {
            self.snapshot.transform = transform;
            self
        }
        /// 🧱️ Attaches an owned brep CHILD handle (never embedded content).
        // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
        pub fn with_brep(mut self, child_id: impl Into<String>, target: store::os_io::ArtifactRef) -> Self {
            self.snapshot.brep = Some(store::ArtifactChild::new(child_id.into(), target));
            self
        }
    }
    //#endregion 🔖️TypedConstructors

    impl ArtifactBuilder for SemioObjectBuilderConstruction {
        type Snapshot = SemioObjectSnapshot;
        type Mutation = SemioObjectMutation;
        type Diff = SemioObjectDiff;
        fn empty() -> Self {
            Self { snapshot: SemioObjectSnapshot::default() }
        }
        fn from_snapshot(snapshot: Self::Snapshot) -> Self {
            Self { snapshot }
        }
        fn from_text(text: &str) -> Result<Self, store::TextError> {
            Ok(Self::from_snapshot(<SemioObjectSnapshot as store::ArtifactDsl>::parse_dsl(text)?))
        }
        fn from_binary(bytes: &[u8]) -> Result<Self, store::PackError> {
            Ok(Self::from_snapshot(<SemioObjectSnapshot as store::ArtifactPack>::decode_pack(bytes)?))
        }
        fn mutate(mut self, mutation: Self::Mutation) -> (Self, protocol::MutationOutcome<Self::Diff>) {
            let diff = apply_semio_object_mutation(&mut self.snapshot, &mutation);
            (self, diff)
        }
        fn absorb(mut self, diff: Self::Diff) -> protocol::MutationApplyResult<Self> {
            self.snapshot = <SemioObjectDiff as protocol::MutationDiff<SemioObjectSnapshot>>::apply(&diff, &self.snapshot)?;
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
    use crate::standards::v1::subsets::object::schema::snapshot::{SemioObjectSnapshot, STDIO_SEMIOOBJECT_DOCUMENT_SCHEMA};
    use semio_framework_plugin::{Analysis, AnalyzeSource, ArtifactAnalysis, Dialect, IoConfidence, StandardId, SubsetId};

    #[derive(Clone, Debug, Default)]
    pub struct SemioObjectParts {
        pub snapshot: Option<SemioObjectSnapshot>,
    }

    pub struct SemioObjectAnalyzerAnalysis;

    impl ArtifactAnalysis for SemioObjectAnalyzerAnalysis {
        type Parts = SemioObjectParts;
        const DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.semio", standard: StandardId("v1"), subset: SubsetId("object") };

        fn sniff(source: &AnalyzeSource<'_>) -> IoConfidence {
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
