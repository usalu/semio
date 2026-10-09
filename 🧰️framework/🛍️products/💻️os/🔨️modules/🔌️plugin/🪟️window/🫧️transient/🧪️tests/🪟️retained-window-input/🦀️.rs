//! 🧪️ Retained work reads the exact captured window transient generation.

use super::*;

fn original_transient_grant() -> RetainedCloneGrant { RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: 65_536, maximum_capacity_bytes: 65_536, maximum_release_bytes: 65_536, maximum_depth: 64 } }

#[test]
fn window_config_paged_registry_actual_transient_partition_custody() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../../🎚️config/🗂️registry/🧫️fixtures/🔣️.json")).unwrap();
    let copies = fixture["owningCopies"].as_u64().unwrap() as usize;
    let (mut registry, birth) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| {
        let mut registry = WindowTransientOwnerRegistry::for_document_generation(9);
        registry.register::<ReplacementWindow>().unwrap();
        for address in fixture["addresses"].as_array().unwrap() {
            for ordinal in 0..copies {
                let address = format!("{}/{ordinal:06}", address.as_str().unwrap().repeat(fixture["addressRepeat"].as_u64().unwrap() as usize));
                drop(registry.owners.get_mut(ReplacementWindow::WINDOW_KIND_ID).unwrap().as_mut().unwrap().capture(&address, 9).unwrap());
            }
        }
        registry
    });
    let mut allocated = birth.requested_bytes;
    let mut released = birth.released_bytes;
    let mut turns = 0;
    while !registry.terminal_is_empty() {
        let body = fixture["maximumPageBytes"].as_u64().unwrap() as usize;
        let demand = registry.retirement_demands(body).unwrap();
        let grant = RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: demand.copy_bytes.max(body), maximum_capacity_bytes: demand.capacity_bytes, maximum_release_bytes: demand.release_bytes, maximum_depth: demand.depth };
        let mut denied = vec![RetainedCloneGrant { maximum_items: 0, ..grant }];
        if demand.copy_bytes > 0 { denied.push(RetainedCloneGrant { maximum_copy_bytes: demand.copy_bytes - 1, ..grant }); }
        if demand.capacity_bytes > 0 { denied.push(RetainedCloneGrant { maximum_capacity_bytes: demand.capacity_bytes - 1, ..grant }); }
        if demand.release_bytes > 0 { denied.push(RetainedCloneGrant { maximum_release_bytes: demand.release_bytes - 1, ..grant }); }
        if demand.depth > 0 { denied.push(RetainedCloneGrant { maximum_depth: demand.depth - 1, ..grant }); }
        for denied in denied {
            let (step, heap) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| registry.close_step(denied));
            assert_eq!(step.unwrap().progress(), Some(RetainedCloneProgress::default()));
            assert_eq!((heap.requested_bytes, heap.released_bytes), (0, 0));
        }
        let (step, heap) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| registry.close_step(grant));
        let progress = step.unwrap().progress().unwrap();
        assert!(progress.fits(grant));
        assert_eq!((heap.requested_bytes, heap.released_bytes), (progress.retained_capacity_bytes, progress.released_bytes));
        allocated += heap.requested_bytes;
        released += heap.released_bytes;
        turns += 1;
        assert!(turns < 2_000_000);
    }
    assert_eq!(allocated, released);
    let (_, heap) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| drop(registry));
    assert_eq!((heap.requested_bytes, heap.released_bytes), (0, fixture["terminalDropBytes"].as_u64().unwrap() as usize));
    eprintln!("[DEBUG] actual original transient partitions={} allocation={allocated} release={released} turns={turns} terminalDrop0", copies * 4);
}

#[test]
fn retained_window_input_preserves_owner_generation() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/🪟️retained-window-input/🔣️.json")).unwrap();
    let mut identities = std::collections::BTreeSet::new();
    let mut digests = std::collections::BTreeSet::new();
    let mut owner = WindowTransientStore::<ReplacementWindow>::new(Default::default());
    for row in fixture["cases"].as_array().unwrap() {
        let snapshot = WindowTransientSnapshot {
            window_id: row["windowId"].as_str().unwrap().into(),
            window_kind_id: if row["windowKindId"] == "canvas" { "canvas" } else { "world" },
            generation: row["generation"].as_u64().unwrap(),
            document_generation: row["documentGeneration"].as_u64().unwrap(),
            snapshot: Arc::new(owner.current_read_erased().unwrap()),
        };
        assert_eq!(snapshot.generation(), row["generation"].as_u64().unwrap());
        let digest = crate::app::test_window_transient_context_identity(Some(&snapshot));
        assert_eq!(digest, crate::app::test_window_transient_context_identity(Some(&snapshot.clone())));
        assert_ne!(digest, crate::app::test_window_transient_context_identity(None));
        digests.insert(digest);
        identities.insert((snapshot.window_id().to_owned(), snapshot.window_kind_id().to_owned(), snapshot.generation(), snapshot.document_generation()));
    }
    assert_eq!(identities.len(), fixture["expectedUniqueOwners"].as_u64().unwrap() as usize);
    assert_eq!(digests.len(), identities.len());
    let mut disposer = transient_store_disposer::<_, crate::publication_fixture::PublicationTransientMutation>(ReplacementWindow::build_owners().state_retirement);
    for _ in 0..128 {
        if matches!(disposer.close_step(&mut owner, original_transient_grant()).unwrap(), PluginLifecycleStep::Complete(_)) {
            break;
        }
    }
    assert!(disposer.terminal_is_empty(&owner));
}

struct ReplacementWindow;

impl WindowTransientOwner for ReplacementWindow {
    const WINDOW_KIND_ID: &'static str = "canvas";
    type State = crate::publication_fixture::PublicationTransient;
    type Mutation = crate::publication_fixture::PublicationTransientMutation;

    fn build_owners() -> WindowTransientOwnerBundle<Self::State, Self::Mutation> {
        crate::window_transient_owners::owners()
    }
}

#[test]
fn retained_window_input_replacement_rejects_old_authority_and_publication() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/🪟️retained-window-input/🔣️.json")).unwrap();
    let expected = &fixture["documentReplacement"];
    let mut old = WindowTransientOwnerRegistry::default();
    old.register::<ReplacementWindow>().unwrap();
    let view = |id: &str| ViewModel {
        window_id: Some(id.into()),
        window_instances: ["canvas-left", "canvas-right"].map(|id| semio_framework::ViewWindowInstance { id: id.into(), window_kind_id: "canvas".into() }).into(),
        ..ViewModel::new(semio_framework_ui_locale::Locale::En, semio_framework_ui_locale::Terminology::Native)
    };
    let mutation = |id: &str, revision| WindowTransientMutation::of::<ReplacementWindow>(id, crate::publication_fixture::ChangePublicationTransient { revision }.into());
    let grant = store::ArtifactStoreOneItemGrant { maximum_items: 1, maximum_copy_bytes: 65_536, maximum_capacity_bytes: 65_536, maximum_release_bytes: 65_536, maximum_depth: 64 };
    for (id, revision) in expected["before"].as_object().unwrap() {
        let authority = old.capture(Some(&view(id))).unwrap().unwrap();
        let mut publication = old.begin(semio_framework_job::OperationId(1), &authority, mutation(id, revision.as_u64().unwrap())).unwrap();
        for _ in 0..1024 {
            if matches!(old.advance(publication.as_mut(), grant).unwrap(), store::ArtifactStoreOneItemAdvance::Published(_)) {
                assert!(publication.acknowledge());
                break;
            }
        }
        publication.begin_close();
        for _ in 0..1024 {
            if matches!(publication.close_step(grant.retained_grant()).unwrap(), RetainedCloneStep::Complete(_)) {
                break;
            }
        }
        assert!(publication.terminal_is_empty());
        assert_eq!(old.capture(Some(&view(id))).unwrap().unwrap().snapshot.get::<ReplacementWindow>().unwrap().revision, revision.as_u64().unwrap());
    }
    let authority = old.capture(Some(&view("canvas-left"))).unwrap().unwrap();
    let mut pending = old.begin(semio_framework_job::OperationId(2), &authority, mutation("canvas-left", 9)).unwrap();
    let mut replacement = WindowTransientOwnerRegistry::for_document_generation(expected["documentGeneration"].as_u64().unwrap());
    replacement.register::<ReplacementWindow>().unwrap();
    assert_eq!(replacement.begin(semio_framework_job::OperationId(3), &authority, mutation("canvas-left", 9)).is_ok(), expected["oldPublicationAccepted"].as_bool().unwrap());
    assert!(replacement.advance(pending.as_mut(), grant).is_err());
    for _ in 0..1024 {
        if matches!(pending.close_step(grant.retained_grant()).unwrap(), RetainedCloneStep::Complete(_)) {
            break;
        }
    }
    assert!(pending.terminal_is_empty());
    let actual: serde_json::Map<String, serde_json::Value> = expected["after"]
        .as_object()
        .unwrap()
        .keys()
        .map(|id| {
            let snapshot = replacement.capture(Some(&view(id))).unwrap().unwrap().snapshot;
            assert_eq!(snapshot.document_generation(), 1);
            (id.clone(), serde_json::json!(snapshot.get::<ReplacementWindow>().unwrap().revision))
        })
        .collect();
    assert_eq!(serde_json::Value::Object(actual), expected["after"]);
    drop(authority);
    for registry in [&mut old, &mut replacement] {
        assert_eq!(registry.close_step(RetainedCloneGrant { maximum_items: 0, ..original_transient_grant() }).unwrap(), PluginLifecycleStep::Progress(Default::default()));
        assert!(!registry.terminal_is_empty());
        for _ in 0..2048 {
            if matches!(registry.close_step(original_transient_grant()).unwrap(), PluginLifecycleStep::Complete(_)) {
                break;
            }
        }
        assert!(registry.terminal_is_empty());
    }
}

struct PausedPartitionDisposer {
    paused: Arc<std::sync::atomic::AtomicBool>,
    inner: Box<dyn ArtifactOwnedDisposer<WindowTransientStore<ReplacementWindow>>>,
}

impl ArtifactOwnedDisposer<WindowTransientStore<ReplacementWindow>> for PausedPartitionDisposer {
    fn close_step(&mut self, owner: &mut WindowTransientStore<ReplacementWindow>, grant: RetainedCloneGrant) -> Result<PluginLifecycleStep, Fault> {
        if self.paused.load(std::sync::atomic::Ordering::Acquire) {
            return Ok(PluginLifecycleStep::AwaitingInput { reason: "original paused transient disposer" });
        }
        self.inner.close_step(owner, grant)
    }

    fn retirement_demands(&self, owner: &WindowTransientStore<ReplacementWindow>, body: usize) -> Result<RetirementDemand, ValueError> { self.inner.retirement_demands(owner, body) }

    fn terminal_is_empty(&self, owner: &WindowTransientStore<ReplacementWindow>) -> bool {
        !self.paused.load(std::sync::atomic::Ordering::Acquire) && self.inner.terminal_is_empty(owner)
    }
}

#[test]
fn retained_window_input_retirement_reaches_later_partitions_and_kinds() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/🪟️retained-window-input/🔣️.json")).unwrap();
    let fairness = &fixture["retirementFairness"];
    let paused = Arc::new(std::sync::atomic::AtomicBool::new(true));
    let mut owner = TypedWindowTransientStoreOwner::<ReplacementWindow> { partitions: WindowRegistry::new(), owners: Some(ReplacementWindow::build_owners()), factory_close: [None, None, None], partition_close_cursor: 0, partition_open: 0, partition_address: None };
    for id in fairness["owners"].as_array().unwrap() {
        owner.partition(id.as_str().unwrap());
    }
    owner.partitions.get_mut("first").unwrap().disposer = Some(Box::new(PausedPartitionDisposer { paused: paused.clone(), inner: transient_store_disposer(ReplacementWindow::build_owners().state_retirement) }));
    assert_eq!(owner.close_step(RetainedCloneGrant { maximum_items: 0, ..original_transient_grant() }).unwrap(), PluginLifecycleStep::Progress(Default::default()));
    assert_eq!(owner.partition_close_cursor, 0);
    for _ in 0..65_536 {
        owner.close_step(original_transient_grant()).unwrap();
        if owner.partitions.iter().filter(|(_, partition)| partition.disposer.is_some()).count() == 1 { break; }
    }
    assert_eq!(serde_json::to_value(owner.partitions.iter().filter(|(_, partition)| partition.disposer.is_some()).map(|(key, _)| key).collect::<Vec<_>>()).unwrap(), fairness["expectedSurvivors"]);
    let mut registry = WindowTransientOwnerRegistry::default();
    registry.owners.insert("first", Some(Box::new(owner)));
    registry.owners.insert("second", Some(Box::new(TypedWindowTransientStoreOwner::<ReplacementWindow> { partitions: WindowRegistry::new(), owners: Some(ReplacementWindow::build_owners()), factory_close: [None, None, None], partition_close_cursor: 0, partition_open: 0, partition_address: None })));
    registry.owner_open = 2;
    assert_eq!(registry.close_step(RetainedCloneGrant { maximum_items: 0, ..original_transient_grant() }).unwrap(), PluginLifecycleStep::Progress(Default::default()));
    assert_eq!(registry.owners.len(), 2);
    for _ in 0..65_536 {
        registry.close_step(original_transient_grant()).unwrap();
        if registry.owner_open == 1 { break; }
    }
    assert_eq!(serde_json::to_value(registry.owners.iter().filter(|(_, owner)| owner.is_some()).map(|(key, _)| key).collect::<Vec<_>>()).unwrap(), fairness["expectedSurvivors"]);
    paused.store(false, std::sync::atomic::Ordering::Release);
    for _ in 0..65_536 {
        if matches!(registry.close_step(original_transient_grant()).unwrap(), PluginLifecycleStep::Complete(_)) {
            break;
        }
    }
    assert!(registry.terminal_is_empty());
}

#[test]
fn retained_window_input_refresh_admits_live_generation_after_a_committed_write() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/🪟️retained-window-input/🔣️.json")).unwrap();
    let expected = &fixture["liveWriteGeneration"];
    let mut registry = WindowTransientOwnerRegistry::default();
    registry.register::<ReplacementWindow>().unwrap();
    let view = ViewModel {
        window_id: Some("canvas-left".into()),
        window_instances: vec![semio_framework::ViewWindowInstance { id: "canvas-left".into(), window_kind_id: "canvas".into() }],
        ..ViewModel::new(semio_framework_ui_locale::Locale::En, semio_framework_ui_locale::Terminology::Native)
    };
    let mutation = |revision| WindowTransientMutation::of::<ReplacementWindow>("canvas-left", crate::publication_fixture::ChangePublicationTransient { revision }.into());
    let grant = store::ArtifactStoreOneItemGrant { maximum_items: 1, maximum_copy_bytes: 65_536, maximum_capacity_bytes: 65_536, maximum_release_bytes: 65_536, maximum_depth: 64 };
    let publish = |registry: &mut WindowTransientOwnerRegistry, authority: &WindowTransientAuthority, revision: u64| {
        let mut publication = registry.begin(semio_framework_job::OperationId(1), authority, mutation(revision)).unwrap();
        for _ in 0..1024 {
            if matches!(registry.advance(publication.as_mut(), grant).unwrap(), store::ArtifactStoreOneItemAdvance::Published(_)) {
                assert!(publication.acknowledge());
                break;
            }
        }
        publication.begin_close();
        for _ in 0..1024 {
            if matches!(publication.close_step(grant.retained_grant()).unwrap(), RetainedCloneStep::Complete(_)) {
                break;
            }
        }
        assert!(publication.terminal_is_empty());
    };
    let captured = registry.capture(Some(&view)).unwrap().unwrap();
    publish(&mut registry, &captured, expected["firstRevision"].as_u64().unwrap());
    assert_eq!(registry.begin(semio_framework_job::OperationId(2), &captured, mutation(expected["staleRevision"].as_u64().unwrap())).is_ok(), expected["staleBeginAccepted"].as_bool().unwrap());
    let mut live = captured.clone();
    registry.refresh(&mut live).unwrap();
    assert_ne!(live.generation, captured.generation);
    assert!(expected["refreshedBeginAccepted"].as_bool().unwrap());
    publish(&mut registry, &live, expected["liveRevision"].as_u64().unwrap());
    drop((captured, live));
    for _ in 0..2048 {
        if matches!(registry.close_step(original_transient_grant()).unwrap(), PluginLifecycleStep::Complete(_)) {
            break;
        }
    }
    assert!(registry.terminal_is_empty());
}
