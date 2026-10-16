//! 🔐️ Session: limits, the server, the session state machine and its handshake.
//!
//! Holds the `🔐️Session` region of the crate root; the root re-exports its items.

use super::*;

//#region 🔐️Session

/// 📏️ Every bounded resource of one server.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Limits {
    pub max_payload_bytes: usize,
    pub max_nesting: usize,
    pub max_page_items: usize,
    pub max_registry_items: usize,
    pub max_recent_ids: usize,
    pub max_event_bytes: usize,
    pub max_events: usize,
    pub max_handlers: usize,
}

impl Default for Limits {
    fn default() -> Limits {
        Limits { max_payload_bytes: 1 << 20, max_nesting: 64, max_page_items: 64, max_registry_items: 4096, max_recent_ids: 4096, max_event_bytes: 64 << 20, max_events: 100_000, max_handlers: 8 }
    }
}

impl Limits {
    fn normalized(self) -> Limits {
        let defaults = Limits::default();
        Limits {
            max_payload_bytes: if self.max_payload_bytes == 0 { defaults.max_payload_bytes } else { self.max_payload_bytes },
            max_nesting: if self.max_nesting == 0 { defaults.max_nesting } else { self.max_nesting },
            max_page_items: if self.max_page_items == 0 { defaults.max_page_items } else { self.max_page_items },
            max_registry_items: if self.max_registry_items == 0 { defaults.max_registry_items } else { self.max_registry_items },
            max_recent_ids: if self.max_recent_ids == 0 { defaults.max_recent_ids } else { self.max_recent_ids },
            max_event_bytes: if self.max_event_bytes == 0 { defaults.max_event_bytes } else { self.max_event_bytes },
            max_events: if self.max_events == 0 { defaults.max_events } else { self.max_events },
            max_handlers: if self.max_handlers == 0 { defaults.max_handlers } else { self.max_handlers },
        }
    }
}

/// ⚙️ What a server needs before it can accept a peer.
#[derive(Clone, Debug)]
pub struct Config {
    pub server_name: String,
    pub server_version: String,
    pub instructions: String,
    pub limits: Limits,
}

/// 🧭️ The per-request handle a handler observes for cancellation.
#[derive(Clone, Debug, Default)]
pub struct Context {
    cancelled: Arc<AtomicBool>,
}

impl Context {
    pub fn is_cancelled(&self) -> bool {
        self.cancelled.load(Ordering::SeqCst)
    }

    pub(super) fn cancel(&self) {
        self.cancelled.store(true, Ordering::SeqCst);
    }
}

/// 📶️ Emits `notifications/progress` for the request it belongs to.
pub struct Progress<'a> {
    pub(super) session: &'a Session,
    pub(super) token: Option<String>,
}

impl Progress<'_> {
    /// 📣️ Reports one progress step; a request without a progress token reports nothing.
    pub fn report(&self, context: &Context, value: f64, total: Option<f64>, message: &str) -> Result<(), Error> {
        if context.is_cancelled() {
            return Err(Error::Closed);
        }
        let Some(token) = self.token.clone() else { return Ok(()) };
        let params = J::object(vec![
            ("progressToken", Some(J::Raw(token))),
            ("progress", Some(J::Float(value))),
            ("total", total.map(J::Float)),
            ("message", (!message.is_empty()).then(|| J::Str(message.to_string()))),
        ]);
        let payload = J::Obj(vec![
            ("jsonrpc".to_string(), J::Str(JSONRPC_VERSION.to_string())),
            ("method".to_string(), J::Str("notifications/progress".to_string())),
            ("params".to_string(), params),
        ])
        .text();
        if payload.len() > self.session.server.config.limits.max_payload_bytes {
            return Err(Error::PayloadTooLarge);
        }
        self.session.server.log.commit(&[EventInput::new("notification.sent", &self.session.peer, self.session.generation, payload.clone())])?;
        self.session.emit(payload.as_bytes())
    }
}

pub(super) type ToolHandler = Box<dyn Fn(&Context, &CallToolParams, &Progress<'_>) -> Result<CallToolResult, HandlerError> + Send + Sync>;
pub(super) type ResourceHandler = Box<dyn Fn(&Context, &str, &Progress<'_>) -> Result<Vec<ResourceContent>, HandlerError> + Send + Sync>;
pub(super) type PromptHandler = Box<dyn Fn(&Context, &str, &BTreeMap<String, String>) -> Result<GetPromptResult, HandlerError> + Send + Sync>;

/// 🖥️ The registry every session routes into.
pub struct Server {
    pub(super) config: Config,
    pub(super) log: EventLog,
    pub(super) tools: BTreeMap<String, (Tool, ToolHandler)>,
    pub(super) resources: BTreeMap<String, (Resource, ResourceHandler)>,
    pub(super) templates: BTreeMap<String, ResourceTemplate>,
    pub(super) prompts: BTreeMap<String, (Prompt, PromptHandler)>,
    pub(super) generation: AtomicU64,
    pub(super) handler_queued: AtomicI64,
    pub(super) handler_active: AtomicI64,
}

impl Server {
    /// 🏗️ Builds an empty server; a name and a version are mandatory.
    pub fn new(mut config: Config) -> Result<Server, Error> {
        if config.server_name.trim().is_empty() || config.server_version.trim().is_empty() {
            return Err(Error::Invalid("server name and version are required".to_string()));
        }
        config.limits = config.limits.normalized();
        let log = EventLog::new(config.limits.max_event_bytes, config.limits.max_events);
        Ok(Server {
            config,
            log,
            tools: BTreeMap::new(),
            resources: BTreeMap::new(),
            templates: BTreeMap::new(),
            prompts: BTreeMap::new(),
            generation: AtomicU64::new(0),
            handler_queued: AtomicI64::new(0),
            handler_active: AtomicI64::new(0),
        })
    }

    pub fn register_tool(&mut self, schema: Tool, handler: ToolHandler) -> Result<(), Error> {
        if schema.name.trim().is_empty() || schema.input_schema.kind != "object" {
            return Err(Error::Invalid("invalid tool registration".to_string()));
        }
        if self.tools.contains_key(&schema.name) {
            return Err(Error::Invalid("duplicate tool".to_string()));
        }
        if self.tools.len() == self.config.limits.max_registry_items {
            return Err(Error::Limit);
        }
        self.tools.insert(schema.name.clone(), (schema, handler));
        Ok(())
    }

    pub fn register_resource(&mut self, schema: Resource, handler: ResourceHandler) -> Result<(), Error> {
        if schema.uri.trim().is_empty() || schema.name.trim().is_empty() {
            return Err(Error::Invalid("invalid resource registration".to_string()));
        }
        if self.resources.contains_key(&schema.uri) {
            return Err(Error::Invalid("duplicate resource".to_string()));
        }
        if self.resources.len() == self.config.limits.max_registry_items {
            return Err(Error::Limit);
        }
        self.resources.insert(schema.uri.clone(), (schema, handler));
        Ok(())
    }

    pub fn register_resource_template(&mut self, schema: ResourceTemplate) -> Result<(), Error> {
        if schema.uri_template.trim().is_empty() || schema.name.trim().is_empty() {
            return Err(Error::Invalid("invalid resource template registration".to_string()));
        }
        if self.templates.contains_key(&schema.uri_template) {
            return Err(Error::Invalid("duplicate resource template".to_string()));
        }
        self.templates.insert(schema.uri_template.clone(), schema);
        Ok(())
    }

    pub fn register_prompt(&mut self, schema: Prompt, handler: PromptHandler) -> Result<(), Error> {
        if schema.name.trim().is_empty() {
            return Err(Error::Invalid("invalid prompt registration".to_string()));
        }
        if self.prompts.contains_key(&schema.name) {
            return Err(Error::Invalid("duplicate prompt".to_string()));
        }
        self.prompts.insert(schema.name.clone(), (schema, handler));
        Ok(())
    }

    /// 📚️ The event log this server writes into.
    pub fn events(&self) -> &EventLog {
        &self.log
    }

    /// 🔌️ Opens a session for one peer, bumping the peer generation and recording `session.opened`.
    pub fn connect(self: &Arc<Self>, peer: &str, sink: Option<Sink>) -> Result<Arc<Session>, Error> {
        if peer.trim().is_empty() {
            return Err(Error::Invalid("peer is required".to_string()));
        }
        let generation = self.generation.fetch_add(1, Ordering::SeqCst) + 1;
        let session = Arc::new(Session {
            server: Arc::clone(self),
            peer: peer.to_string(),
            generation,
            sink,
            state: Mutex::new(SessionState::default()),
            finished: Condvar::new(),
            handshake: Condvar::new(),
        });
        self.log.commit(&[EventInput::new("session.opened", peer, generation, format!("{{\"generation\":{generation}}}"))])?;
        Ok(session)
    }

    pub(super) fn capabilities(&self) -> J {
        J::object(vec![
            ("prompts", (!self.prompts.is_empty()).then(|| J::Obj(Vec::new()))),
            ("resources", (!self.resources.is_empty() || !self.templates.is_empty()).then(|| J::Obj(Vec::new()))),
            ("tools", (!self.tools.is_empty()).then(|| J::Obj(Vec::new()))),
        ])
    }

    /// 📊️ The live handler-pool occupancy, mirroring the Go server's counters.
    pub fn handler_stats(&self) -> (usize, i64, i64) {
        (self.config.limits.max_handlers, self.handler_active.load(Ordering::SeqCst), self.handler_queued.load(Ordering::SeqCst))
    }
}

/// 📤️ Where an out-of-band frame goes.
pub type Sink = Arc<dyn Fn(&[u8]) -> Result<(), Error> + Send + Sync>;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord)]
pub(super) enum Phase {
    #[default]
    Connected,
    Initialized,
    Ready,
    Closing,
    Closed,
}

#[derive(Default)]
pub(super) struct SessionState {
    pub(super) phase: Phase,
    /// 📥️ A `notifications/initialized` that overtook its own `initialize` response. A client is
    /// entitled to pipeline the two, and the reader hands notifications straight through while the
    /// request is still on a worker — so the notification is REMEMBERED here instead of dropped, and
    /// the initialize path applies it the moment the phase it is waiting for exists.
    pub(super) pending_initialized: bool,
    /// 🤝️ How many `initialize` requests the transport has delivered and no worker has answered yet.
    /// A burst is one write, so a request that follows `initialize` on the wire can reach a second
    /// worker before the first has promoted the phase; without this latch it answers `-32002`.
    handshake_pending: usize,
    active: BTreeMap<String, Context>,
    seen: Vec<String>,
    pre_cancelled: BTreeMap<String, String>,
    running: usize,
}

/// 🔐️ One peer connection: the lifecycle phase, the in-flight requests and the dedup window.
pub struct Session {
    pub(super) server: Arc<Server>,
    pub(super) peer: String,
    pub(super) generation: u64,
    sink: Option<Sink>,
    pub(super) state: Mutex<SessionState>,
    finished: Condvar,
    handshake: Condvar,
}

impl Session {
    pub fn peer(&self) -> &str {
        &self.peer
    }

    pub fn generation(&self) -> u64 {
        self.generation
    }

    fn emit(&self, payload: &[u8]) -> Result<(), Error> {
        match &self.sink {
            None => Ok(()),
            Some(sink) => sink(payload).inspect_err(|_| self.drop_session()),
        }
    }

    pub(super) fn lock(&self) -> std::sync::MutexGuard<'_, SessionState> {
        self.state.lock().unwrap_or_else(|poison| poison.into_inner())
    }

    /// 🛑️ Cancels every in-flight request and refuses new ones.
    pub fn drop_session(&self) {
        let mut state = self.lock();
        if state.phase >= Phase::Closing {
            return;
        }
        state.phase = Phase::Closing;
        for context in state.active.values() {
            context.cancel();
        }
        drop(state);
        self.handshake.notify_all();
    }

    /// 🤝️ Records that an `initialize` request has been delivered and is not answered yet. The
    /// transport calls it before the payload reaches the worker pool, so a request delivered after
    /// it in the same burst cannot overtake the phase promotion.
    pub fn expect_handshake(&self) {
        self.lock().handshake_pending += 1;
    }

    /// 🤝️ Releases every request that queued behind a delivered `initialize`.
    pub fn settle_handshake(&self) {
        let mut state = self.lock();
        state.handshake_pending = state.handshake_pending.saturating_sub(1);
        drop(state);
        self.handshake.notify_all();
    }

    /// ⏳️ Blocks while an `initialize` delivered before this request is still unanswered.
    pub(super) fn await_handshake(&self) {
        let mut state = self.lock();
        while state.handshake_pending > 0 && state.phase < Phase::Closing {
            state = self.handshake.wait(state).unwrap_or_else(|poison| poison.into_inner());
        }
    }

    /// 🚪️ Closes the session, waits for the in-flight requests and records `session.closed`.
    pub fn close(&self) -> Result<(), Error> {
        self.drop_session();
        let mut state = self.lock();
        while state.running > 0 {
            state = self.finished.wait(state).unwrap_or_else(|poison| poison.into_inner());
        }
        if state.phase == Phase::Closed {
            return Ok(());
        }
        state.phase = Phase::Closed;
        drop(state);
        self.server.log.commit(&[EventInput::new("session.closed", &self.peer, self.generation, "{\"reason\":\"closed\"}".to_string())])
    }

    pub(super) fn begin(&self, key: &str) -> Result<Context, RpcError> {
        let mut state = self.lock();
        if state.phase >= Phase::Closing {
            return Err(RpcError::new(CODE_STALE_SESSION, "session closed"));
        }
        if state.active.contains_key(key) {
            return Err(RpcError::new(CODE_DUPLICATE_REQUEST, "duplicate request id"));
        }
        if state.seen.iter().any(|seen| seen == key) {
            return Err(RpcError::new(CODE_DUPLICATE_REQUEST, "stale request id"));
        }
        let context = Context::default();
        if state.pre_cancelled.remove(key).is_some() {
            context.cancel();
        }
        state.active.insert(key.to_string(), context.clone());
        state.running += 1;
        Ok(context)
    }

    pub(super) fn finish(&self, key: &str) {
        let mut state = self.lock();
        state.active.remove(key);
        state.seen.push(key.to_string());
        let window = self.server.config.limits.max_recent_ids;
        if state.seen.len() > window {
            state.seen.remove(0);
        }
        state.running -= 1;
        self.finished.notify_all();
    }

    pub(super) fn cancel(&self, key: &str, reason: &str) {
        let mut state = self.lock();
        if let Some(context) = state.active.get(key) {
            context.cancel();
            return;
        }
        if state.seen.iter().any(|seen| seen == key) {
            return;
        }
        if state.pre_cancelled.len() < self.server.config.limits.max_recent_ids {
            state.pre_cancelled.insert(key.to_string(), reason.to_string());
        }
    }
}

//#endregion 🔐️Session
