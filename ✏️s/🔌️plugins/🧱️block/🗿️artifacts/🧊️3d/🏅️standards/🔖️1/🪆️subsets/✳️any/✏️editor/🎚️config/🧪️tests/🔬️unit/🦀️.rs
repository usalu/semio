use super::*;

#[semio_framework_async_macros::async_test]
async fn block3d_config_default_has_all_tags() {
    let config = Block3dConfig::default();
    assert!(config.active_representation_id.is_none());
    assert!(config.wanted_tags.is_empty());
    assert!(config.windows.is_empty());
    assert_eq!(config.brush_radius, 0.3);
}

fn applied(mutation: &Block3dConfigMutation, base: &Block3dConfig) -> Block3dConfig {
    protocol::apply_diff(mutation.diff(base).diff(), base).expect("valid config diff")
}

fn restored(mutation: &Block3dConfigMutation, base: &Block3dConfig) -> Block3dConfig {
    let mut state = applied(mutation, base);
    let mut backward = mutation.inverse(base).expect("valid retained mutation inverse fixture");
    backward.reverse();
    for undo in &backward {
        state = applied(undo, &state);
    }
    state
}

#[semio_framework_async_macros::async_test]
async fn config_operation_inverse_restores_only_the_changed_field() {
    let base = Block3dConfig::default();
    let operation = Block3dConfigMutation::SetActiveRepresentation { representation_id: Some("r0".into()) };
    let next = applied(&operation, &base);
    assert_eq!(next.active_representation_id, Some("r0".to_string()));
    let inverse = operation.inverse(&base).expect("valid retained mutation inverse fixture");
    assert_eq!(inverse, vec![Block3dConfigMutation::SetActiveRepresentation { representation_id: None }]);
    assert_eq!(applied(&inverse[0], &next), base);
}

#[semio_framework_async_macros::async_test]
async fn window_edits_restore_the_exact_row_list_including_middle_rows() {
    let mut base = Block3dConfig::default();
    for window_id in ["w0", "w1", "w2"] {
        base = applied(&Block3dConfigMutation::SetWindowSpacing { window_id: window_id.into(), spacing: 3.0 }, &base);
    }
    assert_eq!(base.windows.iter().map(|row| row.window_id.as_str()).collect::<Vec<_>>(), ["w0", "w1", "w2"]);
    let to_default = Block3dConfigMutation::SetWindowSpacing { window_id: "w1".into(), spacing: Block3dWindowView::for_window("w1").spacing };
    assert_eq!(applied(&to_default, &base).windows.iter().map(|row| row.window_id.as_str()).collect::<Vec<_>>(), ["w0", "w2"]);
    assert_eq!(restored(&to_default, &base), base);
    let created = Block3dConfigMutation::SetWindowArrangement { window_id: "w1b".into(), arrangement: "grid".into() };
    assert_eq!(applied(&created, &base).windows.iter().map(|row| row.window_id.as_str()).collect::<Vec<_>>(), ["w0", "w1", "w1b", "w2"]);
    assert_eq!(restored(&created, &base), base);
    let toggled = Block3dConfigMutation::ToggleWindowRepresentation { window_id: "w1".into(), representation_id: "r0".into(), visible: true };
    assert_eq!(restored(&toggled, &base), base);
}
