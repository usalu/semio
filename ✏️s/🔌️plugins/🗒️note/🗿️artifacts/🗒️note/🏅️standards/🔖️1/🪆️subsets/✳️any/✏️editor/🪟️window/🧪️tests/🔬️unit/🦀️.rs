use super::*;

#[derive(serde::Deserialize)]
struct NeutralWindowFixture {
    windows: Vec<NeutralWindowCase>,
}

#[test]
fn transient_string_retirement_reaches_terminal_empty_with_tiny_grants() {
    let mutation = NoteCompositeWindowTransientMutation::Snapshot {
        transient: NoteCompositeWindowTransient { engagement_input: "retire-owned-note-input".repeat(512) },
    };
    assert!(note_composite_window_transient_preflight(&mutation).expect("large Note transient admission").is_admissible());
    let mut retirement = store::retirement::owned_retirement(mutation);
    for _ in 0..32_768 {
        match retirement.close_step(1, 1).expect("bounded retirement") {
            store::SnapshotRetirementStep::Complete => {
                assert!(retirement.terminal_is_empty());
                return;
            }
            store::SnapshotRetirementStep::Pending { released_items, released_bytes } => {
                assert!(released_items <= 1);
                assert!(released_bytes <= 1);
            }
            store::SnapshotRetirementStep::Blocked => panic!("owned Note transient retirement blocked"),
        }
    }
    panic!("owned Note transient retirement exceeded its bounded cursor turns");
}

fn retire_returned_note_transient(transient: NoteCompositeWindowTransient) {
    let mut retirement = store::retirement::owned_retirement(NoteCompositeWindowTransientMutation::Snapshot { transient });
    for _ in 0..4_096 {
        match retirement.close_step(1, 1).expect("returned Note owner retirement") {
            store::SnapshotRetirementStep::Complete => {
                assert!(retirement.terminal_is_empty());
                return;
            }
            store::SnapshotRetirementStep::Pending { released_items, released_bytes } => {
                assert!(released_items <= 1);
                assert!(released_bytes <= 1);
            }
            store::SnapshotRetirementStep::Blocked => panic!("returned Note owner retirement blocked"),
        }
    }
    panic!("returned Note owner retirement exceeded its bounded cursor turns");
}

#[test]
fn oversized_string_capacity_rejects_and_returns_the_exact_note_owner() {
    let owners = <NoteCompositeWindowTransientOwner as semio_framework_plugin::WindowTransientOwner>::build_owners();
    let mut engagement_input = String::with_capacity(store::ARTIFACT_STORE_ONE_ITEM_MAXIMUM_BYTES + 1);
    engagement_input.push('n');
    let pointer = engagement_input.as_ptr();
    let capacity = engagement_input.capacity();
    let request = store::ArtifactEphemeralOneItemPreparationRequest {
        operation: semio_framework_job::OperationId(1),
        generation: semio_framework_job::Generation(1),
        base: store::ArtifactEphemeralBaseRead(store::ArtifactEphemeralBaseOwner::Transient(std::sync::Arc::new(NoteCompositeWindowTransient::default()))),
        mutation: NoteCompositeWindowTransientMutation::Snapshot { transient: NoteCompositeWindowTransient { engagement_input } },
    };
    let returned = match owners.preparation.begin(request) {
        Err(request) => request,
        Ok(_) => panic!("oversized Note string capacity must reject"),
    };
    let NoteCompositeWindowTransientMutation::Snapshot { transient } = returned.mutation;
    assert_eq!(transient.engagement_input, "n");
    assert_eq!(transient.engagement_input.as_ptr(), pointer);
    assert_eq!(transient.engagement_input.capacity(), capacity);
    retire_returned_note_transient(transient);
}

#[derive(serde::Deserialize)]
struct NeutralWindowCase {
    id: String,
    config: serde_json::Value,
    transient: serde_json::Value,
}

#[test]
fn neutral_window_schema_round_trips_match_the_serde_json_oracle() {
    let fixture: NeutralWindowFixture = serde_json::from_str(include_str!("../../../../🧫️fixtures/🔣️.json")).expect("independent neutral JSON oracle");
    assert_eq!(fixture.windows.len(), 2);
    for case in fixture.windows {
        assert!(!case.id.is_empty());
        let config: NoteCompositeWindowConfig = serde_json::from_value(case.config.clone()).expect("independent config oracle");
        let transient: NoteCompositeWindowTransient = serde_json::from_value(case.transient.clone()).expect("independent transient oracle");
        assert_eq!(serde_json::from_str::<serde_json::Value>(&dsl::json::to_json_string(&config)).expect("config JSON"), case.config);
        assert_eq!(serde_json::from_str::<serde_json::Value>(&dsl::json::to_json_string(&transient)).expect("transient JSON"), case.transient);
        store::os_store::test_support::assert_dsl_pack_equivalence(&config);
        store::os_store::test_support::assert_dsl_pack_equivalence(&transient);
        store::os_store::test_support::assert_op_text_binary_equivalence(&NoteCompositeWindowConfigMutation::Snapshot { config });
        store::os_store::test_support::assert_op_text_binary_equivalence(&NoteCompositeWindowTransientMutation::Snapshot { transient });
    }
}
