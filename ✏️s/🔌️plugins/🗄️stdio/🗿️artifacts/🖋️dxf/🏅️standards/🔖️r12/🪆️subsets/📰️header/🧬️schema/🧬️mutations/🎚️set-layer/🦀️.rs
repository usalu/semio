//! 🎚️ `set-layer` — authored as its own mutation leaf. It builds its own sparse diff and concrete inverse from its payload and reads of `base`.

use super::*;

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
pub struct SetLayer {
    pub name: String,
    pub layer: DxfLayer,
}

impl protocol::MutationKind<DxfSnapshot, DxfMutation> for SetLayer {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "set", entity: "layer", kind: "set-layer", record: "SetLayer" };

    fn diff(&self, base: &DxfSnapshot) -> protocol::MutationOutcome<<DxfMutation as Mutation<DxfSnapshot>>::Diff> {
        let Self { name, layer } = self;
        protocol::MutationOutcome::new({
            let old = base.tables.layers.iter().find(|l| &l.name == name).cloned().unwrap_or_default();
            diff_set_layer(name, layer_diff_between(&old, layer))
        })
    }
    fn inverse(&self, base: &DxfSnapshot) -> Result<Vec<DxfMutation>, semio_framework_value::ValueError> {
        let Self { name, .. } = self;
        Ok({
            match base.tables.layers.iter().find(|l| &l.name == name) {
                Some(l) => vec![DxfMutation::SetLayer(set_layer::SetLayer { name: name.clone(), layer: l.clone() })],
                None => vec![DxfMutation::RemoveLayer(remove_layer::RemoveLayer { name: name.clone() })],
            }
        })
    }
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native("Set layer", "Layer setzen")
    }
    fn target(&self) -> Vec<String> {
        Vec::new()
    }
}
//#endregion 🔖️Payload
