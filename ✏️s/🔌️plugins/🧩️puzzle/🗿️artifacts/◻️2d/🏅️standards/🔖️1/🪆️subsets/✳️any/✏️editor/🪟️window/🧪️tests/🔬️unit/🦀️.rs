use super::*;

    #[test]
    fn same_kind_windows_compose_isolated_runtime() {
        let shared = crate::editor::puzzle2d::config::Puzzle2dConfig::default();
        let first = Puzzle2dWindowConfig { camera_x: 12.0, grid_factor: 4.0, ..Default::default() };
        let second = Puzzle2dWindowConfig { camera_x: -7.0, grid_factor: 0.5, ..Default::default() };
        let first_runtime = runtime(&shared, &first, &Puzzle2dWindowTransient::default(), Some(overview::WINDOW_KIND_ID));
        let second_runtime = runtime(&shared, &second, &Puzzle2dWindowTransient::default(), Some(overview::WINDOW_KIND_ID));
        assert_eq!(first_runtime.camera_x, 12.0);
        assert_eq!(second_runtime.camera_x, -7.0);
        assert_eq!(first_runtime.grid_factor, 4.0);
        assert_eq!(second_runtime.grid_factor, 0.5);
    }

    #[test]
    fn app_pack_and_spr_exclude_window_and_transient_fields() {
        let shared = crate::editor::puzzle2d::config::Puzzle2dConfig::default();
        let spr = dsl::json::to_json_string(&shared);
        let oracle: serde_json::Value = serde_json::from_str(&spr).expect("serde_json oracle accepts the neutral config");
        assert_eq!(oracle.as_object().map(serde_json::Map::len), Some(2));
        let pack = store::ArtifactPack::encode_pack(&shared);
        for forbidden in ["cameraX", "engagementInput", "brushCandidates", "fillJobCheckpointSequence"] {
            assert!(!spr.contains(forbidden));
            assert!(!pack.windows(forbidden.len()).any(|bytes| bytes == forbidden.as_bytes()));
        }
    }

    #[test]
    fn document_camera_is_only_the_initial_window_seed() {
        let document = serde_json::json!({ "camera": { "x": 8.0, "y": -3.0, "zoom": 2.5 } });
        let seed = document_seed(&document);
        assert_eq!((seed.camera_x, seed.camera_y, seed.camera_zoom), (8.0, -3.0, 2.5));
        let live = Puzzle2dWindowConfig { camera_x: 21.0, ..seed.clone() };
        assert_ne!(live.camera_x, document_seed(&document).camera_x);
    }

    #[test]
    fn nested_transient_retirement_and_preflight_are_bounded_by_exact_grants() {
        let transient = Puzzle2dWindowTransient {
            engagement_input: "two-dimensional-input".repeat(256),
            brush_candidates: vec![dsl::DslValue::Object(vec![(
                "candidate".into(),
                dsl::DslValue::Array(vec![dsl::DslValue::String("nested-owner".repeat(512))]),
            )])],
            brush_candidate_source_handle_id: "source-handle".repeat(256),
            ..Default::default()
        };
        let footprint = puzzle2d_window_transient_preflight(&Puzzle2dWindowTransientMutation::Snapshot { transient: transient.clone() }).expect("admitted exact footprint");
        assert_eq!(footprint.work_items, 1);
        assert!(footprint.retained_bytes < store::ARTIFACT_STORE_ONE_ITEM_MAXIMUM_BYTES);
        let mut retirement = store::retirement::owned_retirement(Puzzle2dWindowTransientMutation::Snapshot { transient });
        for _ in 0..32_768 {
            match retirement.close_step(1, 1).expect("bounded retirement") {
                store::SnapshotRetirementStep::Complete => {
                    assert!(retirement.terminal_is_empty());
                    return;
                }
                store::SnapshotRetirementStep::Pending { released_items, released_bytes } => {
                    assert!(released_items <= 1);
                    assert!(released_bytes <= 1);
                }
                store::SnapshotRetirementStep::Blocked => panic!("owned Puzzle 2D transient retirement blocked"),
            }
        }
        panic!("owned Puzzle 2D transient retirement exceeded its bounded cursor turns");
    }

    fn retire_returned_puzzle2d_transient(transient: Puzzle2dWindowTransient) {
        let mut retirement = store::retirement::owned_retirement(Puzzle2dWindowTransientMutation::Snapshot { transient });
        for _ in 0..4_096 {
            match retirement.close_step(1, 1).expect("returned Puzzle 2D owner retirement") {
                store::SnapshotRetirementStep::Complete => {
                    assert!(retirement.terminal_is_empty());
                    return;
                }
                store::SnapshotRetirementStep::Pending { released_items, released_bytes } => {
                assert!(released_items <= 1);
                assert!(released_bytes <= 1);
            }
                store::SnapshotRetirementStep::Blocked => panic!("returned Puzzle 2D owner retirement blocked"),
            }
        }
        panic!("returned Puzzle 2D owner retirement exceeded its bounded cursor turns");
    }

    fn rejected_puzzle2d_transient(transient: Puzzle2dWindowTransient) -> Puzzle2dWindowTransient {
        let owners = <Puzzle2dOverviewWindowTransientOwner as semio_framework_plugin::WindowTransientOwner>::build_owners();
        let request = store::ArtifactEphemeralOneItemPreparationRequest {
            operation: semio_framework_job::OperationId(1),
            generation: semio_framework_job::Generation(1),
            base: store::ArtifactEphemeralBaseRead(store::ArtifactEphemeralBaseOwner::Transient(std::sync::Arc::new(Puzzle2dWindowTransient::default()))),
            mutation: Puzzle2dWindowTransientMutation::Snapshot { transient },
        };
        let returned = match owners.preparation.begin(request) {
            Err(request) => request,
            Ok(_) => panic!("oversized Puzzle 2D retained capacity must reject"),
        };
        let Puzzle2dWindowTransientMutation::Snapshot { transient } = returned.mutation;
        transient
    }

    #[test]
    fn every_oversized_string_and_vector_capacity_returns_the_exact_puzzle2d_owner() {
        let maximum = store::ARTIFACT_STORE_ONE_ITEM_MAXIMUM_BYTES;
        {
            let mut value = String::with_capacity(maximum + 1);
            value.push('i');
            let pointer = value.as_ptr();
            let capacity = value.capacity();
            let returned = rejected_puzzle2d_transient(Puzzle2dWindowTransient { engagement_input: value, ..Default::default() });
            assert_eq!(returned.engagement_input, "i");
            assert_eq!(returned.engagement_input.as_ptr(), pointer);
            assert_eq!(returned.engagement_input.capacity(), capacity);
            retire_returned_puzzle2d_transient(returned);
        }
        {
            let mut value = String::with_capacity(maximum + 1);
            value.push('h');
            let pointer = value.as_ptr();
            let capacity = value.capacity();
            let returned = rejected_puzzle2d_transient(Puzzle2dWindowTransient { brush_candidate_source_handle_id: value, ..Default::default() });
            assert_eq!(returned.brush_candidate_source_handle_id, "h");
            assert_eq!(returned.brush_candidate_source_handle_id.as_ptr(), pointer);
            assert_eq!(returned.brush_candidate_source_handle_id.capacity(), capacity);
            retire_returned_puzzle2d_transient(returned);
        }
        {
            let values = Vec::with_capacity(maximum / std::mem::size_of::<dsl::DslValue>() + 1);
            let pointer = values.as_ptr();
            let capacity = values.capacity();
            let returned = rejected_puzzle2d_transient(Puzzle2dWindowTransient { brush_candidates: values, ..Default::default() });
            assert!(returned.brush_candidates.is_empty());
            assert_eq!(returned.brush_candidates.as_ptr(), pointer);
            assert_eq!(returned.brush_candidates.capacity(), capacity);
            retire_returned_puzzle2d_transient(returned);
        }
        {
            let mut value = String::with_capacity(maximum + 1);
            value.push('s');
            let pointer = value.as_ptr();
            let capacity = value.capacity();
            let returned = rejected_puzzle2d_transient(Puzzle2dWindowTransient {
                brush_candidates: vec![dsl::DslValue::String(value)],
                ..Default::default()
            });
            let dsl::DslValue::String(value) = &returned.brush_candidates[0] else { panic!("returned nested string") };
            assert_eq!(value, "s");
            assert_eq!(value.as_ptr(), pointer);
            assert_eq!(value.capacity(), capacity);
            retire_returned_puzzle2d_transient(returned);
        }
        {
            let values = Vec::with_capacity(maximum / std::mem::size_of::<dsl::DslValue>() + 1);
            let pointer = values.as_ptr();
            let capacity = values.capacity();
            let returned = rejected_puzzle2d_transient(Puzzle2dWindowTransient {
                brush_candidates: vec![dsl::DslValue::Array(values)],
                ..Default::default()
            });
            let dsl::DslValue::Array(values) = &returned.brush_candidates[0] else { panic!("returned nested array") };
            assert!(values.is_empty());
            assert_eq!(values.as_ptr(), pointer);
            assert_eq!(values.capacity(), capacity);
            retire_returned_puzzle2d_transient(returned);
        }
        {
            let entries = Vec::with_capacity(maximum / std::mem::size_of::<(String, dsl::DslValue)>() + 1);
            let pointer = entries.as_ptr();
            let capacity = entries.capacity();
            let returned = rejected_puzzle2d_transient(Puzzle2dWindowTransient {
                brush_candidates: vec![dsl::DslValue::Object(entries)],
                ..Default::default()
            });
            let dsl::DslValue::Object(entries) = &returned.brush_candidates[0] else { panic!("returned nested object") };
            assert!(entries.is_empty());
            assert_eq!(entries.as_ptr(), pointer);
            assert_eq!(entries.capacity(), capacity);
            retire_returned_puzzle2d_transient(returned);
        }
        {
            let mut key = String::with_capacity(maximum + 1);
            key.push('k');
            let pointer = key.as_ptr();
            let capacity = key.capacity();
            let returned = rejected_puzzle2d_transient(Puzzle2dWindowTransient {
                brush_candidates: vec![dsl::DslValue::Object(vec![(key, dsl::DslValue::Null)])],
                ..Default::default()
            });
            let dsl::DslValue::Object(entries) = &returned.brush_candidates[0] else { panic!("returned keyed object") };
            assert_eq!(entries[0].0, "k");
            assert_eq!(entries[0].0.as_ptr(), pointer);
            assert_eq!(entries[0].0.capacity(), capacity);
            retire_returned_puzzle2d_transient(returned);
        }
    }

    /// 🎒️ Every persisted pane option survives one pack and one text round trip — the record-backed
    /// codecs this owner gained when `WindowConfigOwner::State` grew its `DslField` bound.
    #[test]
    fn window_config_pack_round_trips_every_persisted_option() {
        let original = Puzzle2dWindowConfig {
            camera_x: 3.5,
            camera_y: -7.25,
            camera_zoom: 2.5,
            lod_mode: "manual".into(),
            fill_count: 17,
            grid_snap_enabled: true,
            grid_factor: 0.25,
            suggestion_offset: 4.75,
        };
        assert_ne!(original, Puzzle2dWindowConfig::default(), "the fixture must differ from the default in every field it asserts");
        let bytes = store::ArtifactPack::encode_pack(&original);
        let decoded = <Puzzle2dWindowConfig as store::ArtifactPack>::decode_pack(&bytes).expect("window config pack decodes");
        assert_eq!(decoded, original, "one pack round trip must preserve every persisted pane option");
        let printed = store::ArtifactDsl::print_dsl(&original);
        let parsed = <Puzzle2dWindowConfig as store::ArtifactDsl>::parse_dsl(&printed).expect("window config text parses");
        assert_eq!(parsed, original, "one text round trip must preserve every persisted pane option");
    }
