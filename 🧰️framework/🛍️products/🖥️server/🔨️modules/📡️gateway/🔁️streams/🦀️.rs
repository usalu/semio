//! 🔁️ The event stream and the document stream sockets.

use super::*;

//#region 🔖️EventStream
/// 🪡️ The replay/live seam. Everything with a sequence at or below the high-water mark has already
/// been delivered, so a frame arriving twice — once from the replay, once from the live lane —
/// is admitted exactly once.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct EventSeam {
    pub(super) delivered: u64,
}

impl EventSeam {
    /// 🌱️ A seam resuming after `since`.
    pub fn new(since: u64) -> Self {
        Self { delivered: since }
    }

    /// ✅️ Whether this event still has to be delivered, advancing the high-water mark if so.
    pub fn admit(&mut self, event: &EventRecord) -> bool {
        if event.seq <= self.delivered {
            return false;
        }
        self.delivered = event.seq;
        true
    }

    /// 🚩️ The highest sequence delivered so far.
    pub fn delivered(&self) -> u64 {
        self.delivered
    }
}

/// 📼️ The generic durable-lane bridge: **subscribe first**, then replay, then forward live.
///
/// The order is the whole point. Subscribing before reading `events_since` means an event committed
/// between the read and the first `recv` is buffered rather than lost; the [`EventSeam`] then drops
/// whatever the replay already covered, so the seam has neither a gap nor a duplicate. Returns once
/// the sink refuses a frame or the lane closes.
pub async fn pump_events<I: ServerInstance, S>(state: &ServerState<I>, actor: &ActorKey, since: u64, live: &mut Subscription, sink: &mut S) -> Result<(), ServerError>
where
    S: Sink<Message> + Unpin,
{
    let mut seam = EventSeam::new(since);
    for event in state.replay_events(actor, since).await? {
        if seam.admit(&event) && !deliver_event(sink, &event).await {
            return Ok(());
        }
    }
    while let Some(bytes) = live.recv().await {
        let Ok(event) = serde_json::from_slice::<EventRecord>(&bytes) else { continue };
        if seam.admit(&event) && !deliver_event(sink, &event).await {
            break;
        }
    }
    Ok(())
}

async fn deliver_event<S>(sink: &mut S, event: &EventRecord) -> bool
where
    S: Sink<Message> + Unpin,
{
    let Ok(text) = serde_json::to_string(event) else { return true };
    sink.send(Message::Text(text.into())).await.is_ok()
}

/// 🔖️ Where a durable-lane subscriber resumes from.
#[derive(Clone, Copy, Debug, Default, Deserialize)]
pub struct EventStreamQuery {
    /// ⛓️ The last sequence the client already holds; `0` replays the whole stream.
    #[serde(default)]
    pub since: u64,
}

/// 📜️ One page of durable history, for a client that would rather poll than hold a socket.
pub async fn get_events<I: ServerInstance>(
    Path((tenant, kind, id)): Path<(String, String, String)>,
    Query(query): Query<EventStreamQuery>,
    headers: HeaderMap,
    ConnectInfo(peer): ConnectInfo<SocketAddr>,
    State(state): State<ServerState<I>>,
) -> Result<Json<Vec<EventRecord>>, ServerError> {
    let actor = ActorKey { tenant: TenantId(tenant), kind, id };
    let resolved = state.identify(&headers, Some(peer)).await;
    state.authorize(&PolicyRequest { point: PolicyPoint::EventDelivery, principal: resolved.principal, scope: None, resource: stream_lane(&actor), action: "read".to_string() })?;
    Ok(Json(state.replay_events(&actor, query.since).await?))
}

/// 📡️ The durable lane as a websocket: replay then live, gap-free and duplicate-free.
pub async fn get_event_stream_ws<I: ServerInstance>(
    ws: WebSocketUpgrade,
    Path((tenant, kind, id)): Path<(String, String, String)>,
    Query(query): Query<EventStreamQuery>,
    headers: HeaderMap,
    ConnectInfo(peer): ConnectInfo<SocketAddr>,
    State(state): State<ServerState<I>>,
) -> Result<Response, ServerError> {
    let actor = ActorKey { tenant: TenantId(tenant), kind, id };
    let resolved = state.identify(&headers, Some(peer)).await;
    state.authorize(&PolicyRequest { point: PolicyPoint::Subscription, principal: resolved.principal, scope: None, resource: stream_lane(&actor), action: "subscribe".to_string() })?;
    let permit = state.open_socket(&headers, Some(peer))?;
    Ok(bounded_socket(ws, EVENT_STREAM_INBOUND_BYTES).on_upgrade(move |socket| handle_event_stream(socket, actor, query.since, state, permit)))
}

async fn handle_event_stream<I: ServerInstance>(socket: WebSocket, actor: ActorKey, since: u64, state: ServerState<I>, _permit: SocketPermit) {
    let (mut sender, mut receiver) = socket.split();
    let mut live = state.fanout.subscribe(&stream_lane(&actor));
    let pump = pump_events(&state, &actor, since, &mut live, &mut sender);
    tokio::select! {
        _ = pump => {}
        _ = drain_until_close(&mut receiver) => {}
    }
}

async fn drain_until_close(receiver: &mut futures::stream::SplitStream<WebSocket>) {
    while let Some(Ok(message)) = receiver.next().await {
        if matches!(message, Message::Close(_)) {
            return;
        }
    }
}
//#endregion 🔖️EventStream

//#region 🔖️DocumentStream
/// 📄️ Non-identity join hints for a document socket. Actor and session are never query parameters —
/// the gateway binds them from the authenticated principal.
#[derive(Clone, Debug, Deserialize)]
pub struct DocumentStreamQuery {
    /// 🪟️ Which surface the actor joined from.
    pub surface: Option<String>,
    /// ⏮️ The engine's own resumption token, passed through verbatim.
    pub resume: Option<String>,
}

/// 🔗️ Bridge one document socket onto the instance's [`DocumentAuthority`].
pub async fn get_document_ws<I: ServerInstance>(
    ws: WebSocketUpgrade,
    Path(scope): Path<String>,
    Query(query): Query<DocumentStreamQuery>,
    headers: HeaderMap,
    ConnectInfo(peer): ConnectInfo<SocketAddr>,
    State(state): State<ServerState<I>>,
) -> Result<Response, ServerError> {
    let scope = Scope(scope);
    let resolved = state.identify(&headers, Some(peer)).await;
    state.authorize(&PolicyRequest { point: PolicyPoint::Subscription, principal: resolved.principal.clone(), scope: Some(scope.clone()), resource: document_lane(&scope), action: "subscribe".to_string() })?;
    let documents = state.documents.as_ref().ok_or_else(|| ServerError::NotFound("this instance hosts no document authority".to_string()))?;
    let identity = documents.bind_socket(&scope, &resolved).await?;
    Ok(ws.protocols([SESSION_PROTOCOL_V1]).on_upgrade(move |socket| handle_document(socket, scope, query, resolved.principal, identity, state)))
}

/// 📮️ Wrap a relayed frame with the session that produced it, so a session never receives its own
/// frames back through the lane it also sends on.
pub(super) fn wrap_relay(origin: &str, frame: &[u8]) -> Vec<u8> {
    let origin = origin.as_bytes();
    let mut wrapped = Vec::with_capacity(2 + origin.len() + frame.len());
    wrapped.extend_from_slice(&(origin.len() as u16).to_le_bytes());
    wrapped.extend_from_slice(origin);
    wrapped.extend_from_slice(frame);
    wrapped
}

/// 📬️ The inverse of [`wrap_relay`]; a malformed wrapper is dropped rather than trusted.
pub(super) fn unwrap_relay(bytes: &[u8]) -> Option<(&str, &[u8])> {
    let length = usize::from(u16::from_le_bytes([*bytes.first()?, *bytes.get(1)?]));
    let origin = std::str::from_utf8(bytes.get(2..2 + length)?).ok()?;
    Some((origin, bytes.get(2 + length..)?))
}

async fn handle_document<I: ServerInstance>(socket: WebSocket, scope: Scope, query: DocumentStreamQuery, principal: Principal, identity: DocumentSocketIdentity, state: ServerState<I>) {
    let Some(documents) = state.documents.clone() else { return };
    let (mut sender, mut receiver) = socket.split();
    let session = identity.session.clone();
    let actor = identity.actor.clone();
    let lane = document_lane(&scope);
    let mut live = state.fanout.subscribe(&lane);
    state.presence.join(&scope.0, &actor, query.surface.as_deref().unwrap_or("unknown"));

    let handshake = documents.handshake().await;
    let hello = match handshake {
        DocumentHandshake::ServerFirst => None,
        DocumentHandshake::ClientFirst => {
            match receiver.next().await {
                Some(Ok(Message::Binary(payload))) => Some(payload.to_vec()),
                Some(Ok(Message::Ping(payload))) => {
                    let _ = sender.send(Message::Pong(payload)).await;
                    match receiver.next().await {
                        Some(Ok(Message::Binary(payload))) => Some(payload.to_vec()),
                        _ => {
                            state.presence.leave(&scope.0, &actor);
                            return;
                        }
                    }
                }
                _ => {
                    state.presence.leave(&scope.0, &actor);
                    return;
                }
            }
        }
    };

    match documents.welcome(&scope, &actor, query.resume.as_deref(), hello.as_deref()).await {
        Ok(frames) => {
            for frame in frames {
                if sender.send(Message::Binary(frame.into())).await.is_err() {
                    state.presence.leave(&scope.0, &actor);
                    return;
                }
            }
        }
        Err(error) => {
            let _ = sender.send(Message::Text(error.to_string().into())).await;
            state.presence.leave(&scope.0, &actor);
            return;
        }
    }

    loop {
        tokio::select! {
            incoming = receiver.next() => {
                match incoming {
                    Some(Ok(Message::Binary(payload))) => {
                        match documents.submit_frame(&scope, &actor, &principal, &payload).await {
                            Ok(frames) => {
                                for frame in &frames.echo {
                                    if sender.send(Message::Binary(frame.clone().into())).await.is_err() {
                                        break;
                                    }
                                }
                                for frame in &frames.relay {
                                    state.fanout.publish(&lane, wrap_relay(&session, frame));
                                }
                            }
                            Err(error) => {
                                if sender.send(Message::Text(error.to_string().into())).await.is_err() {
                                    break;
                                }
                            }
                        }
                    }
                    Some(Ok(Message::Ping(payload))) => {
                        if sender.send(Message::Pong(payload)).await.is_err() {
                            break;
                        }
                    }
                    Some(Ok(Message::Close(_))) | None => break,
                    Some(Err(_)) => break,
                    Some(Ok(_)) => {}
                }
            }
            relayed = live.recv() => {
                let Some(bytes) = relayed else { break };
                let Some((origin, frame)) = unwrap_relay(&bytes) else { continue };
                if origin == session {
                    continue;
                }
                if sender.send(Message::Binary(frame.to_vec().into())).await.is_err() {
                    break;
                }
            }
            _ = state.kicks.kicked(&session) => break,
        }
    }

    state.presence.leave(&scope.0, &actor);
    state.kicks.forget(&session);
}

//#endregion 🔖️DocumentStream
