//! 📏️ `insert-alternate-content` — authored as its own mutation leaf. It builds its own sparse diff and concrete inverse from its payload and reads of `base`.

use super::*;

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
pub struct InsertAlternateContent {
    pub(crate) path: String,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub(crate) node: Option<XmlNode>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub(crate) index: Option<usize>,
}

impl protocol::MutationKind<DocxSnapshot, DocxStrictMutation> for InsertAlternateContent {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "insert", entity: "alternate-content", kind: "insert-alternate-content", record: "InsertAlternateContent" };

    fn diff(&self, base: &DocxSnapshot) -> protocol::MutationOutcome<DocxDiff> {
        protocol::MutationOutcome::new(diff_insert_alternate_content(base, &self.path, self.node.as_ref(), self.index))
    }

    fn inverse(&self, base: &DocxSnapshot) -> Result<Vec<DocxStrictMutation>, semio_framework_value::ValueError> {
        Ok(insert_alternate_content_inverse(base, &self.path, self.index))
    }

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native("Insert alternate content", "Alternativen Inhalt einfügen")
    }
    fn target(&self) -> Vec<String> {
        Vec::new()
    }
}
//#endregion 🔖️Payload
