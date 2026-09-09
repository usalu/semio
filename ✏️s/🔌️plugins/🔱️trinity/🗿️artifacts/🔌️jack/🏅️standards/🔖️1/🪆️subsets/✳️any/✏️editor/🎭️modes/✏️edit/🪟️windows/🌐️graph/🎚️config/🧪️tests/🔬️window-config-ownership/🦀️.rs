//! 🧪️ Neutral Jack graph-window configuration mutation and routing laws.

use super::*;
use protocol::{Mutation, MutationDiff, OpBinary, OpText};
use semio_framework_plugin::WindowConfigOwner;

#[test]
fn jack_graph_window_config_mutations_match_the_independent_patch_trace() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("🔣️.json")).unwrap();
    let base: JackGraphWindowConfig = pack::from_json_str(&fixture["base"].to_string()).unwrap();
    let mut windows = std::collections::BTreeMap::from([(fixture["leftWindowId"].as_str().unwrap().to_string(), base.clone()), (fixture["rightWindowId"].as_str().unwrap().to_string(), base)]);
    for row in fixture["cases"].as_array().unwrap() {
        let id = row["windowId"].as_str().unwrap();
        let mutation: JackGraphWindowConfigMutation = pack::from_json_str(&row["mutation"].to_string()).unwrap();
        let before = windows[id].clone();
        let after = mutation.diff(&before).diff().apply(&before).unwrap();
        let restored = mutation.inverse(&before).into_iter().fold(after.clone(), |state, inverse| inverse.diff(&state).diff().apply(&state).unwrap());
        assert_eq!(restored, before);
        assert_eq!(JackGraphWindowConfigMutation::parse_op(&mutation.print_op()).unwrap(), mutation);
        assert_eq!(JackGraphWindowConfigMutation::decode_op(&mutation.encode_op().unwrap()).unwrap(), mutation);
        windows.insert(id.into(), after);
        for (id, state) in &windows {
            assert_eq!(serde_json::from_str::<serde_json::Value>(&pack::to_json_string(state)).unwrap(), row["expected"][id]);
        }
    }
    eprintln!("[DEBUG] Jack graph-window config mutations matched the independent camera/LOD patch trace and restored exact inverses");
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
        ..Default::default()
    };
    let emit = crate::editor::jack::commands::set_lod_mode("compact", Some(&view)).unwrap();
    assert!(emit.artifact_mutations.is_empty());
    assert!(emit.config_mutations.is_empty());
    assert_eq!(emit.window_config_mutations.len(), 1);
    assert_eq!(emit.window_config_mutations[0].window_id(), "graph-right");
    assert_eq!(emit.window_config_mutations[0].window_kind_id(), JackGraphWindowConfigOwner::WINDOW_KIND_ID);
    assert!(crate::editor::jack::commands::set_lod_mode("compact", None).is_err());
    eprintln!("[DEBUG] Jack LOD command addressed only the host-selected concrete graph window");
}
