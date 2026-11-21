//! 🧪️ A framework-reserved commit explicitly retires a last-owned view with ordered maps.
use super::ReservedCommitMetadata;
use semio_framework_job::{Generation, OperationId, OriginalJobDecodeOwner, OriginalJobFixtureCaller};
use semio_framework_value::RetainedCloneGrant;
use semio_plugin_decl_emission::ActionMeta;
use std::sync::Arc;

#[test]
fn reserved_metadata_retires_last_view_owner_after_ingress() {
    let law: serde_json::Value = serde_json::from_str(include_str!("metadata-retirement.json")).unwrap();
    let window = law["window"].as_str().unwrap();
    let utility = law["utility"].as_str().unwrap();
    let mut view = semio_framework::ViewModel::new(semio_ui_locale::Locale::En, semio_ui_locale::Terminology::Native);
    view.window_id = Some(window.into());
    view.active_utility_by_window_id.insert(window.into(), Some(utility.into()));
    let ingress = Arc::new(view);
    let mut commit = ReservedCommitMetadata::new(ActionMeta {
        view_state: Some(ingress.clone()),
        ..Default::default()
    });
    assert_eq!(Arc::strong_count(&ingress), law["expectedOwners"][0].as_u64().unwrap() as usize);
    drop(ingress);
    let view = commit.get().view_state.as_ref().unwrap();
    assert_eq!(Arc::strong_count(view), law["expectedOwners"][1].as_u64().unwrap() as usize);
    assert_eq!(view.active_utility_by_window_id.get(window).unwrap().as_deref(), Some(utility));
    let grant = RetainedCloneGrant {
        maximum_items: 64,
        maximum_copy_bytes: 32768,
        maximum_capacity_bytes: 16 << 20,
        maximum_release_bytes: 16 << 20,
        maximum_depth: 4096,
    };
    let mut caller = OriginalJobFixtureCaller::admit(
        OperationId(49233),
        Generation(4),
        RetainedCloneGrant { maximum_items: 1 << 20, maximum_copy_bytes: 1 << 30, ..grant },
        semio_framework_job::default_now_us,
        u64::MAX,
        1 << 20,
    );
    let mut decoder = OriginalJobDecodeOwner::empty();
    for _ in 0..10000 {
        if commit.terminal_is_empty() {
            break;
        }
        caller.turn_with_decoder(grant, &mut decoder, |cx| commit.close_step(cx)).unwrap();
    }
    assert_eq!(usize::from(!commit.terminal_is_empty()), law["expectedOwners"][2].as_u64().unwrap() as usize);
    while !decoder.terminal_is_empty() {
        caller.turn(grant, |cx| cx.with_original_retirement(|cx| decoder.close_on_source(cx).map(|_| ()))).unwrap();
    }
    caller.retire(grant);
}
