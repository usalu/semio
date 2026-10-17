//! 🎟️ Cancellation scenarios use their actual creation issuer and paid lease retirement.
use super::*;
use semio_framework_job::{OriginalJobFixtureCaller,Generation};

fn phase()->RetainedCloneGrant{RetainedCloneGrant{maximum_items:65536,maximum_copy_bytes:134217728,maximum_capacity_bytes:268435456,maximum_release_bytes:536870912,maximum_depth:4096}}
struct Fixture{caller:OriginalJobFixtureCaller,issuer:ToolCancellationIssuer}
impl Fixture{
    fn open()->Self{
        let mut caller=OriginalJobFixtureCaller::admit(semio_framework_job::allocate_operation_id(),Generation(1),phase(),||Some(1000),1500,1_000_000);
        let mut issuer=ToolCancellationIssuer::empty();
        for _ in 0..32{if issuer.ready(){break}caller.turn(phase(),|cx|{let parent=cx.original_source_parent()?;issuer.admit_step(parent,cx)}).unwrap();}
        assert!(issuer.ready());Self{caller,issuer}
    }
    fn begin(&mut self,key:&ToolOperationKey)->ToolCancellationLease{
        for _ in 0..32{if let Some(lease)=self.caller.turn(phase(),|cx|self.issuer.begin(key,cx)).unwrap(){return lease}}
        panic!("original cancellation lease admission stalled")
    }
    fn cursor(&mut self,lease:ToolCancellationLease)->Box<crate::ToolCancellationLeaseRetirement>{
        let mut lease=Some(lease);let mut cursor=None;
        self.caller.turn(phase(),|cx|{let(owner,progress)=crate::original_lease_cursor_admit(&mut lease,cx.retained_grant())?;cursor=owner;cx.consume_retained(progress)?;if progress.copied_items>0{cx.consume_fuel(1)}Ok(())}).unwrap();
        assert!(lease.is_none());cursor.expect("paid original lease cursor")
    }
    fn cursor_step(&mut self,cursor:&mut crate::ToolCancellationLeaseRetirement){
        self.caller.turn(phase(),|cx|{let step=cursor.close_step(&self.issuer,cx.retained_grant())?;cx.consume_retained(step.progress())?;if step.progress().copied_items>0{cx.consume_fuel(1)}Ok(())}).unwrap();
    }
    fn close_cursor(&mut self,mut cursor:Box<crate::ToolCancellationLeaseRetirement>){
        for _ in 0..128{if cursor.terminal_is_empty(){break}self.cursor_step(&mut cursor)}
        assert!(cursor.terminal_is_empty());let mut cursor=Some(cursor);
        self.caller.turn(phase(),|cx|{let progress=crate::original_lease_cursor_release(&mut cursor,cx.retained_grant())?;cx.consume_retained(progress)?;if progress.copied_items>0{cx.consume_fuel(1)}Ok(())}).unwrap();assert!(cursor.is_none());
    }
    fn close_lease(&mut self,lease:ToolCancellationLease){let cursor=self.cursor(lease);self.close_cursor(cursor)}
    fn close(self){
        let Self{mut caller,mut issuer}=self;
        for _ in 0..128{if issuer.terminal_is_empty(){break}caller.turn(phase(),|cx|{let parent=cx.original_source_parent()?;issuer.close_step(parent,cx)}).unwrap();}
        assert!(issuer.terminal_is_empty());drop(issuer);caller.retire(phase());
    }
}
fn key(document:u32)->ToolOperationKey{ToolOperationKey{app_instance_id:document,document:ArtifactDocumentAuthority(document),operation_id:semio_framework_job::allocate_operation_id(),base_revision:semio_framework_job::RevisionId(1),generation:Generation(1)}}

#[test]
fn tool_cancellation_isolated_by_live_instance_with_same_controller_and_document(){
    let mut fixture=Fixture::open();let mut first_key=key(41);first_key.base_revision=semio_framework_job::RevisionId(7);first_key.generation=Generation(3);
    let second_key=ToolOperationKey{app_instance_id:42,document:ArtifactDocumentAuthority(42),operation_id:semio_framework_job::allocate_operation_id(),..first_key.clone()};
    let first_lease=fixture.begin(&first_key);let second_lease=fixture.begin(&second_key);
    assert_eq!(fixture.issuer.active_operation_count(),2);
    assert!(fixture.issuer.cancel_document(ArtifactDocumentAuthority(41)).expect("document cancellation"));
    assert!(first_lease.cancel_token().is_cancelled_now());assert!(!second_lease.cancel_token().is_cancelled_now());
    fixture.close_lease(first_lease);
    assert_eq!(fixture.issuer.active_operation_count(),1);
    fixture.close_lease(second_lease);fixture.close();
}

#[test]
fn cancellation_supersession_and_saturated_app_close_are_parent_scope_constant_time(){
    let mut fixture=Fixture::open();let first_key=key(7);let first=fixture.begin(&first_key);let next=ToolOperationKey{operation_id:semio_framework_job::allocate_operation_id(),..first_key};
    let deferred=fixture.caller.turn(phase(),|cx|fixture.issuer.begin(&next,cx)).unwrap();assert!(deferred.is_none());
    assert!(first.cancel_token().is_cancelled_now());let mut first_close=fixture.cursor(first);fixture.cursor_step(&mut first_close);
    let mut second=fixture.begin(&next);assert!(!second.cancel_token().is_cancelled_now());assert_eq!(fixture.issuer.active_operation_count(),1);
    fixture.close_cursor(first_close);assert_eq!(fixture.issuer.active_operation_count(),1,"a stale lease must not release the replacement generation");
    second.finish();let mut second_close=fixture.cursor(second);fixture.cursor_step(&mut second_close);
    let mut saturated=Vec::new();for index in 0..1_024u32{saturated.push(fixture.begin(&key(index.saturating_add(100))));}
    let generation=fixture.issuer.scope_generation();fixture.issuer.cancel_scope_generation();assert_eq!(fixture.issuer.scope_generation(),generation.saturating_add(1));
    assert!(second_close.original.as_ref().expect("detached original second lease remains owned").token.is_cancelled_now());
    assert!(saturated.first().expect("saturated lease").cancel_token().is_cancelled_now());assert!(saturated.last().expect("saturated lease").cancel_token().is_cancelled_now());
    for lease in saturated{fixture.close_lease(lease)}fixture.close_cursor(second_close);fixture.close();
}

#[test]
fn cancellation_numeric_authority_rejects_collision_capacity_and_contention_without_blocking(){
    let mut fixture=Fixture::open();let first=key(100_000);let authority=fixture.begin(&first);let collision_key=key(100_000+TOOL_CANCELLATION_SLOTS as u32);
    let collision=fixture.caller.turn(phase(),|cx|fixture.issuer.begin(&collision_key,cx)).err().expect("same direct slot must fail closed");
    assert_eq!(collision.primary_value().expect("original native collision refusal").kind,semio_framework_value::ValueRefusalKind::InvariantViolated);
    assert_eq!(collision.primary.message(),"fixed cancellation document slot collision");fixture.close_lease(authority);
    let recycled=fixture.begin(&collision_key);fixture.close_lease(recycled);
    let state=Arc::clone(&fixture.issuer.handle().unwrap().state);let guard=state.try_lock().expect("test owns cancellation authority");let attempt=key(77);let before=fixture.caller.cumulative_receipt();let fuel=fixture.caller.fuel_remaining();
    let contended=fixture.caller.turn(phase(),|cx|fixture.issuer.begin(&attempt,cx)).unwrap();
    assert!(contended.is_none(),"contention must not wait");assert_eq!(fixture.caller.cumulative_receipt(),before);assert_eq!(fixture.caller.fuel_remaining(),fuel);drop(guard);drop(state);fixture.close();
}

#[test]
fn interrupted_cancellation_gate_preserves_original_paid_slot_and_refuses_competing_authority(){
    let law:serde_json::Value=serde_json::from_str(include_str!("../../../../🧪️tests/📦️units/🧪️interruption.json")).unwrap();assert_eq!(law["slots"].as_u64().unwrap(),TOOL_CANCELLATION_SLOTS as u64);
    let mut fixture=Fixture::open();let mut original=key(law["original"]["document"].as_u64().unwrap()as u32);original.operation_id=semio_framework_job::OperationId(law["original"]["operation"].as_u64().unwrap());original.generation=Generation(law["original"]["generation"].as_u64().unwrap());let lease=fixture.begin(&original);
    let state=Arc::clone(&fixture.issuer.handle().unwrap().state);let generation=fixture.issuer.scope_generation();let receipt=fixture.caller.cumulative_receipt();let fuel=fixture.caller.fuel_remaining();
    let interrupted=std::panic::catch_unwind(std::panic::AssertUnwindSafe(||fixture.caller.turn(phase(),|_cx|->Result<(),OriginalTurnFailure>{let _guard=state.try_lock().expect("original paid slot remains exclusively held before commit");panic!("interrupt the original cancellation mutation before its slot commit")})));
    assert!(interrupted.is_err());assert_eq!(fixture.issuer.active_operation_count(),1);assert_eq!(fixture.issuer.scope_generation(),generation);assert_eq!(fixture.caller.cumulative_receipt(),receipt);assert_eq!(fixture.caller.fuel_remaining(),fuel);
    let guard=state.try_lock().expect("actual non-poisoning gate releases exclusion after unwind");let retained=guard.get(ToolCancellationHandle::slot(original.document)).expect("original paid slot remains retained");
    assert_eq!(retained.operation,original);assert!(PublicationClaimHandle::same_original(&retained.publication_claim,&lease.publication_claim));assert!(!lease.cancel_token().is_cancelled_now());drop(guard);drop(state);
    let mut competing=key(law["competing"]["document"].as_u64().unwrap()as u32);competing.operation_id=semio_framework_job::OperationId(law["competing"]["operation"].as_u64().unwrap());competing.generation=Generation(law["competing"]["generation"].as_u64().unwrap());let refused=fixture.caller.turn(phase(),|cx|fixture.issuer.begin(&competing,cx)).err().expect("interruption must not grant the competing slot authority");
    assert_eq!(refused.primary.message(),"fixed cancellation document slot collision");assert_eq!(fixture.issuer.active_operation_count(),1);
    fixture.close_lease(lease);fixture.close();
}
