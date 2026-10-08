//! 📎 `set-relationship` — writes one relationship of an owner part (`""` is the package root): a new id is inserted at `index` (last by default) and an existing id
//! is changed in place. It builds its own sparse diff from its payload and reads of `base`; the inverse restores the exact previous relationship at its position, or
//! removes the one it created.

use super::*;
use semio_s_artifact_stdio_zip::opc::{OpcRelationship, OpcTargetMode};

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
pub struct SetRelationship {
    pub(crate) owner: String,
    pub(crate) id: String,
    pub(crate) rel_type: String,
    pub(crate) target: String,
    #[value(default)]
    pub(crate) external: bool,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub(crate) index: Option<usize>,
}

impl SetRelationship {
    /// 🔗️ The relationship this payload writes.
    pub(crate) fn relationship(&self) -> OpcRelationship {
        OpcRelationship { id: self.id.clone(), rel_type: self.rel_type.clone(), target: self.target.clone(), target_mode: if self.external { OpcTargetMode::External } else { OpcTargetMode::Internal } }
    }

    /// 🔗️ The payload that writes `relationship` into `owner` at `index`.
    pub(crate) fn of(owner: &str, relationship: &OpcRelationship, index: Option<usize>) -> Self {
        Self { owner: owner.to_string(), id: relationship.id.clone(), rel_type: relationship.rel_type.clone(), target: relationship.target.clone(), external: relationship.target_mode == OpcTargetMode::External, index }
    }
}

impl protocol::MutationKind<PptxSnapshot, PptxMutation> for SetRelationship {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "set", entity: "relationship", kind: "set-relationship", record: "SetRelationship" };

    fn diff(&self, base: &PptxSnapshot) -> protocol::MutationOutcome<PptxDiff> {
        opc_layer::outcome(opc_layer::with_package(base, |opc| opc_layer::relationship_write_diff(opc, &self.owner, &self.relationship(), self.index)))
    }

    fn inverse(&self, base: &PptxSnapshot) -> Result<Vec<PptxMutation>, semio_framework_value::ValueError> {
        let previous = opc_layer::with_package(base, |opc| opc_layer::relationship_at(opc, &self.owner, &self.id)).map_err(|message| semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, message))?;
        Ok(match previous {
            Some((at, existing)) => vec![PptxMutation::SetRelationship(Self::of(&self.owner, &existing, Some(at)))],
            None => vec![PptxMutation::RemoveRelationship(remove_relationship::RemoveRelationship { owner: self.owner.clone(), id: self.id.clone() })],
        })
    }

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native("Set relationship", "Beziehung setzen")
    }
    fn target(&self) -> Vec<String> {
        vec!["opc".to_string(), "relationships".to_string(), self.owner.clone(), self.id.clone()]
    }
}
//#endregion 🔖️Payload

#[cfg(test)]
#[path = "🧪️tests/🦀️.rs"]
mod tests;
