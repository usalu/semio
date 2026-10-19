/// 💳️ Member publication, denied axes and original allocation laws: the member owned publication birth, the denied owner and shell, the group visibility, the typed option birth, the catalog sources, the member source constructor and the ephemeral member preparation frame.

#[semio_framework_async_macros::async_test]
async fn member_owned_publication_birth_refuses_without_allocating_or_transferring_source() {
    use semio_framework_value::retained_clone::RetainedCloneGrant;
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../../../🧩️composition/📨️emission/📦️owned/🌱️birth/🧪️tests/🧫️fixtures/🔣️.json")).unwrap();
    for row in fixture["rows"].as_array().unwrap() {
        let mut member = ArtifactStore::bare(create_document_envelope::<DemoSnapshot, DemoMutation>("demo/v1", "birth-refusal", DemoSnapshot { n: Some(0) }, None, &semio_framework_os_vcs::io::binary::genesis::ArtifactPackGenesisProducer)).await;
        member.log.envelope.active_alternative_id = row["line"].as_str().map(str::to_owned);
        let mut factory = DemoOneItemPreparationFactory::admissible();
        factory.stamped_mutation_id = row["stampedId"].as_str().map(|id| MutationId(id.into()));
        member.install_document_store_owners_exact(complete_test_catalog(demo_base_store_owners().with_one_item_preparation(Arc::new(factory),physical_test_close_grant()))).unwrap_or_else(|(error,_)|panic!("funded complete test catalog install refused: {error}"));
        let grant = RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: 64, maximum_capacity_bytes: 4096, maximum_release_bytes: 4096, maximum_depth: 64 };
        let values = vec![DemoMutation::SetN(SetN { n: 8 }), DemoMutation::SetN(SetN { n: 9 })];
        let pointer = values.as_ptr();
        let (mutations, _) = MemberStoreOwnedBatch::try_new(values, grant).unwrap_or_else(|_| panic!("funded original source"));
        let mut request = MemberStoreOwnedBatchRequest { operation: semio_framework_job::OperationId(719), expected_generation: member.generation_now(), expected_revision: member.content_revision_now(), actor: row["actor"].as_str().unwrap().into(), group_id: row["group"].as_str().map(str::to_owned), transaction: row["transaction"].as_object().map(|value| protocol::TransactionRef { id: value["id"].as_str().unwrap().into(), tool: value["tool"].as_str().unwrap().into() }), mutations };
        let metadata = (request.actor.as_ptr(), request.group_id.as_ref().map(|value| value.as_ptr()), request.transaction.as_ref().map(|value| (value.id.as_ptr(), value.tool.as_ptr())));
        let (birth, query_allocated, query_released) = semio_framework_trace_memory_testing::observe_backing(|| member.owned_publication_birth_bytes(&request));
        let birth = birth.unwrap();
        assert_eq!((query_allocated, query_released), (0, 0));
        assert!(birth > row["metadataBytes"].as_u64().unwrap() as usize);
        for funding in [ArtifactStoreOneItemGrant { maximum_items: 0, maximum_capacity_bytes: birth, ..one_item_test_policy() }, ArtifactStoreOneItemGrant { maximum_items: 1, maximum_copy_bytes: 0, ..one_item_test_policy() }, ArtifactStoreOneItemGrant { maximum_items: 1, maximum_capacity_bytes: birth - 1, ..one_item_test_policy() }] {
            let (candidate, allocated, released) = semio_framework_trace_memory_testing::observe_backing(|| member.begin_one_item_owned_publication(&mut request, funding));
            assert!(candidate.unwrap().is_none());
            assert_eq!((allocated, released), (0, 0));
            assert_eq!(request.mutations.mutations::<DemoMutation>().unwrap().as_ptr(), pointer);
            assert_eq!((request.actor.as_ptr(), request.group_id.as_ref().map(|value| value.as_ptr()), request.transaction.as_ref().map(|value| (value.id.as_ptr(), value.tool.as_ptr()))), metadata);
            assert_eq!(request.mutations.mutations::<DemoMutation>().unwrap().len(), 2);
            assert_eq!(member.snapshot_ref().n, Some(0));
        }
        let exact = ArtifactStoreOneItemGrant { maximum_items: 1, maximum_capacity_bytes: birth, ..one_item_test_policy() };
        let (publication, allocated, released) = semio_framework_trace_memory_testing::observe_backing(|| member.begin_one_item_owned_publication(&mut request, exact));
        let mut publication = publication.unwrap().expect("exact birth funded");
        assert_eq!((allocated, released), (birth, 0));
        assert!(request.mutations.mutations::<DemoMutation>().is_none());
        publication.begin_close();
        for _ in 0..65_536 { if publication.terminal_is_empty() { break; } let step = publication.close_step(physical_test_close_grant()).unwrap(); assert!(step.progress().fits(physical_test_close_grant())); }
        assert!(publication.terminal_is_empty());
        for _ in 0..1024 { if request.mutations.terminal_is_empty() { break; } request.mutations.close_granted(grant).unwrap(); }
        assert!(request.mutations.terminal_is_empty());
        for _ in 0..65_536 { if matches!(SpaceMember::close_owned_step(&mut member, physical_test_close_grant()).unwrap(), RetainedCloneStep::Complete(_)) { break; } }
        assert!(SpaceMember::close_owned_terminal_is_empty(&member));
    }
}


#[test]
fn current_box_retirement_preserves_denied_owner_and_exact_physical_shell() {
    use semio_framework_value::retained_clone::{RetainedCloneGrant, RetainedCloneProgress, RetainedCloneStep};
    let policy = physical_test_close_grant();
    let original = String::with_capacity(4096);
    let pointer = original.as_ptr();
    let birth = std::mem::size_of::<ArtifactStoreStringRetirement>();
    let denied = RetainedCloneGrant { maximum_items: 1, maximum_depth: 1, maximum_capacity_bytes: birth - 1, ..Default::default() };
    let (refusal, requested, released) = semio_framework_trace_memory_testing::observe_backing(|| admit_artifact_retirement(original, denied, ArtifactStoreStringRetirement::new));
    let (error, original) = refusal.err().unwrap();
    assert_eq!(error.kind, semio_framework_value::ValueRefusalKind::OwnershipLimit);
    assert_eq!(original.as_ptr(), pointer);
    assert_eq!((requested, released), (0, 0));
    let (admission, requested, released) = semio_framework_trace_memory_testing::observe_backing(|| admit_artifact_retirement(original, policy, ArtifactStoreStringRetirement::new));
    let (owner, receipt) = admission.unwrap_or_else(|(error, _)| panic!("{error}"));
    assert_eq!(requested, birth);
    assert_eq!(released, 0);
    assert_eq!(receipt.retained_capacity_bytes, birth);
    let address = owner.as_ref() as *const dyn ErasedSnapshotRetirement as *const ();
    let mut slot = Some(owner);
    let current_depth = artifact_retirement_box_demands(slot.as_ref().unwrap(), 1).unwrap().depth;
    for depth in [0, 1, current_depth - 1] {
        let (result, requested, released) = semio_framework_trace_memory_testing::observe_backing(|| artifact_retirement_box_close_step(&mut slot, RetainedCloneGrant { maximum_items: 1, maximum_depth: depth, maximum_release_bytes: 4096, ..Default::default() }));
        assert_eq!(result.unwrap_err().kind, semio_framework_value::ValueRefusalKind::DepthLimit);
        assert_eq!((requested, released), (0, 0));
        assert_eq!(slot.as_ref().unwrap().as_ref() as *const dyn ErasedSnapshotRetirement as *const (), address);
    }
    let (step, requested, released) = semio_framework_trace_memory_testing::observe_backing(|| artifact_retirement_box_close_step(&mut slot, RetainedCloneGrant::default()).unwrap());
    assert_eq!(step, RetainedCloneStep::Progress(RetainedCloneProgress::default()));
    assert_eq!((requested, released), (0, 0));
    let mut freed = 0;
    let mut born = 4096 + birth;
    for _ in 0..128 {
        let Some(owner) = slot.as_ref() else { break; };
        let grant = policy;
        let terminal_before = owner.terminal_is_empty();
        let shell = std::mem::size_of_val(owner.as_ref());
        let (step, requested, released) = semio_framework_trace_memory_testing::observe_backing(|| artifact_retirement_box_close_step(&mut slot, grant).unwrap());
        assert!(step.progress().fits(grant));
        assert_eq!(step.progress().retained_capacity_bytes, requested);
        assert_eq!(step.progress().released_bytes, released);
        if terminal_before { assert_eq!(released, shell); assert!(slot.is_none()); }
        else { assert!(slot.is_some()); }
        freed += released;
        born += requested;
    }
    assert!(slot.is_none());
    assert_eq!(freed, born);
}

#[test]
fn artifact_store_group_visibility_retires_original_noncopy_atomic_with_exact_frame_receipt() {
    use semio_framework_value::retirement::{RetireOwned, RetirementStep};
    use semio_framework_value::retained_clone::RetainedCloneGrant;
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../../../♻️retirement/📦️backing/🧪️tests/🧫️fixtures/🔣️.json")).unwrap();
    for row in fixture["visibilityRetirement"].as_array().unwrap() {
        let mut publisher = semio_framework_os_vcs::ArtifactGroupVisibilityOwner::new();
        if row["committed"].as_bool().unwrap() { assert!(publisher.commit()); }
        let view = publisher.view();
        drop(publisher);
        let value = match Arc::try_unwrap(view) { Ok(value) => value, Err(_) => panic!("original publisher and view are uniquely closed") };
        assert_eq!(serde_json::Value::Bool(value.committed()), row["committed"]);
        assert_eq!(serde_json::Value::Bool(value.pending()), row["pending"]);
        let extent = value.retirement_birth_bytes().unwrap();
        let (mut cursor, allocated, released) = semio_framework_trace_memory_testing::observe_backing(|| value.retirement());
        assert_eq!((allocated, released), (extent, 0));
        let grant = RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: 1, maximum_capacity_bytes: 0, maximum_release_bytes: 0, maximum_depth: cursor.next_depth_demand().unwrap() };
        for denial in [RetainedCloneGrant { maximum_items: 0, ..grant }, RetainedCloneGrant { maximum_copy_bytes: 0, ..grant }] {
            let (step, allocated, released) = semio_framework_trace_memory_testing::observe_backing(|| cursor.close_step(denial));
            assert!(matches!(step, RetirementStep::BudgetExhausted));
            assert_eq!((allocated, released), (0, 0));
            assert!(!cursor.terminal_is_empty());
        }
        let mut copied = 0;
        for _ in 0..8 {
            let (step, allocated, released) = semio_framework_trace_memory_testing::observe_backing(|| cursor.close_step(grant));
            assert_eq!((allocated, released), (0, 0));
            match step { RetirementStep::ProcessedBytes(bytes) => { assert!(bytes <= 1); copied += bytes; }, RetirementStep::Complete => break, _ => panic!("atomic-only owner has no hidden heap work") }
        }
        assert_eq!(copied, std::mem::size_of::<semio_framework_os_vcs::ArtifactGroupVisibility>());
        assert!(cursor.terminal_is_empty());
        assert_eq!(cursor.terminal_release_bytes(), Some(extent));
        let (_, allocated, released) = semio_framework_trace_memory_testing::observe_backing(|| drop(cursor));
        assert_eq!((allocated, released), (0, extent));
    }
}

#[test]
fn current_typed_option_birth_retains_exact_original_and_paid_shell() {
    use semio_framework_value::retained_clone::{RetainedCloneGrant, RetainedCloneStep};
    let original = String::with_capacity(4096);
    let pointer = original.as_ptr();
    let mut pending = Some(original);
    let mut active = None;
    let demand = artifact_retirement_owned_birth_demands(&pending).unwrap();
    let paid = physical_test_close_grant();
    for denied in [
        RetainedCloneGrant { maximum_items: 0, ..paid },
        RetainedCloneGrant { maximum_capacity_bytes: demand.capacity_bytes - 1, ..paid },
        RetainedCloneGrant { maximum_depth: demand.depth - 1, ..paid },
    ] {
        let (result, born, freed) = semio_framework_trace_memory_testing::observe_backing(|| artifact_retirement_admit_owned(&mut pending, &mut active, denied));
        if let Ok(step) = result { assert_eq!(step.progress(), Default::default()); }
        assert_eq!((born, freed), (0, 0));
        assert_eq!(pending.as_ref().unwrap().as_ptr(), pointer);
        assert_eq!(pending.as_ref().unwrap().capacity(), 4096);
        assert!(active.is_none());
    }
    let (step, born, freed) = semio_framework_trace_memory_testing::observe_backing(|| artifact_retirement_admit_owned(&mut pending, &mut active, paid).unwrap());
    assert_eq!(step.progress().retained_capacity_bytes, born);
    assert_eq!(born, demand.capacity_bytes);
    assert_eq!(freed, 0);
    assert!(pending.is_none());
    let mut total_freed = 0;
    let mut total_born = born;
    for _ in 0..128 {
        let Some(owner) = active.as_ref() else { break; };
        let grant = physical_test_close_grant();
        let (step, turn_born, turn_freed) = semio_framework_trace_memory_testing::observe_backing(|| artifact_retirement_box_close_step(&mut active, grant).unwrap());
        assert_eq!(step.progress().retained_capacity_bytes, turn_born);
        assert_eq!(step.progress().released_bytes, turn_freed);
        assert!(matches!(step, RetainedCloneStep::Progress(_) | RetainedCloneStep::Complete(_)));
        total_freed += turn_freed;
        total_born += turn_born;
    }
    assert!(active.is_none());
    assert_eq!(total_freed, 4096 + total_born);
}

#[test]
fn current_catalog_sources_admit_original_allocations_then_fund_each_ticket() {
    type Owners = DocumentStoreOwners<DemoSnapshot, DemoMutation>;
    type Disposer = ArtifactStoreCursorDisposer<DemoSnapshot, DemoMutation>;
    let capacity = Owners::source_birth_bytes::<DemoSnapshotRetirementFactory, DemoInitialSnapshotRetirementFactory, DemoMutationRetirementFactory, Disposer>().unwrap();
    let mut originals = Some((DemoSnapshotRetirementFactory, DemoInitialSnapshotRetirementFactory, DemoMutationRetirementFactory, Disposer::new()));
    let paid = physical_test_close_grant();
    for denied in [RetainedCloneGrant { maximum_items: 0, ..paid }, RetainedCloneGrant { maximum_depth: 0, ..paid }, RetainedCloneGrant { maximum_capacity_bytes: capacity - 1, ..paid }] {
        let (snapshots, initial, mutations, disposer) = originals.take().unwrap();
        let (refused, allocated, released) = semio_framework_trace_memory_testing::observe_backing(|| Owners::admit_sources(snapshots, initial, mutations, disposer, denied));
        assert_eq!((allocated, released), (0, 0));
        let Err((_, snapshots, initial, mutations, disposer)) = refused else { panic!("underfunded catalog changed its original source custody"); };
        assert!(!disposer.started);
        assert!(disposer.active.is_none());
        originals = Some((snapshots, initial, mutations, disposer));
    }
    let (snapshots, initial, mutations, disposer) = originals.take().unwrap();
    let (admitted, allocated, released) = semio_framework_trace_memory_testing::observe_backing(|| Owners::admit_sources(snapshots, initial, mutations, disposer, paid));
    let (mut owners, progress) = admitted.ok().expect("funded original catalog source ingress");
    assert_eq!(allocated, capacity);
    assert_eq!(released, 0);
    assert_eq!(progress.retained_capacity_bytes, allocated);
    assert_eq!(progress.copied_items, 1);
    assert!(!owners.constructor_is_complete());
    let mut constructor_turns = 0;
    for _ in 0..128 {
        if owners.constructor_is_complete() { break; }
        let grant = physical_test_close_grant();
        let (admitted, allocated, released) = semio_framework_trace_memory_testing::observe_backing(|| owners.admit_constructor(grant).unwrap());
        assert_eq!(admitted.copied_items, 1);
        assert_eq!(admitted.copied_bytes, 0);
        assert!(admitted.fits(grant));
        assert_eq!(admitted.retained_capacity_bytes, allocated);
        assert_eq!(released, 0);
        constructor_turns += 1;
    }
    assert!(constructor_turns >= 6);
    assert!(owners.constructor_is_complete());
    for _ in 0..128 {
        if owners.uninstalled_owners_terminal_is_empty() { break; }
        let grant = physical_test_close_grant();
        let (step, allocated, released) = semio_framework_trace_memory_testing::observe_backing(|| owners.close_uninstalled_owners_step(grant).unwrap());
        assert_eq!(step.progress().retained_capacity_bytes, allocated);
        assert_eq!(step.progress().released_bytes, released);
    }
    assert!(owners.uninstalled_owners_terminal_is_empty());
}

/// 🏗️ Verifies actual source construction is absent on denial and every later ticket/free is physically paid.
#[test]
fn current_member_source_constructor_is_not_invoked_under_denied_axes() {
    type Owners = DocumentStoreOwners<DemoSnapshot, DemoMutation>;
    type Disposer = ArtifactStoreCursorDisposer<DemoSnapshot, DemoMutation>;
    let capacity = Owners::source_birth_bytes::<DemoSnapshotRetirementFactory, DemoInitialSnapshotRetirementFactory, DemoMutationRetirementFactory, Disposer>().unwrap();
    let paid = physical_test_close_grant();
    let calls = std::cell::Cell::new(0usize);
    let construct = || {
        calls.set(calls.get() + 1);
        (DemoSnapshotRetirementFactory, DemoInitialSnapshotRetirementFactory, DemoMutationRetirementFactory, Disposer::new())
    };
    for denied in [
        RetainedCloneGrant { maximum_items: 0, ..paid },
        RetainedCloneGrant { maximum_capacity_bytes: capacity - 1, ..paid },
        RetainedCloneGrant { maximum_depth: 0, ..paid },
    ] {
        let (refused, allocated, released) = semio_framework_trace_memory_testing::observe_backing(|| Owners::admit_source_constructor(denied, construct));
        let refusal = refused.err().expect("underfunded source constructor is refused");
        assert!(refusal.owners.is_none());
        assert_eq!(refusal.progress, RetainedCloneProgress::default());
        assert_eq!(calls.get(), 0);
        assert_eq!(allocated, 0);
        assert_eq!(released, 0);
    }
    let (admitted, allocated, released) = semio_framework_trace_memory_testing::observe_backing(|| Owners::admit_source_constructor(paid, construct));
    let (mut owners, ingress) = admitted.ok().expect("funded source constructor");
    assert_eq!(calls.get(), 1);
    assert_eq!(allocated, capacity);
    assert_eq!(released, 0);
    assert_eq!(ingress.retained_capacity_bytes, allocated);
    assert_eq!(ingress.copied_items, 1);
    assert!(!owners.constructor_is_complete());
    let mut total_born = allocated;
    let mut total_released = 0usize;
    for _ in 0..128 {
        if owners.uninstalled_owners_terminal_is_empty() { break; }
        let grant = physical_test_close_grant();
        let (step, allocated, released) = semio_framework_trace_memory_testing::observe_backing(|| owners.close_uninstalled_owners_step(grant).unwrap());
        assert!(step.progress().fits(grant));
        assert_eq!(step.progress().retained_capacity_bytes, allocated);
        assert_eq!(step.progress().released_bytes, released);
        total_born += allocated;
        total_released += released;
    }
    assert!(owners.uninstalled_owners_terminal_is_empty());
    assert_eq!(total_born, total_released);
}

#[test]
fn ephemeral_member_original_preparation_frame_requires_exact_granted_release() {
    struct TerminalPreparation<const N: usize>([u8; N]);
    impl<const N: usize> ArtifactEphemeralOneItemPreparation<DemoSnapshot, DemoMutation> for TerminalPreparation<N> {
        fn advance(&mut self, _: ArtifactStoreOneItemGrant,receiving:&mut semio_framework_value::NativeDecodeControl<'_>) -> Result<ArtifactStoreOneItemPreparationStep, ValueError> { Ok(ArtifactStoreOneItemPreparationStep::Blocked) }
        fn checkpoint(&self) -> ArtifactStoreOneItemCheckpoint { Default::default() }
        fn prepared(&self) -> Option<&ArtifactEphemeralOneItemPrepared<DemoSnapshot>> { None }
        fn take_prepared(&mut self) -> Option<ArtifactEphemeralOneItemPrepared<DemoSnapshot>> { None }
        fn cancel(&mut self) {}
        fn begin_close(&mut self) {}
        fn close_step(&mut self, _: ArtifactStoreOneItemGrant) -> Result<RetainedCloneStep, ValueError> { Ok(RetainedCloneStep::Complete(Default::default())) }
        fn next_close_copy_byte_demand(&self) -> Result<usize, ValueError> { Ok(0) }
        fn next_close_capacity_byte_demand(&self, _: usize) -> Result<usize, ValueError> { Ok(0) }
        fn next_close_release_byte_demand(&self) -> Result<usize, ValueError> { Ok(0) }
        fn next_close_depth_demand(&self) -> Result<usize, ValueError> { Ok(0) }
        fn terminal_is_empty(&self) -> bool { true }
    }
    fn check<const N: usize>(row: &serde_json::Value) {
        let mut publication = ArtifactEphemeralOneItemPublication::<DemoSnapshot, DemoMutation>::closing_with_preparation(semio_framework_job::OperationId(1), Box::new(TerminalPreparation::<N>([0; N])));
        let pointer = publication.original_preparation().unwrap() as *const dyn ArtifactEphemeralOneItemPreparation<DemoSnapshot, DemoMutation> as *const ();
        let authority = |items,release,depth| ArtifactStoreOneItemGrant { maximum_items: items,maximum_copy_bytes: 0,maximum_capacity_bytes: 0,maximum_release_bytes: release,maximum_depth: depth };
        assert_eq!(publication.retirement_demands(0).unwrap(), semio_framework_value::RetirementDemand { release_bytes: N,depth: 1,..Default::default() });
        for denied in [authority(0,N,1),authority(1,N-1,1),authority(1,N,0)] {
            let (step,allocated,released) = semio_framework_trace_memory_testing::observe_backing(||publication.close_step(denied));
            assert_eq!((allocated,released),(0,0));
            if denied.maximum_depth==0 { assert_eq!(step.unwrap_err().kind,semio_framework_value::ValueRefusalKind::DepthLimit); }
            else { assert_eq!(step.unwrap().progress(),RetainedCloneProgress::default()); }
            assert_eq!(publication.original_preparation().unwrap() as *const dyn ArtifactEphemeralOneItemPreparation<DemoSnapshot, DemoMutation> as *const (),pointer);
        }
        let caller=row["callerBytes"].as_u64().unwrap() as usize;let expected=row["releasedBytes"].as_u64().unwrap() as usize;
        let(step,allocated,released)=semio_framework_trace_memory_testing::observe_backing(||publication.close_step(authority(1,caller,1)).unwrap());
        assert_eq!((allocated,released),(0,expected));assert_eq!(step.progress(),RetainedCloneProgress { copied_items: usize::from(expected!=0),released_bytes: expected,..Default::default() });
        if expected==0 {
            assert_eq!(publication.original_preparation().unwrap() as *const dyn ArtifactEphemeralOneItemPreparation<DemoSnapshot, DemoMutation> as *const (),pointer);
            let(step,allocated,released)=semio_framework_trace_memory_testing::observe_backing(||publication.close_step(authority(1,N,1)).unwrap());assert_eq!((allocated,released),(0,N));assert_eq!(step.progress().released_bytes,N);
        }
        assert!(publication.original_preparation().is_none());assert!(!publication.terminal_is_empty());
        let(step,allocated,released)=semio_framework_trace_memory_testing::observe_backing(||publication.close_step(authority(1,0,1)).unwrap());assert_eq!((allocated,released),(0,0));assert_eq!(step,RetainedCloneStep::Complete(RetainedCloneProgress { copied_items: 1,..Default::default() }));assert!(publication.terminal_is_empty());
    }
    let fixture:serde_json::Value=serde_json::from_str(include_str!("../../../../../../../../🔨️modules/🧵️job/🧪️tests/🧫️fixtures/📏️close-demand/🔣️.json")).unwrap();
    for row in fixture["cases"].as_array().unwrap(){match row["physicalBytes"].as_u64().unwrap(){1=>check::<1>(row),16384=>check::<16384>(row),65536=>check::<65536>(row),262144=>check::<262144>(row),_=>panic!("authored neutral original frame extent")}}
}
