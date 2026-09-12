//! 🧪️ Retained work reads the exact captured window transient generation.

use super::*;

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
        if disposer.close_step(&mut owner, 1, 4096).unwrap() == PluginCloseStep::Complete {
            break;
        }
    }
    assert!(disposer.terminal_is_empty(&owner));
    eprintln!("[DEBUG] retained window input keeps distinct concrete owner and generation tuples and retires its tracked read leases");
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
    let view = |id: &str| ViewModel { window_id: Some(id.into()), window_instances: ["canvas-left", "canvas-right"].map(|id| semio_framework::ViewWindowInstance { id: id.into(), window_kind_id: "canvas".into() }).into(), ..Default::default() };
    let mutation = |id: &str, revision| WindowTransientMutation::of::<ReplacementWindow>(id, crate::publication_fixture::ChangePublicationTransient { revision }.into());
    let grant = store::ArtifactStoreOneItemGrant { maximum_items: 1, maximum_bytes: 4096 };
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
            if publication.close_step(grant).unwrap() == store::SnapshotRetirementStep::Complete {
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
        if pending.close_step(grant).unwrap() == store::SnapshotRetirementStep::Complete {
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
        assert_eq!(registry.close_step(0, 4096).unwrap(), PluginCloseStep::Pending { released_items: 0, released_bytes: 0 });
        assert!(!registry.terminal_is_empty());
        for _ in 0..2048 {
            if registry.close_step(1, 4096).unwrap() == PluginCloseStep::Complete {
                break;
            }
        }
        assert!(registry.terminal_is_empty());
    }
    eprintln!("[DEBUG] document replacement resets both concrete windows, fences old publications, and retires every owner under one-item grants");
}

struct PausedPartitionDisposer {
    paused: Arc<std::sync::atomic::AtomicBool>,
    inner: Box<dyn ArtifactOwnedDisposer<WindowTransientStore<ReplacementWindow>>>,
}

impl ArtifactOwnedDisposer<WindowTransientStore<ReplacementWindow>> for PausedPartitionDisposer {
    fn close_step(&mut self, owner: &mut WindowTransientStore<ReplacementWindow>, maximum_items: usize, maximum_bytes: usize) -> Result<PluginCloseStep, Fault> {
        if self.paused.load(std::sync::atomic::Ordering::Acquire) {
            return Ok(PluginCloseStep::Pending { released_items: 0, released_bytes: 0 });
        }
        self.inner.close_step(owner, maximum_items, maximum_bytes)
    }

    fn terminal_is_empty(&self, owner: &WindowTransientStore<ReplacementWindow>) -> bool {
        !self.paused.load(std::sync::atomic::Ordering::Acquire) && self.inner.terminal_is_empty(owner)
    }
}

#[test]
fn retained_window_input_retirement_reaches_later_partitions_and_kinds() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/🪟️retained-window-input/🔣️.json")).unwrap();
    let fairness = &fixture["retirementFairness"];
    let paused = Arc::new(std::sync::atomic::AtomicBool::new(true));
    let mut owner = TypedWindowTransientStoreOwner::<ReplacementWindow> { partitions: BTreeMap::new(), owners: ReplacementWindow::build_owners(), maintenance_cursor: None, retirement_cursor: None };
    for id in fairness["owners"].as_array().unwrap() {
        owner.partition(id.as_str().unwrap());
    }
    owner.partitions.get_mut("first").unwrap().disposer = Some(Box::new(PausedPartitionDisposer { paused: paused.clone(), inner: transient_store_disposer(ReplacementWindow::build_owners().state_retirement) }));
    assert_eq!(owner.close_step(0, 4096).unwrap(), PluginCloseStep::Pending { released_items: 0, released_bytes: 0 });
    assert!(owner.retirement_cursor.is_none());
    for _ in 0..32 {
        owner.close_step(1, 4096).unwrap();
    }
    assert_eq!(serde_json::to_value(owner.partitions.keys().collect::<Vec<_>>()).unwrap(), fairness["expectedSurvivors"]);
    let mut registry = WindowTransientOwnerRegistry::default();
    registry.owners.insert("first", Box::new(owner));
    registry.owners.insert("second", Box::new(TypedWindowTransientStoreOwner::<ReplacementWindow> { partitions: BTreeMap::new(), owners: ReplacementWindow::build_owners(), maintenance_cursor: None, retirement_cursor: None }));
    assert_eq!(registry.close_step(0, 4096).unwrap(), PluginCloseStep::Pending { released_items: 0, released_bytes: 0 });
    assert!(registry.retirement_cursor.is_none());
    for _ in 0..8 {
        registry.close_step(1, 4096).unwrap();
    }
    assert_eq!(serde_json::to_value(registry.owners.keys().collect::<Vec<_>>()).unwrap(), fairness["expectedSurvivors"]);
    paused.store(false, std::sync::atomic::Ordering::Release);
    for _ in 0..32 {
        if registry.close_step(1, 4096).unwrap() == PluginCloseStep::Complete {
            break;
        }
    }
    assert!(registry.terminal_is_empty());
    eprintln!("[DEBUG] blocked partition and owner kind do not starve later owners; zero grants preserve both cursors");
}


#[test]
fn retained_window_input_refresh_admits_live_generation_after_a_committed_write() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/🪟️retained-window-input/🔣️.json")).unwrap();
    let expected = &fixture["liveWriteGeneration"];
    let mut registry = WindowTransientOwnerRegistry::default();
    registry.register::<ReplacementWindow>().unwrap();
    let view = ViewModel { window_id: Some("canvas-left".into()), window_instances: vec![semio_framework::ViewWindowInstance { id: "canvas-left".into(), window_kind_id: "canvas".into() }], ..Default::default() };
    let mutation = |revision| WindowTransientMutation::of::<ReplacementWindow>("canvas-left", crate::publication_fixture::ChangePublicationTransient { revision }.into());
    let grant = store::ArtifactStoreOneItemGrant { maximum_items: 1, maximum_bytes: 4096 };
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
            if publication.close_step(grant).unwrap() == store::SnapshotRetirementStep::Complete {
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
        if registry.close_step(1, 4096).unwrap() == PluginCloseStep::Complete {
            break;
        }
    }
    assert!(registry.terminal_is_empty());
    eprintln!("[DEBUG] window-transient refresh rebinds a captured authority onto the live generation so evaluate publication is not retiring a rejected write");
}
