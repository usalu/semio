//! 👓️ Set Layer Visibility in the remodeling config channel.

use super::{RemodelingConfig, RemodelingConfigMutation, ReplaceConfig};

#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize, dsl::ToValue, dsl::FromValue, dsl::DslRecord, dsl::MutationLeaf)]
#[dsl(keyword = "set-layer-visibility")]
#[mutation_leaf(contract = ::protocol)]
pub struct SetLayerVisibility {
    pub layer: String,
    pub visible: bool,
}

impl protocol::MutationKind<RemodelingConfig, RemodelingConfigMutation> for SetLayerVisibility {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "set", entity: "layer-visibility", kind: "set-layer-visibility", record: "SetLayerVisibility" };
    fn diff(&self, base: &RemodelingConfig) -> protocol::MutationOutcome<RemodelingConfig> {
        let mut next = base.clone();
        match self.layer.as_str() {
            "mesh" => next.layers.mesh = self.visible,
            "dense" => next.layers.dense = self.visible,
            "sparse" => next.layers.sparse = self.visible,
            "cameras" => next.layers.cameras = self.visible,
            "gcps" => next.layers.gcps = self.visible,
            _ => {}
        }
        protocol::MutationOutcome::new(next)
    }
    fn inverse(&self, base: &RemodelingConfig) -> Vec<RemodelingConfigMutation> {
        vec![RemodelingConfigMutation::ReplaceConfig(ReplaceConfig { config: base.clone() })]
    }
    fn label(&self) -> String {
        "Set Layer Visibility".into()
    }
    fn target(&self) -> Vec<String> {
        vec!["layer".into()]
    }
}
