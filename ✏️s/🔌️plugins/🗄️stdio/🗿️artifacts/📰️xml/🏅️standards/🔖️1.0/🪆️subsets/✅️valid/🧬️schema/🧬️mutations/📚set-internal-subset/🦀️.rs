//! 📚️ `set-internal-subset` — authored as its own mutation leaf. It builds its own sparse diff and concrete inverse from its payload and reads of `base`.

use super::*;

//#region 🔖️Payload
#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
pub struct SetInternalSubset {
    pub(crate) declarations: Vec<XmlDtdDeclaration>,
}

impl protocol::MutationKind<XmlSnapshot, XmlValidMutation> for SetInternalSubset {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "set", entity: "internal-subset", kind: "set-internal-subset", record: "SetInternalSubset" };

    fn diff(&self, base: &XmlSnapshot) -> protocol::MutationOutcome<<XmlValidMutation as Mutation<XmlSnapshot>>::Diff> {
        let Self { declarations } = self;
        match base.doc.doctype.as_ref() {
            None => rejected("set-internal-subset: the document has no DOCTYPE, so there is no internal subset to replace".to_string()),
            Some(doctype) => protocol::MutationOutcome::new(doctype_diff(XmlDoctype { prolog_position: doctype.prolog_position, name: doctype.name.clone(), external_id: doctype.external_id.clone(), declarations: declarations.clone() })),
        }
    }
    fn inverse(&self, base: &XmlSnapshot) -> Result<Vec<XmlValidMutation>, semio_framework_value::ValueError> {
        Ok(match base.doc.doctype.as_ref() {
            Some(doctype) => vec![XmlValidMutation::SetInternalSubset(set_internal_subset::SetInternalSubset { declarations: doctype.declarations.clone() })],
            None => Vec::new(),
        })
    }
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native("Set internal subset", "Interne Teilmenge setzen")
    }
    fn target(&self) -> Vec<String> {
        Vec::new()
    }
}
//#endregion 🔖️Payload
