//! 🧬️ S Home artifact schema — every field of the artifact with its state class.

use ::semio_framework_schema::ArtifactSchema;

//#region 🔖️Artifact
/// 🧬️ Full S Home launcher artifact state across the artifact and config lanes.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, ArtifactSchema)]
#[value(rename_all = "camelCase")]
#[artifact_schema(id = "s.space.home")]
pub struct SHomeArtifact {
    #[state(artifact)]
    pub schema: String,
    #[state(artifact)]
    pub catalog_generation: u64,
}
//#endregion 🔖️Artifact

//#region 🔖️Conversions
impl Default for SHomeArtifact {
    fn default() -> Self {
        Self { schema: crate::S_HOME_DOCUMENT_SCHEMA.into(), catalog_generation: 0 }
    }
}

impl SHomeArtifact {
    /// 📸️ Persisted subset.
    pub fn to_snapshot(&self) -> crate::SHomeSnapshot {
        crate::SHomeSnapshot { schema: self.schema.clone(), catalog_generation: self.catalog_generation }
    }

    /// 🧬️ Builds the document artifact from its snapshot.
    pub fn from_snapshot(snapshot: crate::SHomeSnapshot) -> Self {
        Self { schema: snapshot.schema, catalog_generation: snapshot.catalog_generation }
    }

    /// 🔄 Writes persistent fields from a snapshot into this artifact.
    pub fn set_snapshot(&mut self, snapshot: crate::SHomeSnapshot) {
        self.schema = snapshot.schema;
        self.catalog_generation = snapshot.catalog_generation;
    }
}
//#endregion 🔖️Conversions

//#region 🔖️Descriptor
/// 🧬️ Descriptor for `s.space.home` — twenty handcrafted schema leaves.
pub fn home_artifact_schema_descriptor() -> ::semio_framework_schema_registry::ArtifactSchemaDescriptor {
    ::semio_framework_schema_registry::ArtifactSchemaDescriptor {
        id: "s.space.home",
        artifact: ::semio_framework_schema_registry::FacetLeaves {
            rust: include_str!("🦀️.rs"),
            typescript: include_str!("🟦️.ts"),
            graphql: include_str!("🔗️.graphql"),
            json_schema: include_str!("🔣️.json"),
            proto: include_str!("🛰️.proto"),
        },
        snapshot: ::semio_framework_schema_registry::FacetLeaves {
            rust: include_str!("📸️snapshot/🦀️.rs"),
            typescript: include_str!("📸️snapshot/🟦️.ts"),
            graphql: include_str!("📸️snapshot/🔗️.graphql"),
            json_schema: include_str!("📸️snapshot/🔣️.json"),
            proto: include_str!("📸️snapshot/🛰️.proto"),
        },
        diff: ::semio_framework_schema_registry::FacetLeaves {
            rust: include_str!("🔺️diff/🦀️.rs"),
            typescript: include_str!("🔺️diff/🟦️.ts"),
            graphql: include_str!("🔺️diff/🔗️.graphql"),
            json_schema: include_str!("🔺️diff/🔣️.json"),
            proto: include_str!("🔺️diff/🛰️.proto"),
        },
        mutations: ::semio_framework_schema_registry::FacetLeaves {
            rust: include_str!("🧬️mutations/🦀️.rs"),
            typescript: include_str!("🧬️mutations/🟦️.ts"),
            graphql: include_str!("🧬️mutations/🔗️.graphql"),
            json_schema: include_str!("🧬️mutations/🔣️.json"),
            proto: include_str!("🧬️mutations/🛰️.proto"),
        },
    }
}
//#endregion 🔖️Descriptor
//#region 🏗️DerivedConstruction

//#endregion 🏗️DerivedConstruction

//#region 🧐️DerivedAnalysis

//#endregion 🧐️DerivedAnalysis

//#region 🧬️DerivedArtifactFacets

//#endregion 🧬️DerivedArtifactFacets

//#region 🔖️DocumentHelpers
/// 🌱️ Relocated from `⚙️engine` (ticket 26/08/12/ENGINELESS-ARTIFACTS-AND-APP-STATE-MACHINES, rule 3:
/// pure helpers over document types live in `🧬️schema/`).
pub fn empty_shome_snapshot() -> crate::SHomeSnapshot {
    crate::SHomeSnapshot::default()
}

/// 🔎 Returns whether `s.space.home` is present in the process-local schema registry. Relocated from
/// `⚙️engine` alongside `empty_shome_snapshot` (same rule).
pub fn artifact_schema_registered() -> bool {
    ::semio_framework_schema_registry::artifact_schema_descriptor_registered("s.space.home")
}
//#endregion 🔖️DocumentHelpers

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
