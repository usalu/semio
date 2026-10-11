use super::*;

#[derive(serde::Deserialize)]
struct NeutralWindowFixture {
    windows: Vec<NeutralWindowCase>,
}

fn retire_note_window_mutation(mutation: NoteCompositeWindowTransientMutation) {
    let grant = semio_framework_value::RetainedCloneGrant { maximum_items: 1, maximum_capacity_bytes: semio_framework_value::retirement::owned_retirement_birth_bytes::<NoteCompositeWindowTransientMutation>(), maximum_depth: 4, ..Default::default() };
    let (mut retirement, birth) = semio_framework_value::retirement::admit_owned_retirement(mutation, grant).map_err(|(error, _)| error).expect("Note transient owner admission");
    assert!(birth.fits(grant));
    store::test_support::drive_retirement(retirement.as_mut()).expect("Note transient owner retirement");
    assert!(retirement.terminal_is_empty());
}

#[test]
fn transient_string_retirement_reaches_terminal_empty_through_quoted_grants() {
    let mutation = NoteCompositeWindowTransientMutation::Snapshot {
        transient: NoteCompositeWindowTransient { engagement_input: "retire-owned-note-input".repeat(512), ink_tool: None },
    };
    assert!(note_composite_window_transient_preflight(&mutation).expect("large Note transient admission").is_admissible());
    retire_note_window_mutation(mutation);
}

fn retire_returned_note_transient(transient: NoteCompositeWindowTransient) {
    retire_note_window_mutation(NoteCompositeWindowTransientMutation::Snapshot { transient });
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
        mutation: NoteCompositeWindowTransientMutation::Snapshot { transient: NoteCompositeWindowTransient { engagement_input, ink_tool: None } },
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
    // 🧫️ This window's OWN neutral oracle (`🪟️window/🧫️fixtures/🔣️.json`). Two `..` too many
    // resolved to the subset-wide `✳️any/🧫️fixtures/🔣️.json` mutation-fixture ARRAY instead, which
    // fails to decode as this object ("invalid type: map, expected a sequence").
    let fixture: NeutralWindowFixture = serde_json::from_str(include_str!("../../🧫️fixtures/🔣️.json")).expect("independent neutral JSON oracle");
    assert_eq!(fixture.windows.len(), 2);
    for case in fixture.windows {
        assert!(!case.id.is_empty());
        let config: NoteCompositeWindowConfig = serde_json::from_value(case.config.clone()).expect("independent config oracle");
        let transient: NoteCompositeWindowTransient = serde_json::from_value(case.transient.clone()).expect("independent transient oracle");
        assert_eq!(serde_json::from_str::<serde_json::Value>(&semio_framework_pack_json::to_json_string(&config)).expect("config JSON"), case.config);
        assert_eq!(serde_json::from_str::<serde_json::Value>(&semio_framework_pack_json::to_json_string(&transient)).expect("transient JSON"), case.transient);
        store::os_store::test_support::assert_dsl_pack_equivalence(&config);
        store::os_store::test_support::assert_dsl_pack_equivalence(&transient);
        store::os_store::test_support::assert_op_text_binary_equivalence(&NoteCompositeWindowConfigMutation::SetCamera(config.camera.clone()));
        store::os_store::test_support::assert_op_text_binary_equivalence(&NoteCompositeWindowTransientMutation::Snapshot { transient });
    }
}
