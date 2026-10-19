/// 🪦️ Snapshot retirement, gesture and agent transaction: the test snapshot retirement, closed dialect pin rejection, the composite undo group and the owned child op groups of an agent transaction.

    struct TestSnapshotRetirement {
        pub owner: Option<Box<dyn store::ErasedSnapshotRetirement>>,
        lie_about_terminal: bool,
    }

    impl store::ErasedSnapshotRetirement for TestSnapshotRetirement {
 fn next_demand(&self,maximum_body_bytes:usize)->Result<semio_framework_value::RetirementDemand,semio_framework_value::ValueError>{self.demands(maximum_body_bytes)}

        fn close_step(&mut self, grant: semio_framework_value::retained_clone::RetainedCloneGrant) -> Result<semio_framework_value::retained_clone::RetainedCloneStep, semio_framework_value::ValueError> {
            use semio_framework_value::retained_clone::{RetainedCloneProgress, RetainedCloneStep};
            if grant.maximum_items == 0 { return Ok(RetainedCloneStep::Progress(RetainedCloneProgress::default())); }
            if self.lie_about_terminal {
                self.lie_about_terminal = false;
                return Ok(RetainedCloneStep::Complete(RetainedCloneProgress { copied_items: 1, ..Default::default() }));
            }
            store::os_store::snapshot_read::artifact_retirement_box_close_step(&mut self.owner, grant)
        }

        fn terminal_is_empty(&self) -> bool { self.owner.is_none() }
        fn next_copy_byte_demand(&self) -> Result<usize, semio_framework_value::ValueError> { Ok(self.demands(0)?.copy_bytes) }
        fn next_capacity_byte_demand(&self, body: usize) -> Result<usize, semio_framework_value::ValueError> { Ok(self.demands(body)?.capacity_bytes) }
        fn next_release_byte_demand(&self) -> Result<usize, semio_framework_value::ValueError> { Ok(self.demands(0)?.release_bytes) }
        fn next_depth_demand(&self) -> Result<usize, semio_framework_value::ValueError> { Ok(self.demands(0)?.depth) }
    }

    impl TestSnapshotRetirement {
        pub fn demands(&self, body: usize) -> Result<semio_framework_value::RetirementDemand, semio_framework_value::ValueError> {
            if self.lie_about_terminal { return Ok(semio_framework_value::RetirementDemand { depth: 1, ..Default::default() }); }
            self.owner.as_ref().map_or(Ok(Default::default()), |owner| store::os_store::snapshot_read::artifact_retirement_box_demands(owner, body))
        }
    }

    /// 🎭️ Spends one invalid terminal witness while retaining the actual controlled payload and its physical child frame.
    #[derive(semio_framework_value::FactoryPayloadRetirement)]
    struct TestLyingOwnedValueRetirementFactory {
        #[factory_owned]
        lie_about_terminal: std::sync::atomic::AtomicBool,
    }

    impl store::ArtifactOwnedValueRetirementFactory<TestSnapshot> for TestLyingOwnedValueRetirementFactory {
        fn retirement_birth_bytes(&self, _: &TestSnapshot) -> usize {
            std::mem::size_of::<TestSnapshotRetirement>() + semio_framework_value::retirement::owned_retirement_birth_bytes::<TestSnapshot>()
        }

        fn retire_owned(&self, value: TestSnapshot, grant: semio_framework_value::retained_clone::RetainedCloneGrant) -> Result<(Box<dyn store::ErasedSnapshotRetirement>, semio_framework_value::retained_clone::RetainedCloneProgress), (semio_framework_value::ValueError, TestSnapshot)> {
            use semio_framework_value::retained_clone::RetainedCloneBirthDemand;
            let frame = std::mem::size_of::<TestSnapshotRetirement>();
            let demand = RetainedCloneBirthDemand { capacity_bytes: self.retirement_birth_bytes(&value), depth: 3 };
            if let Err(error) = demand.admit(grant) { return Err((error, value)); }
            let child = semio_framework_value::retained_clone::RetainedCloneGrant { maximum_capacity_bytes: grant.maximum_capacity_bytes - frame, maximum_depth: grant.maximum_depth - 1, ..grant };
            let (owner, mut progress) = semio_framework_value::retirement::admit_owned_retirement(value, child)?;
            progress.retained_capacity_bytes += frame;
            Ok((Box::new(TestSnapshotRetirement { owner: Some(owner), lie_about_terminal: self.lie_about_terminal.swap(false, std::sync::atomic::Ordering::AcqRel) }), progress))
        }
    }

    /// 🎭️ Installs the member owner catalog whose OWNED-VALUE factory carries the lie —
    /// the one `retire_snapshot_read_erased` draws its unique-owner disposer from, which is the only
    /// place a child-content retirement can meet a lying terminal witness at all. The DISPLACEMENT
    /// factory stays honest on purpose: it retires the snapshot a member replaces, and a lying one
    /// would pin that alias for ever, so the captured root could never become the snapshot's last
    /// owner and `Arc::into_inner` would never reach the owned-value factory.
    #[doc(hidden)]
pub fn install_test_snapshot_retirement(app: &mut VcsArtifactApp<TestApp, TestMembers>, child_id: &str, lie_about_terminal: bool) {
        let TestMembers::Child(child) = &mut app.child_composition.children.get_mut(&("slot".to_string(), child_id.to_string())).expect("exact child retirement owner").member;
        type Catalog = store::os_store::document_store_owners::DocumentStoreOwners<TestSnapshot, TestMutation>;
        let capacity = Catalog::source_birth_bytes::<semio_framework_value::factory::SharedValueRetirementFactory<TestSnapshot>, TestLyingOwnedValueRetirementFactory, semio_framework_value::factory::OwnedValueRetirementFactory<TestMutation>, store::ArtifactStoreCursorDisposer<TestSnapshot, TestMutation>>().expect("exact catalog source birth");
        let admitted = Catalog::admit_source_constructor(semio_framework_value::retained_clone::RetainedCloneGrant { maximum_items: 1, maximum_capacity_bytes: capacity, maximum_depth: 1, ..Default::default() }, || (
            semio_framework_value::factory::SharedValueRetirementFactory::<TestSnapshot>::default(),
            TestLyingOwnedValueRetirementFactory { lie_about_terminal: std::sync::atomic::AtomicBool::new(lie_about_terminal) },
            semio_framework_value::factory::OwnedValueRetirementFactory::<TestMutation>::default(),
            store::ArtifactStoreCursorDisposer::<TestSnapshot, TestMutation>::new(),
        ));
        store::install_unscheduled_catalog(child, store::os_store::document_store_owners::complete_unscheduled_catalog(admitted)).expect("exact original lying-witness catalog");
    }

    #[semio_framework_async_macros::async_test]
    async fn member_factory_closed_dialect_register_rejects_pin_without_mutating_member() {
        let mut original_caller=semio_framework_plugin_testing_caller::OriginalFixtureCaller::admit().expect("original builder fixture caller admission");let mut original_context=original_caller.context();let cx=&mut original_context;let result=async{

        let mounted_grant = semio_framework_value::retained_clone::RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: 32_768, maximum_capacity_bytes: 262_144, maximum_release_bytes: 1_048_576, maximum_depth: 4_096 };
        let mounted_policy = semio_framework_plugin_mounted_owner::MountedOwnerPolicyV1 { preparation: mounted_grant, maintenance: mounted_grant, close: mounted_grant };
        let identity_started = std::time::Instant::now();
        let mut identity_progress = |progress: semio_framework_value::native_encoding::NativeEncodeProgress| {
            progress.owned_bytes <= 1_048_576 && identity_started.elapsed() < std::time::Duration::from_secs(60)
        };
        let mut identity = semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority::new(1_048_576, &mut identity_progress as &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::Observer<'_>).expect("explicit original fixture identity authority");

        let mut app = contract_composed_app(mounted_policy, &mut identity).await;
        let dialect = test_child_dialect().await;
        let expected = ArtifactRef { artifact_id: "child-1".into(), dialect: dialect.clone() };
        app.child_composition.pending_child_pins.push(vcs::CompositionPin { child_ref: expected.clone(), checkpoint_id: "missing-checkpoint".into() });
        let member = TestMembers::create("child-1", &dialect, &TestSnapshot::default().encode_pack(), protocol::ActorId(protocol::LOCAL_ACTOR_ID.into())).await.unwrap();
        let before = member.envelope_pack_bytes().await.unwrap();
        declare_test_child(&mut app, "child-1").await;
        let error = app.register_child("slot", "child-1", dialect, member, &mut semio_framework_plugin_testing_laws::artifact_app_laws::fixture_identity()).await.expect_err("direct transfer must not apply a deferred restore pin");
        let mut returned = error.member.expect("exact caller member returned");
        assert_eq!(returned.envelope_pack_bytes().await.unwrap(), before);
        assert_eq!(returned.artifact_ref().as_ref(), Some(&expected));
        assert!(returned.owner_ref().is_none());
        assert!(app.child_composition.test_child_admission_state(1).children_empty && app.child_composition.test_child_admission_state(1).content_empty && app.child_composition.test_child_admission_state(1).abort_empty);
        assert_eq!(app.child_composition.test_child_admission_state(1).generation, 0);
        assert_eq!(app.child_composition.pending_child_pins.len(), 1);
        assert!(app.child_composition.composition.graph_mut().await.owner_of("child-1").await.is_none());
        close_member_admission_fixture(&mut returned);
        close_member_admission_app(&mut app,cx);

        artifact_app_laws::close_registered_fixture_app(&mut app.0,app.1,cx).expect("original contract fixture paid terminal close");
    
        }.await;let fuel=cx.fuel_remaining();drop(original_context);original_caller.retire(fuel).expect("original builder fixture paid terminal retirement");result
    }

    /// 🧾️ One `KernelMutation` for the parent's own op, one for the child's — each carrying
    /// its OWN document handle, never the parent's, for the child entry (Task 3's "REAL
    /// target" requirement).
    ///
    /// 🧾️ ONE `UndoGroup` names BOTH documents via `member_edits`.
    ///
    /// The child store actually applied its own op.
    ///
    /// And the command log recorded the child's edit id in `child_edit_ids`.
    ///
    /// 🧾️ The migrated route logs the admission row (no edit of its own) before the composed
    /// publication logs the real one, so the subject is the row that CARRIES the gesture: exactly
    /// one composite row names the child's edit, and it is the row that carries the parent's.
    #[semio_framework_async_macros::async_test]
    async fn composite_gesture_produces_one_undo_group_spanning_parent_and_child_with_real_handles() {
        let mut original_caller=semio_framework_plugin_testing_caller::OriginalFixtureCaller::admit().expect("original builder fixture caller admission");let mut original_context=original_caller.context();let cx=&mut original_context;let result=async{

        let mounted_grant = semio_framework_value::retained_clone::RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: 32_768, maximum_capacity_bytes: 262_144, maximum_release_bytes: 1_048_576, maximum_depth: 4_096 };
        let mounted_policy = semio_framework_plugin_mounted_owner::MountedOwnerPolicyV1 { preparation: mounted_grant, maintenance: mounted_grant, close: mounted_grant };
        let identity_started = std::time::Instant::now();
        let mut identity_progress = |progress: semio_framework_value::native_encoding::NativeEncodeProgress| {
            progress.owned_bytes <= 1_048_576 && identity_started.elapsed() < std::time::Duration::from_secs(60)
        };
        let mut identity = semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority::new(1_048_576, &mut identity_progress as &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::Observer<'_>).expect("explicit original fixture identity authority");

        let mut app = contract_composed_app(mounted_policy, &mut identity).await;
        register_test_child(&mut app, "child-1").await;

        let result = app.dispatch_typed(TestCommand::CompositeEdit { slot: "slot".into(), child_id: "child-1".into(), child_value: 7 }, &meta(), &mut identity,cx).await.expect("composite edit");

        assert_eq!(
            result.mutations.len(),
            2,
            "a composite gesture carries the parent's op and the child's; got documents {:?}, {} member edit(s), settle receipt {:?}",
            result.mutations.iter().map(|mutation| mutation.document).collect::<Vec<_>>(),
            result.inverse_group.member_edits.len(),
            result.output
        );
        let parent_handle = ArtifactHandle(meta().instance_id as u128);
        let child_handle = artifact_handle_of("child-1").await;
        assert_ne!(parent_handle, child_handle);
        let mutation_documents: std::collections::HashSet<ArtifactHandle> = result.mutations.iter().map(|mutation| mutation.document).collect();
        assert!(mutation_documents.contains(&parent_handle), "the parent's own edit must carry the parent's handle");
        assert!(mutation_documents.contains(&child_handle), "the child's edit must carry the CHILD's handle, not the parent's");

        assert_eq!(result.inverse_group.member_edits.len(), 2);
        let member_documents: std::collections::HashSet<ArtifactHandle> = result.inverse_group.member_edits.iter().map(|edit_ref| edit_ref.document).collect();
        assert!(member_documents.contains(&parent_handle));
        assert!(member_documents.contains(&child_handle));

        let entry = app.child_composition.children.get_mut(&("slot".to_string(), "child-1".to_string())).expect("child stays registered after dispatch");
        assert_eq!(entry.reference.dialect.artifact_kind, "s.test.child");
        let TestMembers::Child(child_store) = &mut entry.member;
        assert_eq!(child_store.snapshot().expect("child snapshot").count, 7);

        let history = app.render_ctx().test_history().await;
        let composite_rows: Vec<_> = history.commands.iter().filter(|entry| entry.action_id == "compositeEdit").collect();
        let carrying: Vec<_> = composite_rows.iter().filter(|row| !row.child_edit_ids.is_empty()).collect();
        assert_eq!(carrying.len(), 1, "exactly one composite row names the child's own edit id: {:?}", composite_rows.iter().map(|row| (row.edit_id.clone(), row.child_edit_ids.clone())).collect::<Vec<_>>());
        assert_eq!(carrying[0].child_edit_ids.len(), 1, "the composite row names the child's own edit id exactly once: {:?}", carrying[0].child_edit_ids);
        assert!(carrying[0].edit_id.is_some(), "the row that names the child's edit also carries the parent's own edit: {:?}", carrying[0].edit_id);

        artifact_app_laws::close_registered_fixture_app(&mut app.0,app.1,cx).expect("original contract fixture paid terminal close");
    
        }.await;let fuel=cx.fuel_remaining();drop(original_context);original_caller.retire(fuel).expect("original builder fixture paid terminal retirement");result
    }

    #[semio_framework_async_macros::async_test]
    async fn an_agent_transaction_carries_owned_child_op_groups_and_commits_undoes_and_redoes_them_as_one_group() {
        let mut original_caller=semio_framework_plugin_testing_caller::OriginalFixtureCaller::admit().expect("original builder fixture caller admission");let mut original_context=original_caller.context();let cx=&mut original_context;let result=async{

        let mounted_grant = semio_framework_value::retained_clone::RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: 32_768, maximum_capacity_bytes: 262_144, maximum_release_bytes: 1_048_576, maximum_depth: 4_096 };
        let mounted_policy = semio_framework_plugin_mounted_owner::MountedOwnerPolicyV1 { preparation: mounted_grant, maintenance: mounted_grant, close: mounted_grant };
        let identity_started = std::time::Instant::now();
        let mut identity_progress = |progress: semio_framework_value::native_encoding::NativeEncodeProgress| {
            progress.owned_bytes <= 1_048_576 && identity_started.elapsed() < std::time::Duration::from_secs(60)
        };
        let mut identity = semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority::new(1_048_576, &mut identity_progress as &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::Observer<'_>).expect("explicit original fixture identity authority");

        let mut app = contract_composed_app(mounted_policy, &mut identity).await;
        register_test_child(&mut app, "child-1").await;
        let children = vec![test_child_emit("slot", "child-1", &[TestMutation::SetCount(SetCount { value: 5 })])];
        let wire = ChildEmit::encode_groups(&children);
        assert!(!wire.is_empty(), "a gesture that touches a child carries its group");
        assert_eq!(ChildEmit::decode_groups(&wire).expect("the wire pack decodes"), children, "the group survives the wire byte for byte");
        assert!(ChildEmit::encode_groups(&[]).is_empty(), "a childless gesture leaves the wire unchanged");
        assert_eq!(ChildEmit::decode_groups(&[7, 7, 7]).expect_err("rubbish is refused").code.0, "transaction.child-groups-malformed");

        let parent_op = <TestMutation as ::protocol::OpBinary>::encode_op(&TestMutation::SetLabel(SetLabel { value: "agent".into() })).expect("encode parent op");
        let outcome = app.transaction_prepare("txn-agent-1", "", &[], &[parent_op], &wire, Some(protocol::MutationOrigin::Owner)).await;
        assert!(outcome.rejection.is_none(), "prepare admits a held child: {:?}", outcome.rejection.as_ref().map(|fault| &fault.message));
        let edit_id = app.transaction_commit("txn-agent-1", &meta(), &mut semio_framework_plugin_testing_laws::artifact_app_laws::fixture_identity()).await.expect("commit the composite transaction");
        assert!(!edit_id.is_empty());
        macro_rules! child_count {
            ($app:expr) => {{
                let TestMembers::Child(child_store) = &mut $app.child_composition.children.get_mut(&("slot".to_string(), "child-1".to_string())).expect("child stays registered").member;
                child_store.snapshot().expect("child snapshot").count
            }};
        }
        assert_eq!(child_count!(app), 5, "the child applied the agent's op");
        assert_eq!(app.stores.test_snapshot().await.label, "agent", "the parent applied the agent's op");
        assert_eq!(app.stores.store.tail_group_id().await.as_deref(), Some("txn-agent-1"), "the group identity is the transaction id");

        app.transaction_undo("txn-agent-1", &mut semio_framework_plugin_testing_laws::artifact_app_laws::fixture_identity()).await.expect("undo the whole group");
        assert_eq!(child_count!(app), 0, "undo reverts the child");
        assert_ne!(app.stores.test_snapshot().await.label, "agent", "undo reverts the parent");
        app.transaction_redo("txn-agent-1", &mut semio_framework_plugin_testing_laws::artifact_app_laws::fixture_identity()).await.expect("redo the whole group");
        assert_eq!(child_count!(app), 5, "redo reapplies the child");
        assert_eq!(app.stores.test_snapshot().await.label, "agent", "redo reapplies the parent");
        assert!(app.transaction_undo("txn-someone-else", &mut semio_framework_plugin_testing_laws::artifact_app_laws::fixture_identity()).await.is_err(), "a group no member carries is refused by name");

        let only_child = ChildEmit::encode_groups(&[test_child_emit("slot", "child-1", &[TestMutation::SetCount(SetCount { value: 8 })])]);
        let outcome = app.transaction_prepare("txn-agent-2", "", &[], &[], &only_child, Some(protocol::MutationOrigin::Owner)).await;
        assert!(outcome.rejection.is_none(), "a children-only transaction is a pre-planned transaction");
        app.transaction_commit("txn-agent-2", &meta(), &mut semio_framework_plugin_testing_laws::artifact_app_laws::fixture_identity()).await.expect("commit the children-only transaction");
        assert_eq!(child_count!(app), 8);
        app.transaction_undo("txn-agent-2", &mut semio_framework_plugin_testing_laws::artifact_app_laws::fixture_identity()).await.expect("undo the children-only group");
        assert_eq!(child_count!(app), 5);

        let stray = ChildEmit::encode_groups(&[test_child_emit("slot", "ghost", &[TestMutation::SetCount(SetCount { value: 1 })])]);
        let refused = app.transaction_prepare("txn-agent-3", "", &[], &[], &stray, Some(protocol::MutationOrigin::Owner)).await;
        assert_eq!(refused.rejection.expect("a child this instance does not hold is refused").code.0, "transaction.member-rejected");

        artifact_app_laws::close_registered_fixture_app(&mut app.0,app.1,cx).expect("original contract fixture paid terminal close");

        }.await;let fuel=cx.fuel_remaining();drop(original_context);original_caller.retire(fuel).expect("original builder fixture paid terminal retirement");result
    }
