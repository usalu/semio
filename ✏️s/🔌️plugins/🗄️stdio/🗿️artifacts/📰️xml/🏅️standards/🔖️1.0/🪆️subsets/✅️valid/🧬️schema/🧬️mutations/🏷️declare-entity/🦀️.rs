//! 🏷️ `declare-entity` — authored as its own mutation leaf. It builds its own sparse diff and concrete inverse from its payload and reads of `base`.

use super::*;

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
pub struct DeclareEntity {
    pub(crate) index: usize,
    pub(crate) parameter: bool,
    pub(crate) name: String,
    pub(crate) value: String,
}

impl protocol::MutationKind<XmlSnapshot, XmlValidMutation> for DeclareEntity {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "add", entity: "entity", kind: "declare-entity", record: "DeclareEntity" };

    fn diff(&self, base: &XmlSnapshot) -> protocol::MutationOutcome<<XmlValidMutation as Mutation<XmlSnapshot>>::Diff> {
        let Self { index, parameter, name, value } = self;
        match base.doc.doctype.as_ref() {
            None => rejected("declare-entity: the document has no DOCTYPE, so there is no internal subset to declare an entity in".to_string()),
            Some(doctype) if entity_index(doctype, name).is_some() => rejected(format!("declare-entity: '{name}' is already declared — XML 1.0 §4.2 binds the FIRST declaration, so a second one is dead markup rather than an edit")),
            Some(doctype) => {
                let mut declarations = doctype.declarations.clone();
                let at = (*index).min(declarations.len());
                declarations.insert(at, XmlDtdDeclaration::Entity { parameter: *parameter, name: name.clone(), value: value.clone() });
                protocol::MutationOutcome::new(doctype_diff(XmlDoctype { prolog_position: doctype.prolog_position, name: doctype.name.clone(), external_id: doctype.external_id.clone(), declarations }))
            }
        }
    }
    fn inverse(&self, base: &XmlSnapshot) -> Result<Vec<XmlValidMutation>, semio_framework_value::ValueError> {
        Ok(match base.doc.doctype.as_ref() {
            Some(doctype) => vec![XmlValidMutation::SetInternalSubset(set_internal_subset::SetInternalSubset { declarations: doctype.declarations.clone() })],
            None => Vec::new(),
        })
    }
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native("Declare entity", "Entität deklarieren")
    }
    fn target(&self) -> Vec<String> {
        Vec::new()
    }
}
//#endregion 🔖️Payload
