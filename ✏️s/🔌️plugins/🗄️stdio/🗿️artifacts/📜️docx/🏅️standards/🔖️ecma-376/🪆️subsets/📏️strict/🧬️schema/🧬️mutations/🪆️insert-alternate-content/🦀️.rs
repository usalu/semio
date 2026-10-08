//! 📏️ `insert-alternate-content` — authored as its own mutation leaf. It builds its own sparse diff and concrete inverse from its payload and reads of `base`.

use super::*;

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
pub struct InsertAlternateContent {
    pub(crate) path: String,
}

impl protocol::MutationKind<DocxSnapshot, DocxStrictMutation> for InsertAlternateContent {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "insert", entity: "alternate-content", kind: "insert-alternate-content", record: "InsertAlternateContent" };

    fn diff(&self, base: &DocxSnapshot) -> protocol::MutationOutcome<DocxDiff> {
        protocol::MutationOutcome::new(diff_append_alternate_content(base, &self.path))
    }

    fn inverse(&self, base: &DocxSnapshot) -> Result<Vec<DocxStrictMutation>, semio_framework_value::ValueError> {
        Ok(if base.xml_part(&self.path).is_some() { vec![DocxStrictMutation::RemoveAlternateContent(remove_alternate_content::RemoveAlternateContent { path: self.path.clone() })] } else { Vec::new() })
    }

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native("Insert alternate content", "Alternativen Inhalt einfügen")
    }
    fn target(&self) -> Vec<String> {
        Vec::new()
    }
}
//#endregion 🔖️Payload
