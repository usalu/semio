//! 🧬️ SemioKitArtifact schema — full artifact state, mirrors `SemioKitSnapshot` field for field
//! (see `✳️text`'s `SemioTextArtifact`/`✳️object`'s `SemioObjectArtifact` for the precedent).

use crate::artifacts::semio::standards::v1::subsets::kit::schema::snapshot::{SemioKitDesign, SemioKitSnapshot, SemioKitType};
use crate::artifacts::semio::standards::v1::subsets::model::schema::snapshot::SemioModelSnapshot;
use crate::artifacts::semio::standards::v1::subsets::object::schema::snapshot::SemioObjectSnapshot;
use crate::artifacts::semio::standards::v1::subsets::value::schema::snapshot::SemioValueSnapshot;
use schema::ArtifactSchema;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase")]
#[artifact_schema(id = "s.stdio.semio.kit")]
pub struct SemioKitArtifact {
    #[state(artifact)]
    pub schema: String,
    #[state(artifact)]
    pub types: Vec<SemioKitType>,
    #[state(artifact)]
    pub designs: Vec<SemioKitDesign>,
    #[state(artifact)]
    #[child(kind = "s.stdio.semio.object")]
    pub objects: Vec<store::ArtifactChild<SemioObjectSnapshot>>,
    #[state(artifact)]
    #[child(kind = "s.stdio.semio.model")]
    pub models: Vec<store::ArtifactChild<SemioModelSnapshot>>,
    #[state(artifact)]
    #[child(kind = "s.stdio.semio.value")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<store::ArtifactChild<SemioValueSnapshot>>,
    #[state(artifact)]
    #[link_slot(roles("representation"))]
    pub representations: Vec<store::ArtifactLink>,
}

impl Default for SemioKitArtifact {
    fn default() -> Self {
        Self::from_snapshot(SemioKitSnapshot::default())
    }
}

//#region 🔖️ValueCodec
/// 🔀️ Hand-written, not derived: `objects`/`models`/`properties` are `store::ArtifactChild<S>`
/// composed-artifact CHILD handles and `representations` is a `Vec<store::ArtifactLink>` LINK
/// slot — both framework types bridged per-field through `to_dsl_value`/`from_dsl_value`
/// (`🌱️value/🔀️serde`) rather than widening the derive macro. Same pattern as `✳️object`'s
/// `SemioObjectArtifact` and the fan-out playbook's `PlaybookArtifact` reference — `Vec<T>` bridges
/// the identical way `Option<T>` does, `to_dsl_value`/`from_dsl_value` only need `T: Serialize +
/// DeserializeOwned`, which both `ArtifactChild<S>` (`#[serde(bound = "")]`, any `S`) and
/// `ArtifactLink` already are.
impl dsl::ToValue for SemioKitArtifact {
    fn to_value(&self) -> dsl::DslValue {
        dsl::DslValue::object([
            ("schema".to_string(), dsl::ToValue::to_value(&self.schema)),
            ("types".to_string(), dsl::ToValue::to_value(&self.types)),
            ("designs".to_string(), dsl::ToValue::to_value(&self.designs)),
            ("objects".to_string(), dsl::to_dsl_value(&self.objects).expect("ArtifactChild serializes")),
            ("models".to_string(), dsl::to_dsl_value(&self.models).expect("ArtifactChild serializes")),
            ("properties".to_string(), dsl::to_dsl_value(&self.properties).expect("ArtifactChild serializes")),
            ("representations".to_string(), dsl::to_dsl_value(&self.representations).expect("ArtifactLink serializes")),
        ])
    }
}
impl dsl::FromValue for SemioKitArtifact {
    fn from_value(value: dsl::DslValue) -> Result<Self, dsl::ValueError> {
        let entries = dsl::DslValue::into_object(value)?;
        let get = |key: &str| entries.iter().find(|(k, _)| k == key).map(|(_, v)| v.clone());
        let field = |key: &str| get(key).ok_or_else(|| dsl::ValueError::new(format!("missing field `{key}`")));
        Ok(Self {
            schema: dsl::FromValue::from_value(field("schema")?)?,
            types: dsl::FromValue::from_value(field("types")?)?,
            designs: dsl::FromValue::from_value(field("designs")?)?,
            objects: dsl::from_dsl_value(field("objects")?).map_err(dsl::ValueError::new)?,
            models: dsl::from_dsl_value(field("models")?).map_err(dsl::ValueError::new)?,
            properties: dsl::from_dsl_value(field("properties")?).map_err(dsl::ValueError::new)?,
            representations: dsl::from_dsl_value(field("representations")?).map_err(dsl::ValueError::new)?,
        })
    }
}
//#endregion 🔖️ValueCodec

impl SemioKitArtifact {
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn to_snapshot(&self) -> SemioKitSnapshot {
        SemioKitSnapshot {
            schema: self.schema.clone(),
            types: self.types.clone(),
            designs: self.designs.clone(),
            objects: self.objects.clone(),
            models: self.models.clone(),
            properties: self.properties.clone(),
            representations: self.representations.clone(),
        }
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn from_snapshot(snapshot: SemioKitSnapshot) -> Self {
        Self { schema: snapshot.schema, types: snapshot.types, designs: snapshot.designs, objects: snapshot.objects, models: snapshot.models, properties: snapshot.properties, representations: snapshot.representations }
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn set_snapshot(&mut self, snapshot: SemioKitSnapshot) {
        self.schema = snapshot.schema;
        self.types = snapshot.types;
        self.designs = snapshot.designs;
        self.objects = snapshot.objects;
        self.models = snapshot.models;
        self.properties = snapshot.properties;
        self.representations = snapshot.representations;
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn semio_kit_artifact_schema_descriptor() -> schema::ArtifactSchemaDescriptor {
    schema::ArtifactSchemaDescriptor {
        id: "s.stdio.semio.kit",
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
            rust: include_str!("🧬️mutations/🦀️.rs"),
            typescript: include_str!("🧬️mutations/🟦️component.ts"),
            graphql: include_str!("🧬️mutations/🔗️component.graphql"),
            json_schema: include_str!("🧬️mutations/🔣️component.json"),
            proto: include_str!("🧬️mutations/🛰️component.proto"),
        },
    }
}

//#region 🏗️DerivedConstruction
pub mod derived_construction {
    use crate::artifacts::semio::standards::v1::subsets::kit::schema::diff::SemioKitDiff;
    use crate::artifacts::semio::standards::v1::subsets::kit::schema::mutations::{apply_semio_kit_mutation, SemioKitMutation};
    use crate::artifacts::semio::standards::v1::subsets::kit::schema::snapshot::{SemioKitSnapshot, SemioKitType};
    use semio_framework_plugin::ArtifactBuilder;

    #[derive(Clone, Debug, Default)]
    pub struct SemioKitBuilderConstruction {
        snapshot: SemioKitSnapshot,
    }

    //#region 🔖️TypedConstructors
    impl SemioKitBuilderConstruction {
        /// 🏗️ Starts a fresh, empty kit (no types/designs/geometry).
        // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
        pub fn new() -> Self {
            Self { snapshot: SemioKitSnapshot::default() }
        }
        /// 🏷️ Appends one TYPE to the catalog.
        // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
        pub fn add_type(mut self, id: impl Into<String>, name: impl Into<String>, category: impl Into<String>) -> Self {
            self.snapshot.types.push(SemioKitType { id: id.into(), name: name.into(), category: category.into() });
            self
        }
    }
    //#endregion 🔖️TypedConstructors

    impl ArtifactBuilder for SemioKitBuilderConstruction {
        type Snapshot = SemioKitSnapshot;
        type Mutation = SemioKitMutation;
        type Diff = SemioKitDiff;
        fn empty() -> Self {
            Self { snapshot: SemioKitSnapshot::default() }
        }
        fn from_snapshot(snapshot: Self::Snapshot) -> Self {
            Self { snapshot }
        }
        fn from_text(text: &str) -> Result<Self, store::TextError> {
            Ok(Self::from_snapshot(<SemioKitSnapshot as store::ArtifactDsl>::parse_dsl(text)?))
        }
        fn from_binary(bytes: &[u8]) -> Result<Self, store::PackError> {
            Ok(Self::from_snapshot(<SemioKitSnapshot as store::ArtifactPack>::decode_pack(bytes)?))
        }
        fn mutate(mut self, mutation: Self::Mutation) -> (Self, protocol::MutationOutcome<Self::Diff>) {
            let diff = apply_semio_kit_mutation(&mut self.snapshot, &mutation);
            (self, diff)
        }
        fn absorb(mut self, diff: Self::Diff) -> protocol::MutationApplyResult<Self> {
            self.snapshot = <SemioKitDiff as protocol::MutationDiff<SemioKitSnapshot>>::apply(&diff, &self.snapshot)?;
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
        async fn typed_constructors_build_a_populated_snapshot() {
            let snapshot = SemioKitBuilderConstruction::new().add_type("chair", "Chair", "furniture").add_type("table", "Table", "furniture").build().expect("build");
            assert_eq!(snapshot.types.len(), 2);
            assert_eq!(snapshot.types[0].id, "chair");
        }
    }
    //#endregion 🔖️Tests
}
pub use derived_construction::*;
//#endregion 🏗️DerivedConstruction

//#region 🧐️DerivedAnalysis
pub mod derived_analysis {
    use crate::artifacts::semio::standards::v1::subsets::kit::schema::snapshot::{SemioKitSnapshot, STDIO_SEMIOKIT_DOCUMENT_SCHEMA};
    use semio_framework_plugin::{Analysis, AnalyzeSource, ArtifactAnalysis, Dialect, IoConfidence, StandardId, SubsetId};

    #[derive(Clone, Debug, Default)]
    pub struct SemioKitParts {
        pub snapshot: Option<SemioKitSnapshot>,
    }

    pub struct SemioKitAnalyzerAnalysis;

    impl ArtifactAnalysis for SemioKitAnalyzerAnalysis {
        type Parts = SemioKitParts;
        const DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.semio", standard: StandardId("v1"), subset: SubsetId("kit") };

        fn sniff(source: &AnalyzeSource<'_>) -> IoConfidence {
            match source {
                AnalyzeSource::Binary(bytes) => {
                    let marker = STDIO_SEMIOKIT_DOCUMENT_SCHEMA.as_bytes();
                    if bytes.windows(marker.len().max(1)).any(|w| w == marker) {
                        IoConfidence::High
                    } else {
                        IoConfidence::Low
                    }
                }
                AnalyzeSource::Text(text) => {
                    if text.contains(STDIO_SEMIOKIT_DOCUMENT_SCHEMA) {
                        IoConfidence::High
                    } else {
                        IoConfidence::Low
                    }
                }
            }
        }

        fn analyze(sources: &[AnalyzeSource<'_>]) -> Analysis<Self::Parts> {
            let mut parts = SemioKitParts::default();
            let mut diagnostics = Vec::new();
            let mut confidence = IoConfidence::High;
            for source in sources {
                match source {
                    AnalyzeSource::Text(text) => match <SemioKitSnapshot as store::ArtifactDsl>::parse_dsl(text) {
                        Ok(snapshot) => parts.snapshot = Some(snapshot),
                        Err(err) => {
                            confidence = IoConfidence::Low;
                            diagnostics.push(dsl::Diagnostic::error("stdio.analyze.kit.text", dsl::TextSpan::at(1, 1), err.to_string()));
                        }
                    },
                    AnalyzeSource::Binary(bytes) => match <SemioKitSnapshot as store::ArtifactPack>::decode_pack(bytes) {
                        Ok(snapshot) => parts.snapshot = Some(snapshot),
                        Err(err) => {
                            confidence = IoConfidence::Low;
                            diagnostics.push(dsl::Diagnostic::error("stdio.analyze.kit.binary", dsl::TextSpan::at(1, 1), err.to_string()));
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
    pub spec SemioKitBuilderFacets {
        construction: SemioKitBuilderConstruction,
        analysis: SemioKitAnalyzerAnalysis,
        composition: super::super::io::derived_composition::SemioKitComposerComposition,
    }
    builder: SemioKitBuilder,
    analyzer: SemioKitAnalyzer,
    composer: SemioKitComposer,
);
//#endregion 🧬️DerivedArtifactFacets
