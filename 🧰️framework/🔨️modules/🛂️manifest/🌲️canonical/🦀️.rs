//! 🌲️ Canonical native JSON trees of host contributions keep their `ToValue` wire shape.
use super::{ProgramContributionEntry,TopicContribution};
use semio_framework_pack_json::{ArtifactCanonicalJsonNode as Node,ArtifactCanonicalJsonText as Text,ArtifactCanonicalJsonTree as Tree};
use semio_framework_value::{ValueError,ValueRefusalKind};
fn absent(reason:&'static str)->ValueError{ValueError::literal(ValueRefusalKind::InvariantViolated,reason)}
impl Tree for TopicContribution {
 fn canonical_tree_node(&self)->Result<Node<'_>,ValueError>{Ok(Node::Object(2))}
 fn canonical_tree_child(&self,ordinal:usize)->Result<&dyn Tree,ValueError>{match ordinal{0=>Ok(&self.topic),1=>Ok(&self.payload),_=>Err(absent("canonical topic contribution ordinal is absent"))}}
 fn canonical_tree_key(&self,ordinal:usize)->Result<Text<'_>,ValueError>{match ordinal{0=>Ok("topic".into()),1=>Ok("payload".into()),_=>Err(absent("canonical topic contribution key is absent"))}}
}
impl Tree for ProgramContributionEntry {
 fn canonical_tree_node(&self)->Result<Node<'_>,ValueError>{Ok(Node::Object(2))}
 fn canonical_tree_child(&self,ordinal:usize)->Result<&dyn Tree,ValueError>{match ordinal{0=>Ok(&self.plugin_id),1=>Ok(&self.topic_contribution),_=>Err(absent("canonical program contribution ordinal is absent"))}}
 fn canonical_tree_key(&self,ordinal:usize)->Result<Text<'_>,ValueError>{match ordinal{0=>Ok("pluginId".into()),1=>Ok("topicContribution".into()),_=>Err(absent("canonical program contribution key is absent"))}}
}
#[cfg(test)]
#[path="🧪️tests/🦀️.rs"]
mod tests;
