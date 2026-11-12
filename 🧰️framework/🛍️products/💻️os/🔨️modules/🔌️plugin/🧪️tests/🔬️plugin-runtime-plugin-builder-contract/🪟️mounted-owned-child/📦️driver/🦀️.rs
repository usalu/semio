use semio_plugin_decl_app::tool_contract::ArtifactToolPublicationLane;
#[doc(hidden)]
pub async fn test_mounted_original_owned_publication<A: ArtifactApp, M: ReferenceSpaceMember + MemberFactory + Send + 'static>(app: &mut VcsArtifactApp<A, M>, emit: Emit<A::Mutation, A::ConfigMutation, A::DraftMutation>, meta: ActionMeta, parent_touched: bool, observe: impl Fn(&VcsArtifactApp<A, M>) -> Vec<bool>) -> (Vec<KernelMutation>, UndoGroup, u64,semio_framework_job::OriginalJobFixtureCaller) {
    let fixture:serde_json::Value=serde_json::from_str(include_str!("../🧫️fixtures/🔣️.json")).unwrap();
    assert_eq!(fixture["acknowledgement"]["preserveOwnedChildDuringProducerRetirement"],true);
    assert_eq!(fixture["acknowledgement"]["chargeReceiptExactlyOnce"],true);
    let revision = app.stores.store.content_revision_now();
    let command_count = app.history.command_log.len();
        let operation = semio_framework_job::Operation::new(semio_framework_job::allocate_operation_id(), semio_framework_job::RevisionId(u64::from_be_bytes(revision[..8].try_into().unwrap())), semio_framework_job::Generation(app.stores.store.generation_now()), 17);
        let policy=semio_plugin_testing_laws::artifact_app_laws::fixture_mounted_policy();
        let mut caller=semio_framework_job::OriginalJobFixtureCaller::admit(operation.operation,operation.generation,policy.preparation,||Some(1),1_000_000,1);
        assert_eq!(policy.maintenance.maximum_items,fixture["acknowledgement"]["maximumItems"].as_u64().unwrap()as usize);
        let key=ToolOperationKey { app_instance_id: meta.instance_id, document: ArtifactDocumentAuthority(meta.instance_id), operation_id: operation.operation, base_revision: operation.base_revision, generation: operation.generation };
        let mut lease=None;
        let mut source_identity=None;
        for _ in 0..100000 {
            caller.turn(policy.preparation,|cx|{
                semio_plugin_testing_laws::artifact_app_laws::ensure_fixture_cancellation_issuer(app,cx)?;
                lease=app.tools.tool_cancellations.begin(&key,cx)?;
                if lease.is_some(){source_identity=Some(semio_framework_job::OriginalSourceBinding::borrow(cx));}
                Ok(())
            }).unwrap();
            if lease.is_some(){break}
        }
        let lease=lease.expect("original owned-child cancellation lease admitted");
        let completion=semio_plugin_decl_app::tool_contract::ArtifactToolCompletion::<A>::new();
        let mut observer=|_:semio_framework_value::native_encoding::NativeEncodeProgress|!lease.token.is_cancelled_now()&&!lease.publication_claim.is_cancelled();
        let identity=semio_framework_value::NativeEncodeControl::new(1<<20,&mut observer).pause().unwrap();
        drop(observer);
        completion.install_authoring_identity(identity,&lease).unwrap();
        let mut mounted = MountedTypedCommandFullOperation::<A> { reserved_construction: Default::default(),
            source_identity:source_identity.unwrap(),
            original_emission_caller:None,original_emission_decoder:None,original_emission_body:None,
            verb: "compositeEdit".into(), meta, operation, canonical_revision: revision,
            artifact_generation: operation.generation.0, config_generation: 0, draft_generation: 0, presence_generation: 0, transient_generation: 0,
            window_config_authority: None, window_transient_authority: None, publication_lanes: &[ArtifactToolPublicationLane::Artifact, ArtifactToolPublicationLane::Child], contract: Some(semio_framework::ToolExecutionContract::bounded_first_step(1 << 20, 1 << 20, 1 << 20, 1 << 20, 7_999)),
            session: None, session_rejected: None,producer_source:None,producer_request:None,producer_request_retirement:None, reserved_producer: None, completion: Some(completion), completion_retirement:None,publication_retirement:None,output_retirement:None,raw_input: None, output_chunks: None, cancellation_lease: Some(lease), identity: None, identity_progress: semio_framework_value::native_encoding::NativeEncodeProgress { completed: 0, total: 0, owned_bytes: 0 }, worker_semantic_pending: false, worker_outcome_pending: false, worker_fault_capture: None, terminal_seen: true, worker_resume_pending: false,
            publication: Some(ArtifactToolCompletionValue::Emit(Ok(emit), EphemeralEmit::default())), tool_stage:semio_plugin_retained_command::typed_operation::OriginalToolSettlementStage::Capture,tool_press_result:None,pending_artifact_publication:None,publication_failure:Default::default(), pending_stream_abort:None,stream_abort_resident:Default::default(), stream_abort_completed:false, pending_publication_outcome: PendingPublicationOutcome::new(), pending_window_config_receipt: None, cancellation_retirement: None, pending_child_publication: None,
            owned_child_group: None, owned_child_committed: false, owned_child_result_pending: false,
            captured_child_content: Some(semio_framework_value::retirement::arc_custody::original::SharedOriginalStorage::from_cold(ChildContentView::clone(&app.child_composition.child_content_root))), captured_child_content_generation: app.child_composition.child_content_generation,
            page_publication:semio_plugin_retained_command::typed_operation::MountedPagePublication::new(),result_page:None, diagnostics:None, result_page_presented: false, result_sequence: 0, publication_progress: 0, publication_checkpoint: None, publication_ownership_progress: None, original_retirement_receipt: None, actor_capture: None,actor_capture_pending:semio_plugin_mounted_owner::actor_capture::ActorCapture::new(), publication_attempt: 0,
            ui_pending: true, progress: None, progress_pending: false, user_cancel_requested: false, published_artifact: false, published_config: false, published_window_config: false,
            command_logged: false, interaction_revalidated: false, retained_close_fault: None,retained_close_fault_retirement:None,retained_close_fault_refusal:None, terminal_fault: None, stage: MountedTypedCommandFullOperationStage::Publishing,
        };

        let started = std::time::Instant::now();
        let mut child_page = false;
        let mut witnessed = false;
        for _ in 0..100000 {
            if !witnessed && started.elapsed().as_millis() >= 85000 {
                witnessed = true;
                if let Some(owner) = mounted.owned_child_group.and_then(|id| app.child_composition.private_child_groups.get(id)) {
                    let state = owner.publication.as_ref().map(|group| (group.inputs_ready(),group.openings_ready(),group.publications_ready(),group.projection_ready(),group.entries_ready()));
                } else {
                }
            }
            assert!(started.elapsed().as_millis() <= 90000, "original mounted publication wall limit");
            test_mounted_publication_original_turn(app,&mut mounted,&mut caller,false).expect("actual retained publisher owns original parent and children");
            let changed = observe(&app);
            assert!(!changed.is_empty());
            assert!(changed.iter().all(|changed| *changed) || changed.iter().all(|changed| !*changed), "all original child lanes share one visibility decision");
            if parent_touched { assert_eq!(app.stores.store.content_revision_now() != revision, changed[0], "original parent and every child flip together"); }
            if mounted.stage == MountedTypedCommandFullOperationStage::AwaitingAck {
                let mut projected=None;
                for _ in 0..100000 {
                    caller.turn(policy.preparation,|cx|{projected=semio_plugin_testing_laws::artifact_app_laws::present_fixture_mounted_page(&mut mounted,policy.preparation,cx)?;Ok(())}).unwrap();
                    if projected.is_some(){break}
                }
                let mut page=projected.expect("actual financed owned-child result page");
                let bytes = page.bytes().to_vec();
                let token = page.token;
                assert!(semio_framework_value::AuthoringWallet::new(policy.preparation,1<<16).run(policy.preparation,1<<16,|effect|mounted.take_result_page(effect)).unwrap().unwrap().output.unwrap().0.is_none());
                test_mounted_publication_original_turn(app,&mut mounted,&mut caller,false).unwrap();
                assert_eq!(mounted.result_page.as_ref().unwrap().bytes(), bytes);
                assert_eq!(mounted.result_page.as_ref().unwrap().token, token);
                if page.lane == TypedOperationResultLane::Child { child_page = true; assert!(mounted.owned_child_committed); assert_eq!(app.history.command_log.len(),command_count+1); assert_eq!(app.history.command_log.iter().next_back().unwrap().action_id,"compositeEdit"); }
                let terminal = matches!(page.lane, TypedOperationResultLane::Terminal | TypedOperationResultLane::Fault);
                let closed=semio_plugin_testing_laws::artifact_app_laws::close_fixture_result_page(&mut page,semio_plugin_testing_laws::artifact_app_laws::fixture_mounted_policy().close);assert_eq!(closed,token);assert!(mounted.acknowledge_result_page(token).unwrap());
                let child_owner=mounted.owned_child_group;
                assert!(mounted.result_page_retirement_pending());
                for _ in 0..100000 {
                    test_mounted_publication_original_turn(app,&mut mounted,&mut caller,false).unwrap();
                    assert_eq!(mounted.owned_child_group,child_owner,"one-item ACK producer cleanup preserves its owned-child group");
                    if !mounted.result_page_retirement_pending(){break}
                }
                assert!(!mounted.result_page_retirement_pending(),"real publisher retires the acknowledged producer page");
                if terminal { break; }
            }
        }
    assert!(child_page, "actual publisher must deliver its child result page");
    let result = app.tools.typed_composed_outbox.pop().expect("actual original member mutation/inverse outbox");
    assert!(!mounted.owned_child_result_pending, "original child result ACK releases the publication close barrier");
    for _ in 0..100000 { if mounted.owned_child_group.is_none() { break; } test_mounted_publication_original_turn(app,&mut mounted,&mut caller,true).unwrap(); }
    assert!(mounted.owned_child_group.is_none(), "actual private group registry closes before the mounted original owner");
    let mut retired = false;
    for _ in 0..100000 {
        let grant = app.mounted_policy.close;
        let demand = mounted.retirement_demands(grant.maximum_copy_bytes).unwrap();
        assert!(grant.maximum_items>0&&grant.maximum_copy_bytes>=demand.copy_bytes&&grant.maximum_capacity_bytes>=demand.capacity_bytes&&grant.maximum_release_bytes>=demand.release_bytes&&grant.maximum_depth>=demand.depth,"original mounted caller policy must fund its quoted close frontier");
        if matches!(caller.turn(grant,|cx|mounted.retirement_step(cx)).unwrap(), semio_plugin_ownership::bounded_ownership::PluginLifecycleStep::Complete(_)) { retired=true; break; }
    }
    assert!(retired, "mounted original owner reaches its complete terminal retirement");
    assert_eq!(mounted.stage, MountedTypedCommandFullOperationStage::Retiring);
    (result.mutations, result.inverse_group, operation.operation.0,caller)
}

/// 🎟️ Polls the actual mounted publisher once under its transaction's original paid fixture caller.
fn test_mounted_publication_original_turn<A:ArtifactApp,M:ReferenceSpaceMember+MemberFactory+Send+'static>(app:&mut VcsArtifactApp<A,M>,mounted:&mut MountedTypedCommandFullOperation<A>,caller:&mut semio_framework_job::OriginalJobFixtureCaller,closing:bool)->Result<(),semio_framework_job::OriginalTurnFailure>{
    let grant=if closing{app.mounted_policy.close}else{app.mounted_policy.maintenance};
    caller.turn(grant,|cx|{let before=cx.retained_progress();let mut task=std::task::Context::from_waker(std::task::Waker::noop());
        let result:Result<(),semio_framework_job::OriginalTurnFailure>=if closing{let mut ctx=app.typed_ctx();let mut turn=std::pin::pin!(ctx.publish_mounted_owned_child_operation_unit(mounted,cx));match std::future::Future::poll(turn.as_mut(),&mut task){std::task::Poll::Ready(result)=>result.map_err(Into::into),std::task::Poll::Pending=>panic!("private close suspended inside its original caller")}}
        else{let mut ctx=app.typed_ctx();let mut turn=std::pin::pin!(ctx.publish_mounted_typed_operation_run(mounted,cx,grant));match std::future::Future::poll(turn.as_mut(),&mut task){std::task::Poll::Ready(result)=>result.map_err(Into::into),std::task::Poll::Pending=>panic!("private publication suspended inside its original caller")}};
        result?;
        if let Some((returned,progress))=mounted.publication_ownership_progress.take(){
            assert!(progress.fits(returned)&&progress.fits(grant));
            let unposted=semio_plugin_ownership::bounded_ownership::original_lifecycle_unposted(progress,before,cx.retained_progress())?;
            cx.consume_retained(unposted)?;
            assert_eq!(semio_plugin_ownership::bounded_ownership::original_lifecycle_unposted(progress,before,cx.retained_progress())?,Default::default(),"publication receipt is charged exactly once");
        }
        Ok(())
    })
}
