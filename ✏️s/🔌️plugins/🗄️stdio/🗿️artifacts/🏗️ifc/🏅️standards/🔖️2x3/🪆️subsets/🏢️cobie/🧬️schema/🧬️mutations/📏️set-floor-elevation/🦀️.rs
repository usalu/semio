//! 📏️ `set-floor-elevation` -- sets the `IfcBuildingStorey.Elevation` a COBie Floor row carries; the prior elevation (or none) is restored.

use super::*;

//#region 🔖️Payload
#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
pub struct SetFloorElevation {
    pub storey: u64,
    pub elevation: Option<f64>,
}

impl protocol::MutationKind<Ifc2x3Snapshot, Ifc2x3CobieMutation> for SetFloorElevation {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "set", entity: "floor-elevation", kind: "set-floor-elevation", record: "SetFloorElevation" };

    fn diff(&self, base: &Ifc2x3Snapshot) -> protocol::MutationOutcome<Ifc2x3Diff> {
        let Self { storey, elevation } = self;
        match mvd::argument_diff(base, *storey, &[STOREY], STOREY_ELEVATION_INDEX, mvd::optional(elevation.map(|value| Part21Value::Real(value.into())))) {
            Ok(diff) => protocol::MutationOutcome::new(diff),
            Err(message) => rejected(message),
        }
    }

    fn inverse(&self, base: &Ifc2x3Snapshot) -> Result<Vec<Ifc2x3CobieMutation>, semio_framework_value::ValueError> {
        let Self { storey, .. } = self;
        if !matches!(mvd::standing(base, *storey, &[STOREY]), mvd::Standing::Present { .. }) {
            return Ok(Vec::new());
        }
        Ok(vec![Ifc2x3CobieMutation::SetFloorElevation(SetFloorElevation { storey: *storey, elevation: mvd::argument(base, *storey, STOREY_ELEVATION_INDEX).and_then(Part21Value::as_real) })])
    }

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native("Set floor elevation", "Geschosshöhenkote setzen")
    }

    fn target(&self) -> Vec<String> {
        Vec::new()
    }
}
//#endregion 🔖️Payload
