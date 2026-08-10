//! ⚙️ SHome artifact — headless compute (constitutional: engine).

use crate::artifacts::home::{SHomeArtifact, SHomeSnapshot, S_HOME_DOCUMENT_SCHEMA};

//#region 🔖️DocumentHelpers
pub fn empty_shome_snapshot() -> SHomeSnapshot {
    SHomeSnapshot::default()
}
//#endregion 🔖️DocumentHelpers

//#region 🔖️SchemaRegistry
use std::sync::{Mutex, OnceLock};

/// 📎 Registers the home artifact schema descriptor into the process-local registry.
pub fn register_artifact_schema() {
    ::schema::register_artifact_schema_descriptor(crate::artifacts::home::schema::home_artifact_schema_descriptor());
}

/// 🔎 Returns whether `s.space.home` is present in the process-local schema registry.
pub fn artifact_schema_registered() -> bool {
    ::schema::artifact_schema_descriptor_registered("s.space.home")
}
//#endregion 🔖️SchemaRegistry

//#region 🔖️ArtifactEngine
pub struct SHomeEngine {
    artifact: SHomeArtifact,
    snapshot: SHomeSnapshot,
}

impl SHomeEngine {
    pub fn new(snapshot: SHomeSnapshot) -> Self {
        let artifact = SHomeArtifact::from_snapshot(snapshot.clone());
        Self { artifact, snapshot }
    }
}

impl protocol::ArtifactEngine for SHomeEngine {
    type Artifact = SHomeArtifact;
    type Snapshot = SHomeSnapshot;
    type Mutation = crate::artifacts::home::mutations::SHomeMutation;
    type Diff = crate::artifacts::home::diff::SHomeDiff;

    fn artifact(&self) -> &Self::Artifact {
        &self.artifact
    }

    fn snapshot(&self) -> &Self::Snapshot {
        &self.snapshot
    }

    fn apply(&mut self, mutation: &Self::Mutation) -> Result<Self::Diff, protocol::EngineFault> {
        let diff = <Self::Mutation as protocol::Mutation<Self::Snapshot>>::diff(mutation, &self.snapshot);
        self.snapshot = <Self::Diff as protocol::MutationDiff<Self::Snapshot>>::apply(&diff, &self.snapshot);
        self.artifact.set_snapshot(self.snapshot.clone());
        Ok(diff)
    }

    fn inverse(&self, mutation: &Self::Mutation) -> Vec<Self::Mutation> {
        <Self::Mutation as protocol::Mutation<Self::Snapshot>>::inverse(mutation, &self.snapshot)
    }
}
//#endregion 🔖️ArtifactEngine

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use dsl::ArtifactEngine;

    #[test]
    fn empty_snapshot_uses_home_schema() {
        let snapshot = empty_shome_snapshot();
        assert_eq!(snapshot.schema, S_HOME_DOCUMENT_SCHEMA);
    }

    #[test]
    fn engine_apply_updates_catalog_generation() {
        let mut engine = SHomeEngine::new(empty_shome_snapshot());
        let mutation = crate::artifacts::home::mutations::SHomeMutation::SetCatalogGeneration { value: 5 };
        engine.apply(&mutation).expect("apply");
        assert_eq!(engine.snapshot().catalog_generation, 5);
        assert_eq!(engine.artifact().catalog_generation, 5);
    }
}
//#endregion 🧪️Tests

//#region 🔖️IoFacet
pub fn register_io() {
    crate::artifacts::home::io::register();
}
//#endregion 🔖️IoFacet
