//! 🧪️ The issued Store snapshot closes only after returning all of its captured custody.
use semio_framework_value::{
    factory::ArcRootRetirement,
    retained_clone::{RetainedCloneGrant, RetainedCloneProgress},
    retirement::{RetireOwned, RetirementCursor, RetirementStep},
    ArtifactOwnedValueRetirementFactory, FactoryAuthority, RetirementDemand,
};
use semio_framework_value_clone_source_testing::drive::{drive, drive_reply};
use semio_framework_value_factory::OwnedValueRetirementFactory;
use semio_framework_value_snapshot_read::{
    advance_returned_snapshot_read, artifact_retirement_box_close_step,
    artifact_retirement_box_demands, snapshot_registry_alias_close_step,
    snapshot_registry_alias_demands, snapshot_registry_alias_work_demand, CapturedSnapshotSource,
    SnapshotRead, SnapshotReadRegistryAliasRetirement, SnapshotReadRegistryHandle,
};
use semio_framework_value_testing_presence_retirement::with_effect;
use std::sync::Arc;

const BODY: usize = 4096;
type Source = CapturedSnapshotSource<Vec<u8>>;
type Issuer = dyn ArtifactOwnedValueRetirementFactory<Vec<u8>>;

fn grant(demand: RetirementDemand) -> RetainedCloneGrant {
    RetainedCloneGrant {
        maximum_items: 1,
        maximum_copy_bytes: demand.copy_bytes,
        maximum_capacity_bytes: demand.capacity_bytes,
        maximum_release_bytes: demand.release_bytes,
        maximum_depth: demand.depth,
    }
}

fn close(cursor: &mut impl RetirementCursor, grant: RetainedCloneGrant) -> RetirementStep {
    let units = cursor.next_computational_work(BODY).unwrap();
    let step = drive_reply(
        grant,
        units,
        |answer| match answer {
            Ok(RetirementStep::Progress(progress, _)) => *progress,
            Ok(RetirementStep::Failure(error, _)) | Err(error) => error.retained_progress(),
            _ => RetainedCloneProgress::default(),
        },
        |effect, reply, recover| {
            if recover {
                cursor.recover_close_into(BODY, effect, reply)
            } else {
                cursor.close_into(BODY, effect, reply)
            }
        },
    ).result.unwrap();
    if let RetirementStep::Failure(error, _) = step {
        panic!("captured retirement refused: {error}");
    }
    step
}

fn finish_external_registry(registry: SnapshotReadRegistryHandle, issuer: Arc<Issuer>, limit: u64) {
    let mut active = None;
    let admission = ArcRootRetirement::<Vec<u8>>::admission_demand();
    drive(grant(admission), 1, |effect| advance_returned_snapshot_read(&registry, &mut active, &issuer, effect)).result.unwrap();
    assert!(registry.terminal_is_empty());
    assert!(active.is_some());

    for _ in 0..limit {
        let Some(owner) = active.as_ref() else { break };
        let demand = artifact_retirement_box_demands(owner, BODY).unwrap();
        let units = if owner.terminal_is_empty() { 1 } else { owner.next_computational_work(BODY).unwrap() };
        drive(grant(demand), units, |effect| artifact_retirement_box_close_step(&mut active, BODY, effect)).result.unwrap();
    }
    assert!(active.is_none());

    let mut registry = Some(SnapshotReadRegistryAliasRetirement::new(registry));
    for _ in 0..limit {
        if registry.is_none() { break }
        let demand = snapshot_registry_alias_demands(&registry).unwrap();
        let units = snapshot_registry_alias_work_demand(&registry).unwrap();
        drive(grant(demand), units, |effect| snapshot_registry_alias_close_step(&mut registry, effect)).result.unwrap();
    }
    assert!(registry.is_none());

    let mut authority = FactoryAuthority::new(issuer);
    for _ in 0..limit {
        if authority.terminal_is_empty() { break }
        let demand = authority.demands(BODY).unwrap();
        let units = authority.next_computational_work(BODY).unwrap();
        drive(grant(demand), units, |effect| authority.step(BODY, effect)).result.unwrap();
    }
    assert!(authority.terminal_is_empty());
}

#[test]
fn captured_snapshot_quotes_original_root_and_completes_only_after_returning_its_custody() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("🧫️fixtures/🔣️.json")).unwrap();
    let limit = fixture["maximumTurns"].as_u64().unwrap();
    for row in fixture["cases"].as_array().unwrap() {
        let registry_held = row["registryHeld"].as_bool().unwrap();
        let expected = &fixture["retirement"][if registry_held { "shared" } else { "last" }];
        let bytes = serde_json::from_value::<Vec<u8>>(row["bytes"].clone()).unwrap();
        let root = Arc::new(bytes);
        let pointer = Arc::as_ptr(&root);
        let registry = SnapshotReadRegistryHandle::new();
        let lease = registry.try_issue(Arc::clone(&root)).unwrap();
        let mut read = Some(SnapshotRead::new(root, lease));
        let (_, index, generation) = read.as_ref().unwrap().lease_identity().unwrap();
        let mut issuer: Option<Arc<Issuer>> = Some(Arc::new(OwnedValueRetirementFactory::<Vec<u8>>::default()));
        let capture_work = Source::constructor_work_demand(&read);
        let turn = with_effect(grant(Source::constructor_demand()), capture_work, &mut None, |effect| {
            Source::admit(&mut read, issuer.as_ref().unwrap(), effect)
        });
        assert_eq!(turn.work, u128::from(fixture["captureWork"]["success"].as_u64().unwrap()));
        let (source, _) = turn.result.unwrap().unwrap();
        assert!(read.is_none());
        assert_eq!(source.get() as *const _, pointer);
        assert_eq!(serde_json::to_value(source.get()).unwrap(), row["expected"]);
        let mut registry = Some(registry);
        if !registry_held {
            drop(registry.take());
            drop(issuer.take());
        }

        let mut cursor = source.retirement_seed();
        let initial = cursor.next_demand(BODY).unwrap();
        assert!(matches!(close(&mut cursor, grant(initial)), RetirementStep::Progress(_, _)));
        assert!(!cursor.terminal_is_empty());
        let selected = cursor.next_demand(BODY).unwrap();
        let root = ArcRootRetirement::<Vec<u8>>::admission_demand();
        let chose_root = selected.capacity_bytes == root.capacity_bytes && selected.copy_bytes == root.copy_bytes;
        assert_eq!(chose_root, expected["arcRootAdmission"].as_bool().unwrap());

        if chose_root {
            assert_eq!(selected.depth, root.depth + expected["extraDepth"].as_u64().unwrap() as usize);
            assert_eq!(selected.release_bytes, root.release_bytes);
            let exact = grant(selected);
            for short in [
                RetainedCloneGrant { maximum_copy_bytes: exact.maximum_copy_bytes - 1, ..exact },
                RetainedCloneGrant { maximum_capacity_bytes: exact.maximum_capacity_bytes - 1, ..exact },
                RetainedCloneGrant { maximum_depth: exact.maximum_depth - 1, ..exact },
            ] {
                assert!(matches!(close(&mut cursor, short), RetirementStep::Progress(progress, 0) if progress == RetainedCloneProgress::default()));
                let after = cursor.next_demand(BODY).unwrap();
                assert_eq!((after.copy_bytes, after.capacity_bytes, after.release_bytes, after.depth), (selected.copy_bytes, selected.capacity_bytes, selected.release_bytes, selected.depth));
            }
        }

        let step = close(&mut cursor, grant(selected));
        let RetirementStep::Progress(progress, _) = step else {
            panic!("selected registry turn must preserve remaining custody");
        };
        assert_eq!(progress.retained_capacity_bytes, if chose_root { root.capacity_bytes } else { 0 });

        let mut completed = false;
        for _ in 0..limit {
            let demand = cursor.next_demand(BODY).unwrap();
            match close(&mut cursor, grant(demand)) {
                RetirementStep::Complete(_) => {
                    assert!(cursor.terminal_is_empty());
                    completed = true;
                    break;
                }
                RetirementStep::Progress(_, _) => {}
                step => panic!("unexpected captured retirement step with {} work", step.work()),
            }
        }
        assert_eq!(completed, expected["complete"].as_bool().unwrap());
        assert_eq!(
            registry.as_ref().is_some_and(|registry| registry.contains(index, generation)),
            expected["returnedReadSurvives"].as_bool().unwrap(),
        );
        if let Some(registry) = registry.take() {
            assert_eq!(registry.returned_count(), 1);
            finish_external_registry(registry, issuer.take().unwrap(), limit);
        }
    }
}
