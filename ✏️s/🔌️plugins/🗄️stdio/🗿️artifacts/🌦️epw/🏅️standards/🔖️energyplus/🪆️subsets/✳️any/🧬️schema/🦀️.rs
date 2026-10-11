//! 🧬️ EpwArtifact schema — full artifact state, mirrors `EpwSnapshot` field for field.

use crate::standards::energyplus::subsets::any::schema::snapshot::{EpwDataPeriods, EpwLocation, EpwRecord, EpwSnapshot};
use framework_schema::ArtifactSchema;

#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, ArtifactSchema, semio_framework_value::RetireOwned)]
#[value(rename_all = "camelCase")]
#[artifact_schema(id = "s.stdio.epw")]
pub struct EpwArtifact {
    #[state(artifact)]
    pub schema: String,
    #[state(artifact)]
    pub location: EpwLocation,
    #[state(artifact)]
    #[value(default)]
    pub design_conditions: String,
    #[state(artifact)]
    #[value(default)]
    pub typical_extreme_periods: String,
    #[state(artifact)]
    #[value(default)]
    pub ground_temperatures: String,
    #[state(artifact)]
    #[value(default)]
    pub holidays_dst: String,
    #[state(artifact)]
    #[value(default)]
    pub comments_1: String,
    #[state(artifact)]
    #[value(default)]
    pub comments_2: String,
    #[state(artifact)]
    pub data_periods: EpwDataPeriods,
    #[state(artifact)]
    #[value(default)]
    pub records: Vec<EpwRecord>,
}

impl Default for EpwArtifact {
    fn default() -> Self {
        Self::from_snapshot(EpwSnapshot::default())
    }
}

impl EpwArtifact {
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
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
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
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
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
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

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn epw_artifact_schema_descriptor() -> semio_framework_schema_registry::ArtifactSchemaDescriptor {
    semio_framework_schema_registry::ArtifactSchemaDescriptor {
        id: "s.stdio.epw",
        artifact: semio_framework_schema_registry::FacetLeaves { rust: include_str!("🦀️.rs"), typescript: include_str!("🟦️.ts"), graphql: include_str!("🔗️.graphql"), json_schema: include_str!("🔣️.json"), proto: include_str!("🛰️.proto") },
        snapshot: semio_framework_schema_registry::FacetLeaves {
            rust: include_str!("📸️snapshot/🦀️.rs"),
            typescript: include_str!("📸️snapshot/🟦️.ts"),
            graphql: include_str!("📸️snapshot/🔗️.graphql"),
            json_schema: include_str!("📸️snapshot/🔣️.json"),
            proto: include_str!("📸️snapshot/🛰️.proto"),
        },
        diff: semio_framework_schema_registry::FacetLeaves {
            rust: include_str!("🔺️diff/🦀️.rs"),
            typescript: include_str!("🔺️diff/🟦️.ts"),
            graphql: include_str!("🔺️diff/🔗️.graphql"),
            json_schema: include_str!("🔺️diff/🔣️.json"),
            proto: include_str!("🔺️diff/🛰️.proto"),
        },
        mutations: semio_framework_schema_registry::FacetLeaves {
            rust: include_str!("🧬️mutations/🦀️.rs"),
            typescript: include_str!("🧬️mutations/🟦️.ts"),
            graphql: include_str!("🧬️mutations/🔗️.graphql"),
            json_schema: include_str!("🧬️mutations/🔣️.json"),
            proto: include_str!("🧬️mutations/🛰️.proto"),
        },
    }
}
//#region 🏗️DerivedConstruction

//#endregion 🏗️DerivedConstruction

//#region 🧐️DerivedAnalysis

//#endregion 🧐️DerivedAnalysis

//#region 🆕️NewDocument
/// 🆕️ A new epw document: the eight EnergyPlus header records (empty location, no design conditions / periods / ground
/// temperatures / holidays / comments, no data period) and one data record, as the real codec round-trips it — `decode_epw`
/// requires all eight header records and at least one record, and a new document must save and reopen as itself.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn blank_epw_snapshot() -> EpwSnapshot {
    let seed = EpwSnapshot {
        design_conditions: "DESIGN CONDITIONS,0".into(),
        typical_extreme_periods: "TYPICAL/EXTREME PERIODS,0".into(),
        ground_temperatures: "GROUND TEMPERATURES,0".into(),
        holidays_dst: "HOLIDAYS/DAYLIGHT SAVINGS,No,0,0,0".into(),
        comments_1: "COMMENTS 1,".into(),
        comments_2: "COMMENTS 2,".into(),
        records: vec![EpwRecord::default()],
        ..EpwSnapshot::default()
    };
    seed
}
//#endregion 🆕️NewDocument

//#region 🧬️DerivedArtifactFacets

//#endregion 🧬️DerivedArtifactFacets
