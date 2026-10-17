use crate::standards::v1::subsets::any::schema::model::LayoutPoint;
use crate::standards::v1::subsets::any::schema::mutations::set_rule_layout_points::RuleLayoutPlacement;
use crate::standards::v1::subsets::any::schema::rule::*;
use crate::standards::v1::subsets::any::schema::snapshot::*;
use semio_framework_value::retained_clone::{RetainedCloneProgress, RetainedCloneStep};
use semio_framework_value::retirement::{admit_owned_retirement, owned_retirement_birth_bytes};
use semio_s_artifact_trinity_jack::standards::v1::subsets::any::schema::content::{jack_self_funded_grant, JACK_SELF_FUNDED_BODY_BYTES};

fn admit<T: semio_framework_value::RetireOwned>(value: T) -> Box<dyn semio_framework_value::ErasedSnapshotRetirement> {
    let birth = jack_self_funded_grant(semio_framework_value::RetirementDemand { capacity_bytes: owned_retirement_birth_bytes::<T>(), ..Default::default() });
    admit_owned_retirement(value, birth).unwrap_or_else(|(error, _)| panic!("rewriting retirement admission: {error}")).0
}

fn drain(mut owner: Box<dyn semio_framework_value::ErasedSnapshotRetirement>, items: usize) -> usize {
    let mut released = 0;
    for _ in 0..10_000 {
        if owner.terminal_is_empty() {
            return released;
        }
        let demand = owner.next_demand(JACK_SELF_FUNDED_BODY_BYTES).unwrap();
        let grant = semio_framework_value::retained_clone::RetainedCloneGrant { maximum_items: items, ..jack_self_funded_grant(demand) };
        let step = owner.close_step(grant).unwrap();
        let progress: RetainedCloneProgress = step.progress();
        assert!(progress.fits(grant) && progress.copied_items <= items);
        released += progress.released_bytes;
        if matches!(step, RetainedCloneStep::Complete(_)) {
            assert!(owner.terminal_is_empty());
            return released;
        }
    }
    panic!("document retirement did not reach terminal emptiness");
}

#[test]
fn rewriting_window_config_document_retirement_respects_exact_grants() {
    let fixture: serde_json::Value = serde_json::from_str(runtime_fixture_text("../../../../🧪️tests/🧫️fixtures/♻️retirement/🔣️.json")).unwrap();
    for budget in fixture["budgets"].as_array().unwrap() {
        let items = budget["items"].as_u64().unwrap() as usize;
        for row in fixture["snapshots"].as_array().unwrap() {
            let value = crate::standards::v1::subsets::any::io::text::snapshot::decode_rewriting_snapshot_json(&row["value"].to_string()).unwrap();
            let root = std::sync::Arc::new(value.clone());
            let mut shared = admit(std::sync::Arc::clone(&root));
            let demand = shared.next_demand(JACK_SELF_FUNDED_BODY_BYTES).unwrap();
            assert!(matches!(shared.close_step(jack_self_funded_grant(demand)).unwrap(), RetainedCloneStep::Progress(progress) if progress == RetainedCloneProgress::default()), "a live alias must block the last-alias retirement");
            drop(root);
            let expected = drain(shared, items);
            assert_eq!(drain(admit(value), items), expected);
        }
        for row in fixture["mutations"].as_array().unwrap() {
            let value = crate::standards::v1::subsets::any::io::text::mutations::decode_rewriting_mutation_json(&row["value"].to_string()).unwrap();
            assert!(drain(admit(value), items) > 0);
        }
    }
}

fn runtime_fixture_text(relative: &str) -> &'static str {
    let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join(relative);
    Box::leak(std::fs::read_to_string(&path).unwrap_or_else(|error| panic!("fixture {}: {error}", path.display())).into_boxed_str())
}
