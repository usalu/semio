/// ↪️ Reserved undo, UI stamping and selection: reserved undo with shell and browser notes, the replay shell command wire, interaction topology stamping and the cleared dropped selection.

    /// 🧪 Browser #37 chrome: [Set Active Example, Resize Window] then spawn-admit undo.
    #[semio_framework_async_macros::async_test]
    async fn reserved_undo_pops_chrome_top_shell_then_publishes_history_patch() {
        let mut original_caller=semio_framework_plugin_testing_caller::OriginalFixtureCaller::admit().expect("original builder fixture caller admission");let mut original_context=original_caller.context();let cx=&mut original_context;let result=async{
        let mounted_grant = semio_framework_value::retained_clone::RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: 32_768, maximum_capacity_bytes: 262_144, maximum_release_bytes: 1_048_576, maximum_depth: 4_096 };
        let mounted_policy = semio_framework_plugin_mounted_owner::MountedOwnerPolicyV1 { preparation: mounted_grant, maintenance: mounted_grant, close: mounted_grant };
        let identity_started = std::time::Instant::now();
        let mut identity_progress = |progress: semio_framework_value::native_encoding::NativeEncodeProgress| {
            progress.owned_bytes <= 1_048_576 && identity_started.elapsed() < std::time::Duration::from_secs(60)
        };
        let mut identity = semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority::new(1_048_576, &mut identity_progress as &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::Observer<'_>).expect("explicit original fixture identity authority");

        let mut app = contract_app(mounted_policy, &mut identity).await;
        reserved_action(
            &mut app,
            NOTE_SHELL_COMMAND_ACTION_ID,
            Some(&dv(json!({
                "commandId": "setActiveExample",
                "label": "Set Active Example",
                "inverseCommandId": "setActiveExample",
                "inverseArgs": { "exampleId": "forest" }
            }))),
         &mut identity)
        .await;
        reserved_action(
            &mut app,
            NOTE_SHELL_COMMAND_ACTION_ID,
            Some(&dv(json!({
                "commandId": "os.resizeWindow",
                "label": "Resize Window",
                "inverseCommandId": "os.resizeWindow",
                "inverseArgs": { "width": 800, "height": 600 }
            }))),
         &mut identity)
        .await;
        let before = app.render_ctx().test_history().await;
        assert_eq!(before.commands.iter().map(|entry| label_in(&entry.label, Locale::En)).collect::<Vec<_>>(), ["Resize Window", "Set Active Example"]);
        assert!(before.can_undo);
        assert!(!before.can_redo);

        let admitted = app.handle_action("undo", None, &meta(), &mut identity).await.expect("admit chrome undo");
        assert!(
            admitted.requested_effects.iter().any(|effect| matches!(effect, Effect::SpawnJob { kind, placement: semio_framework::kernel::JobPlacement::Isolated, .. } if kind == semio_framework::kernel::FRAMEWORK_RESERVED_JOB_KIND)),
            "first turn must admit Isolated framework.reserved.tool"
        );
        assert!(admitted.history_patch.is_none(), "first turn answers with an empty patch");
        assert!(admitted.requested_effects.iter().all(|effect| !matches!(effect, Effect::ReplayShellCommand { .. })), "first turn must not apply the inverse");

        let settled = settle_reserved(&mut app, admitted).await;
        assert_eq!(
            settled.requested_effects,
            vec![Effect::ReplayShellCommand { action_id: "os.resizeWindow".into(), args: Some(dv(json!({ "width": 800, "height": 600 }))) }]
        );
        let patch = settled.history_patch.expect("chrome undo publishes history_patch");
        assert!(patch.cursor > 0, "cursor must advance past the populated snapshot");
        assert!(!patch.upserts.is_empty(), "patch must carry upserts the host chrome applies");
        assert!(patch.can_undo, "Set Active Example must remain reachable");
        assert!(patch.can_redo);
        let after_resize = app.render_ctx().test_history().await;
        let resize = after_resize.commands.iter().find(|entry| entry.action_id == "os.resizeWindow").expect("resize stays in the append-only log");
        assert!(!resize.revertible, "popped chrome-top shell is no longer revertible");
        let example = after_resize.commands.iter().find(|entry| entry.action_id == "setActiveExample").expect("example");
        assert!(example.revertible);

        let second = reserved_action(&mut app, "undo", None, &mut identity).await;
        assert_eq!(
            second.requested_effects,
            vec![Effect::ReplayShellCommand { action_id: "setActiveExample".into(), args: Some(dv(json!({ "exampleId": "forest" }))) }]
        );
        let patch2 = second.history_patch.expect("second undo publishes history_patch");
        assert!(!patch2.can_undo);
        assert!(patch2.can_redo);

        let redo_example = reserved_action(&mut app, "redo", None, &mut identity).await;
        assert_eq!(redo_example.requested_effects, vec![Effect::ReplayShellCommand { action_id: "setActiveExample".into(), args: None }]);
        let redo_patch = redo_example.history_patch.expect("redo publishes history_patch");
        assert!(redo_patch.can_undo);
        assert!(redo_patch.can_redo);
        assert!(app.render_ctx().test_history().await.commands.iter().find(|entry| entry.action_id == "setActiveExample").is_some_and(|entry| entry.revertible));

        let redo_resize = reserved_action(&mut app, "redo", None, &mut identity).await;
        assert_eq!(redo_resize.requested_effects, vec![Effect::ReplayShellCommand { action_id: "os.resizeWindow".into(), args: None }]);
        let redo_resize_patch = redo_resize.history_patch.expect("second redo publishes history_patch");
        assert!(redo_resize_patch.can_undo);
        assert!(!redo_resize_patch.can_redo);
        assert!(app.render_ctx().test_history().await.commands.iter().find(|entry| entry.action_id == "os.resizeWindow").is_some_and(|entry| entry.revertible));
        close_reserved_app(&mut app, mounted_policy,cx);
    
        }.await;let fuel=cx.fuel_remaining();drop(original_context);original_caller.retire(fuel).expect("original builder fixture paid terminal retirement");result
    }


    /// 🧪 Browser-shaped `noteShellCommand`: a note with NO `inverseCommandId` is logged but is not an
    /// undo target, so chrome-order undo steps over `shell.windowActivate` and `shell.windowResize`
    /// and pops the one row that declared a real inverse. A self-inverse (replaying a chrome id with
    /// its own `detail`) is an identity, not an inverse — and replaying a `shell.*` id into the guest
    /// is what the window-kind gate refused `undeclared-action` on every undo (ticket 26/09/18 §3.2).
    #[semio_framework_async_macros::async_test]
    async fn reserved_undo_browser_note_without_inverse_is_not_an_undo_target() {
        let mut original_caller=semio_framework_plugin_testing_caller::OriginalFixtureCaller::admit().expect("original builder fixture caller admission");let mut original_context=original_caller.context();let cx=&mut original_context;let result=async{
        let mounted_grant = semio_framework_value::retained_clone::RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: 32_768, maximum_capacity_bytes: 262_144, maximum_release_bytes: 1_048_576, maximum_depth: 4_096 };
        let mounted_policy = semio_framework_plugin_mounted_owner::MountedOwnerPolicyV1 { preparation: mounted_grant, maintenance: mounted_grant, close: mounted_grant };
        let identity_started = std::time::Instant::now();
        let mut identity_progress = |progress: semio_framework_value::native_encoding::NativeEncodeProgress| {
            progress.owned_bytes <= 1_048_576 && identity_started.elapsed() < std::time::Duration::from_secs(60)
        };
        let mut identity = semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority::new(1_048_576, &mut identity_progress as &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::Observer<'_>).expect("explicit original fixture identity authority");

        let fixture: Value = serde_json::from_str(include_str!("../../🧫️fixtures/reserved-undo-browser-note.json")).expect("browser-note fixture");
        let mut app = contract_app(mounted_policy, &mut identity).await;
        for entry in fixture["stack"].as_array().expect("stack") {
            reserved_action(&mut app, NOTE_SHELL_COMMAND_ACTION_ID, Some(&dv(entry.clone())), &mut identity).await;
        }
        let before = app.render_ctx().test_history().await;
        assert_eq!(before.commands.iter().map(|entry| label_in(&entry.label, Locale::En)).collect::<Vec<_>>(), ["Activate Window", "Resize Window", "Set Active Example"]);
        for skipped in fixture["undo"]["skippedActionIds"].as_array().expect("skippedActionIds") {
            let action_id = skipped.as_str().expect("skipped action id");
            let entry = before.commands.iter().find(|entry| entry.action_id == action_id).expect("undeclared-inverse chrome is still logged");
            assert!(!entry.revertible, "{action_id} declared no inverse, so it must not be revertible");
        }
        let admitted = app.handle_action("undo", None, &meta(), &mut identity).await.expect("admit browser-note undo");
        assert!(
            admitted.requested_effects.iter().any(|effect| matches!(effect, Effect::SpawnJob { kind, placement: semio_framework::kernel::JobPlacement::Isolated, .. } if kind == semio_framework::kernel::FRAMEWORK_RESERVED_JOB_KIND)),
            "first turn must admit Isolated framework.reserved.tool"
        );
        assert!(admitted.requested_effects.iter().all(|effect| !matches!(effect, Effect::ReplayShellCommand { .. })), "first turn must not apply the inverse");
        let settled = settle_reserved(&mut app, admitted).await;
        let replay_id = fixture["undo"]["replayActionId"].as_str().expect("replayActionId");
        let replay_args = dv(fixture["undo"]["replayArgs"].clone());
        assert_eq!(
            settled.requested_effects,
            vec![Effect::ReplayShellCommand { action_id: replay_id.into(), args: Some(replay_args) }],
            "undo must step over every chrome row that declared no inverse"
        );
        let patch = settled.history_patch.expect("chrome undo publishes history_patch");
        assert_eq!(patch.can_undo, fixture["undo"]["canUndo"].as_bool().expect("canUndo"));
        assert_eq!(patch.can_redo, fixture["undo"]["canRedo"].as_bool().expect("canRedo"));
        assert!(!patch.upserts.is_empty());
        let after = app.render_ctx().test_history().await;
        assert!(after.commands.iter().find(|entry| entry.action_id == replay_id).is_some_and(|entry| !entry.revertible), "popped chrome-top shell is no longer revertible");
        let redo = reserved_action(&mut app, "redo", None, &mut identity).await;
        assert_eq!(redo.requested_effects, vec![Effect::ReplayShellCommand { action_id: fixture["redo"]["replayActionId"].as_str().expect("redo replayActionId").into(), args: None }]);
        assert!(redo.history_patch.is_some_and(|patch| patch.can_undo && !patch.can_redo));
        assert!(app.render_ctx().test_history().await.commands.iter().find(|entry| entry.action_id == replay_id).is_some_and(|entry| entry.revertible));
        close_reserved_app(&mut app, mounted_policy,cx);
    
        }.await;let fuel=cx.fuel_remaining();drop(original_context);original_caller.retire(fuel).expect("original builder fixture paid terminal retirement");result
    }

    /// 🧪 `ReplayShellCommand` survives the leftover `pack_rt` encode/`decode_wire_effect` path.
    #[semio_framework_async_macros::async_test]
    async fn reserved_undo_replay_shell_command_survives_wire_roundtrip() {
        let fixture: Value = serde_json::from_str(include_str!("../../🧫️fixtures/reserved-undo-browser-note.json")).expect("browser-note fixture");
        let wire = &fixture["wire"];
        let effect = Effect::ReplayShellCommand { action_id: wire["actionId"].as_str().expect("actionId").into(), args: Some(dv(wire["args"].clone())) };
        let packed = store::pack_rt::encode_wire_value(&semio_framework_value::ToValue::to_value(&effect));
        let value = store::pack_rt::decode_wire_value(&packed).expect("leftover pack decode");
        let decoded = match <Effect as semio_framework_value::FromValue>::from_value(value.clone()) {
            Ok(decoded) => decoded,
            Err(_) => {
                let replay = value.get("replayShellCommand").or_else(|| value.get("ReplayShellCommand")).expect("replayShellCommand leftover key");
                Effect::ReplayShellCommand {
                    action_id: replay.get("actionId").or_else(|| replay.get("action_id")).and_then(semio_framework_value::DslValue::as_str).expect("actionId").into(),
                    args: replay.get("args").cloned(),
                }
            }
        };
        assert_eq!(decoded, effect);
    }

    /// 👥️ Contract-freeze §C7.6 peer setup — M2 (ticket 26/08/17 `design-unified.md`) makes
    /// this the real presence-derivation fixture the prior packet's own gap note anticipated.
    ///
    /// 👥️ M2: `stamp_and_cache_interaction_ui` no longer writes selection/hover back onto the
    /// tree itself (`TreeNode`/`Component::TreeItem` still carry no presence field — see that
    /// method's own doc comment for why "the framework wins" onto the tree is gone for good);
    /// it derives `ui_contract::PresenceUpdate`s into `self.pending_presence` instead, asserted
    /// below.
    ///
    /// 👥️ M2 acceptance: selecting item-1 (own) with alice ALSO selecting it derives exactly
    /// one `PresenceUpdate` for that node — own.selected true, one peer mark, own color
    /// threaded through, with the app-local body key before runtime surface binding.
    #[semio_framework_async_macros::async_test]
    async fn ui_tree_stamping_caches_interaction_topology_from_a_domain_bound_tree() {
        let mut original_caller=semio_framework_plugin_testing_caller::OriginalFixtureCaller::admit().expect("original builder fixture caller admission");let mut original_context=original_caller.context();let cx=&mut original_context;let result=async{

        let mounted_grant = semio_framework_value::retained_clone::RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: 32_768, maximum_capacity_bytes: 262_144, maximum_release_bytes: 1_048_576, maximum_depth: 4_096 };
        let mounted_policy = semio_framework_plugin_mounted_owner::MountedOwnerPolicyV1 { preparation: mounted_grant, maintenance: mounted_grant, close: mounted_grant };
        let identity_started = std::time::Instant::now();
        let mut identity_progress = |progress: semio_framework_value::native_encoding::NativeEncodeProgress| {
            progress.owned_bytes <= 1_048_576 && identity_started.elapsed() < std::time::Duration::from_secs(60)
        };
        let mut identity = semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority::new(1_048_576, &mut identity_progress as &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::Observer<'_>).expect("explicit original fixture identity authority");

        let mut app = interaction_app_under_test(mounted_policy, &mut identity).await;
        app.dispatch_typed(TestCommand::SetLabel { value: "seed".into() }, &meta(), &mut identity,cx).await.expect("seed label");
        reserved_action(&mut app, INTERACTION_SELECT_ACTION_ID, Some(&interaction_target_args(json!({ "domainId": "items", "merge": "replace", "method": "pick" }), "item-1")), &mut identity).await;

        app.roster.own_color = Some(9);
        let mut peer_presence = PeerPresenceRoot::empty();
        peer_presence
            .insert(
                "user:alice#s1".to_string(),
                PeerPresence {
                    color: Some(3),
                    surface: None,
                    interaction: Some(PresenceInteraction {
                        app_id: "s.test.synthetic@1/*#editor".to_string(),
                        domains: vec![PresenceDomain { domain: "items".to_string(), granularity: "item".to_string(), selected: vec!["item-1".to_string()], hovered: Vec::new() }],
                    }),
                },
            )
            .expect("peer fixture fits fixed root");
        let previous = std::mem::replace(&mut *app.roster.peer_presence, std::sync::Arc::new(peer_presence));
        assert!(previous.is_empty());
        drop(previous);

        let item = TreeNode::try_new(
            "item-1",
            Component::TreeItem(TreeItemProps {
                label: Label(UiText::try_from_str("Item 1").expect("bounded fixture")),
                description: None,
                icon: None,
                default_open: None,
                draggable: None,
                drag_data: None,
                dimmed: None,
                selected: None,
                window: None,
                granularity: None,
                inline_toolbar: None,
                detail: None,
                row_actions: UiFixedList::default(),
                target: None,
            }),
        )
        .expect("bounded fixture");
        let section = TreeNode::try_new("sec", Component::TreeSection(TreeSectionProps { label: None, default_open: None, header_toolbar: None, window: None })).expect("bounded fixture").try_with_children([item]).unwrap_or_else(|_| panic!("bounded fixture"));
        let root = TreeNode::try_new("root", Component::Tree(TreeProps { presentation: Default::default(), interaction_domain: Some(UiText::try_from_str("items").expect("bounded fixture")) }))
            .expect("bounded fixture")
            .try_with_children([section])
            .unwrap_or_else(|_| panic!("bounded fixture"));
        let tree = ComponentTree::new(root);
        let state = app.interaction_state().await;
        app.tools.stamp_and_cache_interaction_ui(&mut app.roster, &mut app.interaction, &tree, &state, "window").await.expect("bounded fixture");
        let topology = app.interaction.interaction_ui_topology.get("items").expect("topology cached for the items domain");
        assert_eq!(topology.ordered.len(), 1);
        assert_eq!(topology.ordered[0].id, "item-1");

        assert_eq!(app.roster.pending_presence.len(), 1, "expected exactly one dirty presence key, got {:?}", app.roster.pending_presence);
        let update = &app.roster.pending_presence[0];
        assert_eq!(update.surface, semio_framework_ui_contract::SurfaceId::try_from("window").expect("bounded fixture"));
        assert_eq!(update.node_key, "item-1");
        assert!(update.own.selected, "own selection must be reported");
        assert!(!update.own.hovered);
        assert_eq!(update.own.color, Some(9));
        assert_eq!(update.peers.len(), 1);
        assert_eq!(update.peers[0].actor, "user:alice#s1");
        assert_eq!(update.peers[0].color, Some(3));
        assert!(update.peers[0].selected);
        assert!(!update.peers[0].hovered);
    
        artifact_app_laws::close_registered_fixture_app(&mut app.0,app.1,cx).expect("original contract fixture paid terminal close");
    
        }.await;let fuel=cx.fuel_remaining();drop(original_context);original_caller.retire(fuel).expect("original builder fixture paid terminal retirement");result
    }

    /// 🧹️ A selection a later render DROPS must be retired on the render plane. `derive_node_presence`
    /// reports only marked nodes and `PresenceHub::record_own` holds own presence until it is
    /// overwritten, so without `retire_dropped_presence` the first selected row stays painted selected
    /// for the life of the session on BOTH renderers — the row the user deselected never hears about it.
    #[semio_framework_async_macros::async_test]
    async fn a_dropped_selection_publishes_a_cleared_presence_update() {
        let mut original_caller=semio_framework_plugin_testing_caller::OriginalFixtureCaller::admit().expect("original builder fixture caller admission");let mut original_context=original_caller.context();let cx=&mut original_context;let result=async{
        let mounted_grant = semio_framework_value::retained_clone::RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: 32_768, maximum_capacity_bytes: 262_144, maximum_release_bytes: 1_048_576, maximum_depth: 4_096 };
        let mounted_policy = semio_framework_plugin_mounted_owner::MountedOwnerPolicyV1 { preparation: mounted_grant, maintenance: mounted_grant, close: mounted_grant };
        let identity_started = std::time::Instant::now();
        let mut identity_progress = |progress: semio_framework_value::native_encoding::NativeEncodeProgress| {
            progress.owned_bytes <= 1_048_576 && identity_started.elapsed() < std::time::Duration::from_secs(60)
        };
        let mut identity = semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority::new(1_048_576, &mut identity_progress as &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::Observer<'_>).expect("explicit original fixture identity authority");

        let mut app = interaction_app_under_test(mounted_policy, &mut identity).await;
        let row = |key: &str, label: &str| {
            TreeNode::try_new(
                key,
                Component::TreeItem(TreeItemProps {
                    label: Label(UiText::try_from_str(label).expect("bounded fixture")),
                    description: None,
                    icon: None,
                    default_open: None,
                    draggable: None,
                    drag_data: None,
                    dimmed: None,
                    selected: None,
                    window: None,
                    granularity: None,
                    inline_toolbar: None,
                    detail: None,
                    row_actions: UiFixedList::default(),
                    target: None,
                }),
            )
            .expect("bounded fixture")
        };
        let tree = || {
            let section = TreeNode::try_new("sec", Component::TreeSection(TreeSectionProps { label: None, default_open: None, header_toolbar: None, window: None }))
                .expect("bounded fixture")
                .try_with_children([row("item-1", "Item 1"), row("item-2", "Item 2")])
                .unwrap_or_else(|_| panic!("bounded fixture"));
            let root = TreeNode::try_new("root", Component::Tree(TreeProps { presentation: Default::default(), interaction_domain: Some(UiText::try_from_str("items").expect("bounded fixture")) }))
                .expect("bounded fixture")
                .try_with_children([section])
                .unwrap_or_else(|_| panic!("bounded fixture"));
            ComponentTree::new(root)
        };

        reserved_action(&mut app, INTERACTION_SELECT_ACTION_ID, Some(&interaction_target_args(json!({ "domainId": "items", "merge": "replace", "method": "pick" }), "item-1")), &mut identity).await;
        let state = app.interaction_state().await;
        app.tools.stamp_and_cache_interaction_ui(&mut app.roster, &mut app.interaction, &tree(), &state, "window").await.expect("bounded fixture");
        let first: Vec<(String, bool)> = app.roster.pending_presence.drain(..).map(|update| (update.node_key, update.own.selected)).collect();
        assert_eq!(first, vec![("item-1".to_string(), true)]);

        reserved_action(&mut app, INTERACTION_SELECT_ACTION_ID, Some(&interaction_target_args(json!({ "domainId": "items", "merge": "replace", "method": "pick" }), "item-2")), &mut identity).await;
        let state = app.interaction_state().await;
        app.tools.stamp_and_cache_interaction_ui(&mut app.roster, &mut app.interaction, &tree(), &state, "window").await.expect("bounded fixture");
        let second: Vec<(String, bool)> = app.roster.pending_presence.drain(..).map(|update| (update.node_key, update.own.selected)).collect();
        assert!(second.contains(&("item-2".to_string(), true)), "the newly selected row must be marked: {second:?}");
        assert!(second.contains(&("item-1".to_string(), false)), "the row that LOST the selection must be retired, not left marked: {second:?}");

        reserved_action(&mut app, INTERACTION_SELECT_ACTION_ID, Some(&interaction_target_args(json!({ "domainId": "items", "merge": "replace", "method": "pick" }), "item-2")), &mut identity).await;
        let state = app.interaction_state().await;
        app.tools.stamp_and_cache_interaction_ui(&mut app.roster, &mut app.interaction, &tree(), &state, "window").await.expect("bounded fixture");
        let third: Vec<(String, bool)> = app.roster.pending_presence.drain(..).map(|update| (update.node_key, update.own.selected)).collect();
        assert_eq!(third, vec![("item-2".to_string(), true)], "an unchanged selection must cost exactly its own mark, never a repeated retirement");
        close_reserved_app(&mut app, mounted_policy,cx);
    
        }.await;let fuel=cx.fuel_remaining();drop(original_context);original_caller.retire(fuel).expect("original builder fixture paid terminal retirement");result
    }
