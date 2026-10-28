    /// ⚖️ Slot and publication fairness: the retained latest-wins slot and publication fairness driver.

    /// 🛑️ Operation 1 is parked in `Worker` with no session — a state `drive_worker_step` refuses
    /// by name, and correctly so (`session` is cleared only as the stage moves to `Publishing`).
    /// The law is about what that refusal costs: it must terminate THAT operation and nothing
    /// else. Before `fault_typed_operation_worker` the `Err` was propagated out of
    /// `advance_typed_operation_publication_one`, so the actor's single publication unit died
    /// with it and the ready publisher above could never have reached generation 1.
    ///
    /// 🧹️ The host ACKs the fault page of the operation it just lost, exactly as the shell
    /// does; operation 2's presented page is deliberately left unacknowledged, because the
    /// clauses below are about a presented-but-unACKed page's effect on retirement.
    ///
    /// 🪪️ Straight at the mounted operation: `acknowledge_typed_operation_result` first
    /// matches the token's receiver against the app's BOUND live instance, and this
    /// fixture app is driven without one (every other clause addresses instance 7 by
    /// hand). The subject here is the operation's own page accounting, not the mount.
    ///
    /// 🎡️ `maintenance_step` ROTATES: a stage that can release nothing hands the call on, so the
    /// step WORD is the rotation's verdict, not stage 18's. What stage 18 owes while the
    /// cancellation state is locked is that it reclaims nothing and leaves its own cursor exactly
    /// where it was — asserted directly below, over the property instead of its proxy.
use semio_framework_plugin_decl_app::tool_contract::TypedOperationResultLane;
pub(super) async fn retained_latest_wins_slot_and_publication_fairness<A: ArtifactApp<Presence = PublicationPresence, PresenceMutation = PublicationPresenceMutation> + Default>() {
        let fixture: Value = serde_json::from_str(include_str!("../../🧫️fixtures/🔗️tool-latest-wins-integration.json")).unwrap();
        let mounted_grant=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:32_768,maximum_capacity_bytes:262_144,maximum_release_bytes:1_048_576,maximum_depth:4_096};
        let mounted_policy=semio_framework_plugin_mounted_owner::MountedOwnerPolicyV1{preparation:mounted_grant,maintenance:mounted_grant,close:mounted_grant};
        let identity_started=std::time::Instant::now();
        let mut identity_progress=|progress:semio_framework_value::native_encoding::NativeEncodeProgress|{progress.owned_bytes<=1_048_576&&identity_started.elapsed()<std::time::Duration::from_secs(60)};
        let mut identity=semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority::new(1_048_576,&mut identity_progress as &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::Observer<'_>).expect("original typed fixture supplied identity authority");
        let mut app = VcsArtifactApp::<A>::new(A::default(), protocol::ActorId(protocol::LOCAL_ACTOR_ID.into()),mounted_policy,&mut identity).await;
        let first = fixture["slotReservation"]["firstOperation"].as_u64().unwrap();
        let collision = fixture["slotReservation"]["collidingOperation"].as_u64().unwrap();
        app.tools.typed_operation_reservations[first as usize % ARTIFACT_LIVE_OUTPUT_SLOTS] = Some(first);
        assert_eq!(serde_json::json!(app.tools.can_admit_typed_operation(collision)), fixture["slotReservation"]["collisionAdmitted"]);
        assert!(!app.tools.can_admit_typed_operation(first));
        app.tools.typed_operation_reservations[first as usize % ARTIFACT_LIVE_OUTPUT_SLOTS] = None;
        assert!(app.tools.can_admit_typed_operation(collision));
        let revision = app.stores.store.content_revision_now();
        for id in [1, 2] {
            let operation = semio_framework_job::Operation::new(semio_framework_job::OperationId(id), semio_framework_job::RevisionId(u64::from_be_bytes(revision[..8].try_into().unwrap())), semio_framework_job::Generation(0), 17);
            let lease =
                app.tools.tool_cancellations.begin_keyed(ToolOperationKey { app_instance_id: 7, document: ArtifactDocumentAuthority(7), operation_id: operation.operation, base_revision: operation.base_revision, generation: operation.generation }).unwrap();
            let pending = if id == 2 {
                Some(PendingArtifactStorePublication::Presence(
                    app.stores.presence_store.begin_publish_one(operation.operation, 0, ChangePublicationPresence { revision: 1 }.into(), Some(&TwoTurnPublicationPresencePreparationFactory), app.stores.presence_local_root_retirement_factory.clone()).unwrap(),
                ))
            } else {
                None
            };
            app.tools.tool_operations.insert_admitted(
                id,
                MountedTypedCommandFullOperation::<A> {
                    verb: "setGraphParameter".into(),
                    meta: ActionMeta { actor: "fixture".into(), instance_id: 7, view_state: None },
                    operation,
                    canonical_revision: revision,
                    artifact_generation: 0,
                    config_generation: 0,
                    draft_generation: 0,
                    presence_generation: 0,
                    transient_generation: 0,
                    window_config_authority: None,
                    window_transient_authority: None,
                    publication_lanes: &[ArtifactToolPublicationLane::Presence], contract: Some(semio_framework::ToolExecutionContract::bounded_first_step(1 << 20, 1 << 20, 1 << 20, 1 << 20, 7_999)),
                    session: None,
                    session_rejected: None,producer_source:None,producer_request:None,producer_request_retirement:None,
                    reserved_producer: None, completion: None,
                    completion_retirement:None,publication_retirement:None,output_retirement:None,raw_input: None,
                    output_chunks: None,
                    cancellation_lease: Some(lease),
                    identity:None,identity_progress:semio_framework_value::native_encoding::NativeEncodeProgress{completed:0,total:0,owned_bytes:0},
                    worker_semantic_pending: false, worker_outcome_pending: false, worker_fault_capture: None,
                    terminal_seen: true,
                worker_resume_pending: false,
                    publication: Some(ArtifactToolCompletionValue::Emit(Ok(Emit::default()), EphemeralEmit::default())),
                    pending_artifact_publication: pending, pending_publication_outcome:PendingPublicationOutcome::new(),pending_window_config_receipt:None,cancellation_retirement:None,
                    pending_child_publication: None,
                    owned_child_group: None,
                    owned_child_committed: false,
                    owned_child_result_pending: false,
                    captured_child_content: Some(std::sync::Arc::new(ChildContentView::EMPTY)),
                    captured_child_content_generation: 0,
                    result_page: None, diagnostics:None,
                    result_page_presented: false,
                    result_sequence: 0,
                    publication_progress: 0,
                    publication_checkpoint: None, publication_ownership_progress: None, original_retirement_receipt: None, actor_capture: None,
                    publication_attempt: 0,
                    ui_pending: false,
                    progress: None,
                    progress_pending: false,
                    user_cancel_requested: false,
                    published_artifact: false,
                    published_config: false,
                    published_window_config: false,
                    command_logged: false,
                interaction_revalidated: false,
                    retained_close_fault: None,retained_close_fault_retirement:None,retained_close_fault_refusal:None, terminal_fault: None,
                    stage: if id == 1 { MountedTypedCommandFullOperationStage::Worker } else { MountedTypedCommandFullOperationStage::Publishing },
                },
            );
        }
        for _ in 0..fixture["fairness"]["secondPublishesWithinMetadataVisits"].as_u64().unwrap() {
            if app.stores.presence_store.generation_now() == 1 {
                break;
            }
            app.typed_ctx().advance_typed_operation_publication_one(semio_framework_plugin_testing_laws::artifact_app_laws::fixture_mounted_policy().maintenance).await.unwrap();
        }
        assert_eq!(app.stores.presence_store.generation_now(), 1);
        let stuck = app.tools.tool_operations.get(1).expect("the structurally faulted operation stays mounted until it retires");
        assert_eq!(stuck.stage, MountedTypedCommandFullOperationStage::AwaitingAck, "a refused worker step leaves its own operation holding its own terminal fault page");
        let mut pages: Vec<(u64, TypedOperationResultLane, String)> = Vec::new();
        while let Some(mut page) = semio_framework_plugin_testing_laws::artifact_app_laws::take_fixture_result_page(&mut app,7,semio_framework_plugin_testing_laws::artifact_app_laws::fixture_mounted_policy().preparation).expect("original fixture page presentation") {
            let code = if page.lane == TypedOperationResultLane::Fault { semio_framework_plugin_decl_app::tool_contract::decode_typed_operation_fault_page(page.bytes()).code.0 } else { String::new() };
            let operation = page.token.operation;
            pages.push((operation, page.lane, code));
            let token =semio_framework_plugin_testing_laws::artifact_app_laws::close_fixture_result_page(&mut page,semio_framework_plugin_testing_laws::artifact_app_laws::fixture_mounted_policy().close);
                let acknowledged = app.tools.acknowledge_typed_operation_result(token).expect("each original published page accepts its exact ACK after receiver closure");
                assert!(acknowledged, "published operation {operation} must retain its original ACK authority");

        }
        assert!(
            pages.contains(&(1, TypedOperationResultLane::Fault, "interactive-job.typed-operation-session".to_string())),
            "the stuck operation is terminated BY NAME, not silently: got {pages:?}"
        );
        assert!(pages.iter().any(|(operation, lane, _)| *operation == 2 && *lane == TypedOperationResultLane::Presence), "the ready publisher still published its own presence receipt: got {pages:?}");
        for id in 3..=ARTIFACT_LIVE_OUTPUT_SLOTS as u64 {
            let operation = semio_framework_job::Operation::new(semio_framework_job::OperationId(id), semio_framework_job::RevisionId(u64::from_be_bytes(revision[..8].try_into().unwrap())), semio_framework_job::Generation(0), 17);
            let page = TypedOperationResultPage::try_new(TypedOperationResultToken { receiver: 7, operation: id, generation: 0, sequence: 0, attempt: 1 }, TypedOperationResultLane::Terminal, b"presented ACK waiter").unwrap();
            app.tools.tool_operations.insert_admitted(
                id,
                MountedTypedCommandFullOperation::<A> {
                    verb: String::new(),
                    meta: ActionMeta { actor: Default::default(), instance_id: 7, view_state: None },
                    operation,
                    canonical_revision: revision,
                    artifact_generation: 0,
                    config_generation: 0,
                    draft_generation: 0,
                    presence_generation: 0,
                    transient_generation: 0,
                    window_config_authority: None,
                    window_transient_authority: None,
                    publication_lanes: &[], contract: Some(semio_framework::ToolExecutionContract::bounded_first_step(1 << 20, 1 << 20, 1 << 20, 1 << 20, 7_999)),
                    session: None,
                    session_rejected: None,producer_source:None,producer_request:None,producer_request_retirement:None,
                    reserved_producer: None, completion: None,
                    completion_retirement:None,publication_retirement:None,output_retirement:None,raw_input: None,
                    output_chunks: None,
                    cancellation_lease: None,
                    identity:None,identity_progress:semio_framework_value::native_encoding::NativeEncodeProgress{completed:0,total:0,owned_bytes:0},
                    worker_semantic_pending: false, worker_outcome_pending: false, worker_fault_capture: None,
                    terminal_seen: true,
                worker_resume_pending: false,
                    publication: None,
                    pending_artifact_publication: None, pending_stream_abort:None, stream_abort_completed:false, pending_publication_outcome:PendingPublicationOutcome::new(),pending_window_config_receipt:None,cancellation_retirement:None,
                    pending_child_publication: None,
                    owned_child_group: None,
                    owned_child_committed: false,
                    owned_child_result_pending: false,
                    captured_child_content: Some(std::sync::Arc::new(ChildContentView::EMPTY)),
                    captured_child_content_generation: 0,
                    result_page: Some(page),
                    result_page_presented: true,
                    result_sequence: 0,
                    publication_progress: 0,
                    publication_checkpoint: None, publication_ownership_progress: None, original_retirement_receipt: None, actor_capture: None,
                    publication_attempt: 0,
                    ui_pending: false,
                    progress: None,
                    progress_pending: false,
                    user_cancel_requested: false,
                    published_artifact: false,
                    published_config: false,
                    published_window_config: false,
                    command_logged: false,
                interaction_revalidated: false,
                    retained_close_fault: None,retained_close_fault_retirement:None,retained_close_fault_refusal:None, terminal_fault: None,
                    stage: MountedTypedCommandFullOperationStage::AwaitingAck,
                },
            );
        }
        app.tools.tool_operations.get_mut(1).unwrap().stage = MountedTypedCommandFullOperationStage::Retiring;
        assert!(app.tools.tool_operations.get(1).unwrap().publication.is_some());
        let quoted = app.mounted_policy.close;
        let released = app.typed_ctx().retire_typed_operation_unit(1, quoted).unwrap();
        assert!(matches!(released, PluginLifecycleStep::Progress(progress) if progress.fits(quoted) && progress != Default::default()), "one bounded retirement unit releases the retiring operation's publication: {released:?}");
        assert!(app.tools.tool_operations.get(1).unwrap().publication.is_none());
        assert_eq!(app.tools.tool_operations.get(2).unwrap().stage, MountedTypedCommandFullOperationStage::AwaitingAck);
        assert!(app.tools.tool_operations.get(2).unwrap().result_page_presented);
        for id in 3..=ARTIFACT_LIVE_OUTPUT_SLOTS as u64 {
            drop(app.tools.tool_operations.remove(id).expect("remove synthetic presented ACK waiter"));
        }
        let cancellation_state = app.tools.tool_cancellations.state.clone();
        let lock = cancellation_state.lock().unwrap();
        for _ in 0..64 {
            if app.tools.tool_operations.get(1).unwrap().terminal_is_empty() {
                break;
            }
            let quoted = app.mounted_policy.close;
            app.typed_ctx().retire_typed_operation_unit(1, quoted).unwrap();
        }
        assert!(app.tools.tool_operations.get(1).unwrap().terminal_is_empty());
        assert_eq!(app.tools.tool_cancellations.active_operation_count(), 2);
        app.maintenance.maintenance_stage = 18;
        app.maintenance.maintenance_cancellation_cursor = TOOL_CANCELLATION_SLOTS + 1;
        let quoted = app.mounted_policy.maintenance;
        let locked_step = app.maintenance_step(quoted).unwrap();
        assert!(
            matches!(locked_step, PluginLifecycleStep::Progress(progress) | PluginLifecycleStep::Complete(progress) if progress.fits(quoted)),
            "a bounded maintenance turn never exceeds its own grant: {locked_step:?}"
        );
        assert_eq!(app.maintenance.maintenance_cancellation_cursor, TOOL_CANCELLATION_SLOTS + 1);
        assert_eq!(app.tools.tool_cancellations.active_operation_count(), 2);
        drop(lock);
        app.maintenance.maintenance_stage = 18;
        let quoted = app.mounted_policy.maintenance;
        app.maintenance_step(quoted).unwrap();
        assert_eq!(app.tools.tool_cancellations.active_operation_count(), 1);
        close_app(&mut app, "fairness fixture close");
        assert!(app.close_terminal_is_empty());
    }
