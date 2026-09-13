mod value_round_trip_tests {
    use super::*;

    #[semio_framework_async_macros::async_test]
    async fn action_descriptor_round_trips() {
        let values = [ActionDescriptor { controller_id: "ctrl".into(), action: "doThing".into(), args: Some(DslValue::float(42.0)) }, ActionDescriptor { controller_id: "ctrl".into(), action: "doOther".into(), args: None }];
        for value in values {
            assert_eq!(ActionDescriptor::from_value(value.to_value()).expect("valid DslValue decodes"), value);
        }
    }

    #[semio_framework_async_macros::async_test]
    async fn window_layout_round_trips() {
        let layout = WindowLayout {
            root: WindowLayoutRoot::Axis(WindowLayoutAxisNode {
                kind: "horizontal".into(),
                size: None,
                children: vec![
                    WindowLayoutChild::Stack(WindowLayoutStackNode {
                        kind: "stack".into(),
                        size: Some(0.5),
                        active_window_kind_id: Some("main".into()),
                        children: vec![WindowLayoutWindowNode { kind: "window".into(), window_kind_id: "main".into(), title: Some("Main".into()), instance_id: None, template_id: None, corner: Some(WindowStackCorner::TopRight) }],
                    }),
                    WindowLayoutChild::Axis(WindowLayoutAxisNode { kind: "vertical".into(), size: Some(0.5), children: vec![] }),
                ],
            }),
        };
        assert_eq!(WindowLayout::from_value(layout.to_value()).expect("valid DslValue decodes"), layout);
    }

    #[semio_framework_async_macros::async_test]
    async fn named_layout_round_trips() {
        let layout = WindowLayout { root: WindowLayoutRoot::Stack(WindowLayoutStackNode { kind: "stack".into(), size: None, active_window_kind_id: None, children: vec![] }) };
        let named = NamedLayout { id: "layout-1".into(), label: "Default".into(), icon_id: Some(IconName::AppWindow), layout, origin: "user".into(), group_path: Some(vec!["group".into()]) };
        assert_eq!(NamedLayout::from_value(named.to_value()).expect("valid DslValue decodes"), named);
    }

    #[semio_framework_async_macros::async_test]
    async fn window_options_round_trips() {
        let options = WindowOptions {
            measures: vec![WindowMeasure::Toggle {
                id: "m1".into(),
                icon_id: IconName::CircleDot,
                label: Some("Toggle".into()),
                pressed: true,
                text: None,
                on_change: ActionDescriptor { controller_id: "ctrl".into(), action: "toggle".into(), args: None },
            }],
            engagement: WindowEngagementSlot::Some(default_viewport_engagement()),
        };
        assert_eq!(WindowOptions::from_value(options.to_value()).expect("valid DslValue decodes"), options);
    }

    #[semio_framework_async_macros::async_test]
    async fn window_measure_number_and_progress_round_trip() {
        let measures = [
            WindowMeasure::Number {
                id: "count".into(),
                label: Some("Count".into()),
                value: 100.0,
                min: Some(0.0),
                max: None,
                step: Some(1.0),
                ready: Some(42.0),
                loading: Some(true),
                waiting: None,
                disabled: Some(false),
                on_change: ActionDescriptor { controller_id: "ctrl".into(), action: "setFillCount".into(), args: None },
            },
            WindowMeasure::Progress {
                id: "fill".into(),
                label: None,
                stage: Some("locking".into()),
                completed: 12.0,
                total: Some(100.0),
                steps: vec![MeasureProgressStep { kind: MeasureProgressStepKind::Warning, text: "no-open-vortex".into() }],
                cancel: Some(ActionDescriptor { controller_id: "ctrl".into(), action: "cancelFill".into(), args: None }),
                loading: None,
            },
        ];
        for measure in measures {
            assert_eq!(WindowMeasure::from_value(measure.to_value()).expect("valid DslValue decodes"), measure);
        }
    }

    #[semio_framework_async_macros::async_test]
    async fn measure_progress_step_round_trips() {
        for kind in [MeasureProgressStepKind::Info, MeasureProgressStepKind::Success, MeasureProgressStepKind::Warning, MeasureProgressStepKind::Danger] {
            let step = MeasureProgressStep { kind, text: format!("{} step", kind.as_str()) };
            assert_eq!(MeasureProgressStep::from_value(step.to_value()).expect("valid DslValue decodes"), step);
            assert_eq!(MeasureProgressStepKind::from_value(kind.to_value()).expect("valid DslValue decodes"), kind);
        }
    }

    #[semio_framework_async_macros::async_test]
    async fn surface_kind_round_trips() {
        // 🔀️ `SurfaceKind` lives in the sibling `pub mod ui`, not here in `layout` —
        // imported explicitly rather than via `use super::*` to avoid pulling that whole
        // module's re-exports into this test scope.
        use crate::wgpu::SurfaceKind;
        for kind in [SurfaceKind::Canvas2d, SurfaceKind::World3d, SurfaceKind::VirtualFileSystem, SurfaceKind::EventFeed] {
            assert_eq!(SurfaceKind::from_value(kind.to_value()).expect("valid DslValue decodes"), kind);
        }
    }

    // 🌱️ Ticket 26/09/01/RUNTIME-DEPENDENCY-ELIMINATION-FOR-S-PLUGINS-AND-ARTIFACTS: round-trip
    // coverage for the additive-phase types this pass gave `ToValue`/`FromValue` to.
    #[semio_framework_async_macros::async_test]
    async fn style_spec_round_trips() {
        let values = [StyleSpec { variant: Some("primary".into()), size: None, density: Some("compact".into()) }, StyleSpec { variant: None, size: None, density: None }];
        for value in values {
            assert_eq!(StyleSpec::from_value(value.to_value()).expect("valid DslValue decodes"), value);
        }
    }

    #[semio_framework_async_macros::async_test]
    async fn ui_presence_round_trips() {
        let values = [
            UiPresence::default(),
            UiPresence {
                state: UiState::Celebrating,
                status: UiStatus::Loading,
                hover: true,
                selected: true,
                color: Some(3),
                peers: vec![UiPeerMark { actor: "peer-1".into(), color: Some(2), hovered: true, selected: false, label: "Peer One".into() }],
            },
        ];
        for value in values {
            assert_eq!(UiPresence::from_value(value.to_value()).expect("valid DslValue decodes"), value);
        }
    }
}
