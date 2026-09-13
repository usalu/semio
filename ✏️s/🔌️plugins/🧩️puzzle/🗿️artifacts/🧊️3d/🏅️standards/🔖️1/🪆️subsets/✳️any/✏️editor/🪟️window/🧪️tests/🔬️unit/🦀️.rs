use super::*;

    #[test]
    fn same_kind_windows_compose_independently() {
        let shared = Puzzle3dConfig::default();
        let first = Puzzle3dWindowConfig { grid_spacing: 2.0, camera: Puzzle3dCamera { zoom: 4.0, ..Default::default() }, ..Default::default() };
        let second = Puzzle3dWindowConfig { grid_spacing: 40.0, camera: Puzzle3dCamera { zoom: 0.5, ..Default::default() }, ..Default::default() };
        let first_runtime = runtime(&shared, &first, &Puzzle3dWindowTransient::default(), None);
        let second_runtime = runtime(&shared, &second, &Puzzle3dWindowTransient::default(), None);
        assert_eq!((first_runtime.grid_spacing, first_runtime.camera.zoom), (2.0, 4.0));
        assert_eq!((second_runtime.grid_spacing, second_runtime.camera.zoom), (40.0, 0.5));
    }

    #[test]
    fn app_pack_and_spr_exclude_window_transient_and_operation_fields() {
        let shared = Puzzle3dConfig::default();
        let spr = dsl::json::to_json_string(&shared);
        let oracle: serde_json::Value = serde_json::from_str(&spr).expect("serde_json oracle accepts the neutral config");
        // 🧮️ Four fill/distribution preferences plus `activeExampleId` (wave B26) — see the twin law in
        // `✏️editor/🧪️tests/🔬️unit/🦀️.rs`.
        assert_eq!(oracle.as_object().map(serde_json::Map::len), Some(5));
        let pack = store::ArtifactPack::encode_pack(&shared);
        for forbidden in ["camera", "windowOptions", "engagementInput", "suggestionMenu", "fillCheckpoint", "fillApplyGeneration"] {
            assert!(!spr.contains(forbidden));
            assert!(!pack.windows(forbidden.len()).any(|bytes| bytes == forbidden.as_bytes()));
        }
    }

    #[test]
    fn suggestion_and_input_retirement_reaches_terminal_empty_with_tiny_grants() {
        let transient = Puzzle3dWindowTransient {
            suggestion_menu: Some(Puzzle3dSuggestionMenu {
                x: 3.0,
                y: 5.0,
                window_id: "three-dimensional-window".repeat(256),
                vortex_full_id: "vortex-owner".repeat(256),
            }),
            engagement_input: "three-dimensional-input".repeat(256),
            brush_candidate_index: 2,
            activation: "three-dimensional-utility".repeat(256),
        };
        let mutation = Puzzle3dWindowTransientMutation::Snapshot { transient };
        assert!(puzzle3d_window_transient_preflight(&mutation).expect("large Puzzle 3D transient admission").is_admissible());
        let mut retirement = store::retirement::owned_retirement(mutation);
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
                store::SnapshotRetirementStep::Blocked => panic!("owned Puzzle 3D transient retirement blocked"),
            }
        }
        panic!("owned Puzzle 3D transient retirement exceeded its bounded cursor turns");
    }

    fn retire_returned_puzzle3d_transient(transient: Puzzle3dWindowTransient) {
        let mut retirement = store::retirement::owned_retirement(Puzzle3dWindowTransientMutation::Snapshot { transient });
        for _ in 0..4_096 {
            match retirement.close_step(1, 1).expect("returned Puzzle 3D owner retirement") {
                store::SnapshotRetirementStep::Complete => {
                    assert!(retirement.terminal_is_empty());
                    return;
                }
                store::SnapshotRetirementStep::Pending { released_items, released_bytes } => {
                assert!(released_items <= 1);
                assert!(released_bytes <= 1);
            }
                store::SnapshotRetirementStep::Blocked => panic!("returned Puzzle 3D owner retirement blocked"),
            }
        }
        panic!("returned Puzzle 3D owner retirement exceeded its bounded cursor turns");
    }

    fn rejected_puzzle3d_transient(transient: Puzzle3dWindowTransient) -> Puzzle3dWindowTransient {
        let owners = <Puzzle3dWindowTransientOwner as semio_framework_plugin::WindowTransientOwner>::build_owners();
        let request = store::ArtifactEphemeralOneItemPreparationRequest {
            operation: semio_framework_job::OperationId(1),
            generation: semio_framework_job::Generation(1),
            base: store::ArtifactEphemeralBaseRead(store::ArtifactEphemeralBaseOwner::Transient(std::sync::Arc::new(Puzzle3dWindowTransient::default()))),
            mutation: Puzzle3dWindowTransientMutation::Snapshot { transient },
        };
        let returned = match owners.preparation.begin(request) {
            Err(request) => request,
            Ok(_) => panic!("oversized Puzzle 3D retained capacity must reject"),
        };
        let Puzzle3dWindowTransientMutation::Snapshot { transient } = returned.mutation;
        transient
    }

    #[test]
    fn every_oversized_string_capacity_returns_the_exact_puzzle3d_owner() {
        {
            let mut engagement_input = String::with_capacity(store::ARTIFACT_STORE_ONE_ITEM_MAXIMUM_BYTES + 1);
            engagement_input.push('3');
            let pointer = engagement_input.as_ptr();
            let capacity = engagement_input.capacity();
            let transient = rejected_puzzle3d_transient(Puzzle3dWindowTransient { engagement_input, ..Default::default() });
            assert_eq!(transient.engagement_input, "3");
            assert_eq!(transient.engagement_input.as_ptr(), pointer);
            assert_eq!(transient.engagement_input.capacity(), capacity);
            retire_returned_puzzle3d_transient(transient);
        }
        {
            let mut window_id = String::with_capacity(store::ARTIFACT_STORE_ONE_ITEM_MAXIMUM_BYTES + 1);
            window_id.push('w');
            let pointer = window_id.as_ptr();
            let capacity = window_id.capacity();
            let transient = rejected_puzzle3d_transient(Puzzle3dWindowTransient {
                suggestion_menu: Some(Puzzle3dSuggestionMenu { x: 0.0, y: 0.0, window_id, vortex_full_id: String::new() }),
                ..Default::default()
            });
            let window_id = &transient.suggestion_menu.as_ref().expect("returned suggestion menu").window_id;
            assert_eq!(window_id, "w");
            assert_eq!(window_id.as_ptr(), pointer);
            assert_eq!(window_id.capacity(), capacity);
            retire_returned_puzzle3d_transient(transient);
        }
        {
            let mut vortex_full_id = String::with_capacity(store::ARTIFACT_STORE_ONE_ITEM_MAXIMUM_BYTES + 1);
            vortex_full_id.push('v');
            let pointer = vortex_full_id.as_ptr();
            let capacity = vortex_full_id.capacity();
            let transient = rejected_puzzle3d_transient(Puzzle3dWindowTransient {
                suggestion_menu: Some(Puzzle3dSuggestionMenu { x: 0.0, y: 0.0, window_id: String::new(), vortex_full_id }),
                ..Default::default()
            });
            let vortex_full_id = &transient.suggestion_menu.as_ref().expect("returned suggestion menu").vortex_full_id;
            assert_eq!(vortex_full_id, "v");
            assert_eq!(vortex_full_id.as_ptr(), pointer);
            assert_eq!(vortex_full_id.capacity(), capacity);
            retire_returned_puzzle3d_transient(transient);
        }
    }

    /// 🫧️🏛️ Interaction scratch belongs to exactly ONE host activation and never outlives it.
    /// `setActiveTool`/`setActiveUtility` reach no app reducer at all — the framework dispatches both
    /// as an empty `Emit` (`🔌️plugin/🦀️.rs` `dispatch_action`) — so this comparison against the
    /// activation the host stamps on the very NEXT call is the only thing that can retire a suggestion
    /// popup, an engagement input line or a brush candidate index when the user switches tool.
    #[test]
    fn window_transient_scratch_does_not_outlive_the_activation_it_was_captured_under() {
        let held = Puzzle3dWindowTransient {
            suggestion_menu: Some(Puzzle3dSuggestionMenu { x: 1.0, y: 2.0, window_id: "puzzle3d-main".into(), vortex_full_id: "object::vortex".into() }),
            engagement_input: "12".into(),
            brush_candidate_index: 3,
            activation: "brush".into(),
        };
        assert_eq!(live_transient(&held, "brush"), held, "the activation that captured the scratch still owns it");
        let switched = live_transient(&held, "transform");
        assert_eq!((switched.suggestion_menu, switched.engagement_input, switched.brush_candidate_index), (None, String::new(), 0), "switching utility retires every scratch field");
        assert_eq!(switched.activation, "transform", "the retired scratch is re-stamped with the activation that now owns the window");
        assert!(live_transient(&held, "").suggestion_menu.is_none(), "deactivating every tool and utility retires the scratch too");
    }

    /// 🏛️ Tool and utility are mutually exclusive interaction owners and the mode-wide tool wins, which
    /// is the same precedence `puzzle3d_scene_active_utility` resolves the scene mode with.
    #[test]
    fn host_activation_reads_the_tool_first_then_the_addressed_window_utility() {
        let mut view = semio_framework_plugin::ViewModel { active_utility_id: Some("transform".into()), ..Default::default() };
        assert_eq!(host_activation(Some(&view)), "transform");
        view.active_tool_id = Some("fill".into());
        assert_eq!(host_activation(Some(&view)), "fill");
        assert_eq!(host_activation(None), String::new());
    }

    /// 🎒️ Every persisted window option survives one pack encode/decode — the record-backed
    /// `ArtifactPack` this owner gained when `WindowConfigOwner::State` grew its `DslField` bound,
    /// exercised on a value where no field holds its default.
    #[test]
    fn window_config_pack_round_trips_every_persisted_option() {
        let mut panel_pages = BTreeMap::new();
        panel_pages.insert("objects".to_string(), 3u32);
        panel_pages.insert("references".to_string(), 1u32);
        let original = Puzzle3dWindowConfig {
            lod_automatic: false,
            lod_depth_variable: true,
            grid_visible: false,
            lod_manual: 42.5,
            grid_snap_enabled: true,
            grid_spacing: 2.5,
            selectable_kinds: Puzzle3dSelectableKinds { objects: false, vortices: true, attractions: false },
            proximity_radius: 7.25,
            chunk_size: 128.0,
            voxel_dims: [2, 3, 4],
            transform_move: false,
            transform_rotate: true,
            vortex_show: "all".into(),
            vortex_direction: "outgoing".into(),
            selection_method: "lasso".into(),
            sun: WorldSunConfig { enabled: true, azimuth: 12.5, elevation: 33.0, intensity: 0.5, color: "#102030".into() },
            camera: Puzzle3dCamera { position: [1.0, 2.0, 3.0], target: [4.0, 5.0, 6.0], zoom: 2.5, up: Some([0.0, 0.0, 1.0]), projection: Default::default() },
            panel_pages,
        };
        assert_ne!(original, Puzzle3dWindowConfig::default(), "the fixture must differ from the default in every field it asserts");
        let bytes = store::ArtifactPack::encode_pack(&original);
        let decoded = <Puzzle3dWindowConfig as store::ArtifactPack>::decode_pack(&bytes).expect("window config pack decodes");
        assert_eq!(decoded, original, "one pack round trip must preserve every persisted window option");
        let printed = store::ArtifactDsl::print_dsl(&original);
        let parsed = <Puzzle3dWindowConfig as store::ArtifactDsl>::parse_dsl(&printed).expect("window config text parses");
        assert_eq!(parsed, original, "one text round trip must preserve every persisted window option");
    }
