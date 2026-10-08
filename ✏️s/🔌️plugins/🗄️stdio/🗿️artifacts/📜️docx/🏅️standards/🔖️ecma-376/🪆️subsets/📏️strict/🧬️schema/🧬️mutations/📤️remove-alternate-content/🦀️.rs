//! 🧮️ `remove-alternate-content` — authored as its own mutation leaf. It builds its own sparse diff and concrete inverse from its payload and reads of `base`.

use super::*;

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
pub struct RemoveAlternateContent {
    pub(crate) path: String,
}

impl protocol::MutationKind<DocxSnapshot, DocxStrictMutation> for RemoveAlternateContent {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "remove", entity: "alternate-content", kind: "remove-alternate-content", record: "RemoveAlternateContent" };

    fn diff(&self, base: &DocxSnapshot) -> protocol::MutationOutcome<DocxDiff> {
        protocol::MutationOutcome::new(diff_strip_alternate_content(base, &self.path))
    }

    fn inverse(&self, base: &DocxSnapshot) -> Result<Vec<DocxStrictMutation>, semio_framework_value::ValueError> {
        Ok(if base.xml_part(&self.path).is_some() { vec![DocxStrictMutation::InsertAlternateContent(insert_alternate_content::InsertAlternateContent { path: self.path.clone() })] } else { Vec::new() })
    }

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native("Remove alternate content", "Alternativen Inhalt entfernen")
    }
    fn target(&self) -> Vec<String> {
        Vec::new()
    }
}
//#endregion 🔖️Payload
