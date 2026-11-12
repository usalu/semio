//! 🧾️ Original-caller doubles, member fixtures and envelope field decoders.
//!
//! 🧭️ Moved from the former store unit test module; the items are `pub` because the test crates of the Store use them through the public API only.

use semio_os_store_testing_prelude::*;
use semio_os_store_testing_fixtures::*;
use crate::*;
use semio_os_store_testing_fixtures::demo::{AddN, AssignN, DeleteN, DemoMutation, SetN};
use semio_os_store_testing_fixtures::lossy::{LossyMutation, SetN as LossySetN};
use semio_os_store_testing_fixtures::severity::{SetErrorN, SetFatalN, SetN as SeveritySetN, SetWarningN, SeverityMutation};
use semio_os_store_testing_fixtures::timestamped::{SetN as TimestampedSetN, TimestampedMutation};
use semio_os_store_testing_fixtures::validated::{AssignN as ValidatedAssignN, SetN as ValidatedSetN, ValidatedMutation};
use pack::snapshot::ArtifactSqliteSnapshot as _;
use semio_os_store_snapshot_clone::RetainedCloneEdit;
use semio_os_store_snapshot_clone::RetainedCloneEditCursor;
use semio_os_store_snapshot_clone::RetainedCloneEditStep;
use semio_os_store_snapshot_clone::RetainedClonePreparationFactory;
use semio_framework_value::retained_clone::{RetainedCloneGrant, RetainedCloneProgress, RetainedCloneRef};
use semio_framework_value::{RetainedAllocationEffect, RetainedWorkOutcome, RetainedWorkPoll};
use semio_framework_pack_json_retained_schema::testing::{OWNED_SCHEMA_TEST_FIELDS, owned_schema_test_cursor, drive_owned_schema, schema_test_close_grant};
use semio_framework_value::testing::physical_test_close_grant;
use crate::store::ArtifactStore;

pub struct TestEnvelopeFieldDecoder {
    pub terminal: bool,
    pub accepted: usize,
}

impl ArtifactEnvelopeFieldDecoder<(), ()> for TestEnvelopeFieldDecoder {
    fn accept_field_token(&mut self, _field_id: u16, _token: OwnedSchemaToken, terminal: bool, _source: &OwnedSchemaRecordCursor, cx: &mut semio_framework_job::StepContext<'_>) -> Result<ArtifactEnvelopeFieldDecodeStep, OwnedSchemaDecodeDiagnostic> {
        cx.consume_fuel(1);
        self.accepted += 1;
        Ok(if terminal { ArtifactEnvelopeFieldDecodeStep::FieldComplete } else { ArtifactEnvelopeFieldDecodeStep::TokenComplete })
    }

    fn finish_record(&mut self, cx: &mut semio_framework_job::StepContext<'_>) -> Result<ArtifactEnvelopeFieldDecodeStep, OwnedSchemaDecodeDiagnostic> {
        cx.consume_fuel(1);
        self.terminal = true;
        Ok(ArtifactEnvelopeFieldDecodeStep::RecordComplete)
    }

    fn maximum_close_byte_demand(&self) -> usize {
        0
    }

    fn maximum_retained_close_bytes(&self) -> usize {
        0
    }

    fn next_close_copy_byte_demand(&self) -> Result<usize, OwnedSchemaDecodeDiagnostic> {
        Ok(0)
    }

    fn next_close_capacity_byte_demand(&self, _maximum_copy_bytes: usize) -> Result<usize, OwnedSchemaDecodeDiagnostic> {
        Ok(0)
    }

    fn next_close_release_byte_demand(&self) -> Result<usize, OwnedSchemaDecodeDiagnostic> {
        Ok(0)
    }

    fn next_close_depth_demand(&self) -> Result<usize, OwnedSchemaDecodeDiagnostic> {
        Ok(0)
    }

    fn close_step(&mut self, effect: &mut RetainedAllocationEffect<'_>) -> ArtifactEnvelopeClosePoll {
        if effect.grant().maximum_items == 0 {
            return RetainedWorkPoll::Complete(RetainedWorkOutcome { result: Ok(RetainedCloneStep::Progress(Default::default())), work: 0 });
        }
        self.terminal = true;
        RetainedWorkPoll::Complete(RetainedWorkOutcome { result: Ok(RetainedCloneStep::Complete(Default::default())), work: effect.performed_work() })
    }

    fn terminal_is_empty(&self) -> bool {
        self.terminal
    }
}

pub fn artifact_envelope_decode_test_cursor(chunks: &[&[u8]]) -> OwnedSchemaRecordCursor {
    let bytes = chunks.iter().map(|chunk| chunk.len()).sum();
    let mut pages = OwnedSchemaDecodePages::try_with_credits(OwnedSchemaDecodeCredits { maximum_pages: chunks.len(), maximum_bytes: bytes }).expect("exact envelope decode credits");
    for chunk in chunks {
        let page = OwnedSchemaDecodePage::try_from_slice(chunk).expect("bounded envelope page");
        pages.admit_page(page).unwrap_or_else(|_| panic!("admitted envelope page"));
    }
    pages.seal().expect("sealed envelope pages");
    let tokens = OwnedSchemaTokenCursor::try_new(None, pages).unwrap_or_else(|_| panic!("sealed envelope token cursor"));
    OwnedSchemaRecordCursor::try_new(artifact_envelope_owned_schema(), tokens).unwrap_or_else(|_| panic!("valid envelope schema"))
}

/// 🧪️ Drives one poll-returning envelope close step inside an owner-less host-pump effect funded exactly by `grant`; a pending or unadmitted turn fails the test.
pub fn envelope_close_turn<T, E>(grant: RetainedCloneGrant, operation: impl FnOnce(&mut RetainedAllocationEffect<'_>) -> RetainedWorkPoll<T, E>) -> RetainedWorkOutcome<T, E> {
    match pump_turn(grant, operation).expect("envelope close work quote is admitted") { RetainedWorkPoll::Complete(turn) => turn, RetainedWorkPoll::Pending { .. } => panic!("envelope close turn stayed pending") }
}

pub fn envelope_field_close_grant() -> RetainedCloneGrant {
    RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES, maximum_capacity_bytes: ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES, maximum_release_bytes: 16 * ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES, maximum_depth: 16 }
}


/// 🎟️ The envelope cleanup caller is the shared once-paid job fixture caller, so every cleanup turn carries its real producer invocation.
pub struct OriginalEnvelopeCaller {
    pub inner: semio_framework_job::OriginalJobFixtureCaller,
}

impl OriginalEnvelopeCaller {
    pub fn admit(operation:semio_framework_job::OperationId,generation:semio_framework_job::Generation,source:RetainedCloneGrant,clock:fn()->Option<u64>,deadline:u64,fuel:u64)->Self{
        Self{inner:semio_framework_job::OriginalJobFixtureCaller::admit(operation,generation,source,clock,deadline,fuel)}
    }


    pub fn admit_cleanup(id:&str)->Self{
        let policies:serde_json::Value=serde_json::from_str(include_str!("../../../../../../../🔨️modules/🧵️job/🎟️context/🪙️loan/🧪️tests/🧫️fixtures/🎟️store.json")).unwrap();
        let policy=policies.as_array().unwrap().iter().find(|policy|policy["id"]==id).unwrap();
        assert_eq!(policy["clockUs"],1000);
        Self::admit(semio_framework_job::OperationId(policy["operation"].as_u64().unwrap()),semio_framework_job::Generation(policy["generation"].as_u64().unwrap()),serde_json::from_value(policy["grant"].clone()).unwrap(),||Some(1000),policy["deadlineUs"].as_u64().unwrap(),policy["fuel"].as_u64().unwrap())
    }

    pub fn context(&mut self)->semio_framework_job::StepContext<'_>{
        self.inner.lend_context()
    }

    pub fn retire(self,source:RetainedCloneGrant){
        self.inner.retire(source);
    }
}

pub fn drive_test_envelope_field_return(registry: &Arc<ArtifactEnvelopeFieldDecoderRegistry<(), ()>>, detached: &mut Option<ArtifactEnvelopeReturnedFieldDecoder<(), ()>>) {
    if let Some(retirement) = detached.as_mut() {
        let step = envelope_close_turn(envelope_field_close_grant(), |effect| ErasedSnapshotRetirement::close_step(retirement, envelope_field_close_grant().maximum_copy_bytes, effect)).result.expect("test app return pump closes one exact decoder owner");
        if matches!(step, RetainedCloneStep::Complete(_)) {
            assert!(retirement.terminal_is_empty());
            drop(detached.take());
        }
        return;
    }
    let Some(ticket) = registry.next_returned_ticket() else { return };
    *detached = Some(registry.take_returned_ticket(ticket).expect("test app return pump detaches the exact returned generation"));
}


pub fn member_dialect_fixture() -> serde_json::Value {
    serde_json::from_str(include_str!("../../../🧩️composition/🪪️member-dialect/🧪️tests/🧫️fixtures/🔣️.json")).expect("neutral member dialect corpus")
}

pub fn close_member_dialect_fixture<M: SpaceMember>(member: &mut M) {
    let grant=physical_test_close_grant();
    let mut stalled=0usize;
    for _ in 0..65_536 {
        let demand=member.close_owned_demands(grant.maximum_copy_bytes).expect("current independent member close frontier");
        let turn=RetainedCloneGrant { maximum_depth: demand.depth.max(grant.maximum_depth), ..grant };
        let(step,allocated,released)=semio_framework_trace_memory_testing::observe_backing(||member.close_owned_step_cold(turn).expect("declared fixture member close"));
        assert!(step.progress().fits(turn));assert_eq!((allocated,released),(step.progress().retained_capacity_bytes,step.progress().released_bytes));
        match step {
            RetainedCloneStep::Complete(_) => { assert!(member.close_owned_terminal_is_empty());return; }
            RetainedCloneStep::Progress(progress) => {
                stalled=if progress==RetainedCloneProgress::default(){stalled+1}else{0};
                assert!(stalled<4096,"fixture member close stalled on a fully funded turn; frontier {demand:?}");
            }
        }
    }
    panic!("fixture member retirement did not finish");
}


/// 🎯️ The dialect every composition fixture below mints children under.
pub fn demo_child_dialect() -> semio_framework_artifact_reference::ArtifactDialect {
    semio_framework_artifact_reference::ArtifactDialect { artifact_kind: "s.stdio.mesh".into(), standard: "1".into(), subset: "*".into() }
}


/// 🎟️ The member-open caller is the shared once-paid job fixture caller with the member policy fixtures, so every turn carries its real producer invocation.
pub struct OriginalMemberCaller{pub inner:Option<semio_framework_job::OriginalJobFixtureCaller>,pub policy:RetainedCloneGrant,pub maximum_turns:usize}

impl OriginalMemberCaller{
 pub fn policy(&self)->RetainedCloneGrant{self.policy}
 pub fn maximum_turns(&self)->usize{self.maximum_turns}
 pub fn new()->Self{
  let fixture:serde_json::Value=serde_json::from_str(include_str!("../../../🧩️composition/🚪️open/🏭️operation/📏️birth/🧪️tests/🧫️fixtures/🔣️.json")).unwrap();
  let policy:RetainedCloneGrant=serde_json::from_value(fixture["snapshotInput"]["policy"].clone()).unwrap();
  let receiving:serde_json::Value=serde_json::from_str(include_str!("../../../🧩️composition/🚪️open/📜️history/🏭️factory/🧪️tests/🧫️receiving/🔣️.json")).unwrap();
  let caller=&receiving["caller"];assert_eq!(caller["clockUs"],1);
  let inner=semio_framework_job::OriginalJobFixtureCaller::admit(semio_framework_job::OperationId(caller["operation"].as_u64().unwrap()),semio_framework_job::Generation(caller["generation"].as_u64().unwrap()),policy,||Some(1),caller["deadlineUs"].as_u64().unwrap(),caller["fuel"].as_u64().unwrap());
  Self{inner:Some(inner),policy,maximum_turns:caller["maximumTurns"].as_u64().unwrap()as usize}
 }
 pub fn run<T>(&mut self,grant:RetainedCloneGrant,run:impl FnOnce(&semio_framework_job::OriginalCallerSourceParent,&mut semio_framework_job::StepContext<'_>)->T)->(T,RetainedCloneProgress){
  assert!(grant.maximum_items<=self.policy.maximum_items&&grant.maximum_copy_bytes<=self.policy.maximum_copy_bytes&&grant.maximum_capacity_bytes<=self.policy.maximum_capacity_bytes&&grant.maximum_release_bytes<=self.policy.maximum_release_bytes&&grant.maximum_depth<=self.policy.maximum_depth);
  let inner=self.inner.as_mut().expect("original member caller is open");
  let result=inner.turn(grant,|cx|{let parent=cx.original_source_parent()?;Ok(run(parent,cx))}).expect("original member caller turn");
  (result,inner.receipt())
 }
 pub fn close(&mut self){if let Some(inner)=self.inner.take(){inner.retire(self.policy);}}
}

pub fn retire_original_member_open(open:&mut impl MemberOpenOperation,caller:&mut OriginalMemberCaller)->(usize,usize,usize){
 let(mut born,mut physical,mut turns)=(0,0,0);
 while !MemberOpenOperation::terminal_is_empty(open)&&turns<caller.maximum_turns{let policy=caller.policy;let((step,receipt),b,r)=semio_framework_trace_memory_testing::observe_backing(||caller.run(policy,|parent,cx|MemberOpenOperation::close_step(open,parent,cx).unwrap()));assert_eq!(receipt,step.progress());assert!(receipt.fits(policy));assert_eq!((receipt.retained_capacity_bytes,receipt.released_bytes),(b,r));assert!(receipt.copied_items>0||matches!(step,RetainedCloneStep::Complete(_)),"original Member turn advances under fixed source policy");born+=b;physical+=r;turns+=1;}
 assert!(MemberOpenOperation::terminal_is_empty(open));(born,physical,turns)
}
