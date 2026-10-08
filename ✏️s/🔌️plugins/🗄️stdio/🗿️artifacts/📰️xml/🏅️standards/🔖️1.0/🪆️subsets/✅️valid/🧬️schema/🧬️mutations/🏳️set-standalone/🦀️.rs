//! 🏳️ `set-standalone` — authored as its own mutation leaf. It builds its own sparse diff and concrete inverse from its payload and reads of `base`.

use super::*;

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
pub struct SetStandalone {
    pub(crate) standalone: Option<bool>,
}

impl protocol::MutationKind<XmlSnapshot, XmlValidMutation> for SetStandalone {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "set", entity: "standalone", kind: "set-standalone", record: "SetStandalone" };

    fn diff(&self, base: &XmlSnapshot) -> protocol::MutationOutcome<<XmlValidMutation as Mutation<XmlSnapshot>>::Diff> {
        let Self { standalone } = self;
        {
            let next = match (&base.doc.declaration, standalone) {
                (None, None) => None,
                (None, Some(value)) => Some(XmlDeclaration { version: "1.0".to_string(), encoding: None, standalone: Some(*value), ..Default::default() }),
                (Some(declaration), value) => Some(XmlDeclaration { version: declaration.version.clone(), encoding: declaration.encoding.clone(), standalone: *value, quote: declaration.quote }),
            };
            protocol::MutationOutcome::new(XmlDiff { prolog: None, epilog: None, declaration: Some(next), doctype: None, root: None })
        }
    }
    fn inverse(&self, base: &XmlSnapshot) -> Result<Vec<XmlValidMutation>, semio_framework_value::ValueError> {
        Ok(match base.doc.declaration.as_ref() {
            Some(declaration) => vec![XmlValidMutation::SetStandalone(set_standalone::SetStandalone { standalone: declaration.standalone })],
            None => Vec::new(),
        })
    }
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native("Set standalone", "Standalone-Deklaration setzen")
    }
    fn target(&self) -> Vec<String> {
        Vec::new()
    }
}
//#endregion 🔖️Payload
