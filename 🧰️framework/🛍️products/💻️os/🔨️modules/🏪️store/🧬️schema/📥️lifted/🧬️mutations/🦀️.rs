//! 📥️ Trait impls of `🧬️mutations` whose trait or type belongs to `os-store-space-schema` (orphan rule, PLAN-OS-KERNEL-CRATES.md 7.7): lifted out of `🏪️store/🚪️io/📝️text/🧬️mutations/🦀️.rs`.
use semio_framework_pack_json::ArtifactCanonicalJsonNode as Node;
use semio_framework_os_spr::OpText;
use crate::space_history_mutations::SpaceHistoryMutation;
use semio_framework_pack_json::ArtifactCanonicalJsonText as Text;
use semio_framework_diagnostic::TextError;
use semio_framework_diagnostic::TextSpan;
use semio_framework_pack_json::ArtifactCanonicalJsonTree as Tree;
use semio_framework_value::ValueError;
use semio_framework_value::ValueRefusalKind;

impl OpText for SpaceHistoryMutation {
    fn print_op(&self) -> String {
        semio_framework_pack_json::to_json_string(self)
    }
    fn parse_op(line: &str) -> Result<Self, TextError> {
        semio_framework_pack_json::from_json_str(line, semio_framework_pack_json::JsonMemberPolicy::Reject).map_err(|error| TextError::new(error.kind, error.message, TextSpan::at(1, 1)))
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

use crate::space_history_mutations::{CommitSpaceCheckpoint, CreateSpaceAlternative, RemoveSpaceAlternative, RemoveSpaceCheckpoint, SetActiveSpaceAlternative, SwitchSpaceAlternative};
record_tree!(CommitSpaceCheckpoint{0=>checkpoint "checkpoint"});
record_tree!(CreateSpaceAlternative{0=>alternative "alternative"});
record_tree!(SwitchSpaceAlternative{0=>alternative_id "alternativeId"});
record_tree!(RemoveSpaceCheckpoint{0=>checkpoint_id "checkpointId"});
record_tree!(RemoveSpaceAlternative{0=>alternative_id "alternativeId"});
record_tree!(SetActiveSpaceAlternative{0=>alternative_id "alternativeId"});

impl Tree for SpaceHistoryMutation {
 fn canonical_tree_node(&self)->Result<Node<'_>,ValueError>{Ok(Node::Object(2))}
 fn canonical_tree_child(&self,ordinal:usize)->Result<&dyn Tree,ValueError>{match(self,ordinal){
  (Self::CommitSpaceCheckpoint(_),0)=>{static KIND:&str="commitSpaceCheckpoint";Ok(&KIND)},(Self::CommitSpaceCheckpoint(value),1)=>Ok(value),
  (Self::CreateSpaceAlternative(_),0)=>{static KIND:&str="createSpaceAlternative";Ok(&KIND)},(Self::CreateSpaceAlternative(value),1)=>Ok(value),
  (Self::SwitchSpaceAlternative(_),0)=>{static KIND:&str="switchSpaceAlternative";Ok(&KIND)},(Self::SwitchSpaceAlternative(value),1)=>Ok(value),
  (Self::RemoveSpaceCheckpoint(_),0)=>{static KIND:&str="removeSpaceCheckpoint";Ok(&KIND)},(Self::RemoveSpaceCheckpoint(value),1)=>Ok(value),
  (Self::RemoveSpaceAlternative(_),0)=>{static KIND:&str="removeSpaceAlternative";Ok(&KIND)},(Self::RemoveSpaceAlternative(value),1)=>Ok(value),
  (Self::SetActiveSpaceAlternative(_),0)=>{static KIND:&str="setActiveSpaceAlternative";Ok(&KIND)},(Self::SetActiveSpaceAlternative(value),1)=>Ok(value),
  _=>Err(canonical_ordinal()),
 }}
 fn canonical_tree_key(&self,ordinal:usize)->Result<Text<'_>,ValueError>{match ordinal{0=>Ok("operation".into()),1=>Ok("payload".into()),_=>Err(canonical_ordinal())}}
}
