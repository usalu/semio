//! 🛰️ The presence socket: admission, the room pump and the watcher.

use super::*;

//#region 🔖️PresenceStream
/// 🧍️ Where a presence socket joins from — a hint every member sees, never an identity.
#[derive(Clone, Debug, Default, Deserialize)]
pub struct PresenceStreamQuery {
    /// 🪟️ The surface the session joins from (≤ [`PresenceSettings::max_surface_chars`] chars).
    pub surface: Option<String>,
}

/// 🎫️ What a presence socket was admitted as: the principal every later `watch` is authorized for,
/// whether it may publish states in the room it joined, and the client address whose share of the
/// rooms' budget its frames are drawn from.
#[derive(Clone, Debug, PartialEq)]
pub struct PresenceGrant {
    pub principal: Principal,
    pub may_publish: bool,
    pub client: ClientKey,
}

/// 🛂️ Admit one presence socket before it is upgraded: the origin (when the instance names an
/// allowlist and the caller presents an `Origin`), the surface length, and
/// [`PolicyPoint::Subscription`] `join` on [`PRESENCE_RESOURCE`]. A joiner whose principal may not
/// also `publish` is refused every state but still sees the room.
pub async fn admit_presence<I: ServerInstance>(state: &ServerState<I>, headers: &HeaderMap, peer: Option<SocketAddr>, scope: &Scope, surface: &str) -> Result<PresenceGrant, ServerError> {
    if let (Some(admits), Some(origin)) = (&state.origin_admission, headers.get(header::ORIGIN)) {
        if !origin.to_str().is_ok_and(|origin| admits(origin)) {
            return Err(ServerError::Forbidden(format!("origin {origin:?} may not join presence")));
        }
    }
    if surface.chars().count() > state.presence_settings.max_surface_chars {
        return Err(ServerError::BadRequest(format!("surface exceeds {} characters", state.presence_settings.max_surface_chars)));
    }
    let principal = state.identify(headers, peer).await.principal;
    state.authorize(&presence_request(&principal, scope, PRESENCE_JOIN))?;
    let may_publish = state.authorize(&presence_request(&principal, scope, PRESENCE_PUBLISH)).is_ok();
    Ok(PresenceGrant { principal, may_publish, client: state.client(headers, peer) })
}

/// 👀️ Admit one `watch`: at most [`PresenceSettings::max_watch_scopes`] scopes, each one granted
/// [`PolicyPoint::Subscription`] [`PRESENCE_WATCH`] on [`PRESENCE_RESOURCE`] for the socket's
/// principal — the scoped policy that admits joins decides watches too. Answers the distinct scopes,
/// or the `refused` reason naming the first scope refused; a refused watch changes nothing.
pub fn admit_watch<I: ServerInstance>(state: &ServerState<I>, principal: &Principal, scopes: &[Scope]) -> Result<BTreeSet<String>, String> {
    if scopes.len() > state.presence_settings.max_watch_scopes {
        return Err(REFUSED_WATCH_TOO_MANY.to_string());
    }
    for scope in scopes {
        if state.authorize(&presence_request(principal, scope, PRESENCE_WATCH)).is_err() {
            return Err(format!("{REFUSED_FORBIDDEN} {}", scope.0));
        }
    }
    Ok(scopes.iter().map(|scope| scope.0.clone()).collect())
}

fn presence_request(principal: &Principal, scope: &Scope, action: &str) -> PolicyRequest {
    PolicyRequest { point: PolicyPoint::Subscription, principal: principal.clone(), scope: Some(scope.clone()), resource: PRESENCE_RESOURCE.to_string(), action: action.to_string() }
}

/// 👥️ `GET /scopes/{scope}/presence/ws` — the presence socket (`semio.presence.v1`): a `welcome`,
/// then at most one bounded `batch` per tick while the room has something the socket was not sent.
pub async fn get_presence_ws<I: ServerInstance>(
    ws: WebSocketUpgrade,
    Path(scope): Path<String>,
    Query(query): Query<PresenceStreamQuery>,
    headers: HeaderMap,
    ConnectInfo(peer): ConnectInfo<SocketAddr>,
    State(state): State<ServerState<I>>,
) -> Result<Response, ServerError> {
    let scope = Scope(scope);
    let surface = query.surface.unwrap_or_default();
    let grant = admit_presence(&state, &headers, Some(peer), &scope, &surface).await?;
    let permit = state.open_socket(&headers, Some(peer))?;
    Ok(bounded_socket(ws, state.presence_settings.inbound_bytes()).protocols([PRESENCE_PROTOCOL_V1]).on_upgrade(move |socket| async move {
        let _permit = permit;
        let (mut sender, mut receiver) = socket.split();
        run_presence(&state, &scope, &surface, &grant, &mut sender, &mut receiver).await;
    }))
}

/// 🔁️ One presence session over any message sink and stream — the socket in production, channels
/// in a test. Joins the room (leasing a colour from [`Presence`]), sends the `welcome`, then serves
/// until the peer closes, stays idle past [`PresenceSettings::idle`], takes longer than that to be
/// sent one frame, or is kicked; it always leaves the room and stops watching on the way out.
///
/// The session reads its room through a [`RoomReader`]: while the room has something it was not
/// sent, one frame of at most [`PresenceSettings::max_frame_bytes`] per tick — and, while it is
/// still being sent the roster it joined, one frame after the other as fast as it takes them.
pub async fn run_presence<I, S, R, E>(state: &ServerState<I>, scope: &Scope, surface: &str, grant: &PresenceGrant, sink: &mut S, stream: &mut R)
where
    I: ServerInstance,
    S: Sink<Message> + Unpin,
    R: Stream<Item = Result<Message, E>> + Unpin,
{
    let settings = *state.presence_settings;
    let lane = presence_lane(scope);
    let session = presence_session_id();
    let mut entry = PresenceEntry { colour: state.presence.acquire_colour(&lane, &session), session, surface: surface.to_string(), state: OpaqueJson::Null };
    let mut room = state.presence_rooms.read(&lane, Some(&entry.session), grant.client);
    state.presence_rooms.join(&lane, &entry);
    let seat = Seat { scope, lane: &lane, may_publish: grant.may_publish, patience: settings.idle };
    let mut watcher = Watcher::default();
    let mut budget = StateBudget::new(settings.max_states_per_second);
    let mut last_seen = Instant::now();
    let mut keepalive = tokio::time::interval_at(Instant::now() + settings.keepalive, settings.keepalive);
    let mut due = Instant::now();
    let mut open = feed(&mut room, &entry, &settings, sink).await.is_some();
    while open {
        tokio::select! {
            incoming = stream.next() => {
                last_seen = Instant::now();
                open = match incoming {
                    Some(Ok(Message::Text(text))) => match admit_frame(text.as_str(), &settings, &mut budget, Instant::now()) {
                        Ok(Some(PresenceFrame::State { state: shared })) => share_state(state, &seat, &mut entry, shared, sink).await,
                        Ok(Some(PresenceFrame::Watch { scopes, interval_ms })) => match admit_watch(state, &grant.principal, &scopes) {
                            Ok(scopes) => watcher.watch(state, &scopes, grant.client, watch_interval(interval_ms, &settings), &settings, sink).await,
                            Err(reason) => refuse(sink, reason, settings.idle).await,
                        },
                        Ok(_) => true,
                        Err(reason) => refuse(sink, reason, settings.idle).await,
                    },
                    Some(Ok(Message::Binary(_))) => refuse(sink, REFUSED_INVALID.to_string(), settings.idle).await,
                    Some(Ok(Message::Ping(payload))) => send(sink, Message::Pong(payload), settings.idle).await,
                    Some(Ok(Message::Pong(_))) => true,
                    Some(Ok(Message::Close(_))) | Some(Err(_)) | None => false,
                };
            }
            () = turn(&mut room, due) => {
                let fed = feed(&mut room, &entry, &settings, sink).await;
                open = fed.is_some();
                due = if fed == Some(true) && room.arriving() { Instant::now() } else { Instant::now() + settings.tick };
            }
            _ = keepalive.tick() => open = send(sink, Message::Ping(Vec::new().into()), settings.idle).await,
            _ = tokio::time::sleep_until(last_seen + settings.idle) => {
                send(sink, Message::Close(None), settings.idle).await;
                open = false;
            }
            _ = state.kicks.kicked(&entry.session) => open = false,
            () = next_flush(&mut watcher.ticks) => open = watcher.send(&settings, sink, false).await,
        }
    }
    drop(watcher);
    state.presence_rooms.leave(&lane, &entry.session);
    drop(room);
    state.presence.release_colour(&lane, &entry.session);
    state.kicks.forget(&entry.session);
}

/// 💺️ Where one presence session sits: its scope and lane, whether it may publish, and how long
/// it is given to take one frame.
struct Seat<'a> {
    pub(super) scope: &'a Scope,
    pub(super) lane: &'a str,
    pub(super) may_publish: bool,
    pub(super) patience: Duration,
}

/// ⏳️ Resolves once `room` has something its reader was not sent and the reader's next page is due.
async fn turn(room: &mut RoomReader, due: Instant) {
    room.unsent().await;
    tokio::time::sleep_until(due).await;
}

/// 📤️ Send a member the next page of its room: a `welcome` when the page replaces what the member
/// holds, a `batch` otherwise, nothing when there is nothing to send. Answers whether a frame went
/// out, or `None` when the peer no longer takes one.
async fn feed<S: Sink<Message> + Unpin>(room: &mut RoomReader, entry: &PresenceEntry, settings: &PresenceSettings, sink: &mut S) -> Option<bool> {
    let page = room.page(settings.max_frame_bytes, Instant::now());
    let frame = if page.replace {
        page.welcome(&entry.session, entry.colour)
    } else if page.is_empty() {
        return Some(false);
    } else {
        page.batch()
    };
    send(sink, Message::Text(frame.into()), settings.idle).await.then_some(true)
}

/// 📣️ Share one admitted state in the joined room — when the principal may publish and every
/// module's [`ServerModule::presence_admission`] accepts it — or answer why not.
async fn share_state<I: ServerInstance, S: Sink<Message> + Unpin>(state: &ServerState<I>, seat: &Seat<'_>, entry: &mut PresenceEntry, shared: OpaqueJson, sink: &mut S) -> bool {
    let verdict = if seat.may_publish { state.modules.iter().try_for_each(|module| module.presence_admission(seat.scope, &shared)) } else { Err(REFUSED_FORBIDDEN.to_string()) };
    match verdict {
        Ok(()) => {
            entry.state = shared;
            state.presence_rooms.update(seat.lane, entry);
            true
        }
        Err(reason) => refuse(sink, reason, seat.patience).await,
    }
}

/// 👁️ The watching half of one presence socket: one [`RoomReader`] per watched scope, in scope
/// order, the interval their changes are sent at, and the scope that goes first next time.
#[derive(Default)]
struct Watcher {
    pub(super) scopes: Vec<(String, RoomReader)>,
    pub(super) interval: Duration,
    ticks: Option<tokio::time::Interval>,
    pub(super) next: usize,
}

impl Watcher {
    /// ⌚️ Replace the watch set: stop reading dropped scopes, start reading new ones on behalf of
    /// `client`, (re)start the interval when it changed, and send the new scopes their `snapshot`.
    pub(super) async fn watch<I: ServerInstance, S: Sink<Message> + Unpin>(&mut self, state: &ServerState<I>, scopes: &BTreeSet<String>, client: ClientKey, interval: Duration, settings: &PresenceSettings, sink: &mut S) -> bool {
        self.scopes.retain(|(scope, _)| scopes.contains(scope));
        for scope in scopes {
            if !self.scopes.iter().any(|(watched, _)| watched == scope) {
                self.scopes.push((scope.clone(), state.presence_rooms.read(&presence_lane(&Scope(scope.clone())), None, client)));
            }
        }
        self.scopes.sort_by(|(left, _), (right, _)| left.cmp(right));
        if self.scopes.is_empty() {
            self.ticks = None;
        } else if self.ticks.is_none() || self.interval != interval {
            let mut ticks = tokio::time::interval_at(Instant::now() + interval, interval);
            ticks.set_missed_tick_behavior(MissedTickBehavior::Delay);
            self.ticks = Some(ticks);
        }
        self.interval = interval;
        self.send(settings, sink, true).await
    }

    /// 🕰️ Send one `watched` frame per scope that has something this socket was not sent — only
    /// the scopes that were sent nothing yet when `arrivals` — until one frame's allowance
    /// ([`PresenceSettings::max_frame_bytes`]) is spent over all of them. The scope the allowance
    /// ran out at goes first next time, so every watched scope gets its turn.
    pub(super) async fn send<S: Sink<Message> + Unpin>(&mut self, settings: &PresenceSettings, sink: &mut S, arrivals: bool) -> bool {
        let count = self.scopes.len();
        let mut spent = 0;
        for offset in 0..count {
            let index = (self.next + offset) % count;
            let (scope, reader) = &mut self.scopes[index];
            if !(if arrivals { reader.fresh() } else { reader.has_unsent() }) {
                continue;
            }
            if spent >= settings.max_frame_bytes {
                self.next = index;
                break;
            }
            let page = reader.page(settings.max_frame_bytes - spent, Instant::now());
            spent += page.bytes();
            if !page.is_empty() && !send(sink, Message::Text(page.watched(scope).into()), settings.idle).await {
                return false;
            }
        }
        true
    }
}

/// ⏰️ The next flush of a watcher's interval — never, while nothing is watched.
async fn next_flush(ticks: &mut Option<tokio::time::Interval>) {
    match ticks {
        Some(ticks) => {
            ticks.tick().await;
        }
        None => std::future::pending().await,
    }
}

/// 📮️ Send one message, giving the peer `patience` to take it: a peer that does not is gone.
pub(super) async fn send<S: Sink<Message> + Unpin>(sink: &mut S, message: Message, patience: Duration) -> bool {
    matches!(tokio::time::timeout(patience, sink.send(message)).await, Ok(Ok(())))
}

/// 🚫️ Answer a frame the server dropped with why.
async fn refuse<S: Sink<Message> + Unpin>(sink: &mut S, reason: String, patience: Duration) -> bool {
    match serde_json::to_string(&PresenceFrame::Refused { reason }) {
        Ok(text) => send(sink, Message::Text(text.into()), patience).await,
        Err(_) => true,
    }
}
//#endregion 🔖️PresenceStream
