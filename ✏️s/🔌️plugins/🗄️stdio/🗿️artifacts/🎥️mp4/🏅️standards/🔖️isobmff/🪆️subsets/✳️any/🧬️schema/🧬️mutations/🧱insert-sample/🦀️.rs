//! 🧱️ `insert-sample` — authored as its own mutation leaf. It builds its own sparse diff and concrete inverse from its payload and reads of `base`.

use super::*;

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[dsl(keyword = "insert-sample")]
#[value(rename_all = "camelCase")]
pub struct InsertSample {
    pub track_index: usize,
    pub index: usize,
    #[dsl(block)]
    pub sample: Mp4Sample,
}

impl protocol::MutationKind<Mp4Snapshot, Mp4Mutation> for InsertSample {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "insert", entity: "sample", kind: "insert-sample", record: "InsertSample" };
    fn diff(&self, base: &Mp4Snapshot) -> protocol::MutationOutcome<<Mp4Mutation as Mutation<Mp4Snapshot>>::Diff> {
        let Self { track_index, index, sample } = self;
        protocol::MutationOutcome::new({
            let count = base.tracks.get(*track_index).map_or(1, |track| track.samples.len() as u32 + 1);
            sample_diff_for(*track_index, IndexedDiff { removed: vec![], modified: vec![], added: vec![IndexedAdded { index: *index, item: sample.clone() }] }, Some(vec![count]))
        })
    }
    fn inverse(&self, base: &Mp4Snapshot) -> Result<Vec<Mp4Mutation>, semio_framework_value::ValueError> {
        let Self { track_index, index, .. } = self;
        Ok({
            {
                vec![Mp4Mutation::RemoveSample(remove_sample::RemoveSample { track_index: *track_index, index: *index })]
            }
        })
    }
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native("Insert sample", "Sample einfügen")
    }
    fn target(&self) -> Vec<String> {
        Vec::new()
    }
}
//#endregion 🔖️Payload
