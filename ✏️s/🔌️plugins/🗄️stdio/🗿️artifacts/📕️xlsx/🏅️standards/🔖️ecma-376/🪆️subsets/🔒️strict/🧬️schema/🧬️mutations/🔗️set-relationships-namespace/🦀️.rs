//! ⚙️ `set-relationships-namespace` — authored as its own mutation leaf. It builds its own sparse diff and concrete inverse from its payload and reads of `base`.

use super::*;

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
pub struct SetRelationshipsNamespace {
    pub(crate) namespace: String,
}

impl protocol::MutationKind<XlsxSnapshot, XlsxStrictMutation> for SetRelationshipsNamespace {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "set", entity: "relationships-namespace", kind: "set-relationships-namespace", record: "SetRelationshipsNamespace" };

    fn diff(&self, base: &XlsxSnapshot) -> protocol::MutationOutcome<XlsxDiff> {
        protocol::MutationOutcome::new(diff_retarget_namespace(base, RELATIONSHIP_NAMESPACES, &self.namespace))
    }

    fn inverse(&self, base: &XlsxSnapshot) -> Result<Vec<XlsxStrictMutation>, semio_framework_value::ValueError> {
        Ok(relationships_namespace_inverse(base))
    }

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native("Set relationships namespace", "Beziehungsnamensraum setzen")
    }
    fn target(&self) -> Vec<String> {
        Vec::new()
    }
}
//#endregion 🔖️Payload
