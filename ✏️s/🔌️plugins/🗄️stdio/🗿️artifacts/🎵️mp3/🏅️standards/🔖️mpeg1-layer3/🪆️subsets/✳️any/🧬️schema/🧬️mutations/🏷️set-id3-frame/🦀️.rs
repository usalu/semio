//! 🏷️ `set-id3-frame` — replaces the ID3v2 frame carrying the given id, or appends it (creating the tag when absent); the exact inverse restores the whole previous tag.

use super::*;

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
pub struct SetId3Frame {
    pub frame: Id3Frame,
}

impl SetId3Frame {
    /// 🏷️ The tag `base` carries with this frame replacing its same-id frame, or appended.
    fn next_tag(&self, base: &Mp3Snapshot) -> Id3v2Tag {
        let mut tag = base.id3v2.clone().unwrap_or(Id3v2Tag { frames: Vec::new() });
        match tag.frames.iter().position(|frame| frame.id == self.frame.id) {
            Some(position) => tag.frames[position] = self.frame.clone(),
            None => tag.frames.push(self.frame.clone()),
        }
        tag
    }
}

impl protocol::MutationKind<Mp3Snapshot, Mp3Mutation> for SetId3Frame {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "set", entity: "id3-frame", kind: "set-id3-frame", record: "SetId3Frame" };

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native("Set ID3 frame", "ID3-Frame setzen")
    }
    fn target(&self) -> Vec<String> {
        vec!["id3v2".into(), self.frame.id.clone()]
    }
}

impl protocol::ColdMutationKind<Mp3Snapshot, Mp3Mutation> for SetId3Frame {
    fn diff(&self, base: &Mp3Snapshot) -> protocol::MutationOutcome<<Mp3Mutation as protocol::ColdMutation<Mp3Snapshot>>::ReferenceDiff> {
        if let Err(message) = crate::standards::mpeg1_layer3::subsets::any::schema::snapshot::validate_id3_frame(&self.frame) {
            return protocol::MutationOutcome::refuse(protocol::OutcomeCode::Invariant, message, ["id3v2"]);
        }
        let next = self.next_tag(base);
        protocol::MutationOutcome::new(if base.id3v2.as_ref() == Some(&next) { Mp3Diff::default() } else { diff_set_id3v2(Some(next)) })
    }
    fn inverse(&self, base: &Mp3Snapshot) -> Result<Vec<Mp3Mutation>, semio_framework_value::ValueError> {
        if crate::standards::mpeg1_layer3::subsets::any::schema::snapshot::validate_id3_frame(&self.frame).is_err() || base.id3v2.as_ref() == Some(&self.next_tag(base)) {
            return Ok(Vec::new());
        }
        Ok(vec![Mp3Mutation::SetId3v2(set_id3v2::SetId3v2 { id3v2: base.id3v2.clone() })])
    }
}
//#endregion 🔖️Payload
