//! 🌐️ Canonical JSON tree of the foreign target.
use super::{ForeignTarget};
use semio_framework_pack_json::{ArtifactCanonicalJsonTree, ArtifactCanonicalJsonNode as Node, ArtifactCanonicalJsonText};
use semio_framework_value::{ValueError, ValueRefusalKind};

fn absent()->ValueError {ValueError::literal(ValueRefusalKind::InvariantViolated,"canonical original protocol field ordinal is absent")}
fn selected(ordinal:&mut usize,present:bool)->bool {if !present{return false;}if *ordinal==0{return true;}*ordinal-=1;false}

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
