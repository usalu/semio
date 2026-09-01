//! 🔗️ Unmounted diagnostic of independent updates, not a single-operation acceptance law.

use super::RuntimePresentationAuthority;

#[test]
fn metrics_observer_independent_updates_are_not_a_composite_transaction() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("🔣️.json")).unwrap();
    let count = |field: &str, key: &str| fixture["independentUpdates"][field][key].as_str().unwrap().parse::<u64>().unwrap();
    let authority = RuntimePresentationAuthority::new();
    authority.observe_input_generation(count("old", "inputGeneration"));
    let old = authority.current();
    let half = std::thread::scope(|scope| {
        let (published, publication) = std::sync::mpsc::sync_channel(0);
        let (resume, resumed) = std::sync::mpsc::sync_channel(0);
        let authority = &authority;
        let next_input = count("committed", "inputGeneration");
        let writer = scope.spawn(move || {
            authority.mark_scene_changed();
            published.send(()).unwrap();
            resumed.recv().unwrap();
            authority.observe_input_generation(next_input);
        });
        publication.recv().unwrap();
        let observed = authority.current();
        resume.send(()).unwrap();
        writer.join().unwrap();
        observed
    });
    let committed = authority.current();
    assert_eq!(old.scene_revision, count("old", "sceneRevision"));
    assert_eq!(old.input_generation, count("old", "inputGeneration"));
    assert_eq!(committed.scene_revision, count("committed", "sceneRevision"));
    assert_eq!(committed.input_generation, count("committed", "inputGeneration"));
    let half_accepted = half != old && half != committed;
    eprintln!("[DEBUG] metrics observer old={old:?} between={half:?} committed={committed:?} halfAccepted={half_accepted}");
    assert_eq!(half_accepted, fixture["independentUpdates"]["halfIsLegitimate"].as_bool().unwrap());
}
