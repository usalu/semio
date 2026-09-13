use super::*;

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
        let spr = dsl::json::to_json_string(&shared);
        let oracle: serde_json::Value = serde_json::from_str(&spr).expect("serde_json oracle accepts the neutral config");
        assert_eq!(oracle.as_object().map(serde_json::Map::len), Some(3));
        let pack = store::ArtifactPack::encode_pack(&shared);
        for forbidden in ["camera2d", "camera3d", "engagementInput", "brushCandidateIndex", "fillCount", "sun"] {
            assert!(!spr.contains(forbidden));
            assert!(!pack.windows(forbidden.len()).any(|bytes| bytes == forbidden.as_bytes()));
        }
    }

    #[test]
    fn transient_string_retirement_reaches_terminal_empty_with_tiny_grants() {
        let transient = Puzzle5dWindowTransient { engagement_input: "five-dimensional-input".repeat(512), brush_candidate_index: 3 };
        let mutation = Puzzle5dWindowTransientMutation::Snapshot { transient };
        assert!(puzzle5d_window_transient_preflight(&mutation).expect("large Puzzle 5D transient admission").is_admissible());
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
                store::SnapshotRetirementStep::Blocked => panic!("owned Puzzle 5D transient retirement blocked"),
            }
        }
        panic!("owned Puzzle 5D transient retirement exceeded its bounded cursor turns");
    }

    fn retire_returned_puzzle5d_transient(transient: Puzzle5dWindowTransient) {
        let mut retirement = store::retirement::owned_retirement(Puzzle5dWindowTransientMutation::Snapshot { transient });
        for _ in 0..4_096 {
            match retirement.close_step(1, 1).expect("returned Puzzle 5D owner retirement") {
                store::SnapshotRetirementStep::Complete => {
                    assert!(retirement.terminal_is_empty());
                    return;
                }
                store::SnapshotRetirementStep::Pending { released_items, released_bytes } => {
                assert!(released_items <= 1);
                assert!(released_bytes <= 1);
            }
                store::SnapshotRetirementStep::Blocked => panic!("returned Puzzle 5D owner retirement blocked"),
            }
        }
        panic!("returned Puzzle 5D owner retirement exceeded its bounded cursor turns");
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
            fill_count: 17,
            lod_mode: "manual".into(),
            suggestion_offset: 4.75,
            grid_snap_enabled: false,
            grid_factor: 0.25,
        };
        assert_ne!(board, Puzzle5dBoardWindowConfig::default(), "the board fixture must differ from the default");
        let decoded = <Puzzle5dBoardWindowConfig as store::ArtifactPack>::decode_pack(&store::ArtifactPack::encode_pack(&board)).expect("board pack decodes");
        assert_eq!(decoded, board, "one board pack round trip must preserve every persisted option");
        let parsed = <Puzzle5dBoardWindowConfig as store::ArtifactDsl>::parse_dsl(&store::ArtifactDsl::print_dsl(&board)).expect("board text parses");
        assert_eq!(parsed, board, "one board text round trip must preserve every persisted option");

        let world = Puzzle5dWorldWindowConfig {
            camera3d: Puzzle5dCamera3d { position: [1.0, 2.0, 3.0], target: [4.0, 5.0, 6.0], zoom: 2.5 },
            sun: WorldSunConfig { enabled: true, azimuth: 12.5, elevation: 33.0, intensity: 0.5, color: "#102030".into() },
        };
        assert_ne!(world, Puzzle5dWorldWindowConfig::default(), "the world fixture must differ from the default");
        let decoded = <Puzzle5dWorldWindowConfig as store::ArtifactPack>::decode_pack(&store::ArtifactPack::encode_pack(&world)).expect("world pack decodes");
        assert_eq!(decoded, world, "one world pack round trip must preserve every persisted option");
        let parsed = <Puzzle5dWorldWindowConfig as store::ArtifactDsl>::parse_dsl(&store::ArtifactDsl::print_dsl(&world)).expect("world text parses");
        assert_eq!(parsed, world, "one world text round trip must preserve every persisted option");
    }
