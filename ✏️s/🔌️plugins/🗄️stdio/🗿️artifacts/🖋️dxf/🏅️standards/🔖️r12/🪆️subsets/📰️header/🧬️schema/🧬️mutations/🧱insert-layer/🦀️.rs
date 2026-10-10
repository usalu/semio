//! 🧱️ `insert-layer` — authored as its own mutation leaf. It builds its own sparse diff and concrete inverse from its payload and reads of `base`.

use super::*;

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf, semio_framework_value::RetireOwned)]
#[mutation_leaf(contract = ::protocol)]
pub struct InsertLayer {
    pub index: usize,
    pub layer: DxfLayer,
}

impl protocol::MutationKind<DxfSnapshot, DxfMutation> for InsertLayer {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "insert", entity: "layer", kind: "insert-layer", record: "InsertLayer" };

    fn diff(&self, base: &DxfSnapshot) -> protocol::MutationOutcome<<DxfMutation as Mutation<DxfSnapshot>>::Diff> {
        let Self { index, layer } = self;
        protocol::MutationOutcome::new(diff_insert_layer(*index, layer.clone()))
    }
    fn inverse(&self, base: &DxfSnapshot) -> Result<Vec<DxfMutation>, semio_framework_value::ValueError> {
        let Self { layer, .. } = self;
        Ok(vec![DxfMutation::RemoveLayer(remove_layer::RemoveLayer { name: layer.name.clone() })])
    }
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native("Insert layer", "Layer einfügen")
    }
    fn target(&self) -> Vec<String> {
        Vec::new()
    }
}
//#endregion 🔖️Payload
