//! ➕️ `insert-track` — authored as its own mutation leaf. It builds its own sparse diff and concrete inverse from its payload and reads of `base`.

use super::*;

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[dsl(keyword = "insert-track")]
pub struct InsertTrack {
    pub index: usize,
    #[dsl(block)]
    pub track: Mp4Track,
}

impl protocol::MutationKind<Mp4Snapshot, Mp4Mutation> for InsertTrack {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "insert", entity: "track", kind: "insert-track", record: "InsertTrack" };
    fn diff(&self, base: &Mp4Snapshot) -> protocol::MutationOutcome<<Mp4Mutation as Mutation<Mp4Snapshot>>::Diff> {
        let Self { index, track } = self;
        protocol::MutationOutcome::new(Mp4Diff { ftyp: None, movie: None, tracks: Some(IndexedDiff { removed: vec![], modified: vec![], added: vec![IndexedAdded { index: *index, item: track.clone() }] }) })
    }
    fn inverse(&self, base: &Mp4Snapshot) -> Result<Vec<Mp4Mutation>, semio_framework_value::ValueError> {
        let Self { index, .. } = self;
        Ok({ vec![Mp4Mutation::RemoveTrack(remove_track::RemoveTrack { index: *index })] })
    }
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native("Insert track", "Spur einfügen")
    }
    fn target(&self) -> Vec<String> {
        Vec::new()
    }
}
//#endregion 🔖️Payload
