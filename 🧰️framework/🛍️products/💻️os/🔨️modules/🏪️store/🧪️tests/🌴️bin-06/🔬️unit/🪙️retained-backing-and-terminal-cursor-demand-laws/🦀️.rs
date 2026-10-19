/// 🪙️ Retained backing and terminal cursor demand laws: the retained backing demand of a fresh release, the nested allocation demand, the terminal snapshot cursor box under a grant, the active history decoder, the displaced store owner and the installed store disposer.

/// 🧮️ The fresh VCS release driver funds the actual retained allocation published by its erased owner.
#[test]
fn fresh_vcs_release_publishes_actual_retained_backing_demand() {
    #[derive(semio_framework_value::FactoryPayloadRetirement)]
    struct UnusedEditDecoder;
    impl ArtifactOwnedHistoryEntryDecoder<Edit<DemoMutation>> for UnusedEditDecoder {
        fn begin_entry(&self, _: semio_framework_job::OperationId, _: semio_framework_job::Generation, _: OwnedSchemaPath, _: Arc<dyn ArtifactOwnedValueRetirementFactory<Edit<DemoMutation>>>) -> Box<dyn ArtifactOwnedHistoryEntryAuthority<Edit<DemoMutation>>> { panic!("the release-only fixture never decodes a history entry") }
    }
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../../../../../../../🔨️modules/🧵️job/🧪️tests/🧫️fixtures/📏️close-demand/🔣️.json")).unwrap();
    let admission = fixture["admissionBytes"].as_u64().unwrap() as usize;
    let mut failures = Vec::new();
    for row in fixture["cases"].as_array().unwrap() {
        let extent = row["physicalBytes"].as_u64().unwrap() as usize;
        let mut backing = Vec::<u8>::new(); backing.try_reserve_exact(extent).unwrap(); assert_eq!(backing.capacity(), extent);
        let mut owner = ArtifactEnvelopeFreshVcsAuthority::<DemoSnapshot, DemoMutation> {
            cursor: OwnedSchemaNestedRecordCursor::try_new(OwnedSchemaRecordSpec { fields: ARTIFACT_ENVELOPE_FRESH_VCS_FIELDS }).unwrap(),
            snapshot: None, snapshot_target: ArtifactEnvelopeFreshSnapshotTarget::new(), genesis_pack: None, genesis_bytes: None, active: None, pending: None,
            edits: None, changes: None, checkpoints: None, alternatives: None, value: None,
            retirement: Some(Box::new(DemandingBufferRetirement { buffer: Some(backing) })),
            initial_snapshot_factory: Arc::new(DemoInitialSnapshotRetirementFactory), mutation_factory: Arc::new(DemoMutationRetirementFactory), edit_decoder: Arc::new(UnusedEditDecoder),
            maximum_snapshot_close_byte_demand: admission, maximum_retained_snapshot_close_bytes: admission, terminal: false,
        };
        let advertised = owner.next_close_release_byte_demand().unwrap();
        let physical = owner.retirement.as_ref().unwrap().next_release_byte_demand().unwrap();
        let caller = row["callerBytes"].as_u64().unwrap() as usize;
        let (caller_step, _, caller_released) = semio_framework_trace_memory_testing::observe_backing(|| owner.close_step(RetainedCloneGrant { maximum_items: 1, maximum_release_bytes: caller, maximum_depth: 16, ..Default::default() }).unwrap());
        let caller_expected = row["releasedBytes"].as_u64().unwrap() as usize;
        let mut progress = vec![(caller_step, caller_released)];
        for _ in 0..512 {
            if owner.terminal_is_empty() { break; }
            let grant = RetainedCloneGrant { maximum_capacity_bytes: admission, maximum_release_bytes: admission, ..physical_test_close_grant() };
            let (step, _, released) = semio_framework_trace_memory_testing::observe_backing(|| owner.close_step(grant).unwrap());
            progress.push((step, released));
        }
        let funded_terminal = owner.terminal_is_empty();
        for _ in 0..1024 { if owner.terminal_is_empty() { break; } let grant = RetainedCloneGrant { maximum_capacity_bytes: admission, maximum_release_bytes: admission, ..physical_test_close_grant() }; owner.close_step(grant).unwrap(); }
        assert!(owner.terminal_is_empty(), "exact independently admitted allocation grant drains the fixture before assertions");
        let exact_accounting = progress.iter().all(|(step, actual)| step.progress().released_bytes == *actual);
        if advertised != physical || !funded_terminal || !exact_accounting || caller_released != caller_expected { failures.push((extent, advertised, physical, funded_terminal, exact_accounting)); }
    }
    assert!(failures.is_empty(), "the fresh VCS driver publishes sufficient exact physical demands: {failures:?}");
}

/// 🪜️ VCS retirement forwards each exact nested allocation demand without losing physical cursor releases.
#[test]
fn vcs_retirement_propagates_nested_physical_allocation_demand() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../../../../../../../🔨️modules/🧵️job/🧪️tests/🧫️fixtures/📏️close-demand/🔣️.json")).unwrap();
    let admission = fixture["admissionBytes"].as_u64().unwrap() as usize;
    let mut failures = Vec::new();
    for row in fixture["cases"].as_array().unwrap() {
        let extent = row["physicalBytes"].as_u64().unwrap() as usize;
        let mut buffer = Vec::new(); buffer.try_reserve_exact(extent).unwrap(); assert_eq!(buffer.capacity(), extent);
        let mut owner = ArtifactStoreVcsRetirement::<DemoSnapshot, DemoMutation> {
            vcs: std::mem::ManuallyDrop::new(None), active: std::mem::ManuallyDrop::new(Some(Box::new(DemandingBufferRetirement { buffer: Some(buffer) }))),
            phase: ArtifactStoreVcsRetirementPhase::Complete, initial_snapshot_factory: std::mem::ManuallyDrop::new(Some(Arc::new(DemoInitialSnapshotRetirementFactory))), mutation_factory: std::mem::ManuallyDrop::new(Some(Arc::new(DemoMutationRetirementFactory))), closing_factories: std::mem::ManuallyDrop::new([None,None]),
        };
        let demand = owner.next_release_byte_demand().unwrap();
        let caller = row["callerBytes"].as_u64().unwrap() as usize;
        let caller_grant = RetainedCloneGrant { maximum_release_bytes: caller, maximum_capacity_bytes: admission, ..physical_test_close_grant() };
        let (caller_step, _, caller_released) = semio_framework_trace_memory_testing::observe_backing(|| owner.close_step(caller_grant).unwrap());
        let caller_expected = row["releasedBytes"].as_u64().unwrap() as usize;
        let mut progress = vec![(caller_step, caller_released)];
        for _ in 0..512 { if owner.terminal_is_empty() { break; } let grant=RetainedCloneGrant { maximum_capacity_bytes:admission,maximum_release_bytes:admission,..physical_test_close_grant() };let (step, _, released) = semio_framework_trace_memory_testing::observe_backing(|| owner.close_step(grant).unwrap());assert!(step.progress().fits(grant));progress.push((step, released)); }
        let terminal = owner.terminal_is_empty();
        for _ in 0..1024 { if owner.terminal_is_empty() { break; } owner.close_step(RetainedCloneGrant { maximum_capacity_bytes:admission,maximum_release_bytes:admission,..physical_test_close_grant() }).unwrap(); }
        assert!(owner.terminal_is_empty());
        let exact_accounting = progress.iter().all(|(step, actual)| step.progress().released_bytes == *actual);
        if demand != extent || !terminal || !exact_accounting || caller_released != caller_expected { failures.push((extent, demand, terminal, exact_accounting)); }
    }
    assert!(failures.is_empty(), "VCS retirement publishes and funds exact nested physical allocations: {failures:?}");
}

/// 📋️ The fresh envelope field driver forwards its retained target's physical release demand.
#[test]
fn fresh_field_release_propagates_retained_target_allocation_demand() {
    #[derive(semio_framework_value::FactoryPayloadRetirement)]
    struct ReleaseOnlyCatalog;
    impl ArtifactEnvelopeOwnedFieldCatalog<DemoSnapshot, DemoMutation> for ReleaseOnlyCatalog {
        fn begin_vcs(&self, _: semio_framework_job::OperationId, _: semio_framework_job::Generation, _: OwnedSchemaPath) -> Result<Box<dyn ArtifactEnvelopeVcsFieldAuthority<DemoSnapshot, DemoMutation>>, Box<dyn ArtifactEnvelopeSnapshotFieldAuthority<DemoSnapshot>>> { panic!("release-only catalog") }
        fn maximum_vcs_close_byte_demand(&self) -> usize { ARTIFACT_ENVELOPE_DECODE_MAXIMUM_CLOSE_ALLOCATION_BYTES }
        fn maximum_retained_vcs_close_bytes(&self) -> usize { ARTIFACT_ENVELOPE_DECODE_MAXIMUM_RETAINED_VCS_BYTES }
        fn begin_snapshot(&self, _: semio_framework_job::OperationId, _: semio_framework_job::Generation, _: OwnedSchemaPath) -> Box<dyn ArtifactEnvelopeSnapshotFieldAuthority<DemoSnapshot>> { panic!("release-only catalog") }
        fn begin_mutation(&self, _: semio_framework_job::OperationId, _: semio_framework_job::Generation, _: OwnedSchemaPath) -> Box<dyn ArtifactEnvelopeMutationFieldAuthority<DemoMutation>> { panic!("release-only catalog") }
        fn begin_spr_conflict(&self, _: semio_framework_job::OperationId, _: semio_framework_job::Generation, _: OwnedSchemaPath) -> Box<dyn ArtifactEnvelopeSprConflictAuthority> { panic!("release-only catalog") }
        fn edit_history_decoder(&self) -> Arc<dyn ArtifactOwnedHistoryEntryDecoder<Edit<DemoMutation>>> { panic!("release-only catalog") }
    }
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../../../../../../../🔨️modules/🧵️job/🧪️tests/🧫️fixtures/📏️close-demand/🔣️.json")).unwrap();
    let admission = fixture["admissionBytes"].as_u64().unwrap() as usize;
    let mut failures = Vec::new();
    for row in fixture["cases"].as_array().unwrap() {
        let extent = row["physicalBytes"].as_u64().unwrap() as usize;
        let mut buffer = Vec::new(); buffer.try_reserve_exact(extent).unwrap(); assert_eq!(buffer.capacity(), extent);
        let mut owner = ArtifactEnvelopeFreshFieldDecoder::new(semio_framework_job::OperationId(7002), semio_framework_job::Generation(1), Arc::new(ReleaseOnlyCatalog), Arc::new(DemoInitialSnapshotRetirementFactory), Arc::new(DemoMutationRetirementFactory), ArtifactEnvelopeCompletedRecordRegistry::new(), ArtifactEnvelopeDecodeCompletion::new());
        owner.active_retirement = Some(Box::new(DemandingBufferRetirement { buffer: Some(buffer) }));
        let demand = owner.next_close_release_byte_demand().unwrap();
        let caller = row["callerBytes"].as_u64().unwrap() as usize;
        let (caller_step, _, caller_released) = semio_framework_trace_memory_testing::observe_backing(|| owner.close_step(RetainedCloneGrant { maximum_items: 1, maximum_release_bytes: caller, maximum_depth: 16, ..Default::default() }).unwrap());
        let caller_expected = row["releasedBytes"].as_u64().unwrap() as usize;
        let mut progress = vec![(caller_step, caller_released)];
        for _ in 0..8 { if owner.terminal_is_empty() { break; } let grant = RetainedCloneGrant { maximum_capacity_bytes: admission, maximum_release_bytes: admission, ..physical_test_close_grant() }; let (step, _, released) = semio_framework_trace_memory_testing::observe_backing(|| owner.close_step(grant).unwrap()); progress.push((step, released)); }
        let terminal = owner.terminal_is_empty();
        for _ in 0..1024 { if owner.terminal_is_empty() { break; } let grant = RetainedCloneGrant { maximum_capacity_bytes: admission, maximum_release_bytes: admission, ..physical_test_close_grant() }; owner.close_step(grant).unwrap(); }
        assert!(owner.terminal_is_empty());
        let exact_accounting = progress.iter().all(|(step, actual)| step.progress().released_bytes == *actual);
        if demand != extent || !terminal || !exact_accounting || caller_released != caller_expected { failures.push((extent, demand, terminal, exact_accounting)); }
    }
    assert!(failures.is_empty(), "fresh field release publishes and funds exact retained target allocations: {failures:?}");
}

/// 🧱️ A terminal typed snapshot cursor retains its actual scaffold allocation until its whole physical grant arrives.
#[test]
fn fresh_vcs_preserves_terminal_snapshot_cursor_box_under_grant() {
    struct TerminalSnapshotCursor<const N: usize>([u8; N]);
    impl<const N: usize> ArtifactEnvelopeSnapshotFieldAuthority<DemoSnapshot> for TerminalSnapshotCursor<N> {
        fn accept_token(&mut self, _: OwnedSchemaToken, _: bool, _: &OwnedSchemaRecordCursor, _: &mut semio_framework_job::StepContext<'_>) -> Result<ArtifactEnvelopeFieldDecodeStep, OwnedSchemaDecodeDiagnostic> { panic!("terminal cursor has no decode work") }
        fn publish_reserved(&mut self, _: &mut dyn ArtifactEnvelopeSnapshotFieldTarget<DemoSnapshot>, _: ArtifactEnvelopeFieldReservation, _: &mut semio_framework_job::StepContext<'_>) -> Result<ArtifactEnvelopeFieldDecodeStep, OwnedSchemaDecodeDiagnostic> { panic!("terminal cursor has no publish work") }
        fn maximum_close_byte_demand(&self) -> usize { N }
        fn maximum_retained_close_bytes(&self) -> usize { N }
        fn next_close_copy_byte_demand(&self) -> Result<usize, OwnedSchemaDecodeDiagnostic> { Ok(0) }
        fn next_close_capacity_byte_demand(&self, _: usize) -> Result<usize, OwnedSchemaDecodeDiagnostic> { Ok(0) }
        fn next_close_release_byte_demand(&self) -> Result<usize, OwnedSchemaDecodeDiagnostic> { Ok(0) }
        fn next_close_depth_demand(&self) -> Result<usize, OwnedSchemaDecodeDiagnostic> { Ok(0) }
        fn close_step(&mut self, _: RetainedCloneGrant) -> Result<RetainedCloneStep, OwnedSchemaDecodeDiagnostic> { Ok(RetainedCloneStep::Complete(Default::default())) }
        fn terminal_is_empty(&self) -> bool { true }
    }
    #[derive(semio_framework_value::FactoryPayloadRetirement)]
    struct UnusedEditDecoder;
    impl ArtifactOwnedHistoryEntryDecoder<Edit<DemoMutation>> for UnusedEditDecoder {
        fn begin_entry(&self, _: semio_framework_job::OperationId, _: semio_framework_job::Generation, _: OwnedSchemaPath, _: Arc<dyn ArtifactOwnedValueRetirementFactory<Edit<DemoMutation>>>) -> Box<dyn ArtifactOwnedHistoryEntryAuthority<Edit<DemoMutation>>> { panic!("terminal snapshot fixture has no history decode work") }
    }
    fn check<const N: usize>(row: &serde_json::Value, admission: usize) -> (bool, usize, usize, usize, RetainedCloneStep) {
        let mut owner = ArtifactEnvelopeFreshVcsAuthority::try_new(Box::new(TerminalSnapshotCursor::<N>([0; N])), Arc::new(DemoInitialSnapshotRetirementFactory), Arc::new(DemoMutationRetirementFactory), Arc::new(UnusedEditDecoder)).unwrap_or_else(|_| panic!("bounded terminal snapshot scaffold admits"));
        let demand = owner.next_close_release_byte_demand().unwrap();
        let caller = row["callerBytes"].as_u64().unwrap() as usize;
        let (step, _, released) = semio_framework_trace_memory_testing::observe_backing(|| owner.close_step(RetainedCloneGrant { maximum_items: 1, maximum_release_bytes: caller, maximum_depth: 16, ..Default::default() }).unwrap());
        let retained = owner.snapshot.is_some();
        for _ in 0..1024 { if owner.terminal_is_empty() { break; } let grant = RetainedCloneGrant { maximum_capacity_bytes: admission, maximum_release_bytes: admission, ..physical_test_close_grant() }; owner.close_step(grant).unwrap(); }
        assert!(owner.terminal_is_empty());
        let expected = row["releasedBytes"].as_u64().unwrap() as usize;
        let reported = step.progress().released_bytes;
        (demand == N && released == expected && reported == released && retained == (expected == 0), N, demand, released, step)
    }
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../../../../../../../🔨️modules/🧵️job/🧪️tests/🧫️fixtures/📏️close-demand/🔣️.json")).unwrap();
    let admission = fixture["admissionBytes"].as_u64().unwrap() as usize;
    let mut results = Vec::new();
    for row in fixture["cases"].as_array().unwrap() { results.push(match row["physicalBytes"].as_u64().unwrap() { 1 => check::<1>(row, admission), 16384 => check::<16384>(row, admission), 65536 => check::<65536>(row, admission), 262144 => check::<262144>(row, admission), _ => panic!("explicit neutral cursor extents") }); }
    assert!(results.iter().all(|row| row.0), "typed terminal snapshot cursor requires exact physical funding: {results:?}");
}

/// 🪺️ The active history array exposes its exact nested edit decoder retirement and scaffold demand.
#[test]
fn fresh_vcs_active_history_decoder_propagates_physical_release_demand() {
    #[derive(semio_framework_value::FactoryPayloadRetirement)]
    struct ReleaseOnlyCatalog;
    impl ArtifactEnvelopeOwnedFieldCatalog<DemoSnapshot, DemoMutation> for ReleaseOnlyCatalog {
        fn begin_vcs(&self, _: semio_framework_job::OperationId, _: semio_framework_job::Generation, _: OwnedSchemaPath) -> Result<Box<dyn ArtifactEnvelopeVcsFieldAuthority<DemoSnapshot, DemoMutation>>, Box<dyn ArtifactEnvelopeSnapshotFieldAuthority<DemoSnapshot>>> { panic!("release-only catalog") }
        fn maximum_vcs_close_byte_demand(&self) -> usize { ARTIFACT_ENVELOPE_DECODE_MAXIMUM_CLOSE_ALLOCATION_BYTES }
        fn maximum_retained_vcs_close_bytes(&self) -> usize { ARTIFACT_ENVELOPE_DECODE_MAXIMUM_RETAINED_VCS_BYTES }
        fn begin_snapshot(&self, _: semio_framework_job::OperationId, _: semio_framework_job::Generation, _: OwnedSchemaPath) -> Box<dyn ArtifactEnvelopeSnapshotFieldAuthority<DemoSnapshot>> { panic!("release-only catalog") }
        fn begin_mutation(&self, _: semio_framework_job::OperationId, _: semio_framework_job::Generation, _: OwnedSchemaPath) -> Box<dyn ArtifactEnvelopeMutationFieldAuthority<DemoMutation>> { panic!("release-only catalog") }
        fn begin_spr_conflict(&self, _: semio_framework_job::OperationId, _: semio_framework_job::Generation, _: OwnedSchemaPath) -> Box<dyn ArtifactEnvelopeSprConflictAuthority> { panic!("release-only catalog") }
        fn edit_history_decoder(&self) -> Arc<dyn ArtifactOwnedHistoryEntryDecoder<Edit<DemoMutation>>> { panic!("release-only catalog") }
    }
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../../../../../../../🔨️modules/🧵️job/🧪️tests/🧫️fixtures/📏️close-demand/🔣️.json")).unwrap();
    let admission = fixture["admissionBytes"].as_u64().unwrap() as usize;
    let mut failures = Vec::new();
    for row in fixture["cases"].as_array().unwrap() {
        let extent = row["physicalBytes"].as_u64().unwrap() as usize;
        let mut backing = Vec::new(); backing.try_reserve_exact(extent).unwrap(); assert_eq!(backing.capacity(), extent);
        let catalog = Arc::new(ReleaseOnlyCatalog);
        let mutation_factory: Arc<dyn ArtifactOwnedValueRetirementFactory<DemoMutation>> = Arc::new(DemoMutationRetirementFactory);
        let edit_factory: Arc<dyn ArtifactOwnedValueRetirementFactory<Edit<DemoMutation>>> = Arc::new(ArtifactStoreDecodedEditRetirementFactory { mutation_factory: mutation_factory.clone() });
        let decoder: Arc<dyn ArtifactOwnedHistoryEntryDecoder<Edit<DemoMutation>>> = Arc::new(ArtifactOwnedSprEditDecoder { catalog: catalog.clone(), mutation_factory: mutation_factory.clone() });
        let mut edit = ArtifactOwnedSprEditAuthority::new(semio_framework_job::OperationId(7003), semio_framework_job::Generation(1), OwnedSchemaPath::ROOT, catalog.clone(), mutation_factory.clone(), edit_factory.clone());
        *edit.retirement = Some(Box::new(DemandingBufferRetirement { buffer: Some(backing) }));
        let mut array = OwnedSchemaBoundedArrayAuthority::new(OwnedSchemaPath::ROOT, edit_factory.clone(), decoder.clone());
        *array.values = None;
        *array.active_decoder = Some(Box::new(edit));
        let mut owner = ArtifactEnvelopeFreshVcsAuthority::<DemoSnapshot, DemoMutation> {
            cursor: OwnedSchemaNestedRecordCursor::try_new(OwnedSchemaRecordSpec { fields: ARTIFACT_ENVELOPE_FRESH_VCS_FIELDS }).unwrap(), snapshot: None, snapshot_target: ArtifactEnvelopeFreshSnapshotTarget::new(), genesis_pack: None, genesis_bytes: None,
            active: Some(ArtifactEnvelopeFreshVcsActive::Edits(array)), pending: None, edits: None, changes: None, checkpoints: None, alternatives: None, value: None, retirement: None,
            initial_snapshot_factory: Arc::new(DemoInitialSnapshotRetirementFactory), mutation_factory: mutation_factory.clone(), edit_decoder: decoder.clone(), maximum_snapshot_close_byte_demand: admission, maximum_retained_snapshot_close_bytes: admission, terminal: false,
        };
        let demand = owner.next_close_release_byte_demand().unwrap();
        let caller = row["callerBytes"].as_u64().unwrap() as usize;
        let (step, _, actual) = semio_framework_trace_memory_testing::observe_backing(|| owner.close_step(RetainedCloneGrant { maximum_items: 1, maximum_release_bytes: caller, maximum_depth: 16, ..Default::default() }).unwrap());
        let mut progress = vec![(step, actual)];
        for _ in 0..1024 { if owner.terminal_is_empty() { break; } let grant = RetainedCloneGrant { maximum_capacity_bytes: admission, maximum_release_bytes: admission, ..physical_test_close_grant() }; let (step, _, released) = semio_framework_trace_memory_testing::observe_backing(|| owner.close_step(grant).unwrap()); progress.push((step, released)); }
        let terminal = owner.terminal_is_empty();
        for _ in 0..1024 { if owner.terminal_is_empty() { break; } let grant = RetainedCloneGrant { maximum_capacity_bytes: admission, maximum_release_bytes: admission, ..physical_test_close_grant() }; owner.close_step(grant).unwrap(); }
        assert!(owner.terminal_is_empty());
        let exact = progress.iter().all(|(step, actual)| step.progress().released_bytes == *actual);
        if demand != extent || actual != row["releasedBytes"].as_u64().unwrap() as usize || !terminal || !exact { failures.push((extent, demand, terminal, exact)); }
    }
    assert!(failures.is_empty(), "active history decoder preserves and funds every exact retained allocation: {failures:?}");
}

/// 🧱️ Envelope and immutable genesis owners retain every terminal cursor until its exact physical grant arrives.
#[test]
fn envelope_and_genesis_preserve_terminal_cursor_physical_grants() {
    struct TerminalCursor<const N: usize>([u8; N]);
    impl<const N: usize> ErasedSnapshotRetirement for TerminalCursor<N> {
            fn next_demand(&self, body:usize)->Result<semio_framework_value::RetirementDemand,semio_framework_value::ValueError>{Ok(Default::default())}
fn close_step(&mut self, _: RetainedCloneGrant) -> Result<RetainedCloneStep, ValueError> { Ok(RetainedCloneStep::Complete(Default::default())) }
        fn terminal_is_empty(&self) -> bool { true }
        fn next_copy_byte_demand(&self)->Result<usize,ValueError>{Ok(0)}
        fn next_capacity_byte_demand(&self,_:usize)->Result<usize,ValueError>{Ok(0)}
        fn next_release_byte_demand(&self)->Result<usize,ValueError>{Ok(0)}
        fn next_depth_demand(&self)->Result<usize,ValueError>{Ok(0)}
    }
    #[derive(semio_framework_value::FactoryPayloadRetirement)]
    struct TerminalSnapshotFactory<const N: usize>;
    impl<const N: usize> ArtifactOwnedValueRetirementFactory<DemoSnapshot> for TerminalSnapshotFactory<N> {
        fn retirement_birth_bytes(&self,_:&DemoSnapshot)->usize { std::mem::size_of::<TerminalCursor<N>>() }
        fn retire_owned(&self,original:DemoSnapshot,grant:RetainedCloneGrant)->Result<(Box<dyn ErasedSnapshotRetirement>,RetainedCloneProgress),(ValueError,DemoSnapshot)> {
            admit_artifact_retirement(original,grant,|_|TerminalCursor::<N>([0;N]))
        }
    }
    fn check<const N: usize>(row: &serde_json::Value, admission: usize, failures: &mut Vec<(String, usize, usize, usize)>) {
        let grant=RetainedCloneGrant {maximum_capacity_bytes:admission,maximum_release_bytes:admission,..physical_test_close_grant()};
        let ((terminal,birth),heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||admit_artifact_retirement([0u8;N],grant,TerminalCursor::<N>).unwrap_or_else(|_|panic!("original fixed policy funds terminal cursor source")));
        assert!(birth.fits(grant));assert_eq!((heap.requested_bytes,heap.released_bytes),(birth.retained_capacity_bytes,0));
        let envelope=ArtifactStoreEnvelopeRetirement::<DemoSnapshot,DemoMutation> {vcs:std::mem::ManuallyDrop::new(None),messages:std::mem::ManuallyDrop::new(None),metadata:std::mem::ManuallyDrop::new(None),active:std::mem::ManuallyDrop::new(Some(terminal))};
        let (envelope,envelope_birth)=admit_artifact_retirement(envelope,grant,|original|original).unwrap_or_else(|_|panic!("fixed source policy funds original envelope frame"));assert!(envelope_birth.fits(grant));
        let pack=Vec::new();let digest=*semio_framework_hash::hash(&pack).as_bytes();
        let required=AdmittedArtifactGenesis::<DemoSnapshot>::verified_pack_birth_demand().unwrap();
        let ((error,original,pack),heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||AdmittedArtifactGenesis::admit_verified_pack(DemoSnapshot{n:Some(7)},pack,digest,RetainedCloneGrant {maximum_capacity_bytes:required.capacity_bytes-1,..grant}).unwrap_err());
        assert_eq!(error.kind,semio_framework_value::ValueRefusalKind::OwnershipLimit);assert_eq!(original.n,Some(7));assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));
        let ((genesis,genesis_birth),heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||AdmittedArtifactGenesis::admit_verified_pack(original,pack,digest,grant).unwrap_or_else(|_|panic!("actual fixed policy admits original verified Genesis")));
        assert_eq!((heap.requested_bytes,heap.released_bytes),(genesis_birth.retained_capacity_bytes,0));assert!(genesis_birth.fits(grant));
        let source_bytes=semio_framework_value::factory_arc_birth_bytes::<TerminalSnapshotFactory<N>>();
        semio_framework_value::retained_clone::RetainedCloneBirthDemand {capacity_bytes:source_bytes,depth:1}.admit(grant).unwrap();
        let (factory,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||Arc::new(TerminalSnapshotFactory::<N>));assert_eq!((heap.requested_bytes,heap.released_bytes),(source_bytes,0));
        let factory:Arc<dyn ArtifactOwnedValueRetirementFactory<DemoSnapshot>>=factory;
        let ((genesis,owner_birth),heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||admit_artifact_retirement((genesis,factory),grant,|(original,issuer)|ArtifactGenesisRetirement::new(original,issuer)).unwrap_or_else(|_|panic!("fixed policy admits actual Genesis retirement frame")));
        assert_eq!((heap.requested_bytes,heap.released_bytes),(owner_birth.retained_capacity_bytes,0));assert!(owner_birth.fits(grant));
        for (name,mut owner) in [("envelope",envelope),("genesis",genesis)] {
            let original=owner.as_ref() as *const dyn ErasedSnapshotRetirement as *const ();
            let mut found=false;
            for _ in 0..64 {
                if owner.next_release_byte_demand().unwrap()==N { found=true;break; }
                assert!(!owner.terminal_is_empty(),"real original owner must reach its actual terminal child frame");
                let (step,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||owner.close_step(grant).unwrap());assert!(step.progress().fits(grant));assert_eq!((heap.requested_bytes,heap.released_bytes),(step.progress().retained_capacity_bytes,step.progress().released_bytes));
            }
            assert!(found,"actual {name} reaches original {N}-byte terminal child frame");
            let demand=owner.next_release_byte_demand().unwrap();let caller=row["callerBytes"].as_u64().unwrap() as usize;
            let (step,allocated,actual)=semio_framework_trace_memory_testing::observe_backing(||owner.close_step(RetainedCloneGrant {maximum_release_bytes: caller,..grant}).unwrap());
            let reported=step.progress().released_bytes;let expected=row["releasedBytes"].as_u64().unwrap() as usize;
            assert_eq!(allocated,0);assert!(step.progress().fits(RetainedCloneGrant {maximum_release_bytes:caller,..grant}));
            if expected==0 { assert_eq!(owner.as_ref() as *const dyn ErasedSnapshotRetirement as *const (),original);assert_eq!(owner.next_release_byte_demand().unwrap(),N); }
            for _ in 0..64 {if owner.terminal_is_empty(){break;}let(step,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||owner.close_step(grant).unwrap());assert!(step.progress().fits(grant));assert_eq!((heap.requested_bytes,heap.released_bytes),(step.progress().retained_capacity_bytes,step.progress().released_bytes));}
            assert!(owner.terminal_is_empty());let shell=std::mem::size_of_val(owner.as_ref());assert!(grant.maximum_release_bytes>=shell);
            let (_,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||drop(owner));assert_eq!((heap.requested_bytes,heap.released_bytes),(0,shell));
            eprintln!("[DEBUG] actual {name} terminal child extent={N} demand={demand} caller={caller} actual={actual} reported={reported}; original pointer retained on denial, outer frame={shell} separately funded");
            if demand!=N||actual!=expected||reported!=actual {failures.push((name.into(),demand,actual,reported));}
        }
    }
    let fixture:serde_json::Value=serde_json::from_str(include_str!("../../../../../../../../🔨️modules/🧵️job/🧪️tests/🧫️fixtures/📏️close-demand/🔣️.json")).unwrap();let admission=fixture["admissionBytes"].as_u64().unwrap() as usize;let mut failures=Vec::new();
    for row in fixture["cases"].as_array().unwrap(){match row["physicalBytes"].as_u64().unwrap(){1=>check::<1>(row,admission,&mut failures),16384=>check::<16384>(row,admission,&mut failures),65536=>check::<65536>(row,admission,&mut failures),262144=>check::<262144>(row,admission,&mut failures),_=>panic!("explicit neutral cursor extents")}}
    assert!(failures.is_empty(),"actual original owners require same-turn independent release funding: {failures:?}");
}

/// 🧺️ The store preserves its terminal disposer allocation under an insufficient physical grant.
#[test]
fn displaced_store_owner_preserves_terminal_physical_grants() {
    struct TerminalOwner<const N: usize>([u8; N]);
    impl<const N: usize> ErasedSnapshotRetirement for TerminalOwner<N> {
            fn next_demand(&self, body:usize)->Result<semio_framework_value::RetirementDemand,semio_framework_value::ValueError>{Ok(Default::default())}
fn close_step(&mut self, _: RetainedCloneGrant) -> Result<RetainedCloneStep, ValueError> { Ok(RetainedCloneStep::Complete(Default::default())) }
        fn terminal_is_empty(&self) -> bool { true }
        fn next_copy_byte_demand(&self) -> Result<usize, ValueError> { Ok(0) }
        fn next_capacity_byte_demand(&self, _: usize) -> Result<usize, ValueError> { Ok(0) }
        fn next_release_byte_demand(&self) -> Result<usize, ValueError> { Ok(0) }
        fn next_depth_demand(&self) -> Result<usize, ValueError> { Ok(0) }
    }
    fn check<const N: usize>(row: &serde_json::Value) {
        let grant = physical_test_close_grant();
        let ((owner, birth), heap) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| admit_artifact_retirement([0u8; N], grant, TerminalOwner::<N>).unwrap_or_else(|_| panic!("declared independent test policy admits original terminal frame")));
        assert!(birth.fits(grant)); assert_eq!((heap.requested_bytes, heap.released_bytes), (birth.retained_capacity_bytes, 0));
        let mut queue = snapshot_read::ArtifactStoreDisplacedRetirements::new(); queue.push_reserved(owner);
        let original = queue.owners.front().unwrap().as_ref() as *const dyn ErasedSnapshotRetirement as *const ();
        assert_eq!(queue.demands(grant.maximum_copy_bytes).unwrap().release_bytes, N);
        let (zero, allocated, released) = semio_framework_trace_memory_testing::observe_backing(|| queue.close_step(RetainedCloneGrant { maximum_items: 0, ..grant }).unwrap());
        assert_eq!((allocated, released), (0, 0)); assert_eq!(zero.progress(), RetainedCloneProgress::default());
        let caller = row["callerBytes"].as_u64().unwrap() as usize; let expected = row["releasedBytes"].as_u64().unwrap() as usize;
        let (step, allocated, released) = semio_framework_trace_memory_testing::observe_backing(|| queue.close_step(RetainedCloneGrant { maximum_release_bytes: caller, ..grant }).unwrap());
        assert_eq!((allocated, released), (0, expected)); assert_eq!(step.progress(), RetainedCloneProgress { copied_items: usize::from(expected != 0), released_bytes: expected, ..Default::default() });
        if expected == 0 {
            assert_eq!(queue.owners.front().unwrap().as_ref() as *const dyn ErasedSnapshotRetirement as *const (), original);
            assert_eq!(queue.demands(grant.maximum_copy_bytes).unwrap().release_bytes, N);
            let (step, allocated, released) = semio_framework_trace_memory_testing::observe_backing(|| queue.close_step(grant).unwrap());
            assert_eq!((allocated, released), (0, N)); assert!(step.progress().fits(grant)); assert_eq!(step.progress().released_bytes, N);
        }
        assert!(queue.terminal_is_empty()); assert_eq!(queue.demands(grant.maximum_copy_bytes).unwrap(), Default::default());
        let backing = queue.backing_release_byte_demand().unwrap();
        let (step, allocated, released) = semio_framework_trace_memory_testing::observe_backing(|| queue.release_empty_backing_step(grant).unwrap());
        assert!(matches!(step, RetainedCloneStep::Complete(_))); assert!(step.progress().fits(grant)); assert_eq!((allocated, released), (0, backing)); assert_eq!(step.progress().released_bytes, backing);
    }
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../../../../../../../🔨️modules/🧵️job/🧪️tests/🧫️fixtures/📏️close-demand/🔣️.json")).unwrap();
    for row in fixture["cases"].as_array().unwrap() { match row["physicalBytes"].as_u64().unwrap() { 1 => check::<1>(row), 16384 => check::<16384>(row), 65536 => check::<65536>(row), 262144 => check::<262144>(row), _ => panic!("declared neutral displaced-owner extents") } }
    let grant = physical_test_close_grant(); let mut queue = snapshot_read::ArtifactStoreDisplacedRetirements::new(); let reservation = queue.reserve_owner_slots(1).unwrap();
    assert_eq!(queue.close_step(grant).unwrap_err().kind, semio_framework_value::ValueRefusalKind::WorkLimit);
    assert_eq!(queue.demands(grant.maximum_copy_bytes).unwrap_err().kind, semio_framework_value::ValueRefusalKind::WorkLimit);
    queue.release_owner_slots(reservation).unwrap(); assert!(queue.terminal_is_empty()); assert!(queue.release_empty_backing_step(grant).unwrap().progress().fits(grant));
}

/// 🧺️ The store preserves its terminal disposer allocation under an insufficient physical grant.
#[semio_framework_async_macros::async_test]
async fn installed_store_disposer_preserves_terminal_physical_grant() {
    struct TerminalDisposer<const N: usize>([u8; N]);
    impl<const N: usize> ArtifactStoreOwnedDisposer<DemoSnapshot, DemoMutation> for TerminalDisposer<N> {
        fn close_step(&mut self, _: &mut ArtifactStoreCloseView<'_, DemoSnapshot, DemoMutation>, _: RetainedCloneGrant) -> Result<RetainedCloneStep, ValueError> { Ok(RetainedCloneStep::Complete(Default::default())) }
        fn terminal_is_empty(&self, _: &dyn semio_framework_os_store_engine_owners::lifted_store::ArtifactStoreCloseHost<DemoSnapshot, DemoMutation>) -> bool { true }
        fn demands(&self, _: &dyn semio_framework_os_store_engine_owners::lifted_store::ArtifactStoreCloseHost<DemoSnapshot, DemoMutation>, _: usize) -> Result<semio_framework_value::RetirementDemand, ValueError> { Ok(Default::default()) }
        fn close_uninstalled_step(&mut self, _: RetainedCloneGrant) -> Result<RetainedCloneStep, ValueError> { Ok(RetainedCloneStep::Complete(Default::default())) }
        fn uninstalled_demands(&self, _: usize) -> Result<semio_framework_value::RetirementDemand, ValueError> { Ok(Default::default()) }
        fn uninstalled_terminal_is_empty(&self) -> bool { true }
    }
    impl<const N: usize> ErasedSnapshotRetirement for TerminalDisposer<N> {
            fn next_demand(&self, body:usize)->Result<semio_framework_value::RetirementDemand,semio_framework_value::ValueError>{Ok(Default::default())}
fn close_step(&mut self, _: RetainedCloneGrant) -> Result<RetainedCloneStep, ValueError> { Ok(RetainedCloneStep::Complete(Default::default())) }
        fn terminal_is_empty(&self) -> bool { true }
        fn next_copy_byte_demand(&self) -> Result<usize, ValueError> { Ok(0) }
        fn next_capacity_byte_demand(&self, _: usize) -> Result<usize, ValueError> { Ok(0) }
        fn next_release_byte_demand(&self) -> Result<usize, ValueError> { Ok(0) }
        fn next_depth_demand(&self) -> Result<usize, ValueError> { Ok(0) }
    }
    async fn check<const N: usize>(row: &serde_json::Value, admission: usize, failures: &mut Vec<(usize, usize, usize, usize)>) {
        let envelope = create_document_envelope::<DemoSnapshot, DemoMutation>("demo/v1", "terminal-disposer", DemoSnapshot { n: Some(0) }, None, &semio_framework_os_vcs::io::binary::genesis::ArtifactPackGenesisProducer);
        let mut store = semio_framework_os_store_engine::lifted_store::ArtifactStore::new::<semio_framework_os_store_initialization_runtime::native_publication_producer::ArtifactSnapshotPublication>(envelope, ActorId(semio_framework_os_spr::LOCAL_ACTOR_ID.into())).await.unwrap();
        store.install_document_store_owners_exact(complete_test_catalog(semio_framework_os_store_testing_doubles::DemoMemberStoreProducer::<_>::new().member_store_owners(physical_test_close_grant()))).unwrap_or_else(|(error,_)|panic!("funded complete test catalog install refused: {error}"));
        close_test_store(&mut store);
        let caller = row["callerBytes"].as_u64().unwrap() as usize;
        let expected = row["releasedBytes"].as_u64().unwrap() as usize;
        let mut cursor = ArtifactStoreCursorDisposer::<DemoSnapshot, DemoMutation>::new();
        cursor.phase = ArtifactStoreCursorDisposerPhase::Complete;
        *cursor.active = Some(Box::new(TerminalDisposer::<N>([0; N])));
        let nested_demand = cursor.demands(&store, 0).unwrap().release_bytes;
        let original = cursor.active.as_ref().unwrap().as_ref() as *const dyn ErasedSnapshotRetirement as *const ();
        let grant_for = |maximum_items, maximum_release_bytes| RetainedCloneGrant { maximum_items, maximum_copy_bytes: 0, maximum_capacity_bytes: 0, maximum_release_bytes, maximum_depth: 1 };
        let (step, requested, released) = semio_framework_trace_memory_testing::observe_backing(|| cursor.close_step(&mut ArtifactStoreCloseView { store: &mut store }, grant_for(0, N)).unwrap());
        assert_eq!(step.progress(), RetainedCloneProgress::default());
        assert_eq!((requested, released), (0, 0));
        assert_eq!(cursor.active.as_ref().unwrap().as_ref() as *const dyn ErasedSnapshotRetirement as *const (), original);
        let (nested_step, _, nested_actual) = semio_framework_trace_memory_testing::observe_backing(|| cursor.close_step(&mut ArtifactStoreCloseView { store: &mut store }, grant_for(1, caller)).unwrap());
        let nested_report = nested_step.progress().released_bytes;
        if expected == 0 { assert_eq!(cursor.active.as_ref().unwrap().as_ref() as *const dyn ErasedSnapshotRetirement as *const (), original); }
        for _ in 0..8 { if cursor.active.is_none() { break; } cursor.close_step(&mut ArtifactStoreCloseView { store: &mut store }, grant_for(1, admission)).unwrap(); }
        assert!(cursor.active.is_none());
        if nested_demand != N || nested_actual != expected || nested_report != nested_actual { failures.push((N, nested_demand, nested_actual, nested_report)); }
        *store.owners.owned_disposer = Some(Box::new(TerminalDisposer::<N>([0; N])));
        store.owners.owned_disposer_terminal = false;
        let demand = store.close_owned_demands(0).unwrap().release_bytes;
        let caller = row["callerBytes"].as_u64().unwrap() as usize;
        let (step, _, actual) = semio_framework_trace_memory_testing::observe_backing(|| store.close_owned_step(grant_for(1, caller)).unwrap());
        let reported = step.progress().released_bytes;
        let expected = row["releasedBytes"].as_u64().unwrap() as usize;
        for _ in 0..8 { if store.close_owned_terminal_is_empty() { break; } store.close_owned_step(grant_for(1, admission)).unwrap(); }
        assert!(store.close_owned_terminal_is_empty());
        if demand != N || actual != expected || reported != actual { failures.push((N, demand, actual, reported)); }
    }
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../../../../../../../🔨️modules/🧵️job/🧪️tests/🧫️fixtures/📏️close-demand/🔣️.json")).unwrap();
    let admission = fixture["admissionBytes"].as_u64().unwrap() as usize;
    let mut failures = Vec::new();
    for row in fixture["cases"].as_array().unwrap() { match row["physicalBytes"].as_u64().unwrap() { 1 => check::<1>(row, admission, &mut failures).await, 16384 => check::<16384>(row, admission, &mut failures).await, 65536 => check::<65536>(row, admission, &mut failures).await, 262144 => check::<262144>(row, admission, &mut failures).await, _ => panic!("explicit neutral disposer extents") } }
    assert!(failures.is_empty(), "the terminal store cursor requires exact physical funding: {failures:?}");
}
