//! 🧪️ Every original causal/conflict allocation is physically measured under independent close grants.
use super::*;
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
    MutationEnvelope {mutation_id:MutationId(fresh()),document_id:ArtifactId(fresh()),actor:ActorId(fresh()),dependencies,observed:optional.then(||MutationId(fresh())),target:strings(row,"targets","targetCapacity"),diff:crate::ArtifactDiff {schema:SchemaId(fresh()),payload:bytes(row,"forward","forwardCapacity")},inverse:crate::InverseMutation {schema:SchemaId(fresh()),payload:bytes(row,"inverse","inverseCapacity")},timestamp:HybridLogicalTimestamp::new(7,11),transaction:optional.then(||crate::TransactionRef {id:fresh(),tool:fresh()}),verb:optional.then(fresh),line:optional.then(fresh)}
}
fn envelope_held(owner:&MutationEnvelope)->usize {
    owner.mutation_id.0.capacity()+owner.document_id.0.capacity()+owner.actor.0.capacity()+owner.diff.schema.0.capacity()+owner.inverse.schema.0.capacity()+owner.diff.payload.capacity()+owner.inverse.payload.capacity()+owner.dependencies.capacity()*size_of::<MutationId>()+owner.dependencies.iter().map(|id|id.0.capacity()).sum::<usize>()+owner.target.capacity()*size_of::<String>()+owner.target.iter().map(String::capacity).sum::<usize>()+owner.observed.as_ref().map_or(0,|id|id.0.capacity())+owner.transaction.as_ref().map_or(0,|transaction|transaction.id.capacity()+transaction.tool.capacity())+owner.verb.as_ref().map_or(0,String::capacity)+owner.line.as_ref().map_or(0,String::capacity)
}
fn conflict(row:&serde_json::Value,law:&serde_json::Value)->Conflict {
    let kind=if row["kind"]=="quarantined" {let mut values=Vec::with_capacity(row["rowCapacity"].as_u64().unwrap()as usize);for index in row["rows"].as_array().unwrap(){values.push(envelope(&law["envelopes"][index.as_u64().unwrap()as usize]));}ConflictKind::Quarantined {envelopes:values}}else{ConflictKind::Degraded {edit_ids:strings(row,"identities","identityCapacity")}};
    let mut actors=Vec::with_capacity(row["actorCapacity"].as_u64().unwrap()as usize);for value in row["actors"].as_array().unwrap(){actors.push(ActorId(string(value.as_str().unwrap(),29)));}
    let mut messages=Vec::with_capacity(row["messageCapacity"].as_u64().unwrap()as usize);for value in row["messages"].as_array().unwrap(){let mut target=Vec::with_capacity(11);target.push(string("",19));target.push(string("field:🐚",37));messages.push(crate::MutationMessage {level:Severity::Fatal,code:string("mutation.invariant",41).into(),message:string(value.as_str().unwrap(),71),target,op_index:Some(7)});}
    Conflict {id:crate::ConflictId(string(row["id"].as_str().unwrap(),row["idCapacity"].as_u64().unwrap()as usize)),kind,status:crate::ConflictStatus::Open,messages,actors,timestamp:HybridLogicalTimestamp::new(7,11)}
}
fn conflict_held(owner:&Conflict)->usize {
    owner.id.0.capacity()+owner.actors.capacity()*size_of::<ActorId>()+owner.actors.iter().map(|id|id.0.capacity()).sum::<usize>()+owner.messages.capacity()*size_of::<crate::MutationMessage>()+owner.messages.iter().map(|message|message.code.0.capacity()+message.message.capacity()+message.target.capacity()*size_of::<String>()+message.target.iter().map(String::capacity).sum::<usize>()).sum::<usize>()+match &owner.kind {ConflictKind::Quarantined {envelopes}=>envelopes.capacity()*size_of::<MutationEnvelope>()+envelopes.iter().map(envelope_held).sum::<usize>(),ConflictKind::Degraded {edit_ids}=>edit_ids.capacity()*size_of::<String>()+edit_ids.iter().map(String::capacity).sum::<usize>()}
}
fn verify(mut cursor:ProtocolConflictRetirement,held:usize,copy:usize,release:usize) {
    let mut returned=0;
    for turn in 0..4000 {
        let ((copied,freed,depth),heap)=observe_heap_allocations_on_this_thread(||(cursor.next_copy_byte_demand().unwrap(),cursor.next_release_byte_demand().unwrap(),cursor.next_depth_demand().unwrap()));assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));
        let grant=RetainedCloneGrant {maximum_items:1,maximum_copy_bytes:copy.max(copied),maximum_capacity_bytes:0,maximum_release_bytes:release.max(freed),maximum_depth:depth};
        for below in [RetainedCloneGrant {maximum_items:0,..grant},RetainedCloneGrant {maximum_copy_bytes:copied.saturating_sub(1),..grant},RetainedCloneGrant {maximum_release_bytes:freed.saturating_sub(1),..grant},RetainedCloneGrant {maximum_depth:depth.saturating_sub(1),..grant}] {
            if cursor.terminal_is_empty(){break;}
            if below.maximum_items!=0&&below.maximum_copy_bytes>=copied&&below.maximum_release_bytes>=freed&&below.maximum_depth>=depth {continue;}
            let(step,heap)=observe_heap_allocations_on_this_thread(||cursor.close_step(below).unwrap());assert_eq!(step.progress(),RetainedCloneProgress::default());assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));
        }
        let(step,heap)=observe_heap_allocations_on_this_thread(||cursor.close_step(grant).unwrap());assert_eq!(heap.requested_bytes,0);assert_eq!(heap.released_bytes,step.progress().released_bytes,"turn{turn}");assert!(step.progress().fits(grant));returned+=heap.released_bytes;
        if matches!(step,RetainedCloneStep::Complete(_)){assert!(cursor.terminal_is_empty());break;}
        assert!(turn<3999);
    }
    assert_eq!(returned,held);
    let(_,heap)=observe_heap_allocations_on_this_thread(||drop(cursor));assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));
}
#[test]
fn conflict_ownership_retirement_preserves_and_releases_every_causal_optional_owner() {
    let law=fixture();
    for row in law["envelopes"].as_array().unwrap(){for copy in law["copyGrants"].as_array().unwrap(){for release in law["releaseGrants"].as_array().unwrap(){let owner=envelope(row);let held=envelope_held(&owner);let original=owner.mutation_id.0.as_ptr();let(cursor,heap)=observe_heap_allocations_on_this_thread(||ProtocolConflictRetirement::causal_envelope(owner));assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));let ProtocolOwner::Envelope(envelope)=&cursor.owner else{panic!("original envelope owner");};assert_eq!(envelope.strings[0].as_ref().unwrap().as_ptr(),original);verify(cursor,held,copy.as_u64().unwrap()as usize,release.as_u64().unwrap()as usize);}}}
    println!("[DEBUG] native causal envelope10 identity/observed/transaction/verb/line strings, targets/dependencies/payloads include original unused capacities; undergrants0effects, physical release exact, terminal Drop0heap");
}
#[test]
fn conflict_ownership_retirement_preserves_all_quarantined_degraded_original_scaffolds() {
    let law=fixture();
    for row in law["conflicts"].as_array().unwrap(){for copy in law["copyGrants"].as_array().unwrap(){for release in law["releaseGrants"].as_array().unwrap(){let owner=conflict(row,&law);let held=conflict_held(&owner);let original=owner.messages.as_ptr();let(cursor,heap)=observe_heap_allocations_on_this_thread(||ProtocolConflictRetirement::new(owner));assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));let ProtocolOwner::Conflict(conflict)=&cursor.owner else{panic!("original conflict owner");};assert_eq!(conflict.value.as_ref().unwrap().messages.as_ptr(),original);verify(cursor,held,copy.as_u64().unwrap()as usize,release.as_u64().unwrap()as usize);}}}
    println!("[DEBUG] native quarantined/degraded/empty conflict branches retain original messages/envelopes/actors/identities and every old vector backing, independent copy/release/depth, terminal Drop0heap");
}
