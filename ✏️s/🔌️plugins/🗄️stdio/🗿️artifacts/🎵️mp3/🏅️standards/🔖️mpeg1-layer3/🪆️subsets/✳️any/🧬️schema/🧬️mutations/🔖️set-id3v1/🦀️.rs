//! 🔖️ `set-id3v1` — authored as its own mutation leaf. It builds its own sparse diff and concrete inverse from its payload and reads of `base`.

use super::*;

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf, semio_framework_value::RetireOwned)]
#[mutation_leaf(contract = ::protocol)]
pub struct SetId3v1 {
    pub id3v1: Option<Id3v1Tag>,
}

impl protocol::MutationKind<Mp3Snapshot, Mp3Mutation> for SetId3v1 {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "set", entity: "id3v1", kind: "set-id3v1", record: "SetId3v1" };

    fn diff(&self, base: &Mp3Snapshot) -> protocol::MutationOutcome<<Mp3Mutation as Mutation<Mp3Snapshot>>::Diff> {
        let Self { id3v1 } = self;
        if let Some(tag)=id3v1{if let Err(message)=crate::standards::mpeg1_layer3::subsets::any::schema::snapshot::validate_id3v1_tag(tag){return protocol::MutationOutcome::refuse(protocol::OutcomeCode::Invariant,message,["id3v1"]);}}
        protocol::MutationOutcome::new(diff_set_id3v1(id3v1.clone()))
    }
    fn inverse(&self, base: &Mp3Snapshot) -> Result<Vec<Mp3Mutation>, semio_framework_value::ValueError> {
        Ok(vec![Mp3Mutation::SetId3v1(set_id3v1::SetId3v1 { id3v1: base.id3v1.clone() })])
    }
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native("Set ID3v1", "ID3v1-Tag setzen")
    }
    fn target(&self) -> Vec<String> {
        Vec::new()
    }
}
//#endregion 🔖️Payload
