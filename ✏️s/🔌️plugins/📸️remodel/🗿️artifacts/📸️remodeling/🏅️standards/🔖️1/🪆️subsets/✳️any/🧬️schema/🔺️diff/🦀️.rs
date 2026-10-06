//! 🧬️ Remodeling diff schema — sparse field delta over the artifact.

use crate::{CalibrationState, GroundControlPoint, MediaStream, ReconstructionParams, ReconstructionResults, RemodelingAssetChild, RemodelingDurableArtifactStore};
use framework_schema::ArtifactSchema;
use semio_framework_value_derive::{FromValue, ToValue};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

//#region 🔖️Diff
/// 🔺️ Sparse field delta for the remodeling artifact; persistent entries apply via MutationDiff.
#[derive(Clone, Debug, Default, PartialEq, ToValue, FromValue, ArtifactSchema)]
#[value(rename_all = "camelCase", default)]
#[artifact_schema(id = "s.remodel.remodeling")]
pub struct RemodelingDiff {
    #[state(artifact)]
    pub artifact: Option<Box<RemodelingArtifact>>,
    #[state(artifact)]
    pub schema: Option<String>,
    #[state(artifact)]
    pub id: Option<String>,
    #[state(artifact)]
    pub streams: Option<RemodelingMediaStreamList>,
    #[state(artifact)]
    pub assets: Option<BTreeMap<String, RemodelingAssetChild>>,
    #[state(artifact)]
    pub durable_artifacts: Option<RemodelingDurableArtifactStore>,
    #[state(artifact)]
    pub calibration: Option<CalibrationState>,
    #[state(artifact)]
    pub params: Option<ReconstructionParams>,
    #[state(artifact)]
    pub gcps: Option<RemodelingGcpList>,
    #[state(artifact)]
    pub results: Option<ReconstructionResults>,
}
//#endregion 🔖️Diff

//#region 🔖️DeltaHelpers
/// 📋 Media-stream list wrapper so optional list diffs stay scalar across formats.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, ToValue, FromValue)]
#[value(rename_all = "camelCase", default)]
#[serde(rename_all = "camelCase", default)]
pub struct RemodelingMediaStreamList {
    pub values: Vec<MediaStream>,
}

/// 📋 GCP list wrapper so optional list diffs stay scalar across formats.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, ToValue, FromValue)]
#[value(rename_all = "camelCase", default)]
#[serde(rename_all = "camelCase", default)]
pub struct RemodelingGcpList {
    pub values: Vec<GroundControlPoint>,
}
//#endregion 🔖️DeltaHelpers

//#region 🔁️Re-exports
/// 🔁️ Entities this module's schema exports and its crate declares elsewhere.
pub use crate::schema::RemodelingArtifact;
//#endregion 🔁️Re-exports


use crate::RemodelingSnapshot;
use protocol::MutationDiff;

impl RemodelingDiff {
    /// 🧬️ Applies sparse document changes to the artifact.
    pub fn apply_to_artifact(&self, artifact: &RemodelingArtifact) -> protocol::MutationApplyResult<RemodelingArtifact> {
        Ok({
            if let Some(replacement) = &self.artifact {
                return Ok((**replacement).clone());
            }
            let mut next = artifact.clone();
            if let Some(schema) = &self.schema {
                next.schema = schema.clone();
            }
            if let Some(id) = &self.id {
                next.id = id.clone();
            }
            if let Some(list) = &self.streams {
                next.streams = list.values.clone();
            }
            if let Some(assets) = &self.assets {
                next.assets = assets.clone();
            }
            if let Some(durable_artifacts) = &self.durable_artifacts {
                next.durable_artifacts = durable_artifacts.clone();
            }
            if let Some(calibration) = &self.calibration {
                next.calibration = calibration.clone();
            }
            if let Some(params) = &self.params {
                next.params = params.clone();
            }
            if let Some(list) = &self.gcps {
                next.gcps = list.values.clone();
            }
            if let Some(results) = &self.results {
                next.results = results.clone();
            }
            next
        })
    }
}

impl MutationDiff<RemodelingSnapshot> for RemodelingDiff {
    fn apply(&self, snapshot: &RemodelingSnapshot) -> protocol::MutationApplyResult<RemodelingSnapshot> {
        Ok({
            if let Some(replacement) = &self.artifact {
                return Ok(replacement.to_snapshot());
            }
            let mut next = snapshot.clone();
            if let Some(schema) = &self.schema {
                next.schema = schema.clone();
            }
            if let Some(id) = &self.id {
                next.id = id.clone();
            }
            if let Some(list) = &self.streams {
                next.streams = list.values.clone();
            }
            if let Some(assets) = &self.assets {
                next.assets = assets.clone();
            }
            if let Some(durable_artifacts) = &self.durable_artifacts {
                next.durable_artifacts = durable_artifacts.clone();
            }
            if let Some(calibration) = &self.calibration {
                next.calibration = calibration.clone();
            }
            if let Some(params) = &self.params {
                next.params = params.clone();
            }
            if let Some(list) = &self.gcps {
                next.gcps = list.values.clone();
            }
            if let Some(results) = &self.results {
                next.results = results.clone();
            }
            next
        })
    }
    fn absorb(&mut self, other: Self) {
        if other.artifact.is_some() {
            *self = other;
            return;
        }
        macro_rules! take {
            ($field:ident) => {
                if other.$field.is_some() {
                    self.$field = other.$field;
                }
            };
        }
        take!(schema);
        take!(id);
        take!(streams);
        take!(assets);
        take!(durable_artifacts);
        take!(calibration);
        take!(params);
        take!(gcps);
        take!(results);
    }
}

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
