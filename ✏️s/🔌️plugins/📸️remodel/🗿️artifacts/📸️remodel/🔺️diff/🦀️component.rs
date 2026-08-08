//! 🔺️ Remodel artifact — sparse field-delta diff codec and apply/absorb.

use crate::artifacts::remodel::diff::schema::{RemodelDiff, RemodelGcpList, RemodelMediaStreamList};
use crate::artifacts::remodel::schema::RemodelArtifact;
use crate::artifacts::remodel::RemodelSnapshot;
use protocol::MutationDiff;

//#region 📖️SemioGrammar
/// 📖️ Normative handcrafted text grammar for this facet (`dialect grammar`).
pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️component.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️component.grammar.semio");
//#endregion 📖️SemioGrammar

pub use super::schema::*;

//#region 🔖️Apply
impl RemodelDiff {
    /// 🧬️ Applies every sparse entry (all state classes) onto a full artifact.
    pub fn apply_to_artifact(&self, artifact: &RemodelArtifact) -> RemodelArtifact {
        if let Some(replacement) = &self.artifact {
            return (**replacement).clone();
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
        if let Some(value) = &self.selection {
            next.selection = value.clone();
        }
        if let Some(value) = &self.active_utility_id {
            next.active_utility_id = value.clone();
        }
        if let Some(value) = &self.report_table {
            next.report_table = value.clone();
        }
        if let Some(value) = &self.frame_cursor {
            next.frame_cursor = value.clone();
        }
        if let Some(value) = &self.camera {
            next.camera = value.clone();
        }
        if let Some(value) = &self.layers {
            next.layers = value.clone();
        }
        if let Some(value) = &self.locale {
            next.locale = value.clone();
        }
        next
    }
}

impl MutationDiff<RemodelSnapshot> for RemodelDiff {
    fn apply(&self, snapshot: &RemodelSnapshot) -> RemodelSnapshot {
        if let Some(replacement) = &self.artifact {
            return replacement.to_snapshot();
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
        take!(calibration);
        take!(params);
        take!(gcps);
        take!(job);
        take!(results);
        take!(selection);
        take!(active_utility_id);
        take!(report_table);
        take!(frame_cursor);
        take!(camera);
        take!(layers);
        take!(locale);
    }
}
//#endregion 🔖️Apply

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::artifacts::remodel::default_remodel_scene;

    #[test]
    fn empty_diff_is_identity_and_absorb_is_fieldwise_last_writer() {
        let scene = default_remodel_scene();
        assert_eq!(RemodelDiff::default().apply(&scene), scene);

        let mut diff = RemodelDiff::default();
        diff.absorb(RemodelDiff {
            gcps: Some(RemodelGcpList { values: Vec::new() }),
            ..Default::default()
        });
        assert!(diff.gcps.is_some());
        diff.absorb(RemodelDiff::default());
        assert!(diff.gcps.is_some(), "absorbing empty never clobbers a real entry");
    }
}
//#endregion 🧪️Tests
