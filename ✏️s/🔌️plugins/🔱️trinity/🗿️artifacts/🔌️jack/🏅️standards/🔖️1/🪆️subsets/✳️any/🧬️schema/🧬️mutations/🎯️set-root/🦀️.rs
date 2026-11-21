//! 🎯️ Sets or clears the root of the Jack document.

use super::{JackDiff, JackSnapshot, TrinityGraphMutation};
use crate::standards::v1::subsets::any::schema::graph::GraphEffect;

/// 🌱️ The root node selected by the parent document after the edit.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
pub struct SetRoot {
    pub value: Option<String>,
}

/// 🏗️ Builds one exact root-selection edit.
pub fn set_root(value: Option<String>) -> TrinityGraphMutation {
    SetRoot { value }.into()
}

/// 🧹️ Clears the selected root when the final effects remove its node from the graph.
pub fn clear_deleted_root(snapshot: &JackSnapshot, effects: &[GraphEffect]) -> Option<TrinityGraphMutation> {
    let root = snapshot.root_node_id.as_deref()?;
    effects.iter().rev().find_map(|effect| match effect {
        GraphEffect::DeleteNode(id) if id == root => Some(true),
        GraphEffect::CreateNode(node) if node.id == root => Some(false),
        _ => None,
    }).filter(|deleted| *deleted).map(|_| set_root(None))
}

impl protocol::MutationKind<JackSnapshot, TrinityGraphMutation> for SetRoot {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "set", entity: "root", kind: "set-root", record: "SetRoot" };

    fn label(&self) -> semio_ui_locale::LocalizedLabel {
        semio_ui_locale::LocalizedLabel::native("Set graph root", "Graphwurzel setzen")
    }

    fn target(&self) -> Vec<String> {
        vec!["root_node_id".into()]
    }
}

impl protocol::ColdMutationKind<JackSnapshot, TrinityGraphMutation> for SetRoot {
    fn diff(&self, base: &JackSnapshot) -> protocol::MutationOutcome<JackDiff> {
        if base.root_node_id == self.value {
            return protocol::MutationOutcome::empty().warning("mutation.no-op", "The Jack graph root is unchanged.");
        }
        protocol::MutationOutcome::new(JackDiff { root_node_id: Some(self.value.clone()), ..Default::default() })
    }

    fn inverse(&self, base: &JackSnapshot) -> Result<Vec<TrinityGraphMutation>, semio_framework_value::ValueError> {
        Ok(vec![set_root(base.root_node_id.clone())])
    }
}
