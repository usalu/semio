//! 🪪️ Native author fields projected directly into canonical JSON traversal.
use crate::Author;
use semio_framework_pack_json::{ArtifactCanonicalJsonTree,ArtifactCanonicalJsonNode,ArtifactCanonicalJsonText};
use semio_framework_value::{ValueError,ValueRefusalKind};
impl ArtifactCanonicalJsonTree for Author {
 fn canonical_tree_node(&self)->Result<ArtifactCanonicalJsonNode<'_>,ValueError>{Ok(ArtifactCanonicalJsonNode::Object(if self.avatar.is_some(){3}else{2}))}
 fn canonical_tree_child(&self,ordinal:usize)->Result<&dyn ArtifactCanonicalJsonTree,ValueError>{match ordinal{0=>Ok(&self.id),1=>Ok(&self.name),2 if self.avatar.is_some()=>Ok(&self.avatar),_=>Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"author canonical child is absent"))}}
 fn canonical_tree_key(&self,ordinal:usize)->Result<ArtifactCanonicalJsonText<'_>,ValueError>{match ordinal{0=>Ok("id".into()),1=>Ok("name".into()),2 if self.avatar.is_some()=>Ok("avatar".into()),_=>Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"author canonical key is absent"))}}
}
