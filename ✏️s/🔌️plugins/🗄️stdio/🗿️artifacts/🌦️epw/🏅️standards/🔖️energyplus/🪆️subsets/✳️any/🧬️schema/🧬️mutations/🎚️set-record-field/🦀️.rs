//! 🎚️ `set-record-field` — authored as its own mutation leaf. It builds its own sparse diff and concrete inverse from its payload and reads of `base`.

use super::*;

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
pub struct SetRecordField {
    pub record_index: usize,
    pub field_index: usize,
    pub value: String,
}

impl protocol::MutationKind<EpwSnapshot, EpwMutation> for SetRecordField {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "set", entity: "record-field", kind: "set-record-field", record: "SetRecordField" };

    fn diff(&self, base: &EpwSnapshot) -> protocol::MutationOutcome<<EpwMutation as Mutation<EpwSnapshot>>::Diff> {
        let Self { record_index, field_index, value } = self;
        protocol::MutationOutcome::new({
            let mut fdiff = EpwRecordDiff::default();
            fdiff.set_at(*field_index, Some(value.clone()));
            EpwDiff { records: Some(EpwRecordsDiff { removed: Vec::new(), modified: vec![EpwRecordModified { index: *record_index, diff: fdiff }], added: Vec::new() }), ..EpwDiff::default() }
        })
    }
    fn inverse(&self, base: &EpwSnapshot) -> Result<Vec<EpwMutation>, semio_framework_value::ValueError> {
        let Self { record_index, field_index, .. } = self;
        Ok({
            match base.records.get(*record_index).and_then(|r| r.field_at(*field_index)) {
                Some(prior) => vec![EpwMutation::SetRecordField(set_record_field::SetRecordField { record_index: *record_index, field_index: *field_index, value: prior.to_string() })],
                None => Vec::new(),
            }
        })
    }
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native("Set record field", "Datensatzfeld setzen")
    }
    fn target(&self) -> Vec<String> {
        Vec::new()
    }
}
//#endregion 🔖️Payload
