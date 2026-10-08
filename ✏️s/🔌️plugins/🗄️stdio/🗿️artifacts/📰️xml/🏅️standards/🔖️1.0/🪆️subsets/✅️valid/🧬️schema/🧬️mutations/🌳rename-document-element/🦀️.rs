//! 🌳️ `rename-document-element` — authored as its own mutation leaf. It builds its own sparse diff and concrete inverse from its payload and reads of `base`.

use super::*;

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
pub struct RenameDocumentElement {
    pub(crate) name: String,
}

impl protocol::MutationKind<XmlSnapshot, XmlValidMutation> for RenameDocumentElement {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "rename", entity: "document-element", kind: "rename-document-element", record: "RenameDocumentElement" };

    fn diff(&self, base: &XmlSnapshot) -> protocol::MutationOutcome<<XmlValidMutation as Mutation<XmlSnapshot>>::Diff> {
        let Self { name } = self;
        match (document_element_name(base), base.doc.doctype.as_ref()) {
            (None, _) => rejected("rename-document-element: the document has no document element to rename".to_string()),
            (Some(_), None) => rejected("rename-document-element: the document has no DOCTYPE to keep in step with the new name — declare one first".to_string()),
            (Some(_), Some(doctype)) => {
                let mut diff = diff_at_path(&[], XmlNodeDiff::Element(XmlElementDiff { name: Some(name.clone()), attributes: None, children: None }));
                diff.doctype = Some(Some(XmlDoctype { prolog_position: doctype.prolog_position, name: name.clone(), external_id: doctype.external_id.clone(), declarations: doctype.declarations.clone() }));
                protocol::MutationOutcome::new(diff)
            }
        }
    }
    fn inverse(&self, base: &XmlSnapshot) -> Result<Vec<XmlValidMutation>, semio_framework_value::ValueError> {
        Ok(match document_element_name(base) {
            Some(name) => vec![XmlValidMutation::RenameDocumentElement(rename_document_element::RenameDocumentElement { name: name.to_string() })],
            None => Vec::new(),
        })
    }
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native("Rename document element", "Dokumentelement umbenennen")
    }
    fn target(&self) -> Vec<String> {
        Vec::new()
    }
}
//#endregion 🔖️Payload
