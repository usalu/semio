use super::*;

fn every_transition() -> Vec<HistoryTransition> {
    vec![
        HistoryTransition::Revert { mutation_ids: vec![MutationId("op-a".into()), MutationId("op-b".into())] },
        HistoryTransition::Reinstate { mutation_ids: vec![MutationId("op-a".into())] },
        HistoryTransition::Commit(TransitionCheckpoint {
            checkpoint_id: "ck-1".into(),
            parent_id: Some("ck-0".into()),
            change_id: "change-1".into(),
            mutation_ids: vec![MutationId("op-a".into())],
            description: Some("first".into()),
            saved_at: "2026-09-19T00:00:00.000Z".into(),
            authors: vec![TransitionAuthor { id: "u1".into(), name: "Ada".into(), avatar: None }, TransitionAuthor { id: "u2".into(), name: "Bo".into(), avatar: Some("b.png".into()) }],
            message: Some("first".into()),
            timestamp: "2026-09-19T00:00:00.001Z".into(),
        }),
        HistoryTransition::Branch { alternative_id: "alt-1".into(), name: "Variant".into(), checkpoint_id: "ck-1".into() },
        HistoryTransition::Checkout { checkpoint_id: "ck-1".into(), alternative_id: None },
        HistoryTransition::Checkout { checkpoint_id: "ck-1".into(), alternative_id: Some("alt-1".into()) },
        HistoryTransition::Repin { checkpoint_id: "ck-1".into(), pinned_checkpoint_id: "ck-2".into(), pins: vec![TransitionPin { child_uri: "semio://child".into(), checkpoint_id: "ck-c".into() }] },
    ]
}

/// 🔁️ Every variant survives an encode/decode round trip byte-exactly.
#[test]
fn every_transition_round_trips() {
    for transition in every_transition() {
        let bytes = encode_history_transition(&transition);
        assert_eq!(decode_history_transition(&bytes).expect("decode"), transition);
    }
}

/// 🚫️ Trailing bytes and unknown tags are refused instead of silently ignored.
#[test]
fn malformed_payloads_are_refused() {
    let mut bytes = encode_history_transition(&HistoryTransition::Checkout { checkpoint_id: "ck".into(), alternative_id: None });
    bytes.push(0);
    assert!(decode_history_transition(&bytes).is_err());
    assert!(decode_history_transition(&[9]).is_err());
    assert!(decode_history_transition(&[]).is_err());
}

/// 🪪️ Transition ids are content-addressed: same author, tick and payload → same id; any change → new id.
#[test]
fn transition_ids_are_content_addressed() {
    let actor = ActorId("actor-a".into());
    let tick = HybridLogicalTimestamp { actor: 1, physical_ms: 10, logical: 2 };
    let transition = HistoryTransition::Revert { mutation_ids: vec![MutationId("op-a".into())] };
    let a = history_transition_envelope(&transition, &ArtifactId("doc".into()), &actor, Vec::new(), tick);
    let b = history_transition_envelope(&transition, &ArtifactId("doc".into()), &actor, Vec::new(), tick);
    assert_eq!(a.mutation_id, b.mutation_id);
    assert!(a.mutation_id.0.starts_with("transition-"));
    let later = history_transition_envelope(&transition, &ArtifactId("doc".into()), &actor, Vec::new(), HybridLogicalTimestamp { logical: 3, ..tick });
    assert_ne!(a.mutation_id, later.mutation_id);
}

/// 🏷️ The schema tag alone routes an envelope: transitions decode, domain ops pass through as `None`.
#[test]
fn schema_tag_routes_envelopes() {
    let transition = HistoryTransition::Reinstate { mutation_ids: vec![MutationId("op-a".into())] };
    let envelope = history_transition_envelope(&transition, &ArtifactId("doc".into()), &ActorId("a".into()), vec![MutationId("op-a".into())], HybridLogicalTimestamp::new(0, 1));
    assert!(is_history_transition(&envelope));
    assert_eq!(history_transition_from_envelope(&envelope).expect("decode"), Some(transition));
    let mut operation = envelope.clone();
    operation.diff.schema = SchemaId("app.lowpoly".into());
    assert_eq!(history_transition_from_envelope(&operation).expect("decode"), None);
}

fn edit(id: &str, actor: &str, physical_ms: u64) -> FoldEdit {
    FoldEdit { id: id.into(), actor: Some(actor.into()), timestamp: HybridLogicalTimestamp { actor: 0, physical_ms, logical: 0 }, mutation_ids: vec![MutationId(format!("op-{id}"))] }
}

fn at(transition: HistoryTransition, actor: &str, physical_ms: u64) -> super::super::MutationEnvelope {
    history_transition_envelope(&transition, &ArtifactId("doc".into()), &ActorId(actor.into()), Vec::new(), HybridLogicalTimestamp { actor: 0, physical_ms, logical: 0 })
}

fn commit(checkpoint_id: &str, parent_id: Option<&str>, operations: &[&str]) -> HistoryTransition {
    HistoryTransition::Commit(TransitionCheckpoint {
        checkpoint_id: checkpoint_id.into(),
        parent_id: parent_id.map(Into::into),
        change_id: format!("change-{checkpoint_id}"),
        mutation_ids: operations.iter().map(|operation| MutationId(format!("op-{operation}"))).collect(),
        description: None,
        saved_at: "t".into(),
        authors: Vec::new(),
        message: None,
        timestamp: "t".into(),
    })
}

fn none() -> std::collections::HashSet<String> {
    std::collections::HashSet::new()
}

/// 🧮️ Edits fold into HLC order regardless of the order they are listed in.
#[test]
fn fold_orders_edits_by_hlc_whatever_their_arrival() {
    let forward = fold_history(&[edit("a", "x", 1), edit("b", "y", 2), edit("c", "x", 3)], &[], &none()).expect("fold");
    let shuffled = fold_history(&[edit("c", "x", 3), edit("a", "x", 1), edit("b", "y", 2)], &[], &none()).expect("fold");
    assert_eq!(forward, shuffled);
    assert_eq!(forward.applied, vec!["a", "b", "c"]);
}

/// ⏪️ Revert moves an edit to redo; a later reinstate restores it at its HLC position, not the tail.
#[test]
fn revert_and_reinstate_toggle_membership_in_hlc_order() {
    let edits = [edit("a", "x", 1), edit("b", "y", 3)];
    let reverted = fold_history(&edits, &[at(HistoryTransition::Revert { mutation_ids: vec![MutationId("op-a".into())] }, "x", 2)], &none()).expect("fold");
    assert_eq!(reverted.applied, vec!["b"]);
    assert_eq!(reverted.redo, vec!["a"]);
    let reinstated = fold_history(&edits, &[at(HistoryTransition::Revert { mutation_ids: vec![MutationId("op-a".into())] }, "x", 2), at(HistoryTransition::Reinstate { mutation_ids: vec![MutationId("op-a".into())] }, "x", 4)], &none()).expect("fold");
    assert_eq!(reinstated.applied, vec!["a", "b"]);
    assert!(reinstated.redo.is_empty());
}

/// ✏️ A new edit by the same author clears that author's redo stack; other authors' entries survive.
#[test]
fn an_authors_new_edit_clears_only_its_own_redo() {
    let edits = [edit("a", "x", 1), edit("b", "y", 2), edit("c", "x", 5)];
    let transitions = [at(HistoryTransition::Revert { mutation_ids: vec![MutationId("op-a".into())] }, "x", 3), at(HistoryTransition::Revert { mutation_ids: vec![MutationId("op-b".into())] }, "y", 4)];
    let fold = fold_history(&edits, &transitions, &none()).expect("fold");
    assert_eq!(fold.applied, vec!["c"]);
    assert_eq!(fold.redo, vec!["b"]);
}

/// 🚩️ Commit materializes change/checkpoint facts; checkout restores exactly the checkpoint's edits.
#[test]
fn commit_and_checkout_materialize_facts_and_positions() {
    let edits = [edit("a", "x", 1), edit("b", "x", 3)];
    let transitions = [at(commit("ck-1", None, &["a"]), "x", 2), at(commit("ck-2", Some("ck-1"), &["b"]), "x", 4), at(HistoryTransition::Checkout { checkpoint_id: "ck-1".into(), alternative_id: None }, "x", 5)];
    let fold = fold_history(&edits, &transitions, &none()).expect("fold");
    assert_eq!(fold.applied, vec!["a"]);
    assert_eq!(fold.checkpoint.as_deref(), Some("ck-1"));
    assert_eq!(fold.checkpoints[1].change_ids, vec!["change-ck-1", "change-ck-2"]);
    assert_eq!(fold.changes[1].edit_ids, vec!["b"]);
}

/// 🔀️ A concurrent edit older than a checkout is subject to it; a newer one lands on top — on every replica.
#[test]
fn checkout_governs_concurrent_edits_by_hlc() {
    let edits = [edit("a", "x", 1), edit("late-old", "y", 3), edit("late-new", "y", 6)];
    let transitions = [at(commit("ck-1", None, &["a"]), "x", 2), at(HistoryTransition::Checkout { checkpoint_id: "ck-1".into(), alternative_id: None }, "x", 5)];
    let fold = fold_history(&edits, &transitions, &none()).expect("fold");
    assert_eq!(fold.applied, vec!["a", "late-new"]);
}

/// 🌿️ Branch roots an alternative, commits grow its chain, repin re-identifies the checkpoint everywhere.
#[test]
fn branch_commit_and_repin_track_alternative_chains() {
    let edits = [edit("a", "x", 1), edit("b", "x", 4)];
    let transitions = [
        at(commit("ck-1", None, &["a"]), "x", 2),
        at(HistoryTransition::Branch { alternative_id: "alt".into(), name: "Variant".into(), checkpoint_id: "ck-1".into() }, "x", 3),
        at(commit("ck-2", Some("ck-1"), &["b"]), "x", 5),
        at(HistoryTransition::Repin { checkpoint_id: "ck-2".into(), pinned_checkpoint_id: "ck-2p".into(), pins: vec![TransitionPin { child_uri: "semio://c".into(), checkpoint_id: "c-1".into() }] }, "x", 6),
    ];
    let fold = fold_history(&edits, &transitions, &none()).expect("fold");
    assert_eq!(fold.alternative.as_deref(), Some("alt"));
    assert_eq!(fold.alternatives[0].checkpoint_ids, vec!["ck-1", "ck-2p"]);
    assert_eq!(fold.checkpoint.as_deref(), Some("ck-2p"));
    assert_eq!(fold.checkpoints[1].pins.len(), 1);
    assert_eq!(fold.applied, vec!["a", "b"]);
}

/// ⚔️ Quarantined edits never become active, even when a checkout names them.
#[test]
fn excluded_edits_stay_inactive() {
    let edits = [edit("a", "x", 1), edit("q", "y", 2)];
    let excluded: std::collections::HashSet<String> = ["q".to_string()].into_iter().collect();
    let transitions = [at(commit("ck-1", None, &["a", "q"]), "x", 3), at(HistoryTransition::Checkout { checkpoint_id: "ck-1".into(), alternative_id: None }, "x", 4)];
    assert_eq!(fold_history(&edits, &transitions, &excluded).expect("fold").applied, vec!["a"]);
}

/// 🚫️ A transition naming an unknown operation or checkpoint is refused instead of silently skipped.
#[test]
fn dangling_references_are_refused() {
    assert!(fold_history(&[], &[at(HistoryTransition::Revert { mutation_ids: vec![MutationId("op-ghost".into())] }, "x", 1)], &none()).is_err());
    assert!(fold_history(&[], &[at(HistoryTransition::Checkout { checkpoint_id: "ghost".into(), alternative_id: None }, "x", 1)], &none()).is_err());
}

/// 🚫️ The event set is a set: a transition delivered twice into one log is refused, never folded twice.
#[test]
fn a_repeated_transition_is_refused() {
    let edits = [edit("a", "x", 1)];
    let commit = at(commit("ck-1", None, &["a"]), "x", 2);
    assert!(fold_history(&edits, &[commit.clone(), commit], &none()).is_err());
}

fn fixture_transition(json: &serde_json::Value) -> HistoryTransition {
    let text = |key: &str| json[key].as_str().unwrap_or_else(|| panic!("fixture field {key}")).to_string();
    let optional = |key: &str| json[key].as_str().map(str::to_string);
    let ids = |key: &str| json[key].as_array().expect("fixture ids").iter().map(|id| MutationId(id.as_str().expect("fixture id").to_string())).collect::<Vec<_>>();
    match json["kind"].as_str().expect("fixture kind") {
        "revert" => HistoryTransition::Revert { mutation_ids: ids("mutationIds") },
        "reinstate" => HistoryTransition::Reinstate { mutation_ids: ids("mutationIds") },
        "commit" => HistoryTransition::Commit(TransitionCheckpoint {
            checkpoint_id: text("checkpointId"),
            parent_id: optional("parentId"),
            change_id: text("changeId"),
            mutation_ids: ids("mutationIds"),
            description: optional("description"),
            saved_at: text("savedAt"),
            authors: json["authors"].as_array().expect("fixture authors").iter().map(|author| TransitionAuthor { id: author["id"].as_str().expect("author id").into(), name: author["name"].as_str().expect("author name").into(), avatar: author["avatar"].as_str().map(str::to_string) }).collect(),
            message: optional("message"),
            timestamp: text("timestamp"),
        }),
        "branch" => HistoryTransition::Branch { alternative_id: text("alternativeId"), name: text("name"), checkpoint_id: text("checkpointId") },
        "checkout" => HistoryTransition::Checkout { checkpoint_id: text("checkpointId"), alternative_id: optional("alternativeId") },
        "repin" => HistoryTransition::Repin {
            checkpoint_id: text("checkpointId"),
            pinned_checkpoint_id: text("pinnedCheckpointId"),
            pins: json["pins"].as_array().expect("fixture pins").iter().map(|pin| TransitionPin { child_uri: pin["childUri"].as_str().expect("pin uri").into(), checkpoint_id: pin["checkpointId"].as_str().expect("pin checkpoint").into() }).collect(),
        },
        other => panic!("unknown fixture kind {other}"),
    }
}

#[test]
fn the_language_agnostic_fixture_matches_the_codec_byte_for_byte() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../../🧫️fixtures/🔀️history-transition-v1/🔣️.json")).expect("history transition fixture parses");
    assert_eq!(fixture["diffSchema"].as_str(), Some(HISTORY_TRANSITION_SCHEMA));
    for case in fixture["cases"].as_array().expect("fixture cases") {
        let id = case["id"].as_str().expect("case id");
        let hex = case["payloadHex"].as_str().expect("payload hex");
        let bytes = (0..hex.len()).step_by(2).map(|index| u8::from_str_radix(&hex[index..index + 2], 16).expect("hex byte")).collect::<Vec<_>>();
        match case["expect"]["outcome"].as_str().expect("outcome") {
            "accepted" => {
                let expected = fixture_transition(&case["expect"]["transition"]);
                assert_eq!(encode_history_transition(&expected), bytes, "{id}: encode");
                assert_eq!(decode_history_transition(&bytes).unwrap_or_else(|error| panic!("{id}: {error:?}")), expected, "{id}: decode");
            }
            "malformed" => {
                let detail = case["expect"]["detail"].as_str().expect("detail");
                let error = decode_history_transition(&bytes).expect_err(id);
                assert!(format!("{error:?}").to_lowercase().contains(detail), "{id}: {error:?} lacks {detail}");
            }
            other => panic!("{id}: unknown outcome {other}"),
        }
    }
}
