mod typed_command_full_operation_tests {
    use semio_framework_test_fixtures::fixture_path;

    use super::*;
    use semio_framework_value::{RetainedCloneGrant, RetainedCloneProgress, RetainedCloneStep};
    use semio_framework_plugin_testing_fixtures_publication::publication_fixture::presence::mutations::change_publication_presence::ChangePublicationPresence;
use semio_framework_plugin_testing_fixtures_publication::publication_fixture::presence::PublicationPresence;
use semio_framework_plugin_testing_fixtures_publication::publication_fixture::presence::mutations::PublicationPresenceMutation;

    /// 📖️ One module tree of the plugin SDK, read at run time from the manifest folder of the hosting crate instead of embedded: the root file first, then every file it mounts in declaration order.
    fn module_source(relative: &str) -> String {
        semio_framework_plugin_test_support::read_module_tree_source(&fixture_path!("../../../..").join(relative)).expect("the module tree reads")
    }

    /// 🧵️ Source text of the mounted artifact instance: its module tree, the parent file followed by every impl band in declaration order.
    #[doc(hidden)]
pub fn instance_source() -> String {
        module_source("🧵️instance/🦀️.rs")
    }

    #[doc(hidden)]
pub const FIXTURE: &str = include_str!("../../🧵️retained-command/🔄️full-operation/🧪️tests/🧫️fixtures/🔣️.json");

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    struct TypedCommandCensusDecision {
        pub bytes: usize,
        pub accepted: bool,
    }

    trait TypedCommandCensusOracle {
        fn decide(&self, description: &str, maximum: usize) -> TypedCommandCensusDecision;
    }

    struct OwnedTypedCommandCensus;

    impl TypedCommandCensusOracle for OwnedTypedCommandCensus {
        fn decide(&self, description: &str, maximum: usize) -> TypedCommandCensusDecision {
            let mut bytes = 0usize;
            let mut accepted = true;
            for _ in description.as_bytes() {
                accepted &= bytes.checked_add(1).is_some_and(|next| {
                    bytes = next;
                    next <= maximum
                });
            }
            accepted &= bytes.checked_add(1).is_some_and(|next| {
                bytes = next;
                next <= maximum
            });
            TypedCommandCensusDecision { bytes, accepted }
        }
    }

    struct SerdeJsonTypedCommandCensus;

    impl TypedCommandCensusOracle for SerdeJsonTypedCommandCensus {
        fn decide(&self, description: &str, maximum: usize) -> TypedCommandCensusDecision {
            let value = serde_json::json!({ "description": description, "uiScopeFields": 1 });
            let bytes = value["description"].as_str().expect("oracle description").as_bytes().len() + value["uiScopeFields"].as_u64().expect("oracle UI scope") as usize;
            TypedCommandCensusDecision { bytes, accepted: bytes <= maximum }
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    enum FixtureTransition {
        Yield,
        Cancelled,
        Checkpoint,
        Advance,
        Retain,
        Fault,
    }

    struct OwnedFixtureMachine {
        pub phase: usize,
        pub advanced: usize,
        retained_page: bool,
        pub owners: usize,
        pub terminal: bool,
    }

    impl OwnedFixtureMachine {
        pub fn new(phase: usize) -> Self {
            Self { phase, advanced: 0, retained_page: true, owners: 0, terminal: false }
        }

        pub fn grant(&mut self, fuel: u64, expired: bool, cancelled: bool) -> FixtureTransition {
            if cancelled {
                return FixtureTransition::Cancelled;
            }
            if fuel == 0 || expired {
                return FixtureTransition::Yield;
            }
            self.phase = self.phase.saturating_add(1);
            self.advanced += 1;
            FixtureTransition::Checkpoint
        }

        pub fn freshness(operation_revision: u64, live_revision: u64, operation_generation: u64, live_generation: u64) -> bool {
            operation_revision == live_revision && operation_generation == live_generation
        }

        pub fn admission(capacity: usize, occupied: usize, roots: usize, required_roots: usize) -> bool {
            occupied < capacity && roots == required_roots
        }

        pub fn publish(&mut self, attempt: u8, acknowledged: bool, maximum_retries: u8) -> FixtureTransition {
            if acknowledged {
                self.retained_page = false;
                FixtureTransition::Advance
            } else if attempt > maximum_retries {
                FixtureTransition::Fault
            } else {
                FixtureTransition::Retain
            }
        }

        fn close_scalar(value: &mut String, maximum_bytes: usize) -> (usize, usize) {
            let Some(bytes) = value.chars().next_back().map(char::len_utf8) else { return (0, 0) };
            if bytes > maximum_bytes {
                return (0, 0);
            }
            value.pop();
            (0, bytes)
        }

        pub fn close_owner(&mut self, maximum_items: usize) -> (usize, usize) {
            if self.owners == 0 {
                self.terminal = true;
                return (0, 0);
            }
            if maximum_items == 0 {
                return (0, 0);
            }
            self.owners -= 1;
            self.terminal = self.owners == 0;
            (1, 0)
        }
    }

    struct SerdeFixtureOracle<'a>(&'a Value);

    impl SerdeFixtureOracle<'_> {
        pub fn transition(&self) -> FixtureTransition {
            match self.0["expected"].as_str().expect("fixture expected transition") {
                "yield" => FixtureTransition::Yield,
                "cancelled" => FixtureTransition::Cancelled,
                "checkpoint" => FixtureTransition::Checkpoint,
                "advance" => FixtureTransition::Advance,
                "retain" => FixtureTransition::Retain,
                "fault" => FixtureTransition::Fault,
                unexpected => panic!("unknown fixture transition {unexpected}"),
            }
        }
    }

    #[derive(semio_framework_value::FactoryPayloadRetirement)]
    struct TwoTurnPublicationPresencePreparationFactory;

    struct TwoTurnPublicationPresencePreparation {
        pub request: Option<store::ArtifactEphemeralOneItemPreparationRequest<PublicationPresence, PublicationPresenceMutation>>,
        pub prepared: Option<store::ArtifactEphemeralOneItemPrepared<PublicationPresence>>,
        pub checkpoint: store::os_store::one_item::ArtifactStoreOneItemCheckpoint,
        pub turn: u8,
        pub closing: bool,
    }

    impl store::ArtifactEphemeralOneItemPreparationFactory<PublicationPresence, PublicationPresenceMutation> for TwoTurnPublicationPresencePreparationFactory {
        fn preflight(&self, _mutation: &PublicationPresenceMutation) -> Result<store::os_store::one_item::ArtifactStoreOneItemFootprint, String> {
            Ok(store::os_store::one_item::ArtifactStoreOneItemFootprint { work_items: 2, retained_bytes: 64 })
        }

        fn begin(
            &self,
            request: store::ArtifactEphemeralOneItemPreparationRequest<PublicationPresence, PublicationPresenceMutation>,
        ) -> Result<Box<dyn store::ArtifactEphemeralOneItemPreparation<PublicationPresence, PublicationPresenceMutation>>, store::ArtifactEphemeralOneItemPreparationRequest<PublicationPresence, PublicationPresenceMutation>> {
            Ok(Box::new(TwoTurnPublicationPresencePreparation { request: Some(request), prepared: None, checkpoint: store::os_store::one_item::ArtifactStoreOneItemCheckpoint::default(), turn: 0, closing: false }))
        }
    }

    impl store::ArtifactEphemeralOneItemPreparation<PublicationPresence, PublicationPresenceMutation> for TwoTurnPublicationPresencePreparation {
        fn advance(&mut self, grant: store::os_store::one_item::ArtifactStoreOneItemGrant) -> Result<store::os_store::one_item::ArtifactStoreOneItemPreparationStep, semio_framework_value::ValueError> {
            if !grant.permits_one() {
                return Ok(store::os_store::one_item::ArtifactStoreOneItemPreparationStep::Blocked);
            }
            if self.turn == 0 {
                self.turn = 1;
                self.checkpoint = store::os_store::one_item::ArtifactStoreOneItemCheckpoint { cursor: 1, completed_items: 1, completed_bytes: 1, digest: [1; 32] };
                return Ok(store::os_store::one_item::ArtifactStoreOneItemPreparationStep::Progress(self.checkpoint, semio_framework_value::RetainedCloneProgress { copied_items: 1, ..Default::default() }));
            }
            if self.prepared.is_none() {
                let request = self.request.take().ok_or_else(|| semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated, "two-turn publication-presence preparation lost its owner bundle"))?;
                let next_root = protocol::apply_diff(request.mutation.diff(request.base.as_ref()).diff(), request.base.as_ref()).map_err(|error| semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, error.to_string()))?;
                self.prepared = Some(store::ArtifactEphemeralOneItemPrepared { next_root: std::sync::Arc::new(next_root) });
                self.checkpoint = store::os_store::one_item::ArtifactStoreOneItemCheckpoint { cursor: 2, completed_items: 2, completed_bytes: 2, digest: [2; 32] };
            }
            Ok(store::os_store::one_item::ArtifactStoreOneItemPreparationStep::Prepared(self.checkpoint, semio_framework_value::RetainedCloneProgress { copied_items: 1, ..Default::default() }))
        }

        fn checkpoint(&self) -> store::os_store::one_item::ArtifactStoreOneItemCheckpoint {
            self.checkpoint
        }

        fn prepared(&self) -> Option<&store::ArtifactEphemeralOneItemPrepared<PublicationPresence>> {
            self.prepared.as_ref()
        }

        fn take_prepared(&mut self) -> Option<store::ArtifactEphemeralOneItemPrepared<PublicationPresence>> {
            self.prepared.take()
        }

        fn cancel(&mut self) {}

        fn begin_close(&mut self) {
            self.closing = true;
        }

        fn close_step(&mut self, grant: store::os_store::one_item::ArtifactStoreOneItemGrant) -> Result<semio_framework_value::retained_clone::RetainedCloneStep, semio_framework_value::ValueError> {
            use semio_framework_value::retained_clone::{RetainedCloneProgress, RetainedCloneStep};
            if !self.closing || !grant.permits_one() {
                return Ok(RetainedCloneStep::Progress(RetainedCloneProgress::default()));
            }
            if self.prepared.take().is_some() || self.request.take().is_some() {
                return Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, ..Default::default() }));
            }
            Ok(RetainedCloneStep::Complete(RetainedCloneProgress::default()))
        }

        fn next_close_copy_byte_demand(&self) -> Result<usize, semio_framework_value::ValueError> {
            Ok(0)
        }

        fn next_close_capacity_byte_demand(&self, _: usize) -> Result<usize, semio_framework_value::ValueError> {
            Ok(0)
        }

        fn next_close_release_byte_demand(&self) -> Result<usize, semio_framework_value::ValueError> {
            Ok(0)
        }

        fn next_close_depth_demand(&self) -> Result<usize, semio_framework_value::ValueError> {
            Ok(usize::from(!self.terminal_is_empty()))
        }

        fn terminal_is_empty(&self) -> bool {
            self.closing && self.request.is_none() && self.prepared.is_none()
        }
    }

    struct PublicationPresenceLocalRootRetirement {
        pub root: Option<std::sync::Arc<PublicationPresence>>,
    }

    impl store::ErasedSnapshotRetirement for PublicationPresenceLocalRootRetirement {
        fn close_step(&mut self, grant: semio_framework_value::retained_clone::RetainedCloneGrant) -> Result<semio_framework_value::retained_clone::RetainedCloneStep, semio_framework_value::ValueError> {
            use semio_framework_value::retained_clone::{RetainedCloneProgress, RetainedCloneStep};
            if self.root.is_none() {
                return Ok(RetainedCloneStep::Complete(RetainedCloneProgress::default()));
            }
            if grant.maximum_items == 0 || grant.maximum_depth == 0 {
                return Ok(RetainedCloneStep::Progress(RetainedCloneProgress::default()));
            }
            self.root.take();
            Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, ..Default::default() }))
        }

        fn terminal_is_empty(&self) -> bool {
            self.root.is_none()
        }

        fn next_copy_byte_demand(&self) -> Result<usize, semio_framework_value::ValueError> {
            Ok(0)
        }

        fn next_capacity_byte_demand(&self, _: usize) -> Result<usize, semio_framework_value::ValueError> {
            Ok(0)
        }

        fn next_release_byte_demand(&self) -> Result<usize, semio_framework_value::ValueError> {
            Ok(0)
        }

        fn next_depth_demand(&self) -> Result<usize, semio_framework_value::ValueError> {
            Ok(usize::from(self.root.is_some()))
        }
    }

    #[derive(semio_framework_value::FactoryPayloadRetirement)]
    struct PublicationPresenceLocalRootRetirementFactory;

    impl store::SnapshotRetirementFactory<PublicationPresence> for PublicationPresenceLocalRootRetirementFactory {
        fn retirement_birth_bytes(&self, _snapshot: &std::sync::Arc<PublicationPresence>) -> usize { semio_framework_value::retirement::shared::shared_retirement_birth_bytes::<PublicationPresence>() }
        fn retire(&self, snapshot: std::sync::Arc<PublicationPresence>, grant: RetainedCloneGrant) -> Result<(Box<dyn store::ErasedSnapshotRetirement>, RetainedCloneProgress), (semio_framework_value::ValueError, std::sync::Arc<PublicationPresence>)> {
            semio_framework_value::retirement::shared::admit_shared_retirement(snapshot, grant, true)
        }
    }

    fn close_mounted<A: ArtifactApp>(mounted: &mut MountedTypedCommandFullOperation<A>, context: &str, original_grant:RetainedCloneGrant) {
        for _ in 0..100_000 {
            if mounted.terminal_is_empty() {
                return;
            }
            let quoted = original_grant;
            assert!(quoted.maximum_items>0 && quoted.maximum_depth>0, "{context}");
            match mounted.retirement_step(quoted).unwrap() {
                PluginLifecycleStep::Progress(progress) => assert!(progress.fits(quoted) && progress.copied_bytes<=TYPED_OPERATION_RESULT_PAGE_BYTES && progress.released_bytes<=TYPED_OPERATION_RESULT_PAGE_BYTES, "{context}"),
                PluginLifecycleStep::Complete(progress) => {
                    assert!(progress.fits(quoted) && progress.copied_bytes<=TYPED_OPERATION_RESULT_PAGE_BYTES && progress.released_bytes<=TYPED_OPERATION_RESULT_PAGE_BYTES, "{context}");
                    return;
                }
                PluginLifecycleStep::AwaitingInput { reason } => panic!("{context} awaited input: {reason}"),
                PluginLifecycleStep::Blocked { reason } => panic!("{context} blocked: {reason}"),
            }
        }
    }

    fn close_app<A: ArtifactApp>(app: &mut VcsArtifactApp<A>, context: &str) {
        for _ in 0..100_000 {
            if app.close_terminal_is_empty() {
                return;
            }
            let quoted = app.mounted_policy.close;
            assert!(quoted.maximum_items>0 && quoted.maximum_depth>0, "{context}");
            match app.close_step(quoted).unwrap() {
                PluginLifecycleStep::Progress(progress) => assert!(progress.fits(quoted) && progress.copied_bytes<=TYPED_OPERATION_RESULT_PAGE_BYTES && progress.released_bytes<=TYPED_OPERATION_RESULT_PAGE_BYTES, "{context}"),
                PluginLifecycleStep::Complete(progress) => {
                    assert!(progress.fits(quoted) && progress.copied_bytes<=TYPED_OPERATION_RESULT_PAGE_BYTES && progress.released_bytes<=TYPED_OPERATION_RESULT_PAGE_BYTES, "{context}");
                    return;
                }
                PluginLifecycleStep::AwaitingInput { reason } => panic!("{context} awaited input: {reason}"),
                PluginLifecycleStep::Blocked { reason } => panic!("{context} blocked: {reason}"),
            }
        }
    }

    fn advance_registry(registry: &mut ToolLatestWinsRegistry, context: &str) {
        let quoted = RetainedCloneGrant { maximum_items:1, maximum_copy_bytes:4096, maximum_capacity_bytes:4096, maximum_release_bytes:4096, maximum_depth:semio_framework_value::ordered::MAX_AVL_HEIGHT * 5 + 16 };
        assert!(quoted.maximum_items>0 && quoted.maximum_depth>0, "{context}");
        let (semio_framework_value::retained_clone::RetainedCloneStep::Progress(progress) | semio_framework_value::retained_clone::RetainedCloneStep::Complete(progress)) = registry.advance(quoted).unwrap();
        assert!(progress.fits(quoted) && progress.copied_bytes<=TYPED_OPERATION_RESULT_PAGE_BYTES && progress.released_bytes<=TYPED_OPERATION_RESULT_PAGE_BYTES, "{context}");
    }

    pub(super) async fn retained_cancellation_publication_boundaries<A: ArtifactApp<Presence = PublicationPresence, PresenceMutation = PublicationPresenceMutation> + Default>() {
        let fixture: Value = serde_json::from_str(include_str!("../🧫️fixtures/🥇️tool-latest-wins.json")).expect("language-neutral cancellation boundaries");
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
                publication_lanes: &[ArtifactToolPublicationLane::Presence],
                session: None,
                session_rejected: None,
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
                result_page: None,
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
                let page = mounted.result_page.as_ref().unwrap().clone();
                assert!(!mounted.reject_cancelled_publication().unwrap());
                assert_eq!(mounted.result_page.as_ref().unwrap().token, page.token);
                assert_eq!(mounted.result_page.as_ref().unwrap().bytes(), page.bytes());
                assert!(mounted.acknowledge_result_page(page.token).unwrap());
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
                assert!(matches!(presence.advance_publish_one(&mut pending, grant).unwrap(), store::os_store::one_item_publication::ArtifactStoreOneItemAdvance::Published(_)));
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

    fn fixture_latest_wins_key(scope: &Value) -> semio_framework_value::ordered::SharedOwner<String> {
        let parts = [scope["document"].as_str().unwrap(), scope["controller"].as_str().unwrap(), scope["tool"].as_str().unwrap(), scope["target"].as_str().unwrap()];
        use semio_framework_value::retained_clone::{RetainedCloneGrant, RetainedCloneProgress, RetainedCloneStep};
        let page = RetainedCloneGrant { maximum_items:1, maximum_copy_bytes:4096, maximum_capacity_bytes:4096, maximum_release_bytes:4096, maximum_depth:semio_framework_value::ordered::MAX_AVL_HEIGHT * 5 + 16 };
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
        use semio_framework_value::{ordered::SharedOwner, retained_clone::RetainedCloneGrant};
        let fixture:Value=serde_json::from_str(include_str!("../🧫️fixtures/🥇️tool-latest-wins.json")).unwrap();
        let scope=&fixture["first"];
        let parts=[scope["document"].as_str().unwrap(),scope["controller"].as_str().unwrap(),scope["tool"].as_str().unwrap(),scope["target"].as_str().unwrap()];
        let page=RetainedCloneGrant { maximum_items:1, maximum_copy_bytes:4096, maximum_capacity_bytes:4096, maximum_release_bytes:4096, maximum_depth:semio_framework_value::ordered::MAX_AVL_HEIGHT * 5 + 16 };
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

    pub(super) async fn retained_document_cancellation<A: ArtifactApp + Default>(factory: std::sync::Arc<dyn store::os_store::one_item_publication::ArtifactStoreOneItemPreparationFactory<A::Snapshot, A::Mutation>>, mutation: fn() -> A::Mutation, observe: fn(&A::Snapshot) -> i32) {
        let fixture: Value = serde_json::from_str(include_str!("../🧫️fixtures/🥇️tool-latest-wins.json")).unwrap();
        for case in fixture["publicationCases"].as_array().unwrap() {
            for delayed_ack in [false, true] {
                let boundary = case["cancelAt"].as_str().unwrap();
                let mounted_grant=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:32_768,maximum_capacity_bytes:262_144,maximum_release_bytes:1_048_576,maximum_depth:4_096};
        let mounted_policy=semio_framework_plugin_mounted_owner::MountedOwnerPolicyV1{preparation:mounted_grant,maintenance:mounted_grant,close:mounted_grant};
        let identity_started=std::time::Instant::now();
        let mut identity_progress=|progress:semio_framework_value::native_encoding::NativeEncodeProgress|{progress.owned_bytes<=1_048_576&&identity_started.elapsed()<std::time::Duration::from_secs(60)};
        let mut identity=semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority::new(1_048_576,&mut identity_progress as &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::Observer<'_>).expect("original typed fixture supplied identity authority");
        let mut app = VcsArtifactApp::<A>::new(A::default(), protocol::ActorId(protocol::LOCAL_ACTOR_ID.into()),mounted_policy,&mut identity).await;
                let before = app.stores.store.snapshot_root();
                let revision = app.stores.store.content_revision_now();
                let generation = app.stores.store.generation_now();
                let operation = semio_framework_job::Operation::new(semio_framework_job::allocate_operation_id(), semio_framework_job::RevisionId(u64::from_be_bytes(revision[..8].try_into().unwrap())), semio_framework_job::Generation(generation), 1);
                let lease = app
                    .tools.tool_cancellations
                    .begin_keyed(ToolOperationKey { app_instance_id: 7, document: ArtifactDocumentAuthority(7), operation_id: operation.operation, base_revision: operation.base_revision, generation: operation.generation })
                    .unwrap();
                let mut mounted = MountedTypedCommandFullOperation::<A> {
                    verb: "setGraphParameter".into(),
                    meta: ActionMeta { actor: "fixture".into(), instance_id: 7, view_state: None },
                    operation,
                    canonical_revision: revision,
                    artifact_generation: generation,
                    config_generation: 0,
                    draft_generation: 0,
                    presence_generation: 0,
                    transient_generation: 0,
                    window_config_authority: None,
                    window_transient_authority: None,
                    publication_lanes: &[ArtifactToolPublicationLane::Artifact],
                    session: None,
                    session_rejected: None,
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
                    result_page: None,
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
                        app.stores.store.begin_apply_batch(operation.operation, generation, revision, "fixture".into(), vec![mutation()], HistoryLane::Document, Some(&factory), None).unwrap_or_else(|_| panic!("exact scalar document preparation admission"));
                    mounted.pending_artifact_publication = Some(PendingArtifactStorePublication::Artifact(pending));
                    let target = match boundary {
                        "preparation" => store::os_store::one_item_publication::ArtifactStoreOneItemPublicationPhase::Preparing,
                        "preflight" => store::os_store::one_item_publication::ArtifactStoreOneItemPublicationPhase::PreflightingCommit,
                        "publishing" => store::os_store::one_item_publication::ArtifactStoreOneItemPublicationPhase::Publishing,
                        "awaitingAck" => store::os_store::one_item_publication::ArtifactStoreOneItemPublicationPhase::AwaitingAck,
                        _ => unreachable!(),
                    };
                    for _ in 0..100_000 {
                        let Some(PendingArtifactStorePublication::Artifact(pending)) = mounted.pending_artifact_publication.as_ref() else {
                            panic!("exact document pending owner");
                        };
                        if pending.phase() == target {
                            break;
                        }
                        app.typed_ctx().publish_mounted_typed_operation_unit(&mut mounted, semio_framework_plugin_testing_laws::artifact_app_laws::fixture_mounted_policy().maintenance).unwrap();
                    }
                    let Some(PendingArtifactStorePublication::Artifact(pending)) = mounted.pending_artifact_publication.as_ref() else {
                        panic!("exact document pending owner");
                    };
                    assert_eq!(pending.phase(), target);
                }
                mounted.cancellation_lease.as_ref().unwrap().cancel();
                if boundary == "awaitingAck" {
                    let receipt = mounted.result_page.as_ref().unwrap().clone();
                    assert!(!mounted.reject_cancelled_publication().unwrap());
                    assert_eq!(mounted.result_page.as_ref().unwrap().bytes(), receipt.bytes());
                    if delayed_ack {
                        let presented = mounted.take_result_page().unwrap();
                        assert_eq!(presented.token.attempt, fixture["resultAck"]["attempt"].as_u64().unwrap() as u8);
                        assert_eq!(mounted.stage, MountedTypedCommandFullOperationStage::AwaitingAck);
                        for _ in 0..fixture["resultAck"]["preAckPolls"].as_u64().unwrap() {
                            assert!(mounted.take_result_page().is_none());
                        }
                        assert!(mounted.acknowledge_result_page(presented.token).unwrap());
                    } else {
                        assert!(mounted.acknowledge_result_page(receipt.token).unwrap());
                    }
                }
                app.typed_ctx().publish_mounted_typed_operation_unit(&mut mounted, semio_framework_plugin_testing_laws::artifact_app_laws::fixture_mounted_policy().maintenance).unwrap();
                let final_page = if delayed_ack {
                    let presented = mounted.take_result_page().unwrap();
                    let mut deliveries = 1;
                    for _ in 0..fixture["resultAck"]["preAckPolls"].as_u64().unwrap() {
                        deliveries += usize::from(mounted.take_result_page().is_some());
                    }
                    assert_eq!(deliveries, fixture["resultAck"]["deliveries"].as_u64().unwrap() as usize);
                    assert_eq!(presented.token.attempt, fixture["resultAck"]["attempt"].as_u64().unwrap() as u8);
                    assert_eq!(mounted.stage, MountedTypedCommandFullOperationStage::AwaitingAck);
                    presented
                } else {
                    mounted.result_page.as_ref().unwrap().clone()
                };
                assert_eq!(final_page.lane, TypedOperationResultLane::Fault);
                let committed = case["committed"].as_bool().unwrap();
                let actual = serde_json::json!({ "count": observe(&app.stores.store.snapshot_root()), "generation": app.stores.store.generation_now() - generation, "sameRoot": std::sync::Arc::ptr_eq(&before, &app.stores.store.snapshot_root()) });
                let expected = serde_json::json!({ "count": if committed { 42 } else { 0 }, "generation": u64::from(committed), "sameRoot": !committed });
                assert_eq!(actual, expected, "{boundary}, delayed ACK={delayed_ack}");
                assert!(mounted.acknowledge_result_page(final_page.token).unwrap());
                close_mounted(&mut mounted, "document cancellation close", mounted_policy.close);
                assert!(mounted.terminal_is_empty());
                drop(before);
                close_app(&mut app, "document cancellation app close");
                assert!(app.close_terminal_is_empty());
                eprintln!("real mounted Document publication {boundary}, delayed ACK={delayed_ack}: count/revision/root retained and close terminal");
            }
        }
    }

    #[test]
    fn retained_latest_wins_full_domain_exact_keys_match_serde_oracle_and_retire() {
        let fixture: Value = serde_json::from_str(include_str!("../🧫️fixtures/🥇️tool-latest-wins.json")).unwrap();
        let first = &fixture["first"];
        assert_eq!(first["target"].as_str().unwrap().len(), 8_192);
        for case in fixture["cases"].as_array().unwrap() {
            let next = &case["next"];
            assert_eq!(next["target"].as_str().unwrap().len(), 8_192);
            let oracle = first == next;
            assert_eq!(oracle, case["superseded"].as_bool().unwrap());
            let cancellations = ToolCancellationHandle::default();
            let mut registry = ToolLatestWinsRegistry::new();
            let first_operation = semio_framework_job::allocate_operation_id();
            let first_lease = cancellations
                .begin_keyed(ToolOperationKey {
                    app_instance_id: first["instance"].as_u64().unwrap() as u32,
                    document: ArtifactDocumentAuthority(7),
                    operation_id: first_operation,
                    base_revision: semio_framework_job::RevisionId(1),
                    generation: semio_framework_job::Generation(0),
                })
                .unwrap();
            let mut first_key=fixture_latest_wins_key(first);
            assert!(registry.begin(first_operation.0, &first_key, &first_lease, RetainedCloneGrant { maximum_items:1, maximum_copy_bytes:4096, maximum_capacity_bytes:4096, maximum_release_bytes:4096, maximum_depth:semio_framework_value::ordered::MAX_AVL_HEIGHT * 5 + 16 }).unwrap().is_some());
            assert!(first_key.release_step(semio_framework_value::retained_clone::RetainedCloneGrant::one_release_turn(first_key.next_release_byte_demand(),1)).unwrap().value.is_none());
            for _ in 0..100_000 {
                if registry.take_outcome(first_operation.0) == Some(true) {
                    break;
                }
                advance_registry(&mut registry, "exact key admission");
            }
            assert!(!first_lease.token.is_cancelled_now());
            for _ in 0..100_000 {
                if registry.can_begin() {
                    break;
                }
                advance_registry(&mut registry, "exact key supersession");
            }
            assert!(registry.can_begin());
            let next_operation = semio_framework_job::allocate_operation_id();
            let next_lease = cancellations
                .begin_keyed(ToolOperationKey {
                    app_instance_id: next["instance"].as_u64().unwrap() as u32,
                    document: ArtifactDocumentAuthority(7),
                    operation_id: next_operation,
                    base_revision: semio_framework_job::RevisionId(1),
                    generation: semio_framework_job::Generation(0),
                })
                .unwrap();
            let mut next_key=fixture_latest_wins_key(next);
            assert!(registry.begin(next_operation.0, &next_key, &next_lease, RetainedCloneGrant { maximum_items:1, maximum_copy_bytes:4096, maximum_capacity_bytes:4096, maximum_release_bytes:4096, maximum_depth:semio_framework_value::ordered::MAX_AVL_HEIGHT * 5 + 16 }).unwrap().is_some());
            assert!(next_key.release_step(semio_framework_value::retained_clone::RetainedCloneGrant::one_release_turn(next_key.next_release_byte_demand(),1)).unwrap().value.is_none());
            let mut accepted = false;
            for _ in 0..100_000 {
                if let Some(result) = registry.take_outcome(next_operation.0) {
                    accepted = result;
                    break;
                }
                advance_registry(&mut registry, "exact key comparison");
            }
            assert!(accepted);
            assert_eq!(first_lease.token.is_cancelled_now(), oracle, "{}", case["id"]);
            assert!(!next_lease.token.is_cancelled_now());
            first_lease.finish();
            next_lease.finish();
            assert_eq!(cancellations.active_operation_count(), 0);
            registry.begin_close();
            for _ in 0..300_000 {
                if registry.terminal_is_empty() {
                    break;
                }
                advance_registry(&mut registry, "exact key close");
            }
            assert!(registry.terminal_is_empty());
            eprintln!("exact latest-wins key {} matched independent serde scope equality and retired its 8192-byte identity", case["id"]);
        }
    }

    #[test]
    fn retained_latest_wins_producer_child_cannot_bypass_document_or_app_publication_claim() {
        let cancellations = ToolCancellationHandle::default();
        let key = ToolOperationKey {
            app_instance_id: 7,
            document: ArtifactDocumentAuthority(7),
            operation_id: semio_framework_job::allocate_operation_id(),
            base_revision: semio_framework_job::RevisionId(1),
            generation: semio_framework_job::Generation(0),
        };
        let lease = cancellations.begin_keyed(key.clone()).unwrap();
        let producer = lease.cancel_token();
        producer.cancel_now();
        assert!(!lease.token.is_cancelled_now());
        assert!(lease.try_claim_publication().is_some());
        assert!(cancellations.cancel_document(key.document).unwrap());
        assert!(lease.try_claim_publication().is_none());
        lease.finish();
        let lease = cancellations.begin_keyed(ToolOperationKey { operation_id: semio_framework_job::allocate_operation_id(), ..key }).unwrap();
        let prior_claim = lease.try_claim_publication().unwrap();
        cancellations.cancel_scope_generation();
        drop(prior_claim);
        assert!(lease.try_claim_publication().is_none());
        lease.finish();
        assert_eq!(cancellations.active_operation_count(), 0);
    }

    #[test]
    fn retained_latest_wins_contended_finish_is_deferred_and_cannot_release_replacement() {
        let fixture: Value = serde_json::from_str(include_str!("../🧫️fixtures/🔗️tool-latest-wins-integration.json")).unwrap();
        let cancellations = ToolCancellationHandle::default();
        let key = ToolOperationKey {
            app_instance_id: 7,
            document: ArtifactDocumentAuthority(7),
            operation_id: semio_framework_job::allocate_operation_id(),
            base_revision: semio_framework_job::RevisionId(1),
            generation: semio_framework_job::Generation(0),
        };
        let lease = cancellations.begin_keyed(key.clone()).unwrap();
        let lock = cancellations.state.lock().unwrap();
        lease.finish();
        assert_eq!(serde_json::json!(cancellations.active_operation_count()), fixture["deferredFinish"]["activeBeforeSweep"]);
        drop(lock);
        let index = TOOL_CANCELLATION_SLOTS + key.operation_id.0 as usize % ARTIFACT_LIVE_OUTPUT_SLOTS;
        assert_eq!(cancellations.cleanup_finished_slot(index).unwrap(), Some(true));
        assert_eq!(serde_json::json!(cancellations.active_operation_count()), fixture["deferredFinish"]["activeAfterSweep"]);
        let old = cancellations.begin_keyed(key.clone()).unwrap();
        assert!(cancellations.cleanup_slot(index).unwrap());
        let replacement = cancellations.begin_keyed(ToolOperationKey { generation: semio_framework_job::Generation(1), ..key }).unwrap();
        old.finish();
        assert_eq!(serde_json::json!(cancellations.active_operation_count() == 1 && !replacement.token.is_cancelled_now()), fixture["deferredFinish"]["replacementPreserved"]);
        replacement.finish();
        assert_eq!(cancellations.active_operation_count(), 0);
    }

    #[test]
    fn retained_latest_wins_rebase_rebinds_exact_registered_cancellation_authority() {
        let fixture: Value = serde_json::from_str(include_str!("../🧫️fixtures/🔗️tool-latest-wins-integration.json")).unwrap();
        let cancellations = ToolCancellationHandle::default();
        let old = ToolOperationKey {
            app_instance_id: 7,
            document: ArtifactDocumentAuthority(7),
            operation_id: semio_framework_job::allocate_operation_id(),
            base_revision: semio_framework_job::RevisionId(1),
            generation: semio_framework_job::Generation(fixture["rebase"]["beforeGeneration"].as_u64().unwrap()),
        };
        let fresh = ToolOperationKey { base_revision: semio_framework_job::RevisionId(2), generation: semio_framework_job::Generation(fixture["rebase"]["afterGeneration"].as_u64().unwrap()), ..old.clone() };
        let mut lease = cancellations.begin_keyed(old.clone()).unwrap();
        let busy = cancellations.state.lock().unwrap();
        assert!(!lease.rebind_keyed(fresh.base_revision, fresh.generation).unwrap());
        drop(busy);
        assert!(lease.rebind_keyed(fresh.base_revision, fresh.generation).unwrap());
        assert_eq!(lease.key, fresh);
        assert_eq!(serde_json::json!({ "old": cancellations.cancel(&old).unwrap(), "fresh": cancellations.cancel(&fresh).unwrap() }), serde_json::json!({ "old": fixture["rebase"]["cancelOldKey"], "fresh": fixture["rebase"]["cancelFreshKey"] }));
        assert!(lease.try_claim_publication().is_none());
        lease.finish();
        assert_eq!(cancellations.active_operation_count(), 0);
    }

    #[test]
    fn retained_latest_wins_full_registry_reclaims_completed_targets_before_admission() {
        let fixture: Value = serde_json::from_str(include_str!("../🧫️fixtures/🔗️tool-latest-wins-integration.json")).unwrap();
        let cancellations = ToolCancellationHandle::default();
        let mut registry = ToolLatestWinsRegistry::new();
        let count = fixture["reclamation"]["sequentialCompletedTargets"].as_u64().unwrap();
        let mut accepted = 0;
        for index in 0..count {
            let operation = semio_framework_job::allocate_operation_id();
            let lease = cancellations
                .begin_keyed(ToolOperationKey { app_instance_id: 7, document: ArtifactDocumentAuthority(7), operation_id: operation, base_revision: semio_framework_job::RevisionId(1), generation: semio_framework_job::Generation(0) })
                .unwrap();
            let mut key = semio_framework_value::ordered::SharedOwner::admit(format!("target-{index:04}"), semio_framework_value::retained_clone::RetainedCloneGrant {maximum_items:1,maximum_copy_bytes:0,maximum_capacity_bytes:TYPED_OPERATION_RESULT_PAGE_BYTES,maximum_release_bytes:0,maximum_depth:1}).unwrap().0;
            let mut begun = false;
            let mut result = None;
            for _ in 0..100_000 {
                if !begun {
                    begun = registry.begin(operation.0, &key, &lease, RetainedCloneGrant { maximum_items:1, maximum_copy_bytes:4096, maximum_capacity_bytes:4096, maximum_release_bytes:4096, maximum_depth:semio_framework_value::ordered::MAX_AVL_HEIGHT * 5 + 16 }).unwrap().is_some();
                }
                if begun {
                    result = registry.take_outcome(operation.0);
                }
                if result.is_some() {
                    break;
                }
                advance_registry(&mut registry, "full-map retained admission");
            }
            assert_eq!(result, Some(true), "completed target {index} must not consume permanent admission capacity");
            assert!(key.release_step(semio_framework_value::retained_clone::RetainedCloneGrant::one_release_turn(key.next_release_byte_demand(),1)).unwrap().value.is_none());
            accepted += 1;
            for _ in 0..100_000 {
                if registry.can_begin() {
                    break;
                }
                advance_registry(&mut registry, "full-map retained reclamation");
            }
            assert!(registry.can_begin());
            lease.finish();
        }
        assert_eq!(serde_json::json!(accepted), fixture["reclamation"]["acceptedTargets"]);
        assert_eq!(cancellations.active_operation_count(), 0);
        registry.begin_close();
        for _ in 0..100_000 {
            if registry.terminal_is_empty() {
                break;
            }
            advance_registry(&mut registry, "full-map retained close");
        }
        assert!(registry.terminal_is_empty());
    }

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
    pub(super) async fn retained_latest_wins_slot_and_publication_fairness<A: ArtifactApp<Presence = PublicationPresence, PresenceMutation = PublicationPresenceMutation> + Default>() {
        let fixture: Value = serde_json::from_str(include_str!("../🧫️fixtures/🔗️tool-latest-wins-integration.json")).unwrap();
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
                    publication_lanes: &[ArtifactToolPublicationLane::Presence],
                    session: None,
                    session_rejected: None,
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
                    result_page: None,
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
        while let Some(page) = app.take_typed_operation_result_page(7) {
            let code = if page.lane == TypedOperationResultLane::Fault { semio_framework_plugin_decl_app::tool_contract::decode_typed_operation_fault_page(page.bytes()).code.0 } else { String::new() };
            let operation = page.token.operation;
            pages.push((operation, page.lane, code));
            if operation == 1 {
                let acknowledged = app.tools.tool_operations.get_mut(1).expect("the terminated operation stays mounted").acknowledge_result_page(page.token).expect("the terminated operation's own fault page is acknowledgeable");
                assert!(acknowledged, "a terminated operation must accept the ACK for the page it minted");
            }
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
                    publication_lanes: &[],
                    session: None,
                    session_rejected: None,
                    reserved_producer: None, completion: None,
                    completion_retirement:None,publication_retirement:None,output_retirement:None,raw_input: None,
                    output_chunks: None,
                    cancellation_lease: None,
                    identity:None,identity_progress:semio_framework_value::native_encoding::NativeEncodeProgress{completed:0,total:0,owned_bytes:0},
                    worker_semantic_pending: false, worker_outcome_pending: false, worker_fault_capture: None,
                    terminal_seen: true,
                worker_resume_pending: false,
                    publication: None,
                    pending_artifact_publication: None, pending_publication_outcome:PendingPublicationOutcome::new(),pending_window_config_receipt:None,cancellation_retirement:None,
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

    #[test]
    fn language_neutral_empty_single_max_and_plus_one_match_the_test_only_oracle() {
        let fixture: Value = serde_json::from_str(FIXTURE).expect("language-neutral typed-command fixture");
        let maximum = fixture["capacities"]["maxOutputBytes"].as_u64().expect("maximum output bytes") as usize;
        let owned = OwnedTypedCommandCensus;
        let oracle = SerdeJsonTypedCommandCensus;
        for case in fixture["cases"].as_array().expect("fixture cases") {
            let description = case["description"].as_str().expect("fixture description");
            let expected = TypedCommandCensusDecision { bytes: case["expectedBytes"].as_u64().expect("fixture expected bytes") as usize, accepted: case["accepted"].as_bool().expect("fixture acceptance") };
            assert_eq!(owned.decide(description, maximum), expected);
            assert_eq!(oracle.decide(description, maximum), expected);
        }
    }

    #[test]
    fn every_language_neutral_hostile_row_executes_the_owned_state_machine_and_serde_oracle() {
        let fixture: Value = serde_json::from_str(FIXTURE).expect("language-neutral typed-command fixture");
        assert_eq!(fixture["capacities"]["maxFaultBytes"].as_u64(), Some(TYPED_OPERATION_FAULT_BYTES as u64));
        assert_eq!(fixture["grantLaws"].as_array().expect("grant laws").len(), 4);
        assert_eq!(fixture["freshnessLaws"].as_array().expect("freshness laws").len(), 3);
        assert_eq!(fixture["admissionLaws"].as_array().expect("admission laws").len(), 4);
        assert_eq!(fixture["publicationLaws"].as_array().expect("publication laws").len(), 3);
        assert_eq!(fixture["laneTrace"].as_array().expect("lane trace").len(), 9);
        assert_eq!(fixture["rawPageLaws"].as_array().expect("raw page laws").len(), 3);
        assert_eq!(fixture["faultLaws"].as_array().expect("fault laws").len(), 3);
        assert_eq!(fixture["closeLaws"].as_array().expect("close laws").len(), 4);
        let declared_rows = fixture["cases"].as_array().expect("output census").len()
            + fixture["grantLaws"].as_array().expect("grant laws").len()
            + fixture["freshnessLaws"].as_array().expect("freshness laws").len()
            + fixture["admissionLaws"].as_array().expect("admission laws").len()
            + fixture["publicationLaws"].as_array().expect("publication laws").len()
            + fixture["laneTrace"].as_array().expect("lane trace").len()
            + fixture["rawPageLaws"].as_array().expect("raw page laws").len()
            + fixture["faultLaws"].as_array().expect("fault laws").len()
            + fixture["closeLaws"].as_array().expect("close laws").len();
        assert_eq!(declared_rows, 40, "the production seam assertions supplement, never replace, the forty language-neutral rows");

        for law in fixture["grantLaws"].as_array().expect("grant laws") {
            let phases = law["phases"].as_array().cloned().unwrap_or_else(|| vec![Value::Null]);
            for (phase, _) in phases.iter().enumerate() {
                let mut machine = OwnedFixtureMachine::new(phase);
                let actual = machine.grant(law["fuel"].as_u64().expect("grant fuel"), law["deadline"] == "expired", law["cancelled"].as_bool().expect("grant cancellation"));
                assert_eq!(actual, SerdeFixtureOracle(law).transition());
                assert_eq!(machine.advanced, law["advancedUnits"].as_u64().expect("advanced units") as usize);
            }
        }
        for law in fixture["freshnessLaws"].as_array().expect("freshness laws") {
            let accepted = OwnedFixtureMachine::freshness(
                law["operationRevision"].as_u64().expect("operation revision"),
                law["liveRevision"].as_u64().expect("live revision"),
                law["operationGeneration"].as_u64().expect("operation generation"),
                law["liveGeneration"].as_u64().expect("live generation"),
            );
            assert_eq!(accepted, law["accepted"].as_bool().expect("freshness oracle"));
        }
        for law in fixture["admissionLaws"].as_array().expect("admission laws") {
            let roots = law["roots"].as_u64().unwrap_or(fixture["capacities"]["roots"].as_u64().expect("root capacity"));
            let required = law["requiredRoots"].as_u64().unwrap_or(roots);
            let capacity = law["capacity"].as_u64().unwrap_or(1);
            let occupied = law["occupied"].as_u64().unwrap_or(0);
            assert_eq!(OwnedFixtureMachine::admission(capacity as usize, occupied as usize, roots as usize, required as usize), law["accepted"].as_bool().expect("admission oracle"));
        }
        for law in fixture["publicationLaws"].as_array().expect("publication laws") {
            let mut machine = OwnedFixtureMachine::new(0);
            let actual = machine.publish(law["attempt"].as_u64().expect("publication attempt") as u8, law["acknowledged"].as_bool().expect("publication ACK"), fixture["capacities"]["maximumRetries"].as_u64().expect("maximum retries") as u8);
            assert_eq!(actual, SerdeFixtureOracle(law).transition());
            assert_eq!(machine.retained_page, !law["acknowledged"].as_bool().expect("publication ACK"));
        }
        let mut lane_machine = OwnedFixtureMachine::new(0);
        for law in fixture["laneTrace"].as_array().expect("lane trace") {
            assert_eq!(law["sequence"].as_u64().expect("lane sequence") as usize, lane_machine.advanced);
            assert_eq!(law["items"].as_u64(), Some(1));
            assert_eq!(law["receipt"].as_bool(), Some(true));
            assert_eq!(lane_machine.publish(0, law["acknowledged"].as_bool().expect("lane ACK"), 2), FixtureTransition::Advance);
            lane_machine.advanced += 1;
            lane_machine.retained_page = true;
        }
        for law in fixture["rawPageLaws"].as_array().expect("raw page laws") {
            let declared = law["declaredBytes"].as_u64().expect("declared bytes") as usize;
            let mut admitted = 0usize;
            let mut returned = 0usize;
            for page in law["pageBytes"].as_array().expect("page bytes") {
                let page = page.as_u64().expect("page length") as usize;
                if admitted.checked_add(page).is_some_and(|next| next <= declared) {
                    admitted += page;
                } else {
                    returned += 1;
                }
            }
            if admitted != declared && returned == 0 {
                returned = law["pageBytes"].as_array().expect("page bytes").len();
            }
            let accepted = admitted == declared && returned == 0;
            assert_eq!(accepted, law["accepted"].as_bool().expect("raw page acceptance"));
            assert_eq!(returned, law["returnedOwners"].as_u64().expect("returned page owners") as usize);
        }
        for law in fixture["faultLaws"].as_array().expect("fault laws") {
            let mut input = String::new();
            for segment in law["segments"].as_array().expect("fault segments") {
                let scalar = segment["scalar"].as_str().expect("fault scalar");
                for _ in 0..segment["count"].as_u64().expect("fault scalar count") {
                    input.push_str(scalar);
                }
            }
            let mut oracle_bytes = 0usize;
            for scalar in input.chars() {
                let next = oracle_bytes + scalar.len_utf8();
                if next > TYPED_OPERATION_FAULT_BYTES {
                    break;
                }
                oracle_bytes = next;
            }
            let bounded = ArtifactBoundedToolFault::from_fault(&Fault::new(FaultOrigin::App, FaultCode::new("fixture"), input));
            assert_eq!(bounded.as_bytes().len(), oracle_bytes);
            assert_eq!(bounded.as_bytes().len(), law["retainedBytes"].as_u64().expect("retained fault bytes") as usize);
            assert!(std::str::from_utf8(bounded.as_bytes()).is_ok());
        }
        for law in fixture["closeLaws"].as_array().expect("close laws") {
            if let Some(value) = law["value"].as_str() {
                let mut value = value.to_string();
                let (items, bytes) = OwnedFixtureMachine::close_scalar(&mut value, law["maximumBytes"].as_u64().expect("close byte credit") as usize);
                assert_eq!(items, law["releasedItems"].as_u64().expect("released items") as usize);
                assert_eq!(bytes, law["releasedBytes"].as_u64().expect("released bytes") as usize);
                assert_eq!(value, law["remaining"].as_str().expect("remaining value"));
            } else {
                let mut machine = OwnedFixtureMachine::new(0);
                machine.owners = law["owners"].as_u64().expect("close owners") as usize;
                for _ in 0..law["calls"].as_u64().expect("close calls") {
                    machine.close_owner(law["maximumItems"].as_u64().expect("close item credit") as usize);
                }
                assert_eq!(machine.terminal, law["terminal"].as_bool().expect("terminal oracle"));
            }
        }
    }

    #[test]
    fn document_revision_change_between_ephemeral_preparation_turns_closes_without_publishing_the_lane_root() {
        let canonical_revision = [3; 32];
        let operation = semio_framework_job::Operation::new(semio_framework_job::OperationId(93), semio_framework_job::RevisionId(u64::from_be_bytes([3; 8])), semio_framework_job::Generation(0), 17);
        let mut presence = store::PresenceStore::<PublicationPresence, PublicationPresenceMutation>::new(PublicationPresence::default());
        let root_factory: std::sync::Arc<dyn store::SnapshotRetirementFactory<PublicationPresence>> = std::sync::Arc::new(PublicationPresenceLocalRootRetirementFactory);
        presence.install_local_retirement_factory(root_factory.clone()).unwrap();
        let initial_root = presence.local_read().unwrap();
        let factory = TwoTurnPublicationPresencePreparationFactory;
        let mut publication = presence.begin_publish_one(operation.operation, 0, ChangePublicationPresence { revision: 1 }.into(), Some(&factory), Some(root_factory.clone())).expect("two-turn presence publication admits");
        let grant = store::os_store::one_item::ArtifactStoreOneItemGrant { maximum_items: 1, maximum_copy_bytes: 64, maximum_capacity_bytes: 64, maximum_release_bytes: 64, maximum_depth: 128 };
        assert!(matches!(presence.advance_publish_one(&mut publication, grant), Ok(store::os_store::one_item_publication::ArtifactStoreOneItemAdvance::Progress(_))));
        assert!(std::ptr::eq(initial_root.get(), presence.local()));
        assert_eq!(presence.generation_now(), 0);

        let changed_document_revision = [4; 32];
        assert!(!typed_operation_document_is_fresh(&operation, canonical_revision, changed_document_revision, 0));
        publication.begin_close();
        for _ in 0..4 {
            if matches!(publication.close_step(grant).expect("stale pending publication closes"),RetainedCloneStep::Complete(_)) {
                break;
            }
        }
        assert!(publication.terminal_is_empty());
        assert!(std::ptr::eq(initial_root.get(), presence.local()));
        assert_eq!(presence.generation_now(), 0);
        drop(initial_root);
        let mut close = presence.begin_retirement(std::sync::Arc::new(PublicationPresence::default()), |_| true).ok().unwrap();
        for _ in 0..2048 {
            if matches!(close.close_step(RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:4096,maximum_capacity_bytes:4096,maximum_release_bytes:4096,maximum_depth:64}).unwrap(),RetainedCloneStep::Complete(_)) {
                break;
            }
        }
        assert!(close.terminal_is_empty());
    }

    #[test]
    fn private_child_genesis_wire_retains_exact_identity_and_indivisible_backing() {
        let fixture: Value = serde_json::from_str(include_str!("../../🧩️composition/📨️emission/🌱️genesis/🧪️tests/🧫️fixtures/🔣️.json")).expect("neutral private genesis contract");
        let source=&fixture["source"];
        let reference=&source["reference"];
        let genesis=ChildEmitGenesis {
            reference: ArtifactRef { artifact_id:reference["artifactId"].as_str().unwrap().into(), dialect: ArtifactDialect { artifact_kind:reference["dialect"]["artifactKind"].as_str().unwrap().into(), standard:reference["dialect"]["standard"].as_str().unwrap().into(), subset:reference["dialect"]["subset"].as_str().unwrap().into() } },
            initial_pack: serde_json::to_vec(&fixture["initial"]["brep"]).unwrap(),
        };
        let mut child=ChildEmit::open(source["slot"].as_str().unwrap(),source["childId"].as_str().unwrap(),0);
        child.genesis=Some(genesis);
        assert_ne!(child.child_id,child.genesis.as_ref().unwrap().reference.artifact_id);
        let independent=serde_json::to_value(&child).expect("independent exact genesis projection");
        assert_eq!(independent["genesis"]["reference"],source["reference"]);
        assert_eq!(Value::from(semio_framework_value::ToValue::to_value(&child)),independent);
        let wire=ChildEmit::encode_groups(std::slice::from_ref(&child));
        let mut decoded=ChildEmit::decode_groups(&wire).expect("exact genesis wire roundtrip");
        assert_eq!(decoded[0],child);
        let pointer=child.genesis.as_ref().unwrap().initial_pack.as_ptr();
        let bytes=child.retirement_demands().unwrap().release_bytes;
        assert!(bytes>0);
        use semio_framework_value::retained_clone::{RetainedCloneGrant, RetainedCloneProgress, RetainedCloneStep};
        let yielded=RetainedCloneStep::Progress(RetainedCloneProgress::default());
        for _ in 0..4 {
            assert_eq!(child.close_one(RetainedCloneGrant::one_release_turn(bytes-1,1)).unwrap(),yielded);
            assert_eq!(child.genesis.as_ref().unwrap().initial_pack.as_ptr(),pointer);
            assert_eq!(child.genesis.as_ref().unwrap().reference.artifact_id,reference["artifactId"].as_str().unwrap());
        }
        assert_eq!(child.close_one(RetainedCloneGrant{maximum_items:0,..RetainedCloneGrant::one_release_turn(bytes,1)}).unwrap(),yielded);
        assert_eq!(child.close_one(RetainedCloneGrant::one_release_turn(bytes,1)).unwrap(),RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,released_bytes:bytes,..Default::default()}));
        for child in std::iter::once(&mut child).chain(decoded.iter_mut()) {
            for _ in 0..32 {
                let grant=RetainedCloneGrant::one_release_turn(child.retirement_demands().unwrap().release_bytes,1);
                match child.close_one(grant).unwrap() {
                    RetainedCloneStep::Progress(progress)=>assert!(progress.fits(grant)),
                    RetainedCloneStep::Complete(_)=>break,
                }
            }
            assert!(child.genesis.is_none());
            assert_eq!(child.retirement_demands().unwrap().release_bytes,0);
        }
    }

    #[test]
    fn retained_child_wire_rejection_retires_nested_owners_under_the_production_grant() {
        let fixture: Value = serde_json::from_str(include_str!("../../../🏪️store/🧪️tests/🧫️fixtures/📢️member-publication.json")).expect("retained child fixture");
        let row = &fixture["orderedMembers"][0];
        let mut wire = row["wire"].as_str().unwrap().as_bytes().to_vec();
        wire.resize(wire.len() + row["paddingBytes"].as_u64().unwrap() as usize, b' ');
        let wire_bytes = wire.len();
        let mut child = ChildEmit {
            genesis: None,
            owner: String::new(),
            slot: "slot".into(),
            child_id: "child".into(),
            ops: vec![wire],
            op_schema: SchemaId("demo.member.json-number".into()),
            labels: vec![LocalizedLabel::native("ä🧩", "ß🎯")],
        };
        use semio_framework_value::retained_clone::{RetainedCloneProgress, RetainedCloneStep};
        let page = RetainedCloneGrant { maximum_items:1, maximum_copy_bytes:4096, maximum_capacity_bytes:4096, maximum_release_bytes:4096, maximum_depth:semio_framework_value::ordered::MAX_AVL_HEIGHT * 5 + 16 };
        assert_eq!(child.close_one(semio_framework_value::retained_clone::RetainedCloneGrant { maximum_items: 0, ..page }).unwrap(), RetainedCloneStep::Progress(RetainedCloneProgress::default()));
        assert_eq!(child.ops[0].len(), wire_bytes);
        let mut bytes = 0;
        let mut complete = false;
        for _ in 0..wire_bytes + 128 {
            match child.close_one(page).unwrap() {
                RetainedCloneStep::Progress(progress) => {
                    assert!(progress.fits(page));
                    bytes += progress.released_bytes;
                }
                RetainedCloneStep::Complete(_) => {
                    complete = true;
                    break;
                }
            }
        }
        assert!(complete);
        assert!(bytes >= wire_bytes);
        assert!(child.ops.is_empty() && child.labels.is_empty() && child.slot.is_empty() && child.child_id.is_empty() && child.op_schema.0.is_empty());
    }

    #[test]
    fn typed_child_publication_has_one_retained_async_group_owner_and_never_clone_publishes() {
        let source = instance_source();
        let child_start = source.rfind("async fn publish_mounted_typed_child_operation_unit(").expect("retained child publisher");
        let child_end = source[child_start..].find("fn publish_mounted_typed_operation_unit(").map(|offset| child_start + offset).expect("retained child publisher end");
        let child_publisher = &source[child_start..child_end];
        for retained_seam in
            ["PendingChildGroupPublicationPhase::Ready", "captured_child_content_generation", "captured.identity_digest()", "try_claim_publication", "begin_dispatch", "&pending.child_emits", "dispatch_emit_group(", "TypedOperationResultLane::Child"]
        {
            assert!(child_publisher.contains(retained_seam), "retained Child publication lost its exact owner seam: {retained_seam}");
        }
        for forbidden in ["child_emits.last().cloned()", "vec![child.clone()]", "for child in pending.child_emits.clone()"] {
            assert!(!child_publisher.contains(forbidden), "retained Child publication restored clone-then-publish: {forbidden}");
        }
        let publisher_start = child_end;
        let publisher_end = source[publisher_start..].find("fn require_tool_operation_authority(").map(|offset| publisher_start + offset).expect("typed publisher end");
        let publisher = &source[publisher_start..publisher_end];
        let parent_guard = publisher.find("if emit.child_emits.is_empty()").expect("parent publication group guard");
        let retained_install = publisher.find("PendingChildGroupPublication::new(").expect("retained child owner install");
        assert!(parent_guard < retained_install, "parent mutations must remain group-owned whenever Child output is present");
        assert!(source.contains("ArtifactToolPublicationLane::Child => None"));
    }

    /// ⚖️ The generic command is constructed to be ENCODED: `admit_command_wire` admits the command's
    /// own wire, so the decoder necessarily precedes its admission. What the clause has always stood
    /// for — no generically constructed command reaches the reducer without the complete pipeline —
    /// is asserted directly, as the order decoder → gate → reducer entry.
    #[test]
    fn full_operation_source_rejects_generic_reducers_and_old_monolithic_shells() {
        let source = [module_source("🧵️retained-command/🔄️full-operation/🦀️.rs"), module_source("👷️job/🛬️lifted/🔄️full-operation/🦀️.rs"), instance_source()].concat();
        let start = source.find("impl<A: ArtifactApp> semio_framework_job::InteractiveJob for TypedCommandFullOperationJob<A>").expect("typed full-operation job");
        let end = source[start..].find("struct TypedCommandFullOperationJobFactory").map(|offset| start + offset).expect("typed full-operation factory");
        let job = &source[start..end];
        for stage in ["typed-command-prepare", "typed-command-reducer"] {
            assert!(job.contains(stage));
        }
        for forbidden in ["A::handle", "A::ephemeral", "bounded_command_output_bytes", "serde_json::to_vec", "fault.message.as_bytes().to_vec()", "for child in emit.child_emits", "ActiveToolCommand", "BoundedFirstStepCommandJob"] {
            assert!(!job.contains(forbidden), "forbidden typed-command shortcut returned: {forbidden}");
        }
        let route = source.rfind("async fn dispatch_typed_command_inner").expect("typed command route");
        let gate = source[route..].find("self.require_complete_tool_operation_pipeline(&admission)?").expect("fail-closed full-operation gate");
        let refresh = source[route..].find("self.refresh_cache().await").expect("deferred legacy preparation census");
        assert!(gate < refresh, "incomplete typed command must fail before legacy preparation");
        for decoder in ["let command = Box::new(A::command_from_action(action, source_args).await?)", "let command = A::command_from_action(command_id, Some(args)).await"] {
            let decoder = source.find(decoder).expect("typed route decoder");
            let gate = source[decoder..].find("self.require_complete_tool_operation_pipeline(&admission)?").expect("typed route full-operation gate") + decoder;
            let reducer = source[decoder..].find("self.dispatch_typed_command_inner(").expect("typed route reducer entry") + decoder;
            assert!(gate < reducer, "generic command construction must not reach the reducer before full-operation admission");
        }
        let intent = source.find("async fn handle_intent_frame(&mut self, intent: &UiIntent").expect("intent route");
        let intent_end = source[intent..].find("async fn resume_task_emit").map(|offset| intent + offset).expect("intent route end");
        let intent_route = &source[intent..intent_end];
        for forbidden in ["serde_json::to_value", "serde_json::to_vec", "A::command_from_intent"] {
            assert!(!intent_route.contains(forbidden), "intent route performed pre-admission work: {forbidden}");
        }
        assert!(source.contains("pub fn dispatch_typed"), "typed-value route must remain available to the dependency testkit");
    }

    /// ♻️ The seven per-lane `Closing` arms collapsed into ONE shared
    /// `MountedTypedCommandFullOperation::granted_retirement_step` (ticket 26/09/09/PROCEDURAL-3D-END-TO-END),
    /// so the retained one-item retirement seam is that call, not a per-lane `close_step`.
    ///
    /// 🧵️ The one-page publisher suspends in EXACTLY one place: the task lane's spawn. Every
    /// other lane stays synchronous, so the awaits are enumerated and the one that is allowed is
    /// named by its own call — a new `.await` anywhere in the publisher fails here by name.
    #[test]
    fn fixture_contract_is_anchored_to_the_production_retained_factory_publisher_and_host_receivers() {
        let source = instance_source();
        let admission_marker = ["async fn admit_command_json_with_", "proof(&self"].concat();
        let admission_start = source.rfind(&admission_marker).expect("production raw JSON admission");
        let admission_end = source[admission_start..].find("pub async fn new(app: A, actor").map(|offset| admission_start + offset).expect("admission route end");
        let admission = &source[admission_start..admission_end];
        let reserve = admission.find(".begin_exact_wire").expect("maximum extent authority");
        let encode = admission.find("semio_framework_pack_json::to_json_string(&(verb, wire_args))").expect("bounded wire encoder");
        let seal = admission.find("finish_admitted_prefix").expect("truthful exact prefix");
        assert!(reserve < encode && encode < seal);

        let dispatch_start = source.rfind("async fn dispatch_typed_command_inner").expect("production typed dispatch");
        let dispatch_end = source[dispatch_start..].find("pub fn dispatch_typed").map(|offset| dispatch_start + offset).expect("production dispatch end");
        let dispatch = &source[dispatch_start..dispatch_end];
        assert!(dispatch.contains("dispatch_wire_retained_with_spec"));
        assert!(dispatch.contains("retained_wire.take()"));

        let publisher_start = source.rfind("fn publish_mounted_typed_operation_unit").expect("production one-page publisher");
        let publisher_end = source[publisher_start..].find("fn require_tool_operation_authority").map(|offset| publisher_start + offset).expect("publisher end");
        let publisher = &source[publisher_start..publisher_end];
        for retained_seam in ["pending_artifact_publication", "begin_apply_batch", "advance_apply_batch", "ArtifactStoreOneItemAdvance::Published", "pending_publication_outcome.pending()"] {
            assert!(publisher.contains(retained_seam), "production publisher lost its retained one-item seam: {retained_seam}");
        }
        for forbidden in [".apply_one(", "artifact_mutations.last().cloned()", "config_mutations.last().cloned()", "draft_mutations.last().cloned()", "presence.last().cloned()", "transient.last().cloned()"] {
            assert!(!publisher.contains(forbidden), "production publisher restored a monolithic or clone-then-pop shortcut: {forbidden}");
        }
        let freshness = publisher.find("typed-operation pending publication rejected a stale immutable document root").expect("per-turn document freshness gate");
        let pending_advance = publisher[freshness..].find("\n            if let Some(pending) = mounted.pending_artifact_publication.as_mut()").map(|offset| freshness + offset).expect("pending publication advance");
        assert!(freshness < pending_advance, "document freshness must fail closed before every retained lane advance");
        assert!(publisher[..pending_advance].contains("pending.begin_close()"));
        assert!(publisher[..pending_advance].contains("self.handoff_mounted_durable_publication(mounted)?"));
        assert!(!publisher.contains("dispatch_emit_group("));
        let publisher_awaits: Vec<&str> = publisher.match_indices(".await").map(|(index, _)| publisher[..index].rsplit('\n').next().unwrap_or_default().trim()).collect();
        assert_eq!(
            publisher_awaits,
            vec!["crate::reactor::spawn_task(mounted.meta.instance_id, &mounted.meta, task)"],
            "the production one-page publisher may suspend only for the task lane's own bounded spawn"
        );
        assert!(publisher.contains("typed_effect_outbox.push"));
        assert!(publisher.contains("typed_event_outbox.push"));
        assert!(publisher.contains("typed_ui_outbox.push"));
        assert!(source.contains("mounted.publication_attempt = mounted.publication_attempt.saturating_add(1)"));
        assert!(source.contains("take_typed_operation_effect"));
        assert!(source.contains("take_typed_operation_event"));
        assert!(source.contains("take_typed_operation_ui_scope"));

        let reactor = [module_source("⚛️reactor/🦀️.rs"), module_source("⚛️reactor/🔄️turn/🦀️.rs")].concat();
        assert!(reactor.contains("output.typed_operation_results.iter()"));
        assert!(reactor.contains("page.renderer_exchange_bytes()"));
        assert!(reactor.contains("TypedOperationResultPage::renderer_ack_token"));
        assert!(reactor.contains("plugin_acknowledge_typed_operation_result(runtime, token)"));
        assert!(!reactor.contains("typed_operation_results: _"), "the old typed-result drop route must remain unreachable");
    }

    #[test]
    fn language_neutral_renderer_page_and_exact_ack_have_bounded_stable_wire_fields() {
        let token = TypedOperationResultToken { receiver: u32::MAX, operation: u64::MAX, generation: u64::MAX - 1, sequence: u32::MAX, attempt: 2 };
        let page = TypedOperationResultPage::try_new(token, TypedOperationResultLane::Terminal, &[0x5a; TYPED_OPERATION_RESULT_PAGE_BYTES]).expect("exact renderer page maximum");
        let wire = page.renderer_exchange_bytes();
        assert!(wire.starts_with(TypedOperationResultPage::RENDERER_PAGE_MAGIC));
        assert_eq!(&wire[wire.len() - TYPED_OPERATION_RESULT_PAGE_BYTES..], &[0x5a; TYPED_OPERATION_RESULT_PAGE_BYTES]);

        let fixture: serde_json::Value = serde_json::from_str(include_str!("../../../../../../🔨️modules/🎭️actor/🧪️tests/🗞️typed-operation-page/🧫️fixtures/🔣️.json")).expect("neutral result lanes");
        for row in fixture["lanes"].as_array().expect("result lanes") {
            let lane: TypedOperationResultLane = semio_framework_pack_json::from_json_str(&serde_json::to_string(&row["name"]).unwrap(), semio_framework_pack_json::JsonMemberPolicy::Reject).expect("own lane decoder");
            let page = TypedOperationResultPage::try_new(token, lane, &[0x5a]).expect("declared result lane");
            let bytes = page.renderer_exchange_bytes();
            assert_eq!(bytes[TypedOperationResultPage::RENDERER_PAGE_MAGIC.len() + 25], row["tag"].as_u64().unwrap() as u8);
        }

        let mut ack = Vec::from(TypedOperationResultPage::RENDERER_ACK_MAGIC);
        ack.extend_from_slice(&token.receiver.to_le_bytes());
        ack.extend_from_slice(&token.operation.to_le_bytes());
        ack.extend_from_slice(&token.generation.to_le_bytes());
        ack.extend_from_slice(&token.sequence.to_le_bytes());
        ack.push(token.attempt);
        assert_eq!(TypedOperationResultPage::renderer_ack_token(&ack), Some(token));
        ack.push(0);
        assert_eq!(TypedOperationResultPage::renderer_ack_token(&ack), None);
    }

    #[test]
    fn host_configuration_uses_one_bounded_event_sourced_lane_before_the_generic_gate() {
        let source = instance_source();
        let start = source.find("pub(crate) async fn dispatch_action").expect("action route");
        let end = source[start..].find("async fn dispatch_command(").map(|offset| start + offset).expect("command route");
        let route = &source[start..end];
        let hook = route.find("A::host_configuration_mutation(action, args)?").expect("host configuration owner hook");
        let admission = route[hook..].find("self.admit_host_configuration_json(action, args).await?").expect("bounded host admission") + hook;
        let authority = route[admission..].find("self.require_tool_operation_authority(&admission)?").expect("bounded host authority") + admission;
        let event = route[authority..].find("Emit::config(vec![config_mutation])").expect("single event-sourced configuration mutation") + authority;
        let generic_gate = route[event..].find("self.require_complete_tool_operation_pipeline(&admission)?").expect("generic retained operation gate") + event;
        assert!(hook < admission && admission < authority && authority < event && event < generic_gate);

        let command_start = end;
        let command_end = source[command_start..].find("async fn fill_agent_revisions(").map(|offset| command_start + offset).expect("manifest command route end");
        let command_route = &source[command_start..command_end];
        let command_hook = command_route.find("A::host_configuration_mutation(command_id, Some(&args))?").expect("manifest command host configuration owner hook");
        let command_admission = command_route[command_hook..].find("self.admit_host_configuration_json(command_id, Some(&args)).await?").expect("manifest command bounded host admission") + command_hook;
        let command_authority = command_route[command_admission..].find("self.require_tool_operation_authority(&admission)?").expect("manifest command bounded host authority") + command_admission;
        let command_event = command_route[command_authority..].find("Emit::config(vec![config_mutation])").expect("manifest command single event-sourced configuration mutation") + command_authority;
        let command_generic_gate = command_route[command_event..].find("self.require_complete_tool_operation_pipeline(&admission)?").expect("manifest command generic retained operation gate") + command_event;
        assert!(command_hook < command_admission && command_admission < command_authority && command_authority < command_event && command_event < command_generic_gate);

        let proof_start = source.find("fn qualified_host_configuration_tool_proof").expect("host configuration proof resolver");
        let proof_end = source[proof_start..].find("async fn admit_command_wire_with_proof").map(|offset| proof_start + offset).expect("host configuration proof resolver end");
        let proof = &source[proof_start..proof_end];
        assert!(proof.contains("QualifiedToolProof::Bounded(proof.clone())"));
        assert!(!proof.contains("interactive-job.missing-owned-reducer"));
    }
}


