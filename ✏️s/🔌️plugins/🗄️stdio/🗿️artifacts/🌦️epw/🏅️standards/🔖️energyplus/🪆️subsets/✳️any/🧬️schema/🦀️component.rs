//! 🧬️ EpwArtifact schema — full artifact state, mirrors `EpwSnapshot` field for field.

use crate::artifacts::epw::standards::energyplus::subsets::any::schema::snapshot::{EpwDataPeriods, EpwLocation, EpwRecord, EpwSnapshot};
use schema::ArtifactSchema;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase")]
#[artifact_schema(id = "s.stdio.epw")]
pub struct EpwArtifact {
    #[state(artifact)]
    pub schema: String,
    #[state(artifact)]
    pub location: EpwLocation,
    #[state(artifact)]
    #[serde(default)]
    pub design_conditions: String,
    #[state(artifact)]
    #[serde(default)]
    pub typical_extreme_periods: String,
    #[state(artifact)]
    #[serde(default)]
    pub ground_temperatures: String,
    #[state(artifact)]
    #[serde(default)]
    pub holidays_dst: String,
    #[state(artifact)]
    #[serde(default)]
    pub comments_1: String,
    #[state(artifact)]
    #[serde(default)]
    pub comments_2: String,
    #[state(artifact)]
    pub data_periods: EpwDataPeriods,
    #[state(artifact)]
    #[serde(default)]
    pub records: Vec<EpwRecord>,
}

impl Default for EpwArtifact {
    fn default() -> Self {
        Self::from_snapshot(EpwSnapshot::default())
    }
}

impl EpwArtifact {
    pub fn to_snapshot(&self) -> EpwSnapshot {
        EpwSnapshot {
            schema: self.schema.clone(),
            location: self.location.clone(),
            design_conditions: self.design_conditions.clone(),
            typical_extreme_periods: self.typical_extreme_periods.clone(),
            ground_temperatures: self.ground_temperatures.clone(),
            holidays_dst: self.holidays_dst.clone(),
            comments_1: self.comments_1.clone(),
            comments_2: self.comments_2.clone(),
            data_periods: self.data_periods.clone(),
            records: self.records.clone(),
        }
    }
    pub fn from_snapshot(snapshot: EpwSnapshot) -> Self {
        Self {
            schema: snapshot.schema,
            location: snapshot.location,
            design_conditions: snapshot.design_conditions,
            typical_extreme_periods: snapshot.typical_extreme_periods,
            ground_temperatures: snapshot.ground_temperatures,
            holidays_dst: snapshot.holidays_dst,
            comments_1: snapshot.comments_1,
            comments_2: snapshot.comments_2,
            data_periods: snapshot.data_periods,
            records: snapshot.records,
        }
    }
    pub fn set_snapshot(&mut self, snapshot: EpwSnapshot) {
        self.schema = snapshot.schema;
        self.location = snapshot.location;
        self.design_conditions = snapshot.design_conditions;
        self.typical_extreme_periods = snapshot.typical_extreme_periods;
        self.ground_temperatures = snapshot.ground_temperatures;
        self.holidays_dst = snapshot.holidays_dst;
        self.comments_1 = snapshot.comments_1;
        self.comments_2 = snapshot.comments_2;
        self.data_periods = snapshot.data_periods;
        self.records = snapshot.records;
    }
}

pub fn epw_artifact_schema_descriptor() -> schema::ArtifactSchemaDescriptor {
    schema::ArtifactSchemaDescriptor {
        id: "s.stdio.epw",
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
    use crate::artifacts::epw::standards::energyplus::subsets::any::schema::diff::EpwDiff;
    use crate::artifacts::epw::standards::energyplus::subsets::any::schema::mutations::{apply_epw_mutation, EpwMutation};
    use crate::artifacts::epw::standards::energyplus::subsets::any::schema::snapshot::EpwSnapshot;
    use semio_framework_plugin::ArtifactBuilder;

    #[derive(Clone, Debug, Default)]
    pub struct EpwBuilderConstruction {
        snapshot: EpwSnapshot,
    }

    impl ArtifactBuilder for EpwBuilderConstruction {
        type Snapshot = EpwSnapshot;
        type Mutation = EpwMutation;
        type Diff = EpwDiff;
        fn empty() -> Self {
            Self { snapshot: EpwSnapshot::default() }
        }
        fn from_snapshot(snapshot: Self::Snapshot) -> Self {
            Self { snapshot }
        }
        fn from_text(text: &str) -> Result<Self, store::TextError> {
            Ok(Self::from_snapshot(<EpwSnapshot as store::ArtifactDsl>::parse_dsl(text)?))
        }
        fn from_binary(bytes: &[u8]) -> Result<Self, store::PackError> {
            Ok(Self::from_snapshot(<EpwSnapshot as store::ArtifactPack>::decode_pack(bytes)?))
        }
        fn mutate(mut self, mutation: Self::Mutation) -> (Self, protocol::MutationOutcome<Self::Diff>) {
            let diff = apply_epw_mutation(&mut self.snapshot, &mutation);
            (self, diff)
        }
        fn absorb(mut self, diff: Self::Diff) -> protocol::MutationApplyResult<Self> {
            self.snapshot = <EpwDiff as protocol::MutationDiff<EpwSnapshot>>::apply(&diff, &self.snapshot)?;
            Ok(self)
        }
        fn build(self) -> Result<Self::Snapshot, Vec<dsl::Diagnostic>> {
            Ok(self.snapshot)
        }
    }
}
pub use derived_construction::*;
//#endregion 🏗️DerivedConstruction

//#region 🧐️DerivedAnalysis
pub mod derived_analysis {
    use crate::artifacts::epw::standards::energyplus::subsets::any::io;
    use crate::artifacts::epw::standards::energyplus::subsets::any::schema::snapshot::{EpwSnapshot, STDIO_EPW_DOCUMENT_SCHEMA};
    use semio_framework_plugin::{Analysis, AnalyzeSource, ArtifactAnalysis, Dialect, IoConfidence, StandardId, SubsetId};

    #[derive(Clone, Debug, Default)]
    pub struct EpwParts {
        pub snapshot: Option<EpwSnapshot>,
    }

    pub struct EpwAnalyzerAnalysis;

    impl ArtifactAnalysis for EpwAnalyzerAnalysis {
        type Parts = EpwParts;
        const DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.epw", standard: StandardId("energyplus"), subset: SubsetId("*") };

        fn sniff(source: &AnalyzeSource<'_>) -> IoConfidence {
            match source {
                AnalyzeSource::Binary(bytes) => {
                    if io::sniff_real_bytes(bytes) {
                        return IoConfidence::High;
                    }
                    let marker = STDIO_EPW_DOCUMENT_SCHEMA.as_bytes();
                    if bytes.windows(marker.len().max(1)).any(|w| w == marker) {
                        IoConfidence::High
                    } else {
                        IoConfidence::Low
                    }
                }
                AnalyzeSource::Text(text) => {
                    if io::sniff_real_bytes(text.as_bytes()) || text.contains(STDIO_EPW_DOCUMENT_SCHEMA) {
                        IoConfidence::High
                    } else {
                        IoConfidence::Low
                    }
                }
            }
        }

        fn analyze(sources: &[AnalyzeSource<'_>]) -> Analysis<Self::Parts> {
            let mut parts = EpwParts::default();
            let mut diagnostics = Vec::new();
            let mut confidence = IoConfidence::High;
            for source in sources {
                match source {
                    AnalyzeSource::Text(text) => match <EpwSnapshot as store::ArtifactDsl>::parse_dsl(text) {
                        Ok(snapshot) => parts.snapshot = Some(snapshot),
                        Err(err) => {
                            confidence = IoConfidence::Low;
                            diagnostics.push(dsl::Diagnostic::error("stdio.analyze.text", dsl::TextSpan::at(1, 1), err.to_string()));
                        }
                    },
                    AnalyzeSource::Binary(bytes) => match <EpwSnapshot as store::ArtifactPack>::decode_pack(bytes) {
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
    pub spec EpwBuilderFacets {
        construction: EpwBuilderConstruction,
        analysis: EpwAnalyzerAnalysis,
        composition: super::super::io::derived_composition::EpwComposerComposition,
    }
    builder: EpwBuilder,
    analyzer: EpwAnalyzer,
    composer: EpwComposer,
);
//#endregion 🧬️DerivedArtifactFacets
