//! 🧬️ EpwArtifact schema — full artifact state, mirrors `EpwSnapshot` field for field.

use crate::artifacts::epw::standards::energyplus::subsets::any::schema::snapshot::{
    EpwDataPeriods, EpwLocation, EpwRecord, EpwSnapshot,
};
use schema::ArtifactSchema;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase")]
#[artifact_schema(id = "s.stdio.epw")]
pub struct EpwArtifact {
    #[state(persistent)]
    pub schema: String,
    #[state(persistent)]
    pub location: EpwLocation,
    #[state(persistent)]
    #[serde(default)]
    pub design_conditions: String,
    #[state(persistent)]
    #[serde(default)]
    pub typical_extreme_periods: String,
    #[state(persistent)]
    #[serde(default)]
    pub ground_temperatures: String,
    #[state(persistent)]
    #[serde(default)]
    pub holidays_dst: String,
    #[state(persistent)]
    #[serde(default)]
    pub comments_1: String,
    #[state(persistent)]
    #[serde(default)]
    pub comments_2: String,
    #[state(persistent)]
    pub data_periods: EpwDataPeriods,
    #[state(persistent)]
    #[serde(default)]
    pub records: Vec<EpwRecord>,
}

impl Default for EpwArtifact {
    fn default() -> Self { Self::from_snapshot(EpwSnapshot::default()) }
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
