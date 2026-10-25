//! 🧠️ The shared server state with its command lane, and the blob and command routes over it.

use super::*;

//#region 🔖️State
/// ☁️ Everything a handler may reach, for one [`ServerInstance`]. Cloneable and cheap: every field
/// is an [`Arc`], so the whole value is a bundle of handles rather than a bundle of data.
pub struct ServerState<I: ServerInstance> {
    /// 🏛️ The command bus, serialized behind a mutex because a turn is by definition one at a time.
    pub authority: Arc<Mutex<ServerAuthority<I>>>,
    /// 🗄️ The authority store the bus commits into, shared with it: history is read and the outbox
    /// drained through this handle, so neither ever waits for the turns queued on the bus.
    pub store: SharedStore<I::AuthorityStore>,
    /// 🧵️ The cross-actor workflows this instance reacts with, over the outbox the bus commits into.
    /// Held here rather than by whoever calls [`ServerBuilder::build`] because the outbox and the
    /// workflows that drain it are two halves of one exactly-once guarantee: a runner living
    /// somewhere else is a runner that can be forgotten, and a forgotten runner is a queue that
    /// grows forever while every event looks delivered.
    pub sagas: Arc<Mutex<ServerSagas<I>>>,
    /// 🔭️ The rebuildable read models every query answers from, shared: queries read them side by
    /// side, and only whoever folds events into them takes them alone.
    pub projections: SharedStore<I::ProjectionStore>,
    /// 🧱️ Content-addressed bytes.
    pub blobs: Arc<Mutex<I::BlobStore>>,
    /// 🎫️ Live authentication state.
    pub sessions: Arc<Mutex<I::SessionStore>>,
    /// ⚖️ Roles as data. A standard lock rather than an async one because the command bus's own
    /// admission hook is synchronous and must consult it inside a turn.
    pub policy: Arc<RwLock<PolicyEngine>>,
    /// 🪜️ The authentication ladder, fixed at build time.
    pub resolvers: Arc<ResolverChain<I::Resolvers>>,
    /// 🚪️ The gate in front of the administration plane.
    pub admin: Arc<AdminGate>,
    /// 📡️ Lane fan-out for both the durable and the ephemeral lane.
    pub fanout: Arc<Fanout>,
    /// 👥️ Who is present where, and which colour they hold.
    pub presence: Arc<Presence>,
    /// 🦵️ Per-session close signals.
    pub kicks: Arc<KickMap>,
    /// 🧩️ The static apps this instance hosts.
    pub apps: Arc<AppRegistry>,
    /// ❓️ The registered read handlers, keyed by query kind.
    pub queries: Arc<ShardedMap<String, Arc<I::Queries>>>,
    /// 📄️ The replication engine, when the instance supplied one.
    pub documents: Option<Arc<I::Documents>>,
    /// 🏗️ The deployment shape this instance's storage was opened in — the profile itself rather
    /// than a path, because [`StorageProfile::Ephemeral`] owns no directory and a handler that
    /// asked for one would have been handed a fabricated empty path.
    pub profile: Arc<StorageProfile>,
    /// 🪝️ The modules this instance was built from, asked at runtime by the hooks that need them
    /// (presence admission).
    pub modules: Arc<Vec<I::Modules>>,
    /// 🪪️ The presence rooms of the presence socket.
    pub presence_rooms: Arc<PresenceRooms>,
    /// ⏱️ The presence socket's knobs.
    pub presence_settings: Arc<PresenceSettings>,
    /// 🌍️ Which browser origins may open a presence socket; `None` admits every origin.
    pub origin_admission: Option<OriginAdmission>,
    /// 🚦️ The bookkeeping of this instance's edge limits: rates per client address, sockets,
    /// requests in flight.
    pub throttle: Arc<Throttle>,
    /// 🧭️ Where a client's address comes from.
    pub addressing: ClientAddressing,
    /// 🛤️ The command lane: what is queued for the bus, and whether a runner is on it.
    pub(super) lane: Arc<CommandLane>,
    /// 🕰️ The instance's hybrid logical clock, advanced once per stamped command.
    pub(super) clock: Arc<StdMutex<HybridLogicalClock>>,
}

impl<I: ServerInstance> Clone for ServerState<I> {
    /// 🧬️ Handle-by-handle, never field-by-field through `derive(Clone)`: the derive would demand
    /// `I: Clone` of the instance MARKER type, which is not a value anybody clones.
    fn clone(&self) -> Self {
        Self {
            authority: Arc::clone(&self.authority),
            store: Arc::clone(&self.store),
            sagas: Arc::clone(&self.sagas),
            projections: Arc::clone(&self.projections),
            blobs: Arc::clone(&self.blobs),
            sessions: Arc::clone(&self.sessions),
            policy: Arc::clone(&self.policy),
            resolvers: Arc::clone(&self.resolvers),
            admin: Arc::clone(&self.admin),
            fanout: Arc::clone(&self.fanout),
            presence: Arc::clone(&self.presence),
            kicks: Arc::clone(&self.kicks),
            apps: Arc::clone(&self.apps),
            queries: Arc::clone(&self.queries),
            documents: self.documents.clone(),
            profile: Arc::clone(&self.profile),
            modules: Arc::clone(&self.modules),
            presence_rooms: Arc::clone(&self.presence_rooms),
            presence_settings: Arc::clone(&self.presence_settings),
            origin_admission: self.origin_admission.clone(),
            throttle: Arc::clone(&self.throttle),
            addressing: self.addressing,
            lane: Arc::clone(&self.lane),
            clock: Arc::clone(&self.clock),
        }
    }
}

impl<I: ServerInstance> ServerState<I> {
    /// ⏰️ The next clock reading: wall-clock milliseconds, with the counter breaking ties so two
    /// commands stamped inside the same millisecond still order.
    pub fn now(&self) -> HybridLogicalClock {
        let millis = SystemTime::now().duration_since(UNIX_EPOCH).map_or(0, |elapsed| elapsed.as_millis() as u64);
        let mut clock = self.clock.lock().unwrap_or_else(|poisoned| poisoned.into_inner());
        *clock = if millis > clock.millis { HybridLogicalClock { millis, counter: 0 } } else { HybridLogicalClock { millis: clock.millis, counter: clock.counter.saturating_add(1) } };
        *clock
    }

    /// 🔏️ Evaluate one policy question, turning a denial into [`ServerError::Forbidden`]. The
    /// engine's reason travels in the error for whoever handles it in process; the caller of a route
    /// is told `forbidden` and nothing of what was missing.
    pub fn authorize(&self, request: &PolicyRequest) -> Result<(), ServerError> {
        let engine = self.policy.read().map_err(|_| ServerError::Internal("policy engine poisoned".to_string()))?;
        match engine.evaluate(request) {
            PolicyDecision::Allow => Ok(()),
            PolicyDecision::Deny { reason } => Err(ServerError::Forbidden(reason)),
        }
    }

    /// 🙋️ Who this caller is, according to the ladder.
    pub async fn identify(&self, headers: &HeaderMap, peer: Option<SocketAddr>) -> Resolved {
        self.resolvers.resolve(&credential(headers, peer)).await
    }

    /// 🏷️ The address this caller is throttled as.
    pub fn client(&self, headers: &HeaderMap, peer: Option<SocketAddr>) -> ClientKey {
        client_key(self.addressing, headers, peer)
    }

    /// 🔌️ Count one more open socket of this caller until the permit is dropped, or refuse it at the
    /// caller's or the instance's socket cap.
    pub fn open_socket(&self, headers: &HeaderMap, peer: Option<SocketAddr>) -> Result<SocketPermit, ServerError> {
        Ok(self.throttle.open_socket(self.client(headers, peer), std::time::Instant::now())?)
    }

    /// 🚧️ What [`throttle_middleware`] needs of this instance.
    pub fn edge(&self) -> Edge {
        Edge { throttle: Arc::clone(&self.throttle), addressing: self.addressing }
    }

    /// 📜️ Every durable event of `actor` after `since`, read from the shared store: the bus and the
    /// turns queued on it are never taken.
    pub async fn replay_events(&self, actor: &ActorKey, since: u64) -> Result<Vec<EventRecord>, ServerError> {
        Ok(self.store.read().await.events_since(actor, since).await?)
    }

    /// 🚰️ Hand up to `limit` committed outbox rows to this instance's workflows and run every
    /// follow-up command as its own turn, returning what each turn answered.
    ///
    /// The two halves are deliberately one call. A drain that only *returned* commands would leave
    /// the rows acknowledged while their consequences sat in a `Vec` the caller might drop, which is
    /// precisely the "state changed but the world was never told" the transactional outbox exists to
    /// rule out. The rows are read and acknowledged through the shared store, never through the bus,
    /// and both locks are released before the first follow-up is submitted, because a turn takes the
    /// store itself.
    ///
    /// Exactly-once is a property of the *queue*, not of this call: a row leaves `pending` only once
    /// it is marked delivered, so a restart between two drains re-delivers nothing, and a crash
    /// between mapping and marking re-delivers a row whose follow-up command carries an
    /// [`IdempotencyKey`](crate::contract::IdempotencyKey) and is deduplicated by the bus.
    pub async fn drain_sagas(&self, limit: usize) -> Vec<CommandOutcome> {
        let commands = {
            let mut sagas = self.sagas.lock().await;
            let mut store = self.store.write().await;
            sagas.drain_outbox(&mut *store, limit).await
        };
        self.submit_all(commands).await
    }

    /// 🧮️ How many events and outbox rows the command lane has committed since this instance was
    /// built. A command answered through the lane is counted before it is answered, so whoever
    /// relays the outbox or folds read models on behalf of a request compares these with what it
    /// last relayed and folded and asks the store only when they moved.
    pub fn committed(&self) -> Committed {
        Committed { events: self.lane.events.load(Ordering::Acquire), outbox: self.lane.outbox.load(Ordering::Acquire) }
    }

    /// 📨️ Run one command through the command lane and answer its outcome.
    pub async fn submit(&self, envelope: CommandEnvelope) -> CommandOutcome {
        let lost = unanswered(&envelope);
        self.submit_all(vec![envelope]).await.pop().unwrap_or(lost)
    }

    /// 📦️ Run commands through the command lane, in order, and answer each one's outcome.
    ///
    /// The lane is how every command of this instance reaches the bus: commands are queued, and
    /// one runner at a time takes whatever is queued — up to [`COMMAND_BATCH`] — stamps each with
    /// the instance clock, runs them as one batch of turns ([`CommandBus::submit_batch`]) and
    /// publishes the accepted events on their durable lanes. A burst of commands therefore becomes
    /// a few batches and a few durable writes, while a lone command is a batch of one and waits for
    /// nobody. The runner is a task of its own: a caller that stops waiting (a client that hung up)
    /// cancels its wait, never a turn or a commit half-way.
    pub async fn submit_all(&self, envelopes: Vec<CommandEnvelope>) -> Vec<CommandOutcome> {
        let mut waiting = Vec::with_capacity(envelopes.len());
        {
            let mut queue = self.lane.queue.lock().unwrap_or_else(|poisoned| poisoned.into_inner());
            for envelope in envelopes {
                let (answer, answered) = oneshot::channel();
                waiting.push((answered, unanswered(&envelope)));
                queue.push_back((envelope, answer));
            }
        }
        self.run_lane();
        let mut outcomes = Vec::with_capacity(waiting.len());
        for (answered, lost) in waiting {
            outcomes.push(answered.await.unwrap_or(lost));
        }
        outcomes
    }

    /// 🏃️ Start the lane's runner unless one is running; it ends once the queue is empty.
    fn run_lane(&self) {
        if self.lane.running.swap(true, Ordering::AcqRel) {
            return;
        }
        let state = self.clone();
        tokio::spawn(async move {
            loop {
                let batch: Vec<Queued> = {
                    let mut queue = state.lane.queue.lock().unwrap_or_else(|poisoned| poisoned.into_inner());
                    let take = queue.len().min(COMMAND_BATCH);
                    queue.drain(..take).collect()
                };
                if batch.is_empty() {
                    state.lane.running.store(false, Ordering::Release);
                    let queued = !state.lane.queue.lock().unwrap_or_else(|poisoned| poisoned.into_inner()).is_empty();
                    if queued && !state.lane.running.swap(true, Ordering::AcqRel) {
                        continue;
                    }
                    return;
                }
                let (envelopes, answers): (Vec<CommandEnvelope>, Vec<oneshot::Sender<CommandOutcome>>) = batch.into_iter().unzip();
                let stamped = envelopes.into_iter().map(|envelope| (envelope, state.now())).collect();
                let (outcomes, committed) = {
                    let mut bus = state.authority.lock().await;
                    (bus.submit_batch(stamped).await, bus.committed())
                };
                state.lane.events.store(committed.events, Ordering::Release);
                state.lane.outbox.store(committed.outbox, Ordering::Release);
                for (outcome, answer) in outcomes.into_iter().zip(answers) {
                    if let CommandOutcome::Accepted { events, .. } = &outcome {
                        for event in events {
                            if let Ok(bytes) = serde_json::to_vec(event) {
                                state.fanout.publish(&stream_lane(&event.stream), bytes);
                            }
                        }
                    }
                    let _ = answer.send(outcome);
                }
            }
        });
    }
}

/// 📮️ The most queued commands one batch of the command lane takes.
pub const COMMAND_BATCH: usize = 64;

/// 📬️ One queued command and where its outcome is answered.
type Queued = (CommandEnvelope, oneshot::Sender<CommandOutcome>);

/// 🚌️ The command lane's queue, whether a runner is draining it, and how many events and outbox
/// rows the bus had committed when the runner last answered.
#[derive(Default)]
pub(super) struct CommandLane {
    queue: StdMutex<VecDeque<Queued>>,
    running: AtomicBool,
    pub(super) events: AtomicU64,
    pub(super) outbox: AtomicU64,
}

/// 🕳️ The outcome of a command whose runner went away before answering it: retryable, under the
/// same idempotency key.
fn unanswered(envelope: &CommandEnvelope) -> CommandOutcome {
    let receipt = CommandReceipt { command_id: envelope.command_id.clone(), actor: envelope.target.clone(), revision: Revision(0), accepted_at: HybridLogicalClock::default() };
    CommandOutcome::Rejected { receipt, reason: Rejection::ActorUnavailable { detail: UNAVAILABLE.to_string() }, notices: Vec::new() }
}
//#endregion 🔖️State

//#region 🔖️Blob
/// #️⃣ Decode a 64-hex-character blob address, refusing anything else rather than panicking.
pub fn parse_content_hash(hex: &str) -> Option<protocol::codec::ids::ContentHash> {
    if hex.len() != 64 {
        return None;
    }
    let mut bytes = [0u8; 32];
    let raw = hex.as_bytes();
    for (index, slot) in bytes.iter_mut().enumerate() {
        let pair = std::str::from_utf8(&raw[index * 2..index * 2 + 2]).ok()?;
        *slot = u8::from_str_radix(pair, 16).ok()?;
    }
    Some(protocol::codec::ids::ContentHash(bytes))
}

/// 🧾️ What a successful upload answers with.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BlobReceipt {
    /// ✅️ The verified content address.
    pub hash: String,
    /// 📏️ How many bytes are stored under it.
    pub size: usize,
}

/// 💾️ Store bytes at a client-supplied content address. The address is re-derived from the bytes
/// and a mismatch is a [`ServerError::Conflict`]: the caller asked to bind an address to content
/// that does not hash to it, which is the one thing a content-addressed store may never do.
pub async fn put_blob<I: ServerInstance>(Path(hash): Path<String>, headers: HeaderMap, ConnectInfo(peer): ConnectInfo<SocketAddr>, State(state): State<ServerState<I>>, body: Bytes) -> Result<Json<BlobReceipt>, ServerError> {
    let resolved = state.identify(&headers, Some(peer)).await;
    state.authorize(&PolicyRequest { point: PolicyPoint::BlobWrite, principal: resolved.principal, scope: None, resource: hash.clone(), action: "write".to_string() })?;
    let addressed = parse_content_hash(&hash).ok_or_else(|| ServerError::BadRequest("blob address is not 64 hex characters".to_string()))?;
    let computed = content_hash(&body);
    if computed != addressed {
        return Err(ServerError::Conflict(format!("blob address {hash} does not match the content hash {computed} of the uploaded bytes")));
    }
    state.blobs.lock().await.put(computed, &body).await?;
    Ok(Json(BlobReceipt { hash, size: body.len() }))
}

/// ⏪️ Read bytes back by address.
pub async fn get_blob<I: ServerInstance>(Path(hash): Path<String>, headers: HeaderMap, ConnectInfo(peer): ConnectInfo<SocketAddr>, State(state): State<ServerState<I>>) -> Result<Response, ServerError> {
    let resolved = state.identify(&headers, Some(peer)).await;
    state.authorize(&PolicyRequest { point: PolicyPoint::BlobRead, principal: resolved.principal, scope: None, resource: hash.clone(), action: "read".to_string() })?;
    let addressed = parse_content_hash(&hash).ok_or_else(|| ServerError::BadRequest("blob address is not 64 hex characters".to_string()))?;
    let bytes = state.blobs.lock().await.get(&addressed).await.ok_or_else(|| ServerError::NotFound(format!("no blob at {hash}")))?;
    Ok(([(header::CONTENT_TYPE, "application/octet-stream")], bytes).into_response())
}

/// 📥️ The cheap half of an upload negotiation: does this address already hold bytes.
pub async fn head_blob<I: ServerInstance>(Path(hash): Path<String>, headers: HeaderMap, ConnectInfo(peer): ConnectInfo<SocketAddr>, State(state): State<ServerState<I>>) -> StatusCode {
    let resolved = state.identify(&headers, Some(peer)).await;
    let request = PolicyRequest { point: PolicyPoint::BlobRead, principal: resolved.principal, scope: None, resource: hash.clone(), action: "read".to_string() };
    if let Err(error) = state.authorize(&request) {
        return error.status();
    }
    let Some(addressed) = parse_content_hash(&hash) else { return StatusCode::BAD_REQUEST };
    if state.blobs.lock().await.has(&addressed).await {
        StatusCode::OK
    } else {
        StatusCode::NOT_FOUND
    }
}
//#endregion 🔖️Blob

//#region 🔖️Command
/// ✉️ Submit one command. The envelope's principal is overwritten with the resolved one before the
/// turn runs — a client may address a command, it may never assert who is sending it. The command
/// lane ([`ServerState::submit`]) publishes accepted events onto the actor's durable lane, so live
/// subscribers see them without polling.
///
/// A command a module classes under a named allowance ([`ServerModule::command_allowance`]) spends
/// a token of its caller's address first — `429` naming the allowance when there is none — and is
/// handed the token back once its outcome shows that it kept nothing ([`kept`]). The token is spent
/// before the turn and held by nobody: a caller that hangs up while its command commits has paid.
pub async fn post_command<I: ServerInstance>(headers: HeaderMap, ConnectInfo(peer): ConnectInfo<SocketAddr>, State(state): State<ServerState<I>>, WireJson(envelope): WireJson<CommandEnvelope>) -> Result<Json<CommandOutcome>, ServerError> {
    let resolved = state.identify(&headers, Some(peer)).await;
    let mut envelope = envelope;
    envelope.principal = resolved.principal;
    envelope.session = resolved.session;
    envelope.device = resolved.device;
    let Some(allowance) = state.modules.iter().find_map(|module| module.command_allowance(&envelope)) else {
        return Ok(Json(state.submit(envelope).await));
    };
    let client = state.client(&headers, Some(peer));
    if let Err(Refusal::Throttled { retry_after } | Refusal::Overloaded { retry_after }) = state.throttle.spend(client, allowance, std::time::Instant::now()) {
        return Err(ServerError::Throttled { wait: retry_after, allowance: Some(allowance) });
    }
    let outcome = state.submit(envelope).await;
    if !kept(&outcome) {
        state.throttle.refund(client, allowance, std::time::Instant::now());
    }
    Ok(Json(outcome))
}

/// 🧺️ Whether a command made its instance keep something: it committed an event, or was deferred
/// to a process that may. A rejection and the replay of a command already answered kept nothing.
pub fn kept(outcome: &CommandOutcome) -> bool {
    match outcome {
        CommandOutcome::Accepted { events, .. } | CommandOutcome::Transformed { canonical_events: events, .. } => !events.is_empty(),
        CommandOutcome::Pending { .. } => true,
        CommandOutcome::Rejected { .. } => false,
    }
}

/// 🔎️ Answer one query from the projections, after checking the caller may read it.
pub async fn post_query<I: ServerInstance>(headers: HeaderMap, ConnectInfo(peer): ConnectInfo<SocketAddr>, State(state): State<ServerState<I>>, WireJson(envelope): WireJson<QueryEnvelope>) -> Result<Json<QueryResult>, ServerError> {
    let resolved = state.identify(&headers, Some(peer)).await;
    let mut envelope = envelope;
    envelope.principal = resolved.principal;
    state.authorize(&PolicyRequest { point: PolicyPoint::QueryAccess, principal: envelope.principal.clone(), scope: Some(envelope.scope.clone()), resource: envelope.kind.clone(), action: "read".to_string() })?;
    let handler = state.queries.get_cloned(&envelope.kind).ok_or_else(|| ServerError::NotFound(format!("no handler for query kind '{}'", envelope.kind)))?;
    let projections = state.projections.read().await;
    Ok(Json(handler.handle(&envelope, &*projections).await?))
}

/// 💨️ Publish one ephemeral frame onto its scope's lossy lane. Nothing is persisted and nothing is
/// replayed — a subscriber that was not listening simply missed it.
pub async fn post_ephemeral<I: ServerInstance>(headers: HeaderMap, ConnectInfo(peer): ConnectInfo<SocketAddr>, State(state): State<ServerState<I>>, WireJson(frame): WireJson<EphemeralFrame>) -> Result<Json<usize>, ServerError> {
    let resolved = state.identify(&headers, Some(peer)).await;
    let mut frame = frame;
    frame.principal = resolved.principal;
    state.authorize(&PolicyRequest { point: PolicyPoint::Subscription, principal: frame.principal.clone(), scope: Some(frame.scope.clone()), resource: frame.kind.clone(), action: "publish".to_string() })?;
    let lane = ephemeral_lane(&frame.scope);
    let bytes = serde_json::to_vec(&frame).map_err(|error| ServerError::BadRequest(error.to_string()))?;
    Ok(Json(state.fanout.publish(&lane, bytes)))
}
//#endregion 🔖️Command
