//! 🧩️ The proctor as a `ServerInstance` of the framework server product, and the process that
//! serves it.
//!
//! **Composition.** One module (`teaching.proctor`) contributes the roster and learner deciders,
//! the enrollment saga and two policy templates; four [`QuizQuery`] handlers answer the reads; the
//! four storage roles live in one SQLite file. No principal resolver exists: identity without
//! passwords is carried by the learner id inside every command, so every caller is `anonymous`
//! and the `quiz-learner` template admits exactly the four command kinds, the four query kinds and
//! the learner event streams for it. The `quiz-proctor` template admits the enrollment relay for
//! the `proctor` service account only.
//!
//! **Read-your-writes.** A `POST /commands` answers only after the enrollment saga has run and the
//! projections have caught up, and a `POST /queries` catches the projections up before it reads,
//! so a client that just submitted sees its own facts in every view. A supervisor repeats the same
//! settling on a cadence for work left over by a crash.
//!
//! **Routing.** The gateway's routes are served at the origin root; every other GET or HEAD is the
//! built site (single-page fallback) and every other method there is `405 Allow: GET, HEAD`. Outermost, the request gate refuses cleartext behind a trusted
//! proxy (`403 x-semio-refusal: insecure-transport`) and grants CORS only to admitted origins.
//!
//! @see ../../../../🧰️framework/🛍️products/🖥️server/🔨️modules/📡️gateway/🦀️.rs — `ServerInstance`, `ServerBuilder`
//! @see ../../../../🧰️framework/🛍️products/🖥️server/🧪️tests/🧩️instance/🦀️.rs — the worked example this follows

use std::fmt;
use std::net::SocketAddr;
use std::path::Path;
use std::sync::Arc;
use std::time::Duration;

use axum::extract::{Request, State};
use axum::http::{header, HeaderMap, HeaderValue, Method, StatusCode, Uri};
use axum::middleware::{from_fn_with_state, Next};
use axum::response::{IntoResponse, Response};
use semio_framework_async::CancelToken;
use server::contract::{CommandDescriptor, CommandOutcome, ModuleManifest, OfflinePolicy, PolicyGrant, PolicyPoint, PolicyTemplate, QueryDescriptor, ServerInstanceDefinition};
use server::gateway::{GatewayRouter, InstanceStores, NoDocumentAuthority, Server, ServerError, ServerInstance, ServerModule, ServerState};
use server::policy::{Credential, PrincipalResolver, Resolved};
use server::storage::{AuthorityStore, StorageError, StorageProfile};

use crate::actors::{enrollment, roster_key, EnrollmentSaga, LearnerDecider, ProctorDeciders, RosterDecider, ENROLL, LEARNER, PROCTOR_SERVICE, ROSTER, ROSTER_ID, WIRE_VERSION};
use crate::catalog::LoadedCatalog;
use crate::config::{CrossOriginPolicy, Gate};
use crate::projections::{CatchUp, Progress, Projector, LEADERBOARD, LEARNERS, META, RUNS, STATES};
use crate::queries::{QueryKind, QuizQuery};
use crate::site::{method_not_allowed, SiteHost};
use crate::storage::{Database, SqliteAuthorityStore, SqliteBlobStore, SqliteProjectionStore, SqliteSessionStore};

/// 🏷️ The instance id `GET /instance` reports.
pub const INSTANCE_ID: &str = "teaching-proctor";
/// 🧩️ The id of the proctor's one module.
pub const MODULE_ID: &str = "teaching.proctor";
/// 🎓️ The template every caller holds.
pub const LEARNER_TEMPLATE: &str = "quiz-learner";
/// 🤖️ The template of the proctor's own service account.
pub const PROCTOR_TEMPLATE: &str = "quiz-proctor";
/// ⏱️ How often the supervisor settles sagas and projections.
pub const SETTLE_INTERVAL: Duration = Duration::from_millis(500);
/// 📦️ Outbox rows handed to the sagas per drain.
pub const DRAIN_BATCH: usize = 64;
/// 🔁️ Drains per settle before the rest is left to the next one.
pub const DRAIN_ROUNDS: usize = 64;
/// 🚧️ The response header naming why the gate refused a request.
pub const REFUSAL_HEADER: &str = "x-semio-refusal";

//#region 🔖️Instance
/// 🛂️ The proctor deployment of the server product.
pub struct ProctorInstance;

impl ServerInstance for ProctorInstance {
    type Modules = ProctorModule;
    type Queries = QuizQuery;
    type Documents = NoDocumentAuthority;
    type Deciders = ProctorDeciders;
    type Sagas = EnrollmentSaga;
    type Resolvers = ProctorResolvers;
    type AuthorityStore = SqliteAuthorityStore;
    type ProjectionStore = SqliteProjectionStore;
    type BlobStore = SqliteBlobStore;
    type SessionStore = SqliteSessionStore;

    /// 🗄️ One SQLite file under the profile's directory, or a private in-memory database.
    async fn open(profile: &StorageProfile) -> Result<InstanceStores<Self>, StorageError> {
        let database = match profile.data_dir() {
            Some(directory) => Database::open(Path::new(directory))?,
            None => Database::memory()?,
        };
        Ok(InstanceStores { authority: SqliteAuthorityStore::new(database.clone()), projections: SqliteProjectionStore::new(database.clone()), blobs: SqliteBlobStore::new(database.clone()), sessions: SqliteSessionStore::new(database) })
    }
}

/// 🕳️ The proctor resolves no principal: identity lives in the commands.
pub enum ProctorResolvers {}

impl PrincipalResolver for ProctorResolvers {
    async fn resolve(&self, _credential: &Credential) -> Option<Resolved> {
        match *self {}
    }

    async fn name(&self) -> &str {
        match *self {}
    }
}

/// 🧩️ The quiz lifecycle module over one loaded catalog.
pub struct ProctorModule {
    pub catalog: Arc<LoadedCatalog>,
}

impl ServerModule for ProctorModule {
    type Instance = ProctorInstance;

    async fn manifest(&self) -> ModuleManifest {
        manifest(self.catalog.id())
    }

    async fn deciders(&self) -> Vec<ProctorDeciders> {
        vec![ProctorDeciders::Roster(RosterDecider { tenant: self.catalog.id().to_string() }), ProctorDeciders::Learner(LearnerDecider { catalog: Arc::clone(&self.catalog) })]
    }

    async fn sagas(&self) -> Vec<EnrollmentSaga> {
        vec![EnrollmentSaga]
    }
}

/// 📇️ What the proctor declares: commands, queries, projections, templates and actor kinds.
pub fn manifest(tenant: &str) -> ModuleManifest {
    let command = |kind: &str, actor_kind: &str, offline: OfflinePolicy| CommandDescriptor { kind: kind.to_string(), version: WIRE_VERSION, actor_kind: actor_kind.to_string(), offline };
    ModuleManifest {
        id: MODULE_ID.to_string(),
        commands: vec![
            command("quiz.identify-learner", ROSTER, OfflinePolicy::AuthorityRequired),
            command("quiz.start-run", LEARNER, OfflinePolicy::AuthorityRequired),
            command("quiz.record-answer", LEARNER, OfflinePolicy::Optimistic),
            command("quiz.submit-run", LEARNER, OfflinePolicy::AuthorityRequired),
            command(ENROLL, LEARNER, OfflinePolicy::AuthorityRequired),
        ],
        queries: QueryKind::ALL.iter().map(|kind| QueryDescriptor { kind: kind.wire().to_string(), version: WIRE_VERSION, projection: projection_of(*kind).to_string() }).collect(),
        projections: [STATES, LEARNERS, RUNS, LEADERBOARD, META].map(str::to_string).to_vec(),
        policies: vec![learner_template(tenant), proctor_template()],
        actor_kinds: vec![ROSTER.to_string(), LEARNER.to_string()],
    }
}

fn projection_of(kind: QueryKind) -> &'static str {
    match kind {
        QueryKind::Catalog => "quiz.catalog",
        QueryKind::Learner => LEARNERS,
        QueryKind::Run => RUNS,
        QueryKind::Leaderboard => LEADERBOARD,
    }
}

/// 🎓️ The four command kinds, the four query kinds and the learner event streams.
pub fn learner_template(tenant: &str) -> PolicyTemplate {
    let grant = |point: PolicyPoint, resource: String, action: &str| PolicyGrant { point, resource, action: action.to_string() };
    let mut grants = vec![grant(PolicyPoint::CommandAdmission, format!("{ROSTER}/{ROSTER_ID}"), "quiz.identify-learner")];
    grants.extend(["quiz.start-run", "quiz.record-answer", "quiz.submit-run"].map(|action| grant(PolicyPoint::CommandAdmission, format!("{LEARNER}/*"), action)));
    grants.extend(QueryKind::ALL.map(|kind| grant(PolicyPoint::QueryAccess, kind.wire().to_string(), "read")));
    grants.push(grant(PolicyPoint::EventDelivery, format!("stream:{tenant}/{LEARNER}/*"), "read"));
    grants.push(grant(PolicyPoint::Subscription, format!("stream:{tenant}/{LEARNER}/*"), "subscribe"));
    PolicyTemplate { name: LEARNER_TEMPLATE.to_string(), auto_apply: true, grants }
}

/// 🤖️ The enrollment relay, for the proctor's own service account.
pub fn proctor_template() -> PolicyTemplate {
    PolicyTemplate { name: PROCTOR_TEMPLATE.to_string(), auto_apply: false, grants: vec![PolicyGrant { point: PolicyPoint::CommandAdmission, resource: format!("{LEARNER}/*"), action: ENROLL.to_string() }] }
}
//#endregion 🔖️Instance

//#region 🔖️Error
/// 🧯️ Why the proctor could not start or keep serving.
#[derive(Debug)]
pub enum ProctorError {
    Storage(StorageError),
    Server(ServerError),
    Io(String),
}

impl fmt::Display for ProctorError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Storage(error) => write!(formatter, "{error}"),
            Self::Server(error) => write!(formatter, "{error}"),
            Self::Io(detail) => formatter.write_str(detail),
        }
    }
}

impl std::error::Error for ProctorError {}

impl From<StorageError> for ProctorError {
    fn from(error: StorageError) -> Self {
        Self::Storage(error)
    }
}

impl From<ServerError> for ProctorError {
    fn from(error: ServerError) -> Self {
        Self::Server(error)
    }
}
//#endregion 🔖️Error

//#region 🔖️Proctor
/// 🧮️ The shared handles settling needs: the server state and the projector.
#[derive(Clone)]
struct Settler {
    state: ServerState<ProctorInstance>,
    projector: Arc<Projector>,
    tenant: String,
}

impl Settler {
    async fn settle(&self, cancel: &CancelToken, progress: impl FnMut(Progress)) -> Result<CatchUp, StorageError> {
        self.drain().await?;
        let log = self.state.authority.lock().await.store().clone();
        let mut projections = self.state.projections.lock().await;
        self.projector.catch_up(&log, &mut projections, cancel, progress).await
    }

    async fn settle_quietly(&self) {
        if let Err(error) = self.settle(&CancelToken::root_now(), |_| {}).await {
            eprintln!("[ERROR] proctor could not settle sagas and projections: {error}");
        }
    }

    async fn drain(&self) -> Result<(), StorageError> {
        for _ in 0..DRAIN_ROUNDS {
            let outcomes = self.state.drain_sagas(DRAIN_BATCH).await;
            if outcomes.iter().any(|outcome| matches!(outcome, CommandOutcome::Rejected { .. })) {
                eprintln!("[ERROR] proctor enrollment was refused; reconciling the roster");
                self.reconcile().await?;
            }
            if self.state.authority.lock().await.store().pending_outbox(1).await?.is_empty() {
                break;
            }
        }
        Ok(())
    }

    async fn reconcile(&self) -> Result<usize, StorageError> {
        let roster = self.state.authority.lock().await.store().events_since(&roster_key(&self.tenant), 0).await?;
        let mut relayed = 0;
        for command in roster.iter().filter_map(enrollment) {
            let now = self.state.now();
            match self.state.authority.lock().await.submit(command, now).await {
                CommandOutcome::Accepted { events, .. } if !events.is_empty() => relayed += 1,
                CommandOutcome::Rejected { reason, .. } => eprintln!("[ERROR] proctor enrollment refused: {reason:?}"),
                _ => {}
            }
        }
        Ok(relayed)
    }
}

/// 🛂️ One assembled proctor: the framework server, its routes and its projector.
pub struct Proctor {
    server: Server<ProctorInstance>,
    settler: Settler,
    router: GatewayRouter,
    catalog: Arc<LoadedCatalog>,
}

impl Proctor {
    /// 🔨️ Compose the server over `profile` for `catalog`, hosting `site` for every other GET.
    pub async fn assemble(profile: StorageProfile, catalog: Arc<LoadedCatalog>, gate: Gate, site: Option<SiteHost>) -> Result<Self, ProctorError> {
        let view = Arc::new(serde_json::to_vec(&catalog.view()).map_err(|error| ProctorError::Io(error.to_string()))?);
        let mut builder = Server::<ProctorInstance>::builder(profile).identity(INSTANCE_ID, env!("CARGO_PKG_VERSION")).module(ProctorModule { catalog: Arc::clone(&catalog) });
        for kind in QueryKind::ALL {
            builder = builder.query(QuizQuery { kind, tenant: catalog.id().to_string(), catalog: Arc::clone(&view) });
        }
        let server = builder.build().await?;
        {
            let mut policy = server.state().policy.write().map_err(|_| ProctorError::Io("policy engine poisoned".to_string()))?;
            policy.assign("anonymous".to_string(), LEARNER_TEMPLATE.to_string());
            policy.assign(format!("service:{PROCTOR_SERVICE}"), PROCTOR_TEMPLATE.to_string());
        }
        let settler = Settler { state: server.state().clone(), projector: Arc::new(Projector::new(Arc::clone(&catalog))), tenant: catalog.id().to_string() };
        let site = site.map(Arc::new);
        let router = server
            .router()
            .fallback(move |method: Method, uri: Uri| {
                let site = site.clone();
                async move {
                    match site {
                        Some(site) => site.serve(&method, uri.path()).await,
                        None if method != Method::GET && method != Method::HEAD => method_not_allowed(&method, uri.path()),
                        None => ServerError::NotFound(format!("{method} {}", uri.path())).into_response(),
                    }
                }
            })
            .layer(from_fn_with_state(settler.clone(), consistency))
            .layer(from_fn_with_state(Arc::new(gate), gatekeeping));
        Ok(Self { server, settler, router, catalog })
    }

    /// 🧠️ The framework state every handler runs against.
    pub fn state(&self) -> &ServerState<ProctorInstance> {
        self.server.state()
    }

    /// 🏛️ What the instance declares itself to be.
    pub fn definition(&self) -> &ServerInstanceDefinition {
        self.server.definition()
    }

    /// 📚️ The catalog being served.
    pub fn catalog(&self) -> &LoadedCatalog {
        &self.catalog
    }

    /// 🧾️ Reset the projections when they were built against another catalog; `true` when reset.
    pub async fn prepare(&self) -> Result<bool, StorageError> {
        let mut projections = self.settler.state.projections.lock().await;
        self.settler.projector.prepare(&mut projections).await
    }

    /// 🧹️ Drop every read model so the next settle rebuilds them from the first event.
    pub async fn reset_projections(&self) -> Result<(), StorageError> {
        let mut projections = self.settler.state.projections.lock().await;
        self.settler.projector.reset(&mut projections).await
    }

    /// 🔁️ Relay every roster fact to its learner again; the count of relays that were missing.
    pub async fn reconcile(&self) -> Result<usize, StorageError> {
        self.settler.reconcile().await
    }

    /// 🚰️ Run the sagas until the outbox is quiet and fold every committed event into the
    /// projections, reporting progress and stopping between batches once `cancel` fires.
    pub async fn settle(&self, cancel: &CancelToken, progress: impl FnMut(Progress)) -> Result<CatchUp, StorageError> {
        self.settler.settle(cancel, progress).await
    }

    /// 🔌️ Bind `address` (port `0` picks a free port).
    pub async fn bind(self, address: SocketAddr) -> Result<BoundProctor, ProctorError> {
        let listener = tokio::net::TcpListener::bind(address).await.map_err(|error| ProctorError::Io(format!("cannot bind {address}: {error}")))?;
        let local = listener.local_addr().map_err(|error| ProctorError::Io(error.to_string()))?;
        Ok(BoundProctor { proctor: self, listener, local })
    }
}

/// 🔌️ A proctor bound to its socket, ready to serve.
pub struct BoundProctor {
    proctor: Proctor,
    listener: tokio::net::TcpListener,
    local: SocketAddr,
}

impl BoundProctor {
    /// 📍️ The address actually bound.
    pub fn local_addr(&self) -> SocketAddr {
        self.local
    }

    /// ▶️ Serve until `shutdown` fires, then finish in-flight requests and settle once more.
    pub async fn serve(self, shutdown: CancelToken) -> Result<(), ProctorError> {
        let Self { proctor, listener, .. } = self;
        let supervisor = {
            let settler = proctor.settler.clone();
            let stop = shutdown.clone();
            tokio::spawn(async move {
                loop {
                    tokio::select! {
                        _ = tokio::time::sleep(SETTLE_INTERVAL) => settler.settle_quietly().await,
                        _ = stop.cancelled() => return,
                    }
                }
            })
        };
        let stop = shutdown.clone();
        let served = axum::serve(listener, proctor.router.clone().into_make_service_with_connect_info::<SocketAddr>()).with_graceful_shutdown(async move { stop.cancelled().await }).await;
        shutdown.cancel_now();
        let _ = supervisor.await;
        proctor.settler.settle_quietly().await;
        served.map_err(|error| ProctorError::Io(error.to_string()))
    }
}
//#endregion 🔖️Proctor

//#region 🔖️Middleware
async fn consistency(State(settler): State<Settler>, request: Request, next: Next) -> Response {
    let posted = request.method() == Method::POST;
    let path = request.uri().path().to_string();
    if posted && path == "/queries" {
        settler.settle_quietly().await;
    }
    let response = next.run(request).await;
    if posted && path == "/commands" {
        settler.settle_quietly().await;
    }
    response
}

async fn gatekeeping(State(gate): State<Arc<Gate>>, request: Request, next: Next) -> Response {
    let proto = request.headers().get("x-forwarded-proto").and_then(|value| value.to_str().ok());
    if !gate.forwarding.secure(proto) {
        let mut response = StatusCode::FORBIDDEN.into_response();
        response.headers_mut().insert(REFUSAL_HEADER, HeaderValue::from_static("insecure-transport"));
        return response;
    }
    let origin = request.headers().get(header::ORIGIN).cloned();
    if request.method() == Method::OPTIONS {
        let mut response = StatusCode::NO_CONTENT.into_response();
        grant(response.headers_mut(), origin.as_ref(), &gate.origins);
        return response;
    }
    let mut response = next.run(request).await;
    grant(response.headers_mut(), origin.as_ref(), &gate.origins);
    response
}

/// 🌍️ Write the cross-origin grant of one response: the caller's own origin and credentials only
/// when the policy admits it, `Vary: Origin` whenever an origin was presented.
fn grant(headers: &mut HeaderMap, origin: Option<&HeaderValue>, policy: &CrossOriginPolicy) {
    headers.remove(header::ACCESS_CONTROL_ALLOW_ORIGIN);
    headers.remove(header::ACCESS_CONTROL_ALLOW_CREDENTIALS);
    if let Some(origin) = origin {
        headers.insert(header::VARY, HeaderValue::from_static("Origin"));
        if origin.to_str().is_ok_and(|value| policy.admits(value)) {
            headers.insert(header::ACCESS_CONTROL_ALLOW_ORIGIN, origin.clone());
            headers.insert(header::ACCESS_CONTROL_ALLOW_CREDENTIALS, HeaderValue::from_static("true"));
        }
    }
    headers.insert(header::ACCESS_CONTROL_ALLOW_METHODS, HeaderValue::from_static("GET, POST, HEAD, OPTIONS"));
    headers.insert(header::ACCESS_CONTROL_ALLOW_HEADERS, HeaderValue::from_static("content-type"));
}
//#endregion 🔖️Middleware

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
