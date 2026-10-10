//! 🏷️ `set-id3v2` — authored as its own mutation leaf. It builds its own sparse diff and concrete inverse from its payload and reads of `base`.

use super::*;

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf, semio_framework_value::RetireOwned)]
#[mutation_leaf(contract = ::protocol)]
pub struct SetId3v2 {
    pub id3v2: Option<Id3v2Tag>,
}

impl protocol::MutationKind<Mp3Snapshot, Mp3Mutation> for SetId3v2 {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "set", entity: "id3v2", kind: "set-id3v2", record: "SetId3v2" };

    fn diff(&self, base: &Mp3Snapshot) -> protocol::MutationOutcome<<Mp3Mutation as Mutation<Mp3Snapshot>>::Diff> {
        let Self { id3v2 } = self;
        if let Some(tag)=id3v2{for frame in &tag.frames{if let Err(message)=crate::standards::mpeg1_layer3::subsets::any::schema::snapshot::validate_id3_frame(frame){return protocol::MutationOutcome::refuse(protocol::OutcomeCode::Invariant,message,["id3v2"]);}}}
        protocol::MutationOutcome::new(diff_set_id3v2(id3v2.clone()))
    }
    fn inverse(&self, base: &Mp3Snapshot) -> Result<Vec<Mp3Mutation>, semio_framework_value::ValueError> {
        Ok(vec![Mp3Mutation::SetId3v2(set_id3v2::SetId3v2 { id3v2: base.id3v2.clone() })])
    }
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native("Set ID3v2", "ID3v2-Tag setzen")
    }
    fn target(&self) -> Vec<String> {
        Vec::new()
    }
}
//#endregion 🔖️Payload
