//! 🆔️ Canonical JSON trees of the replication identifiers and the hybrid logical timestamp.
use super::{HybridLogicalTimestamp, MutationId, ActorId, ArtifactId, SchemaId, PayloadHash};
use semio_framework_pack_json::{ArtifactCanonicalJsonTree, ArtifactCanonicalJsonNode as Node, ArtifactCanonicalJsonText};
use semio_framework_value::{ValueError, ValueRefusalKind};

fn absent()->ValueError {ValueError::literal(ValueRefusalKind::InvariantViolated,"canonical original protocol field ordinal is absent")}
fn selected(ordinal:&mut usize,present:bool)->bool {if !present{return false;}if *ordinal==0{return true;}*ordinal-=1;false}

impl HybridLogicalTimestamp {
 fn canonical_field(&self,mut ordinal:usize)->Result<(&'static str,&dyn ArtifactCanonicalJsonTree),ValueError>{
  if selected(&mut ordinal,true){return Ok(("actor",&self.actor));}
  if selected(&mut ordinal,true){return Ok(("physical_ms",&self.physical_ms));}
  if selected(&mut ordinal,true){return Ok(("logical",&self.logical));}
  Err(absent())
 }
}
impl ArtifactCanonicalJsonTree for HybridLogicalTimestamp {
 fn canonical_tree_node(&self)->Result<Node<'_>,ValueError>{Ok(Node::Object(1+1+1))}
 fn canonical_tree_child(&self,ordinal:usize)->Result<&dyn ArtifactCanonicalJsonTree,ValueError>{Ok(self.canonical_field(ordinal)?.1)}
 fn canonical_tree_key(&self,ordinal:usize)->Result<ArtifactCanonicalJsonText<'_>,ValueError>{Ok(self.canonical_field(ordinal)?.0.into())}
}
impl ArtifactCanonicalJsonTree for MutationId {fn canonical_tree_node(&self)->Result<Node<'_>,ValueError>{Ok(Node::String(&self.0))}}
impl ArtifactCanonicalJsonTree for ActorId {fn canonical_tree_node(&self)->Result<Node<'_>,ValueError>{Ok(Node::String(&self.0))}}
impl ArtifactCanonicalJsonTree for ArtifactId {fn canonical_tree_node(&self)->Result<Node<'_>,ValueError>{Ok(Node::String(&self.0))}}
impl ArtifactCanonicalJsonTree for SchemaId {fn canonical_tree_node(&self)->Result<Node<'_>,ValueError>{Ok(Node::String(&self.0))}}
impl ArtifactCanonicalJsonTree for PayloadHash {
 fn canonical_tree_node(&self)->Result<Node<'_>,ValueError>{self.0.canonical_tree_node()}
 fn canonical_tree_child(&self,ordinal:usize)->Result<&dyn ArtifactCanonicalJsonTree,ValueError>{self.0.canonical_tree_child(ordinal)}
}
