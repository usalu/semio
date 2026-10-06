//! 🧬️ SemioObjectArtifact schema — full artifact state, mirrors `SemioObjectSnapshot` field for
//! field (see `🔤️text`'s `SemioTextArtifact` for the precedent this follows).

use crate::standards::v1::subsets::base::schema::geometry::SemioTransform;
use crate::standards::v1::subsets::brep::schema::snapshot::SemioBrepSnapshot;
use crate::standards::v1::subsets::mesh::schema::snapshot::SemioMeshSnapshot;
use crate::standards::v1::subsets::object::schema::snapshot::SemioObjectSnapshot;
use crate::standards::v1::subsets::value::schema::snapshot::SemioValueSnapshot;
use framework_schema::ArtifactSchema;

#[cfg(test)]
#[path = "🧪️tests/🪪️document-contract/🦀️.rs"]
mod document_contract;

#[derive(Clone, Debug, PartialEq, ArtifactSchema)]
#[artifact_schema(id = "s.stdio.semio.object")]
pub struct SemioObjectArtifact {
    #[state(artifact)]
    pub schema: String,
    #[state(artifact)]
    pub transform: SemioTransform,
    #[state(artifact)]
    #[child(kind = "s.stdio.semio")]
    pub brep: Option<store::ArtifactChild<SemioBrepSnapshot>>,
    #[state(artifact)]
    #[child(kind = "s.stdio.semio")]
    pub mesh: Option<store::ArtifactChild<SemioMeshSnapshot>>,
    #[state(artifact)]
    #[child(kind = "s.stdio.semio")]
    pub properties: Option<store::ArtifactChild<SemioValueSnapshot>>,
}

impl Default for SemioObjectArtifact {
    fn default() -> Self {
        Self::from_snapshot(SemioObjectSnapshot::default())
    }
}

//#region 🔖️ValueCodec
/// 🔀️ Encodes composite child and link fields through their first-party value contracts.
impl semio_framework_value::ToValue for SemioObjectArtifact {
    fn to_value(&self) -> semio_framework_value::DslValue {
        let mut entries = vec![("schema".to_string(), semio_framework_value::ToValue::to_value(&self.schema)), ("transform".to_string(), semio_framework_value::ToValue::to_value(&self.transform))];
        if let Some(brep) = &self.brep {
            entries.push(("brep".to_string(), semio_framework_value::ToValue::to_value(brep)));
        }
        if let Some(mesh) = &self.mesh {
            entries.push(("mesh".to_string(), semio_framework_value::ToValue::to_value(mesh)));
        }
        if let Some(properties) = &self.properties {
            entries.push(("properties".to_string(), semio_framework_value::ToValue::to_value(properties)));
        }
        semio_framework_value::DslValue::object(entries)
    }
}
impl semio_framework_value::FromValue for SemioObjectArtifact {
    fn from_value(value: semio_framework_value::DslValue) -> Result<Self, semio_framework_value::ValueError> {
        <SemioObjectSnapshot as semio_framework_value::FromValue>::from_value(value).map(Self::from_snapshot)
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
pub fn semio_object_artifact_schema_descriptor() -> semio_framework_schema_registry::ArtifactSchemaDescriptor {
    semio_framework_schema_registry::ArtifactSchemaDescriptor {
        id: "s.stdio.semio.object",
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
