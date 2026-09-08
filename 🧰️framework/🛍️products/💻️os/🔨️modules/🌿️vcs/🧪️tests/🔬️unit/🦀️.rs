use super::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
struct DemoItem {
    id: String,
    value: i32,
}

impl Identified<String> for DemoItem {
    fn id(&self) -> &String {
        &self.id
    }
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
struct DemoItemPatch {
    value: Option<i32>,
}

impl Patchable<DemoItemPatch> for DemoItem {
    fn apply_patch(&mut self, patch: &DemoItemPatch) {
        if let Some(value) = patch.value {
            self.value = value;
        }
    }

    fn diff_patch(&self, other: &Self) -> Option<DemoItemPatch> {
        (self.value != other.value).then_some(DemoItemPatch { value: Some(other.value) })
    }
}

#[semio_framework_async_macros::async_test]
async fn collection_diff_from_op_projects_each_variant() {
    let items: Vec<DemoItem> = vec![DemoItem { id: "a".into(), value: 1 }, DemoItem { id: "b".into(), value: 2 }];
    let added = collection_diff_from_mutation::<String, DemoItem, DemoItemPatch>(&items, &CollectionMutation::Add { index: 0, item: DemoItem { id: "c".into(), value: 3 } });
    assert_eq!(added.added.len(), 1);
    assert!(added.removed.is_empty() && added.modified.is_empty());

    let removed = collection_diff_from_mutation::<String, DemoItem, DemoItemPatch>(&items, &CollectionMutation::Remove { id: "a".into() });
    assert_eq!(removed.removed, vec!["a".to_string()]);

    let patched = collection_diff_from_mutation(&items, &CollectionMutation::Patch { id: "b".into(), patch: DemoItemPatch { value: Some(9) } });
    assert_eq!(patched.modified.len(), 1);
    assert_eq!(patched.modified[0].id, "b");

    let moved = collection_diff_from_mutation::<String, DemoItem, DemoItemPatch>(&items, &CollectionMutation::Move { id: "a".into(), to_index: 1 });
    assert_eq!(moved.removed, vec!["a".to_string()], "move is encoded as remove + re-add by identity");
    assert_eq!(moved.added.len(), 1);
    assert_eq!(moved.added[0].id, "a");
}

#[semio_framework_async_macros::async_test]
async fn collection_op_add_and_invert() {
    let items: Vec<DemoItem> = vec![DemoItem { id: "a".into(), value: 1 }];
    let operation = CollectionMutation::Add { index: 1, item: DemoItem { id: "b".into(), value: 2 } };
    let mut applied = items.clone();
    apply_collection_mutation(&mut applied, &operation);
    assert_eq!(applied.len(), 2);
    assert_eq!(applied[1].id, "b");
    let inverse = inverse_collection_mutation(&items, &operation);
    apply_collection_mutation(&mut applied, &inverse);
    assert_eq!(applied, items);
}

#[semio_framework_async_macros::async_test]
async fn collection_op_move_and_invert() {
    let items: Vec<DemoItem> = vec![DemoItem { id: "a".into(), value: 1 }, DemoItem { id: "b".into(), value: 2 }, DemoItem { id: "c".into(), value: 3 }];
    let operation = CollectionMutation::Move { id: "a".into(), to_index: 2 };
    let mut applied = items.clone();
    apply_collection_mutation(&mut applied, &operation);
    assert_eq!(applied.iter().map(|i| i.id.clone()).collect::<Vec<_>>(), vec!["b", "c", "a"]);
    let inverse = inverse_collection_mutation(&items, &operation);
    apply_collection_mutation(&mut applied, &inverse);
    assert_eq!(applied, items);
}

#[semio_framework_async_macros::async_test]
async fn collection_op_patch_and_invert() {
    let items: Vec<DemoItem> = vec![DemoItem { id: "a".into(), value: 1 }];
    let operation = CollectionMutation::Patch { id: "a".into(), patch: DemoItemPatch { value: Some(9) } };
    let mut applied = items.clone();
    apply_collection_mutation(&mut applied, &operation);
    assert_eq!(applied[0].value, 9);
    let inverse = inverse_collection_mutation(&items, &operation);
    apply_collection_mutation(&mut applied, &inverse);
    assert_eq!(applied, items);
}

#[semio_framework_async_macros::async_test]
async fn collection_op_remove_and_invert() {
    let items: Vec<DemoItem> = vec![DemoItem { id: "a".into(), value: 1 }, DemoItem { id: "b".into(), value: 2 }];
    let operation = CollectionMutation::Remove { id: "a".into() };
    let mut applied = items.clone();
    apply_collection_mutation(&mut applied, &operation);
    assert_eq!(applied.len(), 1);
    let inverse = inverse_collection_mutation(&items, &operation);
    apply_collection_mutation(&mut applied, &inverse);
    assert_eq!(applied, items);
}

//#endregion 🔖️ReconcileAlternative

//#region 🔖️ContentAddressedCheckpointAndMergeBase
#[semio_framework_async_macros::async_test]
async fn fixed_history_ledger_preserves_order_capacity_and_aba_rejection() {
    let mut ledger = ArtifactHistoryLedger::new();
    let mut keys = Vec::with_capacity(ARTIFACT_HISTORY_LEDGER_CAPACITY);
    for index in 0..ARTIFACT_HISTORY_LEDGER_CAPACITY {
        keys.push(ledger.try_push(format!("history-{index:02}")).expect("fixed ledger admits its exact capacity"));
    }
    let rejected = ledger.try_push("history-overflow".to_string()).expect_err("capacity + 1 returns the exact rejected owner");
    assert_eq!(rejected, "history-overflow");
    assert_eq!(ledger.iter().next().map(String::as_str), Some("history-00"));
    assert_eq!(ledger.iter().next_back().map(String::as_str), Some("history-63"));

    let removed = ledger.remove_key(keys[17]).expect("live generation removes its exact owner");
    assert_eq!(removed, "history-17");
    assert_eq!(ledger.remove_key(keys[17]), Err(keys[17]), "a stale generation cannot remove the reused slot");
    let replacement = ledger.try_push("history-replacement".to_string()).expect("one tombstone admits one replacement");
    assert_eq!(replacement.index, keys[17].index);
    assert!(replacement.generation > keys[17].generation);
    assert_eq!(ledger.last().map(String::as_str), Some("history-replacement"));

    let mut drained = 0;
    while ledger.pop().is_some() {
        drained += 1;
    }
    assert_eq!(drained, ARTIFACT_HISTORY_LEDGER_CAPACITY);
    assert!(ledger.terminal_is_empty());
}

#[semio_framework_async_macros::async_test]
async fn fixed_history_reservation_returns_exact_rejected_owner_and_blocks_aba() {
    let mut first = ArtifactHistoryLedger::new();
    let mut second = ArtifactHistoryLedger::new();
    let reservation = first.reserve_one().expect("empty fixed ledger reserves one exact slot");
    assert_eq!(first.reserve_one().unwrap_err(), ArtifactHistoryReservationFault::Busy);
    let rejected = first.try_push("parallel-owner".to_string()).expect_err("an outstanding reservation excludes parallel adoption");
    assert_eq!(rejected, "parallel-owner");
    let (reservation, rejected) = second.insert_reserved(reservation, "wrong-ledger-owner".to_string()).expect_err("a reservation cannot cross ledger authority");
    assert_eq!(rejected, "wrong-ledger-owner");
    first.cancel_reservation(reservation).expect("the exact unconsumed token returns to its issuing ledger");

    let reservation = first.reserve_one().expect("cancelled reservation releases capacity");
    let key = first.insert_reserved(reservation, "committed-owner".to_string()).expect("matching token adopts exactly once");
    assert_eq!(first.remove_key(key).expect("live key returns its exact owner"), "committed-owner");
    assert!(first.terminal_is_empty());
    assert!(second.terminal_is_empty());
}

#[semio_framework_async_macros::async_test]
async fn content_addressed_checkpoint_id_is_deterministic_and_content_sensitive() {
    let root_change = Change { id: "change-root".into(), edit_ids: vec!["edit-1".into()], description: Some("root".into()), saved_at: "2026-07-27T00:00:00Z".into() };
    let mut changes = ArtifactHistoryLedger::try_from_preflighted(vec![root_change]).expect("one change fits the fixed ledger");
    let change_ids = vec!["change-root".to_string()];
    let authors = vec![Author { id: "a1".into(), name: "Alice".into(), avatar: None }];

    let id_a = content_addressed_checkpoint_id(None, &change_ids, &changes, Some("root"), &authors, "2026-07-27T00:00:01Z", &[]).await;
    let id_b = content_addressed_checkpoint_id(None, &change_ids, &changes, Some("root"), &authors, "2026-07-27T00:00:01Z", &[]).await;
    assert_eq!(id_a, id_b, "identical inputs converge on the identical id");
    assert!(id_a.starts_with("ck-"), "got {id_a}");

    let id_different_message = content_addressed_checkpoint_id(None, &change_ids, &changes, Some("other message"), &authors, "2026-07-27T00:00:01Z", &[]).await;
    assert_ne!(id_a, id_different_message, "a different message must change the id");

    let id_different_parent = content_addressed_checkpoint_id(Some("ck-parent"), &change_ids, &changes, Some("root"), &authors, "2026-07-27T00:00:01Z", &[]).await;
    assert_ne!(id_a, id_different_parent, "a different parent must change the id");

    let id_different_timestamp = content_addressed_checkpoint_id(None, &change_ids, &changes, Some("root"), &authors, "2026-07-27T00:00:02Z", &[]).await;
    assert_ne!(id_a, id_different_timestamp, "a different timestamp must change the id");
    drop(changes.pop());
}

#[semio_framework_async_macros::async_test]
async fn pending_change_checkpoint_hash_is_byte_identical_before_history_reservation() {
    let mut changes = ArtifactHistoryLedger::new();
    let change = Change { id: "change-pending".into(), edit_ids: vec!["edit-a".into(), "edit-b".into()], description: Some("pending".into()), saved_at: "2026-08-23T00:00:00Z".into() };
    let change_ids = vec![change.id.clone()];
    let authors = vec![Author { id: "actor".into(), name: "Actor".into(), avatar: None }];
    let before_reservation = content_addressed_checkpoint_id_with_pending_change(None, &change_ids, &changes, &change.id, &change.edit_ids, change.description.as_deref(), &change.saved_at, Some("checkpoint"), &authors, "2026-08-23T00:00:01Z", &[]);
    let reservation = changes.reserve_one().expect("hashing does not consume the fixed ledger reservation");
    changes.insert_reserved(reservation, change).expect("exact reservation adopts the pending change");
    let after_commit = content_addressed_checkpoint_id(None, &change_ids, &changes, Some("checkpoint"), &authors, "2026-08-23T00:00:01Z", &[]).await;
    assert_eq!(before_reservation, after_commit, "borrowing the pending change before reservation preserves the exact wire hash");
    drop(changes.pop());
}

/// @emoji 🔬️ `content_addressed_checkpoint_id_core`'s committed-`Change` branch now hashes
/// `crate::os_pack::json::to_json_string(change)` instead of `serde_json::to_vec(change)` —
/// direct proof the two are byte-identical for `Change`, both with and without `description`
/// (its one `Option` field, `skip_serializing_if`-omitted when `None`).
#[test]
fn change_to_json_string_matches_serde_json_byte_for_byte() {
    for description in [Some("a change".to_string()), None] {
        let change = Change { id: "change-x".into(), edit_ids: vec!["edit-1".into(), "edit-2".into()], description, saved_at: "2026-09-01T00:00:00Z".into() };
        let mine = crate::os_pack::json::to_json_string(&change);
        let theirs = serde_json::to_string(&change).unwrap();
        assert_eq!(mine, theirs, "Change's ToValue/pack::json bridge diverged from serde_json for description={:?}", change.description);
    }
}

/// @emoji 🔬️ `pending_change_ref_json`'s hand-built wire shape, byte-for-byte against an
/// independent `serde_json` oracle (a local `#[derive(Serialize)]` twin reproducing
/// `PendingChangeRef`'s pre-conversion shape) — the direct proof this ticket's own
/// `float-format-parity.md` calls for, that converting `content_addressed_checkpoint_id_core`
/// off `serde_json` changed zero bytes. Both branches (`description` present and absent, since
/// unlike `Change` this type has no `skip_serializing_if`) are checked.
#[test]
fn pending_change_ref_json_matches_serde_json_oracle() {
    #[derive(serde::Serialize)]
    #[serde(rename_all = "camelCase")]
    struct Oracle<'a> {
        id: &'a str,
        edit_ids: &'a [String],
        description: Option<&'a str>,
        saved_at: &'a str,
    }
    let edit_ids = vec!["edit-1".to_string(), "edit-2".to_string()];
    for description in [Some("a pending change"), None] {
        let pending = PendingChangeRef { id: "change-x", edit_ids: &edit_ids, description, saved_at: "2026-09-01T00:00:00Z" };
        let mine = pending_change_ref_json(&pending);
        let oracle = Oracle { id: "change-x", edit_ids: &edit_ids, description, saved_at: "2026-09-01T00:00:00Z" };
        let theirs = serde_json::to_string(&oracle).unwrap();
        assert_eq!(mine, theirs, "pending_change_ref_json diverged from the serde_json oracle for description={description:?}");
    }
}

/// @emoji 🧩️ `composition_pins`/`CompositionPin` extension to `content_addressed_checkpoint_id`:
/// the three properties the ticket calls for — pin-set changes flip the id, identical
/// pins-in-identical-order converge, and (critically) an EMPTY pin list must hash to the exact
/// same bytes `content_addressed_checkpoint_id` produced before this field existed, so every
/// checkpoint id ever minted for a non-composite artifact stays valid.
#[semio_framework_async_macros::async_test]
async fn content_addressed_checkpoint_id_composition_pins_are_deterministic_and_backward_compatible() {
    let root_change = Change { id: "change-root".into(), edit_ids: vec!["edit-1".into()], description: Some("root".into()), saved_at: "2026-07-27T00:00:00Z".into() };
    let mut changes = ArtifactHistoryLedger::try_from_preflighted(vec![root_change]).expect("one change fits the fixed ledger");
    let change_ids = vec!["change-root".to_string()];
    let authors = vec![Author { id: "a1".into(), name: "Alice".into(), avatar: None }];
    let args = (None, &change_ids, &changes, Some("root"), &authors, "2026-07-27T00:00:01Z");

    // (1) Empty pins must reproduce the pre-pins hash bytes EXACTLY — recomputed here via the
    // same blake3(parent||changes||message||authors||timestamp) formula
    // `content_addressed_checkpoint_id` used before the `pins` parameter was added, so this is
    // a byte-level backward-compatibility proof, not just "doesn't panic".
    let mut legacy_input = Vec::new();
    legacy_input.extend_from_slice(args.0.unwrap_or("").as_bytes());
    legacy_input.push(0);
    for change_id in args.1 {
        let change_hash = args.2.iter().find(|change| change.id == *change_id).map_or([0u8; 32], |change| *semio_framework_hash::hash(&serde_json::to_vec(change).unwrap_or_default()).as_bytes());
        legacy_input.extend_from_slice(&change_hash);
    }
    legacy_input.push(0);
    legacy_input.extend_from_slice(args.3.unwrap_or("").as_bytes());
    legacy_input.push(0);
    for author in args.4 {
        legacy_input.extend_from_slice(author.id.as_bytes());
        legacy_input.push(0);
    }
    legacy_input.push(0);
    legacy_input.extend_from_slice(args.5.as_bytes());
    let legacy_digest = *semio_framework_hash::hash(&legacy_input).as_bytes();
    let legacy_hex16: String = legacy_digest[..8].iter().map(|byte| format!("{byte:02x}")).collect();
    let legacy_id = format!("ck-{legacy_hex16}");
    let id_no_pins = content_addressed_checkpoint_id(args.0, args.1, args.2, args.3, args.4, args.5, &[]).await;
    assert_eq!(id_no_pins, legacy_id, "an empty pin list must not change a single byte of the pre-existing hash input");

    // (2) A non-empty pin set changes the id relative to no pins at all.
    let child_a_ref = crate::os_io::ArtifactRef::parse_uri("child-a!s.stdio.mesh@87a/mesh").expect("valid test fixture uri");
    let child_b_ref = crate::os_io::ArtifactRef::parse_uri("child-b!s.stdio.image@87a/image").expect("valid test fixture uri");
    let pins_one = vec![CompositionPin { child_ref: child_a_ref.clone(), checkpoint_id: "ck-child-a-1".into() }];
    let id_with_pins = content_addressed_checkpoint_id(args.0, args.1, args.2, args.3, args.4, args.5, &pins_one).await;
    assert_ne!(id_no_pins, id_with_pins, "a non-empty pin list must change the id relative to no composition");

    // (3) Identical pins in identical order converge on the identical id.
    let id_with_pins_again = content_addressed_checkpoint_id(args.0, args.1, args.2, args.3, args.4, args.5, &pins_one).await;
    assert_eq!(id_with_pins, id_with_pins_again, "identical pins in identical order converge on the identical id");

    // (4) A different pin CONTENT (same child, different pinned checkpoint) changes the id.
    let pins_one_moved = vec![CompositionPin { child_ref: child_a_ref.clone(), checkpoint_id: "ck-child-a-2".into() }];
    let id_pin_moved = content_addressed_checkpoint_id(args.0, args.1, args.2, args.3, args.4, args.5, &pins_one_moved);
    assert_ne!(id_with_pins, id_pin_moved.await, "a different pinned checkpoint_id for the same child must change the id");

    // (5) Two peers that discover the same pin SET in different order (e.g. concurrent
    // parallel-child dispatch) still converge — the function sorts by `child_ref.to_uri()` internally.
    let pins_two_ordered = vec![CompositionPin { child_ref: child_a_ref.clone(), checkpoint_id: "ck-child-a-1".into() }, CompositionPin { child_ref: child_b_ref.clone(), checkpoint_id: "ck-child-b-1".into() }];
    let pins_two_reordered = vec![CompositionPin { child_ref: child_b_ref, checkpoint_id: "ck-child-b-1".into() }, CompositionPin { child_ref: child_a_ref, checkpoint_id: "ck-child-a-1".into() }];
    let id_ordered = content_addressed_checkpoint_id(args.0, args.1, args.2, args.3, args.4, args.5, &pins_two_ordered);
    let id_reordered = content_addressed_checkpoint_id(args.0, args.1, args.2, args.3, args.4, args.5, &pins_two_reordered);
    assert_eq!(id_ordered.await, id_reordered.await, "two peers discovering the same pin set in different incidental order must converge on the identical id");
    drop(changes.pop());
}

//#region 🆔️Ids
#[semio_framework_async_macros::async_test]
async fn content_addressed_entity_and_mint_helpers_are_deterministic() {
    assert_eq!(content_addressed_entity_id("x", b"payload").await, content_addressed_entity_id("x", b"payload").await);
    assert_ne!(content_addressed_entity_id("x", b"a").await, content_addressed_entity_id("x", b"b").await);
    assert_eq!(edit_scoped_id("edit-1", 0).await, edit_scoped_id("edit-1", 0).await);
    assert_ne!(edit_scoped_id("edit-1", 0).await, edit_scoped_id("edit-1", 1).await);
    assert!(edit_scoped_id("edit-1", 0).await.starts_with("scoped-"));
    assert_eq!(mint_edit_id(Some("alice"), 3, b"fwd").await, mint_edit_id(Some("alice"), 3, b"fwd").await);
    assert_ne!(mint_edit_id(Some("alice"), 3, b"fwd").await, mint_edit_id(Some("bob"), 3, b"fwd").await);
    assert_eq!(mint_change_id(&["e1".into(), "e2".into()], Some("msg")).await, mint_change_id(&["e1".into(), "e2".into()], Some("msg")).await);
    assert_eq!(mint_alternative_id("main", &["ck1".into()]).await, mint_alternative_id("main", &["ck1".into()]).await);
    assert_eq!(mint_mutation_id(b"op-bytes").await, mint_mutation_id(b"op-bytes").await);
    assert_eq!(create_document_vcs_id("draft").await, create_document_vcs_id("draft").await);
    assert!(create_document_vcs_id("draft").await.starts_with("draft-"));
}
//#endregion 🆔️Ids
