// 🔄️ Refused reload and erased close demand laws: the refused reload that retires its candidate, the demanding buffer, nested and unsupported demand retirements, the initialization runtime physical backing and the erased retirement close demand.

//#region 🔖️RefusedReloadTests
/// 🛡️ A REFUSED reload RETURNS its `VcsError` — it never aborts the process. `set_state` owns the
/// candidate envelope it consumed and `ArtifactEnvelope`'s `Drop` asserts its owners were detached,
/// so letting a `?` drop a POPULATED candidate turned every malformed-document refusal into a
/// process abort instead of the error the caller is meant to read (measured 2026-09-21 as
/// `ArtifactEnvelope::drop ← drop_glue ← set_state::{closure#0} ← reset::{closure#0}`). The refusal
/// path hands the candidate to `ArtifactEnvelope::retire_unadopted`, the live store survives
/// unchanged, and it still closes to terminal-empty afterwards.
#[semio_framework_async_macros::async_test]
async fn a_refused_reload_returns_its_error_and_retires_the_candidate_it_consumed() {
    let mut store = ArtifactStore::bare(create_document_envelope::<DemoSnapshot, DemoMutation>("demo/v1", "refused-reload", DemoSnapshot { n: Some(3) }, None, &semio_framework_os_vcs::io::binary::genesis::ArtifactPackGenesisProducer)).await;
    store.install_document_store_owners_exact(demo_closable_store_owners()).unwrap_or_else(|(error,_)|panic!("funded complete test catalog install refused: {error}"));
    let mut rejected = create_document_envelope::<DemoSnapshot, DemoMutation>("demo/v1", "refused-reload", DemoSnapshot { n: Some(9) }, None, &semio_framework_os_vcs::io::binary::genesis::ArtifactPackGenesisProducer);
    let orphan = vec![semio_framework_os_spr::EditMessages { edit_id: "edit-that-never-existed".to_string(), messages: vec![semio_framework_os_spr::MutationMessage::info("mutation.cascade", "orphan")] }];
    rejected.edit_messages = ArtifactEditMessageLedger::try_from_entries(orphan).unwrap_or_else(|_| panic!("one exact orphan message entry is admitted by the fixed ledger"));
    let error = store.reset(rejected).await.expect_err("a message ledger naming an edit the history never recorded must be refused");
    assert!(format!("{error:?}").contains("invalid message ledger"), "the caller reads the refusal itself: {error:?}");
    assert_eq!(store.snapshot_ref().n, Some(3), "the live store survives a refused reload unchanged");
    close_demo_artifact_store(&mut store);
}

//#endregion 🔖️RefusedReloadTests

//#region 🔖️ErasedCloseDemandTests
/// 🧱️ The ERASED close-byte demand a nested driver reads before it grants.
struct DemandingBufferRetirement {
    buffer: Option<Vec<u8>>,
}

#[test]
fn initialization_runtime_preserves_physical_backing_and_terminal_cursor_box() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../../../../../../../🔨️modules/🧵️job/🧪️tests/🧫️fixtures/📏️close-demand/🔣️.json")).unwrap();
    let policy=physical_test_close_grant();
    let factory:Arc<dyn ArtifactOwnedValueRetirementFactory<DemoSnapshot>>=Arc::new(DemoInitialSnapshotRetirementFactory);
    for method in ["settle", "close"] {
        for row in fixture["cases"].as_array().unwrap() {
            let extent=row["physicalBytes"].as_u64().unwrap() as usize;
            let mut buffer=Vec::new();buffer.try_reserve_exact(extent).unwrap();assert_eq!(buffer.capacity(),extent);
            let mut runtime=ArtifactStoreInitializationRuntime::new("physical-runtime","demo/v1",Arc::new(DemoSnapshot { n:Some(0) }),[7;32],ActorId("actor:runtime-release".into()));
            let(owner,birth)=admit_artifact_retirement(buffer,policy,|original|DemandingBufferRetirement { buffer:Some(original) }).unwrap_or_else(|(error,_)|panic!("funded runtime cursor birth refused: {error}"));
            assert!(birth.fits(policy));*runtime.close_active=Some(owner);
            let demand=runtime.next_close_release_byte_demand().unwrap();
            let advance=|runtime:&mut ArtifactStoreInitializationRuntime<DemoSnapshot>,grant:RetainedCloneGrant|if method=="settle" { runtime.settle_current_retirement_step(grant) } else { runtime.close_step(&factory,grant) };
            let caller=row["callerBytes"].as_u64().unwrap() as usize;
            let first_grant=RetainedCloneGrant { maximum_release_bytes: caller,..policy };
            let(first,allocated,released)=semio_framework_trace_memory_testing::observe_backing(||advance(&mut runtime,first_grant).unwrap());
            assert!(first.progress().fits(first_grant));assert_eq!(allocated,0);assert_eq!(released,first.progress().released_bytes);
            assert_eq!(first.progress().released_bytes,row["releasedBytes"].as_u64().unwrap() as usize);assert_eq!(demand,extent);
            if released==0 {
                assert!(runtime.close_active.is_some());assert_eq!(runtime.next_close_release_byte_demand().unwrap(),extent);
                let(step,allocated,released)=semio_framework_trace_memory_testing::observe_backing(||advance(&mut runtime,policy).unwrap());assert!(step.progress().fits(policy));assert_eq!((allocated,released),(0,extent));
            }
            let frame=size_of::<DemandingBufferRetirement>();assert_eq!(runtime.next_close_release_byte_demand().unwrap(),frame);
            let(denied,allocated,released)=semio_framework_trace_memory_testing::observe_backing(||advance(&mut runtime,RetainedCloneGrant { maximum_release_bytes:frame-1,..policy }).unwrap());
            assert_eq!((allocated,released),(0,0));assert_eq!(denied.progress(),RetainedCloneProgress::default());assert!(runtime.close_active.is_some());
            let(step,allocated,released)=semio_framework_trace_memory_testing::observe_backing(||advance(&mut runtime,policy).unwrap());assert_eq!((allocated,released),(0,frame));assert_eq!(step.progress().released_bytes,frame);assert!(runtime.close_active.is_none());
            for _ in 0..1024 {
                if runtime.terminal_is_empty() { break; }
                let(step,allocated,released)=semio_framework_trace_memory_testing::observe_backing(||runtime.close_step(&factory,policy).unwrap());
                assert!(step.progress().fits(policy));assert_eq!((allocated,released),(step.progress().retained_capacity_bytes,step.progress().released_bytes));
            }
            assert!(runtime.terminal_is_empty());
        }
    }
}

impl ErasedSnapshotRetirement for DemandingBufferRetirement {
        fn next_demand(&self, body:usize)->Result<semio_framework_value::RetirementDemand,semio_framework_value::ValueError>{Ok(semio_framework_value::RetirementDemand { release_bytes: self.buffer.as_ref().map_or(0,Vec::capacity), depth: usize::from(self.buffer.is_some()), ..Default::default() })}
fn close_step(&mut self, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, ValueError> {
        let Some(buffer)=self.buffer.as_ref() else { return Ok(RetainedCloneStep::Complete(Default::default())); };
        if grant.maximum_items==0 { return Ok(RetainedCloneStep::Progress(Default::default())); }
        if grant.maximum_depth==0 { return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::DepthLimit,"original buffer requires supplied depth")); }
        if grant.maximum_release_bytes<buffer.capacity() { return Ok(RetainedCloneStep::Progress(Default::default())); }
        let released=self.buffer.take().expect("original physical buffer remains present").capacity();
        Ok(RetainedCloneStep::Complete(RetainedCloneProgress { copied_items: 1,released_bytes: released,..Default::default() }))
    }
    fn terminal_is_empty(&self)->bool { self.buffer.is_none() }
    fn next_copy_byte_demand(&self)->Result<usize,ValueError>{Ok(0)}
    fn next_capacity_byte_demand(&self,_:usize)->Result<usize,ValueError>{Ok(0)}
    fn next_release_byte_demand(&self)->Result<usize,ValueError>{Ok(self.buffer.as_ref().map_or(0,Vec::capacity))}
    fn next_depth_demand(&self)->Result<usize,ValueError>{Ok(usize::from(self.buffer.is_some()))}
}

struct NestedErasedDriver { inner:Option<Box<dyn ErasedSnapshotRetirement>> }

impl NestedErasedDriver {
    fn close(&mut self, grant:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError>{artifact_retirement_box_close_step(&mut self.inner,grant)}
}

struct UnsupportedDemandRetirement;

impl ErasedSnapshotRetirement for UnsupportedDemandRetirement {
        fn next_demand(&self, body:usize)->Result<semio_framework_value::RetirementDemand,semio_framework_value::ValueError>{Err(ValueError::literal(semio_framework_value::ValueRefusalKind::UnsupportedOwner,"owner has no admitted physical retirement contract"))}
fn close_step(&mut self,_:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError>{Err(ValueError::literal(semio_framework_value::ValueRefusalKind::UnsupportedOwner,"owner has no admitted physical retirement contract"))}
    fn terminal_is_empty(&self)->bool{false}
    fn next_copy_byte_demand(&self)->Result<usize,ValueError>{Err(ValueError::literal(semio_framework_value::ValueRefusalKind::UnsupportedOwner,"copy demand is unsupported"))}
    fn next_capacity_byte_demand(&self,_:usize)->Result<usize,ValueError>{Err(ValueError::literal(semio_framework_value::ValueRefusalKind::UnsupportedOwner,"capacity demand is unsupported"))}
    fn next_release_byte_demand(&self)->Result<usize,ValueError>{Err(ValueError::literal(semio_framework_value::ValueRefusalKind::UnsupportedOwner,"release demand is unsupported"))}
    fn next_depth_demand(&self)->Result<usize,ValueError>{Err(ValueError::literal(semio_framework_value::ValueRefusalKind::UnsupportedOwner,"depth demand is unsupported"))}
}

/// ⚖️ Original buffer and cursor frames require separate actual caller release grants; demand earns no authority.
#[test]
fn an_erased_retirement_publishes_its_physical_close_demand_and_a_nested_driver_consumes_supplied_grants() {
    let unsupported=UnsupportedDemandRetirement;
    assert_eq!(unsupported.next_release_byte_demand().unwrap_err().kind,semio_framework_value::ValueRefusalKind::UnsupportedOwner);
    let grant=physical_test_close_grant();
    let mut buffer=Vec::with_capacity(8192);buffer.resize(8192,0u8);let pointer=buffer.as_ptr();assert_eq!(buffer.capacity(),8192);
    let (owner,birth)=admit_artifact_retirement(buffer,grant,|original|DemandingBufferRetirement { buffer:Some(original) }).unwrap_or_else(|(error,_)|panic!("original buffer cursor birth refused: {error}"));
    assert_eq!(birth.retained_capacity_bytes,std::mem::size_of::<DemandingBufferRetirement>());
    let mut nested=NestedErasedDriver { inner:Some(owner) };
    assert_eq!(nested.inner.as_ref().unwrap().next_release_byte_demand().unwrap(),8192);
    for denied in [RetainedCloneGrant { maximum_items:0,..grant },RetainedCloneGrant { maximum_release_bytes:8191,..grant },RetainedCloneGrant { maximum_depth:0,..grant }] {
        let (result,allocated,released)=semio_framework_trace_memory_testing::observe_backing(||nested.close(denied));assert_eq!((allocated,released),(0,0));
        if denied.maximum_depth==0 { assert_eq!(result.unwrap_err().kind,semio_framework_value::ValueRefusalKind::DepthLimit); } else { assert_eq!(result.unwrap().progress(),RetainedCloneProgress::default()); }
        assert!(nested.inner.is_some());assert_eq!(nested.inner.as_ref().unwrap().next_release_byte_demand().unwrap(),8192);
    }
    let(step,allocated,released)=semio_framework_trace_memory_testing::observe_backing(||nested.close(RetainedCloneGrant { maximum_copy_bytes:1,maximum_release_bytes:8192,..grant }).unwrap());
    assert_eq!((allocated,released),(0,8192));assert_eq!(step.progress(),RetainedCloneProgress { copied_items:1,released_bytes:8192,..Default::default() });
    assert!(nested.inner.as_ref().unwrap().terminal_is_empty());
    let frame=std::mem::size_of::<DemandingBufferRetirement>();
    let(step,allocated,released)=semio_framework_trace_memory_testing::observe_backing(||nested.close(RetainedCloneGrant { maximum_release_bytes:frame-1,..grant }).unwrap());assert_eq!((allocated,released),(0,0));assert_eq!(step.progress(),RetainedCloneProgress::default());assert!(nested.inner.is_some());
    let(step,allocated,released)=semio_framework_trace_memory_testing::observe_backing(||nested.close(grant).unwrap());assert_eq!((allocated,released),(0,frame));assert_eq!(step.progress().released_bytes,frame);assert!(nested.inner.is_none());
}

//#endregion 🔖️ErasedCloseDemandTests
