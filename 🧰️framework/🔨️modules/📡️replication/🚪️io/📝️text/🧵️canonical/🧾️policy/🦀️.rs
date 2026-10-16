//! 🧾️ Canonical JSON tree of the undo policy.
use super::{UndoPolicy};
use semio_framework_pack_json::{ArtifactCanonicalJsonTree, ArtifactCanonicalJsonNode as Node};
use semio_framework_value::{ValueError};

impl ArtifactCanonicalJsonTree for UndoPolicy {
 fn canonical_tree_node(&self)->Result<Node<'_>,ValueError>{Ok(Node::String(match self{Self::ExactBaseOnly=>"ExactBaseOnly",Self::TransformAgainstConcurrent=>"TransformAgainstConcurrent",Self::SemanticUndo=>"SemanticUndo",Self::CompensatingAction=>"CompensatingAction"}))}
}
