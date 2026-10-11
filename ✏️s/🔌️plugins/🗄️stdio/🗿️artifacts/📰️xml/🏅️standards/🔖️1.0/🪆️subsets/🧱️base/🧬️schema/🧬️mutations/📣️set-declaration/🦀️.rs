//! 🧬️ Direct set-declaration mutation owner.
use crate::schema::diff::XmlDiff;
use crate::schema::snapshot::XmlDeclaration;
use crate::XmlSnapshot;

#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
pub struct SetDeclarationMutation {
    pub declaration: Option<XmlDeclaration>,
}

pub type SetDeclarationPayload = SetDeclarationMutation;

impl protocol::MutationKind<XmlSnapshot, super::XmlMutation> for SetDeclarationMutation {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "set", entity: "declaration", kind: "set-declaration", record: "SetDeclaration" };

    fn diff(&self, base: &XmlSnapshot) -> protocol::MutationOutcome<XmlDiff> {
        protocol::MutationOutcome::new(XmlDiff { prolog: None, epilog: None, declaration: Some(self.declaration.clone()), doctype: None, root: None })
    }

    fn inverse(&self, base: &XmlSnapshot) -> Result<Vec<super::XmlMutation>, semio_framework_value::ValueError> {
        Ok(vec![super::XmlMutation::SetDeclaration(Self { declaration: base.doc.declaration.clone() })])
    }

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native("Set Declaration", "Deklaration setzen")
    }
    fn target(&self) -> Vec<String> {
        vec!["set-declaration".to_string()]
    }
}

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
