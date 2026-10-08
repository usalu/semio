//! 🧪️ Neutral Jack graph-window configuration mutation and routing laws.

use super::*;
use protocol::{Mutation, MutationDiff, OpBinary, OpText};
use semio_framework_plugin::WindowConfigOwner;

#[test]
fn jack_graph_window_config_mutations_match_the_independent_patch_trace() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/🔬️window/🔣️.json")).unwrap();
    let base: JackGraphWindowConfig = semio_framework_pack_json::from_json_str(&fixture["base"].to_string(), semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();
    let mut windows = std::collections::BTreeMap::from([(fixture["leftWindowId"].as_str().unwrap().to_string(), base.clone()), (fixture["rightWindowId"].as_str().unwrap().to_string(), base)]);
    for row in fixture["cases"].as_array().unwrap() {
        let id = row["windowId"].as_str().unwrap();
        let mutation: JackGraphWindowConfigMutation = semio_framework_pack_json::from_json_str(&row["mutation"].to_string(), semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();
        let before = windows[id].clone();
        let after = protocol::apply_diff(mutation.diff(&before).diff(), &before).unwrap();
        let restored = mutation.inverse(&before).expect("valid retained mutation inverse fixture").into_iter().fold(after.clone(), |state, inverse| protocol::apply_diff(inverse.diff(&state).diff(), &state).unwrap());
        assert_eq!(restored, before);
        assert_eq!(JackGraphWindowConfigMutation::parse_op(&mutation.print_op()).unwrap(), mutation);
        assert_eq!(JackGraphWindowConfigMutation::decode_op(&mutation.encode_op().unwrap()).unwrap(), mutation);
        windows.insert(id.into(), after);
        for (id, state) in &windows {
            assert_eq!(serde_json::from_str::<serde_json::Value>(&semio_framework_pack_json::to_json_string(state)).unwrap(), row["expected"][id]);
        }
    }
}

#[test]
fn jack_graph_window_config_command_uses_the_trusted_concrete_window() {
    use semio_framework_plugin::{ViewModel, ViewWindowInstance};
    let view = ViewModel {
        window_id: Some("graph-right".into()),
        window_instances: vec![
            ViewWindowInstance { id: "graph-left".into(), window_kind_id: JackGraphWindowConfigOwner::WINDOW_KIND_ID.into() },
            ViewWindowInstance { id: "graph-right".into(), window_kind_id: JackGraphWindowConfigOwner::WINDOW_KIND_ID.into() },
        ],
        ..ViewModel::new(semio_framework_ui_locale::Locale::En, semio_framework_ui_locale::Terminology::Native)
    };
    let emit = crate::editor::jack::commands::set_lod_mode("compact", Some(&view)).unwrap();
    assert!(emit.artifact_mutations.is_empty());
    assert!(emit.config_mutations.is_empty());
    assert_eq!(emit.window_config_mutations.len(), 1);
    assert_eq!(emit.window_config_mutations[0].window_id(), "graph-right");
    assert_eq!(emit.window_config_mutations[0].window_kind_id(), JackGraphWindowConfigOwner::WINDOW_KIND_ID);
    assert!(crate::editor::jack::commands::set_lod_mode("compact", None).is_err());
}

/// ➕️ Every committed window-config mutation's concrete inverse rows sum to the negative of its diff (law L3).
#[semio_framework_async_macros::async_test]
async fn jack_graph_window_config_inverses_sum_to_the_negative_diff() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/🔬️window/🔣️.json")).unwrap();
    let base: JackGraphWindowConfig = semio_framework_pack_json::from_json_str(&fixture["base"].to_string(), semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();
    let mut windows = std::collections::BTreeMap::from([(fixture["leftWindowId"].as_str().unwrap().to_string(), base.clone()), (fixture["rightWindowId"].as_str().unwrap().to_string(), base)]);
    for row in fixture["cases"].as_array().unwrap() {
        let id = row["windowId"].as_str().unwrap();
        let mutation: JackGraphWindowConfigMutation = semio_framework_pack_json::from_json_str(&row["mutation"].to_string(), semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();
        let before = windows[id].clone();
        protocol::os_spr::protocol_laws::assert_mutation_inverse_sum_law(&mutation, &before).await;
        windows.insert(id.into(), protocol::apply_diff(mutation.diff(&before).diff(), &before).unwrap());
    }
}
