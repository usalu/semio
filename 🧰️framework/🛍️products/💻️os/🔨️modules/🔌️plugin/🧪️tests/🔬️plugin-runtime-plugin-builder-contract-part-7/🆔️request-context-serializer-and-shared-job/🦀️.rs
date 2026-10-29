/// 🆔️ Request context, serializer and shared job: the app-owned request context identity oracle, the serializer entry erasure and the shared framework job that is cancellable, resumable and boundedly closeable, with the retained latest-wins guards.

    /// 🏷️ The recorded digest is an oracle for ONE digest domain. Its domain tag is bumped
    /// (`ARC-CONTEXT-3` → `-4` …) exactly when the identity's input set changes, which is what
    /// invalidates the literal — so the tag is joined here first and a drift fails on the clause
    /// that names its cause instead of on a bare number.
    use semio_framework_plugin_decl_app::app_emission::EphemeralEmit;
#[test]
    fn app_owned_request_context_identity_matches_language_neutral_oracle_and_rejects_every_root_drift() {
        let fixture: Value = serde_json::from_str(include_str!("../../../🧵️retained-command/🧪️tests/🧫️fixtures/🧬️request-context.json")).expect("request context fixture");
        let hex = |text: &str| text.as_bytes().chunks_exact(2).map(|pair| u8::from_str_radix(std::str::from_utf8(pair).expect("hex pair"), 16).expect("hex byte")).collect::<Vec<_>>();
        let revision: [u8; 32] = hex(fixture["canonicalBaseRevisionHex"].as_str().expect("revision hex")).try_into().expect("revision width");
        let expected = u64::from_str_radix(fixture["expectedIdentityDigestHex"].as_str().expect("digest hex"), 16).expect("digest");
        let app_instance_id = fixture["appInstanceId"].as_u64().expect("app instance") as u32;
        let draft_generation = fixture["draftGeneration"].as_u64().expect("draft generation");
        let transient_generation = fixture["transientGeneration"].as_u64().expect("transient generation");
        let children_digest = u64::from_str_radix(fixture["childrenDigestHex"].as_str().expect("children digest hex"), 16).expect("children digest");
        let identity = test_artifact_owned_tool_job_context_identity_digest;
        let source = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/../../../../👷️job/🦀️.rs"));
        let tag = source.split("let mut digest = extend(0xcbf2_9ce4_8422_2325, b\"").nth(1).expect("the identity digest seeds itself with its own domain tag").split('"').next().expect("domain tag literal");
        assert_eq!(tag, fixture["domainTag"].as_str().expect("fixture domain tag"), "the recorded oracle belongs to a different identity domain — re-record the digest with the tag that replaced it");
        assert_eq!(identity(app_instance_id, revision, draft_generation, transient_generation, children_digest), expected);
        assert_eq!(fixture["mismatchCases"].as_array().expect("mismatch cases").len(), 5);
        assert_ne!(identity(app_instance_id + 1, revision, draft_generation, transient_generation, children_digest), expected);
        let mut changed_revision = revision;
        changed_revision[31] ^= 1;
        assert_ne!(identity(app_instance_id, changed_revision, draft_generation, transient_generation, children_digest), expected);
        assert_ne!(identity(app_instance_id, revision, draft_generation + 1, transient_generation, children_digest), expected);
        assert_ne!(identity(app_instance_id, revision, draft_generation, transient_generation + 1, children_digest), expected);
        assert_ne!(identity(app_instance_id, revision, draft_generation, transient_generation, children_digest ^ 1), expected);
        assert_eq!(fixture["restartBoundary"]["workerThreadLoss"], "exact-resume-with-retained-context");
        assert_eq!(fixture["restartBoundary"]["processRestartWithoutExactTransientSnapshot"], "fail-closed");
    }

    /// 🧪️ Trivial dummy `ArtifactSerializer`/`ArtifactDeserializer` pair, round-tripping
    /// `TestSnapshot` to itself, for `serializer_entry_of`/`deserializer_entry_of` smoke tests.
    struct DummySerializer;
    impl ArtifactSerializer for DummySerializer {
        type From = TestSnapshot;
        type Into = TestSnapshot;
        const FROM: Dialect = Dialect { artifact_kind: "s.test.dummy", standard: StandardId("1"), subset: SubsetId("*") };
        const INTO: Dialect = Dialect { artifact_kind: "s.test.dummy.out", standard: StandardId("1"), subset: SubsetId("*") };
        async fn serialize(from: &TestSnapshot) -> Result<TestSnapshot, store::PackError> {
            Ok(from.clone())
        }
    }

    struct DummyDeserializer;
    impl ArtifactDeserializer for DummyDeserializer {
        type From = TestSnapshot;
        type Into = TestSnapshot;
        const FROM: Dialect = Dialect { artifact_kind: "s.test.dummy.out", standard: StandardId("1"), subset: SubsetId("*") };
        const INTO: Dialect = Dialect { artifact_kind: "s.test.dummy", standard: StandardId("1"), subset: SubsetId("*") };
        async fn deserialize(from: &TestSnapshot) -> Result<TestSnapshot, store::PackError> {
            Ok(from.clone())
        }
    }

    #[semio_framework_async_macros::async_test]
    async fn serializer_entry_of_and_deserializer_entry_of_erase_correctly() {
        let ser = serializer_entry_of::<DummySerializer>();
        assert_eq!(ser.writes, DummySerializer::INTO);
        assert_eq!(ser.reads.to_vec(), vec![DummySerializer::FROM]);
        let de = deserializer_entry_of::<DummyDeserializer>();
        assert_eq!(de.writes, DummyDeserializer::INTO);
        assert_eq!(de.reads.to_vec(), vec![DummyDeserializer::FROM]);

        let seed = TestSnapshot { count: 7, label: "x".into(), slot: Vec::new() };
        let bytes = ArtifactPack::encode_pack(&seed);
        let composed = ::semio_framework_async::poll::resolve_ready((ser.compose)(&[ErasedComposeSource { dialect: DummySerializer::FROM, payload: IoPayload::Binary(bytes) }])).expect("serializer_entry_of erased compose should succeed with exactly 1 source");
        assert_eq!(composed.dialect, DummySerializer::INTO);
        match composed.payload {
            IoPayload::Binary(out) => assert_eq!(<TestSnapshot as ArtifactPack>::decode_pack(&out).unwrap(), seed),
            IoPayload::Text(_) => panic!("expected Binary payload"),
        }

        let zero_sources_err = match ::semio_framework_async::poll::resolve_ready((de.compose)(&[])) {
            Err(err) => err,
            Ok(_) => panic!("deserializer_entry_of erased compose should reject 0 sources"),
        };
        assert!(zero_sources_err.message.contains("needs exactly 1 source"), "{}", zero_sources_err.message);
        let two_sources = [ErasedComposeSource { dialect: DummyDeserializer::FROM, payload: IoPayload::Binary(Vec::new()) }, ErasedComposeSource { dialect: DummyDeserializer::FROM, payload: IoPayload::Binary(Vec::new()) }];
        let two_sources_err = match ::semio_framework_async::poll::resolve_ready((de.compose)(&two_sources)) {
            Err(err) => err,
            Ok(_) => panic!("deserializer_entry_of erased compose should reject 2 sources"),
        };
        assert!(two_sources_err.message.contains("needs exactly 1 source"), "{}", two_sources_err.message);
    }



    #[derive(semio_framework_value::FactoryPayloadRetirement)]
    struct PublicationPresenceRetirementFactory;

    impl store::SnapshotRetirementFactory<PublicationPresence> for PublicationPresenceRetirementFactory {
        fn retirement_birth_bytes(&self, _snapshot: &std::sync::Arc<PublicationPresence>) -> usize { semio_framework_value::retirement::arc_custody::shared_retirement_birth_bytes::<PublicationPresence>() }
        fn retire(&self, snapshot: std::sync::Arc<PublicationPresence>, grant: semio_framework_value::RetainedCloneGrant) -> Result<(Box<dyn store::ErasedSnapshotRetirement>, semio_framework_value::RetainedCloneProgress), (semio_framework_value::ValueError, std::sync::Arc<PublicationPresence>)> {
            semio_framework_value::retirement::arc_custody::admit_shared_retirement(snapshot, grant, true)
        }
    }

    /// 🚫️ The UNPROVED twin: `contract_registry`'s `BatchOnlyPendingRewrite` declarations against an
    /// app that owns no tool factory at all, for the laws whose whole subject is the refusal.
    /// 🚫️ A `TestApp` that registers no owned factory at all, on `contract_registry` — every verb
    /// `BatchOnlyPendingRewrite`. The migrated registry is not an option here: an app declaring migrated
    /// verbs and supplying no proof row cannot be CONSTRUCTED at all (`interactive-job.catalog-incomplete`),
    /// which is the same fail-closed join from the other side.
    async fn unproved_contract_app(mounted_policy: semio_framework_plugin_mounted_owner::MountedOwnerPolicyV1, identity: &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>) -> VcsArtifactApp<TestApp<false, TEST_APP_TOOLS_NONE>> {
        VcsArtifactApp::with_registry(TestApp::<false, TEST_APP_TOOLS_NONE>::default(), contract_registry().await, protocol::ActorId(protocol::LOCAL_ACTOR_ID.into()), mounted_policy, identity).await.expect("the reference snapshot births")
    }

    #[test]
    fn shared_framework_job_is_cancellable_resumable_and_boundedly_closeable() {
        let policy=semio_framework_plugin_testing_laws::artifact_app_laws::fixture_mounted_policy();
        let mut before=semio_framework_job::OriginalJobFixtureCaller::admit(semio_framework_job::allocate_operation_id(),semio_framework_job::Generation(7),policy.preparation,||Some(0),u64::MAX,4096);
        before.cancel().cancel_now();
        let mut cancelled_before=FrameworkSetSelectionModeJob::new(vec![1;8193],1);
        assert!(matches!(before.turn(policy.maintenance,|cx|Ok(semio_framework_plugin_testing_laws::artifact_app_laws::fixture_step(&mut cancelled_before,cx))).unwrap(),semio_framework_plugin_testing_laws::artifact_app_laws::FixtureStepOutcome::Cancelled));
        semio_framework_job::InteractiveJob::begin_close(&mut cancelled_before);drive_job_close(&mut cancelled_before,&mut before);before.retire(policy.close);

        let mut after=semio_framework_job::OriginalJobFixtureCaller::admit(semio_framework_job::allocate_operation_id(),semio_framework_job::Generation(7),policy.preparation,||Some(0),u64::MAX,4096);
        let mut cancelled_after=FrameworkSetSelectionModeJob::new(vec![1;8193],1);
        assert!(matches!(after.turn(policy.maintenance,|cx|Ok(semio_framework_plugin_testing_laws::artifact_app_laws::fixture_step(&mut cancelled_after,cx))).unwrap(),semio_framework_plugin_testing_laws::artifact_app_laws::FixtureStepOutcome::Preview(_)));
        after.cancel().cancel_now();
        assert!(matches!(after.turn(policy.maintenance,|cx|Ok(semio_framework_plugin_testing_laws::artifact_app_laws::fixture_step(&mut cancelled_after,cx))).unwrap(),semio_framework_plugin_testing_laws::artifact_app_laws::FixtureStepOutcome::Cancelled));
        semio_framework_job::InteractiveJob::begin_close(&mut cancelled_after);drive_job_close(&mut cancelled_after,&mut after);after.retire(policy.close);

        let operation=semio_framework_job::Operation::new(semio_framework_job::allocate_operation_id(),semio_framework_job::RevisionId(11),semio_framework_job::Generation(3),29);
        let mut caller=semio_framework_job::OriginalJobFixtureCaller::admit(operation.operation,operation.generation,policy.preparation,||Some(0),u64::MAX,4096);
        let mut retained=ArtifactReservedToolJob::new(FrameworkSetSelectionModeJob::new(vec![2;8193],1));
        retained.bind_operation(operation).expect("first exact operation authority");
        assert!(retained.clone().bind_operation(operation).is_err(),"a second factory transfer cannot replace the live operation generation");
        semio_framework_job::InteractiveJob::begin_close(&mut retained);
        let quoted=job_close_grant(&retained);
        assert_eq!(caller.turn(semio_framework_value::RetainedCloneGrant{maximum_items:0,..quoted},|cx|semio_framework_job::InteractiveJob::close_step(&mut retained,cx)).unwrap(),semio_framework_job::InteractiveJobCloseStep::Pending{progress:Default::default()});
        let(mut turns,mut released)=(0usize,0usize);
        while !semio_framework_job::InteractiveJob::terminal_is_empty(&retained){
            let grant=job_close_grant(&retained);let step=caller.turn(grant,|cx|semio_framework_job::InteractiveJob::close_step(&mut retained,cx)).unwrap();
            assert!(!matches!(step,semio_framework_job::InteractiveJobCloseStep::Blocked),"original close turn {turns} blocked: {step:?}");
            assert_eq!(step.progress(),caller.receipt());assert!(step.progress().fits(grant));released+=step.progress().released_bytes;turns+=1;assert!(turns<100000);
        }
        assert!(released>=8193,"physical bytes must leave through the original release axis");
        let terminal=job_close_grant(&retained);
        assert_eq!(caller.turn(terminal,|cx|semio_framework_job::InteractiveJob::close_step(&mut retained,cx)).unwrap(),semio_framework_job::InteractiveJobCloseStep::Complete{progress:Default::default()});
        caller.retire(policy.close);
    }

    #[semio_framework_async_macros::async_test]
    async fn retained_latest_wins_cancellation_guards_real_store_publication_and_preserves_committed_ack() {
        test_retained_cancellation_publication_boundaries::<TestApp<false, TEST_APP_TOOLS_FACTORIES>>().await;
    }

    #[semio_framework_async_macros::async_test]
    async fn retained_latest_wins_reserved_slots_and_ready_publisher_are_fair() {
        test_retained_latest_wins_slot_and_publication_fairness::<TestApp<false, TEST_APP_TOOLS_FACTORIES>>().await;
    }

    #[test]
    fn retained_latest_wins_raw_capacity_is_not_initialized_byte_retirement() {
        semio_framework_plugin_retained_command::test_raw_allocation_close::<TestApp>();
    }

    #[semio_framework_async_macros::async_test]
    async fn retained_latest_wins_real_document_publication_cancellation_and_delayed_ack_close() {
        test_retained_document_cancellation::<TestApp<false, TEST_APP_TOOLS_FACTORIES>>(store::mutation_apply_preparation_factory::<TestSnapshot, TestMutation>(), || TestMutation::SetCount(SetCount { value: 42 }), |snapshot| snapshot.count).await;
    }

    #[test]

    fn retained_rejected_completion_closes_original_artifact_and_ephemeral_owners(){

        crate::retained_command::test_rejected_completion_close::<TestApp>(|row|{

            let mut emit=Emit::default();

            let mut ephemeral=EphemeralEmit::default();

            for revision in 0..row["artifactMutations"].as_u64().unwrap(){emit.artifact_mutations.push(TestMutation::SetCount(SetCount{value:revision as _}));}

            for revision in 0..row["presenceMutations"].as_u64().unwrap(){ephemeral.presence.push(ChangePublicationPresence{revision}.into());}

            for revision in 0..row["transientMutations"].as_u64().unwrap(){ephemeral.transient.push(ChangePublicationTransient{revision}.into());}

            (emit,ephemeral)

        });

    }
