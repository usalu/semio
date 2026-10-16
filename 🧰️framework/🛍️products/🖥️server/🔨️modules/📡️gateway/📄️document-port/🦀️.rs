//! 📄️ The document authority and query handler ports with their empty implementations.

use super::*;

//#region 🔖️DocumentPort
/// 📄️ The port a replication engine plugs into, and the only thing this product knows about one.
///
/// The server product deliberately depends on no document engine: not on the os product, not on
/// `db`, not on any concrete CRDT or OT implementation. The gateway can therefore bridge a document
/// websocket — handshake, submit, relay — while naming nothing but opaque byte frames. An instance
/// may supply an implementation, or none at all, in which case the gateway does not mount the
/// document route and the instance is free to own `/scopes/{scope}/document/ws` itself (hub does,
/// because its socket carries grant admission, presence leases and live revocation).
/// **Send futures, declared not inferred.** Every method of this port returns
/// `impl Future<..> + Send` instead of being written `async fn`, and that is structural, not a
/// style choice: [`ServerState`] reaches this port behind an
/// instance's associated type, so the concrete future is opaque at the call site and axum's
/// handler and socket tasks — which are `Send` by construction — cannot otherwise prove it may
/// cross a thread. An `async fn` here compiles and then fails at every route that uses it. The
/// implementations stay ordinary `async fn`, which Rust accepts against this signature, and so does
/// the delegate `dyn_enum_close!` generates for a set of them: the macro emits `async fn .. -> T`
/// over the future's `Output`, because two match arms cannot unify two distinct opaque futures.
/// 🤝️ When the gateway runs [`DocumentAuthority::welcome`] relative to the first client frame.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum DocumentHandshake {
    /// 👋️ Send welcome before reading any client frame.
    #[default]
    ServerFirst,
    /// 👂️ Wait for one client binary frame, then call welcome with that frame as `hello`.
    ClientFirst,
}

/// 🎓️ Socket identity derived server-side from the authenticated principal — never from the URL.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DocumentSocketIdentity {
    /// 🙋️ Hub-issued document actor id envelopes must be authored as.
    pub actor: String,
    /// 🎫️ Presence / kick session id for this socket.
    pub session: String,
}

/// 🧭️ Bind a document socket to the principal's hub-issued actor. Callers never supply actor ids.
pub fn document_socket_identity(resolved: &Resolved) -> Result<DocumentSocketIdentity, ServerError> {
    let actor = match &resolved.actor {
        Some(actor) if !actor.is_empty() => actor.clone(),
        _ => match &resolved.principal {
            Principal::User { id } | Principal::ServiceAccount { id } | Principal::Device { id } if !id.is_empty() => id.clone(),
            Principal::Anonymous | Principal::User { .. } | Principal::ServiceAccount { .. } | Principal::Device { .. } => {
                return Err(ServerError::Unauthorized("document socket requires an authenticated principal with an actor grant".into()));
            }
        },
    };
    let session = resolved.session.as_ref().map(|session| session.0.clone()).filter(|session| !session.is_empty()).unwrap_or_else(|| actor.clone());
    Ok(DocumentSocketIdentity { actor, session })
}

#[dyn_enum]
pub trait DocumentAuthority: Send + Sync {
    /// 🤝️ Whether this engine greets first or waits for a client hello frame.
    fn handshake(&self) -> impl Future<Output = DocumentHandshake> + Send;

    /// 🧭️ Resolve the socket actor for this principal on `scope`, or refuse when the principal holds
    /// no grant. The default binds from [`Resolved::actor`] / the principal id; hub overrides to
    /// require a hub-issued actor grant.
    fn bind_socket(&self, scope: &Scope, resolved: &Resolved) -> impl Future<Output = Result<DocumentSocketIdentity, ServerError>> + Send;

    /// 👋️ The handshake frames for a joining actor. `resume` is the engine's own resumption token;
    /// `hello` is the first client binary frame when [`DocumentHandshake::ClientFirst`], else `None`.
    /// Returning several frames lets an engine stream welcome plus bootstrap chunks as distinct WS
    /// messages — the gateway never concatenates them.
    fn welcome(&self, scope: &Scope, actor: &str, resume: Option<&str>, hello: Option<&[u8]>) -> impl Future<Output = Result<Vec<Vec<u8>>, ServerError>> + Send;

    /// 📨️ Apply one client frame and return the frames for the submitter and for peer relay.
    /// `actor` is the socket's server-bound hub actor id (from [`DocumentAuthority::bind_socket`]),
    /// not a caller-supplied query value — envelopes are authored as that socket actor.
    fn submit_frame(&self, scope: &Scope, actor: &str, principal: &Principal, frame: &[u8]) -> impl Future<Output = Result<DocumentFrames, ServerError>> + Send;
}

/// 📤️ Frames produced by one [`DocumentAuthority::submit_frame`] call.
///
/// Split on purpose: an ack belongs only to the submitter, while the accepted command batch must
/// reach every other session on the document. Engines that have nothing to hide from the submitter
/// put the same bytes in both fields.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct DocumentFrames {
    /// 🪞 Frames returned only to the socket that submitted.
    pub echo: Vec<Vec<u8>>,
    /// 📣 Frames relayed to every other session on the document lane.
    pub relay: Vec<Vec<u8>>,
}

impl DocumentFrames {
    /// 🪞📣 The same frames for the submitter and for every peer.
    pub fn mirrored(frames: Vec<Vec<u8>>) -> Self {
        Self { echo: frames.clone(), relay: frames }
    }
}

/// 🕳️ The document authority of an instance that hosts none.
///
/// Uninhabited on purpose, and the only honest way to say "no replication engine here": naming it
/// as [`ServerInstance::Documents`] makes `Option<Arc<Self>>` permanently `None`, so the gateway
/// never mounts `/scopes/{scope}/document/ws`. An instance that *does* have an engine names that
/// engine instead and registers it with [`ServerBuilder::document_authority`], which mounts the route.
pub enum NoDocumentAuthority {}

impl DocumentAuthority for NoDocumentAuthority {
    async fn handshake(&self) -> DocumentHandshake {
        match *self {}
    }

    async fn bind_socket(&self, _scope: &Scope, _resolved: &Resolved) -> Result<DocumentSocketIdentity, ServerError> {
        match *self {}
    }

    async fn welcome(&self, _scope: &Scope, _actor: &str, _resume: Option<&str>, _hello: Option<&[u8]>) -> Result<Vec<Vec<u8>>, ServerError> {
        match *self {}
    }

    async fn submit_frame(&self, _scope: &Scope, _actor: &str, _principal: &Principal, _frame: &[u8]) -> Result<DocumentFrames, ServerError> {
        match *self {}
    }
}
//#endregion 🔖️DocumentPort

//#region 🔖️Query
/// ❓️ One registered read. A query never touches an actor's private state — it answers from a
/// [`ProjectionStore`], which is why the handler is handed nothing else.
/// The projection store is taken as a type parameter rather than as one instance's concrete
/// backend, so a handler keeps the property that makes it a query: it reads through
/// [`ProjectionStore`] and can reach nothing else.
/// **Send futures, declared not inferred.** Every method of this port returns
/// `impl Future<..> + Send` instead of being written `async fn`, and that is structural, not a
/// style choice: [`ServerState`] reaches this port behind an
/// instance's associated type, so the concrete future is opaque at the call site and axum's
/// handler and socket tasks — which are `Send` by construction — cannot otherwise prove it may
/// cross a thread. An `async fn` here compiles and then fails at every route that uses it. The
/// implementations stay ordinary `async fn`, which Rust accepts against this signature, and so does
/// the delegate `dyn_enum_close!` generates for a set of them: the macro emits `async fn .. -> T`
/// over the future's `Output`, because two match arms cannot unify two distinct opaque futures.
#[dyn_enum]
pub trait QueryHandler: Send + Sync {
    /// 🏷️ The [`QueryEnvelope::kind`] this handler answers.
    fn kind(&self) -> impl Future<Output = &str> + Send;

    /// 📤️ Answer one query against the read models.
    fn handle<P: ProjectionStore>(&self, envelope: &QueryEnvelope, projections: &P) -> impl Future<Output = Result<QueryResult, ServerError>> + Send;
}

/// 🕳️ The query set of an instance that registers no read handler. Uninhabited, so `queries` stays
/// empty and `POST /queries` answers [`ServerError::NotFound`] for every kind.
pub enum NoQueryHandler {}

impl QueryHandler for NoQueryHandler {
    async fn kind(&self) -> &str {
        match *self {}
    }

    async fn handle<P: ProjectionStore>(&self, _envelope: &QueryEnvelope, _projections: &P) -> Result<QueryResult, ServerError> {
        match *self {}
    }
}
//#endregion 🔖️Query
