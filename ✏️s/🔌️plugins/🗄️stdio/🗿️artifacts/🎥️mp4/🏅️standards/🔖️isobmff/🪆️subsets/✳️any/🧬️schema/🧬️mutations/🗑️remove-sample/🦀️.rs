//! 🗑️ `remove-sample` — authored as its own mutation leaf. It builds its own sparse diff and concrete inverse from its payload and reads of `base`.

use super::*;

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[dsl(keyword = "remove-sample")]
#[value(rename_all = "camelCase")]
pub struct RemoveSample {
    pub track_index: usize,
    pub index: usize,
}

impl protocol::MutationKind<Mp4Snapshot, Mp4Mutation> for RemoveSample {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "remove", entity: "sample", kind: "remove-sample", record: "RemoveSample" };
    fn diff(&self, base: &Mp4Snapshot) -> protocol::MutationOutcome<<Mp4Mutation as Mutation<Mp4Snapshot>>::Diff> {
        let Self { track_index, index } = self;
        protocol::MutationOutcome::new({
            let count = base.tracks.get(*track_index).map_or(0, |track| track.samples.len().saturating_sub(1) as u32);
            sample_diff_for(*track_index, IndexedDiff { removed: vec![*index], modified: vec![], added: vec![] }, Some(vec![count]))
        })
    }
    fn inverse(&self, base: &Mp4Snapshot) -> Result<Vec<Mp4Mutation>, semio_framework_value::ValueError> {
        let Self { track_index, index } = self;
        Ok({ base.tracks.get(*track_index).and_then(|track| track.samples.get(*index)).map(|sample| Mp4Mutation::InsertSample(insert_sample::InsertSample { track_index: *track_index, index: *index, sample: sample.clone() })).into_iter().collect() })
    }
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native("Remove sample", "Sample entfernen")
    }
    fn target(&self) -> Vec<String> {
        Vec::new()
    }
}
//#endregion 🔖️Payload
