//! 🧪️ The whole-record replace edit is admitted by its declared footprint, reaches Prepared through the real canonical sealer and closes under exact quoted grants.
use super::{WindowConfigReplaceEdit,WindowConfigReplaceMutation};
use super::retained_pack_load_tests::{RetainedLoadCameraConfig as State,RetainedLoadCameraConfigMutation as Mutation};
use crate::store;
use semio_framework_value::retirement::{OwnedValueRetirementFactory,SharedValueRetirementFactory};
use std::sync::Arc;
use store::snapshot_clone_preparation::{drive_one_item_preparation_law,RetainedCloneEdit,CONFIG_EDIT_RETAINED_BASELINE_BYTES};

impl WindowConfigReplaceMutation<State> for Mutation {
    fn replacement(&self) -> &State { let Self::Snapshot { config } = self; config }
    fn restoring(previous: Box<State>) -> Self { Self::Snapshot { config: previous } }
}

#[test]
fn window_config_replace_reaches_prepared_through_the_real_sealer_and_closes_under_quotes() {
    let policy: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/🔣️.json")).unwrap();
    let parse = |value: &serde_json::Value| -> State { semio_framework_pack_json::from_json_str(&value.to_string(), semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap() };
    let (base, next) = (parse(&policy["camera"]["base"]), parse(&policy["camera"]["next"]));
    let edit = WindowConfigReplaceEdit::<State, Mutation>::new();
    let mutation = Mutation::Snapshot { config: Box::new(next) };
    let footprint = edit.preflight(&mutation, store::HistoryLane::Document).unwrap();
    assert_eq!(footprint.retained_bytes, CONFIG_EDIT_RETAINED_BASELINE_BYTES);
    let factory: Arc<dyn store::ArtifactStoreOneItemPreparationFactory<State, Mutation>> = Arc::new(store::snapshot_clone_preparation::RetainedClonePreparationFactory::new(Arc::new(edit), Arc::new(OwnedValueRetirementFactory::<Mutation>::default()), Arc::new(SharedValueRetirementFactory::<State>::default()), 64).unwrap());
    let grant = store::ArtifactStoreOneItemGrant { maximum_items: 1, maximum_copy_bytes: 1 << 16, maximum_capacity_bytes: 1 << 26, maximum_release_bytes: 1 << 26, maximum_depth: 4096 };
    let law = drive_one_item_preparation_law(&factory, base, mutation, grant);
    assert_eq!(*law.post, next);
    assert_eq!(law.inverse, vec![Mutation::Snapshot { config: Box::new(base) }]);
    assert!(law.peak_live_capacity_bytes <= footprint.retained_bytes, "sealed replace peaks at {} but declares {}", law.peak_live_capacity_bytes, footprint.retained_bytes);
    eprintln!("[DEBUG] window config replace reached Prepared in {} turns, peak live {} of declared {}", law.turns, law.peak_live_capacity_bytes, footprint.retained_bytes);
}
