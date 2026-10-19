/// 🧯️ Retained proof and app maintenance: the retained factory proof, the copied app type refusal, late envelope field returns and completed envelope results kept through app maintenance and close.

    #[semio_framework_async_macros::async_test]
    async fn retained_factory_proof_requires_the_exact_registered_runtime_authority() {
        test_retained_factory_proof_join::<TestApp, TestRetainedCommandFactory, OtherTestRetainedCommandFactory, CopyDrawApp>(contract_registry().await, TEST_RETAINED_COMMAND_CONTROLLER, TEST_RETAINED_COMMAND_TOOL, TestRetainedCommandFactory::new());
    }

    #[semio_framework_async_macros::async_test]
    async fn copied_app_type_cannot_inherit_same_controller_schema_and_id_proof() {
        let mut owner = contract_registry().await;
        let mut declaration = owner.actions.get("noopMutation").expect("fixture action").clone();
        declaration.semantics.execution.interactive_job = InteractiveJobClassification::Migrated;
        owner.actions.insert("canvasPointerDown".into(), declaration);
        owner.controller_id = "s.draw.draw@1/*#editor".into();
        assert!(test_unregistered_tool_job_admission_rejected::<CopyDrawApp>(&owner, &["canvasPointerDown"]));

        let audited_draw_row =
            ArtifactBoundedFirstStepProof::new::<TestApp>("owner.rs", "s.draw.draw@1/*#editor", BOUNDED_FIRST_STEP_FACTORY, "canvasPointerDown", "draw.document", ToolExecutionContract::bounded_first_step(8_192, 32, 32, 16_384, 7_500));
        let audited_draw_proof = QualifiedBoundedFirstStepProof { owner: audited_draw_row.owner, row: audited_draw_row };
        assert!(TypedCommandFullOperationJobFactory::<CopyDrawApp>::from_proof(audited_draw_proof).is_none());
    }

    #[semio_framework_async_macros::async_test]
    async fn app_maintenance_reclaims_late_envelope_field_returns_before_close_terminal() {
        let mut original_caller=semio_framework_plugin_testing_caller::OriginalFixtureCaller::admit().expect("original builder fixture caller admission");let mut original_context=original_caller.context();let cx=&mut original_context;let result=async{
        let mounted_grant = semio_framework_value::retained_clone::RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: 32_768, maximum_capacity_bytes: 262_144, maximum_release_bytes: 1_048_576, maximum_depth: 4_096 };
        let mounted_policy = semio_framework_plugin_mounted_owner::MountedOwnerPolicyV1 { preparation: mounted_grant, maintenance: mounted_grant, close: mounted_grant };
        let identity_started = std::time::Instant::now();
        let mut identity_progress = |progress: semio_framework_value::native_encoding::NativeEncodeProgress| {
            progress.owned_bytes <= 1_048_576 && identity_started.elapsed() < std::time::Duration::from_secs(60)
        };
        let mut identity = semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority::new(1_048_576, &mut identity_progress as &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::Observer<'_>).expect("explicit original fixture identity authority");

        struct ReturnedDecoder {
            terminal: bool,
            drops: std::sync::Arc<std::sync::atomic::AtomicUsize>,
        }

        impl store::os_store::owned_schema_decode::ArtifactEnvelopeFieldDecoder<TestSnapshot, TestMutation> for ReturnedDecoder {
            fn accept_field_token(
                &mut self,
                _field_id: u16,
                _token: store::os_store::owned_schema_decode::OwnedSchemaToken,
                _terminal: bool,
                _source: &store::os_store::owned_schema_decode::OwnedSchemaRecordCursor,
                _cx: &mut semio_framework_job::StepContext<'_>,
            ) -> Result<store::os_store::owned_schema_decode::ArtifactEnvelopeFieldDecodeStep, store::os_store::owned_schema_decode::OwnedSchemaDecodeDiagnostic> {
                Ok(store::os_store::owned_schema_decode::ArtifactEnvelopeFieldDecodeStep::Pending)
            }

            fn finish_record(&mut self, _cx: &mut semio_framework_job::StepContext<'_>) -> Result<store::os_store::owned_schema_decode::ArtifactEnvelopeFieldDecodeStep, store::os_store::owned_schema_decode::OwnedSchemaDecodeDiagnostic> {
                Ok(store::os_store::owned_schema_decode::ArtifactEnvelopeFieldDecodeStep::Pending)
            }

            fn maximum_close_byte_demand(&self) -> usize {
                0
            }

            fn maximum_retained_close_bytes(&self) -> usize {
                0
            }

            fn next_close_copy_byte_demand(&self) -> Result<usize, store::os_store::owned_schema_decode::OwnedSchemaDecodeDiagnostic> {
                Ok(0)
            }

            fn next_close_capacity_byte_demand(&self, _maximum_copy_bytes: usize) -> Result<usize, store::os_store::owned_schema_decode::OwnedSchemaDecodeDiagnostic> {
                Ok(0)
            }

            fn next_close_release_byte_demand(&self) -> Result<usize, store::os_store::owned_schema_decode::OwnedSchemaDecodeDiagnostic> {
                Ok(0)
            }

            fn next_close_depth_demand(&self) -> Result<usize, store::os_store::owned_schema_decode::OwnedSchemaDecodeDiagnostic> {
                Ok(0)
            }

            fn close_step(&mut self, grant: store::RetainedCloneGrant) -> Result<store::RetainedCloneStep, store::os_store::owned_schema_decode::OwnedSchemaDecodeDiagnostic> {
                if grant.maximum_items == 0 {
                    return Ok(store::RetainedCloneStep::Progress(Default::default()));
                }
                self.terminal = true;
                Ok(store::RetainedCloneStep::Complete(Default::default()))
            }

            fn terminal_is_empty(&self) -> bool {
                self.terminal
            }
        }

        impl Drop for ReturnedDecoder {
            fn drop(&mut self) {
                self.drops.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
            }
        }

        let mut app = contract_app_raw(mounted_policy, &mut identity).await;
        let drops = std::sync::Arc::new(std::sync::atomic::AtomicUsize::new(0));
        let lease = app.ingest.envelope_field_decoders.try_admit(Box::new(ReturnedDecoder { terminal: false, drops: drops.clone() })).unwrap_or_else(|_| panic!("app decoder return registry admits one exact owner"));
        let ticket = lease.ticket();
        drop(lease);
        assert_eq!(drops.load(std::sync::atomic::Ordering::SeqCst), 0);

        for _ in 0..4 {
            app.maintenance.maintenance_stage = 12;
            let _ = app_maintenance_turn(&mut app,cx).expect("production maintenance pumps one returned decoder owner");
        }
        assert!(app.ingest.envelope_field_decoders.ticket_reclaimed(ticket));
        assert!(app.ingest.envelope_field_decoder_retirements.is_empty());
        assert!(app.ingest.envelope_field_decoders.terminal_is_empty());
        assert_eq!(drops.load(std::sync::atomic::Ordering::SeqCst), 1);
        assert_eq!(app.ladder_ctx().drive_envelope_field_decoder_returns(semio_framework_value::retained_clone::RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:store::os_store::owned_schema_decode::ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES,maximum_capacity_bytes:0,maximum_release_bytes:store::os_store::owned_schema_decode::ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES,maximum_depth:64},true).expect("close hierarchy observes exact terminal state"), PluginLifecycleStep::Complete(Default::default()));
        artifact_app_laws::close_registered_fixture_app(&mut app, mounted_policy,cx).expect("original contract fixture paid terminal close");

        }.await;let fuel=cx.fuel_remaining();drop(original_context);original_caller.retire(fuel).expect("original builder fixture paid terminal retirement");result
    }

    /// 🔢️ Swept over every declared stage instead of a frozen stage index: which ordinal owns the
    /// completed-record lane is an implementation detail that moves whenever a stage is added, while
    /// the law — ordinary maintenance never retires an unconsumed completed output — is not.
    #[semio_framework_async_macros::async_test]
    async fn app_maintenance_and_close_retain_completed_envelope_results_until_terminal_empty() {
        let mut original_caller=semio_framework_plugin_testing_caller::OriginalFixtureCaller::admit().expect("original builder fixture caller admission");let mut original_context=original_caller.context();let cx=&mut original_context;let result=async{
        let mounted_grant = semio_framework_value::retained_clone::RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: 32_768, maximum_capacity_bytes: 262_144, maximum_release_bytes: 1_048_576, maximum_depth: 4_096 };
        let mounted_policy = semio_framework_plugin_mounted_owner::MountedOwnerPolicyV1 { preparation: mounted_grant, maintenance: mounted_grant, close: mounted_grant };
        let identity_started = std::time::Instant::now();
        let mut identity_progress = |progress: semio_framework_value::native_encoding::NativeEncodeProgress| {
            progress.owned_bytes <= 1_048_576 && identity_started.elapsed() < std::time::Duration::from_secs(60)
        };
        let mut identity = semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority::new(1_048_576, &mut identity_progress as &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::Observer<'_>).expect("explicit original fixture identity authority");

        struct CompletedRecordSentinel {
            remaining: usize,
            terminal: bool,
            drops: std::sync::Arc<std::sync::atomic::AtomicUsize>,
        }

        impl store::os_store::owned_schema_decode::ArtifactEnvelopeCompletedRecord<TestSnapshot, TestMutation> for CompletedRecordSentinel {
            fn try_publish_to(&mut self, _target: &mut dyn store::os_store::owned_schema_decode::ArtifactEnvelopeCompletedRecordTarget<TestSnapshot, TestMutation>) -> bool {
                false
            }

            fn close_step(&mut self, grant: store::RetainedCloneGrant) -> Result<store::RetainedCloneStep, semio_framework_value::ValueError> {
                if grant.maximum_items == 0 {
                    return Ok(store::RetainedCloneStep::Progress(Default::default()));
                }
                if self.remaining != 0 {
                    self.remaining -= 1;
                    return Ok(store::RetainedCloneStep::Progress(store::RetainedCloneProgress { copied_items: 1, ..Default::default() }));
                }
                self.terminal = true;
                Ok(store::RetainedCloneStep::Complete(Default::default()))
            }

            fn next_close_copy_byte_demand(&self) -> Result<usize, semio_framework_value::ValueError> {
                Ok(0)
            }

            fn next_close_capacity_byte_demand(&self, _maximum_copy_bytes: usize) -> Result<usize, semio_framework_value::ValueError> {
                Ok(0)
            }

            fn next_close_release_byte_demand(&self) -> Result<usize, semio_framework_value::ValueError> {
                Ok(0)
            }

            fn next_close_depth_demand(&self) -> Result<usize, semio_framework_value::ValueError> {
                Ok(0)
            }

            fn terminal_is_empty(&self) -> bool {
                self.terminal
            }
        }

        impl Drop for CompletedRecordSentinel {
            fn drop(&mut self) {
                assert!(self.terminal);
                self.drops.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
            }
        }

        let mut app = contract_app_raw(mounted_policy, &mut identity).await;
        let drops = std::sync::Arc::new(std::sync::atomic::AtomicUsize::new(0));
        let owner: Box<dyn store::os_store::owned_schema_decode::ArtifactEnvelopeCompletedRecord<TestSnapshot, TestMutation>> = Box::new(CompletedRecordSentinel { remaining: 2, terminal: false, drops: drops.clone() });
        let ticket = match app.ingest.envelope_completed_records.try_admit(owner) {
            Ok(ticket) => ticket,
            Err((_fault, mut owner)) => {
                while !owner.terminal_is_empty() {
                    let _ = owner.close_step(store::RetainedCloneGrant { maximum_items: 1, ..Default::default() }).expect("rejected completed owner bounded close");
                }
                panic!("app completed-record registry must admit one exact owner")
            }
        };
        assert_eq!(drops.load(std::sync::atomic::Ordering::SeqCst), 0);
        for stage in 0..MAINTENANCE_STAGES {
            app.maintenance.maintenance_stage = stage;
            let _ = app_maintenance_turn(&mut app,cx).expect("ordinary maintenance preserves unconsumed completed output");
        }
        assert_eq!(drops.load(std::sync::atomic::Ordering::SeqCst), 0);
        app.ingest.envelope_completed_records.try_request_close(ticket).expect("cancelled consumer hands exact completed output to maintenance");
        for _ in 0..6 {
            for stage in 0..MAINTENANCE_STAGES {
                app.maintenance.maintenance_stage = stage;
                let _ = app_maintenance_turn(&mut app,cx).expect("production maintenance pumps one completed-record owner");
            }
        }
        match app.ingest.envelope_completed_records.try_detach(ticket) {
            Err(fault) => assert_eq!(fault, store::os_store::owned_schema_decode::ArtifactEnvelopeCompletedRecordFault::Stale),
            Ok(mut owner) => {
                while !owner.terminal_is_empty() {
                    let _ = owner.close_step(store::RetainedCloneGrant { maximum_items: 1, ..Default::default() }).expect("unexpected completed owner bounded close");
                }
                panic!("maintenance did not reclaim the exact completed record")
            }
        }
        assert!(app.ingest.envelope_completed_record_retirements.is_empty());
        assert!(app.ingest.envelope_completed_records.terminal_is_empty());
        assert_eq!(drops.load(std::sync::atomic::Ordering::SeqCst), 1);
        assert_eq!(app.ladder_ctx().drive_envelope_completed_record_returns(semio_framework_value::retained_clone::RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:store::os_store::owned_schema_decode::ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES,maximum_capacity_bytes:0,maximum_release_bytes:store::os_store::owned_schema_decode::ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES,maximum_depth:64},true).expect("close observes exact completed terminal state"), PluginLifecycleStep::Complete(Default::default()));
        artifact_app_laws::close_registered_fixture_app(&mut app, mounted_policy,cx).expect("original contract fixture paid terminal close");

        }.await;let fuel=cx.fuel_remaining();drop(original_context);original_caller.retire(fuel).expect("original builder fixture paid terminal retirement");result
    }
