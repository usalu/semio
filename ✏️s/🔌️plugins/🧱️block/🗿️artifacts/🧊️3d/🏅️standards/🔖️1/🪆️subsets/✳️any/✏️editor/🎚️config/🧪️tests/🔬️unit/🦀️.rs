use super::*;

#[semio_framework_async_macros::async_test]
async fn block3d_config_default_has_all_tags() {
    let config = Block3dConfig::default();
    assert!(config.active_representation_id.is_none());
    assert!(config.wanted_tags.is_empty());
    assert!(config.windows.is_empty());
    assert_eq!(config.brush_radius, 0.3);
}

/// 🕹️ ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM: selection/hover moved off this
/// config onto the framework's `vortex` interaction domain — this now exercises a genuinely
/// remaining config mutation (`SetActiveRepresentation`) for the backwards-restores-snapshot contract.
#[semio_framework_async_macros::async_test]
async fn config_operation_backwards_restores_the_pre_operation_snapshot() {
    let base = Block3dConfig::default();
    let operation = Block3dConfigMutation::SetActiveRepresentation { representation_id: Some("r0".into()) };
    let next = operation.diff(&base).into_parts().0;
    assert_eq!(next.active_representation_id, Some("r0".to_string()));
    let inverse = operation.inverse(&base);
    assert_eq!(inverse, vec![Block3dConfigMutation::Snapshot { config: base.clone() }]);
    let restored = inverse[0].diff(&next).into_parts().0;
    assert_eq!(restored, base);
}
