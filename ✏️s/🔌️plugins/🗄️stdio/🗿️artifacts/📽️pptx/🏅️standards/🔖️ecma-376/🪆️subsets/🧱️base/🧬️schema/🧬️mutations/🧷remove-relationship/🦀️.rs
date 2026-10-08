//! 🧷 `remove-relationship` — removes one relationship of an owner part (the owner disappears with its last relationship). It builds its own sparse diff from its
//! payload and reads of `base`; the inverse writes the exact relationship back at the position it held.

use super::*;

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
pub struct RemoveRelationship {
    pub(crate) owner: String,
    pub(crate) id: String,
}

impl protocol::MutationKind<PptxSnapshot, PptxMutation> for RemoveRelationship {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "remove", entity: "relationship", kind: "remove-relationship", record: "RemoveRelationship" };

    fn diff(&self, base: &PptxSnapshot) -> protocol::MutationOutcome<PptxDiff> {
        opc_layer::outcome(opc_layer::with_package(base, |opc| opc_layer::relationship_removal_diff(opc, &self.owner, &self.id)))
    }

    fn inverse(&self, base: &PptxSnapshot) -> Result<Vec<PptxMutation>, semio_framework_value::ValueError> {
        let previous = opc_layer::with_package(base, |opc| opc_layer::relationship_at(opc, &self.owner, &self.id)).map_err(|message| semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, message))?;
        Ok(previous.map(|(at, existing)| PptxMutation::SetRelationship(set_relationship::SetRelationship::of(&self.owner, &existing, Some(at)))).into_iter().collect())
    }

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native("Remove relationship", "Beziehung entfernen")
    }
    fn target(&self) -> Vec<String> {
        vec!["opc".to_string(), "relationships".to_string(), self.owner.clone(), self.id.clone()]
    }
}
//#endregion 🔖️Payload

#[cfg(test)]
#[path = "🧪️tests/🦀️.rs"]
mod tests;
