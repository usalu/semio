//! 🔺️ Remodeling artifact — sparse field-delta diff codec and apply/absorb.

use crate::schema::diff::RemodelingDiff;
use crate::schema::RemodelingArtifact;
use crate::RemodelingSnapshot;
use protocol::MutationDiff;

//#region 📖️SemioGrammar
/// 📖️ Normative handcrafted text grammar for this facet (`dialect grammar`).
pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️.grammar.semio");
//#endregion 📖️SemioGrammar

//#region 🔖️Apply
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
            if let Some(job) = &self.job {
                next.job = job.clone();
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
            if let Some(job) = &self.job {
                next.job = job.clone();
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
        take!(job);
        take!(results);
    }
}
//#endregion 🔖️Apply

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests

//#region 🚚️Carrier
/// 🚚️ The carrier this facet's `parse`/`print` speak, named as the schema names the export.
pub type RemodelingDiffText = String;
//#endregion 🚚️Carrier
