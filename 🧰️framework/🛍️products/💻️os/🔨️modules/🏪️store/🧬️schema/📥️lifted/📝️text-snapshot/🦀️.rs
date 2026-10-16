//! 📥️ Trait impls of `📝️text-snapshot` whose trait or type belongs to `os-store-space-schema` (orphan rule, PLAN-OS-KERNEL-CRATES.md 7.7): lifted out of `🏪️store/🚪️io/📝️text/📸️snapshot`.
use pack::snapshot::ArtifactDsl;
use semio_framework_diagnostic::{TextError, TextSpan};
use semio_framework_pack_json::{ArtifactCanonicalJsonNode as Node, ArtifactCanonicalJsonText as Text, ArtifactCanonicalJsonTree as Tree};
use semio_framework_value::{ValueError, ValueRefusalKind};
use crate::space_history_document::{SpaceAlternative, SpaceCheckpoint, SpaceHistorySnapshot, SpaceMemberPin};

impl ArtifactDsl for SpaceHistorySnapshot {
    const EXTENSION: &'static str = "space-history";
    fn parse_dsl(text: &str) -> Result<Self, TextError> {
        semio_framework_pack_json::from_json_str(text, semio_framework_pack_json::JsonMemberPolicy::Reject).map_err(|error| TextError::new(error.kind, error.message, TextSpan::at(1, 1)))
    }
    fn print_dsl(&self) -> String {
        semio_framework_pack_json::to_json_string(self)
    }
}

fn canonical_ordinal()->ValueError{ValueError::literal(ValueRefusalKind::InvariantViolated,"space history canonical ordinal is absent")}
macro_rules! record_tree {
 ($type:ty { $($index:literal => $field:ident $key:literal),+ $(,)? })=>{
  impl Tree for $type {
   fn canonical_tree_node(&self)->Result<Node<'_>,ValueError>{Ok(Node::Object([$(stringify!($field)),+].len()))}
   fn canonical_tree_child(&self,ordinal:usize)->Result<&dyn Tree,ValueError>{match ordinal{$($index=>Ok(&self.$field),)+_=>Err(canonical_ordinal())}}
   fn canonical_tree_key(&self,ordinal:usize)->Result<Text<'_>,ValueError>{match ordinal{$($index=>Ok($key.into()),)+_=>Err(canonical_ordinal())}}
  }
 };
}

record_tree!(SpaceMemberPin{0=>document_id "documentId",1=>checkpoint_id "checkpointId",2=>alternative_id "alternativeId"});
record_tree!(SpaceAlternative{0=>id "id",1=>name "name",2=>checkpoint_ids "checkpointIds"});
impl Tree for SpaceCheckpoint {
 fn canonical_tree_node(&self)->Result<Node<'_>,ValueError>{Ok(Node::Object(if self.parent_id.is_some(){6}else{5}))}
 fn canonical_tree_child(&self,ordinal:usize)->Result<&dyn Tree,ValueError>{let ordinal=if self.parent_id.is_none()&&ordinal>=1{ordinal.checked_add(1).ok_or_else(canonical_ordinal)?}else{ordinal};match ordinal{0=>Ok(&self.id),1 if self.parent_id.is_some()=>Ok(&self.parent_id),2=>Ok(&self.message),3=>Ok(&self.authors),4=>Ok(&self.timestamp),5=>Ok(&self.members),_=>Err(canonical_ordinal())}}
 fn canonical_tree_key(&self,ordinal:usize)->Result<Text<'_>,ValueError>{let ordinal=if self.parent_id.is_none()&&ordinal>=1{ordinal.checked_add(1).ok_or_else(canonical_ordinal)?}else{ordinal};match ordinal{0=>Ok("id".into()),1 if self.parent_id.is_some()=>Ok("parentId".into()),2=>Ok("message".into()),3=>Ok("authors".into()),4=>Ok("timestamp".into()),5=>Ok("members".into()),_=>Err(canonical_ordinal())}}
}
