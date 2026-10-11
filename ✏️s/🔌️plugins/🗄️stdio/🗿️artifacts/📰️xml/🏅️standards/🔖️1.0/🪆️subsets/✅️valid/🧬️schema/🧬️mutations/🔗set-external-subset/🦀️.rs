//! 🔗️ `set-external-subset` — authored as its own mutation leaf. It builds its own sparse diff and concrete inverse from its payload and reads of `base`.

use super::*;

//#region 🔖️Payload
#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
pub struct SetExternalSubset {
    pub(crate) external_id: Option<XmlExternalId>,
}

impl protocol::MutationKind<XmlSnapshot, XmlValidMutation> for SetExternalSubset {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "set", entity: "external-subset", kind: "set-external-subset", record: "SetExternalSubset" };

    fn diff(&self, base: &XmlSnapshot) -> protocol::MutationOutcome<<XmlValidMutation as Mutation<XmlSnapshot>>::Diff> {
        let Self { external_id } = self;
        match base.doc.doctype.as_ref() {
            None => rejected("set-external-subset: the document has no DOCTYPE to attach an external subset reference to".to_string()),
            Some(doctype) => protocol::MutationOutcome::new(doctype_diff(XmlDoctype { prolog_position: doctype.prolog_position, name: doctype.name.clone(), external_id: external_id.clone(), declarations: doctype.declarations.clone() })),
        }
    }
    fn inverse(&self, base: &XmlSnapshot) -> Result<Vec<XmlValidMutation>, semio_framework_value::ValueError> {
        Ok(vec![XmlValidMutation::SetExternalSubset(set_external_subset::SetExternalSubset { external_id: base.doc.doctype.as_ref().and_then(|doctype| doctype.external_id.clone()) })])
    }
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native("Set external subset", "Externe Teilmenge setzen")
    }
    fn target(&self) -> Vec<String> {
        Vec::new()
    }
}
//#endregion 🔖️Payload
