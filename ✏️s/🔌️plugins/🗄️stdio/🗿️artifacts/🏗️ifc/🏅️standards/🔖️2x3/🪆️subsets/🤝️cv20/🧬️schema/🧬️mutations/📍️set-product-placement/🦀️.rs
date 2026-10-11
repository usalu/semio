//! 📍️ `set-product-placement` -- points a geometry-bearing product's `ObjectPlacement` at an `IfcLocalPlacement`; the prior reference (or none) is restored.

use super::*;

//#region 🔖️Payload
#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
pub struct SetProductPlacement {
    pub product: u64,
    pub placement: Option<u64>,
}

impl protocol::MutationKind<Ifc2x3Snapshot, Ifc2x3Cv20Mutation> for SetProductPlacement {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "set", entity: "product-placement", kind: "set-product-placement", record: "SetProductPlacement" };

    fn diff(&self, base: &Ifc2x3Snapshot) -> protocol::MutationOutcome<Ifc2x3Diff> {
        let Self { product, placement } = self;
        if let Some(id) = placement {
            let resolved = mvd::instance_type(base, *id).unwrap_or("");
            if !resolved.eq_ignore_ascii_case("IFCLOCALPLACEMENT") {
                return rejected(format!("#{id} is {resolved:?}, not an IFCLOCALPLACEMENT -- Coordination View 2.0 places products through IfcLocalPlacement"));
            }
        }
        match mvd::argument_diff(base, *product, GEOMETRY_BEARING_PRODUCT_TYPES, PRODUCT_PLACEMENT_INDEX, mvd::optional(placement.map(Part21Value::Ref))) {
            Ok(diff) => protocol::MutationOutcome::new(diff),
            Err(message) => rejected(message),
        }
    }

    fn inverse(&self, base: &Ifc2x3Snapshot) -> Result<Vec<Ifc2x3Cv20Mutation>, semio_framework_value::ValueError> {
        let Self { product, .. } = self;
        if !matches!(mvd::standing(base, *product, GEOMETRY_BEARING_PRODUCT_TYPES), mvd::Standing::Present { .. }) {
            return Ok(Vec::new());
        }
        Ok(vec![Ifc2x3Cv20Mutation::SetProductPlacement(SetProductPlacement { product: *product, placement: mvd::reference_argument(base, *product, PRODUCT_PLACEMENT_INDEX) })])
    }

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native("Set product placement", "Produktplatzierung setzen")
    }

    fn target(&self) -> Vec<String> {
        Vec::new()
    }
}
//#endregion 🔖️Payload
