//! ➕️ Revision-bound table row insertion over canonical WordprocessingML.

use super::*;

#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
pub struct InsertTableRow {
    pub address: DocxXmlAddress,
    pub index: usize,
    pub cells: Vec<String>,
}

impl protocol::MutationKind<DocxSnapshot, DocxMutation> for InsertTableRow {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "insert", entity: "table-row", kind: "insert-table-row", record: "InsertTableRow" };

    fn diff(&self, base: &DocxSnapshot) -> protocol::MutationOutcome<<DocxMutation as Mutation<DocxSnapshot>>::Diff> {
        agg_diff(&DocxMutation::InsertTableRow(self.clone()), base)
    }

    fn inverse(&self, base: &DocxSnapshot) -> Vec<DocxMutation> {
        agg_inverse(&DocxMutation::InsertTableRow(self.clone()), base)
    }

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native("Insert table row", "Tabellenzeile einfügen")
    }

    fn target(&self) -> Vec<String> {
        std::iter::once(self.address.part_path.clone()).chain(self.address.node_path.iter().map(usize::to_string)).chain(std::iter::once(self.index.to_string())).collect()
    }
}
