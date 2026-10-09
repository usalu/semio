//! 🧵️ Canonical revision fields borrow only original persistent protocol owners.
#[path="🔏️seal/🦀️.rs"]
mod seal;
pub use seal::ArtifactCanonicalEditSealCursor;
#[path="🪪️identity/🦀️.rs"]
mod identity;
pub use identity::ArtifactCanonicalEditIdentityCursor;
#[path="🛂️authority/🦀️.rs"]
mod authority;
pub use authority::{ArtifactCanonicalEditAuthority,ArtifactCanonicalEditAuthorityCursor};
use super::{Edit,MutationMeta,MutationOrigin,ForeignTarget,TransactionRef};
use crate::{HybridLogicalTimestamp,MutationId,ActorId,ArtifactId,SchemaId,PayloadHash,UndoPolicy};
use semio_framework_pack_json::{ArtifactCanonicalJsonTree,ArtifactCanonicalJsonNode as Node,ArtifactCanonicalJsonText};
use semio_framework_value::{ValueError,ValueRefusalKind,retirement::{RetireOwned,RetirementCursor,sequence,deferred,sequence_birth_bytes,deferred_birth_bytes_for}};

fn absent()->ValueError {ValueError::literal(ValueRefusalKind::InvariantViolated,"canonical original protocol field ordinal is absent")}
fn selected(ordinal:&mut usize,present:bool)->bool {if !present{return false;}if *ordinal==0{return true;}*ordinal-=1;false}
static OWNER_KIND:&str="owner";
static CONTRIBUTED_KIND:&str="contributed";
static TRANSACTION_KIND:&str="transaction";

impl<M:ArtifactCanonicalJsonTree> Edit<M> {
 fn canonical_field(&self,mut ordinal:usize)->Result<(&'static str,&dyn ArtifactCanonicalJsonTree),ValueError>{
  if selected(&mut ordinal,true){return Ok(("id",&self.id));}
  if selected(&mut ordinal,self.actor.is_some()){return Ok(("actor",&self.actor));}
  if selected(&mut ordinal,true){return Ok(("forwards",&self.forwards));}
  if selected(&mut ordinal,true){return Ok(("inverse",&self.inverse));}
  if selected(&mut ordinal,!self.mutation_meta.is_empty()){return Ok(("mutationMeta",&self.mutation_meta));}
  if selected(&mut ordinal,self.verb.is_some()){return Ok(("verb",&self.verb));}
  if selected(&mut ordinal,true){return Ok(("startedAt",&self.started_at));}
  if selected(&mut ordinal,self.finished_at.is_some()){return Ok(("finishedAt",&self.finished_at));}
  if selected(&mut ordinal,true){return Ok(("line",&self.line));}
  Err(absent())
 }
}
impl<M:ArtifactCanonicalJsonTree> ArtifactCanonicalJsonTree for Edit<M> {
 fn canonical_tree_node(&self)->Result<Node<'_>,ValueError>{Ok(Node::Object(1+usize::from(self.actor.is_some())+1+1+usize::from(!self.mutation_meta.is_empty())+usize::from(self.verb.is_some())+1+usize::from(self.finished_at.is_some())+1))}
 fn canonical_tree_child(&self,ordinal:usize)->Result<&dyn ArtifactCanonicalJsonTree,ValueError>{Ok(self.canonical_field(ordinal)?.1)}
 fn canonical_tree_key(&self,ordinal:usize)->Result<ArtifactCanonicalJsonText<'_>,ValueError>{Ok(self.canonical_field(ordinal)?.0.into())}
}
impl MutationMeta {
 fn canonical_field(&self,mut ordinal:usize)->Result<(&'static str,&dyn ArtifactCanonicalJsonTree),ValueError>{
  if selected(&mut ordinal,self.mutation_id.is_some()){return Ok(("mutation_id",&self.mutation_id));}
  if selected(&mut ordinal,!self.dependencies.is_empty()){return Ok(("dependencies",&self.dependencies));}
  if selected(&mut ordinal,true){return Ok(("base_version",&self.base_version));}
  if selected(&mut ordinal,self.author_id.is_some()){return Ok(("author_id",&self.author_id));}
  if selected(&mut ordinal,true){return Ok(("timestamp",&self.timestamp));}
  if selected(&mut ordinal,true){return Ok(("undo_policy",&self.undo_policy));}
  if selected(&mut ordinal,self.payload_hash.is_some()){return Ok(("payload_hash",&self.payload_hash));}
  if selected(&mut ordinal,self.semantic_kind.is_some()){return Ok(("semantic_kind",&self.semantic_kind));}
  if selected(&mut ordinal,self.label.is_some()){return Ok(("label",&self.label));}
  if selected(&mut ordinal,self.group_id.is_some()){return Ok(("group_id",&self.group_id));}
  if selected(&mut ordinal,!self.origin.is_owner()){return Ok(("origin",&self.origin));}
  if selected(&mut ordinal,self.transaction.is_some()){return Ok(("transaction",&self.transaction));}
  Err(absent())
 }
}
impl ArtifactCanonicalJsonTree for MutationMeta {
 fn canonical_tree_node(&self)->Result<Node<'_>,ValueError>{Ok(Node::Object(usize::from(self.mutation_id.is_some())+usize::from(!self.dependencies.is_empty())+1+usize::from(self.author_id.is_some())+1+1+usize::from(self.payload_hash.is_some())+usize::from(self.semantic_kind.is_some())+usize::from(self.label.is_some())+usize::from(self.group_id.is_some())+usize::from(!self.origin.is_owner())+usize::from(self.transaction.is_some())))}
 fn canonical_tree_child(&self,ordinal:usize)->Result<&dyn ArtifactCanonicalJsonTree,ValueError>{Ok(self.canonical_field(ordinal)?.1)}
 fn canonical_tree_key(&self,ordinal:usize)->Result<ArtifactCanonicalJsonText<'_>,ValueError>{Ok(self.canonical_field(ordinal)?.0.into())}
}
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
impl ForeignTarget {
 fn canonical_field(&self,mut ordinal:usize)->Result<(&'static str,&dyn ArtifactCanonicalJsonTree),ValueError>{
  if selected(&mut ordinal,true){return Ok(("artifactId",&self.artifact_id));}
  if selected(&mut ordinal,true){return Ok(("artifactKind",&self.artifact_kind));}
  if selected(&mut ordinal,self.dialect.is_some()){return Ok(("dialect",&self.dialect));}
  Err(absent())
 }
}
impl ArtifactCanonicalJsonTree for ForeignTarget {
 fn canonical_tree_node(&self)->Result<Node<'_>,ValueError>{Ok(Node::Object(1+1+usize::from(self.dialect.is_some())))}
 fn canonical_tree_child(&self,ordinal:usize)->Result<&dyn ArtifactCanonicalJsonTree,ValueError>{Ok(self.canonical_field(ordinal)?.1)}
 fn canonical_tree_key(&self,ordinal:usize)->Result<ArtifactCanonicalJsonText<'_>,ValueError>{Ok(self.canonical_field(ordinal)?.0.into())}
}
impl TransactionRef {
 fn canonical_field(&self,mut ordinal:usize)->Result<(&'static str,&dyn ArtifactCanonicalJsonTree),ValueError>{
  if selected(&mut ordinal,true){return Ok(("id",&self.id));}
  if selected(&mut ordinal,true){return Ok(("tool",&self.tool));}
  Err(absent())
 }
}
impl ArtifactCanonicalJsonTree for TransactionRef {
 fn canonical_tree_node(&self)->Result<Node<'_>,ValueError>{Ok(Node::Object(1+1))}
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
impl ArtifactCanonicalJsonTree for UndoPolicy {
 fn canonical_tree_node(&self)->Result<Node<'_>,ValueError>{Ok(Node::String(match self{Self::ExactBaseOnly=>"ExactBaseOnly",Self::TransformAgainstConcurrent=>"TransformAgainstConcurrent",Self::SemanticUndo=>"SemanticUndo",Self::CompensatingAction=>"CompensatingAction"}))}
}
impl MutationOrigin {
 fn canonical_field(&self,ordinal:usize)->Result<(&'static str,&dyn ArtifactCanonicalJsonTree),ValueError>{
  match(self,ordinal){
   (Self::Owner,0)=>Ok(("kind",&OWNER_KIND)),
   (Self::Contributed{..},0)=>Ok(("kind",&CONTRIBUTED_KIND)),
   (Self::Contributed{plugin_id,..},1)=>Ok(("pluginId",plugin_id)),
   (Self::Contributed{mutation_id,..},2)=>Ok(("mutationId",mutation_id)),
   (Self::Contributed{payload_hash,..},3)=>Ok(("payloadHash",payload_hash)),
   (Self::Transaction{..},0)=>Ok(("kind",&TRANSACTION_KIND)),
   (Self::Transaction{initiator},1)=>Ok(("initiator",initiator)),
   _=>Err(absent()),
  }
 }
}
impl ArtifactCanonicalJsonTree for MutationOrigin {
 fn canonical_tree_node(&self)->Result<Node<'_>,ValueError>{Ok(Node::Object(match self{Self::Owner=>1,Self::Contributed{..}=>4,Self::Transaction{..}=>2}))}
 fn canonical_tree_child(&self,ordinal:usize)->Result<&dyn ArtifactCanonicalJsonTree,ValueError>{Ok(self.canonical_field(ordinal)?.1)}
 fn canonical_tree_key(&self,ordinal:usize)->Result<ArtifactCanonicalJsonText<'_>,ValueError>{Ok(self.canonical_field(ordinal)?.0.into())}
}

semio_framework_value::artifact_retire_struct!(MutationMeta{mutation_id,dependencies,base_version,author_id,timestamp,undo_policy,payload_hash,semantic_kind,label,group_id,origin,transaction});
semio_framework_value::artifact_retire_struct!(ForeignTarget{artifact_id,artifact_kind,dialect});
impl RetireOwned for PayloadHash {
 fn retirement(self)->Box<dyn RetirementCursor>{self.0.retirement()}
 fn retirement_birth_bytes(&self)->Option<usize>{self.0.retirement_birth_bytes()}
 fn controlled_retirement_supported()->bool{<[u8;32]>::controlled_retirement_supported()}
}
semio_framework_value::artifact_retire_leaf!(UndoPolicy);
impl<M:RetireOwned> RetireOwned for Edit<M> {
 fn retirement(self)->Box<dyn RetirementCursor>{sequence(vec![deferred(self.id),deferred(self.actor),deferred(self.line),deferred(self.forwards),deferred(self.inverse),deferred(self.mutation_meta),deferred(self.verb),deferred(self.sequence_number),deferred(self.started_at),deferred(self.finished_at)])}
 fn retirement_birth_bytes(&self)->Option<usize>{sequence_birth_bytes(&[deferred_birth_bytes_for(&self.id),deferred_birth_bytes_for(&self.actor),deferred_birth_bytes_for(&self.line),deferred_birth_bytes_for(&self.forwards),deferred_birth_bytes_for(&self.inverse),deferred_birth_bytes_for(&self.mutation_meta),deferred_birth_bytes_for(&self.verb),deferred_birth_bytes_for(&self.sequence_number),deferred_birth_bytes_for(&self.started_at),deferred_birth_bytes_for(&self.finished_at)])}
 fn controlled_retirement_supported()->bool{M::controlled_retirement_supported()}
}
#[cfg(test)]
#[path="🧪️tests/🦀️.rs"]
mod tests;
