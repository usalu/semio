use super::*;
use crate::editor::puzzle5d::commands::world_relocate::world_relocate;
use crate::editor::puzzle5d::config::Puzzle5dRuntime;
use crate::editor::puzzle5d::{capsule_dream_example_document, concrete_forest_example_document, find_part_by_grip_full_id, nakagin_example_document, Puzzle5dPart, Puzzle5dScene};
use crate::standards::v1::subsets::any::schema::mutations::text::Puzzle5dPlaySnapshot;
use std::time::{Duration, Instant};

const PROXIMITY_FIXTURE: &str = include_str!("../../🧫️fixtures/🔣️.json");

/// 🐢️ The search as it shipped before the spatial index — a scan of every grip of every part with a fastener
/// scan and two whole-document grip lookups per grip in reach — kept verbatim as the differential oracle.
fn reference_peers(document: &Puzzle5dDocument, part_id: &str, radius: f64, gated: bool) -> Vec<String> {
    let resolve_kind = |full_id: &str| find_part_by_grip_full_id(document, full_id).map(|(_, grip)| engine_grip_kind(grip)).filter(|kind| !kind.is_empty());
    let compatible = |source: &str, target: &str| {
        let Some(entries) = document.kind_compatibility.as_ref().and_then(serde_json::Value::as_array) else { return true };
        entries.is_empty()
            || entries.iter().any(|entry| {
                let rule_source = entry.get("source").and_then(serde_json::Value::as_str).unwrap_or("");
                let rule_target = entry.get("target").and_then(serde_json::Value::as_str).unwrap_or("");
                let bidirectional = entry.get("bidirectional").and_then(serde_json::Value::as_bool).unwrap_or(false);
                (rule_source == source && rule_target == target) || (bidirectional && rule_source == target && rule_target == source)
            })
    };
    let Some((moved_id, moved_position)) = document.parts.iter().find(|part| part.id == part_id).and_then(|part| part.grips.first().map(|grip| (puzzle5d_grip_full_id(&part.id, &grip.id), world_grip_position(part, grip)))) else { return Vec::new() };
    let mut peers = Vec::new();
    for other in document.parts.iter().filter(|other| other.id != part_id) {
        for grip in &other.grips {
            let peer_id = puzzle5d_grip_full_id(&other.id, &grip.id);
            let position = world_grip_position(other, grip);
            let distance = ((moved_position[0] - position[0]).powi(2) + (moved_position[1] - position[1]).powi(2) + (moved_position[2] - position[2]).powi(2)).sqrt();
            if peer_id == moved_id || distance > radius || document.fasteners.iter().any(|fastener| (fastener.source == peer_id && fastener.target == moved_id) || (fastener.source == moved_id && fastener.target == peer_id)) {
                continue;
            }
            if gated && !match (resolve_kind(&peer_id), resolve_kind(&moved_id)) {
                (Some(source), Some(target)) => compatible(&source, &target),
                _ => true,
            } {
                continue;
            }
            peers.push(peer_id);
        }
    }
    peers
}

fn example(id: &str) -> Puzzle5dDocument {
    match id {
        "concrete-forest" => concrete_forest_example_document(),
        "nakagin-capsule-tower" => nakagin_example_document(),
        "capsule-dream" => capsule_dream_example_document(),
        other => panic!("unknown example {other}"),
    }
}

/// ⚖️ LAW (language-neutral fixture, regression at maximum document size): for every sampled part of every
/// shipped example — up to the 2 880-part capsule dream — the bounded proximity search finds exactly the peers the
/// pre-bound scan found, in the same order, with and without the kind gate; `proximityConnect` and `worldRelocate`
/// fasten exactly those peers; and each one-shot command stays under the interactive ceiling (per part, the best of
/// the fixture's rounds, which sweep every sampled part once each so one scheduler stall never covers every round of
/// one part).
#[test]
fn proximity_search_matches_the_scan_and_stays_below_the_interactive_ceiling_at_maximum_document_size() {
    let fixture: serde_json::Value = serde_json::from_str(PROXIMITY_FIXTURE).expect("proximity fixture parses");
    let radius = fixture["radius"].as_f64().expect("radius");
    assert_eq!(radius, PUZZLE5D_PROXIMITY_RADIUS);
    let budget = Duration::from_micros(fixture["budgetUs"].as_u64().expect("budget"));
    let rounds = fixture["rounds"].as_u64().expect("rounds") as usize;
    let snapshot = Puzzle5dPlaySnapshot(serde_json::Value::Null);
    let selection = protocol::DomainSelection::default();
    let peers = |document: &Puzzle5dDocument, part_id: &str, gated: bool| puzzle5d_proximity_peers(document, part_id, radius, gated).map(|(_, peers)| peers.into_iter().map(|peer| peer.grip).collect::<Vec<_>>()).unwrap_or_default();
    for case in fixture["cases"].as_array().expect("cases") {
        let name = case["example"].as_str().expect("example");
        let document = example(name);
        let sampled: Vec<&Puzzle5dPart> = document.parts.iter().step_by(case["partStride"].as_u64().expect("stride") as usize).collect();
        let (mut moved, mut proximity_peers, mut relocate_peers) = (0u64, 0u64, 0u64);
        for part in &sampled {
            let (gated, ungated) = (peers(&document, &part.id, true), peers(&document, &part.id, false));
            assert_eq!(gated, reference_peers(&document, &part.id, radius, true), "{name}: gated peers of {}", part.id);
            assert_eq!(ungated, reference_peers(&document, &part.id, radius, false), "{name}: ungated peers of {}", part.id);
            moved += u64::from(!part.grips.is_empty());
            proximity_peers += gated.len() as u64;
            relocate_peers += ungated.len() as u64;
        }
        assert_eq!(moved, case["moved"].as_u64().expect("moved"), "{name}: moved parts");
        assert_eq!(proximity_peers, case["proximityPeers"].as_u64().expect("proximity peers"), "{name}: proximity peers");
        assert_eq!(relocate_peers, case["relocatePeers"].as_u64().expect("relocate peers"), "{name}: relocate peers");
        let mut best = vec![(Duration::MAX, Duration::MAX); sampled.len()];
        for _ in 0..rounds {
            for (index, part) in sampled.iter().enumerate() {
                let mut scene = Puzzle5dScene { document: document.clone(), runtime: Puzzle5dRuntime::default(), active_utility: "select".into() };
                let mut ctx = Puzzle5dActionCtx { scene: &mut scene, snapshot: &snapshot, instance_owner: None, window_id: "puzzle5d-3d", window_kind: "puzzle5d-3d", selection: &selection, view_state: None, tool_run: None, effects: Vec::new(), interaction_writes: Vec::new(), abort: false };
                let started = Instant::now();
                proximity_connect(&mut ctx, Some(&dsl::json!({ "partId": part.id.as_str() })));
                best[index].0 = best[index].0.min(started.elapsed());
                assert_eq!(scene.document.fasteners[document.fasteners.len()..].iter().map(|fastener| fastener.source.clone()).collect::<Vec<_>>(), peers(&document, &part.id, true), "{name}: proximityConnect fastens the gated peers as sources");
                let mut scene = Puzzle5dScene { document: document.clone(), runtime: Puzzle5dRuntime::default(), active_utility: "select".into() };
                let mut ctx = Puzzle5dActionCtx { scene: &mut scene, snapshot: &snapshot, instance_owner: None, window_id: "puzzle5d-3d", window_kind: "puzzle5d-3d", selection: &selection, view_state: None, tool_run: None, effects: Vec::new(), interaction_writes: Vec::new(), abort: false };
                let started = Instant::now();
                world_relocate(&mut ctx, Some(&dsl::json!({ "objectId": part.id.as_str(), "position": part.part_3d.origin.to_vec() })));
                best[index].1 = best[index].1.min(started.elapsed());
                assert_eq!(scene.document.fasteners[document.fasteners.len()..].iter().map(|fastener| fastener.target.clone()).collect::<Vec<_>>(), peers(&document, &part.id, false), "{name}: worldRelocate fastens the ungated peers as targets");
            }
        }
        let worst_proximity = best.iter().map(|best| best.0).max().unwrap_or_default();
        let worst_relocate = best.iter().map(|best| best.1).max().unwrap_or_default();
        assert!(worst_proximity < budget, "{name}: worst proximityConnect {worst_proximity:?} exceeds {budget:?}");
        assert!(worst_relocate < budget, "{name}: worst worldRelocate {worst_relocate:?} exceeds {budget:?}");
    }
}
