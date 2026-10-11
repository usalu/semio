//! ✏️ `set-field` — its own mutation leaf. It builds its own sparse diff and concrete inverse from its payload and reads of `base`.

use super::*;

//#region 🔖️Payload
#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
pub struct SetField {
    pub record_index: usize,
    pub field_index: usize,
    pub value: String,
    pub quoted: bool,
}

impl protocol::MutationKind<CsvSnapshot, CsvMutation> for SetField {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "set", entity: "field", kind: "set-field", record: "SetField" };

    fn diff(&self, base: &CsvSnapshot) -> protocol::MutationOutcome<<CsvMutation as Mutation<CsvSnapshot>>::Diff> {
        let Self { record_index, field_index, value, quoted } = self;
        protocol::MutationOutcome::new({
            let mut fields = vec![None; field_index + 1];
            fields[*field_index] = Some(CsvFieldDiff { value: Some(value.clone()), quoted: Some(*quoted) });
            CsvDiff { has_header: None, records: Some(CsvRecordsDiff { removed: Vec::new(), modified: vec![CsvRecordModified { index: *record_index, diff: CsvRecordDiff { fields: Some(fields) } }], added: Vec::new() }) }
        })
    }
    fn inverse(&self, base: &CsvSnapshot) -> Result<Vec<CsvMutation>, semio_framework_value::ValueError> {
        let Self { record_index, field_index, .. } = self;
        Ok({
            match base.records.get(*record_index).and_then(|r| r.fields.get(*field_index)) {
                Some(field) => vec![CsvMutation::SetField(set_field::SetField { record_index: *record_index, field_index: *field_index, value: field.value.clone(), quoted: field.quoted })],
                None => Vec::new(),
            }
        })
    }
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native("Set field", "Feld setzen")
    }
    fn target(&self) -> Vec<String> {
        Vec::new()
    }
}
//#endregion 🔖️Payload
