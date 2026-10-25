//! 🧪️ Native Store source conserves actual original fields across semantic, paged identity and hash turns.
use std::sync::Arc;
use semio_framework_os_spr::command::{ArtifactCanonicalEditAuthorityCursor, ArtifactCanonicalEditIdentityCursor};
use protocol::io::text::canonical::ArtifactCanonicalEditSealCursor;
use semio_framework_os_store_one_item_publication::native_source::{ArtifactStoreCanonicalOwner, ArtifactStoreCanonicalSource, Text, controlled, permits};
use semio_framework_os_store_one_item_publication::ArtifactStoreOneItemLiveAuthority;
use semio_framework_value::retirement::controlled::ControlledRetirement;
use semio_framework_os_spr::Edit;
use semio_framework_value::{ErasedSnapshotRetirement, ValueError};
use semio_framework_value::retained_clone::{RetainedCloneGrant, RetainedCloneProgress};
use semio_framework_value::retirement::arc_custody::SharedControlledRetirement;
use std::mem::size_of;
use semio_framework_trace::observe_heap_allocations_on_this_thread as observe;

fn grant()->RetainedCloneGrant{RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:4096,maximum_capacity_bytes:4096,maximum_release_bytes:262144,maximum_depth:4096}}
fn authority(actor:String)->Arc<ArtifactStoreOneItemLiveAuthority>{Arc::new(ArtifactStoreOneItemLiveAuthority::mint(semio_framework_job::OperationId(11), semio_framework_job::Generation(7), [9;32], 3, 4, semio_framework_os_spr::HybridLogicalTimestamp{actor:1,physical_ms:42,logical:2}, actor.into(), None, Some("group-1".into()), None))}
fn drive_close(owner:&mut dyn ArtifactStoreCanonicalOwner<bool>){owner.begin_close();for turn in 0..100000{if owner.terminal_is_empty(){return;}let(step,heap)=observe(||owner.advance(grant()).unwrap());assert!(step.progress().fits(grant()));assert_eq!((heap.requested_bytes,heap.released_bytes),(step.progress().retained_capacity_bytes,step.progress().released_bytes));assert!(turn<99999||owner.terminal_is_empty());}panic!("native Store original custody did not close");}

#[test]
fn canonical_store_original_native_source_preserves_semantic_paged_identity_hash_and_all_cancellation_frontiers(){
 let fixture:serde_json::Value=serde_json::from_str(include_str!("🧫️fixtures/🔣️.json")).unwrap();
 for repeat in[1,fixture["actorRepeat"].as_u64().unwrap()as usize]{for cancelled_at in[Some(0),Some(1),Some(2),Some(3),Some(4),Some(5),Some(6),None]{
  let actor="ä\0".repeat(repeat);let authority=authority(actor.clone());let edit=authority.next_edit(true,vec![false]);let digest=semio_framework_os_store_ledger::CursorRevisionAccumulator::edit_digest(&edit);let original=Box::new(edit);let pointer=original.as_ref()as*const Edit<bool>;let id=original.id.as_ptr();let forwards=original.forwards.as_ptr();
  let(mut owner,heap)=observe(||ArtifactStoreCanonicalSource::new(Arc::clone(&authority),original));assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));let mut cancelled=false;let mut turns=0;
  while !owner.ready()&&!owner.terminal_is_empty(){
   assert!(turns<1000000);if !cancelled&&cancelled_at==Some(owner.phase){owner.begin_close();cancelled=true;}
   let(demand,heap)=observe(||owner.next_demand().unwrap());assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));assert!(demand.copy_bytes<=4096&&demand.capacity_bytes<=4096);assert!(demand.depth<=4096);
   for denied in[RetainedCloneGrant::default(),RetainedCloneGrant{maximum_copy_bytes:demand.copy_bytes.saturating_sub(1),..grant()},RetainedCloneGrant{maximum_capacity_bytes:demand.capacity_bytes.saturating_sub(1),..grant()},RetainedCloneGrant{maximum_release_bytes:demand.release_bytes.saturating_sub(1),..grant()},RetainedCloneGrant{maximum_depth:demand.depth.saturating_sub(1),..grant()}]{if permits(demand,denied){continue;}let(step,heap)=observe(||owner.advance(denied).unwrap());assert_eq!(step.progress(),RetainedCloneProgress::default());assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));}
   let(step,heap)=observe(||owner.advance(grant()).unwrap());assert!(step.progress().fits(grant()));assert_eq!((heap.requested_bytes,heap.released_bytes),(step.progress().retained_capacity_bytes,step.progress().released_bytes));turns+=1;
  }
  if !cancelled{assert!(owner.ready());let((edit,ids,actual,progress),heap)=observe(||owner.take(grant()).unwrap().unwrap());assert!(progress.fits(grant()));assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));assert_eq!(actual,digest);assert_eq!(edit.as_ref()as*const Edit<bool>,pointer);assert_eq!(edit.id.as_ptr(),id);assert_eq!(edit.forwards.as_ptr(),forwards);assert_eq!(ids[0].to_string(),actor);assert_eq!(ids[1].to_string(),edit.id);assert_eq!(ids[2].to_string(),edit.id);let[a,b,c]=ids;let mut returned=ControlledRetirement::new((edit,a,b,c)).unwrap_or_else(|(error,_)|panic!("actual native return: {error}"));for turn in 0..1000000{if returned.terminal_is_empty(){break;}let(step,heap)=observe(||returned.step(grant()).unwrap());assert!(step.progress().fits(grant()));assert_eq!((heap.requested_bytes,heap.released_bytes),(step.progress().retained_capacity_bytes,step.progress().released_bytes));assert!(turn<999999||returned.terminal_is_empty());}assert!(returned.terminal_is_empty());let(_,heap)=observe(||drop(returned));assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));}
  assert!(owner.terminal_is_empty());let(_,heap)=observe(||drop(owner));assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));let mut original_authority=SharedControlledRetirement::lease(authority);for turn in 0..1000000{if original_authority.terminal_is_empty(){break;}let(step,heap)=observe(||original_authority.step(grant()).unwrap());assert!(step.progress().fits(grant()));assert_eq!((heap.requested_bytes,heap.released_bytes),(step.progress().retained_capacity_bytes,step.progress().released_bytes));assert!(turn<999999||original_authority.terminal_is_empty());}assert!(original_authority.terminal_is_empty());let(_,heap)=observe(||drop(original_authority));assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));
 }}
 let authority=authority("same-original".into());let mut edit=authority.next_edit(true,vec![false]);edit.actor=Some("different-original".into());let mut owner=ArtifactStoreCanonicalSource::new(authority,Box::new(edit));let mut denied=false;for _ in 0..100000{match owner.advance(grant()){Ok(_)=>{},Err(_)=>{denied=true;break;}}}assert!(denied);assert!(!owner.terminal_is_empty());drive_close(&mut owner);assert!(owner.terminal_is_empty());
}

#[derive(semio_framework_value::FactoryPayloadRetirement)]
struct BoolFactory;
impl semio_framework_value::ArtifactOwnedValueRetirementFactory<bool> for BoolFactory{
 fn retirement_birth_bytes(&self,_:&bool)->usize{semio_framework_value::retirement::controlled::controlled_retirement_birth_bytes::<bool>()}
 fn retire_owned(&self,original:bool,g:RetainedCloneGrant)->Result<(Box<dyn ErasedSnapshotRetirement>,RetainedCloneProgress),(ValueError,bool)>{semio_framework_value::retirement::controlled::admit_typed_controlled_retirement(original,g).map(|(owner,receipt)|(owner as Box<dyn ErasedSnapshotRetirement>,receipt))}
}
#[derive(semio_framework_value::FactoryPayloadRetirement)]
struct RootFactory;
impl semio_framework_value::SnapshotRetirementFactory<u64> for RootFactory{
 fn retirement_birth_bytes(&self,_:&Arc<u64>)->usize{semio_framework_value::retirement::arc_custody::shared_retirement_birth_bytes::<u64>()}
 fn retire(&self,original:Arc<u64>,g:RetainedCloneGrant)->Result<(Box<dyn ErasedSnapshotRetirement>,RetainedCloneProgress),(ValueError,Arc<u64>)>{semio_framework_value::retirement::arc_custody::admit_shared_retirement(original,g,true)}
}

#[test]
fn canonical_store_native_sealer_mounts_original_sources_and_keeps_prepared_and_frame_closure_separate(){
 for repeat in[1,8192]{for cancel_turn in[Some(0),Some(1),Some(2),Some(3),Some(17),Some(257),None]{
  let actor="ä\0".repeat(repeat);let original_authority=authority(actor.clone());let edit=original_authority.next_edit(true,vec![false]);let id=edit.id.as_ptr();let forwards=edit.forwards.as_ptr();let expected=semio_framework_os_store_ledger::CursorRevisionAccumulator::edit_digest(&edit);let post=Arc::new(17u64);let post_pointer=Arc::as_ptr(&post);let mutation_factory=Arc::new(BoolFactory);let snapshot_factory=Arc::new(RootFactory);
  let(result,heap)=observe(||semio_framework_os_store_one_item_publication::lifted_canonical_edit::ArtifactStoreOneItemSealer::admit(Arc::clone(&original_authority),edit,Arc::clone(&post),mutation_factory,snapshot_factory,grant()));let(mut owner,receipt)=result.unwrap_or_else(|(error,_,_,_,_,_)|panic!("actual canonical original source constructor: {error}"));assert!(receipt.fits(grant()));assert_eq!((heap.requested_bytes,heap.released_bytes),(receipt.retained_capacity_bytes,receipt.released_bytes));let mut turns=0;let mut cancelled=false;
  loop{
   if cancel_turn==Some(turns){owner.cancel();owner.begin_close();cancelled=true;break;}
   if let Some(prepared)=owner.prepared(){assert_eq!(prepared.edit_digest(),expected);assert_eq!(prepared.edit().id.as_ptr(),id);assert_eq!(prepared.edit().forwards.as_ptr(),forwards);assert_eq!(Arc::as_ptr(&prepared.post_snapshot()),post_pointer);assert_eq!(prepared.local_actor().as_ref().unwrap().to_string(),actor);assert!(owner.native.is_none());assert_eq!(owner.phase,6);assert_eq!(owner.progress().digest,expected);let query=semio_framework_os_store_one_item::ArtifactStoreOneItemGrant{maximum_items:1,maximum_copy_bytes:0,maximum_capacity_bytes:0,maximum_release_bytes:0,maximum_depth:0};let(step,heap)=observe(||semio_framework_os_store_testing_prelude::FixtureStoreReceiving::new(grant()).receive(|receiving|owner.advance(query,receiving)).unwrap());assert!(matches!(step,semio_framework_os_store_one_item::ArtifactStoreOneItemPreparationStep::Prepared(_,p) if p==RetainedCloneProgress::default()));assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));assert_eq!(owner.ownership_progress(),RetainedCloneProgress::default());owner.begin_close();break;}
   let(demand,heap)=observe(||owner.preparation_demands().unwrap());assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));assert!(demand.copy_bytes<=4096&&demand.capacity_bytes<=4096);let actual=semio_framework_os_store_one_item::ArtifactStoreOneItemGrant{maximum_items:grant().maximum_items,maximum_copy_bytes:grant().maximum_copy_bytes,maximum_capacity_bytes:grant().maximum_capacity_bytes,maximum_release_bytes:grant().maximum_release_bytes,maximum_depth:grant().maximum_depth};let(step,heap)=observe(||semio_framework_os_store_testing_prelude::FixtureStoreReceiving::new(grant()).receive(|receiving|owner.advance(actual,receiving)).unwrap());let _=step;let receipt=owner.ownership_progress();assert!(receipt.fits(grant()));assert_eq!((heap.requested_bytes,heap.released_bytes),(receipt.retained_capacity_bytes,receipt.released_bytes));turns+=1;assert!(turns<1000000);
  }
  for turn in 0..1000000{if owner.terminal_is_empty(){break;}let(demand,heap)=observe(||owner.retirement_demands(4096).unwrap());assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));assert!(demand.copy_bytes<=4096&&demand.capacity_bytes<=4096);let(step,heap)=observe(||owner.close_step(grant()).unwrap());assert!(step.progress().fits(grant()));assert_eq!((heap.requested_bytes,heap.released_bytes),(step.progress().retained_capacity_bytes,step.progress().released_bytes));assert!(turn<999999||owner.terminal_is_empty());}assert!(owner.terminal_is_empty());let(_,heap)=observe(||drop(owner));assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));
  for original in[SharedControlledRetirement::lease(original_authority)]{let mut original=original;for turn in 0..1000000{if original.terminal_is_empty(){break;}let(step,heap)=observe(||original.step(grant()).unwrap());assert!(step.progress().fits(grant()));assert_eq!((heap.requested_bytes,heap.released_bytes),(step.progress().retained_capacity_bytes,step.progress().released_bytes));assert!(turn<999999||original.terminal_is_empty());}assert!(original.terminal_is_empty());let(_,heap)=observe(||drop(original));assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));}
  let mut post=SharedControlledRetirement::lease(post);while !post.terminal_is_empty(){let(step,heap)=observe(||post.step(grant()).unwrap());assert_eq!((heap.requested_bytes,heap.released_bytes),(step.progress().retained_capacity_bytes,step.progress().released_bytes));}let(_,heap)=observe(||drop(post));assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));
 }}
}
