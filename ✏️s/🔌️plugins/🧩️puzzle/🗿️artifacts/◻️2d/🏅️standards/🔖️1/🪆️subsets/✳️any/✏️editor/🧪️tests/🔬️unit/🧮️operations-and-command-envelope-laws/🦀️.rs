// 🧮️ Operations and command envelope laws: the node and example operations, the selection, nudge and undo laws, the fixed edit ledger and the command envelope round trip.

//#region 🔖️Operations
#[semio_framework_async_macros::async_test]
async fn add_node_action_emits_upsert_op_and_appends_node() {
    let mut app = app();
    let result = dispatch(&mut app, "addNode", Some(&json!({ "kind": "node" })), None).expect("add node");
    assert_eq!(committed_edits(&result), 1, "addNode must commit exactly one document edit");
    assert_eq!(board_snapshot_nodes(&fixture_of(&app)).len(), 1);
    close_app(&mut app);
}

/// 🛍️ The example switch is a LOAD: one dispatch answers one `Effect::LoadDocument` carrying the example, commits no document
/// edit and leaves no history row.
#[semio_framework_async_macros::async_test]
async fn set_active_example_loads_concrete_forest_through_the_load_path() {
    let mut app = app();
    assert_eq!(load_example(&mut app, PUZZLE2D_PLAY_EXAMPLE_CONCRETE_FOREST_ID), 0, "an example switch commits no document edit");
    assert!(!board_snapshot_nodes(&fixture_of(&app)).is_empty());
    close_app(&mut app);
}

/// 🔁️ Loading a second example replaces the first one rather than merging into it.
#[semio_framework_async_macros::async_test]
async fn a_newer_example_load_replaces_the_previous_document() {
    let mut app = app();
    load_example(&mut app, PUZZLE2D_PLAY_EXAMPLE_CONCRETE_FOREST_ID);
    let forest_nodes = board_snapshot_nodes(&fixture_of(&app)).len();
    load_example(&mut app, PUZZLE2D_PLAY_EXAMPLE_NAKAGIN_ID);
    assert!(!board_snapshot_edges(&fixture_of(&app)).is_empty());
    assert_ne!(board_snapshot_nodes(&fixture_of(&app)).len(), forest_nodes, "the second load must replace, not append to, the first example");
    close_app(&mut app);
}

/// 🤫️ The switch states no mutation at all, whatever document it replaces: its emit carries one load of exactly the example
/// and nothing else, so history never shows a row — not even a "No change" warning — for it.
#[test]
fn an_example_switch_emits_one_load_and_no_mutation() {
    let command = Puzzle2dCommand::from_action("setActiveExample", Some(json!({ "exampleId": PUZZLE2D_PLAY_EXAMPLE_CONCRETE_FOREST_ID })), None);
    let empty = Puzzle2dPlaySnapshot::new(semio_framework_value::FromValue::from_value(semio_framework_value::ToValue::to_value(&(json!({ "schema": "board.ports.directed.v1", "nodes": [], "edges": [] })))).expect("typed fixture admits"));
    for base in [&empty, &Puzzle2dPlaySnapshot::new(set_active_example::target(PUZZLE2D_PLAY_EXAMPLE_CONCRETE_FOREST_ID).clone())] {
        let emit = puzzle2d_active_example_emit(&command, base, &Puzzle2dConfig::default()).expect("the switch emits");
        assert!(emit.artifact_mutations.is_empty() && emit.config_mutations.is_empty(), "no mutation row: {:?}", emit.artifact_mutations);
        let [Effect::LoadDocument { pack, .. }] = emit.effects.as_slice() else { panic!("one whole-document load: {:?}", emit.effects) };
        let loaded = <crate::Puzzle2dSnapshot as pack::snapshot::ArtifactPack>::decode_pack(pack).expect("the loaded pack decodes");
        assert_eq!(&loaded, set_active_example::target(PUZZLE2D_PLAY_EXAMPLE_CONCRETE_FOREST_ID), "the load carries exactly the example");
    }
}

/// 📦️ `Puzzle2dPlaySnapshot`'s pack encoding round-trips through the same `(RecordSpec,
/// RecordValue)` pair its `parse_dsl`/`print_dsl` do (both delegate to the underlying
/// `semio_framework_pack_json::Value` bridge impls).
#[semio_framework_async_macros::async_test]
async fn puzzle2d_play_projection_pack_round_trips() {
    let mut app = concrete_forest_app();
    semio_framework_os_kernel::os_store::test_support::assert_dsl_pack_equivalence(&app.snapshot().expect("projection"));
    close_app(&mut app);
}

#[semio_framework_async_macros::async_test]
async fn select_then_delete_selection_removes_the_node() {
    let mut app = app_with_registry();
    dispatch(&mut app, "addNode", Some(&json!({ "kind": "node" })), None).expect("add node");
    let node_id = first_node_id(&app);
    select_id(&mut app, PUZZLE2D_GRANULARITY_NODE, &node_id).expect("select");
    dispatch(&mut app, "deleteSelection", None, None).expect("delete");
    assert!(board_snapshot_nodes(&fixture_of(&app)).is_empty());
    close_app(&mut app);
}

/// 🌀️ A nudge is one logical operation: each `translateSelection` dispatch is ONE select-tool transaction and
/// ONE edit, so three nudges are three edits that accumulate on the document, and one undo takes back only the
/// last — never the whole run (a coalesced amend no longer exists).
#[semio_framework_async_macros::async_test]
async fn three_nudges_are_three_transactions_and_one_undo_takes_back_the_last() {
    let mut app = app_with_registry();
    dispatch(&mut app, "addNode", Some(&json!({ "kind": "node" })), None).expect("add node");
    let id = first_node_id(&app);
    select_id(&mut app, PUZZLE2D_GRANULARITY_NODE, &id).expect("select");
    let node_x = |app: &Puzzle2dApp| board_snapshot_nodes(&fixture_of(app))[0].get("x").and_then(Value::as_f64).expect("x");
    let start = node_x(&app);
    for dx in [1.0, 2.0, 3.0] {
        let result = dispatch(&mut app, "translateSelection", Some(&json!({ "dx": dx, "dy": 0.0 })), None).expect("nudge");
        assert_eq!(committed_edits(&result), 1, "every nudge is one edit");
    }
    assert!((node_x(&app) - start - 6.0).abs() < 1e-9, "three nudges accumulate 1+2+3 on x");
    dispatch(&mut app, "undo", None, None).expect("undo");
    assert!((node_x(&app) - start - 3.0).abs() < 1e-9, "one undo takes back only the last nudge");
    close_app(&mut app);
}

/// 🧾️ The 64-slot edit ledger (`ARTIFACT_HISTORY_LEDGER_CAPACITY`) is the app's hard interactive
/// budget: a session of ordinary small edits must reach it and refuse HONESTLY (a named fault the
/// caller sees), never corrupt the store or die silently. This pins where that wall stands so a
/// gesture that quietly spends 100 slots (a per-placement fill) cannot creep back in unnoticed. The refusal
/// is the store's typed `VcsError::HistoryFull` (fault code `history.full`, `🌿️vcs/🦀️.rs`), which names the ceiling and
/// which the shells show as a localized notice.
#[semio_framework_async_macros::async_test]
async fn sequential_small_edits_honour_the_fixed_edit_ledger_ceiling() {
    let mut app = app_with_registry();
    let mut committed = 0usize;
    let mut refusal: Option<String> = None;
    for index in 0..70 {
        match dispatch(&mut app, "addNode", Some(&json!({ "kind": "node" })), None) {
            Ok(_) => committed += 1,
            Err(fault) => {
                refusal = Some(format!("edit {index}: {fault:?}"));
                break;
            }
        }
    }
    assert_eq!(board_snapshot_nodes(&fixture_of(&app)).len(), committed, "every admitted edit landed in the document");
    let undone = dispatch(&mut app, "undo", None, None);
    close_app(&mut app);
    assert!(undone.is_ok(), "the store stays usable at the ceiling: {:?}", undone.err());
    assert!(committed >= 64, "the ledger must admit its full 64 slots, admitted {committed}");
    assert!(refusal.as_deref().is_none_or(|fault| fault.contains("history.full")), "past the ceiling the refusal must be the typed full-history fault, got {refusal:?}");
}

#[semio_framework_async_macros::async_test]
async fn undo_redo_round_trip_through_the_wrapper() {
    let mut app = app();
    dispatch(&mut app, "addNode", Some(&json!({ "kind": "node" })), None).expect("add");
    assert_eq!(board_snapshot_nodes(&fixture_of(&app)).len(), 1);
    dispatch(&mut app, "undo", None, None).expect("undo");
    assert_eq!(board_snapshot_nodes(&fixture_of(&app)).len(), 0);
    dispatch(&mut app, "redo", None, None).expect("redo");
    assert_eq!(board_snapshot_nodes(&fixture_of(&app)).len(), 1);
    close_app(&mut app);
}
//#endregion 🔖️Operations

//#region 🔖️CommandEnvelopeTests
/// 🎫️ CW7 command-envelope law (`POLICY_COMMAND_ENVELOPE_COMPLETENESS_ALLOWLIST`). Deliberately
/// dispatches through a standalone typed `Puzzle2dStore` — NOT through `Puzzle2dPlayApp`/
/// `Puzzle2dPlaySnapshot` (the `🔖️ValueBridge` `semio_framework_pack_json::Value` wrapper this app still uses)
/// — since `Puzzle2dMutation`'s canonical `Mutation<Puzzle2dSnapshot>` impl (not its
/// `Mutation<Value>` bridge impl) is what the CW7 law is about.
#[semio_framework_async_macros::async_test]
async fn command_envelope_round_trip_holds_for_an_applied_operation() {
    use crate::host::owned::{close_puzzle2d_store,puzzle2d_store};

    use {crate::standards::v1::subsets::any::schema::records::Puzzle2dNode, crate::standards::v1::subsets::any::schema::records::PUZZLE_2D_SCHEMA};
    use protocol::{ArtifactId, Edit, SchemaId};
    use store::{os_store::document_materialize::create_document_envelope, os_store::store_schema::ArtifactCommand};

    let mut store = puzzle2d_store(create_document_envelope(PUZZLE_2D_SCHEMA, "puzzle2d", Puzzle2dSnapshot::default(), None, &::semio_framework_os_kernel::os_vcs::io::binary::genesis::ArtifactPackGenesisProducer), protocol::ActorId(protocol::LOCAL_ACTOR_ID.into())).await.expect("store");
    let node = Puzzle2dNode { id: "n1".into(), ..Default::default() };
    store.dispatch(ArtifactCommand::Apply { mutations: vec![crate::standards::v1::subsets::any::schema::mutations::create_node(node, None)], transaction: None }, &mut semio_framework_plugin::artifact_app_laws::fixture_identity()).await.expect("apply");
    let envelope = store.envelope();
    let edit: &Edit<Puzzle2dMutation> = envelope.vcs.edits.last().expect("dispatch must have recorded an edit");
    semio_framework_os_kernel::os_store::test_support::assert_command_envelope_round_trip::<Puzzle2dSnapshot, Puzzle2dMutation>(edit, &ArtifactId(envelope.id.clone()), &SchemaId(envelope.schema.clone())).await;
    close_puzzle2d_store(&mut store).expect("the standalone store retires to its terminal-empty shell");
}
//#endregion 🔖️CommandEnvelopeTests
