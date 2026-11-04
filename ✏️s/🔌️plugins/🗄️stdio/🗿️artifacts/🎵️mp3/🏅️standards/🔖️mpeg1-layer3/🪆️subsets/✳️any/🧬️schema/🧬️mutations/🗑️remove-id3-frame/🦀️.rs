//! 🗑️ `remove-id3-frame` — removes the first ID3v2 frame carrying the given id; a missing tag or frame is refused; the exact inverse restores the whole previous tag.

use super::*;

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
pub struct RemoveId3Frame {
    pub id: String,
}

impl RemoveId3Frame {
    /// 🗑️ The tag `base` carries without its first frame of this id, if there is one.
    fn next_tag(&self, base: &Mp3Snapshot) -> Option<Id3v2Tag> {
        let mut tag = base.id3v2.clone()?;
        let position = tag.frames.iter().position(|frame| frame.id == self.id)?;
        tag.frames.remove(position);
        Some(tag)
    }
}

impl protocol::MutationKind<Mp3Snapshot, Mp3Mutation> for RemoveId3Frame {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "remove", entity: "id3-frame", kind: "remove-id3-frame", record: "RemoveId3Frame" };

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native("Remove ID3 frame", "ID3-Frame entfernen")
    }
    fn target(&self) -> Vec<String> {
        vec!["id3v2".into(), self.id.clone()]
    }
}

impl protocol::ColdMutationKind<Mp3Snapshot, Mp3Mutation> for RemoveId3Frame {
    fn diff(&self, base: &Mp3Snapshot) -> protocol::MutationOutcome<<Mp3Mutation as protocol::ColdMutation<Mp3Snapshot>>::ReferenceDiff> {
        match self.next_tag(base) {
            Some(tag) => protocol::MutationOutcome::new(diff_set_id3v2(Some(tag))),
            None => protocol::MutationOutcome::error("mutation.target-mismatch", format!("the document carries no ID3 frame {}", self.id), ["id3v2".into(), self.id.clone()]),
        }
    }
    fn inverse(&self, base: &Mp3Snapshot) -> Result<Vec<Mp3Mutation>, semio_framework_value::ValueError> {
        Ok(match self.next_tag(base) {
            Some(_) => vec![Mp3Mutation::SetId3v2(set_id3v2::SetId3v2 { id3v2: base.id3v2.clone() })],
            None => Vec::new(),
        })
    }
}
//#endregion 🔖️Payload
