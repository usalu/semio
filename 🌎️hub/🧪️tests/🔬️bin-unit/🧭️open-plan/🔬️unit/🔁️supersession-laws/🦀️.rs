/// 🔁️ Laws of foreign history transitions, admitted supersessions and concurrent supersession convergence.

/// 👥️ The next server frame that is not a presence roster — rosters fan out independently of a law's commands.
async fn next_command_frame<S>(ws: &mut S, phase: &str) -> ServerFrame
where
    S: StreamExt<Item = Result<WsMessage, tokio_tungstenite::tungstenite::Error>> + Unpin,
{
    loop {
        let frame = next_server_frame_at(ws, phase).await;
        if !matches!(frame, ServerFrame::Presence { .. }) {
            return frame;
        }
    }
}

/// 🪪️ LAW (C13, hub 7800 p33): an undo belongs to its author at the hub's socket too — a `Revert` or `Reinstate` naming an
/// operation another actor committed is answered with the typed `history.foreign-transition` refusal, commits nothing and is
/// relayed to nobody (on p33 the hub persisted B's crafted revert of A's note edit and only the replicas' fold refused it);
/// the author's own undo is admitted and relayed. The replicas' fold refusal stays the second line.
#[test]
fn a_foreign_history_transition_is_refused_at_the_socket_and_never_relayed() {
    run_socket_test(|| async {
        let state = test_state().await;
        let token_a = seed_author_token(&state).await;
        let second = AuthSessionIssue {
            user_id: "seed".into(),
            identity_provider: "test-verifier".into(),
            identity_subject_digest: identity_subject_digest("test-verifier", "seed").expect("seed subject digest"),
            ttl_secs: 3_600,
            device_instance_id: "seed-device-b".into(),
            session_kind: AuthSessionKind::DevelopmentLocal,
            correlation_id: directory::os_identity::time_ordered_id(),
            peer_class: "test".into(),
        };
        let token_b = state.directory.issue_auth_session(&second).await.expect("second author session").capability.expose_once();
        announce_document_for_test(&state, STUDIO, "undo-a").await;
        let document = WireArtifactId("undo-a".into());
        let url = {
            let addr = spawn_server(state.clone()).await;
            format!("ws://{addr}/scopes/{STUDIO}%2Fundo-a/document/ws")
        };
        let receipt_a = issue_document_socket_grant_fixture(Path((STUDIO.to_string(), "undo-a".to_string())), bearer_headers(&token_a), State(state.clone())).await.expect("grant a").0;
        let (mut a, _) = connect_async(document_socket_request(&url, &token_a)).await.expect("socket a");
        a.send(client_binary(&socket_hello(), Lane::Command).await).await.expect("hello a");
        assert!(matches!(next_command_frame(&mut a, "a welcome").await, ServerFrame::Welcome { .. }));
        assert!(matches!(next_command_frame(&mut a, "a session").await, ServerFrame::Session { .. }));
        let mut edit = sample_envelope("a-edit", &document).await;
        edit.actor = ActorId(receipt_a.actor_id.clone());
        a.send(client_binary(&ClientFrame::Commands { batch_id: 1, envelopes: vec![edit.clone()] }, Lane::Command).await).await.expect("a edits");
        let accepted = |frame: &ServerFrame, batch: u64| matches!(frame, ServerFrame::Ack { batch_id, stages, .. } if *batch_id == batch && matches!(stages.last(), Some(AckStage::Applied { outcome }) if matches!(outcome.as_ref(), ApplyOutcome::Accepted)));
        let ack = next_command_frame(&mut a, "a edit ack").await;
        assert!(accepted(&ack, 1), "{ack:?}");
        assert!(matches!(next_command_frame(&mut a, "a edit relay").await, ServerFrame::Commands { .. }));
        let receipt_b = issue_document_socket_grant_fixture(Path((STUDIO.to_string(), "undo-a".to_string())), bearer_headers(&token_b), State(state.clone())).await.expect("grant b").0;
        assert_ne!(receipt_b.actor_id, receipt_a.actor_id, "two sessions are two actors");
        let (mut b, _) = connect_async(document_socket_request(&url, &token_b)).await.expect("socket b");
        b.send(client_binary(&socket_hello(), Lane::Command).await).await.expect("hello b");
        assert!(matches!(next_command_frame(&mut b, "b welcome").await, ServerFrame::Welcome { .. }));
        assert!(matches!(next_command_frame(&mut b, "b catch-up").await, ServerFrame::Commands { .. }));
        assert!(matches!(next_command_frame(&mut b, "b session").await, ServerFrame::Session { .. }));
        let frontier = || async { state.db.document(&db_artifact_id(&DocumentScope::new(STUDIO, "undo-a"))).await.expect("document handle").frontier().await.expect("frontier") };
        let before = frontier().await;
        for (batch, transition) in [(2, protocol::HistoryTransition::Revert { mutation_ids: vec![edit.mutation_id.clone()] }), (3, protocol::HistoryTransition::Reinstate { mutation_ids: vec![edit.mutation_id.clone()] })] {
            let crafted = protocol::history_transition_envelope(&transition, &document, &ActorId(receipt_b.actor_id.clone()), vec![edit.mutation_id.clone()], protocol::HybridLogicalTimestamp::new(batch, 0));
            b.send(client_binary(&ClientFrame::Commands { batch_id: batch, envelopes: vec![crafted] }, Lane::Command).await).await.expect("b crafts");
            match next_command_frame(&mut b, "crafted ack").await {
                ServerFrame::Ack { batch_id, stages, .. } if batch_id == batch => match stages.last() {
                    Some(AckStage::Applied { outcome }) => match outcome.as_ref() {
                        ApplyOutcome::Rejected { messages, .. } => {
                            let messages: serde_json::Value = serde_json::from_slice(messages).expect("rejection messages are one JSON MutationMessage array");
                            assert!(messages.as_array().expect("message array").iter().any(|message| message["code"] == "history.foreign-transition"), "{messages}");
                        }
                        other => panic!("a foreign {transition:?} must be refused: {other:?}"),
                    },
                    other => panic!("expected an applied stage: {other:?}"),
                },
                other => panic!("expected the crafted batch's ack: {other:?}"),
            }
        }
        assert_eq!(frontier().await.commit_seq, before.commit_seq, "a refused transition commits nothing");
        let own = protocol::history_transition_envelope(&protocol::HistoryTransition::Revert { mutation_ids: vec![edit.mutation_id.clone()] }, &document, &ActorId(receipt_a.actor_id.clone()), vec![edit.mutation_id.clone()], protocol::HybridLogicalTimestamp::new(9, 0));
        a.send(client_binary(&ClientFrame::Commands { batch_id: 4, envelopes: vec![own.clone()] }, Lane::Command).await).await.expect("a undoes");
        let ack = next_command_frame(&mut a, "own undo ack").await;
        assert!(accepted(&ack, 4), "the author's own undo is admitted: {ack:?}");
        assert!(matches!(next_command_frame(&mut b, "own undo relay").await, ServerFrame::Commands { envelopes, .. } if envelopes[0].mutation_id == own.mutation_id), "b's first relay after the refusals is a's own undo — the refused transitions were never relayed");
        a.close(None).await.expect("close a");
        b.close(None).await.expect("close b");
    });
}

/// ✏️ LAW (ticket 26/09/30 NON-DESTRUCTIVE-HISTORY-EDITING, `📋️design.md` §9.1): a `Supersede` has no ownership rule at the
/// socket — B's supersession of A's operation commits and is relayed to A — and a supersession the hub refuses (it names
/// an operation the log does not hold) is the typed `history.unknown-target`, commits nothing, is relayed to nobody and
/// leaves its author's socket live: no rebuild follows, its author retracts the refused step (`BackboneMessage::Retract`,
/// follow-up 4) and its next supersession commits on the same socket.
#[test]
fn a_supersession_is_admitted_whatever_its_author_and_its_refusal_leaves_the_socket_live() {
    run_socket_test(|| async {
        let state = test_state().await;
        let author_a = issue_test_session(&state, "supersede-a@example.com").await;
        let author_b = issue_test_session(&state, "supersede-b@example.com").await;
        upsert_member_for_test(&state, STUDIO, "supersede-a@example.com", DirectorySpaceRole::Author).await;
        upsert_member_for_test(&state, STUDIO, "supersede-b@example.com", DirectorySpaceRole::Author).await;
        let document_id = artifact_document_id_for_test("supersede-document");
        seed_genesis_for_document_for_test(&state, &author_a.token, STUDIO, &document_id, "supersede-document").await;
        let document = WireArtifactId(document_id.clone());
        let url = {
            let addr = spawn_server(state.clone()).await;
            format!("ws://{addr}/scopes/{STUDIO}%2F{document_id}/document/ws")
        };
        let open = |token: String| {
            let (url, state, document_id) = (url.clone(), state.clone(), document_id.clone());
            async move {
                let receipt = issue_document_socket_grant_fixture(Path((STUDIO.to_string(), document_id)), bearer_headers(&token), State(state)).await.expect("document socket grant").0;
                let (mut socket, _) = connect_async(document_socket_request(&url, &token)).await.expect("document socket");
                socket.send(client_binary(&socket_hello(), Lane::Command).await).await.expect("hello");
                while !matches!(next_command_frame(&mut socket, "until session").await, ServerFrame::Session { .. }) {}
                (receipt.actor_id, socket)
            }
        };
        let applied = |frame: &ServerFrame, batch: u64| match frame {
            ServerFrame::Ack { batch_id, stages, .. } if *batch_id == batch => match stages.last() {
                Some(AckStage::Applied { outcome }) => Some(outcome.as_ref().clone()),
                _ => None,
            },
            _ => None,
        };
        let (actor_a, mut a) = open(author_a.token.clone()).await;
        let mut edit = sample_envelope("supersede-a-edit", &document).await;
        edit.actor = ActorId(actor_a.clone());
        a.send(client_binary(&ClientFrame::Commands { batch_id: 1, envelopes: vec![edit.clone()] }, Lane::Command).await).await.expect("a edits");
        assert!(matches!(applied(&next_command_frame(&mut a, "a edit ack").await, 1), Some(ApplyOutcome::Accepted)));
        assert!(matches!(next_command_frame(&mut a, "a edit relay").await, ServerFrame::Commands { .. }));
        let (actor_b, mut b) = open(author_b.token.clone()).await;
        assert_ne!(actor_a, actor_b, "two sessions are two actors");
        let supersede = |targets: &[&protocol::MutationId], at: u64| {
            let transition = protocol::TransitionSupersede { scope: None, inputs: targets.iter().map(|target| protocol::SupersededInput { target: (*target).clone(), replacement: protocol::InputReplacement::Withdrawn }).collect() };
            let dependencies = transition.targets();
            let mut envelope = protocol::history_transition_envelope(&protocol::HistoryTransition::Supersede(transition), &document, &ActorId(actor_b.clone()), dependencies, protocol::HybridLogicalTimestamp::new(at, 0));
            envelope.observed = Some(edit.mutation_id.clone());
            envelope.target = vec!["value".into()];
            envelope
        };
        let frontier = || async { state.db.document(&db_artifact_id(&DocumentScope::new(STUDIO, document_id.as_str()))).await.expect("document handle").frontier().await.expect("frontier") };

        let foreign = supersede(&[&edit.mutation_id], 2);
        b.send(client_binary(&ClientFrame::Commands { batch_id: 2, envelopes: vec![foreign.clone()] }, Lane::Command).await).await.expect("b supersedes a's edit");
        assert!(matches!(applied(&next_command_frame(&mut b, "foreign supersede ack").await, 2), Some(ApplyOutcome::Accepted)), "a supersession of another author's operation is admitted");
        assert!(matches!(next_command_frame(&mut b, "foreign supersede own relay").await, ServerFrame::Commands { .. }));
        assert!(matches!(next_command_frame(&mut a, "foreign supersede relay").await, ServerFrame::Commands { envelopes, .. } if envelopes[0].mutation_id == foreign.mutation_id), "and relayed to the superseded operation's author");

        let before = frontier().await;
        let ghost = protocol::MutationId("supersede-ghost".into());
        b.send(client_binary(&ClientFrame::Commands { batch_id: 3, envelopes: vec![supersede(&[&ghost], 3)] }, Lane::Command).await).await.expect("b supersedes a ghost");
        match applied(&next_command_frame(&mut b, "ghost supersede ack").await, 3) {
            Some(ApplyOutcome::Rejected { messages, .. }) => {
                let messages: serde_json::Value = serde_json::from_slice(&messages).expect("rejection messages are one JSON MutationMessage array");
                assert!(messages.as_array().expect("message array").iter().any(|message| message["code"] == "history.unknown-target"), "{messages}");
            }
            other => panic!("a supersession of an operation the log does not hold is refused: {other:?}"),
        }
        assert_eq!(frontier().await.commit_seq, before.commit_seq, "a refused supersession commits nothing");
        let next = supersede(&[&edit.mutation_id], 4);
        b.send(client_binary(&ClientFrame::Commands { batch_id: 4, envelopes: vec![next.clone()] }, Lane::Command).await).await.expect("b supersedes again on the same socket");
        match next_command_frame(&mut b, "the frame after the refusal").await {
            frame @ ServerFrame::Ack { .. } => assert!(matches!(applied(&frame, 4), Some(ApplyOutcome::Accepted)), "the live socket commits the next supersession: {frame:?}"),
            other => panic!("a refusal is followed by no rebuild, only the next batch's answer: {other:?}"),
        }
        assert!(matches!(next_command_frame(&mut a, "next supersede relay").await, ServerFrame::Commands { envelopes, .. } if envelopes[0].mutation_id == next.mutation_id), "a's next relay is b's committed supersession — the refused one was never relayed");
        a.close(None).await.expect("close a");
        b.close(None).await.expect("close b");
    });
}

//#region ✏️ConcurrentSupersede
type SupersedeReplica = directory::os_store::ArtifactStore<semio_s_artifact_gis_gismap::GisMapSnapshot, semio_s_artifact_gis_gismap::GisMapMutation>;

/// 🧑‍🤝‍🧑️ One author of the concurrent-supersede law: a GIS Map replica authoring as the hub actor its document socket
/// speaks for — the actor a sync actor stamps — the socket, the events it knows and every batch the hub relayed to it.
struct SupersedeAuthor {
    actor: String,
    socket: tokio_tungstenite::WebSocketStream<tokio_tungstenite::MaybeTlsStream<tokio::net::TcpStream>>,
    replica: SupersedeReplica,
    known: std::collections::HashSet<String>,
    relayed: Vec<MutationEnvelope>,
    batch: u64,
}

impl SupersedeAuthor {
    /// 🔓️ Opens a document socket as `token`'s session and a replica of `document` on the GIS Map genesis pair.
    async fn open(state: &HubState, url: &str, document: &str, token: &str) -> Self {
        use semio_s_artifact_gis_gismap::{GisMapMutation, GisMapSnapshot, GIS_MAP_SCHEMA};
        let actor = issue_document_socket_grant_fixture(Path((STUDIO.to_string(), document.to_string())), bearer_headers(token), State(state.clone())).await.expect("document socket grant").0.actor_id;
        let (mut socket, _) = connect_async(document_socket_request(url, token)).await.expect("document socket");
        socket.send(client_binary(&socket_hello(), Lane::Command).await).await.expect("hello");
        while !matches!(next_command_frame(&mut socket, "until session").await, ServerFrame::Session { .. }) {}
        let pack = <GisMapSnapshot as pack::snapshot::ArtifactPack>::encode_pack(&GisMapSnapshot::default());
        let spr = directory::os_store::text_format::empty_document_spr(document, GIS_MAP_SCHEMA).await;
        let parsed = directory::os_store::text_format::parse_document_pack::<GisMapSnapshot, GisMapMutation>(&pack, &spr).await.expect("genesis pair parses");
        let mut replica = SupersedeReplica::new(parsed.into_envelope(), directory::ActorId(actor.clone())).await.expect("replica");
        replica.install_document_store_owners_exact(directory::os_store::document_store_owners::bounded_artifact_store_owners());
        assert_eq!(replica.local_actor_id(), &directory::ActorId(actor.clone()), "the replica authors as its socket's actor");
        Self { actor, socket, replica, known: std::collections::HashSet::new(), relayed: Vec::new(), batch: 0 }
    }

    /// 🖋️ Dispatches `command` on the replica and answers the events it authored, as its socket sends them.
    async fn author(&mut self, command: directory::os_store::store_schema::ArtifactCommand<semio_s_artifact_gis_gismap::GisMapMutation>) -> Vec<MutationEnvelope> {
        self.replica.dispatch(command).await.expect("the replica authors");
        let fresh: Vec<MutationEnvelope> = self.replica.event_log().expect("event log").into_iter().filter(|event| self.known.insert(event.mutation_id.0.clone())).collect();
        fresh.into_iter().map(|envelope| MutationEnvelope { actor: ActorId(self.actor.clone()), ..envelope }).collect()
    }

    /// 🛫️ Sends `envelopes` as one batch and answers its terminal outcome; relays read meanwhile are kept.
    async fn submit(&mut self, envelopes: &[MutationEnvelope]) -> ApplyOutcome {
        self.batch += 1;
        let batch = self.batch;
        self.socket.send(client_binary(&ClientFrame::Commands { batch_id: batch, envelopes: envelopes.to_vec() }, Lane::Command).await).await.expect("batch");
        loop {
            match next_command_frame(&mut self.socket, "concurrent supersede ack").await {
                ServerFrame::Ack { batch_id, stages, .. } if batch_id == batch => {
                    if let Some(AckStage::Applied { outcome }) = stages.last() {
                        return outcome.as_ref().clone();
                    }
                }
                ServerFrame::Commands { envelopes, .. } => self.relayed.extend(envelopes),
                _ => {}
            }
        }
    }

    /// 🛬️ Reads until the hub relayed every one of `events`.
    async fn await_relay(&mut self, events: &[MutationEnvelope]) {
        while !events.iter().all(|event| self.relayed.iter().any(|relayed| relayed.mutation_id == event.mutation_id)) {
            if let ServerFrame::Commands { envelopes, .. } = next_command_frame(&mut self.socket, "concurrent supersede relay").await {
                self.relayed.extend(envelopes);
            }
        }
    }

    /// 🔄️ Ingests every relayed event this replica does not hold yet; its own relays are the events it authored.
    async fn ingest_relays(&mut self) {
        for envelope in std::mem::take(&mut self.relayed) {
            if self.known.insert(envelope.mutation_id.0.clone()) {
                self.replica.ingest_remote(envelope).await.expect("the replica folds a relayed event");
            }
        }
    }

    /// 🧽️ Closes the replica through its bounded retirement.
    fn close(mut self) {
        while !self.replica.close_owned_terminal_is_empty() {
            let demand = self.replica.close_owned_demands(0).expect("the store quotes its next close turn");
            let grant = directory::os_store::RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: demand.copy_bytes, maximum_capacity_bytes: demand.capacity_bytes, maximum_release_bytes: demand.release_bytes, maximum_depth: demand.depth };
            self.replica.close_owned_step(grant).expect("replica closes");
        }
    }
}

/// 📍️ A GIS Map position `id` at `index`, longitude `lon`.
fn supersede_position(index: usize, id: &str, lon: f64) -> semio_s_artifact_gis_gismap::GisMapMutation {
    use semio_s_artifact_gis_gismap::mutations::create_position::CreatePosition;
    let data = directory::DslValue::object([("lon".into(), directory::DslValue::float(lon)), ("lat".into(), directory::DslValue::float(47.0))]);
    semio_s_artifact_gis_gismap::GisMapMutation::CreatePosition(CreatePosition { index, item: semio_s_artifact_gis_gismap::MapFeature { id: id.into(), data } })
}

/// 🤝️ Both replicas hold the same fold: snapshot, effective supersessions, content revision, per-mutation outcomes,
/// alternatives and the line they stand on.
fn assert_supersede_convergence(a: &SupersedeReplica, b: &SupersedeReplica, round: &str) {
    let alternatives = |replica: &SupersedeReplica| replica.envelope().vcs.alternatives.iter().map(|alternative| (alternative.id.clone(), alternative.name.clone(), alternative.checkpoint_ids.clone())).collect::<Vec<_>>();
    assert_eq!(a.snapshot().expect("snapshot"), b.snapshot().expect("snapshot"), "{round}: the same document");
    assert_eq!(a.supersessions(), b.supersessions(), "{round}: the same effective supersessions");
    assert_eq!(a.content_revision(), b.content_revision(), "{round}: the same content revision");
    assert_eq!(a.mutation_outcomes().expect("outcomes"), b.mutation_outcomes().expect("outcomes"), "{round}: the same outcomes");
    assert_eq!((a.active_line_id(), alternatives(a)), (b.active_line_id(), alternatives(b)), "{round}: the same alternatives and line");
}

/// 🏁️ The transition of `events` the fold keeps for a contested operation: the last by `(hlc, id)`.
fn last_supersession(events: &[&MutationEnvelope]) -> String {
    events.iter().max_by(|left, right| (left.timestamp.cmp_key(), &left.mutation_id.0).cmp(&(right.timestamp.cmp_key(), &right.mutation_id.0))).expect("contested events").mutation_id.0.clone()
}

/// 🌗️ LAW (ticket 26/09/30 NON-DESTRUCTIVE-HISTORY-EDITING follow-up 3): two authors supersede concurrently through the
/// hub — each authored without observing the other — and both replicas converge on one fold: the same document, effective
/// supersessions, content revision, outcomes and alternatives. Under `Normal` the hub takes both: the same operation
/// replaced twice, different operations, a withdrawal against a replacement of one operation, and a trunk-scoped
/// supersession against a new alternative's scoped one; a contested operation folds the last supersession by `(hlc,
/// id)`, and the trunk-scoped one takes effect exactly when its author checks the trunk out again. Under `Vigilant` the
/// later concurrent supersession of the same operation is refused `mutation.clamped`, its author retracts it
/// (`BackboneMessage::Retract`, audit F-m8) and converges with the other.
#[test]
fn concurrent_supersessions_through_the_hub_converge_both_replicas() {
    run_socket_test(|| async {
        use directory::os_store::{store_schema::ArtifactCommand, store_schema::SupersedeInput};
        let supersede = |target: &MutationEnvelope, replacement: Option<semio_s_artifact_gis_gismap::GisMapMutation>| SupersedeInput { target: target.mutation_id.clone(), replacement };
        for policy in [protocol::MergePolicy::Normal, protocol::MergePolicy::Vigilant] {
            let mut state = test_state().await;
            state.merge_policy = policy;
            let document = format!("concurrent-supersede-{}", format!("{policy:?}").to_lowercase());
            announce_document_for_test(&state, STUDIO, &document).await;
            let (seed_token, alice_token, bob_token) = (seed_author_token(&state).await, seed_author_token(&state).await, seed_author_token(&state).await);
            let url = format!("ws://{}/scopes/{STUDIO}%2F{document}/document/ws", spawn_server(state.clone()).await);
            let mut seed = SupersedeAuthor::open(&state, &url, &document, &seed_token).await;
            let mut alice = SupersedeAuthor::open(&state, &url, &document, &alice_token).await;
            let mut bob = SupersedeAuthor::open(&state, &url, &document, &bob_token).await;
            let mut edits = Vec::new();
            for (index, id) in ["p0", "p1", "p2", "p3"].into_iter().enumerate() {
                edits.extend(seed.author(ArtifactCommand::Apply { mutations: vec![supersede_position(index, id, 7.0 + index as f64)], transaction: None }).await);
            }
            assert!(matches!(seed.submit(&edits).await, ApplyOutcome::Accepted), "{policy:?}: the seed edits commit");
            for author in [&mut alice, &mut bob] {
                author.await_relay(&edits).await;
                author.ingest_relays().await;
            }
            assert_supersede_convergence(&alice.replica, &bob.replica, "seed");
            let ops: Vec<MutationEnvelope> = edits.clone();
            if policy == protocol::MergePolicy::Vigilant {
                let first = alice.author(ArtifactCommand::Supersede { scope: None, inputs: vec![supersede(&ops[0], Some(supersede_position(0, "p0", 10.0)))] }).await;
                let second = bob.author(ArtifactCommand::Supersede { scope: None, inputs: vec![supersede(&ops[0], Some(supersede_position(0, "p0", 20.0)))] }).await;
                assert!(matches!(alice.submit(&first).await, ApplyOutcome::Accepted), "vigilant: the first supersession commits");
                bob.await_relay(&first).await;
                match bob.submit(&second).await {
                    ApplyOutcome::Rejected { messages, .. } => {
                        let messages: serde_json::Value = serde_json::from_slice(&messages).expect("rejection messages");
                        assert!(messages.as_array().expect("messages").iter().any(|message| message["code"] == "mutation.clamped"), "vigilant: {messages}");
                    }
                    other => panic!("vigilant refuses the concurrent supersession of the same operation: {other:?}"),
                }
                let (channel, remote) = directory::os_store::backbone::ChannelBackbone::pair("concurrent-supersede-retract").await;
                bob.replica.attach_backbone(directory::os_store::backbone::Backbones::Channel(channel)).await.expect("attach");
                remote.push(directory::os_store::backbone::BackboneMessage::Retract { mutation_ids: second.iter().map(|event| event.mutation_id.0.clone()).collect() }).await.expect("retract");
                bob.replica.tick().await.expect("the refused supersession retracts");
                alice.await_relay(&first).await;
                for author in [&mut alice, &mut bob] {
                    author.ingest_relays().await;
                }
                assert_supersede_convergence(&alice.replica, &bob.replica, "vigilant");
                assert_eq!(bob.replica.supersessions().get(&ops[0].mutation_id).map(|supersession| supersession.transition_id.clone()), Some(first[0].mutation_id.0.clone()), "vigilant: the committed supersession is the one in effect");
            } else {
                let rounds: [(&str, ArtifactCommand<semio_s_artifact_gis_gismap::GisMapMutation>, ArtifactCommand<semio_s_artifact_gis_gismap::GisMapMutation>); 4] = [
                    ("same operation", ArtifactCommand::Supersede { scope: None, inputs: vec![supersede(&ops[0], Some(supersede_position(0, "p0", 10.0)))] }, ArtifactCommand::Supersede { scope: None, inputs: vec![supersede(&ops[0], Some(supersede_position(0, "p0", 20.0)))] }),
                    ("different operations", ArtifactCommand::Supersede { scope: None, inputs: vec![supersede(&ops[1], None)] }, ArtifactCommand::Supersede { scope: None, inputs: vec![supersede(&ops[2], Some(supersede_position(2, "p2", 22.0)))] }),
                    ("withdraw against replace", ArtifactCommand::Supersede { scope: None, inputs: vec![supersede(&ops[3], None)] }, ArtifactCommand::Supersede { scope: None, inputs: vec![supersede(&ops[3], Some(supersede_position(3, "p3", 33.0)))] }),
                    ("trunk against alternative", ArtifactCommand::Supersede { scope: Some(alice.replica.trunk_alternative_id()), inputs: vec![supersede(&ops[0], Some(supersede_position(0, "p0", 40.0)))] }, ArtifactCommand::CreateAlternativeWithSupersede { name: "b".into(), inputs: vec![supersede(&ops[1], Some(supersede_position(1, "p1", 41.0)))] }),
                ];
                let mut trunk_scoped = Vec::new();
                for (round, a_command, b_command) in rounds {
                    let a_events = alice.author(a_command).await;
                    let b_events = bob.author(b_command).await;
                    assert!(matches!(alice.submit(&a_events).await, ApplyOutcome::Accepted), "{round}: normal takes the first");
                    assert!(matches!(bob.submit(&b_events).await, ApplyOutcome::Accepted), "{round}: normal takes the concurrent second and reports it");
                    let all: Vec<MutationEnvelope> = a_events.iter().chain(b_events.iter()).cloned().collect();
                    for author in [&mut alice, &mut bob] {
                        author.await_relay(&all).await;
                        author.ingest_relays().await;
                    }
                    assert_supersede_convergence(&alice.replica, &bob.replica, round);
                    let effective = |op: &MutationEnvelope| alice.replica.supersessions().get(&op.mutation_id).map(|supersession| supersession.transition_id.clone());
                    match round {
                        "same operation" | "withdraw against replace" => {
                            let contested = if round == "same operation" { &ops[0] } else { &ops[3] };
                            assert_eq!(effective(contested), Some(last_supersession(&[&a_events[0], &b_events[0]])), "{round}: the last supersession by (hlc, id) is in effect");
                        }
                        "different operations" => assert_eq!((effective(&ops[1]), effective(&ops[2])), (Some(a_events[0].mutation_id.0.clone()), Some(b_events[0].mutation_id.0.clone())), "{round}: both are in effect"),
                        _ => {
                            let branch = b_events.iter().rev().find(|event| matches!(protocol::history_transition_from_envelope(event), Ok(Some(protocol::HistoryTransition::Supersede(_))))).expect("the scoped supersession");
                            assert_eq!(effective(&ops[1]), Some(branch.mutation_id.0.clone()), "{round}: both stand on the new alternative, its scoped supersession in effect");
                            assert_ne!(effective(&ops[0]), Some(a_events[0].mutation_id.0.clone()), "{round}: the trunk-scoped supersession waits on the trunk");
                            trunk_scoped = a_events;
                        }
                    }
                }
                let trunk = alice.replica.trunk_alternative_id();
                let checkout = alice.author(ArtifactCommand::SwitchAlternative { alternative_id: trunk }).await;
                assert!(matches!(alice.submit(&checkout).await, ApplyOutcome::Accepted), "the trunk checkout commits");
                for author in [&mut alice, &mut bob] {
                    author.await_relay(&checkout).await;
                    author.ingest_relays().await;
                }
                assert_supersede_convergence(&alice.replica, &bob.replica, "back on the trunk");
                assert_eq!(bob.replica.supersessions().get(&ops[0].mutation_id).map(|supersession| supersession.transition_id.clone()), Some(trunk_scoped[0].mutation_id.0.clone()), "on the trunk its scoped supersession is in effect");
            }
            for author in [seed, alice, bob] {
                author.close();
            }
        }
    });
}
//#endregion ✏️ConcurrentSupersede
