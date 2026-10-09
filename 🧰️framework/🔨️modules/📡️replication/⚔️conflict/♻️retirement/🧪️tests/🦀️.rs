//! 🧪️ Every original causal/conflict allocation is physically measured under independent close grants.
use super::*;
use crate::ConflictKind;
use std::mem::size_of;
use semio_framework_value::retained_clone::RetainedCloneProgress;
use crate::ids::{ActorId,ArtifactId,HybridLogicalTimestamp,MutationId,SchemaId};
use semio_framework_diagnostic::Severity;
use semio_framework_trace::observe_heap_allocations_on_this_thread;
fn fixture()->serde_json::Value {serde_json::from_str(include_str!("../🧫️fixtures/🔣️.json")).unwrap()}
fn string(value:&str,capacity:usize)->String {let mut result=String::with_capacity(capacity.max(value.len()));result.push_str(value);result}
fn strings(row:&serde_json::Value,key:&str,capacity:&str)->Vec<String> {let mut values=Vec::with_capacity(row[capacity].as_u64().unwrap()as usize);for value in row[key].as_array().unwrap(){values.push(string(value.as_str().unwrap(),23));}values}
fn bytes(row:&serde_json::Value,key:&str,capacity:&str)->Vec<u8> {let mut values=Vec::with_capacity(row[capacity].as_u64().unwrap()as usize);for value in row[key].as_array().unwrap(){values.push(value.as_u64().unwrap()as u8);}values}
fn envelope(row:&serde_json::Value)->MutationEnvelope {
    let text=row["text"].as_str().unwrap().repeat(row["repeat"].as_u64().unwrap()as usize);let fresh=||string(&text,row["stringCapacity"].as_u64().unwrap()as usize);let optional=row["optional"].as_bool().unwrap();
    let mut dependencies=Vec::with_capacity(row["dependencyCapacity"].as_u64().unwrap()as usize);for value in row["dependencies"].as_array().unwrap(){dependencies.push(MutationId(string(value.as_str().unwrap(),23)));}
    MutationEnvelope {mutation_id:MutationId(fresh()),document_id:ArtifactId(fresh()),actor:ActorId(fresh().into()),dependencies,observed:optional.then(||MutationId(fresh())),target:strings(row,"targets","targetCapacity"),diff:crate::ArtifactDiff {schema:SchemaId(fresh()),payload:bytes(row,"forward","forwardCapacity")},inverse:crate::InverseMutation {schema:SchemaId(fresh()),payload:bytes(row,"inverse","inverseCapacity")},timestamp:HybridLogicalTimestamp::new(7,11),transaction:optional.then(||crate::TransactionRef {id:fresh(),tool:fresh()}),verb:optional.then(fresh),line:optional.then(fresh)}
}
fn conflict(row:&serde_json::Value,law:&serde_json::Value)->Conflict {
    let kind=if row["kind"]=="quarantined" {let mut values=Vec::with_capacity(row["rowCapacity"].as_u64().unwrap()as usize);for index in row["rows"].as_array().unwrap(){values.push(envelope(&law["envelopes"][index.as_u64().unwrap()as usize]));}ConflictKind::Quarantined {envelopes:values}}else{ConflictKind::Degraded {edit_ids:strings(row,"identities","identityCapacity")}};
    let mut actors=Vec::with_capacity(row["actorCapacity"].as_u64().unwrap()as usize);for value in row["actors"].as_array().unwrap(){actors.push(ActorId(string(value.as_str().unwrap(),29).into()));}
    let mut messages=Vec::with_capacity(row["messageCapacity"].as_u64().unwrap()as usize);for value in row["messages"].as_array().unwrap(){let mut target=Vec::with_capacity(11);target.push(string("",19));target.push(string("field:🐚",37));messages.push(crate::MutationMessage {level:Severity::Fatal,code:string("mutation.invariant",41).into(),message:string(value.as_str().unwrap(),71),target,op_index:Some(7)});}
    Conflict {id:crate::ConflictId(string(row["id"].as_str().unwrap(),row["idCapacity"].as_u64().unwrap()as usize)),kind,status:crate::ConflictStatus::Open,messages,actors,timestamp:HybridLogicalTimestamp::new(7,11)}
}
fn verify(mut cursor:ProtocolConflictRetirement,held:usize,copy:usize,release:usize,maximum_turns:usize) {
    let(mut returned,mut born,mut stalled)=(0,0,0);
    let(step,heap)=observe_heap_allocations_on_this_thread(||cursor.close_step(RetainedCloneGrant::default()).unwrap());assert_eq!(step.progress(),RetainedCloneProgress::default());assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));
    let law=fixture();let maximum_turns=held.checked_mul(law["maximumTurnFactor"].as_u64().unwrap()as usize).unwrap().checked_add(law["maximumFixedTurns"].as_u64().unwrap()as usize).unwrap();
    for turn in 0..maximum_turns {
        let ((body,capacity,freed,depth),heap)=observe_heap_allocations_on_this_thread(||{let body=copy.max(1);(body,cursor.next_capacity_byte_demand(body).unwrap(),cursor.next_release_byte_demand().unwrap(),cursor.next_depth_demand().unwrap())});assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));
        let grant=RetainedCloneGrant {maximum_items:1,maximum_copy_bytes:body,maximum_capacity_bytes:capacity,maximum_release_bytes:release.max(freed),maximum_depth:depth};
        let below=[RetainedCloneGrant {maximum_items:0,..grant},RetainedCloneGrant {maximum_capacity_bytes:capacity.saturating_sub(1),..grant},RetainedCloneGrant {maximum_release_bytes:freed.saturating_sub(1),..grant},RetainedCloneGrant {maximum_depth:depth.saturating_sub(1),..grant}];
        for (index,below) in below.into_iter().enumerate() {
            if cursor.terminal_is_empty(){break;}if (index==1&&capacity==0)||(index==2&&freed==0)||(index==3&&depth==0){continue;}
            let(result,heap)=observe_heap_allocations_on_this_thread(||cursor.close_step(below));assert!(result.is_err()||result.is_ok_and(|step|step.progress()==RetainedCloneProgress::default()));assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));
        }
        let(step,heap)=observe_heap_allocations_on_this_thread(||cursor.close_step(grant).unwrap());assert_eq!((heap.requested_bytes,heap.released_bytes),(step.progress().retained_capacity_bytes,step.progress().released_bytes),"turn{turn}");assert!(step.progress().fits(grant));returned+=heap.released_bytes;born+=heap.requested_bytes;
        if step.progress()==RetainedCloneProgress::default(){stalled+=1;if stalled==64{eprintln!("[DEBUG] original Conflict stalled turn={turn} copy={} quoteCopy={} capacity={capacity} release={freed} depth={depth} actual={:?} admitted={born} physical={returned}",grant.maximum_copy_bytes,cursor.next_copy_byte_demand().unwrap(),step.progress());panic!("original Conflict fixed admitted copy stalled");}}else{stalled=0;}
        if matches!(step,RetainedCloneStep::Complete(_)){assert!(cursor.terminal_is_empty());break;}assert!(turn+1<maximum_turns,"original conflict exceeded source-extent turn bound: copy={body} capacity={capacity} release={freed} depth={depth} grant={grant:?} step={step:?} held={held} born={born} returned={returned}");
    }
    assert_eq!(returned,held+born);
    let(_,heap)=observe_heap_allocations_on_this_thread(||drop(cursor));assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));
}

#[test]
fn conflict_ownership_retirement_preserves_and_releases_every_causal_optional_owner() {
    let law=fixture();
    for row in law["envelopes"].as_array().unwrap(){for copy in law["copyGrants"].as_array().unwrap(){for release in law["releaseGrants"].as_array().unwrap(){let(owner,source)=observe_heap_allocations_on_this_thread(||envelope(row));let held=source.requested_bytes-source.released_bytes;let original=owner.mutation_id.0.as_ptr();let(cursor,heap)=observe_heap_allocations_on_this_thread(||ProtocolConflictRetirement::causal_envelope(owner));assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));let ProtocolOwner::Envelope(envelope)=&cursor.owner else{panic!("original envelope owner");};assert_eq!(envelope.original().unwrap().mutation_id.0.as_ptr(),original);verify(cursor,held,copy.as_u64().unwrap()as usize,release.as_u64().unwrap()as usize,law["maximumTurns"].as_u64().unwrap()as usize);}}}
    println!("[DEBUG] native original causal envelope fulltyped identity/actor/observed/transaction/verb/line fields, targets/dependencies/payloads include original unused capacities; undergrants0effects, actualbirth/free exact, terminal Drop0heap");
}
#[test]
fn conflict_ownership_retirement_preserves_all_quarantined_degraded_original_scaffolds() {
    let law=fixture();
    for row in law["conflicts"].as_array().unwrap(){for copy in law["copyGrants"].as_array().unwrap(){for release in law["releaseGrants"].as_array().unwrap(){let(owner,source)=observe_heap_allocations_on_this_thread(||conflict(row,&law));let held=source.requested_bytes-source.released_bytes;let original=owner.messages.as_ptr();let(cursor,heap)=observe_heap_allocations_on_this_thread(||ProtocolConflictRetirement::new(owner));assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));let ProtocolOwner::Conflict(conflict)=&cursor.owner else{panic!("original conflict owner");};assert_eq!(conflict.original().unwrap().messages.as_ptr(),original);verify(cursor,held,copy.as_u64().unwrap()as usize,release.as_u64().unwrap()as usize,law["maximumTurns"].as_u64().unwrap()as usize);}}}
    println!("[DEBUG] native quarantined/degraded/empty conflict branches retain original messages/envelopes/actors/identities and every old vector backing, independent copy/capacity/release/depth, terminal Drop0heap");
}

#[test]
fn conflict_ownership_retirement_preserves_original_shared_actor_unique_and_alias_extent(){
 use semio_framework_value::retirement::controlled::ControlledRetirement;
 let law=fixture();let policy=&law["sharedActor"];
 for row in law["envelopes"].as_array().unwrap(){for aliases in policy["aliases"].as_array().unwrap(){
  let(owner,source)=observe_heap_allocations_on_this_thread(||envelope(row));let held=source.requested_bytes.checked_sub(source.released_bytes).unwrap();let pointer=owner.actor.0.as_ptr();let text=row["text"].as_str().unwrap().repeat(row["repeat"].as_u64().unwrap()as usize);assert_eq!(serde_json::to_value(&owner.actor).unwrap(),serde_json::Value::String(text.clone()));
  let lease_grant=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:size_of::<semio_framework_value::SharedUtf8>(),maximum_depth:1,..Default::default()};
  let mut lease=if aliases.as_bool().unwrap(){let((lease,receipt),heap)=observe_heap_allocations_on_this_thread(||owner.actor.0.admit_clone(lease_grant).unwrap());assert!(receipt.fits(lease_grant));assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));Some(lease)}else{None};
  let mut cursor=ProtocolConflictRetirement::causal_envelope(owner);let mut born=0;let mut released=0;let mut turns=0;
  for _ in 0..policy["maximumTurns"].as_u64().unwrap(){if cursor.terminal_is_empty(){break;}let copy=cursor.next_copy_byte_demand().unwrap();let body=copy.max(policy["copy"].as_u64().unwrap()as usize);let capacity=cursor.next_capacity_byte_demand(body).unwrap();let release=cursor.next_release_byte_demand().unwrap();let depth=cursor.next_depth_demand().unwrap();let full=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:body,maximum_capacity_bytes:capacity,maximum_release_bytes:release,maximum_depth:depth};
   for(axis,denied)in [RetainedCloneGrant{maximum_items:0,..full},RetainedCloneGrant{maximum_copy_bytes:0,..full},RetainedCloneGrant{maximum_capacity_bytes:capacity.saturating_sub(1),..full},RetainedCloneGrant{maximum_release_bytes:release.saturating_sub(1),..full},RetainedCloneGrant{maximum_depth:depth.saturating_sub(1),..full}].into_iter().enumerate(){if (axis==1&&copy==0)||(axis==2&&capacity==0)||(axis==3&&release==0)||(axis==4&&depth==0){continue;}let(before_copy,before_capacity,before_release,before_depth)=(cursor.next_copy_byte_demand().unwrap(),cursor.next_capacity_byte_demand(body).unwrap(),cursor.next_release_byte_demand().unwrap(),cursor.next_depth_demand().unwrap());let(result,heap)=observe_heap_allocations_on_this_thread(||cursor.close_step(denied));assert!(result.is_err()||result.is_ok_and(|step|step.progress()==RetainedCloneProgress::default()));assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));assert_eq!((cursor.next_copy_byte_demand().unwrap(),cursor.next_capacity_byte_demand(body).unwrap(),cursor.next_release_byte_demand().unwrap(),cursor.next_depth_demand().unwrap()),(before_copy,before_capacity,before_release,before_depth));if let Some(lease)=lease.as_ref(){assert_eq!(lease.as_ptr(),pointer);assert_eq!(lease.as_str(),text);}}
   let(step,heap)=observe_heap_allocations_on_this_thread(||cursor.close_step(full).unwrap());assert!(step.progress().fits(full));assert_eq!((heap.requested_bytes,heap.released_bytes),(step.progress().retained_capacity_bytes,step.progress().released_bytes));born+=heap.requested_bytes;released+=heap.released_bytes;turns+=1;
  }
  assert!(cursor.terminal_is_empty());let((),heap)=observe_heap_allocations_on_this_thread(||drop(cursor));assert_eq!((heap.requested_bytes,heap.released_bytes),(0,policy["terminalDropBytes"].as_u64().unwrap()as usize));
  if let Some(original)=lease.take(){assert_eq!(original.as_ptr(),pointer);assert_eq!(original.as_str(),text);let mut original=ControlledRetirement::new(original).unwrap_or_else(|(error,_)|panic!("actual original shared alias: {error}"));for _ in 0..policy["maximumTurns"].as_u64().unwrap(){if original.terminal_is_empty(){break;}let body=original.next_copy_byte_demand().unwrap().max(policy["copy"].as_u64().unwrap()as usize);let full=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:body,maximum_capacity_bytes:original.next_capacity_byte_demand(body).unwrap(),maximum_release_bytes:original.next_release_byte_demand().unwrap(),maximum_depth:original.next_depth_demand().unwrap()};let(step,heap)=observe_heap_allocations_on_this_thread(||original.step(full).unwrap());assert!(step.progress().fits(full));assert_eq!((heap.requested_bytes,heap.released_bytes),(step.progress().retained_capacity_bytes,step.progress().released_bytes));born+=heap.requested_bytes;released+=heap.released_bytes;}assert!(original.terminal_is_empty());let((),heap)=observe_heap_allocations_on_this_thread(||drop(original));assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));}
  assert_eq!(released,held+born);eprintln!("[DEBUG] original conflict shared actor name={} aliases={aliases} turns={turns} sameUTF8pointer, independent Serde, five denials/exactHeap, born={born} released={released}, terminalDrop0",row["name"]);
 }}
}
