//! 🔏️ The operation identity wire of a DAG mutation is its canonical native tree, walked by ordinal path without a mirror.
use crate::os_store::{ArtifactCanonicalJson, ArtifactCanonicalJsonNode, ArtifactCanonicalJsonTree, ArtifactCanonicalJsonText};
use crate::DagMutation;
use semio_framework_value::ValueError;

fn descend<'a>(root: &'a dyn ArtifactCanonicalJsonTree, path: &[usize]) -> Result<&'a dyn ArtifactCanonicalJsonTree, ValueError> {
    let mut node = root;
    for ordinal in path {
        node = node.canonical_tree_child(*ordinal)?;
    }
    Ok(node)
}

impl ArtifactCanonicalJson for DagMutation {
    fn canonical_json_node(&self, path: &[usize]) -> Result<ArtifactCanonicalJsonNode<'_>, ValueError> {
        descend(self, path)?.canonical_tree_node()
    }

    fn canonical_json_key(&self, object_path: &[usize], index: usize) -> Result<ArtifactCanonicalJsonText<'_>, ValueError> {
        descend(self, object_path)?.canonical_tree_key(index)
    }
}
