//! 🧬️ Direct set-doctype mutation owner.
use crate::schema::diff::SvgDiff;
use crate::SvgSnapshot;
use semio_s_artifact_stdio_xml::schema::snapshot::XmlDoctype;

#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
pub struct SetDoctypePayload {
    pub doctype: Option<XmlDoctype>,
}

impl protocol::MutationKind<SvgSnapshot, super::SvgMutation> for SetDoctypePayload {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "set", entity: "doctype", kind: "set-doctype", record: "SetDoctype" };

    fn diff(&self, base: &SvgSnapshot) -> protocol::MutationOutcome<SvgDiff> {
        let Self { doctype } = self;
        if *doctype == base.doc.doctype {
            return protocol::MutationOutcome::new(SvgDiff::default());
        }
        protocol::MutationOutcome::new(SvgDiff { doctype: Some(doctype.clone()), ..Default::default() })
    }

    fn inverse(&self, base: &SvgSnapshot) -> Result<Vec<super::SvgMutation>, semio_framework_value::ValueError> {
        Ok(vec![super::SvgMutation::SetDoctype(Self { doctype: base.doc.doctype.clone() })])
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
