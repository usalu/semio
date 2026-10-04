//! 🔷️ `insert-shape` — authored as its own mutation leaf. The aggregate's original `diff`/`inverse` bodies
//! were lifted verbatim into `agg_diff`/`agg_inverse`; this leaf reconstructs its aggregate value and
//! delegates, so the semantics are preserved by construction rather than re-derived.

use super::*;

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
pub struct InsertShape {
    pub(crate) vacancy: PptxXmlVacancyAddress,
    pub(crate) shape: XmlNode,
}

impl protocol::MutationKind<PptxSnapshot, PptxMutation> for InsertShape {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "insert", entity: "shape", kind: "insert-shape", record: "InsertShape" };

    fn diff(&self, base: &PptxSnapshot) -> protocol::MutationOutcome<<PptxMutation as Mutation<PptxSnapshot>>::Diff> {
        agg_diff(&PptxMutation::InsertShape(self.clone()), base)
    }
    fn inverse(&self, base: &PptxSnapshot) -> Result<Vec<PptxMutation>, semio_framework_value::ValueError> {
    Ok({
        agg_inverse(&PptxMutation::InsertShape(self.clone()), base)?
    
    })
}
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native("Insert shape", "Form einfügen")
    }
    fn target(&self) -> Vec<String> {
        std::iter::once(self.vacancy.container.part_path.clone()).chain(self.vacancy.container.node_path.iter().map(usize::to_string)).chain(std::iter::once(self.vacancy.index.to_string())).collect()
    }
}
//#endregion 🔖️Payload
