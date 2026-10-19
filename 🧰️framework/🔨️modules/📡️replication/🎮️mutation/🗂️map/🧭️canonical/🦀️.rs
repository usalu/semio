//! 🧭️ Canonical map wire traversal borrows original entry fields without a payload mirror.
use super::*;
use semio_framework_pack_json::{ArtifactCanonicalJsonTree as Tree, ArtifactCanonicalJsonNode as Node, ArtifactCanonicalJsonText as Text};

fn missing() -> ValueError { ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated, "map canonical ordinal is absent") }

impl<V: Tree> Tree for MapDelta<V> {
    fn canonical_tree_node(&self) -> Result<Node<'_>, ValueError> { Ok(Node::Object(1)) }
    fn canonical_tree_child(&self, ordinal: usize) -> Result<&dyn Tree, ValueError> { if ordinal == 0 { Ok(&self.entries) } else { Err(missing()) } }
    fn canonical_tree_key(&self, ordinal: usize) -> Result<Text<'_>, ValueError> { if ordinal == 0 { Ok("entries".into()) } else { Err(missing()) } }
}
impl<V: Tree> Tree for MapEntryDelta<V> {
    fn canonical_tree_node(&self) -> Result<Node<'_>, ValueError> { Ok(Node::Object(2)) }
    fn canonical_tree_child(&self, ordinal: usize) -> Result<&dyn Tree, ValueError> { match ordinal { 0 => Ok(&self.operation), 1 => Ok(&self.precondition), _ => Err(missing()) } }
    fn canonical_tree_key(&self, ordinal: usize) -> Result<Text<'_>, ValueError> { match ordinal { 0 => Ok("operation".into()), 1 => Ok("precondition".into()), _ => Err(missing()) } }
}
impl Tree for MapPresence { fn canonical_tree_node(&self) -> Result<Node<'_>, ValueError> { Ok(Node::String(self.name())) } }
impl<V: Tree> Tree for MapEntryOperation<V> {
    fn canonical_tree_node(&self) -> Result<Node<'_>, ValueError> { Ok(Node::Object(if matches!(self, Self::Set(_)) { 2 } else { 1 })) }
    fn canonical_tree_child(&self, ordinal: usize) -> Result<&dyn Tree, ValueError> {
        static SET: &str = "set"; static REMOVE: &str = "remove"; static REJECT: &str = "reject";
        match (ordinal, self) { (0, Self::Set(_)) => Ok(&SET), (0, Self::Remove) => Ok(&REMOVE), (0, Self::Reject) => Ok(&REJECT), (1, Self::Set(value)) => Ok(value), _ => Err(missing()) }
    }
    fn canonical_tree_key(&self, ordinal: usize) -> Result<Text<'_>, ValueError> { match (ordinal, self) { (0, _) => Ok("kind".into()), (1, Self::Set(_)) => Ok("value".into()), _ => Err(missing()) } }
}
