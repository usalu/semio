//! 🎉️ `set-holidays-dst` — authored as its own mutation leaf. It builds its own sparse diff and concrete inverse from its payload and reads of `base`.

use super::*;

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
pub struct SetHolidaysDst {
    pub value: String,
}

impl protocol::MutationKind<EpwSnapshot, EpwMutation> for SetHolidaysDst {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "set", entity: "holidays-dst", kind: "set-holidays-dst", record: "SetHolidaysDst" };

    fn diff(&self, base: &EpwSnapshot) -> protocol::MutationOutcome<<EpwMutation as Mutation<EpwSnapshot>>::Diff> {
        let Self { value } = self;
        protocol::MutationOutcome::new(EpwDiff { holidays_dst: Some(value.clone()), ..EpwDiff::default() })
    }
    fn inverse(&self, base: &EpwSnapshot) -> Result<Vec<EpwMutation>, semio_framework_value::ValueError> {
        Ok(vec![EpwMutation::SetHolidaysDst(set_holidays_dst::SetHolidaysDst { value: base.holidays_dst.clone() })])
    }
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native("Set holidays DST", "Feiertage und Sommerzeit setzen")
    }
    fn target(&self) -> Vec<String> {
        Vec::new()
    }
}
//#endregion 🔖️Payload
