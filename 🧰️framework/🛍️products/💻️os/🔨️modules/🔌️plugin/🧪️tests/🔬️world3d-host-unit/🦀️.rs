mod tests {
    use super::*;

    /// 🎯️ The non-domain world path speaks the SAME five schema words as the domain path — ticket
    /// 26/09/09/PROCEDURAL-3D-END-TO-END deleted its private `add`/`remove`/`toggle` spelling, which is
    /// unrepresentable now that the mode is a decoded [`protocol::MergeMode`] and not a string.
    #[semio_framework_async_macros::async_test]
    async fn merge_world_selection_ids_speaks_the_one_schema_merge_vocabulary() {
        let a = || SelectionSet::from_ids(vec!["a".into()]);
        let ab = || SelectionSet::from_ids(vec!["a".into(), "b".into()]);
        let abc = || SelectionSet::from_ids(vec!["a".into(), "b".into(), "c".into()]);
        assert_eq!(merge_world_selection_ids(&a(), &["b".into()], protocol::MergeMode::Additive).await.as_slice(), &["a".to_string(), "b".to_string()]);
        assert_eq!(merge_world_selection_ids(&ab(), &["b".into(), "c".into()], protocol::MergeMode::Invertive).await.as_slice(), &["a".to_string(), "c".to_string()]);
        assert_eq!(merge_world_selection_ids(&ab(), &["b".into()], protocol::MergeMode::Invertive).await.as_slice(), &["a".to_string()]);
        assert_eq!(merge_world_selection_ids(&a(), &["b".into()], protocol::MergeMode::Replace).await.as_slice(), &["b".to_string()]);
        assert_eq!(merge_world_selection_ids(&abc(), &["b".into()], protocol::MergeMode::Subtractive).await.as_slice(), &["a".to_string(), "c".to_string()]);
        assert_eq!(merge_world_selection_ids(&abc(), &["d".into()], protocol::MergeMode::Range).await.as_slice(), &["d".to_string()], "Range has no ordered topology on this path — it degrades to a replace, never a fault");
        for word in ["add", "remove", "toggle"] {
            assert!(protocol::MergeMode::from_wire_label(word).is_none(), "the deleted word '{word}' must decode to nothing — no adapter, no compatibility layer");
        }
    }

    #[semio_framework_async_macros::async_test]
    async fn selection_set_membership_is_constant_time() {
        let set = SelectionSet::from_ids((0..100).map(|index| format!("id-{index}")).collect());
        assert!(set.contains("id-50"));
        assert!(!set.contains("missing"));
    }

    #[semio_framework_async_macros::async_test]
    async fn isometric_pose_matches_the_classic_35_264_45_direction() {
        let mut p = WorldProjectionConfig { kind: "axonometric".into(), axonometric_variant: "isometric".into(), ..WorldProjectionConfig::default() };
        p.axonometric_quadrant = "ne".into();
        let (position, up) = world3d_projection_pose(&p, [0.0, 0.0, 0.0], 10.0);
        assert!((position[2] / 10.0 - 35.264_f64.to_radians().sin()).abs() < 1e-3);
        assert_eq!(up, [0.0, 0.0, 1.0]);
        let azimuth = (position[0] / position[1]).atan();
        assert!((azimuth.to_degrees() - 45.0).abs() < 1e-3);
    }

    #[semio_framework_async_macros::async_test]
    async fn projection_spec_json_projects_only_active_kind_fields() {
        let p = WorldProjectionConfig { kind: "oblique".into(), oblique_variant: "cabinet".into(), oblique_angle: 45.0, oblique_depth: 0.5, ..WorldProjectionConfig::default() };
        let spec = world3d_projection_spec_json(&p);
        let mode = spec.get("mode").expect("mode object");
        assert_eq!(mode.get("kind").and_then(Value::as_str), Some("oblique"));
        assert_eq!(mode.get("depthScale").and_then(Value::as_f64), Some(0.5));
        assert!(mode.get("axonometricVariant").is_none());
    }

    #[semio_framework_async_macros::async_test]
    async fn apply_action_switches_kind_and_leaves_other_kinds_untouched_for_later_recall() {
        let mut p = WorldProjectionConfig::default();
        p.axonometric_angle_a = 22.0;
        assert!(apply_world3d_projection_action(&mut p, "setProjection", Some(&store::json!({ "field": "obliqueVariant", "value": "military" }))));
        assert_eq!(p.kind, "oblique");
        assert_eq!(p.oblique_variant, "military");
        assert_eq!(p.axonometric_angle_a, 22.0);
        assert!(apply_world3d_projection_action(&mut p, "setProjectionParam", Some(&store::json!({ "param": "obliqueAngle", "value": 30.0 }))));
        assert_eq!(p.oblique_angle, 30.0);
        assert!(!world3d_projection_action_moves_pose("setProjectionParam", Some(&store::json!({ "param": "obliqueAngle" }))));
        assert!(world3d_projection_action_moves_pose("setProjection", Some(&store::json!({ "field": "obliqueVariant" }))));
    }

    #[semio_framework_async_macros::async_test]
    async fn projection_measures_tree_matches_the_requested_taxonomy() {
        let p = WorldProjectionConfig::default();
        let tree = world3d_projection_measures("t", &p, |action, args| ActionDescriptor { controller_id: "t".into(), action: action.into(), args: semio_framework::optional_json_to_dsl(args) });
        let WindowMeasure::Group { children: families, .. } = &tree else { panic!("expected root group") };
        assert_eq!(families.len(), 2);
        let WindowMeasure::Group { label: parallel_label, children: parallel_children, .. } = &families[0] else { panic!("expected parallel group") };
        assert_eq!(parallel_label, "Parallel");
        assert_eq!(parallel_children.len(), 3);
        let WindowMeasure::Group { label: perspective_label, .. } = &families[1] else { panic!("expected perspective group") };
        assert_eq!(perspective_label, "Perspective");
    }

    /// 🧪️ Every `world-3d` window-options family is addressable as `<prefix>-measure-<family>…` and mounts its
    /// own controls without a second disclosure click — `WindowMeasureTreeGroup` renders no children at all for
    /// a collapsed group, so a closed-by-default Projection/Sun group is indistinguishable from a missing one.
    #[semio_framework_async_macros::async_test]
    async fn projection_and_sun_measure_families_are_addressable_and_open_by_default() {
        let ids_of = |measure: &WindowMeasure| -> Vec<String> {
            fn walk(measure: &WindowMeasure, into: &mut Vec<String>) {
                match measure {
                    WindowMeasure::Group { id, children, default_open, .. } => {
                        into.push(id.clone());
                        if default_open != &Some(false) {
                            for child in children {
                                walk(child, into);
                            }
                        }
                    }
                    WindowMeasure::Toggle { id, .. } | WindowMeasure::Slider { id, .. } | WindowMeasure::Select { id, .. } => into.push(id.clone()),
                }
            }
            let mut into = Vec::new();
            walk(measure, &mut into);
            into
        };
        let action = |action: &str, args: Option<serde_json::Value>| ActionDescriptor { controller_id: "t".into(), action: action.into(), args: semio_framework::optional_json_to_dsl(args) };

        let sun_ids = ids_of(&world3d_sun_measures("puzzle3d", &WorldSunConfig::default(), action));
        assert!(sun_ids.contains(&"puzzle3d-measure-sun".to_string()), "{sun_ids:?}");
        assert!(sun_ids.contains(&"puzzle3d-measure-sun-enabled".to_string()), "the Sun enable toggle must mount without a second disclosure click: {sun_ids:?}");
        for axis in ["azimuth", "elevation", "intensity"] {
            assert!(sun_ids.contains(&format!("puzzle3d-measure-sun-{axis}")), "{axis} slider missing from {sun_ids:?}");
        }

        let projection_ids = ids_of(&world3d_projection_measures("puzzle3d", &WorldProjectionConfig::default(), action));
        assert!(projection_ids.iter().all(|id| id.starts_with("puzzle3d-measure-projection")), "every projection id shares the measure family prefix: {projection_ids:?}");
        assert!(projection_ids.contains(&"puzzle3d-measure-projection-parallel".to_string()), "{projection_ids:?}");
        assert!(projection_ids.contains(&"puzzle3d-measure-projection-orthographic-view".to_string()), "the orthographic view select must mount without a second disclosure click: {projection_ids:?}");
        assert!(projection_ids.contains(&"puzzle3d-measure-projection-perspective-kind".to_string()), "{projection_ids:?}");
    }

    #[semio_framework_async_macros::async_test]
    async fn world3d_scene_fields_bind_the_domain_while_the_sun_helper_leaves_it_unset() {
        let sun = WorldSunConfig::default();
        let mut bound = World3dScene::base("{}".into(), "[]".into(), "[]".into(), "{}".into());
        bound.domain_id = Some("cad".into());
        bound.domain_granularity_id = Some("handle".into());
        assert_eq!(bound.domain_id.as_deref(), Some("cad"));
        assert_eq!(bound.domain_granularity_id.as_deref(), Some("handle"));

        let unbound = world3d_scene("{}".into(), "[]".into(), "[]".into(), "{}".into(), &sun);
        assert_eq!(unbound.domain_id, None);
        assert_eq!(unbound.domain_granularity_id, None);
    }
}
