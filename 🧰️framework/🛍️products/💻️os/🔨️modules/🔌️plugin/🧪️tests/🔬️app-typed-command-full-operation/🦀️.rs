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
        fn decide(&self, description: &str, coalesce_key: &str, maximum: usize) -> TypedCommandCensusDecision;
    }

    struct OwnedTypedCommandCensus;

    impl TypedCommandCensusOracle for OwnedTypedCommandCensus {
        fn decide(&self, description: &str, coalesce_key: &str, maximum: usize) -> TypedCommandCensusDecision {
            let mut bytes = 0usize;
            let mut accepted = true;
            for _ in description.as_bytes() {
                accepted &= bytes.checked_add(1).is_some_and(|next| {
                    bytes = next;
                    next <= maximum
                });
            }
            for _ in coalesce_key.as_bytes() {
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
        fn decide(&self, description: &str, coalesce_key: &str, maximum: usize) -> TypedCommandCensusDecision {
            let value = serde_json::json!({ "coalesceKey": coalesce_key, "description": description, "uiScopeFields": 1 });
            let bytes = value["description"].as_str().expect("oracle description").as_bytes().len() + value["coalesceKey"].as_str().expect("oracle coalesce key").as_bytes().len() + value["uiScopeFields"].as_u64().expect("oracle UI scope") as usize;
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
                let next_root = request.mutation.diff(request.base.as_ref()).diff().apply(request.base.as_ref()).map_err(|error| error.to_string())?;
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

        fn close_step(&mut self, grant: store::ArtifactStoreOneItemGrant) -> Result<store::SnapshotRetirementStep, String> {
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
        fn close_step(&mut self, maximum_items: usize, _maximum_bytes: usize) -> Result<store::SnapshotRetirementStep, String> {
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

    struct PublicationPresenceLocalRootRetirementFactory;

    impl store::SnapshotRetirementFactory<PublicationPresence> for PublicationPresenceLocalRootRetirementFactory {
        fn retire(&self, snapshot: std::sync::Arc<PublicationPresence>) -> Box<dyn store::ErasedSnapshotRetirement> {
            Box::new(PublicationPresenceLocalRootRetirement { root: Some(snapshot) })
        }
    }

    pub(super) async fn retained_cancellation_publication_boundaries<A: ArtifactApp<Presence = PublicationPresence, PresenceMutation = PublicationPresenceMutation> + Default>() {
        let fixture: Value = serde_json::from_str(include_str!("../../🥇️tool-latest-wins.json")).expect("language-neutral cancellation boundaries");
        let grant = store::ArtifactStoreOneItemGrant { maximum_items: fixture["maximumItems"].as_u64().unwrap() as usize, maximum_bytes: fixture["maximumBytes"].as_u64().unwrap() as usize };
        assert_eq!(grant.maximum_items, 1);
        assert_eq!(grant.maximum_bytes, TYPED_OPERATION_RESULT_PAGE_BYTES);
        for case in fixture["publicationCases"].as_array().unwrap() {
            let boundary = case["cancelAt"].as_str().unwrap();
            let mut app = VcsArtifactApp::<A>::new(A::default()).await;
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
                completion: None,
                raw_input: None,
                output_chunks: None,
                cancellation_lease: Some(lease),
                terminal_outcome: None,
                terminal_seen: true,
                publication: Some(ArtifactToolCompletionValue::Emit(Ok(Emit::default()), EphemeralEmit::default())),
                pending_artifact_publication: None,
                pending_child_publication: None,
                captured_child_content: Some(std::sync::Arc::new(ChildContentView::EMPTY)),
                captured_child_content_generation: 0,
                result_page: None,
                result_page_presented: false,
                result_sequence: 0,
                publication_attempt: 0,
                ui_pending: true,
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
                    app.publish_mounted_typed_operation_unit(&mut mounted).unwrap();
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
            app.publish_mounted_typed_operation_unit(&mut mounted).unwrap();
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
            eprintln!("[DEBUG] actual mounted publisher cancellation boundary {boundary} retained its exact Store root/ACK and reached bounded terminal close");
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

    fn fixture_latest_wins_key(scope: &Value) -> std::sync::Arc<String> {
        let parts = [scope["document"].as_str().unwrap(), scope["controller"].as_str().unwrap(), scope["tool"].as_str().unwrap(), scope["target"].as_str().unwrap()];
        let mut copy = ToolLatestWinsKeyCopy::new(scope["instance"].as_u64().unwrap() as u32, parts).unwrap();
        assert_eq!(copy.advance(parts, 0, 4_096), PluginCloseStep::Pending { released_items: 0, released_bytes: 0 });
        assert_eq!(copy.advance(parts, 1, 0), PluginCloseStep::Pending { released_items: 0, released_bytes: 0 });
        for _ in 0..100_000 {
            match copy.advance(parts, 1, TYPED_OPERATION_RESULT_PAGE_BYTES) {
                PluginCloseStep::Complete => return copy.take_key().unwrap(),
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

    pub(super) async fn retained_document_cancellation<A: ArtifactApp + Default>(factory: std::sync::Arc<dyn store::ArtifactStoreOneItemPreparationFactory<A::Snapshot, A::Mutation>>, mutation: fn() -> A::Mutation, observe: fn(&A::Snapshot) -> i32) {
        let fixture: Value = serde_json::from_str(include_str!("../../🥇️tool-latest-wins.json")).unwrap();
        for case in fixture["publicationCases"].as_array().unwrap() {
            for delayed_ack in [false, true] {
                let boundary = case["cancelAt"].as_str().unwrap();
                let mut app = VcsArtifactApp::<A>::new(A::default()).await;
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
                    completion: None,
                    raw_input: None,
                    output_chunks: None,
                    cancellation_lease: Some(lease),
                    terminal_outcome: None,
                    terminal_seen: true,
                    publication: Some(ArtifactToolCompletionValue::Emit(Ok(Emit::default()), EphemeralEmit::default())),
                    pending_artifact_publication: None,
                    pending_child_publication: None,
                    captured_child_content: Some(std::sync::Arc::new(ChildContentView::EMPTY)),
                    captured_child_content_generation: 0,
                    result_page: None,
                    result_page_presented: false,
                    result_sequence: 0,
                    publication_attempt: 0,
                    ui_pending: true,
                    stage: MountedTypedCommandFullOperationStage::Publishing,
                };
                if boundary != "producer" {
                    let pending =
                        app.store.begin_apply_batch(operation.operation, generation, revision, "fixture".into(), vec![mutation()], None, HistoryLane::Document, Some(&factory)).unwrap_or_else(|_| panic!("exact scalar document preparation admission"));
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
                        app.publish_mounted_typed_operation_unit(&mut mounted).unwrap();
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
                        assert!(!mounted.has_runnable_work());
                        for _ in 0..fixture["resultAck"]["preAckPolls"].as_u64().unwrap() {
                            assert!(mounted.take_result_page().is_none());
                        }
                        assert!(mounted.acknowledge_result_page(presented.token).unwrap());
                    } else {
                        assert!(mounted.acknowledge_result_page(receipt.token).unwrap());
                    }
                }
                app.publish_mounted_typed_operation_unit(&mut mounted).unwrap();
                let final_page = if delayed_ack {
                    let presented = mounted.take_result_page().unwrap();
                    let mut deliveries = 1;
                    for _ in 0..fixture["resultAck"]["preAckPolls"].as_u64().unwrap() {
                        deliveries += usize::from(mounted.take_result_page().is_some());
                    }
                    assert_eq!(deliveries, fixture["resultAck"]["deliveries"].as_u64().unwrap() as usize);
                    assert_eq!(presented.token.attempt, fixture["resultAck"]["attempt"].as_u64().unwrap() as u8);
                    assert!(!mounted.has_runnable_work());
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
                eprintln!("[DEBUG] real mounted Document publication {boundary}, delayed ACK={delayed_ack}: count/revision/root retained and close terminal");
            }
        }
    }

    #[test]
    fn retained_latest_wins_full_domain_exact_keys_match_serde_oracle_and_retire() {
        let fixture: Value = serde_json::from_str(include_str!("../../🥇️tool-latest-wins.json")).unwrap();
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
            assert!(registry.begin(first_operation.0, fixture_latest_wins_key(first), &first_lease));
            for _ in 0..100_000 {
                if registry.take_outcome(first_operation.0) == Some(true) {
                    break;
                }
                let _ = registry.advance(1, TYPED_OPERATION_RESULT_PAGE_BYTES);
            }
            assert!(!first_lease.token.is_cancelled_now());
            for _ in 0..100_000 {
                if registry.can_begin() {
                    break;
                }
                let _ = registry.advance(1, TYPED_OPERATION_RESULT_PAGE_BYTES);
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
            assert!(registry.begin(next_operation.0, fixture_latest_wins_key(next), &next_lease));
            let mut accepted = false;
            for _ in 0..100_000 {
                if let Some(result) = registry.take_outcome(next_operation.0) {
                    accepted = result;
                    break;
                }
                match registry.advance(1, TYPED_OPERATION_RESULT_PAGE_BYTES) {
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
                match registry.advance(1, TYPED_OPERATION_RESULT_PAGE_BYTES) {
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
            eprintln!("[DEBUG] exact latest-wins key {} matched independent serde scope equality and retired its 8192-byte identity", case["id"]);
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
        let fixture: Value = serde_json::from_str(include_str!("../../🔗️tool-latest-wins-integration.json")).unwrap();
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
        eprintln!("[DEBUG] lock-held keyed finish retained its preadmitted release marker; one-slot sweep released only its exact claim");
    }

    #[test]
    fn retained_latest_wins_rebase_rebinds_exact_registered_cancellation_authority() {
        let fixture: Value = serde_json::from_str(include_str!("../../🔗️tool-latest-wins-integration.json")).unwrap();
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
        eprintln!("[DEBUG] retained keyed rebase rejected the old cancellation key and accepted only the exact refreshed revision/generation");
    }

    #[test]
    fn retained_latest_wins_full_registry_reclaims_completed_targets_before_admission() {
        let fixture: Value = serde_json::from_str(include_str!("../../🔗️tool-latest-wins-integration.json")).unwrap();
        let cancellations = ToolCancellationHandle::default();
        let mut registry = ToolLatestWinsRegistry::new();
        let count = fixture["reclamation"]["sequentialCompletedTargets"].as_u64().unwrap();
        let mut accepted = 0;
        for index in 0..count {
            let operation = semio_framework_job::allocate_operation_id();
            let lease = cancellations
                .begin_keyed(ToolOperationKey { app_instance_id: 7, document: ArtifactDocumentAuthority(7), operation_id: operation, base_revision: semio_framework_job::RevisionId(1), generation: semio_framework_job::Generation(0) })
                .unwrap();
            let key = std::sync::Arc::new(format!("target-{index:04}"));
            let mut begun = false;
            let mut result = None;
            for _ in 0..100_000 {
                if !begun {
                    begun = registry.begin(operation.0, key.clone(), &lease);
                }
                if begun {
                    result = registry.take_outcome(operation.0);
                }
                if result.is_some() {
                    break;
                }
                match registry.advance(1, TYPED_OPERATION_RESULT_PAGE_BYTES) {
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
            accepted += 1;
            for _ in 0..100_000 {
                if registry.can_begin() {
                    break;
                }
                registry.advance(1, TYPED_OPERATION_RESULT_PAGE_BYTES);
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
            registry.advance(1, TYPED_OPERATION_RESULT_PAGE_BYTES);
        }
        assert!(registry.terminal_is_empty());
        eprintln!("[DEBUG] retained latest-wins registry admitted65sequential completed targets under64live slots and one-item/4096-byte grants");
    }

    pub(super) async fn retained_latest_wins_slot_and_publication_fairness<A: ArtifactApp<Presence = PublicationPresence, PresenceMutation = PublicationPresenceMutation> + Default>() {
        let fixture: Value = serde_json::from_str(include_str!("../../🔗️tool-latest-wins-integration.json")).unwrap();
        let mut app = VcsArtifactApp::<A>::new(A::default()).await;
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
                    completion: None,
                    raw_input: None,
                    output_chunks: None,
                    cancellation_lease: Some(lease),
                    terminal_outcome: None,
                    terminal_seen: true,
                    publication: Some(ArtifactToolCompletionValue::Emit(Ok(Emit::default()), EphemeralEmit::default())),
                    pending_artifact_publication: pending,
                    pending_child_publication: None,
                    captured_child_content: Some(std::sync::Arc::new(ChildContentView::EMPTY)),
                    captured_child_content_generation: 0,
                    result_page: None,
                    result_page_presented: false,
                    result_sequence: 0,
                    publication_attempt: 0,
                    ui_pending: false,
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
        assert_eq!(app.tool_operations.get(1).unwrap().stage, MountedTypedCommandFullOperationStage::Worker);
        assert!(app.take_typed_operation_result_page(7).is_none());
        assert!(app.take_typed_operation_result_page(7).is_some());
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
                    completion: None,
                    raw_input: None,
                    output_chunks: None,
                    cancellation_lease: None,
                    terminal_outcome: None,
                    terminal_seen: true,
                    publication: None,
                    pending_artifact_publication: None,
                    pending_child_publication: None,
                    captured_child_content: Some(std::sync::Arc::new(ChildContentView::EMPTY)),
                    captured_child_content_generation: 0,
                    result_page: Some(page),
                    result_page_presented: true,
                    result_sequence: 0,
                    publication_attempt: 0,
                    ui_pending: false,
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
        assert_eq!(app.maintenance_step(1, TYPED_OPERATION_RESULT_PAGE_BYTES).unwrap(), PluginCloseStep::Pending { released_items: 0, released_bytes: 0 });
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
        eprintln!("[DEBUG] reserved modulo-collision rejected; ready second publisher and result ACK progressed past a retained first worker");
    }

    #[test]
    fn language_neutral_empty_single_max_and_plus_one_match_the_test_only_oracle() {
        let fixture: Value = serde_json::from_str(FIXTURE).expect("language-neutral typed-command fixture");
        let maximum = fixture["capacities"]["maxOutputBytes"].as_u64().expect("maximum output bytes") as usize;
        let owned = OwnedTypedCommandCensus;
        let oracle = SerdeJsonTypedCommandCensus;
        for case in fixture["cases"].as_array().expect("fixture cases") {
            let description = case["description"].as_str().expect("fixture description");
            let coalesce_key = case["coalesceKey"].as_str().expect("fixture coalesce key");
            let expected = TypedCommandCensusDecision { bytes: case["expectedBytes"].as_u64().expect("fixture expected bytes") as usize, accepted: case["accepted"].as_bool().expect("fixture acceptance") };
            assert_eq!(owned.decide(description, coalesce_key, maximum), expected);
            assert_eq!(oracle.decide(description, coalesce_key, maximum), expected);
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
    fn retained_child_wire_rejection_retires_nested_owners_under_the_production_grant() {
        let fixture: Value = serde_json::from_str(include_str!("../../../🏪️store/📢️member-publication.json")).expect("retained child fixture");
        let row = &fixture["orderedMembers"][0];
        let mut wire = row["wire"].as_str().unwrap().as_bytes().to_vec();
        wire.resize(wire.len() + row["paddingBytes"].as_u64().unwrap() as usize, b' ');
        let wire_bytes = wire.len();
        let mut child = ChildEmit { slot: "slot".into(), child_id: "child".into(), ops: vec![wire], op_schema: SchemaId("demo.member.json-number".into()), labels: vec!["ä🧩".into()] };
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
        eprintln!("[DEBUG] child wire retirement released {bytes} bytes under the production grant");
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
            let gate = source[..decoder].rfind("self.require_complete_tool_operation_pipeline(&admission)?").expect("typed route pre-decoder gate");
            assert!(gate < decoder, "generic command construction must remain behind full-operation admission");
        }
        let intent = source.find("async fn handle_intent_frame(&mut self, intent: &UiIntent").expect("intent route");
        let intent_end = source[intent..].find("async fn resume_task_emit").map(|offset| intent + offset).expect("intent route end");
        let intent_route = &source[intent..intent_end];
        for forbidden in ["serde_json::to_value", "serde_json::to_vec", "A::command_from_intent"] {
            assert!(!intent_route.contains(forbidden), "intent route performed pre-admission work: {forbidden}");
        }
        assert!(source.contains("pub fn dispatch_typed"), "typed-value route must remain available to the dependency testkit");
    }

    #[test]
    fn fixture_contract_is_anchored_to_the_production_retained_factory_publisher_and_host_receivers() {
        let source = include_str!("../../🦀️.rs");
        let admission_marker = ["async fn admit_command_json_with_", "proof(&self"].concat();
        let admission_start = source.rfind(&admission_marker).expect("production raw JSON admission");
        let admission_end = source[admission_start..].find("pub async fn new(app: A)").map(|offset| admission_start + offset).expect("admission route end");
        let admission = &source[admission_start..admission_end];
        let reserve = admission.find(".begin_exact_wire").expect("maximum extent authority");
        let encode = admission.find("protocol::json::to_json_string(&(verb, wire_args))").expect("bounded wire encoder");
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
        for retained_seam in ["pending_artifact_publication", "begin_apply_batch", "advance_apply_batch", "ArtifactStoreOneItemAdvance::Published", "publication.close_step(grant)"] {
            assert!(publisher.contains(retained_seam), "production publisher lost its retained one-item seam: {retained_seam}");
        }
        for forbidden in [".apply_one(", "artifact_mutations.last().cloned()", "config_mutations.last().cloned()", "draft_mutations.last().cloned()", "presence.last().cloned()", "transient.last().cloned()"] {
            assert!(!publisher.contains(forbidden), "production publisher restored a monolithic or clone-then-pop shortcut: {forbidden}");
        }
        let freshness = publisher.find("typed-operation pending publication rejected a stale immutable document root").expect("per-turn document freshness gate");
        let pending_advance = publisher[freshness..].find("if let Some(pending) = mounted.pending_artifact_publication.as_mut()").map(|offset| freshness + offset).expect("pending publication advance");
        assert!(freshness < pending_advance, "document freshness must fail closed before every retained lane advance");
        assert!(publisher[..pending_advance].contains("pending.begin_close()"));
        assert!(publisher[..pending_advance].contains("pending.close_step(1, TYPED_OPERATION_RESULT_PAGE_BYTES)"));
        assert!(!publisher.contains("dispatch_emit_group("));
        assert!(!publisher.contains(".await"));
        assert!(publisher.contains("typed_effect_outbox.push"));
        assert!(publisher.contains("typed_event_outbox.push"));
        assert!(publisher.contains("typed_ui_outbox.push"));
        assert!(source.contains("mounted.publication_attempt = mounted.publication_attempt.saturating_add(1)"));
        assert!(source.contains("take_typed_operation_effect"));
        assert!(source.contains("take_typed_operation_event"));
        assert!(source.contains("take_typed_operation_ui_scope"));

        let reactor = include_str!("../../⚛️reactor/🦀️.rs");
        assert!(reactor.contains("output.typed_operation_result.as_ref()"));
        assert!(reactor.contains("page.renderer_exchange_bytes()"));
        assert!(reactor.contains("TypedOperationResultPage::renderer_ack_token"));
        assert!(reactor.contains("plugin_acknowledge_typed_operation_result(runtime, token)"));
        assert!(!reactor.contains("typed_operation_result: _"), "the old typed-result drop route must remain unreachable");
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
            let lane: TypedOperationResultLane = protocol::json::from_json_str(&serde_json::to_string(&row["name"]).unwrap()).expect("own lane decoder");
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
        let command_end = source[command_start..].find("async fn dispatch_typed_command(").map(|offset| command_start + offset).expect("typed command route");
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
