use super::*;

    fn retire_to_terminal_empty<T: semio_framework_value::retirement::RetireOwned + 'static>(owner: T) {
        let mut retirement = semio_framework_value::retirement::controlled::ControlledRetirement::new(owner).unwrap_or_else(|(error, _)| panic!("window retirement admission refused: {error:?}"));
        for _ in 0..65_536 {
            if retirement.terminal_is_empty() {
                return;
            }
            let copy = retirement.next_copy_byte_demand().expect("window retirement copy quote");
            let grant = semio_framework_value::retained_clone::RetainedCloneGrant {
                maximum_items: 1,
                maximum_copy_bytes: copy,
                maximum_capacity_bytes: retirement.next_capacity_byte_demand(copy).expect("window retirement capacity quote"),
                maximum_release_bytes: retirement.next_release_byte_demand().expect("window retirement release quote"),
                maximum_depth: retirement.next_depth_demand().expect("window retirement depth quote").max(1),
            };
            retirement.step(grant).expect("bounded window retirement turn");
        }
        panic!("window retirement exceeded its bounded cursor turns");
    }

    #[test]
    fn same_kind_window_values_are_independent() {
        let shared = crate::editor::puzzle5d::config::Puzzle5dConfig::default();
        let first = Puzzle5dBoardWindowConfig { camera2d: Puzzle5dCamera2d { x: 9.0, y: 0.0, zoom: 1.0 }, ..Default::default() };
        let second = Puzzle5dBoardWindowConfig { camera2d: Puzzle5dCamera2d { x: -4.0, y: 0.0, zoom: 1.0 }, ..Default::default() };
        let first = runtime(&shared, &Puzzle5dWindowConfig { camera2d: first.camera2d, ..Default::default() }, &Puzzle5dWindowTransient::default(), "board-first");
        let second = runtime(&shared, &Puzzle5dWindowConfig { camera2d: second.camera2d, ..Default::default() }, &Puzzle5dWindowTransient::default(), "board-second");
        assert_eq!(first.camera2d.x, 9.0);
        assert_eq!(second.camera2d.x, -4.0);
    }

    #[test]
    fn app_pack_and_spr_exclude_window_and_transient_fields() {
        let shared = crate::editor::puzzle5d::config::Puzzle5dConfig::default();
        let spr = semio_framework_pack_json::to_json_string(&shared);
        let oracle: serde_json::Value = serde_json::from_str(&spr).expect("serde_json oracle accepts the neutral config");
        assert_eq!(oracle.as_object().map(serde_json::Map::len), Some(6));
        assert_eq!(oracle["fillCount"], serde_json::json!(crate::editor::puzzle5d::PUZZLE5D_DEFAULT_FILL_COUNT), "the fill count is document-instance configuration the fill run reads");
        let pack = store::ArtifactPack::encode_pack(&shared);
        for forbidden in ["camera2d", "camera3d", "engagementInput", "brushCandidateIndex", "suggestionMenu", "sun"] {
            assert!(!spr.contains(forbidden));
            assert!(!pack.windows(forbidden.len()).any(|bytes| bytes == forbidden.as_bytes()));
        }
    }

    #[test]
    fn transient_string_retirement_reaches_terminal_empty_with_tiny_grants() {
        let transient = Puzzle5dWindowTransient {
            suggestion_menu: Some(super::Puzzle5dSuggestionMenu { x: 3.0, y: 5.0, window_id: "five-dimensional-window".repeat(256), vortex_full_id: "grip-owner".repeat(256), submenu: true }),
            engagement_input: "five-dimensional-input".repeat(512),
            brush_candidate_index: 3,
        };
        let mutation = Puzzle5dWindowTransientMutation::Snapshot { transient };
        assert!(puzzle5d_window_transient_preflight(&mutation).expect("large Puzzle 5D transient admission").is_admissible());
        retire_to_terminal_empty(mutation);
    }

    fn retire_returned_puzzle5d_transient(transient: Puzzle5dWindowTransient) {
        retire_to_terminal_empty(Puzzle5dWindowTransientMutation::Snapshot { transient });
    }

    #[test]
    fn oversized_string_capacity_rejects_and_returns_the_exact_puzzle5d_owner() {
        let owners = <Puzzle5dBoardWindowTransientOwner as semio_framework_plugin::WindowTransientOwner>::build_owners();
        let mut engagement_input = String::with_capacity(store::ARTIFACT_STORE_ONE_ITEM_MAXIMUM_BYTES + 1);
        engagement_input.push('5');
        let pointer = engagement_input.as_ptr();
        let capacity = engagement_input.capacity();
        let request = store::ArtifactEphemeralOneItemPreparationRequest {
            operation: semio_framework_job::OperationId(1),
            generation: semio_framework_job::Generation(1),
            base: store::ArtifactEphemeralBaseRead(store::ArtifactEphemeralBaseOwner::Transient(std::sync::Arc::new(Puzzle5dWindowTransient::default()))),
            mutation: Puzzle5dWindowTransientMutation::Snapshot { transient: Puzzle5dWindowTransient { engagement_input, ..Default::default() } },
        };
        let returned = match owners.preparation.begin(request) {
            Err(request) => request,
            Ok(_) => panic!("oversized Puzzle 5D string capacity must reject"),
        };
        let Puzzle5dWindowTransientMutation::Snapshot { transient } = returned.mutation;
        assert_eq!(transient.engagement_input, "5");
        assert_eq!(transient.engagement_input.as_ptr(), pointer);
        assert_eq!(transient.engagement_input.capacity(), capacity);
        retire_returned_puzzle5d_transient(transient);
    }

    /// 🎒️ Both Puzzle 5D window kinds survive one pack and one text round trip — the record-backed
    /// codecs they gained when `WindowConfigOwner::State` grew its `DslField` bound.
    #[test]
    fn window_configs_pack_round_trip_every_persisted_option() {
        let board = Puzzle5dBoardWindowConfig {
            camera2d: Puzzle5dCamera2d { x: 3.5, y: -7.25, zoom: 2.5 },
            lod_mode: "manual".into(),
            suggestion_offset: 4.75,
            grid_snap_enabled: false,
            grid_factor: 0.25,
            grid_visible: false,
            selectable_kinds: Puzzle5dSelectableKinds { parts: true, grips: false, fasteners: true },
        };
        assert_ne!(board, Puzzle5dBoardWindowConfig::default(), "the board fixture must differ from the default");
        let decoded = <Puzzle5dBoardWindowConfig as store::ArtifactPack>::decode_pack(&store::ArtifactPack::encode_pack(&board)).expect("board pack decodes");
        assert_eq!(decoded, board, "one board pack round trip must preserve every persisted option");
        let parsed = <Puzzle5dBoardWindowConfig as store::ArtifactDsl>::parse_dsl(&store::ArtifactDsl::print_dsl(&board)).expect("board text parses");
        assert_eq!(parsed, board, "one board text round trip must preserve every persisted option");

        let world = Puzzle5dWorldWindowConfig {
            camera3d: Puzzle5dCamera3d {
                position: [1.0, 2.0, 3.0],
                target: [4.0, 5.0, 6.0],
                zoom: 2.5,
                up: Some([0.0, 0.0, 1.0]),
                projection: semio_framework_plugin::WorldProjectionConfig { kind: "orthographic".into(), orthographic_view: "front".into(), fov: 35.0, ..Default::default() },
            },
            sun: WorldSunConfig { enabled: true, azimuth: 12.5, elevation: 33.0, intensity: 0.5, color: "#102030".into() },
            grid_visible: false,
            grid_snap_enabled: true,
            grid_spacing: 2.5,
            lod_automatic: false,
            lod_depth_variable: true,
            lod_manual: 640.0,
            selectable_kinds: Puzzle5dSelectableKinds { parts: false, grips: true, fasteners: true },
            grip_show: crate::editor::puzzle5d::PUZZLE5D_GRIP_SHOW_ALWAYS.into(),
            grip_direction: crate::editor::puzzle5d::PUZZLE5D_GRIP_DIRECTION_INWARDS.into(),
            transform_move: false,
            transform_rotate: true,
            voxel_dims: [7, 2, 64],
        };
        assert_ne!(world, Puzzle5dWorldWindowConfig::default(), "the world fixture must differ from the default");
        let decoded = <Puzzle5dWorldWindowConfig as store::ArtifactPack>::decode_pack(&store::ArtifactPack::encode_pack(&world)).expect("world pack decodes");
        assert_eq!(decoded, world, "one world pack round trip must preserve every persisted option");
        let parsed = <Puzzle5dWorldWindowConfig as store::ArtifactDsl>::parse_dsl(&store::ArtifactDsl::print_dsl(&world)).expect("world text parses");
        assert_eq!(parsed, world, "one world text round trip must preserve every persisted option");
    }
