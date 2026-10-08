//! ⭐️ `set-sample-sync` — authored as its own mutation leaf. It builds its own sparse diff and concrete inverse from its payload and reads of `base`.

use super::*;

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[dsl(keyword = "set-sample-sync")]
#[value(rename_all = "camelCase")]
pub struct SetSampleSync {
    pub track_index: usize,
    pub index: usize,
    pub sync: bool,
}

impl protocol::MutationKind<Mp4Snapshot, Mp4Mutation> for SetSampleSync {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "set", entity: "sample-sync", kind: "set-sample-sync", record: "SetSampleSync" };
    fn diff(&self, base: &Mp4Snapshot) -> protocol::MutationOutcome<<Mp4Mutation as Mutation<Mp4Snapshot>>::Diff> {
        let Self { track_index, index, sync } = self;
        protocol::MutationOutcome::new({
            sample_diff_for(*track_index, IndexedDiff { removed: vec![], modified: vec![IndexedModified { index: *index, diff: Mp4SampleDiff { data: None, duration: None, cts_offset: None, sync: Some(*sync) } }], added: vec![] }, None)
        })
    }
    fn inverse(&self, base: &Mp4Snapshot) -> Result<Vec<Mp4Mutation>, semio_framework_value::ValueError> {
        let Self { track_index, index, .. } = self;
        Ok({
            match base.tracks.get(*track_index).and_then(|t| t.samples.get(*index)) {
                Some(sample) => vec![Mp4Mutation::SetSampleSync(set_sample_sync::SetSampleSync { track_index: *track_index, index: *index, sync: sample.sync })],
                None => Vec::new(),
            }
        })
    }
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native("Set sample sync", "Sync-Kennung des Samples setzen")
    }
    fn target(&self) -> Vec<String> {
        Vec::new()
    }
}
//#endregion 🔖️Payload
