mod typed_command_full_operation_tests {
    use super::*;
    use crate::publication_fixture::{ChangePublicationPresence, PublicationPresence, PublicationPresenceMutation};

    const FIXTURE: &str = include_str!("../../🧵️retained-command/🔄️full-operation/🧫️fixtures/🔣️.json");

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    struct TypedCommandCensusDecision {
        bytes: usize,
        accepted: bool,
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
        phase: usize,
        advanced: usize,
        retained_page: bool,
        owners: usize,
        terminal: bool,
    }

    impl OwnedFixtureMachine {
        fn new(phase: usize) -> Self {
            Self { phase, advanced: 0, retained_page: true, owners: 0, terminal: false }
        }

        fn grant(&mut self, fuel: u64, expired: bool, cancelled: bool) -> FixtureTransition {
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

        fn freshness(operation_revision: u64, live_revision: u64, operation_generation: u64, live_generation: u64) -> bool {
            operation_revision == live_revision && operation_generation == live_generation
        }

        fn admission(capacity: usize, occupied: usize, roots: usize, required_roots: usize) -> bool {
            occupied < capacity && roots == required_roots
        }

        fn publish(&mut self, attempt: u8, acknowledged: bool, maximum_retries: u8) -> FixtureTransition {
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

        fn close_owner(&mut self, maximum_items: usize) -> (usize, usize) {
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
        fn transition(&self) -> FixtureTransition {
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

    struct TwoTurnPublicationPresencePreparationFactory;

    struct TwoTurnPublicationPresencePreparation {
        request: Option<store::ArtifactEphemeralOneItemPreparationRequest<PublicationPresence, PublicationPresenceMutation>>,
        prepared: Option<store::ArtifactEphemeralOneItemPrepared<PublicationPresence>>,
        checkpoint: store::ArtifactStoreOneItemCheckpoint,
        turn: u8,
        closing: bool,
    }

    impl store::ArtifactEphemeralOneItemPreparationFactory<PublicationPresence, PublicationPresenceMutation> for TwoTurnPublicationPresencePreparationFactory {
        fn preflight(&self, _mutation: &PublicationPresenceMutation) -> Result<store::ArtifactStoreOneItemFootprint, String> {
            Ok(store::ArtifactStoreOneItemFootprint { work_items: 2, retained_bytes: 64 })
        }

        fn begin(
            &self,
            request: store::ArtifactEphemeralOneItemPreparationRequest<PublicationPresence, PublicationPresenceMutation>,
        ) -> Result<Box<dyn store::ArtifactEphemeralOneItemPreparation<PublicationPresence, PublicationPresenceMutation>>, store::ArtifactEphemeralOneItemPreparationRequest<PublicationPresence, PublicationPresenceMutation>> {
            Ok(Box::new(TwoTurnPublicationPresencePreparation { request: Some(request), prepared: None, checkpoint: store::ArtifactStoreOneItemCheckpoint::default(), turn: 0, closing: false }))
        }
    }

    impl store::ArtifactEphemeralOneItemPreparation<PublicationPresence, PublicationPresenceMutation> for TwoTurnPublicationPresencePreparation {
        fn advance(&mut self, grant: store::ArtifactStoreOneItemGrant) -> Result<store::ArtifactStoreOneItemPreparationStep, String> {
            if !grant.permits_one() {
                return Ok(store::ArtifactStoreOneItemPreparationStep::Blocked);
            }
            if self.turn == 0 {
                self.turn = 1;
                self.checkpoint = store::ArtifactStoreOneItemCheckpoint { cursor: 1, completed_items: 1, completed_bytes: 1, digest: [1; 32] };
                return Ok(store::ArtifactStoreOneItemPreparationStep::Progress(self.checkpoint));
            }
            if self.prepared.is_none() {
                let request = self.request.take().ok_or_else(|| "two-turn publication-presence preparation lost its owner bundle".to_string())?;
                let next_root = protocol::apply_diff(request.mutation.diff(request.base.as_ref()).diff(), request.base.as_ref()).map_err(|error| error.to_string())?;
                self.prepared = Some(store::ArtifactEphemeralOneItemPrepared { next_root: std::sync::Arc::new(next_root) });
                self.checkpoint = store::ArtifactStoreOneItemCheckpoint { cursor: 2, completed_items: 2, completed_bytes: 2, digest: [2; 32] };
            }
            Ok(store::ArtifactStoreOneItemPreparationStep::Prepared(self.checkpoint))
        }

        fn checkpoint(&self) -> store::ArtifactStoreOneItemCheckpoint {
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

        fn close_step(&mut self, grant: store::ArtifactStoreOneItemGrant) -> Result<store::SnapshotRetirementStep, semio_framework_value::ValueError> {
            if !self.closing || grant.maximum_items == 0 {
                return Ok(store::SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 });
            }
            if self.prepared.take().is_some() || self.request.take().is_some() {
                return Ok(store::SnapshotRetirementStep::Pending { released_items: 1, released_bytes: 0 });
            }
            Ok(store::SnapshotRetirementStep::Complete)
        }

        fn terminal_is_empty(&self) -> bool {
            self.closing && self.request.is_none() && self.prepared.is_none()
        }
    }

    struct PublicationPresenceLocalRootRetirement {
        root: Option<std::sync::Arc<PublicationPresence>>,
    }

    impl store::ErasedSnapshotRetirement for PublicationPresenceLocalRootRetirement {
        fn close_step(&mut self, maximum_items: usize, _maximum_bytes: usize) -> Result<store::SnapshotRetirementStep, semio_framework_value::ValueError> {
            if maximum_items == 0 {
                return Ok(store::SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 });
            }
            if self.root.take().is_some() {
                return Ok(store::SnapshotRetirementStep::Pending { released_items: 1, released_bytes: 0 });
            }
            Ok(store::SnapshotRetirementStep::Complete)
        }

        fn terminal_is_empty(&self) -> bool {
            self.root.is_none()
        }
    }

    #[derive(semio_framework_value::FactoryPayloadRetirement)]
    struct PublicationPresenceLocalRootRetirementFactory;

    impl store::SnapshotRetirementFactory<PublicationPresence> for PublicationPresenceLocalRootRetirementFactory {
    fn retirement_birth_bytes(&self, _snapshot: &std::sync::Arc<PublicationPresence>) -> usize { std::mem::size_of::<PublicationPresenceLocalRootRetirement>() }

        fn retire(&self, snapshot: std::sync::Arc<PublicationPresence>) -> Box<dyn store::ErasedSnapshotRetirement> {
            Box::new(PublicationPresenceLocalRootRetirement { root: Some(snapshot) })
        }
    }

    pub(super) async fn retained_cancellation_publication_boundaries<A: ArtifactApp<Presence = PublicationPresence, PresenceMutation = PublicationPresenceMutation> + Default>() {
        let fixture: Value = serde_json::from_str(include_str!("../../🧫️fixtures/🥇️tool-latest-wins.json")).expect("language-neutral cancellation boundaries");
        let grant = store::ArtifactStoreOneItemGrant { maximum_items: fixture["maximumItems"].as_u64().unwrap() as usize, maximum_bytes: fixture["maximumBytes"].as_u64().unwrap() as usize };
        assert_eq!(grant.maximum_items, 1);
        assert_eq!(grant.maximum_bytes, TYPED_OPERATION_RESULT_PAGE_BYTES);
        for case in fixture["publicationCases"].as_array().unwrap() {
            let boundary = case["cancelAt"].as_str().unwrap();
            let mut app = VcsArtifactApp::<A>::new(A::default(), protocol::ActorId(crate::app::LOCAL_ACTOR_ID.into())).await;
            let revision = app.store.content_revision_now();
            let operation = semio_framework_job::Operation::new(
                semio_framework_job::allocate_operation_id(),
                semio_framework_job::RevisionId(u64::from_be_bytes(revision[..8].try_into().unwrap())),
                semio_framework_job::Generation(app.store.generation_now()),
                17,
            );
            let cancellations = app.tool_cancellations.clone();
            let lease = cancellations.begin(ToolOperationKey { app_instance_id: 7, document: ArtifactDocumentAuthority(7), operation_id: operation.operation, base_revision: operation.base_revision, generation: operation.generation }).unwrap();
            let before = app.presence_store.local_read().unwrap();
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
                raw_input: None,
                output_chunks: None,
                cancellation_lease: Some(lease),
                terminal_outcome: None,
                terminal_seen: true,
                publication: Some(ArtifactToolCompletionValue::Emit(Ok(Emit::default()), EphemeralEmit::default())),
                pending_artifact_publication: None,
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
                publication_checkpoint: None,
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
                terminal_fault: None,
                stage: MountedTypedCommandFullOperationStage::Publishing,
            };
            if boundary != "producer" {
                let pending =
                    app.presence_store.begin_publish_one(operation.operation, 0, ChangePublicationPresence { revision: 1 }.into(), Some(&TwoTurnPublicationPresencePreparationFactory), app.presence_local_root_retirement_factory.clone()).unwrap();
                mounted.pending_artifact_publication = Some(PendingArtifactStorePublication::Presence(pending));
                let target = match boundary {
                    "preparation" => store::ArtifactStoreOneItemPublicationPhase::Preparing,
                    "preflight" => store::ArtifactStoreOneItemPublicationPhase::PreflightingCommit,
                    "publishing" => store::ArtifactStoreOneItemPublicationPhase::Publishing,
                    "awaitingAck" => store::ArtifactStoreOneItemPublicationPhase::AwaitingAck,
                    other => panic!("unknown publication boundary {other}"),
                };
                for _ in 0..8 {
                    let Some(PendingArtifactStorePublication::Presence(pending)) = mounted.pending_artifact_publication.as_ref() else {
                        panic!("exact pending presence owner");
                    };
                    if pending.phase() == target {
                        break;
                    }
                    app.publish_mounted_typed_operation_unit(&mut mounted).await.unwrap();
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
            app.publish_mounted_typed_operation_unit(&mut mounted).await.unwrap();
            assert_eq!(mounted.stage, MountedTypedCommandFullOperationStage::AwaitingAck);
            let cancelled = mounted.result_page.as_ref().unwrap();
            assert_eq!(cancelled.lane, TypedOperationResultLane::Fault);
            assert!(std::str::from_utf8(cancelled.bytes()).unwrap().contains("cancelled"));
            assert_eq!(serde_json::json!({ "committed": app.presence_store.generation_now() != 0 }), serde_json::json!({ "committed": case["committed"] }));
            assert_eq!(std::ptr::eq(before.get(), app.presence_store.local()), !case["committed"].as_bool().unwrap());
            assert!(!mounted.ui_pending);
            let token = cancelled.token;
            assert!(mounted.acknowledge_result_page(token).unwrap());
            for _ in 0..64 {
                if mounted.terminal_is_empty() {
                    break;
                }
                match mounted.retirement_step(grant.maximum_items, grant.maximum_bytes).unwrap() {
                    PluginCloseStep::Pending { released_items, released_bytes } => {
                        assert!(released_items <= 1);
                        assert!(released_bytes <= TYPED_OPERATION_RESULT_PAGE_BYTES);
                    }
                    PluginCloseStep::Complete => break,
                    PluginCloseStep::AwaitingInput { reason } => panic!("cancelled exact publication close awaited input: {reason}"),
                    PluginCloseStep::Blocked { reason } => panic!("cancelled exact publication close blocked: {reason}"),
                }
            }
            assert!(mounted.terminal_is_empty(), "{boundary}");
            assert_eq!(cancellations.active_operation_count(), 0);
            drop(before);
            for _ in 0..100_000 {
                if app.close_terminal_is_empty() {
                    break;
                }
                match app.close_step(grant.maximum_items, grant.maximum_bytes).unwrap() {
                    PluginCloseStep::Pending { released_items, released_bytes } => {
                        assert!(released_items <= 1);
                        assert!(released_bytes <= TYPED_OPERATION_RESULT_PAGE_BYTES);
                    }
                    PluginCloseStep::Complete => break,
                    PluginCloseStep::AwaitingInput { reason } => panic!("cancelled publication app close awaited input: {reason}"),
                    PluginCloseStep::Blocked { reason } => panic!("cancelled publication app close blocked: {reason}"),
                }
            }
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
                if pending.phase() == store::ArtifactStoreOneItemPublicationPhase::Publishing {
                    break;
                }
                assert!(matches!(presence.advance_publish_one(&mut pending, grant).unwrap(), store::ArtifactStoreOneItemAdvance::Progress(_)));
            }
            assert_eq!(pending.phase(), store::ArtifactStoreOneItemPublicationPhase::Publishing);
            let mut permit = if case["claimFirst"].as_bool().unwrap() { Some(lease.try_claim_publication().unwrap()) } else { None };
            assert!(cancellations.cancel(&key).unwrap());
            if !case["claimFirst"].as_bool().unwrap() {
                permit = lease.try_claim_publication();
            }
            if permit.is_some() {
                assert!(matches!(presence.advance_publish_one(&mut pending, grant).unwrap(), store::ArtifactStoreOneItemAdvance::Published(_)));
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
                if close.close_step(1, 4096).unwrap() == store::SnapshotRetirementStep::Complete {
                    break;
                }
            }
            assert!(close.terminal_is_empty());
        }
    }

    fn fixture_latest_wins_key(scope: &Value) -> semio_framework_value::ordered::SharedOwner<String> {
        let parts = [scope["document"].as_str().unwrap(), scope["controller"].as_str().unwrap(), scope["tool"].as_str().unwrap(), scope["target"].as_str().unwrap()];
        let mut copy = ToolLatestWinsKeyCopy::new(scope["instance"].as_u64().unwrap() as u32, parts).unwrap();
        assert_eq!(copy.advance(parts, 0, 4_096), PluginCloseStep::Pending { released_items: 0, released_bytes: 0 });
        assert_eq!(copy.advance(parts, 1, 0), PluginCloseStep::Pending { released_items: 0, released_bytes: 0 });
        for _ in 0..100_000 {
            match copy.advance(parts, 1, TYPED_OPERATION_RESULT_PAGE_BYTES) {
                PluginCloseStep::Complete => return copy.take_key(TYPED_OPERATION_RESULT_PAGE_BYTES).unwrap(),
                PluginCloseStep::Pending { released_items, released_bytes } => {
                    assert!(released_items <= 1);
                    assert!(released_bytes <= 4_096);
                }
                PluginCloseStep::AwaitingInput { reason } => panic!("full-domain key awaited input: {reason}"),
                PluginCloseStep::Blocked { reason } => panic!("full-domain key blocked: {reason}"),
            }
        }
        panic!("full-domain key did not progress under the production grant")
    }

    #[test]
    fn latest_wins_owned_key_retains_zero_and_subexact_headers() {
        use semio_framework_value::{ordered::SharedOwner, retained_clone::RetainedCloneGrant};
        let fixture:Value=serde_json::from_str(include_str!("../../🧫️fixtures/🥇️tool-latest-wins.json")).unwrap();
        let scope=&fixture["first"];
        let parts=[scope["document"].as_str().unwrap(),scope["controller"].as_str().unwrap(),scope["tool"].as_str().unwrap(),scope["target"].as_str().unwrap()];
        let mut copy=ToolLatestWinsKeyCopy::new(scope["instance"].as_u64().unwrap() as u32,parts).unwrap();
        for turn in 0..100_000 {if copy.advance(parts,1,TYPED_OPERATION_RESULT_PAGE_BYTES)==PluginCloseStep::Complete {break;}assert!(turn<99_999);}
        let pointer=copy.key.as_ref().unwrap().as_ptr();
        let expected=copy.key.as_ref().unwrap().clone();
        assert_eq!(serde_json::from_str::<String>(&serde_json::to_string(&expected).unwrap()).unwrap(),expected);
        let demand=SharedOwner::<String>::allocation_bytes();
        assert!(copy.take_key(0).is_none());
        assert!(copy.take_key(demand-1).is_none());
        assert_eq!(copy.key.as_ref().unwrap().as_ptr(),pointer);
        let mut key=copy.take_key(demand).unwrap();
        assert_eq!(key.as_bytes(),expected.as_bytes());
        assert_eq!(key.as_bytes().as_ptr(),pointer);
        let mut alias=key.clone();
        assert_eq!(alias.as_bytes().as_ptr(),pointer);
        for bytes in [0,demand-1] {let step=key.release_step(RetainedCloneGrant::one_release_turn(bytes,1)).unwrap();assert_eq!(step.progress.copied_items,0);assert!(step.value.is_none());assert!(!key.terminal_is_empty());}
        let step=key.release_step(RetainedCloneGrant::one_release_turn(demand,1)).unwrap();
        assert_eq!(step.progress.released_bytes,0);assert!(step.value.is_none());assert!(key.terminal_is_empty());
        let step=alias.release_step(RetainedCloneGrant::one_release_turn(demand,1)).unwrap();
        assert_eq!(step.progress.released_bytes,demand);let text=step.value.unwrap();assert_eq!(text,expected);assert_eq!(text.as_ptr(),pointer);assert!(alias.terminal_is_empty());
        eprintln!("[DEBUG] latest-wins neutral key matched serde identity and retained subexact shared headers");
    }

    pub(super) async fn retained_document_cancellation<A: ArtifactApp + Default>(factory: std::sync::Arc<dyn store::ArtifactStoreOneItemPreparationFactory<A::Snapshot, A::Mutation>>, mutation: fn() -> A::Mutation, observe: fn(&A::Snapshot) -> i32) {
        let fixture: Value = serde_json::from_str(include_str!("../../🧫️fixtures/🥇️tool-latest-wins.json")).unwrap();
        for case in fixture["publicationCases"].as_array().unwrap() {
            for delayed_ack in [false, true] {
                let boundary = case["cancelAt"].as_str().unwrap();
                let mut app = VcsArtifactApp::<A>::new(A::default(), protocol::ActorId(crate::app::LOCAL_ACTOR_ID.into())).await;
                let before = app.store.snapshot_root();
                let revision = app.store.content_revision_now();
                let generation = app.store.generation_now();
                let operation = semio_framework_job::Operation::new(semio_framework_job::allocate_operation_id(), semio_framework_job::RevisionId(u64::from_be_bytes(revision[..8].try_into().unwrap())), semio_framework_job::Generation(generation), 1);
                let lease = app
                    .tool_cancellations
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
                    raw_input: None,
                    output_chunks: None,
                    cancellation_lease: Some(lease),
                    terminal_outcome: None,
                    terminal_seen: true,
                    publication: Some(ArtifactToolCompletionValue::Emit(Ok(Emit::default()), EphemeralEmit::default())),
                    pending_artifact_publication: None,
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
                    publication_checkpoint: None,
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
                    terminal_fault: None,
                    stage: MountedTypedCommandFullOperationStage::Publishing,
                };
                if boundary != "producer" {
                    let pending =
                        app.store.begin_apply_batch(operation.operation, generation, revision, "fixture".into(), vec![mutation()], HistoryLane::Document, Some(&factory), None).unwrap_or_else(|_| panic!("exact scalar document preparation admission"));
                    mounted.pending_artifact_publication = Some(PendingArtifactStorePublication::Artifact(pending));
                    let target = match boundary {
                        "preparation" => store::ArtifactStoreOneItemPublicationPhase::Preparing,
                        "preflight" => store::ArtifactStoreOneItemPublicationPhase::PreflightingCommit,
                        "publishing" => store::ArtifactStoreOneItemPublicationPhase::Publishing,
                        "awaitingAck" => store::ArtifactStoreOneItemPublicationPhase::AwaitingAck,
                        _ => unreachable!(),
                    };
                    for _ in 0..100_000 {
                        let Some(PendingArtifactStorePublication::Artifact(pending)) = mounted.pending_artifact_publication.as_ref() else {
                            panic!("exact document pending owner");
                        };
                        if pending.phase() == target {
                            break;
                        }
                        app.publish_mounted_typed_operation_unit(&mut mounted).await.unwrap();
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
                app.publish_mounted_typed_operation_unit(&mut mounted).await.unwrap();
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
                let actual = serde_json::json!({ "count": observe(&app.store.snapshot_root()), "generation": app.store.generation_now() - generation, "sameRoot": std::sync::Arc::ptr_eq(&before, &app.store.snapshot_root()) });
                let expected = serde_json::json!({ "count": if committed { 42 } else { 0 }, "generation": u64::from(committed), "sameRoot": !committed });
                assert_eq!(actual, expected, "{boundary}, delayed ACK={delayed_ack}");
                assert!(mounted.acknowledge_result_page(final_page.token).unwrap());
                for _ in 0..100_000 {
                    if mounted.terminal_is_empty() {
                        break;
                    }
                    match mounted.retirement_step(1, TYPED_OPERATION_RESULT_PAGE_BYTES).unwrap() {
                        PluginCloseStep::Pending { released_items, released_bytes } => {
                            assert!(released_items <= 1);
                            assert!(released_bytes <= TYPED_OPERATION_RESULT_PAGE_BYTES);
                        }
                        PluginCloseStep::Complete => {}
                        PluginCloseStep::AwaitingInput { reason } => panic!("document cancellation close awaited input: {reason}"),
                        PluginCloseStep::Blocked { reason } => panic!("document cancellation close blocked: {reason}"),
                    }
                }
                assert!(mounted.terminal_is_empty());
                drop(before);
                for _ in 0..100_000 {
                    if app.close_terminal_is_empty() {
                        break;
                    }
                    match app.close_step(1, TYPED_OPERATION_RESULT_PAGE_BYTES).unwrap() {
                        PluginCloseStep::Pending { released_items, released_bytes } => {
                            assert!(released_items <= 1);
                            assert!(released_bytes <= TYPED_OPERATION_RESULT_PAGE_BYTES);
                        }
                        PluginCloseStep::Complete => {}
                        PluginCloseStep::AwaitingInput { reason } => panic!("document cancellation app close awaited input: {reason}"),
                        PluginCloseStep::Blocked { reason } => panic!("document cancellation app close blocked: {reason}"),
                    }
                }
                assert!(app.close_terminal_is_empty());
                eprintln!("real mounted Document publication {boundary}, delayed ACK={delayed_ack}: count/revision/root retained and close terminal");
            }
        }
    }

    #[test]
    fn retained_latest_wins_full_domain_exact_keys_match_serde_oracle_and_retire() {
        let fixture: Value = serde_json::from_str(include_str!("../../🧫️fixtures/🥇️tool-latest-wins.json")).unwrap();
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
            assert!(registry.begin(first_operation.0, &first_key, &first_lease, TYPED_OPERATION_RESULT_PAGE_BYTES));
            assert!(first_key.release_step(semio_framework_value::retained_clone::RetainedCloneGrant::one_release_turn(first_key.next_release_byte_demand(),1)).unwrap().value.is_none());
            for _ in 0..100_000 {
                if registry.take_outcome(first_operation.0) == Some(true) {
                    break;
                }
                let _ = registry.advance(1, TYPED_OPERATION_RESULT_PAGE_BYTES).unwrap();
            }
            assert!(!first_lease.token.is_cancelled_now());
            for _ in 0..100_000 {
                if registry.can_begin() {
                    break;
                }
                let _ = registry.advance(1, TYPED_OPERATION_RESULT_PAGE_BYTES).unwrap();
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
            assert!(registry.begin(next_operation.0, &next_key, &next_lease, TYPED_OPERATION_RESULT_PAGE_BYTES));
            assert!(next_key.release_step(semio_framework_value::retained_clone::RetainedCloneGrant::one_release_turn(next_key.next_release_byte_demand(),1)).unwrap().value.is_none());
            let mut accepted = false;
            for _ in 0..100_000 {
                if let Some(result) = registry.take_outcome(next_operation.0) {
                    accepted = result;
                    break;
                }
                match registry.advance(1, TYPED_OPERATION_RESULT_PAGE_BYTES).unwrap() {
                    PluginCloseStep::Pending { released_items, released_bytes } => {
                        assert!(released_items <= 1);
                        assert!(released_bytes <= TYPED_OPERATION_RESULT_PAGE_BYTES);
                    }
                    PluginCloseStep::Complete => {}
                    PluginCloseStep::AwaitingInput { reason } => panic!("exact key comparison awaited input: {reason}"),
                    PluginCloseStep::Blocked { reason } => panic!("exact key comparison blocked: {reason}"),
                }
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
                match registry.advance(1, TYPED_OPERATION_RESULT_PAGE_BYTES).unwrap() {
                    PluginCloseStep::Pending { released_items, released_bytes } => {
                        assert!(released_items <= 1);
                        assert!(released_bytes <= TYPED_OPERATION_RESULT_PAGE_BYTES);
                    }
                    PluginCloseStep::Complete => {}
                    PluginCloseStep::AwaitingInput { reason } => panic!("exact key close awaited input: {reason}"),
                    PluginCloseStep::Blocked { reason } => panic!("exact key close blocked: {reason}"),
                }
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
        let fixture: Value = serde_json::from_str(include_str!("../../🧫️fixtures/🔗️tool-latest-wins-integration.json")).unwrap();
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
        let fixture: Value = serde_json::from_str(include_str!("../../🧫️fixtures/🔗️tool-latest-wins-integration.json")).unwrap();
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
        let fixture: Value = serde_json::from_str(include_str!("../../🧫️fixtures/🔗️tool-latest-wins-integration.json")).unwrap();
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
                    begun = registry.begin(operation.0, &key, &lease, TYPED_OPERATION_RESULT_PAGE_BYTES);
                }
                if begun {
                    result = registry.take_outcome(operation.0);
                }
                if result.is_some() {
                    break;
                }
                match registry.advance(1, TYPED_OPERATION_RESULT_PAGE_BYTES).unwrap() {
                    PluginCloseStep::Pending { released_items, released_bytes } => {
                        assert!(released_items <= 1);
                        assert!(released_bytes <= TYPED_OPERATION_RESULT_PAGE_BYTES);
                    }
                    PluginCloseStep::Complete => {}
                    PluginCloseStep::AwaitingInput { reason } => panic!("full-map retained admission awaited input: {reason}"),
                    PluginCloseStep::Blocked { reason } => panic!("full-map retained admission blocked: {reason}"),
                }
            }
            assert_eq!(result, Some(true), "completed target {index} must not consume permanent admission capacity");
            assert!(key.release_step(semio_framework_value::retained_clone::RetainedCloneGrant::one_release_turn(key.next_release_byte_demand(),1)).unwrap().value.is_none());
            accepted += 1;
            for _ in 0..100_000 {
                if registry.can_begin() {
                    break;
                }
                registry.advance(1, TYPED_OPERATION_RESULT_PAGE_BYTES).unwrap();
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
            registry.advance(1, TYPED_OPERATION_RESULT_PAGE_BYTES).unwrap();
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
        let fixture: Value = serde_json::from_str(include_str!("../../🧫️fixtures/🔗️tool-latest-wins-integration.json")).unwrap();
        let mut app = VcsArtifactApp::<A>::new(A::default(), protocol::ActorId(crate::app::LOCAL_ACTOR_ID.into())).await;
        let first = fixture["slotReservation"]["firstOperation"].as_u64().unwrap();
        let collision = fixture["slotReservation"]["collidingOperation"].as_u64().unwrap();
        app.typed_operation_reservations[first as usize % ARTIFACT_LIVE_OUTPUT_SLOTS] = Some(first);
        assert_eq!(serde_json::json!(app.can_admit_typed_operation(collision)), fixture["slotReservation"]["collisionAdmitted"]);
        assert!(!app.can_admit_typed_operation(first));
        app.typed_operation_reservations[first as usize % ARTIFACT_LIVE_OUTPUT_SLOTS] = None;
        assert!(app.can_admit_typed_operation(collision));
        let revision = app.store.content_revision_now();
        for id in [1, 2] {
            let operation = semio_framework_job::Operation::new(semio_framework_job::OperationId(id), semio_framework_job::RevisionId(u64::from_be_bytes(revision[..8].try_into().unwrap())), semio_framework_job::Generation(0), 17);
            let lease =
                app.tool_cancellations.begin_keyed(ToolOperationKey { app_instance_id: 7, document: ArtifactDocumentAuthority(7), operation_id: operation.operation, base_revision: operation.base_revision, generation: operation.generation }).unwrap();
            let pending = if id == 2 {
                Some(PendingArtifactStorePublication::Presence(
                    app.presence_store.begin_publish_one(operation.operation, 0, ChangePublicationPresence { revision: 1 }.into(), Some(&TwoTurnPublicationPresencePreparationFactory), app.presence_local_root_retirement_factory.clone()).unwrap(),
                ))
            } else {
                None
            };
            app.tool_operations.insert_admitted(
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
                    raw_input: None,
                    output_chunks: None,
                    cancellation_lease: Some(lease),
                    terminal_outcome: None,
                    terminal_seen: true,
                    publication: Some(ArtifactToolCompletionValue::Emit(Ok(Emit::default()), EphemeralEmit::default())),
                    pending_artifact_publication: pending,
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
                    publication_checkpoint: None,
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
                    terminal_fault: None,
                    stage: if id == 1 { MountedTypedCommandFullOperationStage::Worker } else { MountedTypedCommandFullOperationStage::Publishing },
                },
            );
        }
        for _ in 0..fixture["fairness"]["secondPublishesWithinMetadataVisits"].as_u64().unwrap() {
            if app.presence_store.generation_now() == 1 {
                break;
            }
            app.advance_typed_operation_publication_one().await.unwrap();
        }
        assert_eq!(app.presence_store.generation_now(), 1);
        let stuck = app.tool_operations.get(1).expect("the structurally faulted operation stays mounted until it retires");
        assert_eq!(stuck.stage, MountedTypedCommandFullOperationStage::AwaitingAck, "a refused worker step leaves its own operation holding its own terminal fault page");
        let mut pages: Vec<(u64, TypedOperationResultLane, String)> = Vec::new();
        while let Some(page) = app.take_typed_operation_result_page(7) {
            let code = if page.lane == TypedOperationResultLane::Fault { crate::app::decode_typed_operation_fault_page(page.bytes()).code.0 } else { String::new() };
            let operation = page.token.operation;
            pages.push((operation, page.lane, code));
            if operation == 1 {
                let acknowledged = app.tool_operations.get_mut(1).expect("the terminated operation stays mounted").acknowledge_result_page(page.token).expect("the terminated operation's own fault page is acknowledgeable");
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
            app.tool_operations.insert_admitted(
                id,
                MountedTypedCommandFullOperation::<A> {
                    verb: String::new(),
                    meta: ActionMeta { actor: String::new(), instance_id: 7, view_state: None },
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
                    raw_input: None,
                    output_chunks: None,
                    cancellation_lease: None,
                    terminal_outcome: None,
                    terminal_seen: true,
                    publication: None,
                    pending_artifact_publication: None,
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
                    publication_checkpoint: None,
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
                    terminal_fault: None,
                    stage: MountedTypedCommandFullOperationStage::AwaitingAck,
                },
            );
        }
        app.tool_operations.get_mut(1).unwrap().stage = MountedTypedCommandFullOperationStage::Retiring;
        assert!(app.tool_operations.get(1).unwrap().publication.is_some());
        app.maintenance_stage = 0;
        app.maintenance_tool_cursor = 2;
        assert_eq!(app.maintenance_step(1, TYPED_OPERATION_RESULT_PAGE_BYTES).unwrap(), PluginCloseStep::Pending { released_items: 1, released_bytes: 0 });
        assert!(app.tool_operations.get(1).unwrap().publication.is_none());
        assert_eq!(app.tool_operations.get(2).unwrap().stage, MountedTypedCommandFullOperationStage::AwaitingAck);
        assert!(app.tool_operations.get(2).unwrap().result_page_presented);
        for id in 3..=ARTIFACT_LIVE_OUTPUT_SLOTS as u64 {
            drop(app.tool_operations.remove(id).expect("remove synthetic presented ACK waiter"));
        }
        let cancellation_state = app.tool_cancellations.state.clone();
        let lock = cancellation_state.lock().unwrap();
        for _ in 0..64 {
            let first = app.tool_operations.get_mut(1).unwrap();
            if first.terminal_is_empty() {
                break;
            }
            first.retirement_step(1, TYPED_OPERATION_RESULT_PAGE_BYTES).unwrap();
        }
        assert!(app.tool_operations.get(1).unwrap().terminal_is_empty());
        assert_eq!(app.tool_cancellations.active_operation_count(), 2);
        app.maintenance_stage = 18;
        app.maintenance_cancellation_cursor = TOOL_CANCELLATION_SLOTS + 1;
        let locked_step = app.maintenance_step(1, TYPED_OPERATION_RESULT_PAGE_BYTES).unwrap();
        assert!(
            matches!(locked_step, PluginCloseStep::Pending { released_items, released_bytes } if released_items <= 1 && released_bytes <= TYPED_OPERATION_RESULT_PAGE_BYTES),
            "a bounded maintenance turn never exceeds its own grant: {locked_step:?}"
        );
        assert_eq!(app.maintenance_cancellation_cursor, TOOL_CANCELLATION_SLOTS + 1);
        assert_eq!(app.tool_cancellations.active_operation_count(), 2);
        drop(lock);
        app.maintenance_stage = 18;
        app.maintenance_step(1, TYPED_OPERATION_RESULT_PAGE_BYTES).unwrap();
        assert_eq!(app.tool_cancellations.active_operation_count(), 1);
        for _ in 0..100_000 {
            if app.close_terminal_is_empty() {
                break;
            }
            match app.close_step(1, TYPED_OPERATION_RESULT_PAGE_BYTES).unwrap() {
                PluginCloseStep::Pending { released_items, released_bytes } => {
                    assert!(released_items <= 1);
                    assert!(released_bytes <= TYPED_OPERATION_RESULT_PAGE_BYTES);
                }
                PluginCloseStep::Complete => break,
                PluginCloseStep::AwaitingInput { reason } => panic!("fairness fixture close awaited input: {reason}"),
                PluginCloseStep::Blocked { reason } => panic!("fairness fixture close blocked: {reason}"),
            }
        }
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
        let grant = store::ArtifactStoreOneItemGrant { maximum_items: 1, maximum_bytes: 64 };
        assert!(matches!(presence.advance_publish_one(&mut publication, grant), Ok(store::ArtifactStoreOneItemAdvance::Progress(_))));
        assert!(std::ptr::eq(initial_root.get(), presence.local()));
        assert_eq!(presence.generation_now(), 0);

        let changed_document_revision = [4; 32];
        assert!(!typed_operation_document_is_fresh(&operation, canonical_revision, changed_document_revision, 0));
        publication.begin_close();
        for _ in 0..4 {
            if publication.close_step(grant).expect("stale pending publication closes") == store::SnapshotRetirementStep::Complete {
                break;
            }
        }
        assert!(publication.terminal_is_empty());
        assert!(std::ptr::eq(initial_root.get(), presence.local()));
        assert_eq!(presence.generation_now(), 0);
        drop(initial_root);
        let mut close = presence.begin_retirement(std::sync::Arc::new(PublicationPresence::default()), |_| true).ok().unwrap();
        for _ in 0..2048 {
            if close.close_step(1, 4096).unwrap() == store::SnapshotRetirementStep::Complete {
                break;
            }
        }
        assert!(close.terminal_is_empty());
    }

    #[test]
    fn private_child_genesis_wire_retains_exact_identity_and_indivisible_backing() {
        let fixture: Value = serde_json::from_str(include_str!("../../🧩️composition/📨️emission/🌱️genesis/🧫️fixtures/🔣️.json")).expect("neutral private genesis contract");
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
        let bytes=child.next_close_byte_demand();
        assert!(bytes>0);
        for _ in 0..4 {
            assert_eq!(child.close_one(1,bytes-1),PluginCloseStep::Pending{released_items:0,released_bytes:0});
            assert_eq!(child.genesis.as_ref().unwrap().initial_pack.as_ptr(),pointer);
            assert_eq!(child.genesis.as_ref().unwrap().reference.artifact_id,reference["artifactId"].as_str().unwrap());
        }
        assert_eq!(child.close_one(0,bytes),PluginCloseStep::Pending{released_items:0,released_bytes:0});
        assert_eq!(child.close_one(1,bytes),PluginCloseStep::Pending{released_items:1,released_bytes:bytes});
        for child in std::iter::once(&mut child).chain(decoded.iter_mut()) {
            for _ in 0..32 {
                let demand=child.next_close_byte_demand();
                match child.close_one(1,demand) {
                    PluginCloseStep::Pending{released_items,released_bytes}=>assert!(released_items<=1&&released_bytes<=demand),
                    PluginCloseStep::Complete=>break,
                    _=>panic!("exact private genesis owner release failed"),
                }
            }
            assert!(child.genesis.is_none());
            assert_eq!(child.next_close_byte_demand(),0);
        }
        println!("[DEBUG] Private genesis native exact wire local={} target={} physicalBytes={bytes}",source["childId"],reference["artifactId"]);
    }

    #[test]
    fn retained_child_wire_rejection_retires_nested_owners_under_the_production_grant() {
        let fixture: Value = serde_json::from_str(include_str!("../../../🏪️store/🧫️fixtures/📢️member-publication.json")).expect("retained child fixture");
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
        assert_eq!(child.close_one(0, TYPED_OPERATION_RESULT_PAGE_BYTES), PluginCloseStep::Pending { released_items: 0, released_bytes: 0 });
        assert_eq!(child.ops[0].len(), wire_bytes);
        let mut bytes = 0;
        let mut complete = false;
        for _ in 0..wire_bytes + 128 {
            match child.close_one(1, TYPED_OPERATION_RESULT_PAGE_BYTES) {
                PluginCloseStep::Pending { released_items, released_bytes } => {
                    assert!(released_items <= 1);
                    assert!(released_bytes <= TYPED_OPERATION_RESULT_PAGE_BYTES);
                    bytes += released_bytes;
                }
                PluginCloseStep::Complete => {
                    complete = true;
                    break;
                }
                PluginCloseStep::AwaitingInput { reason } => panic!("admitted child wire awaited input: {reason}"),
                PluginCloseStep::Blocked { .. } => panic!("admitted child wire must retire under the maximum production grant"),
            }
        }
        assert!(complete);
        assert!(bytes >= wire_bytes);
        assert!(child.ops.is_empty() && child.labels.is_empty() && child.slot.is_empty() && child.child_id.is_empty() && child.op_schema.0.is_empty());
    }

    #[test]
    fn typed_child_publication_has_one_retained_async_group_owner_and_never_clone_publishes() {
        let source = include_str!("../../🦀️.rs");
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
        let source = include_str!("../../🦀️.rs");
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
        for decoder in ["let command = Box::new(A::command_from_action(action, args).await?)", "let command = A::command_from_action(command_id, Some(&args)).await"] {
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
    /// `PendingArtifactStorePublication::retirement_turn` (ticket 26/09/09/PROCEDURAL-3D-END-TO-END),
    /// so the retained one-item retirement seam is that call, not a per-lane `close_step`.
    ///
    /// 🧵️ The one-page publisher suspends in EXACTLY one place: the task lane's spawn. Every
    /// other lane stays synchronous, so the awaits are enumerated and the one that is allowed is
    /// named by its own call — a new `.await` anywhere in the publisher fails here by name.
    #[test]
    fn fixture_contract_is_anchored_to_the_production_retained_factory_publisher_and_host_receivers() {
        let source = include_str!("../../🦀️.rs");
        let admission_marker = ["async fn admit_command_json_with_", "proof(&self"].concat();
        let admission_start = source.rfind(&admission_marker).expect("production raw JSON admission");
        let admission_end = source[admission_start..].find("pub async fn new(app: A)").map(|offset| admission_start + offset).expect("admission route end");
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
        for retained_seam in ["pending_artifact_publication", "begin_apply_batch", "advance_apply_batch", "ArtifactStoreOneItemAdvance::Published", "pending.retirement_turn(grant.maximum_items, grant.maximum_bytes)"] {
            assert!(publisher.contains(retained_seam), "production publisher lost its retained one-item seam: {retained_seam}");
        }
        for forbidden in [".apply_one(", "artifact_mutations.last().cloned()", "config_mutations.last().cloned()", "draft_mutations.last().cloned()", "presence.last().cloned()", "transient.last().cloned()"] {
            assert!(!publisher.contains(forbidden), "production publisher restored a monolithic or clone-then-pop shortcut: {forbidden}");
        }
        let freshness = publisher.find("typed-operation pending publication rejected a stale immutable document root").expect("per-turn document freshness gate");
        let pending_advance = publisher[freshness..].find("\n            if let Some(pending) = mounted.pending_artifact_publication.as_mut()").map(|offset| freshness + offset).expect("pending publication advance");
        assert!(freshness < pending_advance, "document freshness must fail closed before every retained lane advance");
        assert!(publisher[..pending_advance].contains("pending.begin_close()"));
        assert!(publisher[..pending_advance].contains("pending.close_step(1, TYPED_OPERATION_RESULT_PAGE_BYTES)"));
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

        let reactor = [include_str!("../../⚛️reactor/🦀️.rs"), include_str!("../../⚛️reactor/🔄️turn/🦀️.rs")].concat();
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

        let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/🔬️app-typed-command-full-operation/🔣️renderer-result-lanes.json")).expect("neutral result lanes");
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
        let source = include_str!("../../🦀️.rs");
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
        let command_end = source[command_start..].find("fn addressed_preview_view(").map(|offset| command_start + offset).expect("manifest command route end");
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

#[cfg(test)]
mod child_complete_group_candidate_tests{
    mod paged_encoder{include!("./🧩️child-operations/🛫️encoder/🦀️.rs");}
    mod paged_owner{include!("./🧩️child-operations/🫙️owner/🦀️.rs");}
    mod candidate{include!("./🧩️child-operations/🧩️projection/🦀️.rs");}
    mod reader{include!("./🧩️child-operations/📖️reader/🦀️.rs");}
    use candidate::{ChildGroupSources,ChildGroupSource};
    use semio_framework_os_kernel::os_spr::operation_bytes::{OwnedOperationBytes,OperationByteOutput,OperationByteCloseStep};
    use semio_framework_value::{NativeEncodeControl,list::PagedList};
    use semio_framework_ui_locale::LocalizedLabel;
    struct Group{owner:String,slot:String,child_id:String,schema:String,ops:PagedList<OwnedOperationBytes,2>,labels:PagedList<LocalizedLabel,2>}
    struct Groups(Vec<Group>);
    impl ChildGroupSources for Groups{
        fn len(&self)->usize{self.0.len()}
        fn group_at(&self,index:usize)->Option<ChildGroupSource<'_>>{self.0.get(index).map(|group|ChildGroupSource{owner:&group.owner,slot:&group.slot,child_id:&group.child_id,schema:&group.schema,operations:&group.ops,labels:&group.labels})}
    }
    fn drain(bytes:&mut OwnedOperationBytes){for _ in 0..bytes.len()+256{if bytes.close_one(1,4096).unwrap()==OperationByteCloseStep::Complete{break}}assert!(bytes.terminal_is_empty());}
    fn groups(fixture:&serde_json::Value)->Groups{
        let mut groups=Vec::new();
        for value in fixture["groups"].as_array().unwrap(){
            let mut ops=PagedList::<OwnedOperationBytes,2>::empty();let mut labels=PagedList::<LocalizedLabel,2>::empty();
            while !ops.has_reserved_slot(){assert!(ops.reserve_one(4096).unwrap().progressed)}
            while !labels.has_reserved_slot(){assert!(labels.reserve_one(4096).unwrap().progressed)}
            let mut bytes=OwnedOperationBytes::try_new(8194,65536).unwrap();let mut allow=|_|true;let mut encoding=NativeEncodeControl::new(65536,&mut allow);
            for byte in value["ops"][0].as_array().unwrap(){bytes.write_bytes(&[byte.as_u64().unwrap()as u8],&mut encoding).unwrap();}
            assert!(ops.push_reserved(bytes).is_ok());
            let label=LocalizedLabel::from_fn(|terminology,locale|value["labels"][0][terminology.as_str()][locale.as_str()].as_str().unwrap().to_owned());
            assert!(labels.push_reserved(label).is_ok());
            groups.push(Group{owner:value["owner"].as_str().unwrap().into(),slot:value["slot"].as_str().unwrap().into(),child_id:value["child_id"].as_str().unwrap().into(),schema:value["op_schema"].as_str().unwrap().into(),ops,labels});
        }
        Groups(groups)
    }
    struct Visitor<'a>{expected:&'a Groups,group:usize,operation:usize,byte:usize,label:usize,partial:Option<OwnedOperationBytes>,complete:PagedList<OwnedOperationBytes,4>}
    impl<'a> Visitor<'a>{
        fn new(expected:&'a Groups)->Self{Self{expected,group:0,operation:0,byte:0,label:0,partial:None,complete:PagedList::empty()}}
        fn close(&mut self){if let Some(source)=self.partial.as_mut(){drain(source);}self.partial.take();while let Some(source)=self.complete.get_mut(self.complete.len().saturating_sub(1)){drain(source);self.complete.pop();}while !self.complete.terminal_is_empty(){assert!(self.complete.release_empty_page(4096).unwrap().progressed)}}
    }
    impl reader::ChildGroupDecodeVisitor for Visitor<'_>{
        fn begin_groups(&mut self,count:usize,_:&mut semio_framework_value::NativeDecodeControl<'_>)->Result<(),semio_framework_os_kernel::os_pack::PackRefusal>{assert_eq!(count,self.expected.0.len());Ok(())}
        fn begin_group(&mut self,index:usize,_:&mut semio_framework_value::NativeDecodeControl<'_>)->Result<(),semio_framework_os_kernel::os_pack::PackRefusal>{self.group=index;Ok(())}
        fn text(&mut self,field:reader::ChildGroupText,source:semio_framework_os_kernel::os_pack::codec::ByteSpan<'_>,_:&mut semio_framework_value::NativeDecodeControl<'_>)->Result<(),semio_framework_os_kernel::os_pack::PackRefusal>{let group=&self.expected.0[self.group];let expected=match field{reader::ChildGroupText::Owner=>&group.owner,reader::ChildGroupText::Slot=>&group.slot,reader::ChildGroupText::ChildId=>&group.child_id,reader::ChildGroupText::Schema=>&group.schema};assert!(source.iter().eq(expected.bytes()));Ok(())}
        fn begin_operations(&mut self,count:usize,_:&mut semio_framework_value::NativeDecodeControl<'_>)->Result<(),semio_framework_os_kernel::os_pack::PackRefusal>{assert_eq!(count,self.expected.0[self.group].ops.len());Ok(())}
        fn begin_operation(&mut self,index:usize,count:usize,control:&mut semio_framework_value::NativeDecodeControl<'_>)->Result<(),semio_framework_os_kernel::os_pack::PackRefusal>{
            self.operation=index;self.byte=0;assert!(self.partial.is_none());assert_eq!(count,self.expected.0[self.group].ops[index].len());
            self.partial=Some(OwnedOperationBytes::try_new(count.max(1),control.maximum_bytes()).unwrap());Ok(())
        }
        fn operation_byte(&mut self,byte:u8,control:&mut semio_framework_value::NativeDecodeControl<'_>)->Result<(),semio_framework_os_kernel::os_pack::PackRefusal>{
            assert_eq!(self.expected.0[self.group].ops[self.operation].byte_at(self.byte),Some(byte));
            let source=self.partial.as_mut().unwrap();
            while !source.has_reserved_slot(){let required=source.next_allocation_bytes().map_err(|fault|semio_framework_value::ValueError::new(fault.kind,fault.reason))?;assert!(required<=4096);control.checkpoint()?;control.charge(required)?;assert!(source.reserve_one(4096).map_err(|fault|semio_framework_value::ValueError::new(fault.kind,fault.reason))?.progressed)}
            source.push_reserved(byte).unwrap();self.byte+=1;Ok(())
        }
        fn end_operation(&mut self,control:&mut semio_framework_value::NativeDecodeControl<'_>)->Result<(),semio_framework_os_kernel::os_pack::PackRefusal>{
            while !self.complete.has_reserved_slot(){let required=self.complete.next_allocation_bytes().map_err(semio_framework_value::ValueError::from)?;assert!(required<=4096);control.charge(required)?;assert!(self.complete.reserve_one(4096).unwrap().progressed)}
            assert!(self.complete.push_reserved(self.partial.take().unwrap()).is_ok());Ok(())
        }
        fn begin_labels(&mut self,count:usize,_:&mut semio_framework_value::NativeDecodeControl<'_>)->Result<(),semio_framework_os_kernel::os_pack::PackRefusal>{assert_eq!(count,self.expected.0[self.group].labels.len());Ok(())}
        fn begin_label(&mut self,index:usize,_:&mut semio_framework_value::NativeDecodeControl<'_>)->Result<(),semio_framework_os_kernel::os_pack::PackRefusal>{self.label=index;Ok(())}
        fn label_text(&mut self,terminology:semio_framework_ui_locale::Terminology,locale:semio_framework_ui_locale::Locale,source:semio_framework_os_kernel::os_pack::codec::ByteSpan<'_>,_:&mut semio_framework_value::NativeDecodeControl<'_>)->Result<(),semio_framework_os_kernel::os_pack::PackRefusal>{assert!(source.iter().eq(self.expected.0[self.group].labels[self.label].resolve(terminology,locale).bytes()));Ok(())}
        fn end_label(&mut self,_:&mut semio_framework_value::NativeDecodeControl<'_>)->Result<(),semio_framework_os_kernel::os_pack::PackRefusal>{Ok(())}
        fn end_group(&mut self,_:&mut semio_framework_value::NativeDecodeControl<'_>)->Result<(),semio_framework_os_kernel::os_pack::PackRefusal>{Ok(())}
        fn end_groups(&mut self,_:&mut semio_framework_value::NativeDecodeControl<'_>)->Result<(),semio_framework_os_kernel::os_pack::PackRefusal>{assert!(self.partial.is_none());Ok(())}
    }
    fn close_groups(groups:&mut Groups){for group in &mut groups.0{while let Some(source)=group.ops.get_mut(group.ops.len().saturating_sub(1)){drain(source);group.ops.pop();}while !group.ops.terminal_is_empty(){assert!(group.ops.release_empty_page(4096).unwrap().progressed)}while let Some(label)=group.labels.get_mut(group.labels.len().saturating_sub(1)){while label.close_owned_cell_one(4096).unwrap().is_some(){}group.labels.pop();}while !group.labels.terminal_is_empty(){assert!(group.labels.release_empty_page(4096).unwrap().progressed)}}}
    #[test]
    fn child_complete_group_source_candidate_reader_retains_actual_partial8194_pages(){
        let fixture:serde_json::Value=serde_json::from_str(include_str!("./🧫️fixtures/🎒️child-group-wire/🔣️.json")).unwrap();let mut groups=groups(&fixture);
        let mut output=OwnedOperationBytes::try_new(65536,65536).unwrap();let mut allow=|_|true;let mut encoding=NativeEncodeControl::new(65536,&mut allow);candidate::encode_groups_into(&groups,&semio_framework_os_kernel::os_pack::codec::PackEncodeOptions::default(),&mut output,&mut encoding).unwrap();
        let options=semio_framework_os_kernel::os_pack::codec::PackDecodeOptions::default();
        let mut visitor=Visitor::new(&groups);let mut allow=|_|true;let mut decoding=semio_framework_value::NativeDecodeControl::new(65536,&mut allow);reader::visit_groups_span(semio_framework_os_kernel::os_pack::codec::ByteSpan::from_source(&output),&options,&mut decoding,&mut visitor).unwrap();assert_eq!(visitor.complete.len(),2);assert!(visitor.complete[0].iter().eq(groups.0[0].ops[0].iter()));assert_eq!(visitor.complete[0].len(),8194);assert!(visitor.complete[1].iter().eq(groups.0[1].ops[0].iter()));let decoded_owned=decoding.owned_bytes();assert_eq!(decoded_owned,visitor.complete.allocated_bytes()+visitor.complete.iter().map(OwnedOperationBytes::allocated_bytes).sum::<usize>());visitor.close();
        let mut visitor=Visitor::new(&groups);let mut cancel=|progress:semio_framework_value::native_decoding::NativeDecodeProgress|progress.owned_bytes<8192;let mut decoding=semio_framework_value::NativeDecodeControl::new(65536,&mut cancel);assert!(reader::visit_groups_span(semio_framework_os_kernel::os_pack::codec::ByteSpan::from_source(&output),&options,&mut decoding,&mut visitor).is_err());let partial=visitor.partial.as_ref().unwrap();assert!(partial.len()>0&&partial.len()<8194);assert!(partial.allocated_bytes()>=8192);assert_eq!(partial.allocated_bytes(),decoding.owned_bytes());let retained=partial.len();visitor.close();
        let mut visitor=Visitor::new(&groups);let mut allow=|_|true;let mut decoding=semio_framework_value::NativeDecodeControl::new(4096,&mut allow);assert!(reader::visit_groups_span(semio_framework_os_kernel::os_pack::codec::ByteSpan::from_source(&output),&options,&mut decoding,&mut visitor).is_err());assert!(visitor.partial.as_ref().unwrap().is_empty());assert!(visitor.partial.as_ref().unwrap().allocated_bytes()>0);assert!(decoding.owned_bytes()<=4096);visitor.close();
        let mut visitor=Visitor::new(&groups);let mut allow=|_|true;let mut decoding=semio_framework_value::NativeDecodeControl::new(65536,&mut allow);let short=semio_framework_os_kernel::os_pack::codec::ByteSpan::from_source(&output).slice(0,output.len()-1).unwrap();assert!(reader::visit_groups_span(short,&options,&mut decoding,&mut visitor).is_err());assert_eq!(visitor.complete.len(),2);visitor.close();
        drain(&mut output);close_groups(&mut groups);
        eprintln!("[DEBUG] held group span reader reconstructed all8194 exact octets into caller-retained real pages; cancellation retains{retained} partial octets; initial4096 rejects real next page; truncation retains both completed sources; all owners drain1/4096");
    }
    #[test]
    fn child_complete_group_source_candidate_matches_independent_wire(){
        let fixture:serde_json::Value=serde_json::from_str(include_str!("./🧫️fixtures/🎒️child-group-wire/🔣️.json")).unwrap();
        let mut groups=groups(&fixture);assert_eq!(groups.0[0].ops[0].len(),8194);
        let options=semio_framework_os_kernel::os_pack::codec::PackEncodeOptions::default();
        let mut output=OwnedOperationBytes::try_new(65536,65536).unwrap();let mut allow=|_|true;let mut control=NativeEncodeControl::new(65536,&mut allow);
        let count=candidate::encode_groups_into(&groups,&options,&mut output,&mut control).unwrap();assert_eq!(count,output.len());
        let expected=fixture["wire"].as_array().unwrap();assert_eq!(output.len(),expected.len());for(index,byte)in expected.iter().enumerate(){assert_eq!(output.byte_at(index),Some(byte.as_u64().unwrap()as u8),"exact original group octet {index}")}
        let mut comparison=semio_framework_os_kernel::os_spr::operation_bytes::OperationByteComparison::new(semio_framework_os_kernel::os_pack::codec::ByteSpan::from_source(&output));let mut allow=|_|true;let mut control=NativeEncodeControl::new(65536,&mut allow);candidate::encode_groups_into(&groups,&options,&mut comparison,&mut control).unwrap();comparison.finish().unwrap();
        let mut empty=OwnedOperationBytes::try_new(65536,65536).unwrap();let mut allow=|_|true;let mut control=NativeEncodeControl::new(65536,&mut allow);assert_eq!(candidate::encode_groups_into(&Groups(Vec::new()),&options,&mut empty,&mut control).unwrap(),0);assert_eq!(empty.len(),0);assert_eq!(empty.allocated_bytes(),0);drain(&mut empty);
        let mut partial=OwnedOperationBytes::try_new(65536,65536).unwrap();let mut allow=|progress:semio_framework_value::native_encoding::NativeEncodeProgress|progress.owned_bytes<8192;let mut control=NativeEncodeControl::new(65536,&mut allow);assert!(candidate::encode_groups_into(&groups,&options,&mut partial,&mut control).is_err());assert!(partial.allocated_bytes()>0);let retained=partial.allocated_bytes();drain(&mut partial);
        let mut refused=OwnedOperationBytes::try_new(65536,65536).unwrap();let mut small=options.clone();small.limits.max_total_alloc=128;let mut allow=|_|true;let mut control=NativeEncodeControl::new(65536,&mut allow);assert!(candidate::encode_groups_into(&groups,&small,&mut refused,&mut control).is_err());assert!(control.owned_bytes()<=128);drain(&mut refused);
        drain(&mut output);
        close_groups(&mut groups);
        eprintln!("[DEBUG] held complete child group candidate equals independent JSON/TextEncoder/Ajv wire; actual paged input8194 and paged output drain1/4096; cancellation retains{retained} physical bytes; production ChildEmit remains held");
    }
    #[test]
    fn child_complete_original_semantic_metadata8194_grant4096(){
        let corpus:serde_json::Value=serde_json::from_str(include_str!("./🧫️fixtures/🔤️semantic-text-wire/🔣️.json")).unwrap();
        for case in corpus["cases"].as_array().unwrap(){
            let field=case["field"].as_str().unwrap();let large="m".repeat(8194);assert_eq!(large.len(),8194);let mut child=super::ChildEmit::open("","",0);
            match field{"owner"=>child.owner=large,"slot"=>child.slot=large,"child_id"=>child.child_id=large,"op_schema"=>child.op_schema.0=large,_=>{let keys:Vec<&str>=field.split('.').collect();assert_eq!(keys.len(),3);let label=semio_framework_ui_locale::LocalizedLabel::from_fn(|terminology,locale|if terminology.as_str()==keys[1]&&locale.as_str()==keys[2]{large.clone()}else{String::new()});child.labels.push(label);}}
            assert_eq!(child.close_one(0,4096),super::PluginCloseStep::Pending{released_items:0,released_bytes:0});let mut complete=false;let mut physical=0;
            for _ in 0..8194+128{match child.close_one(1,4096){super::PluginCloseStep::Pending{released_items,released_bytes}=>{assert!(released_items<=1);assert!(released_bytes<=4096);physical+=released_bytes;},super::PluginCloseStep::Complete=>{complete=true;break},step=>panic!("natural child semantic owner refused original grant: {step:?}")}}
            assert!(complete,"actual ChildEmit full8194 semantic owner remained held at original1/4096: {field}");assert!(physical>=8194);assert!(child.ops.is_empty());assert!(child.labels.is_empty());assert!(child.owner.is_empty());assert!(child.slot.is_empty());assert!(child.child_id.is_empty());assert!(child.op_schema.0.is_empty());
        }
    }
    fn drain_retained(owner:&mut dyn semio_framework_value::ErasedSnapshotRetirement)->usize{
        use semio_framework_value::SnapshotRetirementStep;
        let mut released=0;for _ in 0..100000{match owner.close_step(1,4096).unwrap(){SnapshotRetirementStep::Complete=>break,SnapshotRetirementStep::Pending{released_items,released_bytes}=>{assert!(released_items<=1);assert!(released_bytes<=4096);released+=released_bytes;},step=>panic!("retained child owner refused its genuine physical page grant: {step:?}")}}assert!(owner.terminal_is_empty());released
    }
    fn drain_typed_groups(groups:&mut semio_framework_value::list::PagedList<paged_owner::PagedChildOwner,{isize::MAX as usize}>)->usize{
        let mut released=0;while let Some(child)=groups.get_mut(groups.len().saturating_sub(1)){released+=drain_retained(child);groups.pop();}while !groups.terminal_is_empty(){let step=groups.release_empty_page(4096).unwrap();assert!(step.progressed);assert!(step.released_allocation_bytes<=4096);released+=step.released_allocation_bytes;}released
    }
    #[test]
    fn child_complete_group_source_candidate_paged_semantic8194_metadata_and_labels(){
        use semio_framework_value::{NativeDecodeControl,NativeEncodeControl,native_decoding::NativeDecodeRetirementRecipient,ErasedSnapshotRetirement};
        use semio_framework_os_kernel::{os_pack::codec::{ByteSpan,PackEncodeOptions,PackDecodeOptions},os_spr::operation_bytes::{OperationByteOutput,OperationByteComparison}};
        let corpus:serde_json::Value=serde_json::from_str(include_str!("./🧫️fixtures/🔤️semantic-text-wire/🔣️.json")).unwrap();assert_eq!(corpus["maximumItems"],1);assert_eq!(corpus["maximumBytes"],4096);assert_eq!(corpus["maximumAllocationBytes"],65536);
        let mut cases=0;for case in corpus["cases"].as_array().unwrap(){
            let wire:Vec<u8>=case["wire"].as_array().unwrap().iter().map(|byte|byte.as_u64().unwrap()as u8).collect();let mut source=OwnedOperationBytes::try_new(65536,65536).unwrap();let mut allow=|_|true;let mut encoding=NativeEncodeControl::new(65536,&mut allow);source.write_bytes(&wire,&mut encoding).unwrap();let source_bytes=source.allocated_bytes();
            let mut recipient=NativeDecodeRetirementRecipient::new();let mut allow=|_|true;let mut decoding=NativeDecodeControl::new(65536,&mut allow);decoding.install_retirement_recipient(&mut recipient).unwrap();let mut allow=|_|true;let mut encoding=NativeEncodeControl::new(65536,&mut allow);let mut groups=paged_owner::decode_paged_groups_span(ByteSpan::from_source(&source),&PackDecodeOptions::default(),&PackEncodeOptions::default(),&mut decoding,&mut encoding).unwrap();let admitted=decoding.owned_bytes();drop(decoding);assert!(recipient.has_owner());assert_eq!(groups.len(),1);assert_eq!(groups[0].operations[0].len(),8194);assert!(groups[0].operations[0].iter().eq("17".bytes().chain(std::iter::repeat_n(b' ',8192))));
            let typed_bytes=groups.allocated_bytes()+groups.iter().map(paged_owner::PagedChildOwner::allocated_bytes).sum::<usize>();assert_eq!(admitted,std::mem::size_of::<paged_owner::PagedChildDecodeOwner>()+typed_bytes);let source_group=paged_encoder::PagedChildGroups::group(&groups,0).unwrap();let field=case["field"].as_str().unwrap();let text=match field{"owner"=>source_group.owner,"slot"=>source_group.slot,"child_id"=>source_group.child_id,"op_schema"=>source_group.schema,"label.native.en"=>source_group.labels.text(0,semio_framework_ui_locale::Terminology::ALL[0],semio_framework_ui_locale::Locale::ALL[0]).unwrap(),"label.native.de"=>source_group.labels.text(0,semio_framework_ui_locale::Terminology::ALL[0],semio_framework_ui_locale::Locale::ALL[1]).unwrap(),"label.reuse.en"=>source_group.labels.text(0,semio_framework_ui_locale::Terminology::ALL[1],semio_framework_ui_locale::Locale::ALL[0]).unwrap(),"label.reuse.de"=>source_group.labels.text(0,semio_framework_ui_locale::Terminology::ALL[1],semio_framework_ui_locale::Locale::ALL[1]).unwrap(),_=>panic!("closed semantic field changed")};assert_eq!(text.len(),8194);for index in 0..8194{assert_eq!(text.byte(index).unwrap(),b'm')}
            let scaffold_released=drain_retained(&mut recipient);assert!(scaffold_released>=std::mem::size_of::<paged_owner::PagedChildDecodeOwner>());assert_eq!(source.allocated_bytes(),source_bytes);assert_eq!(groups.allocated_bytes()+groups.iter().map(paged_owner::PagedChildOwner::allocated_bytes).sum::<usize>(),typed_bytes);
            let mut symbols=paged_encoder::ChildSymbolOwner::empty();let mut comparison=OperationByteComparison::new(ByteSpan::from_source(&source));let mut allow=|_|true;let mut encoding=NativeEncodeControl::new(65536,&mut allow);assert_eq!(paged_encoder::encode_paged_groups_into(&groups,&PackEncodeOptions::default(),&mut comparison,&mut symbols,&mut encoding).unwrap(),wire.len());comparison.finish().unwrap();drain_retained(&mut symbols);assert_eq!(drain_typed_groups(&mut groups),typed_bytes);assert_eq!(source.allocated_bytes(),source_bytes);
            let mut recipient=NativeDecodeRetirementRecipient::new();let mut stages=0;let mut cancel=|progress:semio_framework_value::native_decoding::NativeDecodeProgress|{if progress.total==8194&&progress.completed==0{stages+=1;}!(progress.total==8194&&stages>=2&&progress.completed>=256)};let mut decoding=NativeDecodeControl::new(65536,&mut cancel);decoding.install_retirement_recipient(&mut recipient).unwrap();let mut allow=|_|true;let mut encoding=NativeEncodeControl::new(65536,&mut allow);assert!(paged_owner::decode_paged_groups_span(ByteSpan::from_source(&source),&PackDecodeOptions::default(),&PackEncodeOptions::default(),&mut decoding,&mut encoding).is_err());let partial_admitted=decoding.owned_bytes();drop(decoding);assert!(recipient.has_owner());assert!(partial_admitted>=std::mem::size_of::<paged_owner::PagedChildDecodeOwner>()+4096);assert_eq!(drain_retained(&mut recipient),partial_admitted);
            let mut recipient=NativeDecodeRetirementRecipient::new();let mut allow=|_|true;let mut decoding=NativeDecodeControl::new(4096,&mut allow);decoding.install_retirement_recipient(&mut recipient).unwrap();let mut allow=|_|true;let mut encoding=NativeEncodeControl::new(65536,&mut allow);assert!(paged_owner::decode_paged_groups_span(ByteSpan::from_source(&source),&PackDecodeOptions::default(),&PackEncodeOptions::default(),&mut decoding,&mut encoding).is_err());let small_admitted=decoding.owned_bytes();drop(decoding);assert!(small_admitted<=4096);assert_eq!(drain_retained(&mut recipient),small_admitted);
            let mut allow=|_|true;let mut decoding=NativeDecodeControl::new(65536,&mut allow);let mut allow=|_|true;let mut encoding=NativeEncodeControl::new(65536,&mut allow);assert!(paged_owner::decode_paged_groups_span(ByteSpan::from_source(&source),&PackDecodeOptions::default(),&PackEncodeOptions::default(),&mut decoding,&mut encoding).is_err());assert_eq!(decoding.owned_bytes(),0);drain(&mut source);cases+=1;
        }assert_eq!(cases,8);eprintln!("[DEBUG] held paged semantic child metadata/locale family preserves all8 full8194 positive fields, actual partial cancellation and initial4096 recipient refusal; source, returned typed text and canonical scaffolds drain independently1/4096");
    }
    #[test]
    fn child_complete_group_source_candidate_direct_producer_and_parent(){
        use semio_framework_value::{NativeEncodeControl,ErasedSnapshotRetirement,retirement::allocation_return::{ParentAllocationReturn,AllocationReturnStep}};
        use semio_framework_os_kernel::{os_pack::codec::PackEncodeOptions,os_spr::operation_bytes::OperationByteOutput};
        use reader::ChildGroupText;
        let fixture:serde_json::Value=serde_json::from_str(include_str!("./🧫️fixtures/📤️child-group-producer/🔣️.json")).unwrap();assert_eq!(fixture["sourceBytes"],8194);assert_eq!(fixture["textBytes"],8194);assert_eq!(fixture["maximumAllocationBytes"],65536);assert_eq!(fixture["maximumItems"],1);assert_eq!(fixture["maximumBytes"],4096);let large=fixture["text"].as_str().unwrap().repeat(8194);
        fn encode(output:&mut dyn OperationByteOutput,control:&mut NativeEncodeControl<'_>)->Result<(),semio_framework_os_kernel::os_spr::ProtocolError>{output.write_bytes(b"17",control)?;let padding=[b' ';256];for _ in 0..32{output.write_bytes(&padding,control)?;}Ok(())}
        fn initialize(owner:&mut paged_owner::PagedChildOwner,text:&str,options:&PackEncodeOptions,control:&mut NativeEncodeControl<'_>){for(field,value)in[(ChildGroupText::Owner,text),(ChildGroupText::Slot,"slot"),(ChildGroupText::ChildId,"child"),(ChildGroupText::Schema,"child.empty")]{owner.read_metadata_from_encoding_source(field,&value,options,control).unwrap();}}
        let options=PackEncodeOptions::default();let mut owner=paged_owner::PagedChildOwner::empty();let mut allow=|_|true;let mut control=NativeEncodeControl::new(65536,&mut allow);initialize(&mut owner,&large,&options,&mut control);let mut receipt=owner.produce_owned_operation(8194,&options,&mut control,|_,output,control|encode(output,control)).unwrap();receipt.read_first_schema_from_source(&"schema.actual",&mut control).unwrap();for terminology in semio_framework_ui_locale::Terminology::ALL{for locale in semio_framework_ui_locale::Locale::ALL{let text=if terminology==semio_framework_ui_locale::Terminology::ALL[0]&&locale==semio_framework_ui_locale::Locale::ALL[0]{large.as_str()}else{"small"};receipt.read_label_from_source(terminology,locale,&text,&mut control).unwrap();}}receipt.commit(&mut control).unwrap();assert_eq!(owner.operations.len(),1);assert_eq!(owner.operations[0].len(),8194);assert!(owner.operations[0].iter().eq(b"17".iter().copied().chain(std::iter::repeat_n(b' ',8192))));assert_eq!(owner.allocated_bytes(),control.owned_bytes());let allocated=owner.allocated_bytes();let mut parent=ParentAllocationReturn::<128>::try_new(4096,allocated).unwrap();assert!(!owner.return_one(&mut parent,0,4096).unwrap().progressed);let mut returned=0;
        for _ in 0..8194*3+256{if owner.terminal_is_empty(){break;}let before=owner.allocated_bytes();let step=owner.return_one(&mut parent,1,4096).unwrap();assert!(step.progressed);assert_eq!(before-owner.allocated_bytes(),step.returned_allocation_bytes);returned+=step.returned_allocation_bytes;}assert!(owner.terminal_is_empty());assert_eq!(returned,allocated);assert_eq!(parent.retained_bytes(),allocated);assert!(!parent.terminal_is_empty());let mut released=0;for _ in 0..256{match parent.close_step(1,4096){AllocationReturnStep::Complete=>break,AllocationReturnStep::Pending{released_items,released_bytes}=>{assert!(released_items<=1&&released_bytes<=4096);released+=released_bytes;}}}assert!(parent.terminal_is_empty());assert_eq!(released,allocated);
        for mode in["encoder","label","file","segment"]{let mut owner=paged_owner::PagedChildOwner::empty();let mut allow=|_|true;let mut control=NativeEncodeControl::new(65536,&mut allow);initialize(&mut owner,"root",&options,&mut control);let mut limits=options.clone();if mode=="file"{limits.limits.max_file_len=1;}if mode=="segment"{limits.limits.max_segment_len=128;}
            let result=owner.produce_owned_operation(8194,&limits,&mut control,|_,output,control|{encode(output,control)?;if mode=="encoder"{return Err(semio_framework_os_kernel::os_pack::PackRefusal::from(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::Canceled,"neutral direct encoder refused after source")).into())}Ok(())});
            match result{Err(_)=>{assert!(mode=="encoder"||mode=="file")},Ok(mut receipt)=>{receipt.read_first_schema_from_source(&"schema.actual",&mut control).unwrap();assert!(mode=="label"||mode=="segment");if mode=="segment"{assert!(receipt.read_label_from_source(semio_framework_ui_locale::Terminology::ALL[0],semio_framework_ui_locale::Locale::ALL[0],&large.as_str(),&mut control).is_err());}else{receipt.read_label_from_source(semio_framework_ui_locale::Terminology::ALL[0],semio_framework_ui_locale::Locale::ALL[0],&large.as_str(),&mut control).unwrap();}assert!(receipt.commit(&mut control).is_err());}}
            assert_eq!(owner.operations.len(),0);let view=owner.view().unwrap();assert_eq!(view.labels.len(),0);assert_eq!(view.schema.len(),11);for(index,byte)in b"child.empty".iter().enumerate(){assert_eq!(view.schema.byte(index).unwrap(),*byte)}let allocated=owner.allocated_bytes();assert_eq!(allocated,control.owned_bytes());assert_eq!(drain_retained(&mut owner),allocated);
        }
        eprintln!("[DEBUG] held direct child source receipt preserves exact8194 octets and independent original8194 owner/locale text under64k, admits both prefix pages before transfer, retains encoder/label/file/segment refusal and pending schema, real parent alone physically disposes1/4096");
    }
    #[test]
    fn child_complete_group_source_candidate_repeated_prefix_preserves_accepted_owners(){
        use semio_framework_value::{NativeEncodeControl,ErasedSnapshotRetirement,retirement::allocation_return::ParentAllocationReturn};
        use semio_framework_os_kernel::{os_pack::codec::PackEncodeOptions,os_spr::operation_bytes::OperationByteOutput};
        let fixture:serde_json::Value=serde_json::from_str(include_str!("./🧫️fixtures/🔁️repeated-prefix/🔣️.json")).unwrap();assert_eq!(fixture["sourceBytes"],8194);assert_eq!(fixture["maximumAllocationBytes"],65536);assert_eq!(fixture["maximumItems"],1);assert_eq!(fixture["maximumBytes"],4096);assert_eq!(fixture["acceptedOperations"],2);
        fn encode(output:&mut dyn OperationByteOutput,control:&mut NativeEncodeControl<'_>)->Result<(),semio_framework_os_kernel::os_spr::ProtocolError>{output.write_bytes(b"17",control)?;for _ in 0..32{output.write_bytes(&[b' ';256],control)?;}Ok(())}
        let options=PackEncodeOptions::default();let original_large_schema=std::iter::repeat_n('s',8194).collect::<String>();for original_schema in["child.empty",original_large_schema.as_str()]{for refused in[false,true]{let mut owner=paged_owner::PagedChildOwner::empty();let mut allow=|_|true;let mut control=NativeEncodeControl::new(65536,&mut allow);for(field,text)in[(reader::ChildGroupText::Owner,"root"),(reader::ChildGroupText::Slot,"slot"),(reader::ChildGroupText::ChildId,"child"),(reader::ChildGroupText::Schema,original_schema)]{owner.read_metadata_from_encoding_source(field,&text,&options,&mut control).unwrap();}
            let mut receipt=owner.produce_owned_operation(8194,&options,&mut control,|_,output,control|encode(output,control)).unwrap();receipt.read_first_schema_from_source(&"schema.actual",&mut control).unwrap();for terminology in semio_framework_ui_locale::Terminology::ALL{for locale in semio_framework_ui_locale::Locale::ALL{receipt.read_label_from_source(terminology,locale,&"first",&mut control).unwrap();}}receipt.commit(&mut control).unwrap();let source_pointer=owner.operations[0].byte_ref(0).map(|byte|byte as *const u8).unwrap();let allocated=owner.allocated_bytes();let mut parent=ParentAllocationReturn::<16>::try_new(4096,65536).unwrap();assert!(!owner.return_committed_scaffold_one(&mut parent,0).unwrap().progressed);assert_eq!(owner.allocated_bytes(),allocated);
            let mut returned=0;for _ in 0..8194+128{let before=owner.allocated_bytes();let step=owner.return_committed_scaffold_one(&mut parent,1).unwrap();if !step.progressed{break;}assert_eq!(before-owner.allocated_bytes(),step.returned_allocation_bytes);returned+=step.returned_allocation_bytes;assert_eq!(owner.operations.len(),1);assert_eq!(owner.operations[0].byte_ref(0).map(|byte|byte as *const u8),Some(source_pointer));assert_eq!(owner.view().unwrap().labels.len(),1);}assert_eq!(parent.retained_bytes(),returned);if original_schema.len()==8194{assert!(returned>8194);assert!(!parent.terminal_is_empty());}else{assert_eq!(returned,0);assert!(parent.terminal_is_empty());}let mut schema_released=0;for _ in 0..128{match parent.close_step(1,4096){semio_framework_value::retirement::allocation_return::AllocationReturnStep::Complete=>break,semio_framework_value::retirement::allocation_return::AllocationReturnStep::Pending{released_items,released_bytes}=>{assert!(released_items<=1&&released_bytes<=4096);schema_released+=released_bytes;}}}assert!(parent.terminal_is_empty());assert_eq!(schema_released,returned);
            let result=owner.produce_owned_operation(8194,&options,&mut control,|_,output,control|{encode(output,control)?;if refused{return Err(semio_framework_os_kernel::os_pack::PackRefusal::from(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::Canceled,"second direct encoder refused")).into())}Ok(())});match result{Err(_)=>assert!(refused),Ok(mut receipt)=>{assert!(!refused);for terminology in semio_framework_ui_locale::Terminology::ALL{for locale in semio_framework_ui_locale::Locale::ALL{receipt.read_label_from_source(terminology,locale,&"second",&mut control).unwrap();}}receipt.commit(&mut control).unwrap();}}
            assert_eq!(owner.operations.len(),if refused{1}else{2});assert_eq!(owner.operations[0].byte_ref(0).map(|byte|byte as *const u8),Some(source_pointer));assert!(owner.operations.iter().all(|source|source.len()==8194&&source.iter().eq(b"17".iter().copied().chain(std::iter::repeat_n(b' ',8192)))));let view=owner.view().unwrap();assert_eq!(view.labels.len(),if refused{1}else{2});for(index,byte)in b"schema.actual".iter().enumerate(){assert_eq!(view.schema.byte(index).unwrap(),*byte)}assert_eq!(owner.allocated_bytes()+schema_released,control.owned_bytes());let allocated=owner.allocated_bytes();assert_eq!(drain_retained(&mut owner),allocated);
        }}
        eprintln!("[DEBUG] repeated child source8194 accepted prefix preserves pointer, schema, both locale matrices and first source on second refusal; only committed scaffold returns before next producer; every actual allocation retires1/4096");
    }
    #[test]
    fn child_complete_group_source_candidate_refuses_partial_last_locale_before_commit(){
        use semio_framework_value::{NativeEncodeControl,native_encoding::NativeEncodeProgress};
        use semio_framework_os_kernel::os_pack::codec::PackEncodeOptions;
        let fixture:serde_json::Value=serde_json::from_str(include_str!("./🧫️fixtures/🌓️partial-label/🔣️.json")).unwrap();
        assert_eq!(fixture["sourceBytes"],8194);assert_eq!(fixture["textBytes"],8194);assert_eq!(fixture["partialBytes"],256);assert_eq!(fixture["completeCells"],3);assert_eq!(fixture["maximumAllocationBytes"],65536);assert_eq!(fixture["maximumItems"],1);assert_eq!(fixture["maximumBytes"],4096);
        let options=PackEncodeOptions::default();let mut owner=paged_owner::PagedChildOwner::empty();let mut allow=|_|true;let mut control=NativeEncodeControl::new(65536,&mut allow);
        for(field,text)in[(reader::ChildGroupText::Owner,"root"),(reader::ChildGroupText::Slot,"slot"),(reader::ChildGroupText::ChildId,"child"),(reader::ChildGroupText::Schema,"child.empty")]{owner.read_metadata_from_encoding_source(field,&text,&options,&mut control).unwrap();}
        let mut receipt=owner.produce_owned_operation(8194,&options,&mut control,|_,output,control|{output.write_bytes(b"17",control)?;for _ in 0..32{output.write_bytes(&[b' ';256],control)?;}Ok(())}).unwrap();receipt.read_first_schema_from_source(&"schema.actual",&mut control).unwrap();
        let mut axes=Vec::new();for terminology in semio_framework_ui_locale::Terminology::ALL{for locale in semio_framework_ui_locale::Locale::ALL{axes.push((terminology,locale));}}
        for(terminology,locale)in &axes[..3]{receipt.read_label_from_source(*terminology,*locale,&"first",&mut control).unwrap();}
        let baseline=control.owned_bytes();let continuation=control.pause().unwrap();let mut cancel=|progress:NativeEncodeProgress|progress.owned_bytes<=baseline||progress.completed<256;let mut control=NativeEncodeControl::resume(continuation,&mut cancel).unwrap();let large="x".repeat(8194);assert!(receipt.read_label_from_source(axes[3].0,axes[3].1,&large.as_str(),&mut control).is_err());assert!(control.owned_bytes()>baseline);
        let continuation=control.pause().unwrap();let mut allow=|_|true;let mut control=NativeEncodeControl::resume(continuation,&mut allow).unwrap();assert!(receipt.commit(&mut control).is_err(),"partial last locale was published despite lacking complete text authority");
        assert_eq!(owner.operations.len(),0);let view=owner.view().unwrap();assert_eq!(view.labels.len(),0);assert_eq!(view.schema.len(),11);for(index,byte)in b"child.empty".iter().enumerate(){assert_eq!(view.schema.byte(index).unwrap(),*byte)}let allocated=owner.allocated_bytes();assert_eq!(allocated,control.owned_bytes());assert_eq!(drain_retained(&mut owner),allocated);
        eprintln!("[DEBUG] three complete locale cells and fourth partial256/8194 retain their real owners; commit refuses and original schema/prefix remain unchanged; all physical owners drain1/4096");
    }
    #[test]
    fn child_complete_actual_authored_label_producer_preserves_all_axes_and_retained_source(){
        use semio_framework_value::{paged_text::{PagedText,InlineTextBuffer},NativeEncodeControl,ErasedSnapshotRetirement,SnapshotRetirementStep};
        let fixture:serde_json::Value=serde_json::from_str(include_str!("./🧫️fixtures/✍️authored-label/🔣️.json")).unwrap();assert_eq!(fixture["englishBytes"],8194);assert_eq!(fixture["germanBytes"],8205);assert_eq!(fixture["maximumAllocationBytes"],65536);assert_eq!(fixture["initialAllocationBytes"],4096);assert_eq!(fixture["maximumItems"],1);assert_eq!(fixture["maximumBytes"],4096);
        fn drain<const N:usize>(paged:&mut PagedText<N>,inline:&mut InlineTextBuffer)->usize{if !inline.terminal_is_empty(){assert!(inline.close_one(1));}let mut released=0;for _ in 0..8205+128{match paged.close_step(1,4096).unwrap(){SnapshotRetirementStep::Complete=>break,SnapshotRetirementStep::Pending{released_items,released_bytes}=>{assert!(released_items<=1&&released_bytes<=4096);released+=released_bytes;},SnapshotRetirementStep::Blocked=>panic!("authored label owner blocked")}}assert!(paged.terminal_is_empty()&&inline.terminal_is_empty());released}
        for large in[false,true]{let operation=crate::test_app_mutation_fixture::SetLabel{value:if large{std::iter::repeat_n('x',8181).collect()}else{fixture["shortValue"].as_str().unwrap().to_owned()}};for terminology in semio_framework_ui_locale::Terminology::ALL{for locale in semio_framework_ui_locale::Locale::ALL{let(prefix,suffix)=match locale{semio_framework_ui_locale::Locale::En=>(fixture["englishPrefix"].as_str().unwrap(),fixture["englishSuffix"].as_str().unwrap()),semio_framework_ui_locale::Locale::De=>(fixture["germanPrefix"].as_str().unwrap(),fixture["germanSuffix"].as_str().unwrap())};let mut paged=PagedText::<{isize::MAX as usize}>::empty();let mut inline=InlineTextBuffer::empty();let mut active=false;let mut allow=|_|true;let mut control=NativeEncodeControl::new(65536,&mut allow);paged.encode_with_inline(&mut inline,&mut active,65536,&mut control,|output|operation.label_into(terminology,locale,output)).unwrap();let expected=prefix.as_bytes().iter().copied().chain(operation.value.as_bytes().iter().copied()).chain(suffix.as_bytes().iter().copied());if large{assert!(active&&inline.borrow().is_err());assert!(paged.borrow().unwrap().bytes().eq(expected));}else{assert!(!active&&paged.borrow().is_err());assert!(inline.borrow().unwrap().bytes().eq(expected));assert_eq!(control.owned_bytes(),0);}let allocated=paged.allocated_bytes();assert_eq!(allocated,control.owned_bytes());assert_eq!(drain(&mut paged,&mut inline),allocated);
            if large{let mut paged=PagedText::<{isize::MAX as usize}>::empty();let mut inline=InlineTextBuffer::empty();let mut active=false;let mut allow=|_|true;let mut control=NativeEncodeControl::new(4096,&mut allow);assert!(paged.encode_with_inline(&mut inline,&mut active,65536,&mut control,|output|operation.label_into(terminology,locale,output)).is_err());assert!(paged.borrow().is_err()&&inline.borrow().is_err());assert!(active&&paged.allocated_bytes()>0&&paged.allocated_bytes()<=4096);assert_eq!(paged.allocated_bytes(),control.owned_bytes());let allocated=paged.allocated_bytes();assert_eq!(drain(&mut paged,&mut inline),allocated);}
        }}}
        eprintln!("[DEBUG] actual authored SetLabel directly streams every locale/terminology at original64k with full8194 English/original8205 German; short native cells own zero heap; initial4096 preserves actual prefix+metadata; no label String producer or flattening adapter; typed input remains a separate lifetime");
    }
    #[test]
    fn child_complete_actual_authored_label_receipt_joins_original8194_source_and_prefix(){
        use semio_framework_value::NativeEncodeControl;
        use semio_framework_os_kernel::{os_pack::codec::PackEncodeOptions,os_spr::operation_bytes::OperationByteOutput};
        let fixture:serde_json::Value=serde_json::from_str(include_str!("./🧫️fixtures/🧾️label-receipt/🔣️.json")).unwrap();assert_eq!(fixture["sourceBytes"],8194);assert_eq!(fixture["maximumAllocationBytes"],65536);assert_eq!(fixture["maximumItems"],1);assert_eq!(fixture["maximumBytes"],4096);
        fn encode(output:&mut dyn OperationByteOutput,control:&mut NativeEncodeControl<'_>)->Result<(),semio_framework_os_kernel::os_spr::ProtocolError>{output.write_bytes(b"17",control)?;for _ in 0..32{output.write_bytes(&[b' ';256],control)?;}Ok(())}
        let operation=crate::test_app_mutation_fixture::SetLabel{value:fixture["typedValue"].as_str().unwrap().to_owned()};for refused in[false,true]{let mut options=PackEncodeOptions::default();if refused{options.limits.max_segment_len=11;}let mut owner=paged_owner::PagedChildOwner::empty();let mut allow=|_|true;let mut control=NativeEncodeControl::new(65536,&mut allow);for(field,text)in[(reader::ChildGroupText::Owner,"root"),(reader::ChildGroupText::Slot,"slot"),(reader::ChildGroupText::ChildId,"child"),(reader::ChildGroupText::Schema,"child.empty")]{owner.read_metadata_from_encoding_source(field,&text,&options,&mut control).unwrap();}let mut receipt=owner.produce_owned_operation(8194,&options,&mut control,|_,output,control|encode(output,control)).unwrap();receipt.read_first_schema_from_source(&if refused{"schema"}else{fixture["acceptedSchema"].as_str().unwrap()},&mut control).unwrap();let mut label_failed=false;for terminology in semio_framework_ui_locale::Terminology::ALL{for locale in semio_framework_ui_locale::Locale::ALL{let result=receipt.produce_label(terminology,locale,&mut control,|output|operation.label_into(terminology,locale,output));if refused{assert!(result.is_err());label_failed=true;break;}result.unwrap();}if label_failed{break;}}if refused{assert!(receipt.commit(&mut control).is_err());assert_eq!(owner.operations.len(),0);let view=owner.view().unwrap();assert_eq!(view.labels.len(),0);assert_eq!(view.schema.len(),11);}else{receipt.commit(&mut control).unwrap();assert_eq!(owner.operations.len(),1);assert!(owner.operations[0].iter().eq(b"17".iter().copied().chain(std::iter::repeat_n(b' ',8192))));let view=owner.view().unwrap();assert_eq!(view.labels.len(),1);for terminology in semio_framework_ui_locale::Terminology::ALL{for locale in semio_framework_ui_locale::Locale::ALL{let expected=match locale{semio_framework_ui_locale::Locale::En=>"Set label to target",semio_framework_ui_locale::Locale::De=>"Beschriftung auf target setzen"};let text=view.labels.text(0,terminology,locale).unwrap();assert_eq!(text.len(),expected.len());for(index,byte)in expected.as_bytes().iter().enumerate(){assert_eq!(text.byte(index).unwrap(),*byte)}}}}
            assert_eq!(owner.allocated_bytes(),control.owned_bytes());let allocated=owner.allocated_bytes();assert_eq!(drain_retained(&mut owner),allocated);
        }
        eprintln!("[DEBUG] actual authored locale producer joins the retained original8194 wire success receipt; intrinsic labels and first schema commit atomically at original64k; real segment refusal preserves zero prefix and all partial owners until1/4096; neutral byte producer is not credited as actual typed OpBinary");
    }
    #[test]
    fn child_complete_group_source_candidate_owned_publication_preserves_empty_original_and_parent(){
        mod publication{include!("./🧩️child-operations/📢️publication/🦀️.rs");}
        use semio_framework_value::{NativeEncodeControl,ErasedSnapshotRetirement,retirement::allocation_return::{ParentAllocationReturn,AllocationReturnStep},paged_text::TextReadSource};
        use semio_framework_os_kernel::os_pack::codec::PackEncodeOptions;
        use paged_encoder::PagedChildGroups;
        const CAPACITY:usize=isize::MAX as usize;
        struct Repeated;impl TextReadSource for Repeated{fn byte_len(&self)->usize{8194}fn byte_at(&self,index:usize)->Option<u8>{(index<8194).then_some(b'e')}}
        let fixture:serde_json::Value=serde_json::from_str(include_str!("./🧫️fixtures/📢️owned-publication/🔣️.json")).unwrap();let wire:serde_json::Value=serde_json::from_str(include_str!("./🧫️fixtures/🎒️child-group-wire/🔣️.json")).unwrap();assert_eq!(fixture["originalGroups"],3);assert_eq!(fixture["originalSourceBytes"],8194);assert_eq!(fixture["emptyOwnerTextBytes"],8194);assert_eq!(fixture["maximumAllocationBytes"],65536);assert_eq!(fixture["maximumItems"],1);assert_eq!(fixture["maximumBytes"],4096);
        fn append(children:&mut PagedList<paged_owner::PagedChildOwner,CAPACITY>,child:paged_owner::PagedChildOwner,control:&mut NativeEncodeControl<'_>){while !children.has_reserved_slot(){let required=children.next_allocation_bytes().unwrap();assert!(required<=4096);control.charge(required).unwrap();let step=children.reserve_one(4096).unwrap();assert!(step.progressed);assert_eq!(step.allocated_bytes,required);}assert!(children.push_reserved(child).is_ok());}
        fn child(value:&serde_json::Value,options:&PackEncodeOptions,control:&mut NativeEncodeControl<'_>)->paged_owner::PagedChildOwner{let mut child=paged_owner::PagedChildOwner::empty();for(field,text)in[(reader::ChildGroupText::Owner,value["owner"].as_str().unwrap()),(reader::ChildGroupText::Slot,value["slot"].as_str().unwrap()),(reader::ChildGroupText::ChildId,value["child_id"].as_str().unwrap()),(reader::ChildGroupText::Schema,"child.empty")]{child.read_metadata_from_encoding_source(field,&text,options,control).unwrap();}let bytes=value["ops"][0].as_array().unwrap();let mut receipt=child.produce_owned_operation(bytes.len(),options,control,|_,output,control|{for byte in bytes{output.write_bytes(&[byte.as_u64().unwrap()as u8],control)?;}Ok(())}).unwrap();receipt.read_first_schema_from_source(&value["op_schema"].as_str().unwrap(),control).unwrap();for terminology in semio_framework_ui_locale::Terminology::ALL{for locale in semio_framework_ui_locale::Locale::ALL{receipt.read_label_from_source(terminology,locale,&value["labels"][0][terminology.as_str()][locale.as_str()].as_str().unwrap(),control).unwrap();}}receipt.commit(control).unwrap();child}
        for canceled in[false,true]{let options=PackEncodeOptions::default();let mut allow=|_|true;let mut control=NativeEncodeControl::new(65536,&mut allow);let mut children=PagedList::<paged_owner::PagedChildOwner,CAPACITY>::empty();let first=child(&wire["groups"][0],&options,&mut control);append(&mut children,first,&mut control);let mut empty=paged_owner::PagedChildOwner::empty();empty.read_metadata_from_encoding_source(reader::ChildGroupText::Owner,&Repeated,&options,&mut control).unwrap();for(field,text)in[(reader::ChildGroupText::Slot,"slot"),(reader::ChildGroupText::ChildId,"empty-child"),(reader::ChildGroupText::Schema,"child.empty")]{empty.read_metadata_from_encoding_source(field,&text,&options,&mut control).unwrap();}append(&mut children,empty,&mut control);let second=child(&wire["groups"][1],&options,&mut control);append(&mut children,second,&mut control);let pointer=children[0].operations[0].byte_ref(0).map(|byte|byte as *const u8).unwrap();let allocated=children.allocated_bytes()+children.iter().map(paged_owner::PagedChildOwner::allocated_bytes).sum::<usize>();assert_eq!(allocated,control.owned_bytes());let mut incoming=Some(children);let mut owner=publication::ChildGroupPublicationOwner::take(&mut incoming,&mut control).unwrap();assert!(incoming.is_none());assert!(owner.sources().is_err());assert!(!owner.step(0,4096,&mut control).unwrap());assert_eq!(owner.allocated_bytes(),allocated);assert_eq!(owner.original_count(),3);assert_eq!(owner.original(0).unwrap().operations[0].byte_ref(0).map(|byte|byte as *const u8),Some(pointer));
            if canceled{let continuation=control.pause().unwrap();let mut reject=|_|false;let mut control=NativeEncodeControl::resume(continuation,&mut reject).unwrap();assert!(owner.step(1,4096,&mut control).is_err());assert!(owner.sources().is_err());assert_eq!(owner.original_count(),3);assert_eq!(owner.allocated_bytes(),allocated);assert_eq!(drain_retained(&mut owner),allocated);continue;}
            for _ in 0..32{if owner.step(1,4096,&mut control).unwrap(){break;}}let selected=owner.sources().unwrap();assert_eq!(selected.len(),2);assert_eq!(owner.original_count(),3);assert_eq!(owner.original(1).unwrap().operations.len(),0);assert_eq!(owner.original(1).unwrap().view().unwrap().owner.len(),8194);assert_eq!(owner.original(0).unwrap().operations[0].byte_ref(0).map(|byte|byte as *const u8),Some(pointer));
            let mut encoded=OwnedOperationBytes::try_new(65536,65536).unwrap();let mut symbols=paged_encoder::ChildSymbolOwner::empty();let mut allow=|_|true;let mut encoding=NativeEncodeControl::new(65536,&mut allow);paged_encoder::encode_paged_groups_into(&selected,&options,&mut encoded,&mut symbols,&mut encoding).unwrap();let expected=wire["wire"].as_array().unwrap();assert_eq!(encoded.len(),expected.len());for(index,byte)in expected.iter().enumerate(){assert_eq!(encoded.byte_at(index),Some(byte.as_u64().unwrap()as u8));}assert_eq!(owner.original_count(),3);assert_eq!(owner.original(0).unwrap().operations[0].byte_ref(0).map(|byte|byte as *const u8),Some(pointer));let writer_owned=encoded.allocated_bytes()+symbols.allocated_bytes();assert_eq!(writer_owned,encoding.owned_bytes());assert_eq!(drain_retained(&mut symbols)+{let bytes=encoded.allocated_bytes();drain(&mut encoded);bytes},writer_owned);
            let allocated=owner.allocated_bytes();assert_eq!(allocated,control.owned_bytes());let mut parent=ParentAllocationReturn::<128>::try_new(4096,allocated).unwrap();assert!(!owner.return_one(&mut parent,0,4096).unwrap().progressed);assert_eq!(owner.allocated_bytes(),allocated);let mut returned=0;for _ in 0..32768{if owner.terminal_is_empty(){break;}let before=owner.allocated_bytes();let step=owner.return_one(&mut parent,1,4096).unwrap();assert!(step.progressed);assert_eq!(before-owner.allocated_bytes(),step.returned_allocation_bytes);returned+=step.returned_allocation_bytes;assert!(owner.sources().is_err());}assert!(owner.terminal_is_empty());assert_eq!(returned,allocated);assert_eq!(parent.retained_bytes(),allocated);let mut physical=0;for _ in 0..256{match parent.close_step(1,4096){AllocationReturnStep::Complete=>break,AllocationReturnStep::Pending{released_items,released_bytes}=>{assert!(released_items<=1&&released_bytes<=4096);physical+=released_bytes;}}}assert!(parent.terminal_is_empty());assert_eq!(physical,allocated);
        }
        eprintln!("[DEBUG] owned publication binds original3 children to selected0,2 and exact16722 framing; original8194 source pointer and empty8194 semantic metadata survive; canceled selection retains all originals; real parent alone disposes every index/child/outer allocation1/4096");
    }
    #[test]
    fn child_complete_group_source_candidate_accepted_ledger_retains_original_registry_sources(){
        mod registry{include!("./🧩️child-operations/🪪️registry/🦀️.rs");}
        use semio_framework_value::{ErasedSnapshotRetirement,SnapshotRetirementStep};
        use semio_framework_tool_machine::{ScrubLedger,ScrubInput,ToolStep,ToolAbortReason};
        use semio_framework_os_kernel::{ActorId,HybridLogicalTimestamp};
        use semio_framework_os_kernel::os_pack::codec::PackEncodeOptions;
        let fixture:serde_json::Value=serde_json::from_str(include_str!("./🧫️fixtures/🪪️retained-source-registry/🔣️.json")).unwrap();
        assert_eq!(fixture["sourceBytes"],8194);assert_eq!(fixture["maximumItems"],1);assert_eq!(fixture["maximumBytes"],4096);
        let reuse:serde_json::Value=serde_json::from_str(include_str!("./🧫️fixtures/🔁️retained-source-registry-reuse/🔣️.json")).unwrap();assert_eq!(reuse["sourceBytes"],8194);assert_eq!(reuse["maximumAllocationBytes"],65536);assert_eq!(reuse["maximumItems"],1);assert_eq!(reuse["maximumBytes"],4096);assert_eq!(reuse["reusedIdentity"],3);
        fn child(control:&mut NativeEncodeControl<'_>)->paged_owner::PagedChildOwner{
            let options=PackEncodeOptions::default();let mut owner=paged_owner::PagedChildOwner::empty();
            for(field,text)in[(reader::ChildGroupText::Owner,"owner"),(reader::ChildGroupText::Slot,"slot"),(reader::ChildGroupText::ChildId,"child"),(reader::ChildGroupText::Schema,"child.empty")]{owner.read_metadata_from_encoding_source(field,&text,&options,control).unwrap();}
            let mut receipt=owner.produce_owned_operation(8194,&options,control,|_,output,control|{output.write_bytes(b"17",control)?;for _ in 0..8192{output.write_bytes(b" ",control)?;}Ok(())}).unwrap();
            receipt.read_first_schema_from_source(&"label.set-label",control).unwrap();
            for terminology in semio_framework_ui_locale::Terminology::ALL{for locale in semio_framework_ui_locale::Locale::ALL{receipt.read_label_from_source(terminology,locale,&"label",control).unwrap();}}
            eprintln!("[DEBUG] registry source before outer commit cumulative admitted bytes={}",control.owned_bytes());receipt.commit(control).unwrap();eprintln!("[DEBUG] registry full8194 source after commit cumulative={} retained={}",control.owned_bytes(),owner.allocated_bytes());owner
        }
        fn reconcile(owner:&mut registry::RetainedSourceRegistry<paged_owner::PagedChildOwner>,ledger:&ScrubLedger<registry::RetainedSourceIdentity>,control:&mut NativeEncodeControl<'_>)->Result<(),semio_framework_value::ValueError>{
            owner.reconcile(|identity,control|{for accepted in ledger.provisional(){control.checkpoint()?;control.step()?;if *accepted==identity{return Ok(true)}}Ok(false)},control)
        }
        for host_abort in[false,true]{
            let actor=ActorId("actor".into());let clock=|physical_ms|HybridLogicalTimestamp{actor:0,physical_ms,logical:0};let mut ledger=ScrubLedger::default();
            let mut allow=|_|true;let mut control=NativeEncodeControl::new(65536,&mut allow);let mut owner=registry::RetainedSourceRegistry::empty();
            let first=child(&mut control);let first_allocated=first.allocated_bytes();let first_pointer=first.operations[0].byte_ref(0).map(|byte|byte as *const u8).unwrap();let mut incoming=Some(first);let mut allow_zero=|_|true;let mut zero=NativeEncodeControl::new(0,&mut allow_zero);assert!(owner.insert(&mut incoming,&mut zero).is_err());assert_eq!(owner.retained_count(),0);assert!(owner.terminal_is_empty());assert_eq!(incoming.as_ref().unwrap().operations[0].byte_ref(0).map(|byte|byte as *const u8),Some(first_pointer));let one=owner.insert(&mut incoming,&mut control).unwrap();assert!(incoming.is_none());let scalar_clone=one.clone();let another_scalar_clone=scalar_clone.clone();assert_eq!(another_scalar_clone,one);
            ledger.send("window","tool",&actor,"base",ScrubInput::Tick{gesture:"press".into(),leaves:vec![one]},clock(1)).unwrap();reconcile(&mut owner,&ledger,&mut control).unwrap();
            assert_eq!(owner.borrow(one).unwrap().operations[0].byte_ref(0).map(|byte|byte as *const u8),Some(first_pointer));assert_eq!(owner.retained_count(),1);
            let second=child(&mut control);let second_pointer=second.operations[0].byte_ref(0).map(|byte|byte as *const u8).unwrap();let second_allocated=second.allocated_bytes();let mut incoming=Some(second);let continuation=control.pause().unwrap();let mut reject=|_|false;let mut rejected=NativeEncodeControl::resume(continuation,&mut reject).unwrap();
            assert!(owner.insert(&mut incoming,&mut rejected).is_err());assert_eq!(incoming.as_ref().unwrap().operations[0].byte_ref(0).map(|byte|byte as *const u8),Some(second_pointer));assert!(reconcile(&mut owner,&ledger,&mut rejected).is_err());assert_eq!(owner.borrow(one).unwrap().operations[0].byte_ref(0).map(|byte|byte as *const u8),Some(first_pointer));
            let continuation=rejected.pause().unwrap();let mut allow=|_|true;let mut control=NativeEncodeControl::resume(continuation,&mut allow).unwrap();let two=owner.insert(&mut incoming,&mut control).unwrap();assert!(incoming.is_none());assert_eq!(owner.retained_count(),2);
            ledger.send("window","tool",&actor,"base",ScrubInput::Tick{gesture:"press".into(),leaves:vec![two]},clock(2)).unwrap();
            assert!(owner.borrow(one).is_some());assert_eq!(ledger.provisional().copied().collect::<Vec<_>>(),vec![two]);reconcile(&mut owner,&ledger,&mut control).unwrap();assert!(owner.borrow(one).is_none());assert_eq!(owner.borrow(two).unwrap().operations[0].byte_ref(0).map(|byte|byte as *const u8),Some(second_pointer));assert!(owner.take(one,&mut control).is_err());
            assert_eq!(owner.close_step(0,4096).unwrap(),SnapshotRetirementStep::Pending{released_items:0,released_bytes:0});let mut displaced=0;
            for _ in 0..16384{if owner.retained_count()==1{break}match owner.close_step(1,4096).unwrap(){SnapshotRetirementStep::Pending{released_items,released_bytes}=>{assert!(released_items<=1&&released_bytes<=4096);displaced+=released_bytes;},SnapshotRetirementStep::Complete=>panic!("live accepted source disappeared"),SnapshotRetirementStep::Blocked=>panic!("admitted displaced original cannot block under1/4096")}}
            assert_eq!(owner.retained_count(),1);assert_eq!(displaced,first_allocated);assert_eq!(owner.borrow(two).unwrap().operations[0].byte_ref(0).map(|byte|byte as *const u8),Some(second_pointer));
            if host_abort{
                assert!(matches!(ledger.abort("window",Some("press"),ToolAbortReason::CaptureLost),ToolStep::Aborted(..)));assert!(ledger.is_empty());let mut incoming=Some(owner.take(two,&mut control).unwrap());assert_eq!(incoming.as_ref().unwrap().operations[0].byte_ref(0).map(|byte|byte as *const u8),Some(second_pointer));reconcile(&mut owner,&ledger,&mut control).unwrap();assert!(owner.borrow(two).is_none());let metadata=owner.allocated_bytes();let mut second_released=0;
                for _ in 0..16384{match owner.close_step(1,4096).unwrap(){SnapshotRetirementStep::Pending{released_items,released_bytes}=>{assert!(released_items<=1&&released_bytes<=4096);second_released+=released_bytes;if released_items==0&&released_bytes==0{break}},SnapshotRetirementStep::Complete=>break,SnapshotRetirementStep::Blocked=>panic!("admitted aborted original cannot block under1/4096")}}
                assert_eq!(owner.retained_count(),0);let third_pointer=second_pointer;let third_allocated=second_allocated;let before_readmission=control.owned_bytes();let three=owner.insert(&mut incoming,&mut control).unwrap();assert!(incoming.is_none());assert!(control.owned_bytes()>=before_readmission&&control.owned_bytes()<=65536);
                ledger.send("window","tool",&actor,"base",ScrubInput::Tick{gesture:"new-press".into(),leaves:vec![three]},clock(3)).unwrap();let retained=owner.borrow(three).expect("open registry lost monotone identity after abort and bounded source drain");assert_eq!(retained.operations[0].byte_ref(0).map(|byte|byte as *const u8),Some(third_pointer));assert_eq!(second_released,0);assert!(owner.allocated_bytes()>=metadata);reconcile(&mut owner,&ledger,&mut control).unwrap();
                let ToolStep::Committed(_,leaves)=ledger.send("window","tool",&actor,"base",ScrubInput::Commit{gesture:"new-press".into(),leaves:vec![three]},clock(4)).unwrap()else{panic!("reused open registry did not commit actual new press")};assert_eq!(leaves,vec![three]);let mut published=owner.take(three,&mut control).unwrap();assert_eq!(published.operations[0].byte_ref(0).map(|byte|byte as *const u8),Some(third_pointer));assert_eq!(published.operations[0].len(),8194);reconcile(&mut owner,&ledger,&mut control).unwrap();assert!(owner.take(three,&mut control).is_err());assert_eq!(drain_retained(&mut published),third_allocated);let metadata=owner.allocated_bytes();owner.begin_close();assert_eq!(drain_retained(&mut owner),metadata);
                assert!(matches!(ledger.send("window","tool",&actor,"base",ScrubInput::Commit{gesture:"new-press".into(),leaves:vec![three]},clock(5)).unwrap(),ToolStep::Idle));
            }else{
                let ToolStep::Committed(_,leaves)=ledger.send("window","tool",&actor,"base",ScrubInput::Commit{gesture:"press".into(),leaves:vec![two]},clock(3)).unwrap()else{panic!("accepted release did not commit")};assert_eq!(leaves,vec![two]);assert!(ledger.is_empty());
                let mut published=owner.take(leaves[0],&mut control).unwrap();assert_eq!(published.operations[0].byte_ref(0).map(|byte|byte as *const u8),Some(second_pointer));assert_eq!(published.operations[0].len(),8194);assert_eq!(published.operations[0].byte_at(0),Some(b'1'));assert_eq!(published.operations[0].byte_at(1),Some(b'7'));for at in 2..8194{assert_eq!(published.operations[0].byte_at(at),Some(b' '));}
                reconcile(&mut owner,&ledger,&mut control).unwrap();assert!(owner.take(two,&mut control).is_err());assert!(matches!(ledger.send("window","tool",&actor,"base",ScrubInput::Commit{gesture:"press".into(),leaves:vec![two]},clock(4)).unwrap(),ToolStep::Idle));
                assert_eq!(drain_retained(&mut published),second_allocated);let metadata=owner.allocated_bytes();owner.begin_close();assert_eq!(drain_retained(&mut owner),metadata);
            }
            assert!(owner.terminal_is_empty());assert_eq!(owner.retained_count(),0);
        }
        eprintln!("[DEBUG] actual ScrubLedger scalar clones preserve original8194 sources; accepted tick displacement retires only absent original; extraction precedes accepted-ledger reconcile; commit transfers pointer once, late commit stays idle, host abort revokes authority first; all original/registry/publication allocations independently drain1/4096");
    }
    #[test]
    fn child_complete_actual_authored_operation_producer_joins_original_projection_and_retained_receipt(){
        use semio_framework_value::{ErasedSnapshotRetirement,NativeEncodeControl};
        use semio_framework_os_kernel::os_pack::codec::PackEncodeOptions;
        use std::sync::atomic::{AtomicUsize,Ordering};
        struct Observed<'a>{inner:&'a mut dyn OperationByteOutput,accepted:&'a AtomicUsize}
        impl OperationByteOutput for Observed<'_>{fn write_bytes(&mut self,bytes:&[u8],control:&mut NativeEncodeControl<'_>)->Result<(),semio_framework_os_kernel::os_pack::PackRefusal>{for byte in bytes{self.inner.write_bytes(&[*byte],control)?;self.accepted.fetch_add(1,Ordering::Relaxed);}Ok(())}}
        let fixture:serde_json::Value=serde_json::from_str(include_str!("./🧫️fixtures/📨️authored-operation-source/🔣️.json")).unwrap();assert_eq!(fixture["textBytes"],8194);assert_eq!(fixture["frameBytes"],8202);assert_eq!(fixture["maximumAllocationBytes"],65536);assert_eq!(fixture["maximumItems"],1);assert_eq!(fixture["maximumBytes"],4096);
        for mode in["complete","file-one-short","depth-zero","initial4096","interior-cancel"]{
            let mutation=crate::test_app_mutation_fixture::TestMutation::from(crate::test_app_mutation_fixture::SetLabel{value:"x".repeat(8194)});let input_pointer=match &mutation{crate::test_app_mutation_fixture::TestMutation::SetLabel(leaf)=>leaf.value.as_ptr(),_=>unreachable!()};
            let accepted=AtomicUsize::new(0);let mut continue_encoding=|_:semio_framework_value::native_encoding::NativeEncodeProgress|mode!="interior-cancel"||accepted.load(Ordering::Relaxed)<256;let mut control=NativeEncodeControl::new(if mode=="initial4096"{4096}else{65536},&mut continue_encoding);let mut options=PackEncodeOptions::default();options.limits.max_file_len=if mode=="file-one-short"{8201}else{8202};if mode=="depth-zero"{options.limits.max_depth=0;}
            let mut child=paged_owner::PagedChildOwner::empty();for(field,text)in[(reader::ChildGroupText::Owner,"owner"),(reader::ChildGroupText::Slot,"slot"),(reader::ChildGroupText::ChildId,"child"),(reader::ChildGroupText::Schema,"child.empty")]{child.read_metadata_from_encoding_source(field,&text,&options,&mut control).unwrap();}
            let mut capsule=mutation.retained_pack_operation();
            let result=child.produce_owned_operation(8202,&options,&mut control,|options,output,control|semio_framework_os_kernel::variants_binary::encode_op_into(&mut capsule,options,&mut Observed{inner:output,accepted:&accepted},control));
            match result{Ok(mut receipt)=>{assert_eq!(mode,"complete");receipt.read_first_schema_from_source(&fixture["expected"]["schema"].as_str().unwrap(),&mut control).unwrap();for terminology in semio_framework_ui_locale::Terminology::ALL{for locale in semio_framework_ui_locale::Locale::ALL{let crate::test_app_mutation_fixture::TestMutation::SetLabel(leaf)=&mutation else{unreachable!()};receipt.produce_label(terminology,locale,&mut control,|output|leaf.label_into(terminology,locale,output)).unwrap();}}receipt.commit(&mut control).unwrap();},Err(_)=>assert_ne!(mode,"complete")}
            assert_eq!(match &mutation{crate::test_app_mutation_fixture::TestMutation::SetLabel(leaf)=>leaf.value.as_ptr(),_=>unreachable!()},input_pointer);
            if mode=="complete"{assert_eq!(child.operations.len(),1);assert_eq!(child.operations[0].len(),8202);let canonical=<crate::test_app_mutation_fixture::TestMutation as protocol::OpBinary>::encode_op(&mutation).unwrap();assert_eq!(canonical.len(),8202);for(index,byte)in canonical.iter().copied().enumerate(){assert_eq!(child.operations[0].byte_at(index),Some(byte));}let prefix=[1,1,0,1,0,7,130,64];for(index,byte)in prefix.into_iter().enumerate(){assert_eq!(child.operations[0].byte_at(index),Some(byte));}for index in 8..8202{assert_eq!(child.operations[0].byte_at(index),Some(b'x'));}let schema=fixture["expected"]["schema"].as_str().unwrap();let view=child.view().unwrap();assert_eq!(view.schema.len(),schema.len());for(index,byte)in schema.bytes().enumerate(){assert_eq!(view.schema.byte(index).unwrap(),byte);}}
            else{assert_eq!(child.operations.len(),0);assert!(child.retained_partial_source().is_some());if mode=="interior-cancel"{assert_eq!(accepted.load(Ordering::Relaxed),256);assert_eq!(child.retained_partial_source().unwrap().len(),256);}assert!(child.retained_partial_source().unwrap().len()<8202);}
            let scratch=capsule.allocated_bytes();let before=semio_framework_trace::retained_heap_bytes_on_this_thread();assert_eq!(capsule.retire_one(0,4096).unwrap(),(false,0,0));assert_eq!(semio_framework_trace::retained_heap_bytes_on_this_thread(),before);assert_eq!(capsule.allocated_bytes(),scratch);
            let mut released=0;for _ in 0..32768{let before=semio_framework_trace::retained_heap_bytes_on_this_thread();let step=capsule.retire_one(1,4096).unwrap();let after=semio_framework_trace::retained_heap_bytes_on_this_thread();assert_eq!(usize::try_from(before-after).unwrap(),step.2);assert!(step.1<=1&&step.2<=4096);released+=step.2;if !step.0{break}}assert_eq!(released,scratch);assert_eq!(capsule.allocated_bytes(),0);assert_eq!(capsule.variant_identity().unwrap_err().kind(),semio_framework_value::ValueRefusalKind::InvariantViolated);
            let allocated=child.allocated_bytes();assert_eq!(drain_retained(&mut child),allocated);assert!(child.terminal_is_empty());
        }
        eprintln!("[DEBUG] actual authored TestMutation direct projection emits exact8202 frame with full8194 payload into retained receipt and actual four locale cells; caller input pointer survives full-policy success/refusal; whole-file/depth/initial4096/interior256 cancellation retains actual source prefix and explicit1/4096 child closure; typed String and producer metadata/scaffold lifetime remain separate unqualified owners");
    }
    #[test]
    fn child_complete_actual_authored_typed_source_event_observer_matches_system_backing(){
        let(mut bytes,allocated)=semio_framework_trace::observe_heap_allocations_on_this_thread(||vec![b'x';8194]);assert_eq!((allocated.requested_bytes,allocated.released_bytes),(8194,0));assert!(!allocated.overflowed);
        let(_,shortened)=semio_framework_trace::observe_heap_allocations_on_this_thread(||bytes.truncate(4096));assert_eq!((shortened.requested_bytes,shortened.released_bytes),(0,0));assert_eq!(bytes.capacity(),8194);
        let(_,resized)=semio_framework_trace::observe_heap_allocations_on_this_thread(||bytes.shrink_to_fit());assert_eq!(bytes.capacity(),4096);assert_eq!((resized.requested_bytes,resized.released_bytes,resized.largest_release_bytes),(4096,8194,8194));assert!(!resized.overflowed);
        let(_,released)=semio_framework_trace::observe_heap_allocations_on_this_thread(||drop(bytes));assert_eq!((released.requested_bytes,released.released_bytes,released.largest_release_bytes),(0,4096,4096));
        let(pages,allocated)=semio_framework_trace::observe_heap_allocations_on_this_thread(||(vec![0u8;4096],vec![0u8;4096]));assert_eq!((allocated.requested_bytes,allocated.released_bytes),(8192,0));let(inner,outer)=semio_framework_trace::observe_heap_allocations_on_this_thread(||semio_framework_trace::observe_heap_allocations_on_this_thread(||drop(pages)).1);assert_eq!(inner,outer);assert_eq!((outer.requested_bytes,outer.released_bytes,outer.largest_release_bytes),(0,8192,4096));assert!(!outer.overflowed);
        eprintln!("[DEBUG] actual system truncate releases0; realloc8194-to4096 records original8194 release+new4096 request; nested two4096 releases total8192 with largest4096");
    }
    #[test]
    fn child_complete_actual_authored_typed_source_physical8194_grant4096(){
        use semio_framework_value::retirement::{RetireOwned,RetirementStep};
        let fixture:serde_json::Value=serde_json::from_str(include_str!("./🧫️fixtures/♻️typed-source-physical/🔣️.json")).unwrap();
        let backing_bytes=fixture["textBytes"].as_u64().unwrap() as usize;
        let work_bytes=fixture["maximumBytes"].as_u64().unwrap() as usize;
        let admission=fixture["maximumAllocationBytes"].as_u64().unwrap() as usize;
        assert_eq!((backing_bytes,work_bytes,admission),(8194,4096,65536));assert_eq!(fixture["maximumItems"],1);assert_eq!(fixture["textByte"],b'x');
        let mutation=crate::test_app_mutation_fixture::SetLabel{value:"x".repeat(backing_bytes)};
        let pointer=mutation.value.as_ptr();let backing=mutation.value.capacity();assert_eq!(backing,backing_bytes);
        let ((mut cursor,source_pointer),construction)=semio_framework_trace::observe_heap_allocations_on_this_thread(||{let source_pointer=mutation.value.as_ptr();(mutation.value.retirement(),source_pointer)});
        assert_eq!(pointer,source_pointer);assert_eq!(construction.released_bytes,0);assert!(!construction.overflowed);
        let (step,zero)=semio_framework_trace::observe_heap_allocations_on_this_thread(||cursor.close_step(Default::default()));
        assert!(matches!(step,RetirementStep::BudgetExhausted));assert_eq!((zero.requested_bytes,zero.released_bytes),(0,0));assert!(!cursor.terminal_is_empty());
        let mut processed=0;
        while cursor.next_work_byte_demand()!=0{
            let (step,observed)=semio_framework_trace::observe_heap_allocations_on_this_thread(||cursor.close_step(semio_framework_value::retained_clone::RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:work_bytes,..Default::default()}));
            assert!(!observed.overflowed);assert_eq!((observed.requested_bytes,observed.released_bytes),(0,0));
            let RetirementStep::ProcessedBytes(bytes)=step else{panic!("logical source drain must preserve the exact backing")};
            assert!(bytes>0&&bytes<=work_bytes);processed+=bytes;assert!(processed<=backing_bytes);
            eprintln!("[DEBUG] actual typed String logical processed={bytes} total={processed} physicalRelease=0 originalBacking={backing}");
        }
        assert_eq!(processed,backing_bytes);
        for grant in fixture["physicalRelease"]["deniedBytes"].as_array().unwrap(){
            let bytes=grant.as_u64().unwrap() as usize;assert_eq!(cursor.next_close_byte_demand(),Some(backing));
            let (step,observed)=semio_framework_trace::observe_heap_allocations_on_this_thread(||cursor.close_step(semio_framework_value::retained_clone::RetainedCloneGrant{maximum_items:1,maximum_release_bytes:bytes,..Default::default()}));
            assert!(matches!(step,RetirementStep::BudgetExhausted));assert!(!cursor.terminal_is_empty());
            assert_eq!(cursor.next_close_byte_demand(),Some(backing));assert_eq!((observed.requested_bytes,observed.released_bytes),(0,0));assert!(!observed.overflowed);
            eprintln!("[DEBUG] actual typed String physical denied grant={bytes} retainedDemand={backing} released=0");
        }
        let demand=cursor.next_close_byte_demand().expect("retained physical source demand");
        assert_eq!(demand,fixture["physicalRelease"]["demandBytes"].as_u64().unwrap() as usize);assert!(demand<=admission);
        let (step,released)=semio_framework_trace::observe_heap_allocations_on_this_thread(||cursor.close_step(semio_framework_value::retained_clone::RetainedCloneGrant{maximum_items:1,maximum_release_bytes:demand,..Default::default()}));
        let RetirementStep::Bytes(reported)=step else{panic!("whole funded source release must publish its exact physical receipt")};
        assert_eq!(reported,fixture["physicalRelease"]["reportedBytes"].as_u64().unwrap() as usize);
        assert_eq!((released.requested_bytes,released.released_bytes,released.largest_release_bytes),(0,reported,reported));assert!(!released.overflowed);assert_eq!(reported,backing);
        let (step,terminal)=semio_framework_trace::observe_heap_allocations_on_this_thread(||cursor.close_step(semio_framework_value::retained_clone::RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:work_bytes,..Default::default()}));
        assert!(matches!(step,RetirementStep::Complete));assert!(cursor.terminal_is_empty());assert_eq!((terminal.requested_bytes,terminal.released_bytes),(0,0));assert!(!terminal.overflowed);
        let frame=cursor.terminal_release_bytes().expect("separate terminal cursor extent");
        assert_eq!(frame,std::mem::size_of_val(cursor.as_ref()));assert_eq!(frame,construction.requested_bytes);assert!(frame<=admission);assert_eq!(fixture["physicalRelease"]["terminalFrameSeparate"],true);
        let (_,scaffold)=semio_framework_trace::observe_heap_allocations_on_this_thread(||drop(cursor));
        assert_eq!((scaffold.requested_bytes,scaffold.released_bytes,scaffold.largest_release_bytes),(0,frame,frame));assert!(!scaffold.overflowed);
        eprintln!("[DEBUG] actual typed String whole release grant={demand} physical={reported}; separate terminal frame funded={frame} physical={}",scaffold.released_bytes);
    }
}
