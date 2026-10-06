//! 🧬️ SemioKitArtifact schema — full artifact state, mirrors `SemioKitSnapshot` field for field
//! (see `🔤️text`'s `SemioTextArtifact`/`📦️object`'s `SemioObjectArtifact` for the precedent).

use crate::standards::v1::subsets::kit::schema::snapshot::{SemioKitDesign, SemioKitSnapshot, SemioKitType};
use crate::standards::v1::subsets::model::schema::snapshot::SemioModelSnapshot;
use crate::standards::v1::subsets::object::schema::snapshot::SemioObjectSnapshot;
use crate::standards::v1::subsets::value::schema::snapshot::SemioValueSnapshot;
use framework_schema::ArtifactSchema;

#[derive(Clone, Debug, PartialEq, ArtifactSchema)]
#[artifact_schema(id = "s.stdio.semio.kit")]
pub struct SemioKitArtifact {
    #[state(artifact)]
    pub schema: String,
    #[state(artifact)]
    pub types: Vec<SemioKitType>,
    #[state(artifact)]
    pub designs: Vec<SemioKitDesign>,
    #[state(artifact)]
    #[child(kind = "s.stdio.semio")]
    pub objects: Vec<store::ArtifactChild<SemioObjectSnapshot>>,
    #[state(artifact)]
    #[child(kind = "s.stdio.semio")]
    pub models: Vec<store::ArtifactChild<SemioModelSnapshot>>,
    #[state(artifact)]
    #[child(kind = "s.stdio.semio")]
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
/// 🔀️ Encodes composite child and link fields through their first-party value contracts.
impl semio_framework_value::ToValue for SemioKitArtifact {
    fn to_value(&self) -> semio_framework_value::DslValue {
        let mut entries = vec![
            ("schema".to_string(), semio_framework_value::ToValue::to_value(&self.schema)),
            ("types".to_string(), semio_framework_value::ToValue::to_value(&self.types)),
            ("designs".to_string(), semio_framework_value::ToValue::to_value(&self.designs)),
            ("objects".to_string(), semio_framework_value::ToValue::to_value(&self.objects)),
            ("models".to_string(), semio_framework_value::ToValue::to_value(&self.models)),
            ("representations".to_string(), semio_framework_value::ToValue::to_value(&self.representations)),
        ];
        if let Some(properties) = &self.properties {
            entries.push(("properties".to_string(), semio_framework_value::ToValue::to_value(properties)));
        }
        semio_framework_value::DslValue::object(entries)
    }
}
impl semio_framework_value::FromValue for SemioKitArtifact {
    fn from_value(value: semio_framework_value::DslValue) -> Result<Self, semio_framework_value::ValueError> {
        let entries = semio_framework_value::DslValue::into_object(value)?;
        let get = |key: &str| entries.iter().find(|(k, _)| k == key).map(|(_, v)| v.clone());
        let field = |key: &str| get(key).ok_or_else(|| semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, format!("missing field `{key}`")));
        Ok(Self {
            schema: semio_framework_value::FromValue::from_value(field("schema")?)?,
            types: get("types").map(semio_framework_value::FromValue::from_value).transpose()?.unwrap_or_default(),
            designs: get("designs").map(semio_framework_value::FromValue::from_value).transpose()?.unwrap_or_default(),
            objects: get("objects").map(semio_framework_value::FromValue::from_value).transpose()?.unwrap_or_default(),
            models: get("models").map(semio_framework_value::FromValue::from_value).transpose()?.unwrap_or_default(),
            properties: get("properties").map(semio_framework_value::FromValue::from_value).transpose()?,
            representations: get("representations").map(semio_framework_value::FromValue::from_value).transpose()?.unwrap_or_default(),
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
pub fn semio_kit_artifact_schema_descriptor() -> semio_framework_schema_registry::ArtifactSchemaDescriptor {
    semio_framework_schema_registry::ArtifactSchemaDescriptor {
        id: "s.stdio.semio.kit",
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

//#region 🧬️DerivedArtifactFacets

//#endregion 🧬️DerivedArtifactFacets
