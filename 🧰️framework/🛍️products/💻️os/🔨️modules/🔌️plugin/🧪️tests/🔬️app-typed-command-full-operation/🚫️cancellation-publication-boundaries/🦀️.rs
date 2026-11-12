    /// 🚫️ Cancellation publication boundaries: the retained cancellation publication boundaries and the owned latest-wins key with its zero and sub-exact headers.

    use semio_plugin_instance_app::VcsArtifactApp;
use semio_plugin_decl_emission::Emit;
use semio_plugin_decl_app::app_emission::EphemeralEmit;
use semio_plugin_decl_views::child_view::ChildContentView;
use semio_plugin_composition_publication::MountedTypedCommandFullOperationStage;
use semio_plugin_composition_publication::PendingArtifactStorePublication;
use semio_plugin_operation::ToolCancellationHandle;
use semio_plugin_command::command_admission::original_latest_key::ToolLatestWinsKeyCopy;
use semio_plugin_decl_app::tool_contract::ArtifactToolPublicationLane;
use semio_plugin_decl_app::tool_contract::ArtifactToolCompletionValue;
fn advance_cancellation_presence(
    presence: &mut store::PresenceStore<PublicationPresence, PublicationPresenceMutation>,
    pending: &mut store::ArtifactEphemeralOneItemPublication<PublicationPresence, PublicationPresenceMutation>,
    grant: store::os_store::one_item::ArtifactStoreOneItemGrant,
    cx: &mut semio_framework_job::StepContext<'_>,
) -> store::os_store::one_item_publication::ArtifactStoreOneItemAdvance {
    let mut output = None;
    let mut receiving = None;
    cx.with_retained_loan(grant.retained_grant(), |same| {
        let ran = same.with_native_receiving(&mut receiving, |native, issued| {
            output = Some(presence.advance_publish_one(pending, store::os_store::one_item::ArtifactStoreOneItemGrant::from_retained(issued), native));
            Ok(())
        })?;
        assert!(ran, "the original receiving scope was admitted");
        Ok(())
    }).unwrap();
    output.unwrap().expect("the presence publication advances")
}

fn close_cancellation_presence_publication(
    pending: &mut store::ArtifactEphemeralOneItemPublication<PublicationPresence, PublicationPresenceMutation>,
    grant: store::os_store::one_item::ArtifactStoreOneItemGrant,
    cx: &mut semio_framework_job::StepContext<'_>,
) {
    cx.with_retained_loan(grant.retained_grant(), |same| {
        same.with_original_retirement(|same| {
            let issued = same.retained_grant();
            let body = issued.maximum_copy_bytes;
            let outcome = same.with_original_effect(issued, |same, effect| same.with_native_retirement(|control| Ok(pending.close_step_controlled(body, effect, control)))).map_err(semio_framework_job::OriginalTurnFailure::from)?;
            let Some((result, performed)) = outcome else { return Ok(()) };
            same.consume_fuel(u64::try_from(performed).unwrap_or(u64::MAX));
            if let semio_framework_value::RetainedWorkPoll::Complete(turn) = result? { turn.result?; }
            Ok(())
        })
    }).unwrap();
}

pub(super) async fn retained_cancellation_publication_boundaries<A: ArtifactApp<Presence = PublicationPresence, PresenceMutation = PublicationPresenceMutation> + Default>() {
        let mut original_scope = artifact_app_laws::OriginalFixtureScope::admit();
        let cx = original_scope.cx();
        let fixture: Value = serde_json::from_str(semio_framework_test_fixtures::fixture_text!("../../../../🧪️tests/🧫️fixtures/🥇️tool-latest-wins.json")).expect("language-neutral cancellation boundaries");
        let bytes = fixture["maximumBytes"].as_u64().unwrap() as usize;
        let grant = store::os_store::one_item::ArtifactStoreOneItemGrant { maximum_items: fixture["maximumItems"].as_u64().unwrap() as usize, maximum_copy_bytes: bytes, maximum_capacity_bytes: bytes, maximum_release_bytes: bytes, maximum_depth: 128 };
        assert_eq!(grant.maximum_items, 1);
        assert_eq!(bytes, TYPED_OPERATION_RESULT_PAGE_BYTES);
        for case in fixture["publicationCases"].as_array().unwrap() {
            let boundary = case["cancelAt"].as_str().unwrap();
            let mounted_grant=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:32_768,maximum_capacity_bytes:262_144,maximum_release_bytes:1_048_576,maximum_depth:4_096};
        let mounted_policy=semio_plugin_mounted_owner::MountedOwnerPolicyV1{preparation:mounted_grant,maintenance:mounted_grant,close:mounted_grant};
        let identity_started=std::time::Instant::now();
        let mut identity_progress=|progress:semio_framework_value::native_encoding::NativeEncodeProgress|{progress.owned_bytes<=1_048_576&&identity_started.elapsed()<std::time::Duration::from_secs(60)};
        let mut identity=semio_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority::new(1_048_576,&mut identity_progress as &mut semio_os_kernel::os_vcs::io::binary::entity_identity::control::Observer<'_>).expect("original typed fixture supplied identity authority");
        let mut app = VcsArtifactApp::<A>::new(A::default(), protocol::ActorId(protocol::LOCAL_ACTOR_ID.into()),mounted_policy,&mut identity).await.expect("the reference snapshot births");
            let revision = app.stores.store.content_revision_now();
            let operation = semio_framework_job::Operation::new(
                semio_framework_job::allocate_operation_id(),
                semio_framework_job::RevisionId(u64::from_be_bytes(revision[..8].try_into().unwrap())),
                semio_framework_job::Generation(app.stores.store.generation_now()),
                17,
            );
            artifact_app_laws::ensure_fixture_cancellation_issuer(&mut app, cx).unwrap();
            let key = ToolOperationKey { app_instance_id: 7, document: ArtifactDocumentAuthority(7), operation_id: operation.operation, base_revision: operation.base_revision, generation: operation.generation };
            let mut lease = None;
            for _ in 0..artifact_app_laws::OriginalFixtureCaller::maximum_turns() {
                lease = app.tools.tool_cancellations.begin(&key, cx).unwrap();
                if lease.is_some() { break; }
            }
            let lease = lease.expect("original publication cancellation lease admitted");
            let before = app.stores.presence_store.local_read().unwrap();
            let mut mounted = MountedTypedCommandFullOperation::<A> { reserved_construction: Default::default(),
                    source_identity: semio_framework_job::OriginalSourceBinding::borrow(cx),
                    original_emission_caller: None,
                    original_emission_decoder: None,
                    original_emission_body: None,
                verb: "setGraphParameter".into(),
                meta: ActionMeta { expected_document_revision:None, actor: "fixture".into(), instance_id: 7, view_state: None },
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
                tool_stage:semio_plugin_retained_command::typed_operation::OriginalToolSettlementStage::Capture,tool_press_result:None,pending_artifact_publication:None,publication_failure:Default::default(), pending_stream_abort:None,stream_abort_resident:Default::default(), stream_abort_completed:false, pending_publication_outcome:PendingPublicationOutcome::new(),pending_window_config_receipt:None,cancellation_retirement:None,
                pending_child_publication: None,
                owned_child_group: None,
                owned_child_committed: false,
                owned_child_result_pending: false,
                captured_child_content: Some(semio_framework_value::retirement::arc_custody::original::SharedOriginalStorage::from_cold(ChildContentView::EMPTY)),
                captured_child_content_generation: 0,
                page_publication:semio_plugin_retained_command::typed_operation::MountedPagePublication::new(),result_page:None, diagnostics:None,
                result_page_presented: false,
                result_sequence: 0,
                publication_progress: 0,
                publication_checkpoint: None, publication_ownership_progress: None, original_retirement_receipt: None, actor_capture: None,actor_capture_pending:semio_plugin_mounted_owner::actor_capture::ActorCapture::new(),
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
                    publish_mounted_fixture_unit(&mut app, &mut mounted, semio_plugin_testing_laws::artifact_app_laws::fixture_mounted_policy().maintenance, cx);
                }
                let Some(PendingArtifactStorePublication::Presence(pending)) = mounted.pending_artifact_publication.as_ref() else {
                    panic!("exact pending presence owner");
                };
                assert_eq!(pending.phase(), target);
            }
            mounted.cancellation_lease.as_ref().unwrap().cancel();
            if boundary == "awaitingAck" {
                let mut page = present_mounted_page(&mut mounted, semio_plugin_testing_laws::artifact_app_laws::fixture_mounted_policy().preparation, cx).unwrap();
                assert!(!mounted.reject_cancelled_publication().unwrap());
                assert_eq!(mounted.result_page.as_ref().unwrap().token, page.token);
                assert_eq!(mounted.result_page.as_ref().unwrap().bytes(), page.bytes());
                assert!(mounted.acknowledge_result_page(crate::app_testing::artifact_app_laws::close_fixture_result_page(&mut page,crate::app_testing::artifact_app_laws::fixture_mounted_policy().close)).unwrap());
                retire_acknowledged_mounted_page(&mut mounted, semio_plugin_testing_laws::artifact_app_laws::fixture_mounted_policy().close, cx);
            }
            publish_mounted_fixture_unit(&mut app, &mut mounted, semio_plugin_testing_laws::artifact_app_laws::fixture_mounted_policy().maintenance, cx);
            assert_eq!(mounted.stage, MountedTypedCommandFullOperationStage::AwaitingAck);
            let mut cancelled = present_mounted_page(&mut mounted, mounted_policy.preparation, cx).expect("cancelled result page presented");
            assert_eq!(cancelled.lane, TypedOperationResultLane::Fault);
            assert!(std::str::from_utf8(cancelled.bytes()).unwrap().contains("cancelled"));
            assert_eq!(serde_json::json!({ "committed": app.stores.presence_store.generation_now() != 0 }), serde_json::json!({ "committed": case["committed"] }));
            assert_eq!(std::ptr::eq(before.get(), app.stores.presence_store.local()), !case["committed"].as_bool().unwrap());
            assert!(!mounted.ui_pending);
            let token = artifact_app_laws::close_fixture_result_page(&mut cancelled, mounted_policy.close);
            assert!(mounted.acknowledge_result_page(token).unwrap());
            retire_acknowledged_mounted_page(&mut mounted, mounted_policy.close, cx);
            close_mounted(&mut mounted, boundary, mounted_policy.close, cx);
            assert!(mounted.terminal_is_empty(), "{boundary}");
            assert_eq!(app.tools.tool_cancellations.active_operation_count(), 0);
            drop(before);
            close_app(&mut app, boundary, cx);
            assert!(app.close_terminal_is_empty(), "{boundary}");
        }
        for case in fixture["linearizationCases"].as_array().unwrap() {
            let mut cancellations = KeyedCancellationFixture::open();
            let key = ToolOperationKey {
                app_instance_id: 7,
                document: ArtifactDocumentAuthority(7),
                operation_id: semio_framework_job::allocate_operation_id(),
                base_revision: semio_framework_job::RevisionId(3),
                generation: semio_framework_job::Generation(0),
            };
            let mut lease = cancellations.begin(key.clone());
            let mut presence = store::PresenceStore::<PublicationPresence, PublicationPresenceMutation>::new(PublicationPresence::default());
            let factory: std::sync::Arc<dyn store::SnapshotRetirementFactory<PublicationPresence>> = std::sync::Arc::new(semio_framework_value::SharedValueRetirementFactory::<PublicationPresence>::default());
            presence.install_local_retirement_factory(factory.clone()).unwrap();
            let mut pending = presence.begin_publish_one(key.operation_id, 0, ChangePublicationPresence { revision: 1 }.into(), Some(&TwoTurnPublicationPresencePreparationFactory), Some(factory.clone())).unwrap();
            for _ in 0..8 {
                if pending.phase() == store::os_store::one_item_publication::ArtifactStoreOneItemPublicationPhase::Publishing {
                    break;
                }
                assert!(matches!(advance_cancellation_presence(&mut presence, &mut pending, grant, cancellations.scope.cx()), store::os_store::one_item_publication::ArtifactStoreOneItemAdvance::Progress(_) | store::os_store::one_item_publication::ArtifactStoreOneItemAdvance::PreparationProgress(_, _)));
            }
            assert_eq!(pending.phase(), store::os_store::one_item_publication::ArtifactStoreOneItemPublicationPhase::Publishing);
            let mut permit = if case["claimFirst"].as_bool().unwrap() { Some(lease.try_claim_publication().unwrap()) } else { None };
            assert!(cancellations.handle().cancel(&key).unwrap());
            if !case["claimFirst"].as_bool().unwrap() {
                permit = lease.try_claim_publication();
            }
            if permit.is_some() {
                assert!(matches!(advance_cancellation_presence(&mut presence, &mut pending, grant, cancellations.scope.cx()), store::os_store::one_item_publication::ArtifactStoreOneItemAdvance::Published(_, _)));
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
                close_cancellation_presence_publication(&mut pending, grant, cancellations.scope.cx());
            }
            assert!(pending.terminal_is_empty());
            lease.finish();
            cancellations.close_lease(lease);
            assert_eq!(cancellations.handle().active_operation_count(), 0);
            let mut close = presence.begin_retirement(std::sync::Arc::new(PublicationPresence::default()), |_| true).ok().unwrap();
            for _ in 0..2048 {
                if cancellations.scope.cx().with_retained_loan(RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:4096,maximum_capacity_bytes:4096,maximum_release_bytes:4096,maximum_depth:64}, |same| { let issued = same.retained_grant(); let outcome = same.with_original_effect(issued, |_, effect| { let result = close.close_step(effect); semio_framework_value::RetirementPollExit::settle(effect, result) }).map_err(semio_framework_job::OriginalTurnFailure::from)?; let Some((poll, performed)) = outcome else { return Ok(false) }; same.consume_fuel(u64::try_from(performed).unwrap_or(u64::MAX)); let semio_framework_value::RetainedWorkPoll::Complete(turn) = poll else { return Ok(false) }; let step = turn.result.map_err(semio_framework_job::OriginalTurnFailure::from)?; Ok(matches!(step, RetainedCloneStep::Complete(_))) }).unwrap() {
                    break;
                }
            }
            assert!(close.terminal_is_empty());
            cancellations.close();
        }
    }

    fn fixture_latest_wins_key(scope: &Value) -> semio_framework_value_ordered::ordered::SharedOwner<String> {
        let parts = [scope["document"].as_str().unwrap(), scope["controller"].as_str().unwrap(), scope["tool"].as_str().unwrap(), scope["target"].as_str().unwrap()];
        use semio_framework_value::retained_clone::{RetainedCloneGrant, RetainedCloneProgress, RetainedCloneStep};
        let page = RetainedCloneGrant { maximum_items:1, maximum_copy_bytes:4096, maximum_capacity_bytes:4096, maximum_release_bytes:4096, maximum_depth:semio_framework_value_ordered::ordered::MAX_AVL_HEIGHT * 5 + 16 };
        let mut caller = semio_plugin_testing_laws::OriginalFixtureCaller::admit().expect("original latest-wins key caller");
        let mut context = caller.context();
        let cx = &mut context;
        let (fresh, fresh_grant) = (semio_plugin_testing_laws::OriginalFixtureCaller::turn_fuel(), cx.retained_grant());
        let renew = |cx: &mut semio_framework_job::StepContext<'_>| semio_plugin_testing_laws::artifact_app_laws::fixture_renew_turn(cx, fresh, fresh_grant).unwrap();
        let instance = scope["instance"].as_u64().unwrap() as u32;
        let mut copy = None;
        let mut turns = 0;
        while !ToolLatestWinsKeyCopy::admit(&mut copy, instance, parts, cx).unwrap() { turns += 1; assert!(turns < 100_000, "full-domain key admission never funded"); renew(cx); }
        let mut copy = copy.expect("admitted key copy");
        while !copy.ready() { turns += 1; assert!(turns < 100_000, "full-domain key did not progress under the production grant"); copy.advance(parts, cx).unwrap(); renew(cx); }
        let mut key = None;
        while !copy.take_key(&mut key, cx).unwrap() { turns += 1; assert!(turns < 100_000, "full-domain key was never taken"); renew(cx); }
        while !copy.close_step(cx).unwrap() { turns += 1; assert!(turns < 100_000, "full-domain key copy never closed"); renew(cx); }
        let fuel = cx.fuel_remaining();
        drop(context);
        caller.retire(fuel).expect("original latest-wins key caller paid retirement");
        return key.expect("full-domain key taken");

    }

    #[test]
    fn latest_wins_owned_key_retains_zero_and_subexact_headers() {
        use semio_framework_value::retained_clone::RetainedCloneGrant;
        use semio_framework_value::retirement::arc_custody::original::SharedOriginalRelease as SharedRelease;
        let fixture:Value=serde_json::from_str(semio_framework_test_fixtures::fixture_text!("../../../../🧪️tests/🧫️fixtures/🥇️tool-latest-wins.json")).unwrap();
        let scope=&fixture["first"];
        let parts=[scope["document"].as_str().unwrap(),scope["controller"].as_str().unwrap(),scope["tool"].as_str().unwrap(),scope["target"].as_str().unwrap()];
        let mut caller = semio_plugin_testing_laws::OriginalFixtureCaller::admit().expect("original latest-wins key caller");
        let mut context = caller.context();
        let cx = &mut context;
        let (fresh, fresh_grant) = (semio_plugin_testing_laws::OriginalFixtureCaller::turn_fuel(), cx.retained_grant());
        let renew = |cx: &mut semio_framework_job::StepContext<'_>| semio_plugin_testing_laws::artifact_app_laws::fixture_renew_turn(cx, fresh, fresh_grant).unwrap();
        let mut copy = None;
        let mut turns = 0;
        while !ToolLatestWinsKeyCopy::admit(&mut copy, scope["instance"].as_u64().unwrap() as u32, parts, cx).unwrap() { turns += 1; assert!(turns < 100_000, "key admission never funded"); renew(cx); }
        let mut copy = copy.expect("admitted key copy");
        while !copy.ready() { turns += 1; assert!(turns < 100_000, "key did not progress"); copy.advance(parts, cx).unwrap(); renew(cx); }
        let pointer=copy.key.as_ref().unwrap().as_ptr();
        let expected=copy.key.as_ref().unwrap().clone();
        let demand=ToolLatestWinsKeyCopy::take_demands().unwrap();
        let exact=semio_framework_value_testing_admission::demand_grant(demand);
        for denied in [RetainedCloneGrant{maximum_items:0,..exact},RetainedCloneGrant{maximum_capacity_bytes:demand.capacity_bytes-1,..exact},RetainedCloneGrant{maximum_copy_bytes:demand.copy_bytes.saturating_sub(1),..exact}] {
            if denied==exact {continue;}
            let mut taken=None;
            assert!(!cx.with_retained_loan(denied,|same|copy.take_key(&mut taken,same)).unwrap());
            assert!(taken.is_none());
            assert_eq!(copy.key.as_ref().unwrap().as_ptr(),pointer);
        }
        let mut key=None;
        while !cx.with_retained_loan(exact,|same|copy.take_key(&mut key,same)).unwrap() { turns += 1; assert!(turns < 100_000, "key was never taken"); renew(cx); }
        let mut key=key.expect("taken key");
        assert_eq!(key.as_bytes(),expected.as_bytes());
        assert_eq!(key.as_bytes().as_ptr(),pointer);
        let mut alias=key.clone();
        assert_eq!(alias.as_bytes().as_ptr(),pointer);
        let release=semio_framework_value_testing_admission::demand_grant(key.retire_demand().unwrap());
        for denied in [RetainedCloneGrant{maximum_items:0,..release},RetainedCloneGrant{maximum_depth:release.maximum_depth-1,..release},RetainedCloneGrant{maximum_copy_bytes:release.maximum_copy_bytes-1,..release}] {
            let turn=key.retire_step(denied,4);assert_eq!(turn.work,0);assert!(turn.result.unwrap().is_none());assert!(!key.terminal_is_empty());
        }
        let turn=key.retire_step(release,4);assert_eq!(turn.work,4);let (released,progress)=turn.result.unwrap().expect("alias election funded");assert!(progress.fits(release));assert!(matches!(released,SharedRelease::Alias));assert!(key.terminal_is_empty());
        let turn=alias.retire_step(release,4);assert_eq!(turn.work,3);let (released,progress)=turn.result.unwrap().expect("last election funded");assert!(progress.fits(release));
        let SharedRelease::Original(mut owner)=released else{panic!("the last alias elects the original owner")};
        let text=owner.take_value().expect("unique original payload");assert_eq!(text,expected);assert_eq!(text.as_ptr(),pointer);assert!(alias.terminal_is_empty());
        while !copy.close_step(cx).unwrap() { turns += 1; assert!(turns < 100_000, "key copy never closed"); renew(cx); }
        let fuel = cx.fuel_remaining();
        drop(context);
        caller.retire(fuel).expect("original latest-wins key caller paid retirement");
    }
