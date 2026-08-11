//! 🧬️ ObjArtifact schema — full artifact state, mirrors `ObjSnapshot` field-for-field.

use crate::artifacts::obj::ObjSnapshot;
use schema::ArtifactSchema;
use serde::{Deserialize, Serialize};

//#region 🔖️Artifact
/// 🧬️ Full `stdio.obj` artifact state.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase")]
#[artifact_schema(id = "s.stdio.obj")]
pub struct ObjArtifact {
    #[state(persistent)]
    pub schema: String,
    #[state(persistent)]
    #[serde(default)]
    pub vertices: Vec<crate::artifacts::obj::schema::snapshot::ObjVertex>,
    #[state(persistent)]
    #[serde(default)]
    pub texcoords: Vec<crate::artifacts::obj::schema::snapshot::ObjTexCoord>,
    #[state(persistent)]
    #[serde(default)]
    pub normals: Vec<crate::artifacts::obj::schema::snapshot::ObjNormal>,
    #[state(persistent)]
    #[serde(default)]
    pub faces: Vec<crate::artifacts::obj::schema::snapshot::ObjFace>,
    #[state(persistent)]
    #[serde(default)]
    pub groups: Vec<crate::artifacts::obj::schema::snapshot::ObjGroup>,
    #[state(persistent)]
    #[serde(default)]
    pub objects: Vec<crate::artifacts::obj::schema::snapshot::ObjObject>,
    #[state(persistent)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mtllib: Option<String>,
    #[state(persistent)]
    #[serde(default)]
    pub usemtl: Vec<crate::artifacts::obj::schema::snapshot::ObjUsemtlRange>,
    #[state(persistent)]
    #[serde(default)]
    pub smoothing_groups: Vec<crate::artifacts::obj::schema::snapshot::ObjSmoothingRange>,
    #[state(persistent)]
    #[serde(default)]
    pub unknown_statements: Vec<crate::artifacts::obj::schema::snapshot::ObjUnknownStatement>,
}
//#endregion 🔖️Artifact

//#region 🔖️Conversions
impl Default for ObjArtifact {
    fn default() -> Self {
        Self::from_snapshot(ObjSnapshot::default())
    }
}

impl ObjArtifact {
    /// 📸️ Persisted subset.
    pub fn to_snapshot(&self) -> ObjSnapshot {
        ObjSnapshot {
            schema: self.schema.clone(),
            vertices: self.vertices.clone(),
            texcoords: self.texcoords.clone(),
            normals: self.normals.clone(),
            faces: self.faces.clone(),
            groups: self.groups.clone(),
            objects: self.objects.clone(),
            mtllib: self.mtllib.clone(),
            usemtl: self.usemtl.clone(),
            smoothing_groups: self.smoothing_groups.clone(),
            unknown_statements: self.unknown_statements.clone(),
        }
    }

    /// 🧬️ Builds a full artifact from a snapshot.
    pub fn from_snapshot(snapshot: ObjSnapshot) -> Self {
        Self {
            schema: snapshot.schema,
            vertices: snapshot.vertices,
            texcoords: snapshot.texcoords,
            normals: snapshot.normals,
            faces: snapshot.faces,
            groups: snapshot.groups,
            objects: snapshot.objects,
            mtllib: snapshot.mtllib,
            usemtl: snapshot.usemtl,
            smoothing_groups: snapshot.smoothing_groups,
            unknown_statements: snapshot.unknown_statements,
        }
    }

    /// 🔄 Writes persistent fields from a snapshot into this artifact.
    pub fn set_snapshot(&mut self, snapshot: ObjSnapshot) {
        self.schema = snapshot.schema;
        self.vertices = snapshot.vertices;
        self.texcoords = snapshot.texcoords;
        self.normals = snapshot.normals;
        self.faces = snapshot.faces;
        self.groups = snapshot.groups;
        self.objects = snapshot.objects;
        self.mtllib = snapshot.mtllib;
        self.usemtl = snapshot.usemtl;
        self.smoothing_groups = snapshot.smoothing_groups;
        self.unknown_statements = snapshot.unknown_statements;
    }
}
//#endregion 🔖️Conversions

//#region 🔖️Descriptor
/// 🧬️ Descriptor for `s.stdio.obj`.
pub fn obj_artifact_schema_descriptor() -> schema::ArtifactSchemaDescriptor {
    schema::ArtifactSchemaDescriptor {
        id: "s.stdio.obj",
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
//#endregion 🔖️Descriptor
