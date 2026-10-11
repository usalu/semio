//! 🧬️ Direct set-declaration mutation owner.
use crate::schema::diff::SvgDiff;
use crate::SvgSnapshot;
use semio_s_artifact_stdio_xml::schema::snapshot::XmlDeclaration;

#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
pub struct SetDeclarationPayload {
    pub declaration: Option<XmlDeclaration>,
}

impl protocol::MutationKind<SvgSnapshot, super::SvgMutation> for SetDeclarationPayload {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "set", entity: "declaration", kind: "set-declaration", record: "SetDeclaration" };

    fn diff(&self, base: &SvgSnapshot) -> protocol::MutationOutcome<SvgDiff> {
        let Self { declaration } = self;
        if *declaration == base.doc.declaration {
            return protocol::MutationOutcome::new(SvgDiff::default());
        }
        protocol::MutationOutcome::new(SvgDiff { declaration: Some(declaration.clone()), ..Default::default() })
    }

    fn inverse(&self, base: &SvgSnapshot) -> Result<Vec<super::SvgMutation>, semio_framework_value::ValueError> {
        Ok(vec![super::SvgMutation::SetDeclaration(Self { declaration: base.doc.declaration.clone() })])
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
