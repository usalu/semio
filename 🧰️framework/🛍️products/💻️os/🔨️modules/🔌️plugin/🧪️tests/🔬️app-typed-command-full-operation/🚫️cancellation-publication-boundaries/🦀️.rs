    /// 🚫️ Cancellation publication boundaries: the retained cancellation publication boundaries and the owned latest-wins key with its zero and sub-exact headers.

    use semio_framework_plugin_instance_app::VcsArtifactApp;
use semio_framework_plugin_decl_emission::Emit;
use semio_framework_plugin_decl_app::app_emission::EphemeralEmit;
use semio_framework_plugin_decl_views::child_view::ChildContentView;
use semio_framework_plugin_composition_publication::MountedTypedCommandFullOperationStage;
use semio_framework_plugin_composition_publication::PendingArtifactStorePublication;
use semio_framework_plugin_operation::ToolCancellationHandle;
use semio_framework_plugin_command::command_admission::original_latest_key::ToolLatestWinsKeyCopy;
use semio_framework_plugin_decl_app::tool_contract::ArtifactToolPublicationLane;
use semio_framework_plugin_decl_app::tool_contract::ArtifactToolCompletionValue;
use semio_framework_plugin_decl_app::tool_contract::TypedOperationResultPage;
pub(super) async fn retained_cancellation_publication_boundaries<A: ArtifactApp<Presence = PublicationPresence, PresenceMutation = PublicationPresenceMutation> + Default>() {
        let fixture: Value = serde_json::from_str(semio_framework_test_fixtures::fixture_text!("../../../../🧪️tests/🧫️fixtures/🥇️tool-latest-wins.json")).expect("language-neutral cancellation boundaries");
        let bytes = fixture["maximumBytes"].as_u64().unwrap() as usize;
        let grant = store::os_store::one_item::ArtifactStoreOneItemGrant { maximum_items: fixture["maximumItems"].as_u64().unwrap() as usize, maximum_copy_bytes: bytes, maximum_capacity_bytes: bytes, maximum_release_bytes: bytes, maximum_depth: 128 };
        assert_eq!(grant.maximum_items, 1);
        assert_eq!(bytes, TYPED_OPERATION_RESULT_PAGE_BYTES);
        for case in fixture["publicationCases"].as_array().unwrap() {
            let boundary = case["cancelAt"].as_str().unwrap();
            let mounted_grant=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:32_768,maximum_capacity_bytes:262_144,maximum_release_bytes:1_048_576,maximum_depth:4_096};
        let mounted_policy=semio_framework_plugin_mounted_owner::MountedOwnerPolicyV1{preparation:mounted_grant,maintenance:mounted_grant,close:mounted_grant};
        let identity_started=std::time::Instant::now();
        let mut identity_progress=|progress:semio_framework_value::native_encoding::NativeEncodeProgress|{progress.owned_bytes<=1_048_576&&identity_started.elapsed()<std::time::Duration::from_secs(60)};
        let mut identity=semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority::new(1_048_576,&mut identity_progress as &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::Observer<'_>).expect("original typed fixture supplied identity authority");
        let mut app = VcsArtifactApp::<A>::new(A::default(), protocol::ActorId(protocol::LOCAL_ACTOR_ID.into()),mounted_policy,&mut identity).await;
            let revision = app.stores.store.content_revision_now();
            let operation = semio_framework_job::Operation::new(
                semio_framework_job::allocate_operation_id(),
                semio_framework_job::RevisionId(u64::from_be_bytes(revision[..8].try_into().unwrap())),
                semio_framework_job::Generation(app.stores.store.generation_now()),
                17,
            );
            let cancellations = app.tools.tool_cancellations.clone();
            let lease = cancellations.begin(ToolOperationKey { app_instance_id: 7, document: ArtifactDocumentAuthority(7), operation_id: operation.operation, base_revision: operation.base_revision, generation: operation.generation }).unwrap();
            let before = app.stores.presence_store.local_read().unwrap();
            let mut mounted = MountedTypedCommandFullOperation::<A> {
                verb: "setGraphParameter".into(),
                meta: ActionMeta { actor: "fixture".into(), instance_id: 7, view_state: None },
                operation,
                canonical_revision: revision,
                artifact_generation: operation.generation.0,
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
                pending_artifact_publication: None, pending_publication_outcome:PendingPublicationOutcome::new(),pending_window_config_receipt:None,cancellation_retirement:None,
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
                ui_pending: true,
                progress: None,
                progress_pending: false,
                user_cancel_requested: false,
                published_artifact: false,
                published_config: false,
                published_window_config: false,
                command_logged: false,
                interaction_revalidated: false,
                retained_close_fault: None,retained_close_fault_retirement:None,retained_close_fault_refusal:None, terminal_fault: None,
                stage: MountedTypedCommandFullOperationStage::Publishing,
            };
            if boundary != "producer" {
                let pending =
                    app.stores.presence_store.begin_publish_one(operation.operation, 0, ChangePublicationPresence { revision: 1 }.into(), Some(&TwoTurnPublicationPresencePreparationFactory), app.stores.presence_local_root_retirement_factory.clone()).unwrap();
                mounted.pending_artifact_publication = Some(PendingArtifactStorePublication::Presence(pending));
                let target = match boundary {
                    "preparation" => store::os_store::one_item_publication::ArtifactStoreOneItemPublicationPhase::Preparing,
                    "preflight" => store::os_store::one_item_publication::ArtifactStoreOneItemPublicationPhase::PreflightingCommit,
                    "publishing" => store::os_store::one_item_publication::ArtifactStoreOneItemPublicationPhase::Publishing,
                    "awaitingAck" => store::os_store::one_item_publication::ArtifactStoreOneItemPublicationPhase::AwaitingAck,
                    other => panic!("unknown publication boundary {other}"),
                };
                for _ in 0..8 {
                    let Some(PendingArtifactStorePublication::Presence(pending)) = mounted.pending_artifact_publication.as_ref() else {
                        panic!("exact pending presence owner");
                    };
                    if pending.phase() == target {
                        break;
                    }
                    app.typed_ctx().publish_mounted_typed_operation_unit(&mut mounted, semio_framework_plugin_testing_laws::artifact_app_laws::fixture_mounted_policy().maintenance).unwrap();
                }
                let Some(PendingArtifactStorePublication::Presence(pending)) = mounted.pending_artifact_publication.as_ref() else {
                    panic!("exact pending presence owner");
                };
                assert_eq!(pending.phase(), target);
            }
            mounted.cancellation_lease.as_ref().unwrap().cancel();
            if boundary == "awaitingAck" {
                let mut page = mounted.take_fixture_result_page(semio_framework_plugin_testing_laws::artifact_app_laws::fixture_mounted_policy().preparation).unwrap();
                assert!(!mounted.reject_cancelled_publication().unwrap());
                assert_eq!(mounted.result_page.as_ref().unwrap().token, page.token);
                assert_eq!(mounted.result_page.as_ref().unwrap().bytes(), page.bytes());
                assert!(mounted.acknowledge_result_page(crate::app_testing::artifact_app_laws::close_fixture_result_page(&mut page,crate::app_testing::artifact_app_laws::fixture_mounted_policy().close)).unwrap());
                mounted.close_fixture_original_page(semio_framework_plugin_testing_laws::artifact_app_laws::fixture_mounted_policy().close);
            }
            app.typed_ctx().publish_mounted_typed_operation_unit(&mut mounted, semio_framework_plugin_testing_laws::artifact_app_laws::fixture_mounted_policy().maintenance).unwrap();
            assert_eq!(mounted.stage, MountedTypedCommandFullOperationStage::AwaitingAck);
            let cancelled = mounted.result_page.as_ref().unwrap();
            assert_eq!(cancelled.lane, TypedOperationResultLane::Fault);
            assert!(std::str::from_utf8(cancelled.bytes()).unwrap().contains("cancelled"));
            assert_eq!(serde_json::json!({ "committed": app.stores.presence_store.generation_now() != 0 }), serde_json::json!({ "committed": case["committed"] }));
            assert_eq!(std::ptr::eq(before.get(), app.stores.presence_store.local()), !case["committed"].as_bool().unwrap());
            assert!(!mounted.ui_pending);
            let token = cancelled.token;
            assert!(mounted.acknowledge_result_page(token).unwrap());
            close_mounted(&mut mounted, boundary, mounted_policy.close);
            assert!(mounted.terminal_is_empty(), "{boundary}");
            assert_eq!(cancellations.active_operation_count(), 0);
            drop(before);
            close_app(&mut app, boundary);
            assert!(app.close_terminal_is_empty(), "{boundary}");
        }
        for case in fixture["linearizationCases"].as_array().unwrap() {
            let cancellations = ToolCancellationHandle::default();
            let key = ToolOperationKey {
                app_instance_id: 7,
                document: ArtifactDocumentAuthority(7),
                operation_id: semio_framework_job::allocate_operation_id(),
                base_revision: semio_framework_job::RevisionId(3),
                generation: semio_framework_job::Generation(0),
            };
            let lease = cancellations.begin(key.clone()).unwrap();
            let mut presence = store::PresenceStore::<PublicationPresence, PublicationPresenceMutation>::new(PublicationPresence::default());
            let factory: std::sync::Arc<dyn store::SnapshotRetirementFactory<PublicationPresence>> = std::sync::Arc::new(PublicationPresenceLocalRootRetirementFactory);
            presence.install_local_retirement_factory(factory.clone()).unwrap();
            let mut pending = presence.begin_publish_one(key.operation_id, 0, ChangePublicationPresence { revision: 1 }.into(), Some(&TwoTurnPublicationPresencePreparationFactory), Some(factory.clone())).unwrap();
            for _ in 0..8 {
                if pending.phase() == store::os_store::one_item_publication::ArtifactStoreOneItemPublicationPhase::Publishing {
                    break;
                }
                assert!(matches!(presence.advance_publish_one(&mut pending, grant).unwrap(), store::os_store::one_item_publication::ArtifactStoreOneItemAdvance::Progress(_)));
            }
            assert_eq!(pending.phase(), store::os_store::one_item_publication::ArtifactStoreOneItemPublicationPhase::Publishing);
            let mut permit = if case["claimFirst"].as_bool().unwrap() { Some(lease.try_claim_publication().unwrap()) } else { None };
            assert!(cancellations.cancel(&key).unwrap());
            if !case["claimFirst"].as_bool().unwrap() {
                permit = lease.try_claim_publication();
            }
            if permit.is_some() {
                assert!(matches!(presence.advance_publish_one(&mut pending, grant).unwrap(), store::os_store::one_item_publication::ArtifactStoreOneItemAdvance::Published(_, _)));
                assert!(pending.acknowledge());
            }
            drop(permit);
            assert!(lease.try_claim_publication().is_none(), "a cancelled lease can never claim a later publication unit");
            assert_eq!(serde_json::json!({ "committed": presence.generation_now() != 0 }), serde_json::json!({ "committed": case["committed"] }));
            pending.begin_close();
            for _ in 0..16 {
                if pending.terminal_is_empty() {
                    break;
                }
                let _ = pending.close_step(grant).unwrap();
            }
            assert!(pending.terminal_is_empty());
            lease.finish();
            assert_eq!(cancellations.active_operation_count(), 0);
            let mut close = presence.begin_retirement(std::sync::Arc::new(PublicationPresence::default()), |_| true).ok().unwrap();
            for _ in 0..2048 {
                if matches!(close.close_step(RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:4096,maximum_capacity_bytes:4096,maximum_release_bytes:4096,maximum_depth:64}).unwrap(),RetainedCloneStep::Complete(_)) {
                    break;
                }
            }
            assert!(close.terminal_is_empty());
        }
    }

    fn fixture_latest_wins_key(scope: &Value) -> semio_framework_value_ordered::ordered::SharedOwner<String> {
        let parts = [scope["document"].as_str().unwrap(), scope["controller"].as_str().unwrap(), scope["tool"].as_str().unwrap(), scope["target"].as_str().unwrap()];
        use semio_framework_value::retained_clone::{RetainedCloneGrant, RetainedCloneProgress, RetainedCloneStep};
        let page = RetainedCloneGrant { maximum_items:1, maximum_copy_bytes:4096, maximum_capacity_bytes:4096, maximum_release_bytes:4096, maximum_depth:semio_framework_value_ordered::ordered::MAX_AVL_HEIGHT * 5 + 16 };
        let birth = page;
        let (mut copy, _) = ToolLatestWinsKeyCopy::admit_original(scope["instance"].as_u64().unwrap() as u32, parts, birth).unwrap().unwrap();
        assert_eq!(copy.advance(parts, RetainedCloneGrant { maximum_items: 0, ..page }).unwrap(), RetainedCloneStep::Progress(RetainedCloneProgress::default()));
        assert_eq!(copy.advance(parts, RetainedCloneGrant { maximum_copy_bytes: 0, ..page }).unwrap(), RetainedCloneStep::Progress(RetainedCloneProgress::default()));
        for _ in 0..100_000 {
            match copy.advance(parts, page).unwrap() {
                RetainedCloneStep::Complete(progress) => {
                    assert!(progress.fits(page));
                    return copy.take_key(page).unwrap().unwrap().0;
                }
                RetainedCloneStep::Progress(progress) => assert!(progress.fits(page)),
            }
        }
        panic!("full-domain key did not progress under the production grant")
    }

    #[test]
    fn latest_wins_owned_key_retains_zero_and_subexact_headers() {
        use semio_framework_value::retained_clone::RetainedCloneGrant;
        use semio_framework_value_ordered::ordered::SharedOwner;
        let fixture:Value=serde_json::from_str(semio_framework_test_fixtures::fixture_text!("../../../../🧪️tests/🧫️fixtures/🥇️tool-latest-wins.json")).unwrap();
        let scope=&fixture["first"];
        let parts=[scope["document"].as_str().unwrap(),scope["controller"].as_str().unwrap(),scope["tool"].as_str().unwrap(),scope["target"].as_str().unwrap()];
        let page=RetainedCloneGrant { maximum_items:1, maximum_copy_bytes:4096, maximum_capacity_bytes:4096, maximum_release_bytes:4096, maximum_depth:semio_framework_value_ordered::ordered::MAX_AVL_HEIGHT * 5 + 16 };
        let birth=RetainedCloneGrant{maximum_capacity_bytes:parts.iter().map(|part|part.len()*2).sum::<usize>()+72,..page};
        let (mut copy,_)=ToolLatestWinsKeyCopy::admit_original(scope["instance"].as_u64().unwrap() as u32,parts,birth).unwrap().unwrap();
        for turn in 0..100_000 {if matches!(copy.advance(parts,page).unwrap(),semio_framework_value::retained_clone::RetainedCloneStep::Complete(_)) {break;}assert!(turn<99_999);}
        let pointer=copy.key.as_ref().unwrap().as_ptr();
        let expected=copy.key.as_ref().unwrap().clone();
        assert_eq!(serde_json::from_str::<String>(&serde_json::to_string(&expected).unwrap()).unwrap(),expected);
        let demand=SharedOwner::<String>::allocation_bytes();
        let at=|bytes:usize|RetainedCloneGrant{maximum_capacity_bytes:bytes,..page};
        assert!(copy.take_key(at(0)).unwrap().is_none());
        assert!(copy.take_key(at(demand-1)).unwrap().is_none());
        assert_eq!(copy.key.as_ref().unwrap().as_ptr(),pointer);
        let (mut key,_)=copy.take_key(at(demand)).unwrap().unwrap();
        assert_eq!(key.as_bytes(),expected.as_bytes());
        assert_eq!(key.as_bytes().as_ptr(),pointer);
        let mut alias=key.clone();
        assert_eq!(alias.as_bytes().as_ptr(),pointer);
        for bytes in [0,demand-1] {let step=key.release_step(RetainedCloneGrant::one_release_turn(bytes,1)).unwrap();assert_eq!(step.progress.copied_items,0);assert!(step.value.is_none());assert!(!key.terminal_is_empty());}
        let step=key.release_step(RetainedCloneGrant::one_release_turn(demand,1)).unwrap();
        assert_eq!(step.progress.released_bytes,0);assert!(step.value.is_none());assert!(key.terminal_is_empty());
        let step=alias.release_step(RetainedCloneGrant::one_release_turn(demand,1)).unwrap();
        assert_eq!(step.progress.released_bytes,demand);let text=step.value.unwrap();assert_eq!(text,expected);assert_eq!(text.as_ptr(),pointer);assert!(alias.terminal_is_empty());
    }
