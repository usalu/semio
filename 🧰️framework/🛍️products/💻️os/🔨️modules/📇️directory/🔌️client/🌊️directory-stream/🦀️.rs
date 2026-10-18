//! 🌊️ Directory stream of the directory client: the reconnect bounds, the WS urls, the backoff, the stream turns and the directory stream driver. See [`super`].

use super::*;

//#region 🔖️Stream
/// ⏱️ Reconnect backoff floor/ceiling — same constants `🏪️store/👷️worker/🟦️.ts`'s
/// `HUB_RECONNECT_MIN_MS`/`HUB_RECONNECT_MAX_MS` already use for the document WS.
pub const HUB_RECONNECT_MIN_MS: u64 = 500;

pub const HUB_RECONNECT_MAX_MS: u64 = 30_000;

/// 🔗️ `remote://host:port` / `http(s)://…` → `ws(s)://host:port/directory/socket/v1?since=`
/// (contract §C2). Pure and independently testable, mirroring `🏪️store/🔄️sync`'s `hub_ws_url`.
pub fn directory_ws_url(base_url: &str, since: u64) -> String {
    let secure = base_url.starts_with("https://") || base_url.starts_with("wss://");
    let authority = base_url.split_once("://").map_or(base_url, |(_, rest)| rest).split('/').next().unwrap_or(base_url);
    let scheme = if secure { "wss" } else { "ws" };
    format!("{scheme}://{authority}/directory/socket/v1?since={since}")
}

/// 🎯️ Builds the exact URL paired with one scope-bound directory grant.
pub fn directory_scoped_ws_url(base_url: &str, scope: &DocumentScope, since: u64) -> String {
    let secure = base_url.starts_with("https://") || base_url.starts_with("wss://");
    let authority = base_url.split_once("://").map_or(base_url, |(_, rest)| rest).split('/').next().unwrap_or(base_url);
    let scheme = if secure { "wss" } else { "ws" };
    format!("{scheme}://{authority}/directory/spaces/{}/documents/{}/socket/v1?since={since}", encode_url_component(&scope.space_id), encode_url_component(&scope.document_id),)
}

/// ⏱️ Doubling backoff capped at `HUB_RECONNECT_MAX_MS`, floored at `HUB_RECONNECT_MIN_MS`.
pub fn next_backoff_ms(current_ms: u64) -> u64 {
    current_ms.saturating_mul(2).clamp(HUB_RECONNECT_MIN_MS, HUB_RECONNECT_MAX_MS)
}

/// 📡️ One finite stream turn. `Dial` is executed separately on `Lane::Io`; `ReconnectAt` is armed
/// on the shared `TimerWheel`; `Idle` yields immediately instead of parking a worker on a socket.
pub enum DirectoryStreamTurn<T: DirectoryTransport> {
    Dial { client: Arc<DirectoryClient<T>>, since: u64 },
    DialScoped { client: Arc<DirectoryClient<T>>, scope: DocumentScope, since: u64 },
    Message(DirectoryStreamMessage),
    ReconnectAt(u64),
    Idle,
    Revoked(DocumentScope),
    Closed,
}

/// 🔁️ `GET /directory/socket/v1?since=` with auto-reconnect that resumes from the last `seq`/`headSeq`
/// this stream has observed — never re-subscribes from the caller's original `since` after a
/// drop, so the hub replays only the gap.
pub struct DirectoryStream<T: DirectoryTransport> {
    pub(super) client: Arc<DirectoryClient<T>>,
    pub(super) scope: Option<DocumentScope>,
    pub(super) since: u64,
    track_observed_frontier: bool,
    pub(super) connection: Option<T::Ws>,
    backoff_ms: u64,
    reconnect_at_ms: Option<u64>,
    dialing: bool,
    pub(super) closed: bool,
}

impl<T: DirectoryTransport + Clone> DirectoryStream<T> {
    pub(super) fn new(client: Arc<DirectoryClient<T>>, scope: Option<DocumentScope>, since: u64, track_observed_frontier: bool) -> Self {
        Self { client, scope, since, track_observed_frontier, connection: None, backoff_ms: HUB_RECONNECT_MIN_MS, reconnect_at_ms: None, dialing: false, closed: false }
    }

    pub fn since(&self) -> u64 {
        self.since
    }

    /// 📌️ Advances the durable reconnect frontier after an exact Home page acknowledgement.
    pub fn acknowledge(&mut self, through: u64) -> Result<(), DirectoryClientError> {
        if self.track_observed_frontier || through < self.since || through > DOCUMENT_OPEN_MAX_SAFE_INTEGER {
            return Err(DirectoryClientError::Decode("directory stream acknowledgement frontier invalid".into()));
        }
        self.since = through;
        Ok(())
    }

    pub fn close(&mut self) {
        self.closed = true;
        if let Some(mut connection) = self.connection.take() {
            connection.close();
        }
    }

    /// 🔄️ Applies a completed I/O-lane dial without reading from the socket in the same turn.
    pub fn complete_dial(&mut self, now_ms: u64, result: Result<T::Ws, TransportError>) -> DirectoryStreamTurn<T> {
        self.dialing = false;
        if self.closed {
            if let Ok(mut connection) = result {
                connection.close();
            }
            return DirectoryStreamTurn::Closed;
        }
        match result {
            Ok(connection) => {
                self.connection = Some(connection);
                self.backoff_ms = HUB_RECONNECT_MIN_MS;
                self.reconnect_at_ms = None;
                DirectoryStreamTurn::Idle
            }
            Err(_) => self.reconnecting(now_ms),
        }
    }

    /// 🔁️ Performs bounded, nonblocking protocol work only; at most eight invalid/control frames
    /// are skipped before yielding so malformed traffic cannot monopolize a worker turn.
    pub fn turn(&mut self, ctx: &OperationContext, now_ms: u64) -> DirectoryStreamTurn<T> {
        if self.closed || ctx.cancel.is_cancelled_now() {
            self.close();
            return DirectoryStreamTurn::Closed;
        }
        if self.dialing {
            return DirectoryStreamTurn::Idle;
        }
        if let Some(reconnect_at_ms) = self.reconnect_at_ms {
            if now_ms < reconnect_at_ms {
                return DirectoryStreamTurn::ReconnectAt(reconnect_at_ms);
            }
            self.reconnect_at_ms = None;
        }
        if self.connection.is_none() {
            self.dialing = true;
            return match &self.scope {
                Some(scope) => DirectoryStreamTurn::DialScoped { client: self.client.clone(), scope: scope.clone(), since: self.since },
                None => DirectoryStreamTurn::Dial { client: self.client.clone(), since: self.since },
            };
        }
        for _ in 0..8 {
            let Some(connection) = self.connection.as_mut() else { return self.reconnecting(now_ms) };
            match connection.try_recv_text() {
                Ok(DirectoryWsPoll::Text(text)) => {
                    if let Ok(message) = semio_framework_pack_json::from_json_str::<DirectoryStreamMessage>(&text, semio_framework_pack_json::JsonMemberPolicy::Reject) {
                        self.track(&message);
                        return DirectoryStreamTurn::Message(message);
                    }
                }
                Ok(DirectoryWsPoll::Pending) => return DirectoryStreamTurn::Idle,
                Ok(DirectoryWsPoll::Closed(Some(4401))) => {
                    self.connection = None;
                    self.closed = true;
                    return self.scope.clone().map_or(DirectoryStreamTurn::Closed, DirectoryStreamTurn::Revoked);
                }
                Ok(DirectoryWsPoll::Closed(_)) | Err(_) => {
                    self.connection = None;
                    return self.reconnecting(now_ms);
                }
            }
        }
        DirectoryStreamTurn::Idle
    }

    fn reconnecting(&mut self, now_ms: u64) -> DirectoryStreamTurn<T> {
        let reconnect_at_ms = now_ms.saturating_add(self.backoff_ms);
        self.reconnect_at_ms = Some(reconnect_at_ms);
        self.backoff_ms = next_backoff_ms(self.backoff_ms);
        DirectoryStreamTurn::ReconnectAt(reconnect_at_ms)
    }

    #[doc(hidden)] pub fn track(&mut self, message: &DirectoryStreamMessage) {
        if !self.track_observed_frontier {
            return;
        }
        match message {
            DirectoryStreamMessage::Event { event } => self.since = self.since.max(event.seq),
            DirectoryStreamMessage::Heartbeat { head_seq } => self.since = self.since.max(*head_seq),
            DirectoryStreamMessage::Connection { .. } | DirectoryStreamMessage::Presence { .. } | DirectoryStreamMessage::RebootstrapRequired { .. } | DirectoryStreamMessage::AccessChanged { .. } => {}
        }
    }
}
//#endregion 🔖️Stream
