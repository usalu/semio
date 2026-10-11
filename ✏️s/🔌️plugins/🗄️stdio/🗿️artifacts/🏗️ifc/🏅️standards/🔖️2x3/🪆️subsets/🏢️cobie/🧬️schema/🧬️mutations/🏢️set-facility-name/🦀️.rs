//! 🏢️ `set-facility-name` -- names the `IfcBuilding` a COBie Facility row is; the prior name (or none) is restored.

use super::*;

//#region 🔖️Payload
#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
pub struct SetFacilityName {
    pub building: u64,
    pub name: Option<String>,
}

impl protocol::MutationKind<Ifc2x3Snapshot, Ifc2x3CobieMutation> for SetFacilityName {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "set", entity: "facility-name", kind: "set-facility-name", record: "SetFacilityName" };

    fn diff(&self, base: &Ifc2x3Snapshot) -> protocol::MutationOutcome<Ifc2x3Diff> {
        let Self { building, name } = self;
        match mvd::argument_diff(base, *building, &[BUILDING], NAME_INDEX, mvd::optional(name.clone().map(Part21Value::Str))) {
            Ok(diff) => protocol::MutationOutcome::new(diff),
            Err(message) => rejected(message),
        }
    }

    fn inverse(&self, base: &Ifc2x3Snapshot) -> Result<Vec<Ifc2x3CobieMutation>, semio_framework_value::ValueError> {
        let Self { building, .. } = self;
        if !matches!(mvd::standing(base, *building, &[BUILDING]), mvd::Standing::Present { .. }) {
            return Ok(Vec::new());
        }
        Ok(vec![Ifc2x3CobieMutation::SetFacilityName(SetFacilityName { building: *building, name: mvd::argument(base, *building, NAME_INDEX).and_then(Part21Value::as_str).map(str::to_string) })])
    }

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native("Set facility name", "Bauwerksname setzen")
    }

    fn target(&self) -> Vec<String> {
        Vec::new()
    }
}
//#endregion 🔖️Payload
