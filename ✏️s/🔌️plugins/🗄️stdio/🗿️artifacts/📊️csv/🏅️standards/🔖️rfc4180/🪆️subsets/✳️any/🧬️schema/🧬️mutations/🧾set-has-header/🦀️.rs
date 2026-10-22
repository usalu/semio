//! 🧾️ `set-has-header` — its own mutation leaf. It builds its own sparse diff and concrete inverse from its payload and reads of `base`.

use super::*;

//#region 🔖️Payload
#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
pub struct SetHasHeader {
    pub has_header: bool,
}

impl protocol::MutationKind<CsvSnapshot, CsvMutation> for SetHasHeader {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "set", entity: "has-header", kind: "set-has-header", record: "SetHasHeader" };

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native("Set has header", "Kopfzeile vorhanden setzen")
    }
    fn target(&self) -> Vec<String> {
        Vec::new()
    }
}

impl protocol::ColdMutationKind<CsvSnapshot, CsvMutation> for SetHasHeader {
    fn diff(&self, _base: &CsvSnapshot) -> protocol::MutationOutcome<<CsvMutation as protocol::ColdMutation<CsvSnapshot>>::ReferenceDiff> {
        let Self { has_header } = self;
        protocol::MutationOutcome::new(CsvDiff { has_header: Some(*has_header), records: None })
    }
    fn inverse(&self, base: &CsvSnapshot) -> Result<Vec<CsvMutation>, semio_framework_value::ValueError> {
        Ok(vec![CsvMutation::SetHasHeader(SetHasHeader { has_header: base.has_header })])
    }
}
//#endregion 🔖️Payload
