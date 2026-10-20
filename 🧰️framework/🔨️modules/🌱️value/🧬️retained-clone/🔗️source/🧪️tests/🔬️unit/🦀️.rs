use crate::retained_clone::FixtureSourceBirth;
use crate::retained_clone::{RetainedClone,RetainedCloneCursor,RetainedCloneGrant,RetainedCloneProgress,RetainedCloneSource,RetainedCloneStep};
use semio_framework_value_testing_allocator::observe_retirement_allocations;


#[test]
fn retained_clone_owned_source_birth_is_funded_before_original_owner_transfer(){
    let law:serde_json::Value=serde_json::from_str(crate::text("🧬️retained-clone/🔗️source/🧪️tests/🧫️fixtures/🎟️owned-birth/🔣️.json")).unwrap();
    let((mut owner,mut authority),original)=observe_retirement_allocations(||{
        let mut owner=String::with_capacity(law["ownerCapacity"].as_u64().unwrap()as usize);owner.push_str(law["owner"].as_str().unwrap());
        let mut authority=String::with_capacity(law["authorityCapacity"].as_u64().unwrap()as usize);authority.push_str(law["authority"].as_str().unwrap());(owner,authority)
    });
    let owner_pointer=owner.as_ptr();let authority_pointer=authority.as_ptr();
    let(bytes,heap)=observe_retirement_allocations(||RetainedCloneSource::<String>::owned_constructor_capacity_bytes::<String>());assert_eq!(heap,(0,0));
    let grant=RetainedCloneGrant{maximum_items:law["birth"]["items"].as_u64().unwrap()as usize,maximum_copy_bytes:RetainedCloneSource::<String>::constructor_copy_bytes(),maximum_capacity_bytes:bytes,maximum_depth:law["birth"]["depth"].as_u64().unwrap()as usize,..Default::default()};
    for refusal in law["refusals"].as_array().unwrap(){
        let short=refusal["shortBy"].as_u64().unwrap()as usize;
        let denied=match refusal["currency"].as_str().unwrap(){"items"=>RetainedCloneGrant{maximum_items:grant.maximum_items-short,..grant},"copy"=>RetainedCloneGrant{maximum_copy_bytes:grant.maximum_copy_bytes-short,..grant},"capacity"=>RetainedCloneGrant{maximum_capacity_bytes:bytes-short,..grant},"depth"=>RetainedCloneGrant{maximum_depth:grant.maximum_depth-short,..grant},_=>unreachable!()};
        let(result,heap)=observe_retirement_allocations(||RetainedCloneSource::admit_owned(owner,authority,denied));let(error,returned,returned_authority)=result.err().unwrap();owner=returned;authority=returned_authority;
        assert_eq!(heap,(0,0));assert_eq!(format!("{:?}",error.kind),refusal["kind"].as_str().unwrap());assert_eq!(owner.as_ptr(),owner_pointer);assert_eq!(authority.as_ptr(),authority_pointer);
    }
    let(result,heap)=observe_retirement_allocations(||RetainedCloneSource::admit_owned(owner,authority,grant));let(mut source,receipt)=result.map_err(|(error,_,_)|error).unwrap();assert_eq!(heap,(bytes,0));assert_eq!(receipt.retained_capacity_bytes,bytes);assert_eq!(receipt.copied_items,1);assert_eq!(receipt.copied_bytes,RetainedCloneSource::<String>::constructor_copy_bytes());assert_eq!(receipt.released_bytes,law["birth"]["releaseBytes"].as_u64().unwrap()as usize);assert_eq!(source.borrow().get().as_ptr(),owner_pointer);
    let(mut born,mut released)=(bytes,0usize);
    for _ in 0..law["maximumTurns"].as_u64().unwrap(){
        if source.terminal_is_empty(){break;}
        let d=source.next_close_demand(4096).unwrap();let grant=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:d.copy_bytes,maximum_capacity_bytes:d.capacity_bytes,maximum_release_bytes:d.release_bytes,maximum_depth:d.depth};assert_eq!(source.next_close_demand(grant.maximum_copy_bytes).unwrap(),d);
        let(step,heap)=observe_retirement_allocations(||source.close_step(grant).unwrap());assert!(step.progress().fits(grant));assert_eq!(heap,(step.progress().retained_capacity_bytes,step.progress().released_bytes));born+=heap.0;released+=heap.1;
    }
    assert!(source.terminal_is_empty());assert_eq!(released,original.0+born);assert_eq!(observe_retirement_allocations(||drop(source)).1,(0,0));
}



#[test]
fn retained_clone_owned_source_shared_alias_keeps_original_identity_in_every_close_order(){
 use crate::{retirement::{RetireOwned,controlled::ControlledRetirement},shared_issuance::{SealedShared,SharedIssuer}};
 fn drain<T:RetireOwned>(value:T)->(usize,usize){let mut owner=ControlledRetirement::new(value).unwrap_or_else(|_|panic!("shared original typed closure"));let(mut born,mut released)=(0,0);for _ in 0..10000{if owner.terminal_is_empty(){break;}let d=owner.selected_demand(4096).unwrap();let grant=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:d.copy_bytes,maximum_capacity_bytes:d.capacity_bytes,maximum_release_bytes:d.release_bytes,maximum_depth:d.depth};assert_eq!(owner.selected_demand(grant.maximum_copy_bytes).unwrap(),d);let(step,heap)=observe_retirement_allocations(||owner.step(grant).unwrap());assert!(step.progress().fits(grant));assert_eq!(heap,(step.progress().retained_capacity_bytes,step.progress().released_bytes));born+=heap.0;released+=heap.1;}assert!(owner.terminal_is_empty());assert_eq!(observe_retirement_allocations(||drop(owner)).1,(0,0));(born,released)}
 let law:serde_json::Value=serde_json::from_str(crate::text("🧬️retained-clone/🔗️issuance-clone/🧪️tests/🧫️fixtures/🔣️.json")).unwrap();
 for text in law["texts"].as_array().unwrap(){for order in law["closeOrders"].as_array().unwrap(){for stop in std::iter::once(None).chain(law["interruptAfter"].as_array().unwrap().iter().map(|x|Some(x.as_u64().unwrap()as usize))){
  let(value,original)=observe_retirement_allocations(||text.as_str().unwrap().to_owned());let pointer=value.as_ptr();let grant=RetainedCloneGrant{maximum_items:1,maximum_capacity_bytes:SealedShared::<String>::birth_bytes(),maximum_depth:1,..Default::default()};let((value,receipt),heap)=observe_retirement_allocations(||SealedShared::admit(value,SharedIssuer::owned(),grant).unwrap_or_else(|_|panic!("original shared admission")));assert_eq!(heap,(receipt.retained_capacity_bytes,0));let(mut born,mut released)=heap;let identity=value.identity();
  let grant=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:RetainedCloneSource::<SealedShared<String>>::constructor_copy_bytes(),maximum_capacity_bytes:RetainedCloneSource::<SealedShared<String>>::owned_constructor_capacity_bytes::<()>(),maximum_depth:1,..Default::default()};let((source,receipt),heap)=observe_retirement_allocations(||RetainedCloneSource::admit_owned(value,(),grant).unwrap_or_else(|_|panic!("original alias source admission")));assert_eq!(heap,(receipt.retained_capacity_bytes,0));born+=heap.0;
  let mut cursor=SealedShared::<String>::retained_clone_cursor();let mut output=None;for turn in 0..3{if stop.is_some_and(|stop|turn>=stop){break;}let demand=cursor.advance_demands(source.borrow(),0).unwrap();let grant=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:demand.copy_bytes,maximum_capacity_bytes:demand.capacity_bytes,maximum_release_bytes:demand.release_bytes,maximum_depth:demand.depth};for denied in [RetainedCloneGrant{maximum_items:0,..grant},RetainedCloneGrant{maximum_depth:0,..grant}]{let(step,heap)=observe_retirement_allocations(||cursor.advance(source.borrow(), 0, denied).unwrap());assert_eq!(heap,(0,0));assert_eq!(step.progress(),Default::default());assert_eq!(cursor.advance_demands(source.borrow(),0).unwrap(),demand);}let(step,heap)=observe_retirement_allocations(||cursor.advance(source.borrow(), 0, grant).unwrap());assert!(step.progress().fits(grant));assert_eq!(heap,(0,0));if matches!(step,RetainedCloneStep::Complete(_)){if stop.is_none(){output=cursor.take();}break;}}
  if stop.is_none(){let output=output.as_ref().unwrap();assert_eq!(output.identity(),identity);assert_eq!(output.get().as_ptr(),pointer);assert_eq!(output.get(),text.as_str().unwrap());}
  if order=="outputFirst"{if let Some(output)=output.take(){let heap=drain(output);born+=heap.0;released+=heap.1;}}
  let heap=drain(source);born+=heap.0;released+=heap.1;if let Some(output)=output.as_ref(){assert_eq!(output.identity(),identity);assert_eq!(output.get().as_ptr(),pointer);assert_eq!(output.get(),text.as_str().unwrap());}
  let heap=drain(cursor);born+=heap.0;released+=heap.1;if let Some(output)=output{assert_eq!(output.get().as_ptr(),pointer);let heap=drain(output);born+=heap.0;released+=heap.1;}
  assert_eq!(original.0-original.1+born,released);
 }}}
}

#[test]
fn retained_clone_owned_source_controlled_facade_retains_original_projection_in_both_close_orders(){
 use crate::retirement::{RetireOwned,controlled::ControlledRetirement};
 fn drain<T:RetireOwned>(mut owner:ControlledRetirement<T>)->(usize,usize){let(mut born,mut released)=(0,0);for _ in 0..10000{if owner.terminal_is_empty(){break;}let d=owner.selected_demand(4096).unwrap();let grant=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:d.copy_bytes,maximum_capacity_bytes:d.capacity_bytes,maximum_release_bytes:d.release_bytes,maximum_depth:d.depth};assert_eq!(owner.selected_demand(grant.maximum_copy_bytes).unwrap(),d);for denied in [Some(RetainedCloneGrant{maximum_items:0,..grant}),grant.maximum_capacity_bytes.checked_sub(1).map(|v|RetainedCloneGrant{maximum_capacity_bytes:v,..grant}),grant.maximum_release_bytes.checked_sub(1).map(|v|RetainedCloneGrant{maximum_release_bytes:v,..grant})].into_iter().flatten(){let(step,heap)=observe_retirement_allocations(||owner.step(denied).unwrap());assert_eq!(step.progress(),Default::default());assert_eq!(heap,(0,0));}let(step,heap)=observe_retirement_allocations(||owner.step(grant).unwrap());assert!(step.progress().fits(grant));assert_eq!(heap,(step.progress().retained_capacity_bytes,step.progress().released_bytes));born+=heap.0;released+=heap.1;}assert!(owner.terminal_is_empty());assert_eq!(observe_retirement_allocations(||drop(owner)).1,(0,0));(born,released)}
 let law:serde_json::Value=serde_json::from_str(crate::text("🧬️retained-clone/🔗️source/🧪️tests/🧫️fixtures/🎟️owned-birth/🔣️.json")).unwrap();
 for order in law["closeOrders"].as_array().unwrap(){let projection_first=order=="projectionFirst";let((value,authority),original)=observe_retirement_allocations(||{let mut value=String::with_capacity(law["ownerCapacity"].as_u64().unwrap()as usize);value.push_str(law["owner"].as_str().unwrap());let mut authority=String::with_capacity(law["authorityCapacity"].as_u64().unwrap()as usize);authority.push_str(law["authority"].as_str().unwrap());(value,authority)});let pointer=value.as_ptr();let grant=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:RetainedCloneSource::<String>::constructor_copy_bytes(),maximum_capacity_bytes:RetainedCloneSource::<String>::owned_constructor_capacity_bytes::<String>(),maximum_depth:1,..Default::default()};let((source,receipt),heap)=observe_retirement_allocations(||RetainedCloneSource::admit_owned(value,authority,grant).map_err(|(error,_,_)|error).unwrap());assert_eq!(heap,(receipt.retained_capacity_bytes,0));let(mut born,mut released)=heap;
  let grant=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:RetainedCloneSource::<String>::constructor_copy_bytes(),maximum_depth:1,..Default::default()};let((projection,receipt),heap)=observe_retirement_allocations(||source.project_owned(1,|value|value.as_str(),grant).unwrap());assert_eq!(heap,(0,0));assert!(receipt.fits(grant));assert_eq!(projection.borrow().unwrap().get().as_ptr(),pointer);
  let(source,heap)=observe_retirement_allocations(||ControlledRetirement::new(source).map_err(|(error,_)|error).unwrap());assert_eq!(heap,(0,0));
  fn drain_projection<T:?Sized+Sync>(mut owner:crate::retained_clone::RetainedOwnedProjection<T>)->(usize,usize){let(mut born,mut released)=(0,0);for _ in 0..10000{if owner.terminal_is_empty(){break;}let d=owner.close_demand(4096).unwrap();let grant=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:d.copy_bytes,maximum_capacity_bytes:d.capacity_bytes,maximum_release_bytes:d.release_bytes,maximum_depth:d.depth};assert_eq!(owner.close_demand(grant.maximum_copy_bytes).unwrap(),d);let(step,heap)=observe_retirement_allocations(||owner.close_step(grant).unwrap());assert!(step.progress().fits(grant));assert_eq!(heap,(step.progress().retained_capacity_bytes,step.progress().released_bytes));born+=heap.0;released+=heap.1;}assert!(owner.terminal_is_empty());assert_eq!(observe_retirement_allocations(||drop(owner)).1,(0,0));(born,released)}if projection_first{let heap=drain_projection(projection);born+=heap.0;released+=heap.1;let heap=drain(source);born+=heap.0;released+=heap.1;}else{let heap=drain(source);born+=heap.0;released+=heap.1;assert_eq!(projection.borrow().unwrap().get().as_ptr(),pointer);assert_eq!(projection.borrow().unwrap().get(),law["owner"].as_str().unwrap());let heap=drain_projection(projection);born+=heap.0;released+=heap.1;}assert_eq!(released,original.0-original.1+born);
 }
}

#[test]
fn retained_clone_owned_source_closes_its_actual_shared_authority_lease(){
    use crate::{retirement::controlled::ControlledRetirement,shared_issuance::{SealedShared,SharedIssuer}};
    let law:serde_json::Value=serde_json::from_str(crate::text("🧬️retained-clone/🔗️source/🧪️tests/🧫️fixtures/🎟️owned-birth/🔣️.json")).unwrap();
    let((owner,authority),original)=observe_retirement_allocations(||{let mut owner=String::with_capacity(law["ownerCapacity"].as_u64().unwrap()as usize);owner.push_str(law["owner"].as_str().unwrap());let mut authority=String::with_capacity(law["authorityCapacity"].as_u64().unwrap()as usize);authority.push_str(law["authority"].as_str().unwrap());(owner,authority)});
    let pointer=authority.as_ptr();let grant=RetainedCloneGrant{maximum_items:1,maximum_capacity_bytes:SealedShared::<String>::birth_bytes(),maximum_depth:1,..Default::default()};
    let(result,heap)=observe_retirement_allocations(||SealedShared::admit(authority,SharedIssuer::owned(),grant));let(authority,progress)=result.map_err(|(error,_)|error).unwrap();assert_eq!(heap,(progress.retained_capacity_bytes,0));let(mut born,mut released)=heap;
    let grant=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:size_of::<SealedShared<String>>(),maximum_depth:1,..Default::default()};let(result,heap)=observe_retirement_allocations(||authority.try_duplicate(grant));let(lease,progress)=result.unwrap();assert_eq!(heap,(0,0));assert!(progress.fits(grant));
    let demand=RetainedCloneSource::<String>::owned_constructor_demand::<SealedShared<String>>();let grant=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:RetainedCloneSource::<String>::constructor_copy_bytes(),maximum_capacity_bytes:demand.capacity_bytes,maximum_depth:demand.depth,..Default::default()};
    let(result,heap)=observe_retirement_allocations(||RetainedCloneSource::admit_owned(owner,lease,grant));let(mut source,receipt)=result.map_err(|(error,_,_)|error).unwrap();assert_eq!(heap,(receipt.retained_capacity_bytes,0));born+=heap.0;
    for _ in 0..law["maximumTurns"].as_u64().unwrap(){if source.terminal_is_empty(){break;}let d=source.next_close_demand(4096).unwrap();let grant=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:d.copy_bytes,maximum_capacity_bytes:d.capacity_bytes,maximum_release_bytes:d.release_bytes,maximum_depth:d.depth};assert_eq!(source.next_close_demand(grant.maximum_copy_bytes).unwrap(),d);let(step,heap)=observe_retirement_allocations(||source.close_step(grant).unwrap());assert!(step.progress().fits(grant));assert_eq!(heap,(step.progress().retained_capacity_bytes,step.progress().released_bytes));born+=heap.0;released+=heap.1;}
    assert!(source.terminal_is_empty());assert_eq!(authority.get().as_ptr(),pointer);assert_eq!(authority.get().as_str(),law["authority"].as_str().unwrap());assert_eq!(observe_retirement_allocations(||drop(source)).1,(0,0));let mut authority=ControlledRetirement::new(authority).map_err(|(error,_)|error).unwrap();
    for _ in 0..law["maximumTurns"].as_u64().unwrap(){if authority.terminal_is_empty(){break;}let d=authority.selected_demand(4096).unwrap();let grant=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:d.copy_bytes,maximum_capacity_bytes:d.capacity_bytes,maximum_release_bytes:d.release_bytes,maximum_depth:d.depth};assert_eq!(authority.selected_demand(grant.maximum_copy_bytes).unwrap(),d);let(step,heap)=observe_retirement_allocations(||authority.step(grant).unwrap());assert!(step.progress().fits(grant));assert_eq!(heap,(step.progress().retained_capacity_bytes,step.progress().released_bytes));born+=heap.0;released+=heap.1;}
    assert!(authority.terminal_is_empty());assert_eq!(released,original.0+born);assert_eq!(observe_retirement_allocations(||drop(authority)).1,(0,0));
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
        let grant=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:RetainedCloneSource::<String>::constructor_copy_bytes(),maximum_capacity_bytes:bytes,maximum_depth:1,..Default::default()};
        let mut owner=owner;let mut authority=authority;
        for short in [RetainedCloneGrant{maximum_items:0,..grant},RetainedCloneGrant{maximum_capacity_bytes:bytes-1,..grant}]{
            let (refused,heap)=observe_retirement_allocations(||RetainedCloneSource::admit_owned(owner,authority,short));
            let (_,returned,returned_authority)=refused.err().unwrap();owner=returned;authority=returned_authority;
            assert_eq!(heap,(0,0));assert_eq!(owner.as_ptr(),pointer);assert_eq!(authority.as_ptr(),authority_pointer);
        }
        let (admitted,heap)=observe_retirement_allocations(||RetainedCloneSource::admit_owned(owner,authority,grant));
        let (mut source,receipt)=admitted.map_err(|(error,_,_)|error).unwrap();
        assert_eq!(heap,(receipt.retained_capacity_bytes,0));assert_eq!(heap.0,bytes);
        let (mut born,mut released)=(heap.0,0usize);
        assert_eq!(source.borrow().get().as_ptr(),pointer);
        assert_eq!(serde_json::to_value(source.borrow().get()).unwrap(),row["owner"]);
        let mut cursor=String::retained_clone_cursor();
        let turn=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:work,maximum_capacity_bytes:4096,maximum_release_bytes:0,maximum_depth:64};
        let (step,heap)=observe_retirement_allocations(||cursor.advance(source.borrow(), work, turn).unwrap());
        assert!(step.progress().fits(turn));assert_eq!(heap,(step.progress().retained_capacity_bytes,step.progress().released_bytes));born+=heap.0;released+=heap.1;
        cursor.begin_close();
        for _ in 0..100000{
            if source.terminal_is_empty(){break;}
            let d=source.next_close_demand(work).unwrap();
            let grant=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:d.copy_bytes,maximum_capacity_bytes:d.capacity_bytes,maximum_release_bytes:d.release_bytes,maximum_depth:d.depth};assert_eq!(source.next_close_demand(grant.maximum_copy_bytes).unwrap(),d);
            let (zero,heap)=observe_retirement_allocations(||source.close_step(RetainedCloneGrant{maximum_items:0,..grant}).unwrap());assert_eq!(zero.progress(),RetainedCloneProgress::default());assert_eq!(heap,(0,0));
            let (step,heap)=observe_retirement_allocations(||source.close_step(grant).unwrap());assert!(step.progress().fits(grant));assert_eq!(heap,(step.progress().retained_capacity_bytes,step.progress().released_bytes));born+=heap.0;released+=heap.1;
        }
        assert!(source.terminal_is_empty());assert_eq!(observe_retirement_allocations(||drop(source)).1,(0,0));
        for _ in 0..100000{
            let d=cursor.close_demands(work).unwrap();
            let grant=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:d.copy_bytes,maximum_capacity_bytes:d.capacity_bytes,maximum_release_bytes:d.release_bytes,maximum_depth:d.depth};assert_eq!(cursor.close_demands(grant.maximum_copy_bytes).unwrap(),d);
            let (step,heap)=observe_retirement_allocations(||cursor.close_step(grant).unwrap());assert!(step.progress().fits(grant));assert_eq!(heap,(step.progress().retained_capacity_bytes,step.progress().released_bytes));born+=heap.0;released+=heap.1;
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
    for _ in 0..4096{if let RetainedCloneStep::Complete(_)=cursor.advance(source.borrow(), 1 << 16, wide).unwrap(){output=cursor.take();break;}}
    assert_eq!(output.unwrap(),expected);
    cursor.begin_close();
    for _ in 0..4096{if cursor.terminal_is_empty(){break;}cursor.close_step(RetainedCloneGrant{maximum_capacity_bytes:cursor.next_close_capacity_byte_demand(1<<16).unwrap(),maximum_release_bytes:cursor.next_close_release_byte_demand().unwrap(),..wide}).unwrap();}
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
    for _ in 0..turns{if let RetainedCloneStep::Complete(_)=cursor.advance(source.borrow(), 1 << 16, wide).unwrap(){output=cursor.take();break;}}
    cursor.begin_close();
    for _ in 0..1024{if cursor.terminal_is_empty(){break}cursor.close_step(RetainedCloneGrant{maximum_capacity_bytes:cursor.next_close_capacity_byte_demand(1<<16).unwrap(),maximum_release_bytes:cursor.next_close_release_byte_demand().unwrap(),..wide}).unwrap();}
    assert!(cursor.terminal_is_empty());
    if let Some(output)=output{let mut owner=crate::retirement::controlled::ControlledRetirement::new(output).map_err(|(error,_)|error).unwrap();for _ in 0..1024{if owner.terminal_is_empty(){break}owner.step(RetainedCloneGrant{maximum_capacity_bytes:owner.next_capacity_byte_demand(1<<16).unwrap(),maximum_release_bytes:owner.next_release_byte_demand().unwrap(),maximum_depth:owner.next_depth_demand().unwrap(),..wide}).unwrap();}assert!(owner.terminal_is_empty());}
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
    use crate::retirement::controlled::ControlledRetirement;
    let law:serde_json::Value=serde_json::from_str(crate::text("🧬️retained-clone/🔗️source/🧪️tests/🧫️fixtures/🎟️header.json")).unwrap();let payload=&law["typedPayload"];
    for body in payload["closeBodies"].as_array().unwrap(){let body=body.as_u64().unwrap()as usize;
        let((source,original),heap)=observe_retirement_allocations(||{let values:Vec<[f32;3]>=payload["values"].as_array().unwrap().iter().map(|row|serde_json::from_value(row.clone()).unwrap()).collect();let original=values.as_ptr();let demand=RetainedCloneSource::<Vec<[f32;3]>>::owned_constructor_demand::<()>();let grant=RetainedCloneGrant{maximum_items:1,maximum_capacity_bytes:demand.capacity_bytes,maximum_depth:demand.depth,..Default::default()};let(source,receipt)=RetainedCloneSource::admit_owned(values,(),grant).map_err(|(e,_,_)|e).unwrap();assert_eq!(receipt.copied_bytes,0);(source,original)});
        assert_eq!(source.borrow().get().as_ptr(),original);let(mut born,mut freed,mut turns,mut spills,mut arrays,mut leaves)=(heap.0,heap.1,0,0,0,0);let(mut owner,heap)=observe_retirement_allocations(||ControlledRetirement::new(source).ok().unwrap());assert_eq!(heap,(0,0));assert_eq!(owner.original().unwrap().borrow().get().as_ptr(),original);
        while !owner.terminal_is_empty(){assert!(turns<10000);let(demand,heap)=observe_retirement_allocations(||owner.selected_demand(body).unwrap());assert_eq!(heap,(0,0));let raw=owner.next_copy_byte_demand().unwrap();if raw==payload["rawArrayBytes"].as_u64().unwrap()as usize{arrays+=1;}if raw==payload["rawElementBytes"].as_u64().unwrap()as usize{leaves+=1;}if raw>body&&demand.capacity_bytes>0{assert_eq!(demand.copy_bytes,payload["selectedSpillCopyBytes"].as_u64().unwrap()as usize,"[DEBUG] original typed source must quote the same child event as capacity body={body} raw={raw} selected={demand:?}");spills+=1;}
            let grant=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:demand.copy_bytes,maximum_capacity_bytes:demand.capacity_bytes,maximum_release_bytes:demand.release_bytes,maximum_depth:demand.depth};assert_eq!(owner.selected_demand(grant.maximum_copy_bytes).unwrap(),demand);
            for denied in [Some(RetainedCloneGrant{maximum_items:0,..grant}),Some(RetainedCloneGrant{maximum_depth:demand.depth.saturating_sub(1),..grant}),(demand.capacity_bytes>0).then_some(RetainedCloneGrant{maximum_capacity_bytes:demand.capacity_bytes.saturating_sub(1),..grant}),(demand.release_bytes>0).then_some(RetainedCloneGrant{maximum_release_bytes:demand.release_bytes.saturating_sub(1),..grant}),(demand.copy_bytes>0).then_some(RetainedCloneGrant{maximum_copy_bytes:demand.copy_bytes.saturating_sub(1),..grant})].into_iter().flatten(){let(result,heap)=observe_retirement_allocations(||owner.step(denied));assert_eq!(heap,(0,0));if let Ok(step)=result{assert_eq!(step.progress(),Default::default());}assert_eq!(owner.selected_demand(body).unwrap(),demand);}
            let(step,heap)=observe_retirement_allocations(||owner.step(grant).unwrap());let receipt=step.progress();assert!(receipt.fits(grant));assert_eq!(heap,(receipt.retained_capacity_bytes,receipt.released_bytes));born+=heap.0;freed+=heap.1;assert!(receipt.copied_items!=0,"original typed source close stalled under literal body={body} demand={demand:?}");turns+=1;
        }
        assert!(arrays>0,"original source raw array12 remains visible");if body<payload["rawElementBytes"].as_u64().unwrap()as usize{assert!(spills>0);assert!(leaves>0,"original source raw leaf4 remains visible");}assert_eq!(born,freed);assert_eq!(observe_retirement_allocations(||drop(owner)).1,(0,0));
    }
}
