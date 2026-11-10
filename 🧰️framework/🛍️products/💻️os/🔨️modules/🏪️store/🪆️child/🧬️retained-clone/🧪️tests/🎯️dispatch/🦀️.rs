//! 🎯️ artifact child generic selected close dispatch through actual source lifecycle.
use super::*;
use semio_framework_value::{RetirementDemand,ValueError,retained_clone::{RetainedCloneSource,RetainedClone,RetainedCloneCursor,RetainedCloneGrant,RetainedCloneProgress,RetainedCursorRelease}};
use semio_framework_value_testing_allocator::observe_retirement_allocations;
#[path="../../../../../../../../🔨️modules/🌱️value/🧬️retained-clone/🧬️contract/🧪️tests/♻️close/🎯️dispatch/🦀️.rs"]
mod contract;
pub(super) use contract::record;
#[test]
fn artifact_child_generic_close_dispatch_selects_one_body_correlated_event(){contract::verify(||ArtifactChild::<()>::new("selected".into(),semio_framework_artifact_reference::ArtifactRef{artifact_id:"target".into(),dialect:semio_framework_artifact_reference::ArtifactDialect{artifact_kind:"test.child.value".into(),standard:"test".into(),subset:"*".into()}}),ArtifactChildCloneCursor::<()>::close_demands,|c|Some(c as *const _ as usize));}

#[test]
fn original_child_spent_take_has_no_phantom_work(){
 use semio_framework_value_testing_admission::{driven,EffectBank,Wallet};
 let mut cursor=ArtifactChildCloneCursor::<()>::default();cursor.spent=true;
 let grant=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:1<<20,maximum_depth:1,..Default::default()};
 let mut bank=EffectBank::new(Wallet::new(RetainedCloneGrant{maximum_items:1<<20,maximum_copy_bytes:1<<30,maximum_capacity_bytes:1<<30,maximum_release_bytes:1<<30,maximum_depth:1<<10},1<<30));
 let(mut output,mut taken)=(None,None);
 let(poll,_)=driven::<ValueError>(&mut bank,grant,1,|effect,reply,_|{let before=effect.performed_work();let done=cursor.take(&mut output,effect,reply);taken=Some(effect.performed_work()-before);done});
 assert_eq!(taken,Some(0));assert!(output.is_none());
 let close=RetainedCloneGrant{maximum_items:1<<10,maximum_copy_bytes:1<<20,maximum_capacity_bytes:1<<20,maximum_release_bytes:1<<20,maximum_depth:1<<6};
 for _ in 0..64{
  if cursor.terminal_is_empty(){break}
  let started=cursor.close_started();
  let(poll,_)=driven::<semio_framework_value::OriginalRetirementFailure>(&mut bank,close,64,|effect,reply,recover|{
   if recover{return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"spent child close never suspends"))}
   if started{cursor.close_into(4096,effect,reply)}else{cursor.begin_close_into(effect,reply)}
  });
  assert!(matches!(poll,semio_framework_value::RetainedWorkPoll::Complete(turn) if turn.result.is_ok()));
 }
 assert!(cursor.terminal_is_empty());
 assert!(matches!(poll,semio_framework_value::RetainedWorkPoll::Complete(turn) if turn.result.is_err()));
}
