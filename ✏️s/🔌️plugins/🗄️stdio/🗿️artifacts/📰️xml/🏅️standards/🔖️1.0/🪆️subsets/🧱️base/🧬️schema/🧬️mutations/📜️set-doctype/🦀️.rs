//! 🧬️ Direct set-doctype mutation owner.
use crate::schema::diff::XmlDiff;
use crate::schema::snapshot::XmlDoctype;
use crate::XmlSnapshot;

#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
pub struct SetDoctypeMutation {
    pub doctype: Option<XmlDoctype>,
}

pub type SetDoctypePayload = SetDoctypeMutation;

impl protocol::MutationKind<XmlSnapshot, super::XmlMutation> for SetDoctypeMutation {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "set", entity: "doctype", kind: "set-doctype", record: "SetDoctype" };

    fn diff(&self, base: &XmlSnapshot) -> protocol::MutationOutcome<XmlDiff> {
        protocol::MutationOutcome::new(XmlDiff { prolog: None, epilog: None, declaration: None, doctype: Some(self.doctype.clone()), root: None })
    }

    fn inverse(&self, base: &XmlSnapshot) -> Result<Vec<super::XmlMutation>, semio_framework_value::ValueError> {
        Ok(vec![super::XmlMutation::SetDoctype(Self { doctype: base.doc.doctype.clone() })])
    }

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native("Set Doctype", "Dokumenttyp setzen")
    }
    fn target(&self) -> Vec<String> {
        vec!["set-doctype".to_string()]
    }
}

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
