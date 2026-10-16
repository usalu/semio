//! 🧪️ Supersede fold laws through the public API of the history crate only.

use semio_framework_replication_history::{EffectiveSupersession, FoldEdit, HistoryFoldIndex, HistoryTransition, InputReplacement, SupersededInput, TransitionAuthor, TransitionCheckpoint, TransitionPin, TransitionSupersede, ViewerHead, fold_history, fold_history_for, history_transition_envelope, trunk_alternative_id};
use semio_framework_replication_ids::{ActorId, ArtifactId, HybridLogicalTimestamp, MutationId};

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
            line_id: optional("lineId"),
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

fn head(line: &str, checkpoint: Option<&str>) -> ViewerHead {
    ViewerHead { line_id: line.into(), checkpoint_id: checkpoint.map(str::to_string) }
}

fn none() -> std::collections::HashSet<String> {
    std::collections::HashSet::new()
}

fn supersede_fold_fixture() -> serde_json::Value {
    serde_json::from_str(include_str!("../../../🧪️tests/🧫️fixtures/🧫️supersede-fold/🔣️.json")).expect("supersede fold fixture parses")
}

fn fixture_step_envelope(document: &ArtifactId, step: &serde_json::Value) -> semio_framework_replication_causal::MutationEnvelope {
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

fn expected_supersessions(expect: &serde_json::Value) -> HistoryFoldIndex<MutationId, EffectiveSupersession> {
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
    assert_eq!(fixture["trunkAlternativeId"].as_str(), Some(trunk_alternative_id(&document).as_str()));
    let edits: Vec<FoldEdit> = fixture["edits"]
        .as_array()
        .expect("edits")
        .iter()
        .map(|row| FoldEdit {
            id: row["id"].as_str().expect("edit id").into(),
            actor: Some(row["actor"].as_str().expect("actor").into()),
            timestamp: HybridLogicalTimestamp { actor: 0, physical_ms: row["logicalMs"].as_u64().expect("logicalMs"), logical: 0 },
            mutation_ids: row["mutationIds"].as_array().expect("mutationIds").iter().map(|id| MutationId(id.as_str().expect("mutation id").into())).collect(),
            line: row["line"].as_str().map(str::to_string),
        })
        .collect();
    let mut transitions: Vec<semio_framework_replication_causal::MutationEnvelope> = Vec::new();
    for step in fixture["steps"].as_array().expect("steps") {
        let label = step["label"].as_str().unwrap_or("transition");
        match step["kind"].as_str().expect("step kind") {
            "transition" => transitions.push(fixture_step_envelope(&document, step)),
            "expect" => {
                let viewer = match step["expect"].get("head").filter(|head| !head.is_null()) {
                    Some(viewer) => head(viewer["lineId"].as_str().expect("head lineId"), viewer["checkpointId"].as_str()),
                    None => ViewerHead::canonical_trunk(&document),
                };
                let fold = fold_history_for(&document, &edits, &transitions, &none(), &viewer).unwrap_or_else(|error| panic!("{label}: {error:?}"));
                assert_eq!(fold.alternative, step["expect"]["alternative"].as_str().map(str::to_string), "{label}: alternative");
                assert_eq!(fold.supersessions, expected_supersessions(&step["expect"]), "{label}: supersessions");
            }
            "reload" => assert_eq!(fold_history(&document, &edits, &transitions, &none()).expect(label), fold_history(&document, &edits, &transitions, &none()).expect(label), "{label}"),
            "refuse" => {
                let mut refused = transitions.clone();
                refused.push(fixture_step_envelope(&document, &step["transition"]));
                let error = fold_history(&document, &edits, &refused, &none()).expect_err(label);
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
            line: row["line"].as_str().map(str::to_string),
        })
        .collect();
    let transitions: Vec<semio_framework_replication_causal::MutationEnvelope> = fixture["steps"].as_array().expect("steps").iter().filter(|step| step["kind"] == "transition").map(|step| fixture_step_envelope(&document, step)).collect();
    let expected = fold_history(&document, &edits, &transitions, &none()).expect("fold");
    assert!(!expected.supersessions.is_empty());
    for rotation in 1..transitions.len() {
        let mut shuffled = transitions.clone();
        shuffled.rotate_left(rotation);
        shuffled.reverse();
        assert_eq!(fold_history(&document, &edits, &shuffled, &none()).expect("fold"), expected, "rotation {rotation}");
    }
}
