mod layout_wire_format_tests {
    use super::*;

    const GOLDEN_ACTION_DESCRIPTOR_JSON: &str = "[{\"controllerId\":\"ctrl\",\"action\":\"doThing\",\"args\":42.0},{\"controllerId\":\"ctrl\",\"action\":\"doOther\"},{\"variant\":\"primary\",\"size\":\"md\"}]";

    #[semio_framework_async_macros::async_test]
    async fn action_descriptor_and_style_spec_serialize_to_golden_json() {
        let values = (
            ActionDescriptor { controller_id: "ctrl".into(), action: "doThing".into(), args: Some(DslValue::float(42.0)) },
            ActionDescriptor { controller_id: "ctrl".into(), action: "doOther".into(), args: None },
            StyleSpec { variant: Some("primary".into()), size: Some("md".into()), density: None },
        );
        let json = serde_json::to_string(&values).unwrap();
        assert_eq!(json, GOLDEN_ACTION_DESCRIPTOR_JSON);
    }

    const GOLDEN_WINDOW_LAYOUT_JSON: &str = "{\"root\":{\"kind\":\"horizontal\",\"children\":[{\"kind\":\"stack\",\"size\":0.5,\"activeWindowKindId\":\"main\",\"children\":[{\"kind\":\"window\",\"windowKindId\":\"main\",\"title\":\"Main\"}]},{\"kind\":\"vertical\",\"size\":0.5,\"children\":[]}]}}";

    #[semio_framework_async_macros::async_test]
    async fn window_layout_serializes_to_golden_json() {
        let layout = WindowLayout {
            root: WindowLayoutRoot::Axis(WindowLayoutAxisNode {
                kind: "horizontal".into(),
                size: None,
                children: vec![
                    WindowLayoutChild::Stack(WindowLayoutStackNode {
                        kind: "stack".into(),
                        size: Some(0.5),
                        active_window_kind_id: Some("main".into()),
                        children: vec![WindowLayoutWindowNode { kind: "window".into(), window_kind_id: "main".into(), title: Some("Main".into()), instance_id: None, template_id: None, corner: None }],
                    }),
                    WindowLayoutChild::Axis(WindowLayoutAxisNode { kind: "vertical".into(), size: Some(0.5), children: vec![] }),
                ],
            }),
        };
        let json = serde_json::to_string(&layout).unwrap();
        assert_eq!(json, GOLDEN_WINDOW_LAYOUT_JSON);
        let roundtripped: WindowLayout = serde_json::from_str(&json).unwrap();
        assert_eq!(roundtripped, layout);
    }

    #[semio_framework_async_macros::async_test]
    async fn window_layout_discriminator_is_exact_for_serde_and_first_party_value() {
        let empty_stack: WindowLayout = serde_json::from_str(r#"{"root":{"kind":"stack","children":[]}}"#).expect("empty stack is selected by its discriminator");
        assert!(matches!(empty_stack.root, WindowLayoutRoot::Stack(_)));
        assert!(serde_json::from_str::<WindowLayout>(r#"{"root":{"kind":"bogus","children":[]}}"#).is_err());
        assert!(WindowLayoutRoot::from_value(DslValue::Object(vec![("kind".into(), DslValue::String("bogus".into()))])).is_err());
    }

    const GOLDEN_WINDOW_MEASURE_JSON: &str = "[{\"kind\":\"select\",\"id\":\"m1\",\"label\":\"Mode\",\"value\":\"a\",\"items\":[{\"id\":\"a\",\"value\":\"a\",\"label\":\"A\"}],\"onChange\":{\"controllerId\":\"ctrl\",\"action\":\"measureSelect\"}},{\"kind\":\"slider\",\"id\":\"m2\",\"label\":null,\"value\":1.0,\"min\":0.0,\"max\":2.0,\"step\":0.5,\"onChange\":{\"controllerId\":\"ctrl\",\"action\":\"measureSlider\"}},{\"kind\":\"toggle\",\"id\":\"m3\",\"iconId\":\"layout-grid\",\"label\":null,\"pressed\":true,\"text\":null,\"onChange\":{\"controllerId\":\"ctrl\",\"action\":\"measureToggle\"}},{\"kind\":\"group\",\"id\":\"m4\",\"label\":\"Group\",\"defaultOpen\":true,\"children\":[]}]";

    #[semio_framework_async_macros::async_test]
    async fn window_measure_serializes_to_golden_json() {
        let measures = vec![
            WindowMeasure::Select {
                id: "m1".into(),
                label: Some("Mode".into()),
                value: "a".into(),
                items: vec![MeasureSelectItem { id: "a".into(), value: "a".into(), label: "A".into() }],
                on_change: ActionDescriptor { controller_id: "ctrl".into(), action: "measureSelect".into(), args: None },
            },
            WindowMeasure::Slider {
                id: "m2".into(),
                label: None,
                value: 1.0,
                min: 0.0,
                max: 2.0,
                step: Some(0.5),
                ready: None,
                loading: None,
                waiting: None,
                disabled: None,
                reveal: None,
                on_change: ActionDescriptor { controller_id: "ctrl".into(), action: "measureSlider".into(), args: None },
            },
            WindowMeasure::Toggle { id: "m3".into(), icon_id: IconName::LayoutGrid, label: None, pressed: true, text: None, on_change: ActionDescriptor { controller_id: "ctrl".into(), action: "measureToggle".into(), args: None } },
            WindowMeasure::Group {
                id: "m4".into(),
                label: "Group".into(),
                default_open: Some(true),
                active_utility_id: None,
                value: None,
                min: None,
                max: None,
                step: None,
                ready: None,
                loading: None,
                waiting: None,
                on_change: None,
                children: vec![],
            },
        ];
        let json = serde_json::to_string(&measures).unwrap();
        assert_eq!(json, GOLDEN_WINDOW_MEASURE_JSON);
        let roundtripped: Vec<WindowMeasure> = serde_json::from_str(&json).unwrap();
        assert_eq!(roundtripped, measures);
    }

    const GOLDEN_MEASURE_NUMBER_JSON: &str = "[{\"kind\":\"number\",\"id\":\"count\",\"label\":\"Count\",\"value\":100.0,\"min\":0.0,\"max\":1000.0,\"step\":1.0,\"ready\":42.0,\"loading\":true,\"onChange\":{\"controllerId\":\"ctrl\",\"action\":\"setFillCount\"}},{\"kind\":\"number\",\"id\":\"count\",\"label\":null,\"value\":250.0,\"onChange\":{\"controllerId\":\"ctrl\",\"action\":\"setFillCount\"}}]";
    const GOLDEN_MEASURE_PROGRESS_JSON: &str = "[{\"kind\":\"progress\",\"id\":\"fill\",\"label\":\"Fill\",\"stage\":\"testing\",\"completed\":12.0,\"total\":100.0,\"steps\":[{\"kind\":\"info\",\"text\":\"trying 13\"},{\"kind\":\"danger\",\"text\":\"collision\"},{\"kind\":\"success\",\"text\":\"locked 12\"},{\"kind\":\"warning\",\"text\":\"document-capacity\"}],\"cancel\":{\"controllerId\":\"ctrl\",\"action\":\"cancelFill\"},\"loading\":true},{\"kind\":\"progress\",\"id\":\"fill\",\"label\":null,\"completed\":3.0,\"steps\":[]}]";

    #[semio_framework_async_macros::async_test]
    async fn window_measure_number_wire_format_round_trips_through_serde() {
        let bounded = WindowMeasure::Number {
            id: "count".into(),
            label: Some("Count".into()),
            value: 100.0,
            min: Some(0.0),
            max: Some(1000.0),
            step: Some(1.0),
            ready: Some(42.0),
            loading: Some(true),
            waiting: None,
            disabled: None,
            on_change: ActionDescriptor { controller_id: "ctrl".into(), action: "setFillCount".into(), args: None },
        };
        let unbounded = WindowMeasure::Number {
            id: "count".into(),
            label: None,
            value: 250.0,
            min: None,
            max: None,
            step: None,
            ready: None,
            loading: None,
            waiting: None,
            disabled: None,
            on_change: ActionDescriptor { controller_id: "ctrl".into(), action: "setFillCount".into(), args: None },
        };
        let json = serde_json::to_string(&vec![bounded.clone(), unbounded.clone()]).unwrap();
        assert_eq!(json, GOLDEN_MEASURE_NUMBER_JSON);
        let decoded: Vec<WindowMeasure> = serde_json::from_str(GOLDEN_MEASURE_NUMBER_JSON).unwrap();
        assert_eq!(decoded, vec![bounded, unbounded]);
        assert_eq!(serde_json::to_string(&decoded).unwrap(), GOLDEN_MEASURE_NUMBER_JSON);
        let no_ceiling = decoded.last().expect("two measures");
        assert!(matches!(no_ceiling, WindowMeasure::Number { max: None, min: None, .. }), "an absent ceiling must decode as None, never as a default number");
    }

    #[semio_framework_async_macros::async_test]
    async fn window_measure_progress_wire_format_round_trips_through_serde() {
        let determinate = WindowMeasure::Progress {
            id: "fill".into(),
            label: Some("Fill".into()),
            stage: Some("testing".into()),
            completed: 12.0,
            total: Some(100.0),
            steps: vec![
                MeasureProgressStep { kind: MeasureProgressStepKind::Info, text: "trying 13".into() },
                MeasureProgressStep { kind: MeasureProgressStepKind::Danger, text: "collision".into() },
                MeasureProgressStep { kind: MeasureProgressStepKind::Success, text: "locked 12".into() },
                MeasureProgressStep { kind: MeasureProgressStepKind::Warning, text: "document-capacity".into() },
            ],
            cancel: Some(ActionDescriptor { controller_id: "ctrl".into(), action: "cancelFill".into(), args: None }),
            loading: Some(true),
        };
        let indeterminate = WindowMeasure::Progress { id: "fill".into(), label: None, stage: None, completed: 3.0, total: None, steps: vec![], cancel: None, loading: None };
        let json = serde_json::to_string(&vec![determinate.clone(), indeterminate.clone()]).unwrap();
        assert_eq!(json, GOLDEN_MEASURE_PROGRESS_JSON);
        let decoded: Vec<WindowMeasure> = serde_json::from_str(GOLDEN_MEASURE_PROGRESS_JSON).unwrap();
        assert_eq!(decoded, vec![determinate, indeterminate]);
        assert_eq!(serde_json::to_string(&decoded).unwrap(), GOLDEN_MEASURE_PROGRESS_JSON);
        assert!(matches!(decoded.last(), Some(WindowMeasure::Progress { total: None, .. })), "an absent total must stay indeterminate, never collapse to zero");
    }

    #[semio_framework_async_macros::async_test]
    async fn measure_progress_step_kind_wire_tags_match_the_semantic_tokens() {
        for (kind, tag) in [
            (MeasureProgressStepKind::Info, "\"info\""),
            (MeasureProgressStepKind::Success, "\"success\""),
            (MeasureProgressStepKind::Warning, "\"warning\""),
            (MeasureProgressStepKind::Danger, "\"danger\""),
        ] {
            assert_eq!(serde_json::to_string(&kind).unwrap(), tag);
            assert_eq!(serde_json::from_str::<MeasureProgressStepKind>(tag).unwrap(), kind);
            assert_eq!(format!("\"{}\"", kind.as_str()), tag);
        }
    }

    fn utility_scoped_group(id: &str, utility: Option<&str>, children: Vec<WindowMeasure>) -> WindowMeasure {
        WindowMeasure::Group {
            id: id.into(),
            label: id.to_uppercase(),
            default_open: None,
            active_utility_id: utility.map(str::to_string),
            children,
            value: None,
            min: None,
            max: None,
            step: None,
            ready: None,
            loading: None,
            waiting: None,
            on_change: None,
        }
    }

    fn measure_toggle(id: &str) -> WindowMeasure {
        WindowMeasure::Toggle { id: id.into(), icon_id: IconName::PanelLeft, label: Some(id.into()), pressed: true, text: None, on_change: ActionDescriptor { controller_id: "c".into(), action: "t".into(), args: None } }
    }

    #[semio_framework_async_macros::async_test]
    async fn partition_window_measures_unwraps_matching_utility_group_children_into_utility_options() {
        let measures = vec![utility_scoped_group("brush-params", Some("brush"), vec![measure_toggle("size")])];
        let partition = partition_window_measures(&measures, Some("brush")).expect("fixed partition");
        assert!(partition.general.is_empty());
        assert_eq!(partition.utility_options.len(), 1);
        assert!(matches!(partition.utility_options.get(0).copied(), Some(WindowMeasure::Toggle { id, .. }) if id == "size"), "tagged wrapper is routing-only — children render flat");
    }

    #[semio_framework_async_macros::async_test]
    async fn partition_window_measures_drops_non_matching_utility_group_from_both_buckets() {
        let measures = vec![utility_scoped_group("brush-params", Some("brush"), vec![measure_toggle("size")])];
        let other = partition_window_measures(&measures, Some("fill")).expect("fixed partition");
        assert!(other.general.is_empty() && other.utility_options.is_empty(), "wrong active utility drops the group entirely");
        let none = partition_window_measures(&measures, None).expect("fixed partition");
        assert!(none.general.is_empty() && none.utility_options.is_empty(), "no active utility drops the group entirely");
    }

    #[semio_framework_async_macros::async_test]
    async fn partition_window_measures_keeps_untagged_group_and_non_group_in_general() {
        let measures = vec![
            utility_scoped_group("grid", None, vec![]),
            WindowMeasure::Slider {
                id: "zoom".into(),
                label: None,
                value: 1.0,
                min: 0.0,
                max: 2.0,
                step: None,
                ready: None,
                loading: None,
                waiting: None,
                disabled: None,
                reveal: None,
                on_change: ActionDescriptor { controller_id: "c".into(), action: "z".into(), args: None },
            },
        ];
        let partition = partition_window_measures(&measures, Some("brush")).expect("fixed partition");
        assert_eq!(partition.general.len(), 2, "untagged group and slider both stay general");
        assert!(partition.utility_options.is_empty());
    }

    #[semio_framework_async_macros::async_test]
    async fn partition_window_measures_empty_input_roundtrips_to_empty() {
        let partition = partition_window_measures(&[], Some("brush")).expect("fixed partition");
        assert!(partition.general.is_empty() && partition.utility_options.is_empty());
    }

    #[semio_framework_async_macros::async_test]
    async fn partition_window_measures_max_plus_one_returns_exact_borrowed_owner() {
        let measures = (0..=WINDOW_MEASURE_PARTITION_CAPACITY).map(|index| measure_toggle(&format!("measure-{index}"))).collect::<Vec<_>>();
        let rejected = partition_window_measures(&measures, None).expect_err("maximum plus one must refuse");
        assert!(std::ptr::eq(rejected, &measures[WINDOW_MEASURE_PARTITION_CAPACITY]));
    }

    const GOLDEN_WINDOW_ENGAGEMENT_JSON: &str = "{\"sessionActive\":true,\"options\":[{\"id\":\"opt1\",\"label\":\"Option\",\"pressed\":false}],\"input\":{\"id\":\"in1\",\"value\":\"v\"},\"control\":{\"kind\":\"slider\",\"id\":\"sl1\",\"label\":null,\"value\":1.0,\"min\":0.0,\"max\":2.0,\"step\":null,\"unit\":null,\"disabled\":null,\"onChange\":null,\"onCommit\":null},\"status\":[{\"id\":\"st1\",\"text\":\"Ready\"}],\"possibleEngagements\":[{\"id\":\"pe1\",\"label\":\"Possible\"}]}";

    #[semio_framework_async_macros::async_test]
    async fn window_engagement_serializes_to_golden_json() {
        let engagement = WindowEngagement {
            session_active: Some(true),
            options: Some(vec![WindowEngagementOption { id: "opt1".into(), label: Some("Option".into()), icon_id: None, pressed: Some(false), disabled: None, action: None }]),
            input: Some(WindowEngagementInput { id: Some("in1".into()), value: Some("v".into()), placeholder: None, disabled: None, on_change: None, on_submit: None, on_repeat_last: None, on_abort: None }),
            control: Some(WindowEngagementControl::Slider { id: Some("sl1".into()), label: None, value: 1.0, min: 0.0, max: 2.0, step: None, unit: None, disabled: None, on_change: None, on_commit: None }),
            controls: None,
            status: Some(vec![WindowEngagementStatus { id: "st1".into(), text: "Ready".into() }]),
            possible_engagements: Some(vec![WindowEngagementPossible { id: "pe1".into(), label: "Possible".into(), detail: None, action: None }]),
        };
        let json = serde_json::to_string(&engagement).unwrap();
        assert_eq!(json, GOLDEN_WINDOW_ENGAGEMENT_JSON);
        let roundtripped: WindowEngagement = serde_json::from_str(&json).unwrap();
        assert_eq!(roundtripped, engagement);
    }
}
