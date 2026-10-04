//! ➖️ Revision-bound table row removal over canonical WordprocessingML.

use super::*;

#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
pub struct RemoveTableRow {
    pub address: DocxXmlAddress,
    pub index: usize,
}

impl protocol::MutationKind<DocxSnapshot, DocxMutation> for RemoveTableRow {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "remove", entity: "table-row", kind: "remove-table-row", record: "RemoveTableRow" };

    fn diff(&self, base: &DocxSnapshot) -> protocol::MutationOutcome<<DocxMutation as Mutation<DocxSnapshot>>::Diff> {
        agg_diff(&DocxMutation::RemoveTableRow(self.clone()), base)
    }

    fn inverse(&self, base: &DocxSnapshot) -> Result<Vec<DocxMutation>, semio_framework_value::ValueError> {
    Ok({
        agg_inverse(&DocxMutation::RemoveTableRow(self.clone()), base)?
    
    })
}

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native("Remove table row", "Tabellenzeile entfernen")
    }

    fn target(&self) -> Vec<String> {
        std::iter::once(self.address.part_path.clone()).chain(self.address.node_path.iter().map(usize::to_string)).chain(std::iter::once(self.index.to_string())).collect()
    }
}
