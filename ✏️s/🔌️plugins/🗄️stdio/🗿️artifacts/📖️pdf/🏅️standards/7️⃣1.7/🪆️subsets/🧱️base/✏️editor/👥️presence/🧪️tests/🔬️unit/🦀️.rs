//! 🧪️ Presence lane tests: the fold and overlay projection against the language-neutral fixture, the lane's own mutation laws, the layers and the intent reducer.

use crate::{peer_layers, presence_of, set_presence, PdfPresence, PdfPresenceDiff, PdfPresenceMutation, PeerBoard, PeerStatus, PresenceEvent, PresenceIntent, SetPresence, AWAY_AFTER_MS};
use protocol::os_spr::protocol_laws::assert_mutation_inverse_sum_law;
use protocol::{ColdMutation, Mutation};
use semio_ui_locale::Locale;
use serde_json::{json, Value};

const TWO_COLLABORATORS: &str = include_str!("../🔬️fixtures/🧮️two-collaborators.json");

fn status(board: &PeerBoard, now: u64) -> std::collections::BTreeMap<String, &'static str> {
    board.overlay(now, 0).roster.iter().map(|row| (row.mark.actor.clone(), if row.mark.status == PeerStatus::Live { "live" } else { "away" })).collect()
}

/// 🧮️ The fixture's events fold to the fixture's overlays, step by step.
#[test]
fn fixture_events_fold_to_the_expected_overlays() {
    let fixture: Value = serde_json::from_str(TWO_COLLABORATORS).unwrap();
    let mut board = PeerBoard::default();
    for (index, step) in fixture["steps"].as_array().unwrap().iter().enumerate() {
        if let Some(event) = step.get("event") {
            board.fold(PresenceEvent::from_json(event).unwrap_or_else(|| panic!("step {index}: unreadable event")));
            continue;
        }
        let (at, page) = (step["at"].as_u64().unwrap(), step["page"].as_u64().unwrap() as u32);
        let overlay = board.overlay(at, page);
        if let Some(expected) = step.get("expect") {
            assert_eq!(&overlay.to_json(), expected, "step {index}");
        }
        if let Some(pages) = step.get("expectRosterPages") {
            for row in &overlay.roster {
                assert_eq!(pages[&row.mark.actor].as_u64().unwrap() as u32, row.page, "step {index}: {}", row.mark.actor);
            }
        }
        if let Some(expected) = step.get("expectStatus") {
            let actual = status(&board, at);
            for (actor, state) in expected.as_object().unwrap() {
                assert_eq!(actual.get(actor).copied(), state.as_str(), "step {index}: {actor}");
            }
        }
        if let Some(count) = step.get("expectPointers") {
            assert_eq!(overlay.pointers.len() as u64, count.as_u64().unwrap(), "step {index}");
        }
        if let Some(count) = step.get("expectSelections") {
            assert_eq!(overlay.selections.len() as u64, count.as_u64().unwrap(), "step {index}");
        }
        if let Some(offline) = step.get("expectOffline") {
            assert_eq!(board.offline(), offline.as_bool().unwrap(), "step {index}");
        }
        if let Some(peers) = step.get("expectPeers") {
            let actual: Vec<_> = overlay.roster.iter().map(|row| row.mark.actor.clone()).collect();
            assert_eq!(json!(actual), *peers, "step {index}");
        }
    }
}

/// ⚖️ Folding is idempotent for a repeated heartbeat and independent of the arrival order of stale ones.
#[test]
fn stale_and_repeated_heartbeats_do_not_change_the_board() {
    let fixture: Value = serde_json::from_str(TWO_COLLABORATORS).unwrap();
    let newer = PresenceEvent::from_json(&fixture["steps"][0]["event"]).unwrap();
    let mut once = PeerBoard::default();
    once.fold(newer.clone());
    let mut twice = once.clone();
    twice.fold(newer);
    assert_eq!(once, twice);
}

/// 🗑️ The board is bounded: the 65th collaborator evicts the longest unheard one.
#[test]
fn the_board_keeps_at_most_as_many_peers_as_the_framework_table() {
    let mut board = PeerBoard::default();
    for index in 0..70u64 {
        board.fold(PresenceEvent::Heard { at_ms: 100 + index, sample: crate::PeerSample { actor: format!("peer{index:02}"), label: String::new(), color: None, presence: PdfPresence::default(), selection: Vec::new() } });
    }
    assert_eq!(board.len(), crate::PEER_SLOTS);
    assert!(status(&board, 200).contains_key("peer69") && !status(&board, 200).contains_key("peer00"));
}

/// ♿️ Every peer is described as text in both languages, away peers say so, and unnamed peers fall back to their actor id.
#[test]
fn roster_is_described_in_both_languages() {
    let fixture: Value = serde_json::from_str(TWO_COLLABORATORS).unwrap();
    let mut board = PeerBoard::default();
    board.fold(PresenceEvent::from_json(&fixture["steps"][0]["event"]).unwrap());
    let live = board.overlay(1500, 2);
    assert_eq!(live.describe(Locale::En), vec!["Ada is on page 3, 2 objects selected, text caret placed".to_string()]);
    assert_eq!(live.describe(Locale::De), vec!["Ada ist auf Seite 3, 2 Objekte ausgewählt, Textcursor gesetzt".to_string()]);
    let away = board.overlay(1000 + AWAY_AFTER_MS, 2);
    assert!(away.describe(Locale::En)[0].ends_with("(away)") && away.describe(Locale::De)[0].ends_with("(abwesend)"));
}

/// 🖍️ Live peers draw pointer, viewport, caret and selection; away peers keep only a dimmed selection frame.
#[test]
fn layers_follow_the_overlay_and_dim_away_peers() {
    let fixture: Value = serde_json::from_str(TWO_COLLABORATORS).unwrap();
    let mut board = PeerBoard::default();
    board.fold(PresenceEvent::from_json(&fixture["steps"][0]["event"]).unwrap());
    let object = |id: &str| (id == "p2.o3").then_some([10.0, 20.0, 30.0, 40.0]);
    let live = peer_layers(&board.overlay(1500, 2), [0.0, 0.0, 600.0, 800.0], &object);
    let ids: Vec<_> = live.iter().map(|layer| layer["id"].as_str().unwrap()).collect();
    assert_eq!(ids, ["peer.ada.selection.p2.o3", "peer.ada.viewport", "peer.ada.caret", "peer.ada.pointer"]);
    assert_eq!(live[3]["segments"][0]["to"], json!([150.0, 400.0]));
    let away = peer_layers(&board.overlay(9000, 2), [0.0, 0.0, 600.0, 800.0], &object);
    assert_eq!(away.len(), 1);
    assert!(away[0]["stroke"]["color"][3].as_f64().unwrap() < 0.5);
}

/// 🧭️ A page change drops the pointer; pointer and caret intents set and clear their fields only.
#[test]
fn intents_reduce_presence() {
    let snapshot = Default::default();
    let on_two = PdfPresence { page: 2, pointer_active: true, pointer_x: 10, pointer_y: 20, ..Default::default() };
    let view = semio_s_artifact_stdio_pdf_page_editor_view::ViewState { page: 3, ..Default::default() };
    let moved = crate::reduce_presence(&snapshot, &on_two, &PresenceIntent::View { view, viewport: (1024.0, 768.0) });
    assert!(moved.page == 3 && !moved.pointer_active);
    let pointed = crate::reduce_presence(&snapshot, &moved, &PresenceIntent::Pointer { page: 3, x: 5000, y: 7 });
    assert_eq!((pointed.pointer_x, pointed.pointer_y, pointed.pointer_active), (1000, 7, true));
    let careted = crate::reduce_presence(&snapshot, &pointed, &PresenceIntent::Caret { object: "p3.o1".into(), offset: 2, anchor: 0 });
    assert_eq!(crate::reduce_presence(&snapshot, &careted, &PresenceIntent::CaretClear).caret_object, "");
    assert_eq!(crate::presence_intent("presence-caret", &json!({ "object": "p3.o1", "x": 2.0, "y": 2.0 }), 5), Ok(Some(PresenceIntent::Caret { object: "p3.o1".into(), offset: 2, anchor: 2 })));
    assert_eq!(crate::presence_intent("zoom-in", &json!({}), 5), Ok(None));
}

/// 🔢️ A pointer's page is the one-based number a person reads: number 4 of 5 is index 3, and 0 or 6 is outside `1..=5`.
#[test]
fn pointer_page_is_a_one_based_number() {
    let pointer = |page: u64| crate::presence_intent("presence-pointer", &json!({ "page": page, "x": 10.0, "y": 20.0 }), 5);
    assert_eq!(pointer(4), Ok(Some(PresenceIntent::Pointer { page: 3, x: 10, y: 20 })));
    assert_eq!(pointer(1), Ok(Some(PresenceIntent::Pointer { page: 0, x: 10, y: 20 })));
    assert_eq!(pointer(0), Err(semio_s_artifact_stdio_pdf_page_editor_view::PageNumberRange { number: 0, last: 5 }));
    assert_eq!(pointer(6), Err(semio_s_artifact_stdio_pdf_page_editor_view::PageNumberRange { number: 6, last: 5 }));
}

fn peer() -> PdfPresence {
    PdfPresence { page: 4, pointer_active: true, pointer_x: 100, pointer_y: 200, view_left: 0, view_top: 0, view_right: 1000, view_bottom: 500, caret_object: "p4.o9".into(), caret_offset: 3, caret_anchor: 1 }
}

/// ⚖️ A set that changes several fields restores every one of them exactly, and the leaf round trips the presence.
#[semio_framework_async_macros::async_test]
async fn setting_presence_inverts_to_the_negative_diff() {
    let mutation = PdfPresenceMutation::SetPresence(SetPresence { page: 1, caret_object: String::new(), ..set_presence(&peer()) });
    assert_mutation_inverse_sum_law(&mutation, &peer()).await;
    assert_eq!(presence_of(&set_presence(&peer())), peer());
}

/// ⚖️ A set that changes one field carries only that field in its diff.
#[semio_framework_async_macros::async_test]
async fn setting_one_field_carries_only_that_field() {
    let mutation = PdfPresenceMutation::SetPresence(SetPresence { pointer_x: 101, ..set_presence(&peer()) });
    assert_eq!(*mutation.diff(&peer()).diff(), PdfPresenceDiff { pointer_x: Some(101), ..Default::default() });
    assert_mutation_inverse_sum_law(&mutation, &peer()).await;
}
