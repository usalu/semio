/// 🗒️ Peers, clipboard and history panel: interaction view peers, the parent document lane rule, clipboard kind actions and the history panel filter, clipping and windowing laws.

    #[semio_framework_async_macros::async_test]
    async fn interaction_view_peers_selecting_returns_actor_and_color() {
        let mut peers = PeerPresenceRoot::empty();
        let mark_interaction = |selected: &[&str], hovered: &[&str]| {
            Some(PresenceInteraction {
                app_id: "draw".to_string(),
                domains: vec![PresenceDomain { domain: "items".to_string(), granularity: "item".to_string(), selected: selected.iter().map(|id| id.to_string()).collect(), hovered: hovered.iter().map(|id| id.to_string()).collect() }],
            })
        };
        peers.insert("user:zed#s1".to_string(), PeerPresence { color: Some(7), surface: None, interaction: mark_interaction(&["item-1"], &[]) }).expect("zed peer");
        peers.insert("user:alice#s1".to_string(), PeerPresence { color: Some(2), surface: None, interaction: mark_interaction(&["item-1"], &[]) }).expect("alice peer");
        peers.insert("user:bob#s1".to_string(), PeerPresence { color: Some(4), surface: None, interaction: mark_interaction(&[], &["item-1"]) }).expect("bob peer");

        let state = InteractionState::default();
        let hover = InteractionHoverState::new();
        let view = InteractionView { state: &state, hover: &hover, peers: &peers, draft_references: &std::collections::BTreeMap::new() };

        let selecting = view.peers_selecting("items", "item-1");
        assert_eq!(selecting.len(), 2, "only alice and zed selected item-1");
        assert_eq!(selecting[0].actor, "user:alice#s1", "sorted by actor");
        assert_eq!(selecting[0].color, Some(2));
        assert_eq!(selecting[1].actor, "user:zed#s1");
        assert_eq!(selecting[1].color, Some(7));
        assert!(view.peers_selecting("items", "item-2").is_empty(), "no peer selected item-2");

        let hovering = view.peers_hovering("items", "item-1");
        assert_eq!(hovering.len(), 1);
        assert_eq!(hovering[0].actor, "user:bob#s1");
        assert_eq!(hovering[0].color, Some(4));
    }

    impl store::os_store::document_store_owners::MemberStoreProducer<TestSnapshot,TestMutation> for semio_framework_plugin_testing_fixtures::test_app_mutation_fixture::TestMemberStoreProducer {
        /// 🚪️ The fixture child is a real openable member, not an un-openable one. While this was
        /// `UnsupportedMemberSnapshotOpen` every composed replacement that carried a `TestMembers`
        /// candidate was refused at its first member-open step with `MemberOpenDiagnostic::Decode`
        /// (`UnsupportedMemberSnapshotOpen::step` has no other answer), so the three
        /// `retained_composed_replacement_*` laws could never reach `ValidatingClosure` or
        /// `CandidateReady`. `ProducedMemberSnapshotOpen` is the framework's own pack decoder and
        /// `TestSnapshot` is an `ArtifactPack`, which is exactly what a real member declares.
        type SnapshotOpen = store::ProducedMemberSnapshotOpen<TestSnapshot,store::ArtifactPackSnapshotDecoder<TestSnapshot>>;

        fn member_store_owners_birth_demand(&self) -> Result<semio_framework_value::retained_clone::RetainedCloneBirthDemand, semio_framework_value::ValueError> {
            let base = store::os_store::document_store_owners::DocumentStoreOwners::<TestSnapshot, TestMutation>::source_birth_bytes::<semio_framework_value::factory::SharedValueRetirementFactory<TestSnapshot>, semio_framework_value::factory::OwnedValueRetirementFactory<TestSnapshot>, semio_framework_value::factory::OwnedValueRetirementFactory<TestMutation>, store::ArtifactStoreCursorDisposer<TestSnapshot, TestMutation>>()?;
            let preparation = store::operation_wire_preparation_factory_source_birth_demand::<TestSnapshot, TestMutation>(semio_framework_value::retained_clone::RetainedCloneBirthDemand { capacity_bytes: store::paged_one_item_factory_birth_bytes::<TestSnapshot, TestMutation, store::MutationApplyEdit<TestSnapshot, TestMutation>>(), depth: 1 })?;
            let capacity_bytes = base.checked_add(preparation.capacity_bytes).ok_or_else(|| semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::OwnershipLimit, "member original source tree capacity overflow"))?;
            Ok(semio_framework_value::retained_clone::RetainedCloneBirthDemand { capacity_bytes, depth: preparation.depth.max(1) })
        }

        fn member_store_owners(&self,grant: semio_framework_value::retained_clone::RetainedCloneGrant) -> Result<(store::os_store::document_store_owners::DocumentStoreOwners<TestSnapshot, TestMutation>, semio_framework_value::retained_clone::RetainedCloneProgress), store::os_store::document_store_owners::DocumentStoreOwnersAdmissionError<TestSnapshot, TestMutation>> {
            let preparation = store::operation_wire_preparation_factory_source_birth_demand::<TestSnapshot, TestMutation>(semio_framework_value::retained_clone::RetainedCloneBirthDemand { capacity_bytes: store::paged_one_item_factory_birth_bytes::<TestSnapshot, TestMutation, store::MutationApplyEdit<TestSnapshot, TestMutation>>(), depth: 1 }).map_err(|error| store::os_store::document_store_owners::DocumentStoreOwnersAdmissionError { error, owners: None, progress: Default::default() })?;
            store::os_store::document_store_owners::DocumentStoreOwners::admit_source_constructor_with_one_item_preparation(grant, preparation, || (
                semio_framework_value::factory::SharedValueRetirementFactory::<TestSnapshot>::default(),
                semio_framework_value::factory::OwnedValueRetirementFactory::<TestSnapshot>::default(),
                semio_framework_value::factory::OwnedValueRetirementFactory::<TestMutation>::default(),
                store::ArtifactStoreCursorDisposer::<TestSnapshot, TestMutation>::new(),
                test_document_preparation_factory(),
            ))
        }
    }

    /// ⏪️ A row follows its parent document lane, else its children (design §20.13, L4: a config-lane edit is never part
    /// of a row). 💠️lowpoly `addPrimitive` and 🎥️shooting `addShot` once read applied for ever after an undo because a
    /// config lane answered for them (ticket 26/09/18 S11 §3.4b, S12 §4); no lane but the document and its children can.
    #[test]
    fn a_row_follows_its_parent_document_lane_and_else_its_children() {
        use semio_framework_plugin_command::history_row_applied as applied;
        assert!(!applied(true, false, true), "a row that published a parent edit is UNDONE once that edit is retracted, however live its child edit is");
        assert!(!applied(true, false, false), "a parent-lane row with nothing applied anywhere is undone");
        assert!(applied(true, true, false), "a parent-lane row whose document edit is applied is applied");
        assert!(applied(true, true, true), "a multi-lane row is applied while its parent edit is");
        assert!(applied(false, false, true), "a CHILD-only row answers with its children — 🌊️flow's addWidget and 🎬️sequence's addStep");
        assert!(!applied(false, false, false), "a row applied nowhere is not applied");
    }

    #[semio_framework_async_macros::async_test]
    async fn copy_cut_paste_are_registered_as_clipboard_kind_actions() {
        let definition = synthetic_play_app().await.definition;
        for id in ["copy", "cut", "paste"] {
            let action = definition
                .actions
                .iter()
                .chain(definition.window_kinds.iter().flat_map(|window| window.actions.iter()))
                .find(|a| a.id == id)
                .unwrap_or_else(|| panic!("{id} must be auto-injected into every app's manifest"));
            assert_eq!(action.kind, ActionKind::Clipboard);
        }
    }

    /// 📌️ Six since the explicit check-in row landed (`#s-checkin`, React's
    /// `framework.history.checkin`) between Commit Checkpoint and Create Alternative.
    #[semio_framework_async_macros::async_test]
    async fn ui_history_panel_filters_rows_and_gates_the_backwards_action() {
        let history = HistoryView {
            columns: Vec::new(),
            can_undo: true,
            can_redo: false,
            active_alternative_id: None, alternatives: Vec::new(),
            current_checkpoint_id: None,
            commands: vec![
                CommandView {
                    seq: 1,
                    action_id: "increment".into(),
                    label: LocalizedLabel::native("Increment", "Erhöhen"),
                    kind: ActionKind::Mutation,
                    timestamp: "0".into(),
                    edit_id: Some("e1".into()),
                    child_edit_ids: Vec::new(),
                    transition_id: None,
                    author: None,
                    op_lines: vec!["set-count value=1".into()],
                    op_count: 1,
                    applied: true,
                    revertible: true,
                    count: 1,
                    inverse: None,
                    transaction: None,
                    mutations: Vec::new(),
                },
                CommandView {
                    seq: 2,
                    action_id: "undo".into(),
                    label: LocalizedLabel::native("Undo", "Rückgängig"),
                    kind: ActionKind::History,
                    timestamp: "1".into(),
                    edit_id: None,
                    child_edit_ids: Vec::new(),
                    transition_id: None,
                    author: None,
                    op_lines: Vec::new(),
                    op_count: 0,
                    applied: false,
                    revertible: false,
                    count: 1,
                    inverse: None,
                    transaction: None,
                    mutations: Vec::new(),
                },
            ],
            command_filter: HistoryCommandFilter::All,
        };
        let all_panel = ui_history_panel(&history, None, None, &Default::default(), "ctrl", Locale::En, false, false, &ViewModel::new(semio_framework_ui_locale::Locale::En, semio_framework_ui_locale::Terminology::Native)).await.expect("bounded history panel");
        assert_eq!(all_panel.children.len(), 2, "Actions + Commands sections");
        let Component::TreeSection(actions_props) = &all_panel.children[0].component else { panic!("expected a TreeSection") };
        assert_eq!(actions_props.label.as_ref().map(|label| label.0.as_str()), Some("Actions"));
        assert_eq!(all_panel.children[0].children.len(), 6, "undo/redo/commit/checkin/alternative/filter");
        assert_eq!(all_panel.children[0].children[3].key.as_str(), "framework.history.checkin", "the check-in row follows Commit Checkpoint");
        let viewer_panel = ui_history_panel(&history, None, None, &Default::default(), "ctrl", Locale::En, true, false, &ViewModel::new(semio_framework_ui_locale::Locale::En, semio_framework_ui_locale::Terminology::Native)).await.expect("viewer history panel");
        assert_eq!(viewer_panel.children[0].children.len(), 5, "a viewer never gets the check-in row at all \u{2014} React's `canCheckIn` gate removes it rather than disabling it");
        assert!(all_panel.children[0].children.iter().all(|item| !item.children.is_empty()), "Actions rows carry their control as a child node");
        let Component::TreeSection(commands_props) = &all_panel.children[1].component else { panic!("expected a TreeSection") };
        assert_eq!(commands_props.label.as_ref().map(|label| label.0.as_str()), Some("Commands"));
        assert_eq!(all_panel.children[1].children.len(), 2);
        let Component::TreeItem(revertible_props) = &all_panel.children[1].children[0].component else { panic!("expected a TreeItem") };
        assert!(!revertible_props.row_actions.is_empty(), "the revertible entry must offer inverse");
        let Component::TreeItem(non_revertible_props) = &all_panel.children[1].children[1].component else { panic!("expected a TreeItem") };
        assert!(non_revertible_props.row_actions.is_empty(), "the non-revertible entry must not offer inverse");

        let only_ops = HistoryView { command_filter: HistoryCommandFilter::OnlyMutations, ..history.clone() };
        let ops_panel = ui_history_panel(&only_ops, None, None, &Default::default(), "ctrl", Locale::En, false, false, &ViewModel::new(semio_framework_ui_locale::Locale::En, semio_framework_ui_locale::Terminology::Native)).await.expect("bounded history panel");
        assert_eq!(ops_panel.children[1].children.len(), 1);
        assert_eq!(ops_panel.children[1].children[0].key.as_str(), "framework.history.entry.1");

        let without_ops = HistoryView { command_filter: HistoryCommandFilter::WithoutMutations, ..history };
        let no_ops_panel = ui_history_panel(&without_ops, None, None, &Default::default(), "ctrl", Locale::En, false, false, &ViewModel::new(semio_framework_ui_locale::Locale::En, semio_framework_ui_locale::Terminology::Native)).await.expect("bounded history panel");
        assert_eq!(no_ops_panel.children[1].children.len(), 1);
        assert_eq!(no_ops_panel.children[1].children[0].key.as_str(), "framework.history.entry.2");
    }

    #[semio_framework_async_macros::async_test]
    async fn ui_history_panel_clips_an_oversized_operation_description() {
        let history = HistoryView {
            columns: Vec::new(),
            can_undo: true,
            can_redo: false,
            active_alternative_id: None, alternatives: Vec::new(),
            current_checkpoint_id: None,
            commands: vec![CommandView {
                seq: 1,
                action_id: "fill".into(),
                label: LocalizedLabel::native("Fill", "Füllen"),
                kind: ActionKind::Mutation,
                timestamp: "0".into(),
                edit_id: Some("e1".into()),
                child_edit_ids: Vec::new(),
                transition_id: None,
                author: None,
                op_lines: vec![format!("register-mesh vertices=[{}]", "1.0 ".repeat(1_024))],
                op_count: 1,
                applied: true,
                revertible: true,
                count: 1,
                inverse: None,
                transaction: None,
                mutations: Vec::new(),
            }],
            command_filter: HistoryCommandFilter::All,
        };
        let panel = ui_history_panel(&history, None, None, &Default::default(), "ctrl", Locale::En, false, false, &ViewModel::new(semio_framework_ui_locale::Locale::En, semio_framework_ui_locale::Terminology::Native)).await.expect("an oversized operation line must not fail admission");
        let Component::TreeItem(props) = &panel.children[1].children[0].component else { panic!("expected a TreeItem") };
        let description = props.description.as_ref().expect("clipped description").as_str();
        assert!(description.starts_with("register-mesh vertices=[1.0 "));
        assert!(description.ends_with(UI_TEXT_CLIP_MARK));
    }

    /// 🏷️ A command label is authored copy that can carry the whole argument form of an edit; a folded
    /// row appends ` xN` on top. Both must clip on a char boundary instead of failing the admission of
    /// every panel in the refresh (seen 2026-09-10: `duplicateSelection` on the puzzle 3d Perspective
    /// window failed `refreshUi` with `history-panel.command-label`).
    #[semio_framework_async_macros::async_test]
    async fn ui_history_panel_clips_an_oversized_command_label_and_its_folded_count() {
        let long_label = format!("Duplicate {}", "seed-left-001 ".repeat(64));
        let entry = |seq: u64, count: u32| CommandView {
            seq,
            action_id: "duplicateSelection".into(),
            label: LocalizedLabel::data(long_label.clone()),
            kind: ActionKind::Mutation,
            timestamp: "0".into(),
            edit_id: Some("e1".into()),
            child_edit_ids: Vec::new(),
            transition_id: None,
            author: None,
            op_lines: Vec::new(),
            op_count: 0,
            applied: true,
            revertible: true,
            count,
            inverse: None,
            transaction: None,
            mutations: Vec::new(),
        };
        let history = HistoryView { columns: Vec::new(), can_undo: true, can_redo: false, active_alternative_id: None, alternatives: Vec::new(), current_checkpoint_id: None, commands: vec![entry(1, 1), entry(2, 3)], command_filter: HistoryCommandFilter::All };
        let panel = ui_history_panel(&history, None, None, &Default::default(), "ctrl", Locale::En, false, false, &ViewModel::new(semio_framework_ui_locale::Locale::En, semio_framework_ui_locale::Terminology::Native)).await.expect("an oversized command label must not fail admission");
        for (index, expected_tail) in [(0, UI_TEXT_CLIP_MARK), (1, UI_TEXT_CLIP_MARK)] {
            let Component::TreeItem(props) = &panel.children[1].children[index].component else { panic!("expected a TreeItem") };
            let label = props.label.0.as_str();
            assert!(label.starts_with("Duplicate seed-left-001 "), "{label}");
            assert!(label.ends_with(expected_tail), "{label}");
            assert!(label.len() <= UI_TEXT_MAX_BYTES, "{label}");
        }
    }

    fn history_window_fixture() -> Value {
        serde_json::from_str(include_str!("../../🧫️fixtures/history-panel-command-window/🔣️.json")).unwrap()
    }

    /// 🪙️ A revertible row materialises a `UiValue::Map`; a non-revertible one costs zero argument
    /// arena. The window laws below use the zero-cost shape so their row counts are the WINDOW's
    /// arithmetic alone, never the process-global arena another test in this binary is holding.
    fn history_window_log(rows: usize, revertible: bool) -> HistoryView {
        let entry = move |seq: u64| CommandView {
            seq,
            action_id: "translateSelection".into(),
            label: LocalizedLabel::data(format!("Move {seq}")),
            kind: ActionKind::Mutation,
            timestamp: "0".into(),
            edit_id: Some(format!("edit-{seq}")),
            child_edit_ids: Vec::new(),
            transition_id: None,
            author: None,
            op_lines: Vec::new(),
            op_count: 0,
            applied: true,
            revertible,
            count: 1,
            inverse: None,
            transaction: None,
            mutations: Vec::new(),
        };
        HistoryView { columns: Vec::new(), can_undo: true, can_redo: false, active_alternative_id: None, alternatives: Vec::new(), current_checkpoint_id: None, commands: (1..=rows as u64).map(entry).collect(), command_filter: HistoryCommandFilter::All }
    }

    /// 🧾️ The body as the host reads it — `BuiltChildren` is retained page transport and refuses a
    /// direct serde walk, so the tree is projected node by node.
    fn history_body_json(node: &BuiltNode) -> Value {
        json!({
            "key": node.key.as_str(),
            "component": serde_json::to_value(&node.component).expect("component JSON"),
            "children": node.children.iter().map(history_body_json).collect::<Vec<_>>(),
        })
    }

    fn history_commands_window(panel: &BuiltNode) -> Option<TreeWindow> {
        let Component::TreeSection(props) = &panel.children[1].component else { panic!("expected the Commands TreeSection") };
        props.window
    }

    /// 🪟️ ARTIFACT-TREE-VIRTUALISED-STREAMING: the Commands section is a window, not a page. Its rows
    /// are the live entries themselves (no `UI_BUILT_CHILDREN_MAX`-ary page columns between the section
    /// and its rows), it publishes the FULL filtered row count as `TreeWindow::total` however few rows
    /// it materialises, a cold paint draws `TREE_WINDOW_DEFAULT_ROWS` clamped by the built-children
    /// ceiling, a host request draws exactly its slice keyed by the raw entry id, and no `+N`
    /// continuation row exists at any point.
    #[semio_framework_async_macros::async_test]
    async fn ui_history_panel_windows_command_rows_over_the_live_count() {
        let fixture = history_window_fixture();
        let rows = fixture["commandRowCount"].as_u64().unwrap() as usize;
        let default_rows = fixture["defaultWindowRows"].as_u64().unwrap() as usize;
        assert_eq!(default_rows, TREE_WINDOW_DEFAULT_ROWS as usize, "fixture must pin the live first-paint budget");
        let prefix = fixture["entryKeyPrefix"].as_str().unwrap();
        let history = history_window_log(rows, false);

        let cold = ui_history_panel(&history, None, None, &Default::default(), "ctrl", Locale::En, false, false, &ViewModel::new(semio_framework_ui_locale::Locale::En, semio_framework_ui_locale::Terminology::Native)).await.expect("a log of any length must assemble");
        assert_eq!(serde_json::to_value(&cold.layout).expect("the authored history root layout"), fixture["rootLayout"]);
        assert_eq!(cold.children[1].children.len(), default_rows.min(UI_BUILT_CHILDREN_MAX), "a cold paint materialises one viewport, clamped by the built-children ceiling");
        assert!(cold.children[1].children.iter().all(|row| row.key.as_str().starts_with(prefix)), "rows are the entries themselves, never page columns");
        assert_eq!(history_commands_window(&cold), Some(TreeWindow { row_extent: Default::default(), total: rows as u32, offset: 0 }), "the host sees the whole extent");

        let offset = fixture["requestOffset"].as_u64().unwrap() as u32;
        let requested = fixture["requestRows"].as_u64().unwrap() as u32;
        assert!(requested as usize <= UI_BUILT_CHILDREN_MAX, "fixture slice must fit one built-children page");
        let view = ViewModel {
            tree_windows: vec![TreeWindowRequest {
                body_key: fixture["bodyKey"].as_str().unwrap().to_string(),
                node_key: fixture["nodeKey"].as_str().unwrap().to_string(),
                open: Some(true),
                offset,
                rows: requested,
            }],
            ..ViewModel::new(semio_framework_ui_locale::Locale::En, semio_framework_ui_locale::Terminology::Native)
        };
        let scrolled = ui_history_panel(&history, None, None, &Default::default(), "ctrl", Locale::En, false, false, &view).await.expect("a scrolled window must assemble");
        assert_eq!(scrolled.children[1].children.len(), requested as usize);
        assert_eq!(scrolled.children[1].children[0].key.as_str(), format!("{prefix}{}", offset + 1), "the slice starts where the host scrolled to");
        assert_eq!(history_commands_window(&scrolled), Some(TreeWindow { row_extent: Default::default(), total: rows as u32, offset }));

        for panel in [&cold, &scrolled] {
            let json = history_body_json(panel).to_string();
            assert!(!json.contains(".more"), "no continuation row key survives: {json}");
            assert!(!json.contains("\"+"), "no `+N` label survives: {json}");
        }
    }

    /// 🪙️ Every revertible row the window materialises keeps its inline revert, and the window can
    /// never out-spend the process-wide argument arena: a slice is capped by `UI_BUILT_CHILDREN_MAX`,
    /// which the contract prices at `UI_VALUE_PAGE_ROWS` interactive rows. A refusal under live arena
    /// pressure shortens the window — it never faults the render and never drops a row's revert.
    #[semio_framework_async_macros::async_test]
    async fn ui_history_panel_keeps_every_materialised_revert_inside_the_arena_page() {
        let entries = 20usize;
        let history = history_window_log(entries, true);
        let panel = ui_history_panel(&history, None, None, &Default::default(), "ctrl", Locale::En, false, false, &ViewModel::new(semio_framework_ui_locale::Locale::En, semio_framework_ui_locale::Terminology::Native)).await.expect("revertible entries must assemble without an alias refusal");
        fn census(node: &BuiltNode, rows: &mut usize, actions: &mut usize) {
            if node.key.as_str().starts_with("framework.history.entry.") {
                *rows += 1;
                if let Component::TreeItem(props) = &node.component {
                    *actions += props.row_actions.len();
                }
            }
            for child in node.children.iter() {
                census(child, rows, actions);
            }
        }
        let (mut rows, mut actions) = (0usize, 0usize);
        census(&panel, &mut rows, &mut actions);
        assert!(rows > 0 && rows <= entries, "the window never exceeds the live count: {rows}");
        assert_eq!(actions, rows, "every materialised row keeps its inline revert");
        assert!(actions <= UI_VALUE_PAGE_ROWS, "a window can never out-spend the arena page: {actions} > {UI_VALUE_PAGE_ROWS}");
        assert_eq!(history_commands_window(&panel), Some(TreeWindow { row_extent: Default::default(), total: entries as u32, offset: 0 }));
    }

    /// 🙈️ The row predicate: interactions never, views only with an edit or inverse, everything else always.
    #[test]
    fn history_rows_exclude_interactions_and_pure_views() {
        for (kind, edited, inverse, expected) in [
            (ActionKind::Interaction, true, true, false),
            (ActionKind::View, false, false, false),
            (ActionKind::View, true, false, true),
            (ActionKind::View, false, true, true),
            (ActionKind::Mutation, false, false, true),
            (ActionKind::History, false, false, true),
            (ActionKind::Clipboard, false, false, true),
            (ActionKind::Shell, false, false, true),
        ] {
            assert_eq!(history_row_is_recorded(kind, edited, inverse), expected, "{kind:?} edited={edited} inverse={inverse}");
        }
    }
