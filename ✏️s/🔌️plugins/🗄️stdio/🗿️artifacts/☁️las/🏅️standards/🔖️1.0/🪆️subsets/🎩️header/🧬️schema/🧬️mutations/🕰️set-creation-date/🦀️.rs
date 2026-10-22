//! 🕰️ `set-creation-date` — its own mutation leaf. It builds its own sparse diff and concrete inverse from its payload and reads of `base`.
//!
//! 🕰️ Sets the file creation day-of-year / year.
use super::*;

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
pub struct SetCreationDate {
    pub day_of_year: u16,
    pub year: u16,
}

impl protocol::MutationKind<LasSnapshot, LasMutation> for SetCreationDate {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "set", entity: "creation-date", kind: "set-creation-date", record: "SetCreationDate" };

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native("Set creation date", "Erstellungsdatum setzen")
    }
    fn target(&self) -> Vec<String> {
        Vec::new()
    }
}

impl protocol::ColdMutationKind<LasSnapshot, LasMutation> for SetCreationDate {
    fn diff(&self, _base: &LasSnapshot) -> protocol::MutationOutcome<<LasMutation as protocol::ColdMutation<LasSnapshot>>::ReferenceDiff> {
        let Self { day_of_year, year } = self;
        protocol::MutationOutcome::new(diff::diff_set_creation_date(*day_of_year, *year))
    }
    fn inverse(&self, base: &LasSnapshot) -> Result<Vec<LasMutation>, semio_framework_value::ValueError> {
        Ok(vec![LasMutation::SetCreationDate(SetCreationDate { day_of_year: base.header.creation_day_of_year, year: base.header.creation_year })])
    }
}
//#endregion 🔖️Payload
