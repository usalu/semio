//! ➕️ `insert-stream` — authored as its own mutation leaf. It builds its own sparse diff and concrete inverse from its payload and reads of `base`.

use super::*;

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
pub struct InsertStream {
    pub index: usize,
    pub stream: AviStream,
}

impl protocol::MutationKind<AviSnapshot, AviMutation> for InsertStream {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "insert", entity: "stream", kind: "insert-stream", record: "InsertStream" };

    fn diff(&self, base: &AviSnapshot) -> protocol::MutationOutcome<<AviMutation as Mutation<AviSnapshot>>::Diff> {
        let Self { index, stream } = self;
        protocol::MutationOutcome::new(AviDiff { streams: Some(IndexedDiff { removed: vec![], modified: vec![], added: vec![IndexedAdded { index: *index, item: stream.clone() }] }), ..AviDiff::default() })
    }
    fn inverse(&self, base: &AviSnapshot) -> Result<Vec<AviMutation>, semio_framework_value::ValueError> {
        let Self { index, .. } = self;
        Ok({ vec![AviMutation::RemoveStream(remove_stream::RemoveStream { index: *index })] })
    }
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native("Insert stream", "Datenstrom einfügen")
    }
    fn target(&self) -> Vec<String> {
        Vec::new()
    }
}
//#endregion 🔖️Payload
