/// 🎞️ Host events, media cancel and download seam: the test host events, the retained child command work, completed media cancellation and the download seam chunks.

    /// 📨️ Takes the host events recorded on this test thread.
    use semio_framework_os_store_backbone::MemoryBackbone;
use semio_framework_os_store_backbone::BackboneMessage;
fn take_test_host_events() -> Vec<HostEvent> {
        TEST_HOST_EVENTS.with(|events| std::mem::take(&mut *events.borrow_mut()))
    }

    /// 🪟️ A view of the two-window roster `main-1`/`main-2` targeting `target` under `utility`.
    fn host_event_view(target: &str, utility: Option<&str>) -> ViewModel {
        let roster = vec![ViewWindowInstance { id: "main-1".into(), window_kind_id: "main".into() }, ViewWindowInstance { id: "main-2".into(), window_kind_id: "main".into() }];
        let view = ViewModel { window_instances: roster, ..ViewModel::new(semio_framework_ui_locale::Locale::En, semio_framework_ui_locale::Terminology::Native) }.for_window_instance(target).expect("a roster window");
        ViewModel { active_utility_id: utility.map(str::to_string), ..view }
    }

    struct TestRetainedChildCommandWork {
        pub emit: Option<Emit<TestMutation, TestConfigMutation, NoDraftMutation>>,
    }

    impl semio_framework_plugin_retained_command::ArtifactCommandWork<TestApp> for TestRetainedChildCommandWork {
        fn tool_id(&self) -> &'static str {
            TEST_RETAINED_COMMAND_TOOL
        }

        fn extent(&self, _command: &TestCommand, _snapshot: &TestSnapshot, _interaction: &InteractionState, _context: Option<&ArtifactOwnedToolJobContext<TestApp>>) -> Option<usize> {
            Some(1)
        }

        fn step(&mut self, input: &semio_framework_plugin_retained_command::ArtifactCommandInputs<'_, TestApp>, _cx: &mut semio_framework_job::StepContext<'_>) -> Result<semio_framework_plugin_retained_command::ArtifactCommandWorkStep<TestApp>, semio_framework_job::OriginalTurnFailure> {
            let owner = input.snapshot_owner.expect("retained runtime supplies its immutable snapshot capability");
            assert!(std::ptr::eq(owner.as_ref(), input.snapshot));
            let fixture: serde_json::Value = serde_json::from_str(include_str!("../../../🧵️retained-command/🧪️tests/🧫️fixtures/🔒️immutable-reader.json")).expect("neutral immutable reader fixture");
            let observed = serde_json::to_value(owner.as_ref()).expect("independent snapshot projection");
            assert_eq!(observed, fixture["initial"]);
            let mut replacement = owner.clone();
            *std::sync::Arc::make_mut(&mut replacement) = serde_json::from_value(fixture["replacement"].clone()).expect("independent replacement snapshot");
            assert_eq!(serde_json::to_value(owner.as_ref()).unwrap(), fixture["initial"]);
            assert_eq!(serde_json::to_value(replacement.as_ref()).unwrap(), fixture["replacement"]);
            let semio_framework_plugin_retained_command::ArtifactCommandInputs { snapshot_owner: _, command: _command, snapshot: _snapshot, config: _config, history: _history, interaction: _interaction, hover: _hover, context: _context, operation: _operation } = *input;
            self.emit.take().map(semio_framework_plugin_retained_command::ArtifactCommandWorkStep::Complete).ok_or_else(|| Fault::from("test-retained-child-work-repeated").into())
        }
    }

    async fn test_retained_child_command_payload(completion: ArtifactToolCompletion<TestApp>, emit: Emit<TestMutation, TestConfigMutation, NoDraftMutation>) -> (semio_framework_plugin_retained_command::ArtifactRetainedCommandPayload<TestApp>,crate::job_close_fixture::FixtureSnapshotOwner) {
        let snapshot=std::sync::Arc::new(TestSnapshot::default());
        let(snapshot_read,issuer)=crate::job_close_fixture::FixtureSnapshotOwner::new(snapshot.clone(),3,[5;32]);
        let payload=semio_framework_plugin_retained_command::ArtifactRetainedCommandPayload::new(
            semio_framework_plugin_retained_command::ArtifactRetainedCommandInputs {
                command: TestCommand::SetLabel { value: "wire".into() },
                snapshot,
                snapshot_read,
                config: std::sync::Arc::new(TestConfig::default()),
                history: std::sync::Arc::new(HistoryView::empty()),
                interaction_state: std::sync::Arc::new(InteractionState::default()),
                interaction_hover: std::sync::Arc::new(InteractionHoverState::new()),
                context: None,
                operation: AppOperationContext { app_instance_id: 7, parent_document_id: "test-document".into(), operation_id: 42, generation: 3, canonical_base_revision: [5; 32], retained: TEST_COMMAND_RETAINED_GRANT, authoring_seed: "authoring-seed-test".into() },
                completion,
            },
            test_retained_command_id,
            TEST_RETAINED_COMMAND_RAW_BYTES,
            1,
            Box::new(TestRetainedChildCommandWork { emit: Some(emit) }),
        );(payload,issuer)
    }

    /// 🎯️ The intent twin of {@link ContractApp::dispatch_typed}: `handle_intent_frame` answers with the
    /// same admission receipt every migrated dispatch answers with, so a law that reads the store right
    /// after it observed the previous revision. Settles the operation and rebuilds the settled
    /// mutations/inverse through the SAME `result_from_last_edit` the direct route uses.
    async fn handle_intent_settled(app: &mut ContractApp, intent: &UiIntent, meta: &ActionMeta, identity: &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>,cx:&mut semio_framework_job::StepContext<'_>) -> Result<semio_framework::InvocationResult, semio_framework_plugin_ownership::bounded_ownership::PluginLifecycleFailure> {
        let verb = intent.action.name.to_string();
        let before_edit_id = app.0.test_last_edit_id();
        let before_tail = app.0.test_edit_tail_lengths();
        let mut admitted = PluginApp::handle_intent_frame(&mut app.0, intent, meta, identity).await?;
        let receipt = artifact_app_laws::settle_registered_typed_operation(&mut app.0, meta.instance_id, semio_framework_plugin_testing_laws::artifact_app_laws::fixture_mounted_policy(), identity,cx).await?;
        admitted.requested_effects.extend(receipt.effects);
        admitted.events.extend(receipt.events);
        if let Some(scope) = receipt.ui_scope {
            admitted.ui_scope = scope;
        }
        let after_edit_id = app.0.test_last_edit_id();
        if after_edit_id.is_some() {
            let tail_offset = if after_edit_id == before_edit_id { before_tail } else { (0, 0) };
            let settled = app.0.test_result_from_last_edit(&verb, meta, tail_offset).await;
            admitted.mutations = settled.mutations;
            admitted.inverse_group = settled.inverse_group;
            admitted.history_patch = settled.history_patch;
        }
        Ok(admitted)
    }

    #[semio_framework_async_macros::async_test]
    async fn completed_media_cancel_moves_exact_output_to_bounded_cleanup() {
        let mut original_caller=semio_framework_plugin_testing_caller::OriginalFixtureCaller::admit().expect("original builder fixture caller admission");let mut original_context=original_caller.context();let cx=&mut original_context;let result=async{
        let mounted_grant = semio_framework_value::retained_clone::RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: 32_768, maximum_capacity_bytes: 262_144, maximum_release_bytes: 1_048_576, maximum_depth: 4_096 };
        let mounted_policy = semio_framework_plugin_mounted_owner::MountedOwnerPolicyV1 { preparation: mounted_grant, maintenance: mounted_grant, close: mounted_grant };
        let identity_started = std::time::Instant::now();
        let mut identity_progress = |progress: semio_framework_value::native_encoding::NativeEncodeProgress| {
            progress.owned_bytes <= 1_048_576 && identity_started.elapsed() < std::time::Duration::from_secs(60)
        };
        let mut identity = semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority::new(1_048_576, &mut identity_progress as &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::Observer<'_>).expect("explicit original fixture identity authority");

        let mut app = contract_app_under_test(mounted_policy, &mut identity).await;
        let operation_id = semio_framework_job::allocate_operation_id_in_slot(ARTIFACT_LIVE_OUTPUT_SLOTS as u64, 0);
        let handle = ArtifactMediaExportHandle {
            app_instance_id: meta().instance_id,
            parent_document_id: app.stores.store.envelope().id.clone(),
            operation_id,
            base_revision: semio_framework_job::RevisionId(9_007_199_254_740_993),
            generation: semio_framework_job::Generation(9_007_199_254_740_995),
        };
        let chunks = ArtifactOutputChunks::new(ARTIFACT_OUTPUT_CHUNK_BYTES * 2);
        assert_eq!(chunks.push(vec![1; ARTIFACT_OUTPUT_CHUNK_BYTES]), Ok(ARTIFACT_OUTPUT_CHUNK_BYTES));
        assert_eq!(chunks.push(vec![2; 17]), Ok(ARTIFACT_OUTPUT_CHUNK_BYTES + 17));
        assert_eq!(chunks.seal(), Ok(ARTIFACT_OUTPUT_CHUNK_BYTES + 17));
        let mut donated_handle=Some(handle.clone());let mut source=Some(ArtifactMediaExportPoll::Complete(ArtifactMediaExportResult::structured(MediaType{class:MediaClass::Presentation,form:MediaForm::Sequence},"fixture.schema","audio/mpeg",chunks.clone()).expect("exact fixture media")));let mut received=None;
        assert!(ArtifactDownloadOutput::receive_media_export_original(&mut donated_handle,&mut source,&mut received,cx).expect("exact original media output"));let output=received.take().expect("funded download receiving");
        app.media.segmented_downloads.insert_admitted(operation_id.0, output);

        app.cancel_media_export(&handle).await.expect("completed output cancellation");
        assert!(!app.media.segmented_downloads.contains(operation_id.0));
        assert!(app.media.test_segmented_closure_contains(operation_id.0));
        assert_eq!(chunks.chunks_remaining(), 2, "cancellation only transfers ownership");
        assert_eq!(chunks.bytes_remaining(), ARTIFACT_OUTPUT_CHUNK_BYTES + 17, "cancellation performs no synchronous output destruction");

        app.maintenance.maintenance_stage = 2;
        assert_eq!(app_maintenance_turn(&mut *app,cx).expect("first bounded cleanup"), semio_framework_plugin_ownership::bounded_ownership::PluginLifecycleStep::Progress(store::RetainedCloneProgress { copied_items: 1, released_bytes: ARTIFACT_OUTPUT_CHUNK_BYTES, ..Default::default() }));
        assert_eq!(chunks.bytes_remaining(), 17);
        app.maintenance.maintenance_stage = 2;
        assert_eq!(app_maintenance_turn(&mut *app,cx).expect("second bounded cleanup"), semio_framework_plugin_ownership::bounded_ownership::PluginLifecycleStep::Progress(store::RetainedCloneProgress { copied_items: 1, released_bytes: 17, ..Default::default() }));
        assert_eq!(chunks.bytes_remaining(), 0);
        app.maintenance.maintenance_stage = 2;
        assert_eq!(app_maintenance_turn(&mut *app,cx).expect("terminal cleanup"), semio_framework_plugin_ownership::bounded_ownership::PluginLifecycleStep::Progress(store::RetainedCloneProgress { copied_items: 1, ..Default::default() }));
        assert!(!app.media.test_segmented_closure_contains(operation_id.0));
        artifact_app_laws::close_registered_fixture_app(&mut *app, mounted_policy,cx).expect("original contract fixture paid terminal close");

        }.await;let fuel=cx.fuel_remaining();drop(original_context);original_caller.retire(fuel).expect("original builder fixture paid terminal retirement");result
    }

    #[semio_framework_async_macros::async_test]
    async fn retained_command_download_seam_keeps_original_chunks_and_bounded_cancel_owner(){
        use semio_framework_job::{InteractiveJob,InteractiveJobCloseStep};
        use semio_framework_plugin_retained_command::{ArtifactCommandWork, ArtifactCommandWorkStep, ArtifactCommandInputs, ArtifactRetainedCommandJob};
        let fixture:serde_json::Value=serde_json::from_str(include_str!("../../../🧵️retained-command/🔄️full-operation/🧪️tests/🧫️fixtures/🔣️.json")).unwrap();let law=&fixture["download"];
        struct DownloadWork{output:Option<ArtifactDownloadOutput>,first:bool,closing:bool,retirement:Option<Box<dyn semio_framework_value::ErasedSnapshotRetirement>>}
        impl DownloadWork{fn close_demands(&self,body:usize)->Result<semio_framework_value::RetirementDemand,semio_framework_value::ValueError>{if let Some(active)=self.retirement.as_ref(){return store::os_store::snapshot_read::artifact_retirement_box_demands(active,body)}store::os_store::snapshot_read::artifact_retirement_owned_birth_demands(&self.output)}}
        impl ArtifactCommandWork<TestApp> for DownloadWork{
            fn tool_id(&self)->&'static str{TEST_RETAINED_COMMAND_TOOL}
            fn extent(&self,_:&TestCommand,_:&TestSnapshot,_:&InteractionState,_:Option<&ArtifactOwnedToolJobContext<TestApp>>)->Option<usize>{Some(1)}
            fn step(&mut self,_:&ArtifactCommandInputs<'_,TestApp>,_:&mut semio_framework_job::StepContext<'_>)->Result<ArtifactCommandWorkStep<TestApp>, semio_framework_job::OriginalTurnFailure> {
        let original: Result<_, Fault> = (|| {if self.first{self.first=false;return Ok(ArtifactCommandWorkStep::Progress{stage:"download-source",preview:b"{}"})}Ok(ArtifactCommandWorkStep::CompleteDownload{download:self.output.take().unwrap(),ephemeral:EphemeralEmit::default()})
        })();
        original.map_err(semio_framework_job::OriginalTurnFailure::from)
    }
            fn begin_close(&mut self){self.closing=true;}
            fn close_step(&mut self,cx:&mut semio_framework_job::StepContext<'_>)->Result<InteractiveJobCloseStep,semio_framework_job::OriginalTurnFailure>{
                let original=cx.retained_grant();let demand=self.close_demands(original.maximum_copy_bytes)?;
                let step=semio_framework_job::close_original_retained_step(cx,demand,|grant|{
                    if self.retirement.is_some(){store::os_store::snapshot_read::artifact_retirement_box_close_step(&mut self.retirement,grant)}
                    else{store::os_store::snapshot_read::artifact_retirement_admit_owned(&mut self.output,&mut self.retirement,grant)}
                })?;
                step.admit(original,self.terminal_is_empty()).map_err(semio_framework_job::OriginalTurnFailure::from)
            }
            fn next_close_copy_byte_demand(&self)->Result<usize,semio_framework_value::ValueError>{Ok(self.close_demands(0)?.copy_bytes)}
            fn next_close_capacity_byte_demand(&self,body:usize)->Result<usize,semio_framework_value::ValueError>{Ok(self.close_demands(body)?.capacity_bytes)}
            fn next_close_release_byte_demand(&self)->Result<usize,semio_framework_value::ValueError>{Ok(self.close_demands(0)?.release_bytes)}
            fn next_close_depth_demand(&self)->Result<usize,semio_framework_value::ValueError>{Ok(self.close_demands(0)?.depth)}
            fn terminal_is_empty(&self)->bool{self.closing&&self.output.is_none()&&self.retirement.is_none()}
        }
        let make=||{let length=law["payloadBytes"].as_u64().unwrap()as usize;assert!(length>law["wireBytes"].as_u64().unwrap()as usize);let chunks=ArtifactOutputChunks::new(length);for index in 0..length.div_ceil(ArtifactOutputChunks::CHUNK_BYTES){let start=index*ArtifactOutputChunks::CHUNK_BYTES;let end=(start+ArtifactOutputChunks::CHUNK_BYTES).min(length);chunks.push((start..end).map(|index|(index%251)as u8).collect()).unwrap();}chunks.seal().unwrap();ArtifactDownloadOutput::new("original.txt","text/plain",None,chunks).unwrap()};
        for (busy,cancel) in [(false,false),(true,false),(false,true)]{
            let completion=ArtifactToolCompletion::<TestApp>::new();let consumer=completion.clone();let (mut payload,issuer)=test_retained_command_payload(completion.clone()).await;payload.work=Box::new(DownloadWork{output:Some(make()),first:true,closing:false,retirement:None});let mut job=ArtifactRetainedCommandJob::new(payload);let mut yielded=false;let policy=semio_framework_plugin_testing_laws::artifact_app_laws::fixture_mounted_policy();let mut caller=semio_framework_job::OriginalJobFixtureCaller::admit(semio_framework_job::OperationId(41),semio_framework_job::Generation(3),crate::job_close_fixture::original_fixture_stock(),||Some(0),u64::MAX,1<<24);
            let mut decoder=HistoryView::empty();assert!(caller.turn_with_decoder(policy.preparation,&mut decoder,|cx|job.test_admit_original_source(cx)).unwrap());
            for _ in 0..128{let result=caller.turn(policy.maintenance,|cx|Ok(if busy{completion.with_busy_test_lock(||semio_framework_plugin_testing_laws::artifact_app_laws::fixture_step(&mut job,cx))}else{semio_framework_plugin_testing_laws::artifact_app_laws::fixture_step(&mut job,cx)})).unwrap();match result{semio_framework_plugin_testing_laws::artifact_app_laws::FixtureStepOutcome::Preview(_)=>{yielded=true;if cancel{caller.cancel().cancel_now()}},semio_framework_plugin_testing_laws::artifact_app_laws::FixtureStepOutcome::Checkpoint{..}=>{},semio_framework_plugin_testing_laws::artifact_app_laws::FixtureStepOutcome::Complete{..}=>break,semio_framework_plugin_testing_laws::artifact_app_laws::FixtureStepOutcome::Fault(_)=>break,semio_framework_plugin_testing_laws::artifact_app_laws::FixtureStepOutcome::Cancelled=>{assert!(cancel);break},_=>{}}}
            assert!(yielded);job.begin_close();for _ in 0..100000{if job.terminal_is_empty(){break}let grant=job_close_grant(&job);let before=caller.turn(semio_framework_value::RetainedCloneGrant{maximum_items:0,..grant},|cx|job.close_step(cx)).unwrap();assert!(matches!(before,InteractiveJobCloseStep::Pending{progress}if progress==Default::default())||matches!(before,InteractiveJobCloseStep::Blocked));let step=caller.turn(grant,|cx|job.close_step(cx)).unwrap();assert_eq!(step.progress(),caller.receipt());assert!(step.progress().fits(grant));assert!(!matches!(step,InteractiveJobCloseStep::Blocked))}assert!(job.terminal_is_empty());
            if busy||cancel{assert!(consumer.test_take_download().unwrap().is_none())}else{let (Ok(output),ephemeral)=consumer.test_take_download().unwrap().unwrap()else{panic!("original download completion")};let mut actual=Vec::new();while let Some(chunk)=output.chunks.take_chunk().unwrap(){assert!(chunk.len()<=law["chunkBytes"].as_u64().unwrap()as usize);actual.extend_from_slice(&chunk);crate::job_close_fixture::retire_fixture_owned(chunk,&mut caller)}let expected:Vec<u8>=serde_json::from_value(serde_json::json!((0..law["payloadBytes"].as_u64().unwrap()).map(|index|(index%251)as u8).collect::<Vec<_>>())).unwrap();assert_eq!(actual,expected);crate::job_close_fixture::retire_fixture_owned(output,&mut caller);crate::job_close_fixture::retire_fixture_owned(ephemeral,&mut caller);}
            crate::job_close_fixture::retire_fixture_owned(consumer,&mut caller);crate::job_close_fixture::retire_fixture_owned(completion,&mut caller);issuer.close(&mut caller);while !decoder.terminal_is_empty(){caller.turn(policy.close,|cx|{let step=decoder.close_step(cx.retained_grant())?;cx.consume_retained(step.progress())?;cx.consume_fuel(step.progress().copied_items as u64);Ok(())}).unwrap();}caller.retire(crate::job_close_fixture::original_fixture_stock());
        }
        let mut original=std::collections::VecDeque::with_capacity(law["queueSlots"].as_u64().unwrap()as usize);original.push_back("Mesh 😀".to_string());
        let payload_bytes=original[0].len();let backing_bytes=original.capacity()*std::mem::size_of::<String>();assert_eq!(std::mem::size_of::<String>(),law["vectorMetadataWords"].as_u64().unwrap()as usize*std::mem::size_of::<usize>());
        let birth=semio_framework_value::retirement::owned_retirement_birth_bytes::<std::collections::VecDeque<String>>();
        let admission=semio_framework_value::retained_clone::RetainedCloneGrant{maximum_capacity_bytes:birth,..semio_framework_plugin_testing_laws::artifact_app_laws::fixture_mounted_policy().preparation};
        let (mut retirement,admitted)=semio_framework_value::retirement::admit_owned_retirement(original,admission).unwrap_or_else(|_|panic!("original queue typed constructor refused its funded grant"));
        assert!(admitted.fits(admission));let (mut born,mut released)=(admitted.retained_capacity_bytes,admitted.released_bytes);
        for _ in 0..100000{if retirement.terminal_is_empty(){break}
            let copy=retirement.next_copy_byte_demand().unwrap();
            let grant=semio_framework_value::retained_clone::RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:copy,maximum_capacity_bytes:retirement.next_capacity_byte_demand(copy).unwrap(),maximum_release_bytes:retirement.next_release_byte_demand().unwrap(),maximum_depth:retirement.next_depth_demand().unwrap().max(1)};
            assert_eq!(retirement.close_step(semio_framework_value::retained_clone::RetainedCloneGrant{maximum_items:0,..grant}).unwrap().progress(),Default::default());
            let progress=retirement.close_step(grant).unwrap().progress();assert!(progress.fits(grant));born+=progress.retained_capacity_bytes;released+=progress.released_bytes;}
        assert!(retirement.terminal_is_empty());assert_eq!(released,payload_bytes+backing_bytes+born,"original queue backing allocation remains owned until its exact byte credits drain");

    }

    /// 📥️ Every accepted remote prefix is observed even when a later wire-valid envelope refuses.

    #[semio_framework_async_macros::async_test]

    async fn accepted_remote_prefix_notifies_before_later_refusal() {
        let mut original_caller=semio_framework_plugin_testing_caller::OriginalFixtureCaller::admit().expect("original builder fixture caller admission");let mut original_context=original_caller.context();let cx=&mut original_context;let result=async{

        let law: serde_json::Value = serde_json::from_str(include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/../../../../🧫️fixtures/📥️remote-prefix/🔣️.json"))).unwrap();

        let mounted_grant = semio_framework_value::retained_clone::RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: 32_768, maximum_capacity_bytes: 262_144, maximum_release_bytes: 1_048_576, maximum_depth: 4_096 };

        let mounted_policy = semio_framework_plugin_mounted_owner::MountedOwnerPolicyV1 { preparation: mounted_grant, maintenance: mounted_grant, close: mounted_grant };

        let mut identity = semio_framework_plugin_testing_laws::artifact_app_laws::fixture_identity();

        let mut sender = contract_app(mounted_policy, &mut identity).await;

        let (near, mut far) = MemoryBackbone::pair("mem://remote-prefix", "mem://remote-prefix").await;

        sender.attach_backbone(store::os_store::backbone::Backbones::Memory(near)).await.expect("attach sender");

        sender.dispatch_typed(TestCommand::Increment, &meta(), &mut identity,cx).await.expect("sender edit");

        let mut envelopes = Vec::new();

        for message in far.receive().await.expect("receive sender edit") {

            if let BackboneMessage::Mutations { envelopes: operations } = message {

                envelopes.extend(protocol::decode_envelopes(&operations).expect("sender envelopes"));

            }

        }

        assert_eq!(envelopes.len(), 1, "fixture owns one actual authored operation");

        let valid = envelopes.pop().unwrap();

        for row in law["cases"].as_array().unwrap() {

            let mut receiver = contract_app(mounted_policy, &mut identity).await;

            let under = ActionMeta { view_state: Some(host_event_view("main-1", Some("select"))), ..meta() };

            receiver.dispatch_typed(TestCommand::ViewNoScope, &under, &mut identity,cx).await.expect("admitted window context");

            take_test_host_events();

            let batch: Vec<_> = row["steps"].as_array().unwrap().iter().map(|step| {

                let mut envelope = valid.clone();

                if step == "malformed" { envelope.diff.payload = vec![0xff]; }

                envelope

            }).collect();

            let result = receiver.ingest_operations(&protocol::encode_envelopes(&batch), &mut identity).await;

            assert_eq!(result.is_err(), row["refused"].as_bool().unwrap(), "{}", row["name"]);

            assert_eq!(receiver.test_snapshot().await.count, row["count"].as_i64().unwrap() as i32, "{}", row["name"]);

            let events = take_test_host_events();

            let expected = row["notifications"].as_u64().unwrap() as usize;

            let base_moved: Vec<_> = events.into_iter().filter(|event| matches!(event, HostEvent::BaseMoved { .. })).collect();

            assert_eq!(base_moved.len(), expected * 2, "{}", row["name"]);

            if expected > 0 { assert_eq!(base_moved, vec![HostEvent::BaseMoved { window_id: "main-1".into() }, HostEvent::BaseMoved { window_id: "main-2".into() }]); }

            close_reserved_app(&mut receiver,mounted_policy,cx);
            eprintln!("[DEBUG] remote prefix case={} committed={} base_notifications={}", row["name"], row["count"], base_moved.len());

        }

        close_reserved_app(&mut sender,mounted_policy,cx);
        }.await;let fuel=cx.fuel_remaining();drop(original_context);original_caller.retire(fuel).expect("original builder fixture paid terminal retirement");result
    }
