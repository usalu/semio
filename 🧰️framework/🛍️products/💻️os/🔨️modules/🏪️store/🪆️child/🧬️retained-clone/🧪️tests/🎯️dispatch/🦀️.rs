//! 🎯️ artifact child generic selected close dispatch through actual source lifecycle.
use super::*;
use semio_framework_value::{RetirementDemand,ValueError,retained_clone::{RetainedCloneSource,RetainedClone,RetainedCloneCursor,RetainedCloneGrant,RetainedCloneProgress}};
use semio_framework_value_testing_allocator::observe_retirement_allocations;
#[path="../../../../../../../../🔨️modules/🌱️value/🧬️retained-clone/🧬️contract/🧪️tests/♻️close/🎯️dispatch/🦀️.rs"]
mod contract;
pub(super) use contract::record;
#[test]
fn artifact_child_generic_close_dispatch_selects_one_body_correlated_event(){contract::verify(||ArtifactChild::<()>::new("selected".into(),semio_framework_artifact_reference::ArtifactRef{artifact_id:"target".into(),dialect:semio_framework_artifact_reference::ArtifactDialect{artifact_kind:"test.child.value".into(),standard:"test".into(),subset:"*".into()}}),ArtifactChildCloneCursor::<()>::close_demands);}

#[test]
fn original_child_spent_take_has_no_phantom_work(){
 let mut cursor=ArtifactChildCloneCursor::<()>::default();cursor.spent=true;
 let grant=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:usize::MAX,maximum_depth:1,..Default::default()};
 let turn=cursor.take(grant);assert_eq!(turn.work,0);assert!(turn.result.unwrap().is_none());
}
