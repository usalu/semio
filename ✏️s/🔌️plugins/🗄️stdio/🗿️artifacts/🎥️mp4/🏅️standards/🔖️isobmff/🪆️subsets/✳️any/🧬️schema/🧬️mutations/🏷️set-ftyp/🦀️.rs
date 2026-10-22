//! 🏷️ `set-ftyp` — authored as its own mutation leaf. It builds its own sparse diff and concrete inverse from its payload and reads of `base`.

use super::*;

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[dsl(keyword = "set-ftyp")]
pub struct SetFtyp {
    #[dsl(block)]
    pub ftyp: Mp4Ftyp,
}

impl protocol::MutationKind<Mp4Snapshot, Mp4Mutation> for SetFtyp {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "set", entity: "ftyp", kind: "set-ftyp", record: "SetFtyp" };

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native("Set ftyp", "ftyp-Box setzen")
    }
    fn target(&self) -> Vec<String> {
        Vec::new()
    }
}

impl protocol::ColdMutationKind<Mp4Snapshot, Mp4Mutation> for SetFtyp {
    fn diff(&self, _base: &Mp4Snapshot) -> protocol::MutationOutcome<<Mp4Mutation as protocol::ColdMutation<Mp4Snapshot>>::ReferenceDiff> {
        let Self { ftyp } = self;
        protocol::MutationOutcome::new(Mp4Diff { ftyp: Some(ftyp.clone()), movie: None, tracks: None })
    }
    fn inverse(&self, base: &Mp4Snapshot) -> Result<Vec<Mp4Mutation>, semio_framework_value::ValueError> {
        Ok(vec![Mp4Mutation::SetFtyp(SetFtyp { ftyp: base.ftyp.clone() })])
    }
}
//#endregion 🔖️Payload
