//! 🧪️ Laws of the wgpu hub first-run twin. The same vectors (`📇️directory/🎓️first-run/🧪️tests/🧫️fixtures/🔣️.json`)
//! and the same contract (`📇️directory/🎓️first-run/🔣️.json`) are read by the React twin's vitest suite,
//! so a drift on either side turns exactly one renderer red.

use super::*;
use semio_ui_locale::Terminology;
use std::collections::HashMap;

const VECTORS: &str = include_str!("../../../../../../../../../🔨️modules/📇️directory/🎓️first-run/🧪️tests/🧫️fixtures/🔣️.json");
const CONTRACT: &str = include_str!("../../../../../../../../../🔨️modules/📇️directory/🎓️first-run/🔣️.json");

fn vectors() -> serde_json::Value {
    serde_json::from_str(VECTORS).expect("the shared vectors are valid JSON")
}

fn rows(value: &serde_json::Value) -> Vec<serde_json::Value> {
    value.as_array().cloned().unwrap_or_default()
}

fn state(value: &serde_json::Value) -> HubFirstRunState {
    HubFirstRunState {
        signed_in: value["signedIn"].as_bool().unwrap(),
        space_count: value["spaceCount"].as_u64().unwrap() as usize,
        open_space_id: value["openSpaceId"].as_str().map(str::to_string),
        can_invite: value["canInvite"].as_bool().unwrap(),
    }
}

#[derive(Default)]
struct MemoryStorage(HashMap<String, String>);

impl HubFirstRunStorage for MemoryStorage {
    fn get_item(&self, key: &str) -> Option<String> {
        self.0.get(key).cloned()
    }

    fn set_item(&mut self, key: &str, value: &str) {
        self.0.insert(key.to_string(), value.to_string());
    }
}

fn stored(value: &serde_json::Value) -> MemoryStorage {
    let mut storage = MemoryStorage::default();
    if let Some(value) = value.as_str() {
        storage.set_item(&hub_first_run_storage_key(), value);
    }
    storage
}

#[test]
fn the_stage_order_is_the_contracts_and_the_vectors() {
    let contract: serde_json::Value = serde_json::from_str(CONTRACT).unwrap();
    let names: Vec<&str> = HubFirstRunStage::ALL.iter().map(|stage| stage.as_str()).collect();
    assert_eq!(serde_json::json!(names), contract["stages"]);
    assert_eq!(serde_json::json!(names), vectors()["stages"]);
    for (index, stage) in HubFirstRunStage::ALL.iter().enumerate() {
        assert_eq!(stage.index(), index);
    }
}

#[test]
fn every_state_vector_derives_its_stage_and_step_index() {
    for row in rows(&vectors()["stateVectors"]) {
        let id = row["id"].as_str().unwrap();
        let state = state(&row["state"]);
        assert_eq!(hub_first_run_stage(&state).as_str(), row["stage"].as_str().unwrap(), "{id}");
        assert_eq!(hub_first_run_step_index(&state), row["stepIndex"].as_u64().unwrap() as usize, "{id}");
    }
}

#[test]
fn every_projection_vector_projects_phase_rows_and_active_space() {
    for row in rows(&vectors()["projectionVectors"]) {
        let source = rows(&row["rows"]);
        let pairs: Vec<(&str, &str)> = source.iter().map(|entry| (entry["id"].as_str().unwrap(), entry["access"].as_str().unwrap())).collect();
        let projected = hub_first_run_state_from(row["sessionPhase"].as_str().unwrap(), &pairs, row["activeSpaceId"].as_str());
        assert_eq!(projected, state(&row["state"]), "{}", row["id"]);
    }
}

#[test]
fn every_seen_vector_reads_the_flag_the_same_way() {
    for row in rows(&vectors()["seenVectors"]) {
        let storage = stored(&row["stored"]);
        let door: Option<&dyn HubFirstRunStorage> = row["storeAvailable"].as_bool().unwrap().then_some(&storage);
        assert_eq!(hub_first_run_seen(door), row["seen"].as_bool().unwrap(), "{}", row["id"]);
    }
}

#[test]
fn auto_open_follows_the_lifecycle_vectors() {
    for row in rows(&vectors()["lifecycleVectors"]) {
        let storage = stored(&row["stored"]);
        let step = hub_first_run_auto_open_step(Some(&storage), row["surfaceOpen"].as_bool().unwrap(), &state(&row["state"]));
        assert_eq!(step, row["autoOpenStep"].as_u64().map(|step| step as usize), "{}", row["id"]);
    }
}

#[test]
fn auto_open_never_starts_without_a_hub_surface_or_after_a_dismissal() {
    let signed_out = HubFirstRunState { signed_in: false, space_count: 0, open_space_id: None, can_invite: false };
    let mut storage = MemoryStorage::default();
    assert_eq!(hub_first_run_auto_open_step(Some(&storage), false, &signed_out), None);
    assert_eq!(hub_first_run_auto_open_step(Some(&storage), true, &signed_out), Some(1));
    mark_hub_first_run_seen(Some(&mut storage));
    assert_eq!(hub_first_run_auto_open_step(Some(&storage), true, &signed_out), None);
}

#[test]
fn dismiss_marks_seen_under_the_shared_introduction_key() {
    let vectors = vectors();
    let mut storage = MemoryStorage::default();
    assert!(!hub_first_run_seen(Some(&storage)));
    mark_hub_first_run_seen(Some(&mut storage));
    assert_eq!(storage.get_item(vectors["dismiss"]["storageKey"].as_str().unwrap()).as_deref(), vectors["dismiss"]["storedValue"].as_str());
    assert!(hub_first_run_seen(Some(&storage)));
    assert_eq!(hub_first_run_storage_key(), format!("ui.introduction.seen.{}", hub_first_run_introduction_id()));
    mark_hub_first_run_seen(None);
}

#[test]
fn replay_starts_at_step_zero_even_for_someone_deep_in_the_flow() {
    assert_eq!(hub_first_run_replay_step(), vectors()["replayStep"].as_u64().unwrap() as usize);
    let in_space = HubFirstRunState { signed_in: true, space_count: 1, open_space_id: Some("spc_1".into()), can_invite: true };
    assert_ne!(hub_first_run_step_index(&in_space), hub_first_run_replay_step());
}

#[test]
fn every_completion_vector_answers_the_shared_completion_rule() {
    for row in rows(&vectors()["completionVectors"]) {
        let stage = *HubFirstRunStage::ALL.iter().find(|stage| stage.as_str() == row["stage"].as_str().unwrap()).unwrap();
        assert_eq!(hub_first_run_stage_done(stage, &state(&row["state"]), row["inviteReady"].as_bool().unwrap()), row["done"].as_bool().unwrap(), "{}", row["id"]);
    }
}

#[test]
fn a_step_completes_by_doing_what_it_teaches() {
    let signed_out = HubFirstRunState { signed_in: false, space_count: 0, open_space_id: None, can_invite: false };
    let no_space = HubFirstRunState { signed_in: true, space_count: 0, open_space_id: None, can_invite: false };
    let in_space = HubFirstRunState { signed_in: true, space_count: 1, open_space_id: Some("spc_1".into()), can_invite: true };
    assert!(!hub_first_run_stage_done(HubFirstRunStage::SignIn, &signed_out, false));
    assert!(hub_first_run_stage_done(HubFirstRunStage::SignIn, &no_space, false));
    assert!(!hub_first_run_stage_done(HubFirstRunStage::Space, &no_space, false));
    assert!(hub_first_run_stage_done(HubFirstRunStage::Space, &in_space, false));
    assert!(!hub_first_run_stage_done(HubFirstRunStage::Invite, &in_space, false));
    assert!(hub_first_run_stage_done(HubFirstRunStage::Invite, &in_space, true));
    assert!(!hub_first_run_stage_done(HubFirstRunStage::Welcome, &in_space, true) && !hub_first_run_stage_done(HubFirstRunStage::Done, &in_space, true));
}

#[test]
fn the_definition_has_one_step_per_stage_with_the_contracts_text_in_both_locales() {
    let introduction = hub_first_run_introduction();
    let ids: Vec<String> = introduction.steps.iter().map(|step| step.id.clone()).collect();
    assert_eq!(ids, HubFirstRunStage::ALL.map(|stage| format!("hub.firstRun.{}", stage.as_str())).to_vec());
    for locale in [Locale::En, Locale::De] {
        for terminology in Terminology::ALL {
            assert_eq!(introduction.title.resolve(terminology, locale), hub_first_run_text(locale, "title"));
            assert_eq!(introduction.steps[1].title.resolve(terminology, locale), hub_first_run_text(locale, "signInTitle"));
            assert_eq!(introduction.steps[4].body.resolve(terminology, locale), hub_first_run_text(locale, "doneBody"));
        }
    }
    assert_ne!(hub_first_run_text(Locale::En, "signInBody"), hub_first_run_text(Locale::De, "signInBody"));
}

#[test]
fn screen_steps_are_centered_and_task_steps_anchor_with_unordered_checklists() {
    let steps = &hub_first_run_introduction().steps;
    for index in [0, 4] {
        assert_eq!(steps[index].introduce, None);
        assert_eq!(steps[index].placement, IntroductionPlacement::Center);
        assert!(steps[index].show.is_empty() && steps[index].interactions.is_empty());
    }
    for (index, interactions) in [(1, 1), (2, 2), (3, 1)] {
        assert!(steps[index].introduce.as_deref().is_some_and(|id| id.starts_with("framework.hub.")));
        assert_eq!(steps[index].interactions.len(), interactions);
        assert!(!steps[index].ordered);
    }
}

#[test]
fn the_two_workspace_controls_keep_the_react_ids_icons_variants_labels_and_order() {
    let [replay, close] = hub_first_run_controls();
    assert_eq!((replay.id(), replay.icon(), replay.variant()), ("os.hub.firstRun.replay", "graduation-cap", "ghost"));
    assert_eq!((close.id(), close.icon(), close.variant()), ("os.hub.signIn.cancel", "x", "outline"));
    assert_eq!((replay.label(Locale::En), replay.label(Locale::De)), ("How this works", "So funktioniert das"));
    assert_eq!((close.label(Locale::En), close.label(Locale::De)), ("Cancel sign-in", "Anmeldung abbrechen"));
}
