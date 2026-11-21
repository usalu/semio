//! 🪝️ Boxed retained work preserves the original receiver behind its trait object.
use super::{SetCount, TestMutation, ToyRunApp};
use semio_framework_job::{OriginalTurnFailure, StepContext};
use semio_framework_value::{RetainedCloneProgress, RetirementDemand, ValueError};
use semio_plugin_decl_media::ArtifactOutputChunks;
use semio_plugin_retained_work::{ArtifactCommandInputs, ArtifactCommandWork, ArtifactCommandWorkStep};
use semio_plugin_testing_laws::artifact_app_laws::OriginalWorkFixture;
use std::mem::size_of;
use std::sync::{Arc, Mutex};

struct OriginalWork(Arc<Mutex<Option<ArtifactOutputChunks>>>);

impl ArtifactCommandWork<ToyRunApp> for OriginalWork {
    fn tool_id(&self) -> &'static str {
        "boxed-original-work"
    }

    fn original_wire_demands(&self, wire: &[u8], maximum_copy_bytes: usize) -> Result<Option<RetirementDemand>, ValueError> {
        Ok(Some(RetirementDemand { copy_bytes: wire.len().min(maximum_copy_bytes), depth: 1, ..Default::default() }))
    }

    fn receive_original_wire(&mut self, wire: &[u8], cx: &mut StepContext<'_>) -> Result<Option<TestMutation>, OriginalTurnFailure> {
        cx.consume_fuel(1);
        Ok(Some(TestMutation::SetCount(SetCount { value: i32::from(wire[0]) })))
    }

    fn receive_original_output_chunks(&mut self, chunks: &mut Option<ArtifactOutputChunks>, cx: &mut StepContext<'_>) -> Result<bool, OriginalTurnFailure> {
        if cx.retained_grant().maximum_items == 0 {
            return Ok(false);
        }
        cx.consume_retained(RetainedCloneProgress { copied_items: 1, copied_bytes: 2 * size_of::<Option<ArtifactOutputChunks>>(), ..Default::default() })?;
        *self.0.lock().unwrap() = chunks.take();
        Ok(true)
    }

    fn retains_original_wire(&self) -> bool {
        true
    }

    fn extent(&self, _: &TestMutation, _: &super::TestSnapshot, _: &semio_os_kernel::InteractionState, _: Option<&semio_plugin_decl_app::tool_contract::ArtifactOwnedToolJobContext<ToyRunApp>>) -> Option<usize> {
        Some(1)
    }

    fn step(&mut self, _: &ArtifactCommandInputs<'_, ToyRunApp>, _: &mut StepContext<'_>) -> Result<ArtifactCommandWorkStep<ToyRunApp>, OriginalTurnFailure> {
        unreachable!("this law exercises only the original receiver hooks")
    }
}

#[test]
fn boxed_work_forwards_original_wire_and_output_custody() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("fixture.json")).unwrap();
    let wire: Vec<u8> = serde_json::from_value(fixture["wire"].clone()).unwrap();
    let copy_budget = fixture["copyBudget"].as_u64().unwrap() as usize;
    let observed = Arc::new(Mutex::new(None));
    let mut work: Box<dyn ArtifactCommandWork<ToyRunApp>> = Box::new(OriginalWork(observed.clone()));
    let demand = ArtifactCommandWork::original_wire_demands(&work, &wire, copy_budget).unwrap().expect("boxed work exposes the inner quote");
    let retain_wire = ArtifactCommandWork::retains_original_wire(&work);

    let mut caller = OriginalWorkFixture::new();
    let grant = OriginalWorkFixture::stock();
    let mut chunks = None;
    let mut decoded = None;
    caller
        .turn(grant, |cx| {
            assert!(ArtifactOutputChunks::original_birth_admit(256, &mut chunks, cx)?);
            decoded = ArtifactCommandWork::receive_original_wire(&mut work, &wire, cx)?;
            Ok(())
        })
        .unwrap();
    let identity = chunks.as_ref().unwrap().original_identity();
    caller
        .turn(semio_framework_value::RetainedCloneGrant { maximum_items: 0, ..grant }, |cx| {
            assert!(!ArtifactCommandWork::receive_original_output_chunks(&mut work, &mut chunks, cx)?);
            Ok(())
        })
        .unwrap();
    assert_eq!(chunks.as_ref().unwrap().original_identity(), identity);
    caller
        .turn(grant, |cx| {
            assert!(ArtifactCommandWork::receive_original_output_chunks(&mut work, &mut chunks, cx)?);
            Ok(())
        })
        .unwrap();
    let received = observed.lock().unwrap().take().expect("the inner output receiver holds the queue");
    let output_transferred = chunks.is_none() && received.original_identity() == identity;
    caller.retire_owned(received, grant);
    caller.finish();

    let Some(TestMutation::SetCount(decoded)) = decoded else { panic!("the inner wire receiver must return its command") };
    assert_eq!(serde_json::json!({ "copyBytes": demand.copy_bytes, "decodedCount": decoded.value, "retainWire": retain_wire, "outputTransferred": output_transferred }), fixture["expected"]);
}
