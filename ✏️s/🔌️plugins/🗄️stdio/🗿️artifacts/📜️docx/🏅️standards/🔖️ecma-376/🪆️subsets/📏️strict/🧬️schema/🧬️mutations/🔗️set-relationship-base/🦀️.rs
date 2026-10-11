//! ⚙️ `set-relationship-base` — authored as its own mutation leaf. It builds its own sparse diff and concrete inverse from its payload and reads of `base`.

use super::*;

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
pub struct SetRelationshipBase {
    pub(crate) base: String,
}

impl protocol::MutationKind<DocxSnapshot, DocxStrictMutation> for SetRelationshipBase {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "set", entity: "relationship-base", kind: "set-relationship-base", record: "SetRelationshipBase" };

    fn diff(&self, base: &DocxSnapshot) -> protocol::MutationOutcome<DocxDiff> {
        protocol::MutationOutcome::new(diff_retarget_relationship_base(base, RELATIONSHIP_NAMESPACES, &self.base))
    }

    fn inverse(&self, base: &DocxSnapshot) -> Result<Vec<DocxStrictMutation>, semio_framework_value::ValueError> {
        Ok(relationship_base_inverse(base))
    }

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native("Set relationship base", "Beziehungsbasis setzen")
    }
    fn target(&self) -> Vec<String> {
        Vec::new()
    }
}
//#endregion 🔖️Payload
