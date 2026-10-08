//! 📜️ `declare-doctype` — authored as its own mutation leaf. It builds its own sparse diff and concrete inverse from its payload and reads of `base`.

use super::*;

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
pub struct DeclareDoctype {
    pub(crate) external_id: Option<XmlExternalId>,
}

impl protocol::MutationKind<XmlSnapshot, XmlValidMutation> for DeclareDoctype {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "create", entity: "doctype", kind: "declare-doctype", record: "DeclareDoctype" };

    fn diff(&self, base: &XmlSnapshot) -> protocol::MutationOutcome<<XmlValidMutation as Mutation<XmlSnapshot>>::Diff> {
        let Self { external_id } = self;
        match document_element_name(base) {
            None => rejected("declare-doctype: the document has no document element, so §2.8 gives the DOCTYPE no Name to carry".to_string()),
            Some(name) => protocol::MutationOutcome::new(doctype_diff(XmlDoctype {
                prolog_position: base.doc.prolog.len() as u64,
                name: name.to_string(),
                external_id: external_id.clone(),
                declarations: base.doc.doctype.as_ref().map(|d| d.declarations.clone()).unwrap_or_default(),
            })),
        }
    }
    fn inverse(&self, base: &XmlSnapshot) -> Result<Vec<XmlValidMutation>, semio_framework_value::ValueError> {
        Ok(match base.doc.doctype.as_ref() {
            Some(doctype) => vec![XmlValidMutation::DeclareDoctype(declare_doctype::DeclareDoctype { external_id: doctype.external_id.clone() })],
            None => Vec::new(),
        })
    }
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native("Declare doctype", "Dokumenttyp deklarieren")
    }
    fn target(&self) -> Vec<String> {
        Vec::new()
    }
}
//#endregion 🔖️Payload
