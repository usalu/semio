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
        HistoryTransition::Supersede(TransitionSupersede { scope: None, inputs: vec![input("op-a", &[1, 2, 3]), SupersededInput { target: MutationId("op-b".into()), replacement: InputReplacement::Withdrawn }] }),
        HistoryTransition::Supersede(TransitionSupersede { scope: Some("alt-1".into()), inputs: vec![input("op-a", &[])] }),
    ]
}

fn input(target: &str, payload: &[u8]) -> SupersededInput {
    SupersededInput { target: MutationId(target.into()), replacement: InputReplacement::Input { schema: "demo/v1".into(), payload: payload.to_vec() } }
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

/// 🛂️ The actor string is authentication metadata, not content: the same authored transition (HLC and payload)
/// has the same id whichever actor string an author, a relay or the hub's socket binding stamps on it.
#[test]
fn a_transition_id_ignores_the_actor_string() {
    let tick = HybridLogicalTimestamp { actor: 2_557_761_449, physical_ms: 1_790_622_765_027, logical: 3 };
    for transition in every_transition() {
        let authored = history_transition_envelope(&transition, &ArtifactId("doc".into()), &ActorId("local".into()), Vec::new(), tick);
        let rebound = history_transition_envelope(&transition, &ArtifactId("doc".into()), &ActorId("hub.v1.370722f0cfb78d4e3eaf44a8b576001a0afd3711c6d35e600e17b516da8ac1a0".into()), Vec::new(), tick);
        assert_eq!(authored.mutation_id, rebound.mutation_id, "{transition:?}");
        assert_eq!(authored.mutation_id, history_transition_id(&tick, &authored.diff.payload));
        assert_ne!(authored.mutation_id, history_transition_envelope(&transition, &ArtifactId("doc".into()), &ActorId("local".into()), Vec::new(), HybridLogicalTimestamp { actor: tick.actor + 1, ..tick }).mutation_id, "the numeric HLC actor stays part of the address");
    }
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

/// 🛂️ An undo belongs to its author: a revert or reinstate naming another actor's operation — alone or beside the
/// actor's own — withdraws or restores nothing and is listed as refused, on every replica.
#[test]
fn a_transition_naming_another_actors_operation_is_refused_whole() {
    let edits = [edit("a", "x", 1), edit("b", "y", 2)];
    let foreign = at(HistoryTransition::Revert { mutation_ids: vec![MutationId("op-a".into())] }, "y", 3);
    let mixed = at(HistoryTransition::Revert { mutation_ids: vec![MutationId("op-a".into()), MutationId("op-b".into())] }, "y", 4);
    let fold = fold_history(&edits, &[foreign.clone(), mixed.clone()], &none()).expect("fold");
    assert_eq!(fold.applied, vec!["a", "b"]);
    assert!(fold.redo.is_empty());
    assert_eq!(fold.refused, vec![foreign.mutation_id.0.clone(), mixed.mutation_id.0.clone()]);
    let own = at(HistoryTransition::Revert { mutation_ids: vec![MutationId("op-a".into())] }, "x", 5);
    let stolen = at(HistoryTransition::Reinstate { mutation_ids: vec![MutationId("op-a".into())] }, "y", 6);
    let fold = fold_history(&edits, &[own, stolen.clone()], &none()).expect("fold");
    assert_eq!(fold.applied, vec!["b"]);
    assert_eq!(fold.redo, vec!["a"]);
    assert_eq!(fold.refused, vec![stolen.mutation_id.0.clone()]);
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
        "supersede" => HistoryTransition::Supersede(TransitionSupersede {
            scope: optional("scope"),
            inputs: json["inputs"]
                .as_array()
                .expect("fixture inputs")
                .iter()
                .map(|input| SupersededInput { target: MutationId(input["target"].as_str().expect("input target").into()), replacement: fixture_replacement(&input["replacement"]) })
                .collect(),
        }),
        other => panic!("unknown fixture kind {other}"),
    }
}

fn fixture_hex(hex: &str) -> Vec<u8> {
    (0..hex.len()).step_by(2).map(|index| u8::from_str_radix(&hex[index..index + 2], 16).expect("hex byte")).collect()
}

fn fixture_replacement(json: &serde_json::Value) -> InputReplacement {
    match json["kind"].as_str().expect("replacement kind") {
        "input" => InputReplacement::Input { schema: json["schema"].as_str().expect("replacement schema").into(), payload: fixture_hex(json["payloadHex"].as_str().expect("replacement payloadHex")) },
        "withdrawn" => InputReplacement::Withdrawn,
        other => panic!("unknown replacement kind {other}"),
    }
}

/// ♻️ Language-agnostic durable collaborative redo: two authors interleave, each undoes/redoes only
/// their own mutations, another actor's revert/reinstate of them is refused, and reload/`hub-restart`
/// steps re-fold the same event set to the same applied/redo/refused projection (pure durability
/// proof shared with the TypeScript runner).
#[test]
fn durable_collaborative_redo_fixture_survives_reload_and_hub_restart() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../../🧫️fixtures/🗄️durable-collaborative-redo-v1/🔣️.json")).expect("durable collaborative redo fixture parses");
    assert_eq!(fixture["schema"].as_str(), Some("semio.history.durable-collaborative-redo.v1"));
    let edits: Vec<FoldEdit> = fixture["edits"]
        .as_array()
        .expect("edits")
        .iter()
        .map(|row| FoldEdit {
            id: row["id"].as_str().expect("edit id").into(),
            actor: Some(row["actor"].as_str().expect("actor").into()),
            timestamp: HybridLogicalTimestamp { actor: 0, physical_ms: row["logicalMs"].as_u64().expect("logicalMs"), logical: 0 },
            mutation_ids: row["mutationIds"].as_array().expect("mutationIds").iter().map(|id| MutationId(id.as_str().expect("mutation id").into())).collect(),
        })
        .collect();
    let mut transitions: Vec<crate::causal::MutationEnvelope> = Vec::new();
    let document = ArtifactId(fixture["documentId"].as_str().expect("documentId").into());
    let none = none();
    for step in fixture["steps"].as_array().expect("steps") {
        match step["kind"].as_str().expect("kind") {
            "revert" | "reinstate" => {
                let mutation_ids = step["mutationIds"].as_array().expect("mutationIds").iter().map(|id| MutationId(id.as_str().expect("id").into())).collect::<Vec<_>>();
                let transition = if step["kind"] == "revert" { HistoryTransition::Revert { mutation_ids } } else { HistoryTransition::Reinstate { mutation_ids } };
                let actor = ActorId(step["actor"].as_str().expect("actor").into());
                let clock = HybridLogicalTimestamp { actor: 0, physical_ms: step["logicalMs"].as_u64().expect("logicalMs"), logical: 0 };
                let mut envelope = history_transition_envelope(&transition, &document, &actor, Vec::new(), clock);
                if let Some(id) = step["id"].as_str() {
                    envelope.mutation_id = MutationId(id.into());
                }
                transitions.push(envelope);
            }
            "expect" => {
                let fold = fold_history(&edits, &transitions, &none).unwrap_or_else(|error| panic!("{}: {error:?}", step["label"].as_str().unwrap_or("expect")));
                let applied: Vec<String> = step["expect"]["applied"].as_array().expect("applied").iter().map(|id| id.as_str().expect("id").into()).collect();
                let redo: Vec<String> = step["expect"]["redo"].as_array().expect("redo").iter().map(|id| id.as_str().expect("id").into()).collect();
                assert_eq!(fold.applied, applied, "{}", step["label"].as_str().unwrap_or("applied"));
                assert_eq!(fold.redo, redo, "{}", step["label"].as_str().unwrap_or("redo"));
                let refused: Vec<String> = step["expect"]["refused"].as_array().expect("refused").iter().map(|id| id.as_str().expect("id").into()).collect();
                assert_eq!(fold.refused, refused, "{}", step["label"].as_str().unwrap_or("refused"));
            }
            "reload" | "hub-restart" => {
                let first = fold_history(&edits, &transitions, &none).expect("fold before restart");
                let second = fold_history(&edits, &transitions, &none).expect("fold after restart");
                assert_eq!(first, second, "{} must be a pure re-fold of the same event set", step["label"].as_str().unwrap_or("restart"));
            }
            other => panic!("unknown step kind {other}"),
        }
    }
    let observations: Vec<&str> = fixture["observations"].as_array().expect("observations").iter().map(|row| row.as_str().expect("obs")).collect();
    assert!(observations.contains(&"durable-collaborative-redo"));
    assert!(observations.contains(&"survives-hub-restart"));
    assert!(observations.contains(&"foreign-transition-refused"));
}

#[test]
fn the_language_agnostic_fixture_matches_the_codec_byte_for_byte() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../../🧫️fixtures/🧫️history-transition/🔣️.json")).expect("history transition fixture parses");
    assert_eq!(fixture["diffSchema"].as_str(), Some(HISTORY_TRANSITION_SCHEMA));
    let clock = HybridLogicalTimestamp { actor: fixture["idClock"]["actor"].as_u64().expect("idClock actor"), physical_ms: fixture["idClock"]["physicalMs"].as_u64().expect("idClock physicalMs"), logical: fixture["idClock"]["logical"].as_u64().expect("idClock logical") };
    for case in fixture["cases"].as_array().expect("fixture cases") {
        let id = case["id"].as_str().expect("case id");
        let hex = case["payloadHex"].as_str().expect("payload hex");
        let bytes = (0..hex.len()).step_by(2).map(|index| u8::from_str_radix(&hex[index..index + 2], 16).expect("hex byte")).collect::<Vec<_>>();
        match case["expect"]["outcome"].as_str().expect("outcome") {
            "accepted" => {
                let expected = fixture_transition(&case["expect"]["transition"]);
                assert_eq!(encode_history_transition(&expected), bytes, "{id}: encode");
                assert_eq!(decode_history_transition(&bytes).unwrap_or_else(|error| panic!("{id}: {error:?}")), expected, "{id}: decode");
                let transition_id = case["expect"]["transitionId"].as_str().expect("transitionId");
                assert_eq!(history_transition_id(&clock, &bytes).0, transition_id, "{id}: first-party id");
                let mut material = Vec::new();
                crate::wire::write_varint_u64(&mut material, clock.actor);
                crate::wire::write_varint_u64(&mut material, clock.physical_ms);
                crate::wire::write_varint_u64(&mut material, clock.logical);
                crate::write_bytes(&mut material, &bytes);
                assert_eq!(format!("transition-{}", &blake3::hash(&material).to_hex()[..16]), transition_id, "{id}: third-party blake3 id");
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

/// 🛂️ Authoring refuses exactly what the codec refuses: no input, a repeated target, an oversized scope or payload.
#[test]
fn supersede_validation_refuses_what_the_codec_refuses() {
    let valid = TransitionSupersede { scope: Some("s".repeat(SUPERSEDE_SCOPE_MAX_BYTES)), inputs: vec![input("op-a", &vec![0; SUPERSEDE_PAYLOAD_MAX_BYTES])] };
    assert!(valid.validate().is_ok());
    assert_eq!(decode_history_transition(&encode_history_transition(&HistoryTransition::Supersede(valid.clone()))).expect("the bounds are inclusive"), HistoryTransition::Supersede(valid));
    let refusals = [
        (TransitionSupersede { scope: None, inputs: Vec::new() }, "supersede names no input"),
        (TransitionSupersede { scope: None, inputs: vec![input("op-a", &[1]), input("op-a", &[2])] }, "supersede repeats target op-a"),
        (TransitionSupersede { scope: Some("s".repeat(SUPERSEDE_SCOPE_MAX_BYTES + 1)), inputs: vec![input("op-a", &[])] }, "supersede scope exceeds 256 bytes"),
        (TransitionSupersede { scope: None, inputs: vec![input("op-a", &vec![0; SUPERSEDE_PAYLOAD_MAX_BYTES + 1])] }, "supersede payload exceeds 262144 bytes"),
    ];
    for (supersede, detail) in refusals {
        assert!(format!("{:?}", supersede.validate().expect_err(detail)).contains(detail), "{detail}");
        let bytes = encode_history_transition(&HistoryTransition::Supersede(supersede));
        assert!(format!("{:?}", decode_history_transition(&bytes).expect_err(detail)).contains(detail), "{detail}: decode");
    }
}

/// 🔗️ A supersession's envelope depends on its targets, so a replica holds it until every superseded operation arrived.
#[test]
fn a_supersede_envelope_depends_on_its_targets() {
    let supersede = TransitionSupersede { scope: None, inputs: vec![input("op-b", &[1]), input("op-a", &[2])] };
    let envelope = history_transition_envelope(&HistoryTransition::Supersede(supersede.clone()), &ArtifactId("doc".into()), &ActorId("x".into()), supersede.targets(), HybridLogicalTimestamp::new(0, 1));
    assert_eq!(envelope.dependencies, vec![MutationId("op-b".into()), MutationId("op-a".into())]);
    assert!(envelope.target.is_empty());
    assert_eq!(envelope.transaction, None);
}

fn supersede_fold_fixture() -> serde_json::Value {
    serde_json::from_str(include_str!("../../../🧫️fixtures/🧫️supersede-fold/🔣️.json")).expect("supersede fold fixture parses")
}

fn fixture_step_envelope(document: &ArtifactId, step: &serde_json::Value) -> crate::causal::MutationEnvelope {
    let transition = fixture_transition(&step["transition"]);
    let dependencies = match &transition {
        HistoryTransition::Supersede(supersede) => supersede.targets(),
        _ => Vec::new(),
    };
    let clock = HybridLogicalTimestamp { actor: 0, physical_ms: step["logicalMs"].as_u64().expect("logicalMs"), logical: 0 };
    let mut envelope = history_transition_envelope(&transition, document, &ActorId(step["actor"].as_str().expect("actor").into()), dependencies, clock);
    envelope.mutation_id = MutationId(step["id"].as_str().expect("transition id").into());
    envelope
}

fn expected_supersessions(expect: &serde_json::Value) -> std::collections::BTreeMap<MutationId, EffectiveSupersession> {
    expect["supersessions"]
        .as_array()
        .expect("supersessions")
        .iter()
        .map(|row| {
            (
                MutationId(row["target"].as_str().expect("target").into()),
                EffectiveSupersession {
                    transition_id: row["transitionId"].as_str().expect("transitionId").into(),
                    actor: row["actor"].as_str().expect("actor").into(),
                    timestamp: HybridLogicalTimestamp { actor: 0, physical_ms: row["logicalMs"].as_u64().expect("logicalMs"), logical: 0 },
                    scope: row["scope"].as_str().map(str::to_string),
                    replacement: fixture_replacement(&row["replacement"]),
                },
            )
        })
        .collect()
}

/// ✏️ Language-agnostic supersede fold law (shared with the TypeScript fold twin): last-wins by `(hlc, id)` whatever
/// the arrival order, no ownership rule, scope against the final alternative, composition with revert/commit/branch/
/// checkout, a pure re-fold on reload, and an unknown target refused as a fold error.
#[test]
fn the_supersede_fold_fixture_matches_the_fold_law() {
    let fixture = supersede_fold_fixture();
    assert_eq!(fixture["schema"].as_str(), Some("semio.history.supersede-fold"));
    let document = ArtifactId(fixture["documentId"].as_str().expect("documentId").into());
    let edits: Vec<FoldEdit> = fixture["edits"]
        .as_array()
        .expect("edits")
        .iter()
        .map(|row| FoldEdit {
            id: row["id"].as_str().expect("edit id").into(),
            actor: Some(row["actor"].as_str().expect("actor").into()),
            timestamp: HybridLogicalTimestamp { actor: 0, physical_ms: row["logicalMs"].as_u64().expect("logicalMs"), logical: 0 },
            mutation_ids: row["mutationIds"].as_array().expect("mutationIds").iter().map(|id| MutationId(id.as_str().expect("mutation id").into())).collect(),
        })
        .collect();
    let mut transitions: Vec<crate::causal::MutationEnvelope> = Vec::new();
    for step in fixture["steps"].as_array().expect("steps") {
        let label = step["label"].as_str().unwrap_or("transition");
        match step["kind"].as_str().expect("step kind") {
            "transition" => transitions.push(fixture_step_envelope(&document, step)),
            "expect" => {
                let fold = fold_history(&edits, &transitions, &none()).unwrap_or_else(|error| panic!("{label}: {error:?}"));
                assert_eq!(fold.alternative, step["expect"]["alternative"].as_str().map(str::to_string), "{label}: alternative");
                assert_eq!(fold.supersessions, expected_supersessions(&step["expect"]), "{label}: supersessions");
            }
            "reload" => assert_eq!(fold_history(&edits, &transitions, &none()).expect(label), fold_history(&edits, &transitions, &none()).expect(label), "{label}"),
            "refuse" => {
                let mut refused = transitions.clone();
                refused.push(fixture_step_envelope(&document, &step["transition"]));
                let error = fold_history(&edits, &refused, &none()).expect_err(label);
                assert!(format!("{error:?}").contains(step["detail"].as_str().expect("detail")), "{label}: {error:?}");
            }
            other => panic!("unknown step kind {other}"),
        }
    }
}

/// 🔀️ Supersessions are a function of the event SET: every rotation of the fixture's transitions folds to the same projection.
#[test]
fn supersessions_are_a_function_of_the_event_set() {
    let fixture = supersede_fold_fixture();
    let document = ArtifactId(fixture["documentId"].as_str().expect("documentId").into());
    let edits: Vec<FoldEdit> = fixture["edits"]
        .as_array()
        .expect("edits")
        .iter()
        .map(|row| FoldEdit {
            id: row["id"].as_str().expect("edit id").into(),
            actor: Some(row["actor"].as_str().expect("actor").into()),
            timestamp: HybridLogicalTimestamp { actor: 0, physical_ms: row["logicalMs"].as_u64().expect("logicalMs"), logical: 0 },
            mutation_ids: row["mutationIds"].as_array().expect("mutationIds").iter().map(|id| MutationId(id.as_str().expect("mutation id").into())).collect(),
        })
        .collect();
    let transitions: Vec<crate::causal::MutationEnvelope> = fixture["steps"].as_array().expect("steps").iter().filter(|step| step["kind"] == "transition").map(|step| fixture_step_envelope(&document, step)).collect();
    let expected = fold_history(&edits, &transitions, &none()).expect("fold");
    assert!(!expected.supersessions.is_empty());
    for rotation in 1..transitions.len() {
        let mut shuffled = transitions.clone();
        shuffled.rotate_left(rotation);
        shuffled.reverse();
        assert_eq!(fold_history(&edits, &shuffled, &none()).expect("fold"), expected, "rotation {rotation}");
    }
}

/// 🌉️ The value shapes of a replacement and an effective supersession round-trip.
#[test]
fn supersession_values_round_trip() {
    for replacement in [InputReplacement::Input { schema: "demo/v1".into(), payload: vec![1, 2] }, InputReplacement::Withdrawn] {
        let supersession = EffectiveSupersession { transition_id: "transition-1".into(), actor: "alice".into(), timestamp: HybridLogicalTimestamp::new(3, 4), scope: Some("alt".into()), replacement };
        let decoded: EffectiveSupersession = crate::value::FromValue::from_value(crate::value::ToValue::to_value(&supersession)).expect("decode");
        assert_eq!(decoded, supersession);
    }
}
