use crate::retained_clone::FixtureSourceBirth;
use crate::retained_clone::{RetainedClone,RetainedCloneCursor,RetainedCloneGrant,RetainedCloneProgress,RetainedCloneRef,RetainedCloneSource,RetainedCloneStep};
use crate::{effect::{self,CPU},retirement::{RetireOwned,controlled::ControlledRetirement},ValueError};
use semio_framework_value_clone_source_testing::{admit_owned,FixtureRecoveryStep};
use semio_framework_value_testing_allocator::observe_retirement_allocations;
use semio_framework_value_testing_effect_bank::{EffectBank,Wallet};
type Failure=semio_framework_value_retirement::OriginalRetirementFailure;
type Alias<'r>=crate::retained_clone::OriginalEffectReply<'r,RetainedCloneStep,Failure>;
type Aliased<'r>=crate::retained_clone::OriginalEffectReplyComplete<'r,RetainedCloneStep,Failure>;

fn grant_of(d:crate::RetirementDemand)->RetainedCloneGrant{RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:d.copy_bytes,maximum_capacity_bytes:d.capacity_bytes,maximum_release_bytes:d.release_bytes,maximum_depth:d.depth}}
fn take_value<T:RetainedClone,C:RetainedCloneCursor<T>>(cursor:&mut C)->Option<T>{let demand=cursor.take_demand().unwrap();effect::take(cursor,grant_of(demand)).result.unwrap().map(|(value,_)|value)}
/// 🚗️ Drives one poll-shaped close turn under a real original action; the harness asserts the physical heap equals the recorded progress.
fn stepped(grant:RetainedCloneGrant,call:impl FnMut(&mut crate::RetainedAllocationEffect<'_>)->crate::OriginalRetirementPoll)->(RetainedCloneStep,(usize,usize)){let(outcome,heap)=effect::driven(grant,call);(outcome.result.unwrap(),heap)}
/// 🧵️ Drives one reply-shaped call under a real original action; the harness asserts the physical heap equals the recorded progress.
fn driven_reply(grant:RetainedCloneGrant,call:impl for<'r> FnMut(&mut crate::RetainedAllocationEffect<'_>,Alias<'r>,bool)->Result<Aliased<'r>,ValueError>)->crate::RetainedWorkOutcome<RetainedCloneStep,Failure>{
    semio_framework_value_clone_source_testing::drive::warm();
    semio_framework_value_clone_source_testing::drive::drive_reply(grant,CPU,|result|match result{Ok(step)=>step.progress(),Err(error)=>error.retained_progress()},call)
}
/// 🎟️ Advances one cursor turn under the physical heap check.
fn advanced<T:RetainedClone>(cursor:&mut T::Cursor,source:RetainedCloneRef<'_,T>,body:usize,grant:RetainedCloneGrant)->crate::RetainedWorkOutcome<RetainedCloneStep,Failure>{driven_reply(grant,|effect,reply,recover|if recover{cursor.recover_advance_into(source,body,CPU,effect,reply)}else{cursor.advance_into(source,body,CPU,effect,reply)})}
/// ♻️ Closes one cursor turn under the physical heap check.
fn closed<T:RetainedClone>(cursor:&mut T::Cursor,body:usize,grant:RetainedCloneGrant)->crate::RetainedWorkOutcome<RetainedCloneStep,Failure>{driven_reply(grant,|effect,reply,recover|if recover{cursor.recover_close_into(body,effect,reply)}else{cursor.close_into(body,effect,reply)})}
/// 🎫️ Admits one owner and authority as an original Source under `grant`, or recovers both inputs exactly as given.
fn admitted<A:RetireOwned+Sync+'static,B:RetireOwned>(owner:A,authority:B,grant:RetainedCloneGrant)->Result<(RetainedCloneSource<A>,RetainedCloneProgress),(ValueError,A,B)>{
    let mut bank=EffectBank::new(Wallet::new(effect::STOCK,CPU));
    let admitted=match admit_owned(&mut bank,owner,authority,grant).result{
        Ok(admitted)=>admitted,
        Err(mut recovery)=>{
            let mut output=None;
            for _ in 0..1000{match recovery.resume(effect::STOCK){FixtureRecoveryStep::Complete=>break,FixtureRecoveryStep::Fault=>panic!("original admission handback faulted: {:?}",recovery.cleanup_error()),_=>{}}}
            assert_eq!(recovery.finish_into(&mut output,effect::STOCK),FixtureRecoveryStep::Complete);
            return Err(output.expect("original admission handback"));
        }
    };
    Ok(admitted)
}
/// 🧱️ The exact capacity of one owned birth under one item and one depth level, with every other currency funded.
fn birth_grant(capacity:usize)->RetainedCloneGrant{RetainedCloneGrant{maximum_items:1,maximum_capacity_bytes:capacity,maximum_depth:1,..effect::STOCK}}
/// ♻️ Closes one Source to its terminal state and reports its physical births and releases.
fn close_source<T:Sync+'static,A:RetireOwned+Sync>(source:&mut RetainedCloneSource<T,A>,body:usize,limit:usize)->(usize,usize){
    let(mut born,mut released)=(0,0);
    for _ in 0..limit{
        if source.terminal_is_empty(){break;}
        let d=source.next_close_demand(body).unwrap();let grant=grant_of(d);assert_eq!(source.next_close_demand(grant.maximum_copy_bytes).unwrap(),d);
        let step=driven_reply(grant,|effect,reply,recover|if recover{source.recover_close_into(body,effect,reply)}else{source.close_into(body,effect,reply)}).result.unwrap();
        assert!(step.progress().fits(grant));born+=step.progress().retained_capacity_bytes;released+=step.progress().released_bytes;
    }
    assert!(source.terminal_is_empty());(born,released)
}
/// ♻️ Drains one owner through its controlled retirement and reports its physical births and releases.
fn drain<T:RetireOwned>(value:T)->(usize,usize){let mut owner=ControlledRetirement::new(value).unwrap_or_else(|_|panic!("original typed closure"));let(mut born,mut released)=(0,0);for _ in 0..10000{if owner.terminal_is_empty(){break;}let d=owner.selected_demand(4096).unwrap();let grant=grant_of(d);assert_eq!(owner.selected_demand(grant.maximum_copy_bytes).unwrap(),d);let(step,heap)=stepped(grant,|effect|owner.step(grant.maximum_copy_bytes,effect));assert!(step.progress().fits(grant));assert_eq!(heap,(step.progress().retained_capacity_bytes,step.progress().released_bytes));born+=heap.0;released+=heap.1;}assert!(owner.terminal_is_empty());assert_eq!(observe_retirement_allocations(||drop(owner)).1,(0,0));(born,released)}
/// 🏛️ Publishes one value as the original shared allocation under its actual funded effect.
fn sealed<T:RetireOwned+Sync>(value:T)->(crate::shared_issuance::SealedShared<T>,RetainedCloneProgress){
    use crate::shared_issuance::{SealedShared,SharedIssuer};
    let mut input=Some(value);let mut output=None;let grant=grant_of(SealedShared::<T>::source_admission_demand().unwrap());let mut actual=None;
    let(turn,_)=effect::with_effect(grant,CPU,&mut actual,|effect|SealedShared::admit_from(&mut input,SharedIssuer::owned(),&mut output,effect));
    let progress=turn.result.unwrap().progress();assert!(progress.fits(grant));assert_eq!(actual.unwrap_or(0),progress.retained_capacity_bytes);assert!(input.is_none());(output.expect("original shared admission"),progress)
}
/// 🪢️ Retains one issued reference as an owned projection through its two separately funded original turns and reports their physical heap.
fn retain<U:?Sized+Sync+'static>(row:RetainedCloneRef<'_,U>)->(crate::retained_clone::RetainedOwnedProjection<U>,(usize,usize)){
    let mut admission=crate::retained_clone::RetainedOwnedProjectionAdmission::<U>::default();let mut output=None;
    let g=grant_of(admission.demand(row).unwrap());let(first,first_heap)=observe_retirement_allocations(||effect::turn::<ValueError>(g,CPU,&mut |e,r|admission.admit(row,e,r)));let p=first.result.unwrap().progress();assert!(p.fits(g));assert_eq!(first_heap,(p.retained_capacity_bytes,p.released_bytes));
    let g=grant_of(admission.take_demand().unwrap());let(second,second_heap)=observe_retirement_allocations(||effect::turn::<ValueError>(g,CPU,&mut |e,r|admission.take(&mut output,e,r)));let p=second.result.unwrap().progress();assert!(p.fits(g));assert_eq!(second_heap,(p.retained_capacity_bytes,p.released_bytes));
    assert!(admission.terminal_is_empty());(output.expect("owned projection published"),(first_heap.0+second_heap.0,first_heap.1+second_heap.1))
}
/// 🧵️ Closes one retained projection alias to its terminal state and reports its physical births and releases.
fn drain_projection<T:?Sized+Sync>(mut owner:crate::retained_clone::RetainedOwnedProjection<T>)->(usize,usize){let(mut born,mut released)=(0,0);for _ in 0..10000{if owner.terminal_is_empty(){break;}let d=owner.close_demand(4096).unwrap();let grant=grant_of(d);assert_eq!(owner.close_demand(grant.maximum_copy_bytes).unwrap(),d);let step=driven_reply(grant,|effect,reply,recover|if recover{owner.recover_close_into(4096,effect,reply)}else{owner.close_into(4096,effect,reply)}).result.unwrap();assert!(step.progress().fits(grant));born+=step.progress().retained_capacity_bytes;released+=step.progress().released_bytes;}assert!(owner.terminal_is_empty());assert_eq!(observe_retirement_allocations(||drop(owner)).1,(0,0));(born,released)}

#[test]
fn retained_clone_owned_source_birth_is_funded_before_original_owner_transfer(){
    let law:serde_json::Value=serde_json::from_str(crate::text("🧬️retained-clone/🔗️source/🧪️tests/🧫️fixtures/🎟️owned-birth/🔣️.json")).unwrap();
    let((mut owner,mut authority),original)=observe_retirement_allocations(||{
        let mut owner=String::with_capacity(law["ownerCapacity"].as_u64().unwrap()as usize);owner.push_str(law["owner"].as_str().unwrap());
        let mut authority=String::with_capacity(law["authorityCapacity"].as_u64().unwrap()as usize);authority.push_str(law["authority"].as_str().unwrap());(owner,authority)
    });
    let owner_pointer=owner.as_ptr();let authority_pointer=authority.as_ptr();
    let(bytes,heap)=observe_retirement_allocations(||RetainedCloneSource::<String>::owned_constructor_capacity_bytes::<String>());assert_eq!(heap,(0,0));
    let grant=RetainedCloneGrant{maximum_items:law["birth"]["items"].as_u64().unwrap()as usize,maximum_capacity_bytes:bytes,maximum_depth:law["birth"]["depth"].as_u64().unwrap()as usize,..effect::STOCK};
    for refusal in law["refusals"].as_array().unwrap(){
        let short=refusal["shortBy"].as_u64().unwrap()as usize;
        let denied=match refusal["currency"].as_str().unwrap(){"items"=>RetainedCloneGrant{maximum_items:grant.maximum_items-short,..grant},"copy"=>RetainedCloneGrant{maximum_copy_bytes:grant.maximum_copy_bytes-short,..grant},"capacity"=>RetainedCloneGrant{maximum_capacity_bytes:0,..grant},"depth"=>RetainedCloneGrant{maximum_depth:grant.maximum_depth-short,..grant},_=>unreachable!()};
        let(error,returned,returned_authority)=admitted(owner,authority,denied).err().unwrap();owner=returned;authority=returned_authority;
        assert_eq!(format!("{:?}",error.kind),refusal["kind"].as_str().unwrap());assert_eq!(owner.as_ptr(),owner_pointer);assert_eq!(authority.as_ptr(),authority_pointer);
    }
    let(mut source,receipt)=admitted(owner,authority,grant).map_err(|(error,_,_)|error).unwrap();
    assert!(receipt.retained_capacity_bytes<=grant.maximum_capacity_bytes);assert_eq!(receipt.retained_capacity_bytes,bytes);assert!(receipt.copied_items>=1);assert_eq!(receipt.released_bytes,law["birth"]["releaseBytes"].as_u64().unwrap()as usize);assert_eq!(source.borrow().get().as_ptr(),owner_pointer);
    let(mut born,mut released)=(receipt.retained_capacity_bytes,receipt.released_bytes);
    let heap=close_source(&mut source,4096,law["maximumTurns"].as_u64().unwrap()as usize);born+=heap.0;released+=heap.1;
    assert!(source.terminal_is_empty());assert_eq!(released,original.0+born);assert_eq!(observe_retirement_allocations(||drop(source)).1,(0,0));
}

#[test]
fn retained_clone_owned_source_shared_alias_keeps_original_identity_in_every_close_order(){
 use crate::shared_issuance::SealedShared;
 let law:serde_json::Value=serde_json::from_str(crate::text("🧬️retained-clone/🔗️issuance-clone/🧪️tests/🧫️fixtures/🔣️.json")).unwrap();
 for text in law["texts"].as_array().unwrap(){for order in law["closeOrders"].as_array().unwrap(){for stop in std::iter::once(None).chain(law["interruptAfter"].as_array().unwrap().iter().map(|x|Some(x.as_u64().unwrap()as usize))){
  let(value,original)=observe_retirement_allocations(||text.as_str().unwrap().to_owned());let pointer=value.as_ptr();let(value,receipt)=sealed(value);let(mut born,mut released)=(receipt.retained_capacity_bytes,receipt.released_bytes);let identity=value.identity();
  let(mut source,receipt)=admitted(value,(),effect::STOCK).map_err(|(error,..)|error).unwrap_or_else(|error|panic!("original alias source admission: {error:?}"));born+=receipt.retained_capacity_bytes;released+=receipt.released_bytes;
  let mut cursor=SealedShared::<String>::retained_clone_cursor();let mut output=None;for turn in 0..3{if stop.is_some_and(|stop|turn>=stop){break;}let demand=cursor.advance_demands(source.borrow(),0,CPU).unwrap();let grant=grant_of(demand);for denied in [RetainedCloneGrant{maximum_items:0,..grant},RetainedCloneGrant{maximum_depth:0,..grant}]{let outcome=effect::advance(&mut cursor,source.borrow(),0,CPU,denied);assert_eq!(effect::heap_of(&outcome),(0,0));assert_eq!(outcome.result.unwrap().progress(),Default::default());assert_eq!(cursor.advance_demands(source.borrow(),0,CPU).unwrap(),demand);}let step=advanced::<SealedShared<String>>(&mut cursor,source.borrow(),0,grant).result.unwrap();assert!(step.progress().fits(grant));assert_eq!(step.progress().retained_capacity_bytes,0);born+=step.progress().retained_capacity_bytes;released+=step.progress().released_bytes;if matches!(step,RetainedCloneStep::Complete(_)){if stop.is_none(){output= take_value(&mut cursor);}break;}}
  if stop.is_none(){let output=output.as_ref().unwrap();assert_eq!(output.identity(),identity);assert_eq!(output.get().as_ptr(),pointer);assert_eq!(output.get(),text.as_str().unwrap());}
  if order=="outputFirst"{if let Some(output)=output.take(){let heap=drain(output);born+=heap.0;released+=heap.1;}}
  let heap=close_source(&mut source,4096,10000);born+=heap.0;released+=heap.1;drop(source);if let Some(output)=output.as_ref(){assert_eq!(output.identity(),identity);assert_eq!(output.get().as_ptr(),pointer);assert_eq!(output.get(),text.as_str().unwrap());}
  let heap=drain(cursor);born+=heap.0;released+=heap.1;if let Some(output)=output{assert_eq!(output.get().as_ptr(),pointer);let heap=drain(output);born+=heap.0;released+=heap.1;}
  assert_eq!(original.0-original.1+born,released);
 }}}
}

#[test]
fn retained_clone_owned_source_controlled_facade_retains_original_projection_in_both_close_orders(){
 let law:serde_json::Value=serde_json::from_str(crate::text("🧬️retained-clone/🔗️source/🧪️tests/🧫️fixtures/🎟️owned-birth/🔣️.json")).unwrap();
 for order in law["closeOrders"].as_array().unwrap(){let projection_first=order=="projectionFirst";let((value,authority),original)=observe_retirement_allocations(||{let mut value=String::with_capacity(law["ownerCapacity"].as_u64().unwrap()as usize);value.push_str(law["owner"].as_str().unwrap());let mut authority=String::with_capacity(law["authorityCapacity"].as_u64().unwrap()as usize);authority.push_str(law["authority"].as_str().unwrap());(value,authority)});let pointer=value.as_ptr();let(source,receipt)=admitted(value,authority,effect::STOCK).map_err(|(error,_,_)|error).unwrap();let(mut born,mut released)=(receipt.retained_capacity_bytes,receipt.released_bytes);
  let(projection,heap)=retain(source.borrow().project(1,|value:&String|value.as_str()));assert_eq!(heap,(0,0));assert_eq!(projection.borrow().unwrap().get().as_ptr(),pointer);
  let(mut source,heap)=observe_retirement_allocations(||ControlledRetirement::new(source).map_err(|(error,_)|error).unwrap());assert_eq!(heap,(0,0));
  fn drain_source(owner:&mut ControlledRetirement<RetainedCloneSource<String>>)->(usize,usize){let(mut born,mut released)=(0,0);for _ in 0..10000{if owner.terminal_is_empty(){break;}let d=owner.selected_demand(4096).unwrap();let grant=grant_of(d);assert_eq!(owner.selected_demand(grant.maximum_copy_bytes).unwrap(),d);for denied in [Some(RetainedCloneGrant{maximum_items:0,..grant}),grant.maximum_capacity_bytes.checked_sub(1).map(|v|RetainedCloneGrant{maximum_capacity_bytes:v,..grant}),grant.maximum_release_bytes.checked_sub(1).map(|v|RetainedCloneGrant{maximum_release_bytes:v,..grant})].into_iter().flatten(){let(step,heap)=stepped(denied,|effect|owner.step(denied.maximum_copy_bytes,effect));assert_eq!(step.progress(),Default::default());assert_eq!(heap,(0,0));}let(step,heap)=stepped(grant,|effect|owner.step(grant.maximum_copy_bytes,effect));assert!(step.progress().fits(grant));assert_eq!(heap,(step.progress().retained_capacity_bytes,step.progress().released_bytes));born+=heap.0;released+=heap.1;}assert!(owner.terminal_is_empty());(born,released)}
  if projection_first{let heap=drain_projection(projection);born+=heap.0;released+=heap.1;let heap=drain_source(&mut source);born+=heap.0;released+=heap.1;assert_eq!(observe_retirement_allocations(||drop(source)).1,(0,0));}else{let heap=drain_source(&mut source);born+=heap.0;released+=heap.1;assert_eq!(observe_retirement_allocations(||drop(source)).1,(0,0));assert_eq!(projection.borrow().unwrap().get().as_ptr(),pointer);assert_eq!(projection.borrow().unwrap().get(),law["owner"].as_str().unwrap());let heap=drain_projection(projection);born+=heap.0;released+=heap.1;}assert_eq!(released,original.0-original.1+born);
 }
}

#[test]
fn retained_clone_owned_source_closes_its_actual_shared_authority_lease(){
    use crate::shared_issuance::SealedShared;
    let law:serde_json::Value=serde_json::from_str(crate::text("🧬️retained-clone/🔗️source/🧪️tests/🧫️fixtures/🎟️owned-birth/🔣️.json")).unwrap();
    let((owner,authority),original)=observe_retirement_allocations(||{let mut owner=String::with_capacity(law["ownerCapacity"].as_u64().unwrap()as usize);owner.push_str(law["owner"].as_str().unwrap());let mut authority=String::with_capacity(law["authorityCapacity"].as_u64().unwrap()as usize);authority.push_str(law["authority"].as_str().unwrap());(owner,authority)});
    let pointer=authority.as_ptr();
    let(authority,progress)=sealed(authority);let(mut born,mut released)=(progress.retained_capacity_bytes,progress.released_bytes);
    let grant=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:SealedShared::<String>::duplicate_copy_bytes(),maximum_depth:1,..Default::default()};let mut actual=None;let(turn,_)=effect::with_effect(grant,CPU,&mut actual,|effect|authority.try_duplicate(effect));let(lease,progress)=turn.result.unwrap();assert_eq!(actual.unwrap_or(0),0);assert!(progress.fits(grant));
    let(mut source,receipt)=admitted(owner,lease,effect::STOCK).map_err(|(error,_,_)|error).unwrap();born+=receipt.retained_capacity_bytes;released+=receipt.released_bytes;
    let heap=close_source(&mut source,4096,law["maximumTurns"].as_u64().unwrap()as usize);born+=heap.0;released+=heap.1;
    assert!(source.terminal_is_empty());assert_eq!(authority.get().as_ptr(),pointer);assert_eq!(authority.get().as_str(),law["authority"].as_str().unwrap());assert_eq!(observe_retirement_allocations(||drop(source)).1,(0,0));
    let heap=drain(authority);born+=heap.0;released+=heap.1;
    assert_eq!(released,original.0+born);
}

#[test]
fn admitted_source_authority_preserves_original_owners_and_canceled_cursor_custody() {
    let law:serde_json::Value=serde_json::from_str(crate::text("🧬️retained-clone/🔗️source/🧪️tests/🧫️fixtures/🔣️.json")).unwrap();
    for row in law["vectors"].as_array().unwrap(){for work in law["workGrants"].as_array().unwrap(){
        let work=work.as_u64().unwrap()as usize;
        let ((owner,authority),original)=observe_retirement_allocations(||{
            let mut owner=String::with_capacity(row["ownerCapacity"].as_u64().unwrap()as usize);owner.push_str(row["owner"].as_str().unwrap());
            let mut authority=String::with_capacity(row["authorityCapacity"].as_u64().unwrap()as usize);authority.push_str(row["authority"].as_str().unwrap());
            (owner,authority)
        });
        let pointer=owner.as_ptr();let authority_pointer=authority.as_ptr();
        let bytes=RetainedCloneSource::<String>::constructor_capacity_bytes::<String>();
        let grant=birth_grant(bytes);
        let mut owner=owner;let mut authority=authority;
        for short in [RetainedCloneGrant{maximum_items:0,..grant},RetainedCloneGrant{maximum_capacity_bytes:0,..grant}]{
            let (_,returned,returned_authority)=admitted(owner,authority,short).err().unwrap();owner=returned;authority=returned_authority;
            assert_eq!(owner.as_ptr(),pointer);assert_eq!(authority.as_ptr(),authority_pointer);
        }
        let (mut source,receipt)=admitted(owner,authority,grant).map_err(|(error,_,_)|error).unwrap();
        assert_eq!(receipt.retained_capacity_bytes,bytes);
        let (mut born,mut released)=(receipt.retained_capacity_bytes,receipt.released_bytes);
        assert_eq!(source.borrow().get().as_ptr(),pointer);
        assert_eq!(serde_json::to_value(source.borrow().get()).unwrap(),row["owner"]);
        let mut cursor=String::retained_clone_cursor();
        let turn=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:work,maximum_capacity_bytes:4096,maximum_release_bytes:0,maximum_depth:64};
        let step=advanced::<String>(&mut cursor,source.borrow(),work,turn).result.unwrap();
        assert!(step.progress().fits(turn));born+=step.progress().retained_capacity_bytes;released+=step.progress().released_bytes;
        effect::begin_close(&mut cursor);
        for _ in 0..100000{
            if source.terminal_is_empty(){break;}
            let d=source.next_close_demand(work).unwrap();
            let grant=grant_of(d);assert_eq!(source.next_close_demand(grant.maximum_copy_bytes).unwrap(),d);
            let zero=driven_reply(RetainedCloneGrant{maximum_items:0,..grant},|effect,reply,recover|if recover{source.recover_close_into(work,effect,reply)}else{source.close_into(work,effect,reply)}).result.unwrap();assert_eq!(zero.progress(),RetainedCloneProgress::default());
            let step=driven_reply(grant,|effect,reply,recover|if recover{source.recover_close_into(work,effect,reply)}else{source.close_into(work,effect,reply)}).result.unwrap();assert!(step.progress().fits(grant));born+=step.progress().retained_capacity_bytes;released+=step.progress().released_bytes;
        }
        assert!(source.terminal_is_empty());assert_eq!(observe_retirement_allocations(||drop(source)).1,(0,0));
        for _ in 0..100000{
            let d=cursor.close_demands(work).unwrap();
            let grant=grant_of(d);assert_eq!(cursor.close_demands(grant.maximum_copy_bytes).unwrap(),d);
            let step=closed::<String>(&mut cursor,work,grant).result.unwrap();assert!(step.progress().fits(grant));born+=step.progress().retained_capacity_bytes;released+=step.progress().released_bytes;
            if matches!(step,RetainedCloneStep::Complete(_)){break;}
        }
        assert!(cursor.terminal_is_empty());assert_eq!(released,original.0+born);assert_eq!(original.1,0);
        assert_eq!(observe_retirement_allocations(||drop(cursor)).1,(0,0));
    }}
}

fn clone_through_source<T:RetainedClone+Clone+std::fmt::Debug+PartialEq+Send+Sync+crate::retirement::RetireOwned>(original:T){
    let expected=original.clone();let source=RetainedCloneSource::from_owner(original);let mut cursor=T::retained_clone_cursor();
    let wide=RetainedCloneGrant{maximum_items:64,maximum_copy_bytes:1<<16,maximum_capacity_bytes:1<<16,maximum_release_bytes:1<<16,maximum_depth:64};
    let mut output=None;
    for _ in 0..4096{if let RetainedCloneStep::Complete(_)=effect::advance(&mut cursor,source.borrow(),1<<16,CPU,wide).result.unwrap(){output= take_value(&mut cursor);break;}}
    assert_eq!(output.unwrap(),expected);
    effect::begin_close(&mut cursor);
    for _ in 0..4096{if cursor.terminal_is_empty(){break;}let funded=RetainedCloneGrant{maximum_capacity_bytes:cursor.next_close_capacity_byte_demand(1<<16).unwrap(),maximum_release_bytes:cursor.next_close_release_byte_demand().unwrap(),..wide};effect::close_selected(&mut cursor,1<<16,funded).result.unwrap();}
    assert!(cursor.terminal_is_empty());
}

#[test]
fn retained_optional_clone_matches_the_clone_oracle_for_none_and_some(){
    clone_through_source(Option::<String>::None);
    clone_through_source(Some(String::from("retained")));
    clone_through_source(Option::<u32>::None);
    clone_through_source(Some(7u32));
    clone_through_source(Some(Option::<String>::None));
}

fn cancel_clone_after<T:RetainedClone+Clone+Send+Sync+crate::retirement::RetireOwned>(original:T,turns:usize){
    let source=RetainedCloneSource::from_owner(original);let mut cursor=T::retained_clone_cursor();
    let wide=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:1<<16,maximum_capacity_bytes:1<<16,maximum_release_bytes:1<<16,maximum_depth:64};
    let mut output=None;
    for _ in 0..turns{if let RetainedCloneStep::Complete(_)=effect::advance(&mut cursor,source.borrow(),1<<16,CPU,wide).result.unwrap(){output= take_value(&mut cursor);break;}}
    effect::begin_close(&mut cursor);
    for _ in 0..1024{if cursor.terminal_is_empty(){break}let funded=RetainedCloneGrant{maximum_capacity_bytes:cursor.next_close_capacity_byte_demand(1<<16).unwrap(),maximum_release_bytes:cursor.next_close_release_byte_demand().unwrap(),..wide};effect::close_selected(&mut cursor,1<<16,funded).result.unwrap();}
    assert!(cursor.terminal_is_empty());
    if let Some(output)=output{let mut owner=crate::retirement::controlled::ControlledRetirement::new(output).map_err(|(error,_)|error).unwrap();for _ in 0..1024{if owner.terminal_is_empty(){break}let grant=RetainedCloneGrant{maximum_capacity_bytes:owner.next_capacity_byte_demand(1<<16).unwrap(),maximum_release_bytes:owner.next_release_byte_demand().unwrap(),maximum_depth:owner.next_depth_demand().unwrap(),..wide};stepped(grant,|effect|owner.step(1<<16,effect));}assert!(owner.terminal_is_empty());}
}

#[test]
fn retained_array_clone_matches_the_clone_oracle_for_scalar_and_owned_elements(){
    clone_through_source([1u32,2,3]);
    clone_through_source([[1.5f64,2.5],[3.5,4.5]]);
    clone_through_source([String::from("alpha"),String::from("γ中🐚")]);
    clone_through_source([vec![1u8,2,3],Vec::new()]);
    clone_through_source([Some(String::from("x")),None]);
    clone_through_source([[String::from("a"),String::new()],[String::from("c"),String::from("d")]]);
    clone_through_source(<[String;0]>::default());
}

#[test]
fn retained_array_clone_cancels_at_every_turn_without_abandoning_element_owners(){
    for turns in 0..48{
        cancel_clone_after([String::from("alpha"),String::from("beta"),String::from("gamma")],turns);
        cancel_clone_after([vec![1u8,2,3],vec![4u8],Vec::new()],turns);
        cancel_clone_after([7u32,8,9],turns);
    }
}

#[test]
fn original_source_typed_payload_close_preserves_raw_atoms_and_selected_child_currency(){
    let law:serde_json::Value=serde_json::from_str(crate::text("🧬️retained-clone/🔗️source/🧪️tests/🧫️fixtures/🎟️header.json")).unwrap();let payload=&law["typedPayload"];
    for body in payload["closeBodies"].as_array().unwrap(){let body=body.as_u64().unwrap()as usize;
        let(values,heap)=observe_retirement_allocations(||{let values:Vec<[f32;3]>=payload["values"].as_array().unwrap().iter().map(|row|serde_json::from_value(row.clone()).unwrap()).collect();values});let original=values.as_ptr();
        let(source,receipt)=admitted(values,(),effect::STOCK).map_err(|(e,_,_)|e).unwrap();
        assert_eq!(source.borrow().get().as_ptr(),original);let(mut born,mut freed,mut turns)=(heap.0+receipt.retained_capacity_bytes,heap.1+receipt.released_bytes,0);let leading=payload["bodyZeroLeadingFrames"].as_array().unwrap();let(raw_array,raw_element)=(payload["rawArrayBytes"].as_u64().unwrap()as usize,payload["rawElementBytes"].as_u64().unwrap()as usize);let(mut owner,heap)=observe_retirement_allocations(||ControlledRetirement::new(source).ok().unwrap());assert_eq!(heap,(0,0));assert_eq!(owner.original().unwrap().borrow().get().as_ptr(),original);
        while !owner.terminal_is_empty(){assert!(turns<10000);let(demand,heap)=observe_retirement_allocations(||owner.selected_demand(body).unwrap());assert_eq!(heap,(0,0));
            if payload["capacityFramesCarryCopy"].as_bool().unwrap()&&demand.capacity_bytes>0{assert!(demand.copy_bytes>0,"capacity frames quote their copy in the same event body={body} turn={turns} demand={demand:?}");}
            if !payload["isolatedAtomQuotes"].as_bool().unwrap(){assert!(demand.copy_bytes!=raw_array&&demand.copy_bytes!=raw_element,"no isolated raw atom quote body={body} turn={turns} demand={demand:?}");}
            if body==0{if let Some(frame)=leading.get(turns){assert_eq!((demand.copy_bytes,demand.capacity_bytes),(frame["copyBytes"].as_u64().unwrap()as usize,frame["capacityBytes"].as_u64().unwrap()as usize),"leading close frame turn={turns}");}}
            let grant=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:demand.copy_bytes,maximum_capacity_bytes:demand.capacity_bytes,maximum_release_bytes:demand.release_bytes,maximum_depth:demand.depth};assert_eq!(owner.selected_demand(grant.maximum_copy_bytes).unwrap(),demand);
            for denied in [Some(RetainedCloneGrant{maximum_items:0,..grant}),Some(RetainedCloneGrant{maximum_depth:demand.depth.saturating_sub(1),..grant}),(demand.capacity_bytes>0).then_some(RetainedCloneGrant{maximum_capacity_bytes:demand.capacity_bytes.saturating_sub(1),..grant}),(demand.release_bytes>0).then_some(RetainedCloneGrant{maximum_release_bytes:demand.release_bytes.saturating_sub(1),..grant}),(demand.copy_bytes>0).then_some(RetainedCloneGrant{maximum_copy_bytes:demand.copy_bytes.saturating_sub(1),..grant})].into_iter().flatten(){if semio_framework_value::retained_clone::retained_clone_grant_admits(denied,owner.selected_demand(denied.maximum_copy_bytes).unwrap()){continue}let(outcome,heap)=effect::driven(denied,|effect|owner.step(denied.maximum_copy_bytes,effect));assert_eq!(heap,(0,0));if let Ok(step)=outcome.result{assert_eq!(step.progress(),Default::default());}assert_eq!(owner.selected_demand(body).unwrap(),demand);}
            let(step,heap)=stepped(grant,|effect|owner.step(grant.maximum_copy_bytes,effect));let receipt=step.progress();assert!(receipt.fits(grant));assert_eq!(heap,(receipt.retained_capacity_bytes,receipt.released_bytes));born+=heap.0;freed+=heap.1;assert!(receipt.copied_items!=0,"original typed source close stalled under literal body={body} demand={demand:?}");turns+=1;
        }
        assert_eq!(born,freed);assert_eq!(observe_retirement_allocations(||drop(owner)).1,(0,0));
    }
}
