//! 🧩️ The proctor as a `ServerInstance` of the framework server product, and the process that
//! serves it.
//!
//! **Composition.** One module (`teaching.proctor`) contributes the handle and learner deciders,
//! the enrollment saga, three policy templates and the admission of commands and presence states;
//! six [`QuizQuery`] handlers answer the reads; the four storage roles live in one SQLite file. No
//! principal resolver exists: identity without passwords is carried by the learner id inside every
//! command, so every caller is `anonymous` and the `quiz-learner` template admits exactly the four
//! command kinds, the six query kinds and the learner event streams for it. The `quiz-proctor`
//! template admits the enrollment relay for the `proctor` service account only.
//!
//! **One count, one board.** The projector keeps the number of registered learners and the
//! leaderboards' rank indexes — one per period and quiz that was asked for; the command admission
//! reads the first (the learner cap) and the leaderboard read answers from the second, as its
//! period stands at the wall clock, so neither walks a log or a table per request.
//!
//! **Presence.** The `quiz-presence` template lets every caller join, publish in and watch a
//! presence room, and it is assigned inside the catalog's room scopes only ([`Rooms`]), so a socket
//! to — or a watch of — any other scope is closed by policy. The module admits a state only when it
//! is the room's type, passes the quiz core's rules and names only ids the catalog renders; a socket
//! whose `Origin` the cross-origin policy does not admit is refused before it opens.
//!
//! **Read-your-writes.** A `POST /commands` answers only after the enrollment saga has seen the
//! events it committed and what the saga asked for has run (a relay), so the caller's next command
//! finds it done, and a `POST /queries` relays and folds the read models before it reads (a
//! settle), so a client that just submitted sees its own facts in every view. A command does not
//! wait for the fold. Both are single-flight ([`Flight`]): however many requests ask at once, one
//! run is under way and at most one more follows it. Both compare what the command lane has
//! committed ([`ServerState::committed`]) with what the last complete run covered, so a command
//! that committed nothing and a query after which nothing was committed wait for nothing and ask
//! the store nothing. Queries read the read models side by side; only a fold takes them alone. A
//! supervisor settles on a cadence as well, for work left over by a crash.
//!
//! **API only.** The proctor mounts the gateway's core routes at its origin root and nothing else
//! (no blobs, no static apps, no ephemeral frames): the site is a static build on a CDN of its own
//! origin, so every other path answers the gateway's JSON `404`, and `GET /instance` says what a
//! client addresses, never how the proctor authorizes.
//!
//! **The edge, outside in.** The request gate refuses cleartext behind a trusted proxy
//! (`403 x-semio-refusal: insecure-transport`), grants CORS — never with credentials — only to
//! admitted origins (the site origin in production) and answers every preflight itself, cacheable
//! for [`PREFLIGHT_MAX_AGE`] seconds so a quiz session does not double its `POST /commands` and
//! `POST /queries` traffic. Inside it the framework's throttle spends the caller's token and counts
//! the request in flight before anything else runs (`429`/`503` with `Retry-After`, the client
//! address being the last `X-Forwarded-For` entry behind the trusted proxy and the peer otherwise);
//! only then come the settling and the routes, whose bodies are capped and whose sockets are counted
//! and bounded by the same limits ([`Gate::limits`]). A registration spends a token of its address's
//! sign-up allowance as well ([`ProctorModule`]'s `command_allowance`: `429` naming `sign-up` when
//! there is none) and is handed it back when nothing was registered, so one address fills the
//! learner cap in weeks, not seconds. A command whose ids, handle or target are not what a quiz
//! command of its kind carries — or a registration beyond the learner cap — is refused before the
//! bus reads or places anything ([`ProctorModule`]'s `command_admission`, the [`Admission`] the
//! deciders hold as well).
//!
//! @see ../../../../🧰️framework/🛍️products/🖥️server/🔨️modules/📡️gateway/🦀️.rs — `ServerInstance`, `ServerBuilder`
//! @see ../../../../🧰️framework/🛍️products/🖥️server/🧪️tests/🧩️instance/🦀️.rs — the worked example this follows

use std::fmt;
use std::net::SocketAddr;
use std::path::Path;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Arc;
use std::time::Duration;

use axum::extract::{Request, State};
use axum::http::{header, HeaderMap, HeaderValue, Method, StatusCode, Uri};
use axum::middleware::{from_fn_with_state, Next};
use axum::response::{IntoResponse, Response};
use axum::serve::ListenerExt;
use quiz::WIRE_VERSION;
use semio_framework_async::CancelToken;
use server::contract::{CommandDescriptor, CommandEnvelope, CommandOutcome, ModuleManifest, OfflinePolicy, OpaqueJson, PolicyGrant, PolicyPoint, PolicyTemplate, QueryDescriptor, Rejection, Scope, ServerInstanceDefinition};
use server::gateway::{undelayed, ClientAddressing, GatewayRouter, InstanceDisclosure, InstanceStores, NoDocumentAuthority, PresenceSettings, RouteGroups, Server, ServerError, ServerInstance, ServerModule, ServerState, PRESENCE_JOIN, PRESENCE_PUBLISH, PRESENCE_RESOURCE, PRESENCE_WATCH};
use server::policy::{Credential, PrincipalResolver, Resolved};
use server::storage::{AuthorityStore, StorageError, StorageProfile};

use crate::actors::{deciders, unrelayed, Admission, EnrollmentSaga, ProctorDeciders, ENROLL, HANDLE, IDENTIFY, LEARNER, PROCTOR_SERVICE};
use crate::catalog::LoadedCatalog;
use crate::config::{CrossOriginPolicy, Forwarding, Gate, SIGN_UP};
use crate::presence::Rooms;
use crate::projections::{CatchUp, Progress, Projector, CROWDS, HANDLES, LEADERBOARD, LEARNERS, META, RUNS, STATES, TALLIES};
use crate::queries::{wall_clock, QueryKind, QuizQuery};
use crate::storage::{Database, SqliteAuthorityStore, SqliteBlobStore, SqliteProjectionStore, SqliteSessionStore};

/// 🏷️ The instance id `GET /instance` reports.
pub const INSTANCE_ID: &str = "teaching-proctor";
/// 🧩️ The id of the proctor's one module.
pub const MODULE_ID: &str = "teaching.proctor";
/// 🎓️ The template every caller holds.
pub const LEARNER_TEMPLATE: &str = "quiz-learner";
/// 🤖️ The template of the proctor's own service account.
pub const PROCTOR_TEMPLATE: &str = "quiz-proctor";
/// 👥️ The template every caller holds inside each presence room of the catalog.
pub const PRESENCE_TEMPLATE: &str = "quiz-presence";
/// 🙈️ The principal key of every caller, the proctor resolving no identity.
pub const ANONYMOUS: &str = "anonymous";
/// ⏱️ How often the supervisor settles sagas and projections.
pub const SETTLE_INTERVAL: Duration = Duration::from_millis(500);
/// 📦️ Outbox rows handed to the sagas per drain.
pub const DRAIN_BATCH: usize = 64;
/// 🔁️ Drains per settle before the rest is left to the next one.
pub const DRAIN_ROUNDS: usize = 64;
/// ⏳️ How long a browser may cache a granted preflight, in seconds (Chromium's ceiling).
pub const PREFLIGHT_MAX_AGE: &str = "7200";
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

/// 🧩️ The quiz lifecycle module over one loaded catalog, its presence rooms and the admission its
/// commands pass.
pub struct ProctorModule {
    catalog: Arc<LoadedCatalog>,
    rooms: Rooms,
    admission: Arc<Admission>,
}

impl ProctorModule {
    /// 🌱️ The module serving `catalog`, admitting commands through `admission`.
    pub fn new(catalog: Arc<LoadedCatalog>, admission: Arc<Admission>) -> Self {
        Self { rooms: Rooms::of(&catalog), catalog, admission }
    }

    /// 🗺️ The presence rooms of the served catalog.
    pub fn rooms(&self) -> &Rooms {
        &self.rooms
    }
}

impl ServerModule for ProctorModule {
    type Instance = ProctorInstance;

    async fn manifest(&self) -> ModuleManifest {
        manifest(self.catalog.id())
    }

    async fn deciders(&self) -> Vec<ProctorDeciders> {
        deciders(&self.catalog, &self.admission)
    }

    async fn sagas(&self) -> Vec<EnrollmentSaga> {
        vec![EnrollmentSaga]
    }

    fn presence_admission(&self, scope: &Scope, state: &OpaqueJson) -> Result<(), String> {
        self.rooms.admit(&scope.0, state)
    }

    fn command_admission(&self, envelope: &CommandEnvelope) -> Result<(), Rejection> {
        self.admission.admit(envelope)
    }

    /// 🎟️ A registration — an `identify-learner`, anonymous or claiming a handle — is counted
    /// against the sign-up allowance of its client address ([`SIGN_UP`]); nothing else is. The
    /// admission holds a command of this kind to a payload of this kind, and the relay of a claim
    /// to its learner is the proctor's own command, which no client posts.
    fn command_allowance(&self, envelope: &CommandEnvelope) -> Option<&'static str> {
        (envelope.kind == IDENTIFY).then_some(SIGN_UP)
    }
}

/// 📇️ What the proctor declares: commands, queries, projections, templates and actor kinds. A
/// registration under a pseudonym or name is decided by its handle; an anonymous one targets the
/// learner itself.
pub fn manifest(tenant: &str) -> ModuleManifest {
    let command = |kind: &str, actor_kind: &str, offline: OfflinePolicy| CommandDescriptor { kind: kind.to_string(), version: WIRE_VERSION, actor_kind: actor_kind.to_string(), offline };
    ModuleManifest {
        id: MODULE_ID.to_string(),
        commands: vec![
            command(IDENTIFY, HANDLE, OfflinePolicy::AuthorityRequired),
            command("quiz.start-run", LEARNER, OfflinePolicy::AuthorityRequired),
            command("quiz.open-task", LEARNER, OfflinePolicy::Optimistic),
            command("quiz.record-answer", LEARNER, OfflinePolicy::Optimistic),
            command("quiz.submit-run", LEARNER, OfflinePolicy::AuthorityRequired),
            command(ENROLL, LEARNER, OfflinePolicy::AuthorityRequired),
        ],
        queries: QueryKind::ALL.iter().map(|kind| QueryDescriptor { kind: kind.wire().to_string(), version: WIRE_VERSION, projection: projection_of(*kind).to_string() }).collect(),
        projections: [STATES, LEARNERS, RUNS, LEADERBOARD, HANDLES, TALLIES, CROWDS, META].map(str::to_string).to_vec(),
        policies: vec![learner_template(tenant), proctor_template(), presence_template()],
        actor_kinds: vec![HANDLE.to_string(), LEARNER.to_string()],
    }
}

fn projection_of(kind: QueryKind) -> &'static str {
    match kind {
        QueryKind::Catalog => "quiz.catalog",
        QueryKind::Learner => LEARNERS,
        QueryKind::Run => RUNS,
        QueryKind::Leaderboard => LEADERBOARD,
        QueryKind::Crowd => CROWDS,
        QueryKind::Handle => HANDLES,
    }
}

/// 🎓️ The five command kinds — a registration on a handle or on the learner itself —, the six query
/// kinds and the learner event streams. The handle streams are readable by nobody.
pub fn learner_template(tenant: &str) -> PolicyTemplate {
    let grant = |point: PolicyPoint, resource: String, action: &str| PolicyGrant { point, resource, action: action.to_string() };
    let mut grants = vec![grant(PolicyPoint::CommandAdmission, format!("{HANDLE}/*"), IDENTIFY)];
    grants.extend([IDENTIFY, "quiz.start-run", "quiz.open-task", "quiz.record-answer", "quiz.submit-run"].map(|action| grant(PolicyPoint::CommandAdmission, format!("{LEARNER}/*"), action)));
    grants.extend(QueryKind::ALL.map(|kind| grant(PolicyPoint::QueryAccess, kind.wire().to_string(), "read")));
    grants.push(grant(PolicyPoint::EventDelivery, format!("stream:{tenant}/{LEARNER}/*"), "read"));
    grants.push(grant(PolicyPoint::Subscription, format!("stream:{tenant}/{LEARNER}/*"), "subscribe"));
    PolicyTemplate { name: LEARNER_TEMPLATE.to_string(), auto_apply: true, grants }
}

/// 🤖️ The enrollment relay, for the proctor's own service account.
pub fn proctor_template() -> PolicyTemplate {
    PolicyTemplate { name: PROCTOR_TEMPLATE.to_string(), auto_apply: false, grants: vec![PolicyGrant { point: PolicyPoint::CommandAdmission, resource: format!("{LEARNER}/*"), action: ENROLL.to_string() }] }
}

/// 👥️ Joining a presence room, sharing a state in it and watching it; assigned per room scope.
pub fn presence_template() -> PolicyTemplate {
    let grant = |action: &str| PolicyGrant { point: PolicyPoint::Subscription, resource: PRESENCE_RESOURCE.to_string(), action: action.to_string() };
    PolicyTemplate { name: PRESENCE_TEMPLATE.to_string(), auto_apply: false, grants: vec![grant(PRESENCE_JOIN), grant(PRESENCE_PUBLISH), grant(PRESENCE_WATCH)] }
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
/// 🛫️ What makes a kind of work single-flight: the turn only one run holds at a time, how many runs
/// have begun, and what the last complete run left everything current at — a count of
/// [`ServerState::committed`], which no count equals before the first run.
struct Flight {
    turn: tokio::sync::Mutex<()>,
    begun: AtomicU64,
    current: AtomicU64,
}

impl Default for Flight {
    fn default() -> Self {
        Self { turn: tokio::sync::Mutex::new(()), begun: AtomicU64::new(0), current: AtomicU64::new(u64::MAX) }
    }
}

impl Flight {
    /// ✅️ Whether the last complete run left everything up to `count` done.
    fn is_current(&self, count: u64) -> bool {
        self.current.load(Ordering::Acquire) == count
    }

    /// 🤝️ Run `work` for this caller, or share a run another caller made: a caller that arrives
    /// while a run is under way waits for it, and of all the callers that waited the first runs
    /// `work` once more — a run that began after every one of them arrived, so it serves them all.
    /// However many callers arrive at once, at most two runs follow.
    async fn join<F: std::future::Future<Output = ()>>(&self, work: impl FnOnce() -> F) {
        let arrived = self.begun.load(Ordering::Acquire);
        let _turn = self.turn.lock().await;
        if self.begun.load(Ordering::Acquire) > arrived {
            return;
        }
        self.begun.fetch_add(1, Ordering::AcqRel);
        work().await;
    }
}

/// 🧮️ The shared handles settling needs: the server state, the log it commits into and the
/// projector.
#[derive(Clone)]
struct Settler {
    state: ServerState<ProctorInstance>,
    log: SqliteAuthorityStore,
    projector: Arc<Projector>,
    tenant: String,
    relaying: Arc<Flight>,
    folding: Arc<Flight>,
}

impl Settler {
    /// 🚰️ Relay and fold; how far the fold came.
    async fn settle(&self, cancel: &CancelToken, progress: impl FnMut(Progress)) -> Result<CatchUp, StorageError> {
        self.relay().await?;
        self.fold(cancel, progress, None).await
    }

    /// 🔭️ Fold every committed event into the read models, taking them from the queries meanwhile.
    /// A fold that caught up records `covers` — the count of committed events it is known to
    /// include — while it still holds the read models, so nothing that resets them is overtaken.
    async fn fold(&self, cancel: &CancelToken, progress: impl FnMut(Progress), covers: Option<u64>) -> Result<CatchUp, StorageError> {
        let mut projections = self.state.projections.write().await;
        let caught = self.projector.catch_up(&self.log, &mut projections, cancel, progress).await?;
        if let (Some(count), CatchUp::Current(_)) = (covers, &caught) {
            self.folding.current.store(count, Ordering::Release);
        }
        Ok(caught)
    }

    /// 📮️ Hand every outbox row committed before the call to the sagas and run what they ask for;
    /// whether the outbox was left empty. Relays are single-flight ([`Flight::join`]) — one that
    /// arrives while another runs waits for it, so no row is still on its way when a relay returns —
    /// and a relay that finds no outbox row committed since the last complete one asks the store
    /// nothing.
    async fn relay(&self) -> Result<bool, StorageError> {
        if self.relaying.is_current(self.state.committed().outbox) {
            return Ok(true);
        }
        let mut relayed = None;
        self.relaying.join(|| async { relayed = Some(self.relay_in_turn().await) }).await;
        match relayed {
            Some(quiet) => quiet,
            None => Ok(self.log.pending_outbox(1).await?.is_empty()),
        }
    }

    /// 🔂️ The run of one relay, while it holds the turn: drain, and record the count of committed
    /// outbox rows the drain left everything relayed at.
    async fn relay_in_turn(&self) -> Result<bool, StorageError> {
        let relayed = self.drain().await?;
        if let Some(count) = relayed {
            self.relaying.current.store(count, Ordering::Release);
        }
        Ok(relayed.is_some())
    }

    /// 🧘️ Settle on behalf of a query: everything committed before the call is relayed and folded
    /// when it returns. Concurrent callers share one settle ([`Flight::join`]), and a call that
    /// finds no event committed since the last complete settle returns at once. The settle is a
    /// task of its own: a request that stops waiting (its client hung up) cancels its wait, never a
    /// drain between acknowledging outbox rows and submitting what they ask for.
    async fn settle_quietly(&self) {
        if self.folding.is_current(self.state.committed().events) {
            return;
        }
        let settler = self.clone();
        if let Err(error) = tokio::spawn(async move { settler.folding.join(|| settler.catch_up_with_the_log()).await }).await {
            eprintln!("[ERROR] proctor settling ended abnormally: {error}");
        }
    }

    /// 📬️ Relay on behalf of a command: whatever the sagas make of the events it committed has
    /// been run when this returns, so the caller's next command finds it done. A command that
    /// committed nothing waits for nothing. The relay is a task of its own for the reason a settle
    /// is.
    async fn relay_quietly(&self) {
        if self.relaying.is_current(self.state.committed().outbox) {
            return;
        }
        let settler = self.clone();
        match tokio::spawn(async move { settler.relay().await }).await {
            Ok(Ok(_)) => {}
            Ok(Err(error)) => eprintln!("[ERROR] proctor could not relay the outbox: {error}"),
            Err(error) => eprintln!("[ERROR] proctor relaying ended abnormally: {error}"),
        }
    }

    /// 🏃️ Settle unless no event was committed since the last complete settle. The count is read
    /// before the relay: every event up to it has its outbox rows relayed and is folded when the
    /// settle ends, whatever was committed meanwhile — which the next settle then finds.
    async fn catch_up_with_the_log(&self) {
        let owed = self.state.committed().events;
        if self.folding.is_current(owed) {
            return;
        }
        let settled = async {
            let quiet = self.relay().await?;
            self.fold(&CancelToken::root_now(), |_| {}, quiet.then_some(owed)).await
        };
        if let Err(error) = settled.await {
            eprintln!("[ERROR] proctor could not settle sagas and projections: {error}");
        }
    }

    /// 🧵️ Hand the outbox to the sagas round by round — a round's own follow-ups queue rows the
    /// next round takes. When a round leaves the outbox empty: the count of outbox rows committed
    /// before that was seen, every one of which is then relayed, since only one drain runs at a
    /// time; `None` when the rounds ran out first.
    async fn drain(&self) -> Result<Option<u64>, StorageError> {
        for _ in 0..DRAIN_ROUNDS {
            let outcomes = self.state.drain_sagas(DRAIN_BATCH).await;
            if outcomes.iter().any(|outcome| matches!(outcome, CommandOutcome::Rejected { .. })) {
                eprintln!("[ERROR] proctor enrollment was refused; relaying every registration still owed");
                self.reconcile().await?;
            }
            let committed = self.state.committed().outbox;
            if self.log.pending_outbox(1).await?.is_empty() {
                return Ok(Some(committed));
            }
        }
        Ok(None)
    }

    /// 🔁️ Relay every handle registration whose learner stream is still empty; the count relayed.
    async fn reconcile(&self) -> Result<usize, StorageError> {
        let mut relayed = 0;
        for command in unrelayed(&self.log, &self.tenant).await? {
            match self.state.submit(command).await {
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
    admission: Arc<Admission>,
}

impl Proctor {
    /// 🔨️ Compose the API server over `profile` for `catalog` behind `gate` and its limits, its
    /// presence rooms ticking as `presence` says. The projector, the command admission and the
    /// leaderboard read share one learner count and one leaderboard; the quiz caps are the defaults
    /// until [`Proctor::capped`] states others.
    pub async fn assemble(profile: StorageProfile, catalog: Arc<LoadedCatalog>, gate: Gate, presence: PresenceSettings) -> Result<Self, ProctorError> {
        let view = Arc::new(serde_json::to_vec(&catalog.view()).map_err(|error| ProctorError::Io(error.to_string()))?);
        let projector = Arc::new(Projector::new(Arc::clone(&catalog)));
        let admission = Arc::new(Admission::new(catalog.id(), projector.learners()));
        let module = ProctorModule::new(Arc::clone(&catalog), Arc::clone(&admission));
        let rooms = module.rooms().scopes();
        let origins = gate.origins.clone();
        let mut builder = Server::<ProctorInstance>::builder(profile)
            .identity(INSTANCE_ID, env!("CARGO_PKG_VERSION"))
            .module(module)
            .presence(presence)
            .limits(gate.limits.clone())
            .addressing(match gate.forwarding {
                Forwarding::Untrusted => ClientAddressing::Peer,
                Forwarding::TerminatingProxy => ClientAddressing::Forwarded,
            })
            .routes(RouteGroups::CORE)
            .disclosure(InstanceDisclosure::Public)
            .origin_admission(Arc::new(move |origin: &str| origins.admits(origin)));
        for kind in QueryKind::ALL {
            builder = builder.query(QuizQuery { kind, tenant: catalog.id().to_string(), catalog: Arc::clone(&view), board: projector.board(), now: wall_clock });
        }
        let server = builder.build().await?;
        {
            let mut policy = server.state().policy.write().map_err(|_| ProctorError::Io("policy engine poisoned".to_string()))?;
            policy.assign(ANONYMOUS.to_string(), LEARNER_TEMPLATE.to_string());
            policy.assign(format!("service:{PROCTOR_SERVICE}"), PROCTOR_TEMPLATE.to_string());
            for room in rooms {
                policy.assign_scoped(ANONYMOUS.to_string(), Scope(room), PRESENCE_TEMPLATE.to_string());
            }
        }
        let log = server.state().store.read().await.clone();
        let settler = Settler { state: server.state().clone(), log, projector, tenant: catalog.id().to_string(), relaying: Arc::new(Flight::default()), folding: Arc::new(Flight::default()) };
        let served = server.router().fallback(|method: Method, uri: Uri| async move { ServerError::NotFound(format!("{method} {}", uri.path())).into_response() }).layer(from_fn_with_state(settler.clone(), consistency));
        let router = server.throttled(served).layer(from_fn_with_state(Arc::new(gate), gatekeeping));
        Ok(Self { server, settler, router, catalog, admission })
    }

    /// 🧱️ The same proctor deciding under `caps`: how many learners it registers, how many runs a
    /// learner submits and how many answers a run records.
    pub fn capped(self, caps: quiz::Limits) -> Self {
        self.admission.cap(caps);
        self
    }

    /// 🛃️ What every command passes before it is placed, with the caps in force.
    pub fn admission(&self) -> &Arc<Admission> {
        &self.admission
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

    /// 🧾️ Set up fresh projections and reset those built for another catalog or projector revision;
    /// `true` when such projections were dropped.
    pub async fn prepare(&self) -> Result<bool, StorageError> {
        let mut projections = self.settler.state.projections.write().await;
        self.settler.folding.current.store(u64::MAX, Ordering::Release);
        self.settler.projector.prepare(&mut projections).await
    }

    /// 🧹️ Drop every read model so the next settle rebuilds them from the first event.
    pub async fn reset_projections(&self) -> Result<(), StorageError> {
        let mut projections = self.settler.state.projections.write().await;
        self.settler.folding.current.store(u64::MAX, Ordering::Release);
        self.settler.projector.reset(&mut projections).await
    }

    /// 🔁️ Relay every handle registration whose learner never received it; the count of relays that
    /// were missing.
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
        let served = axum::serve(listener.tap_io(undelayed), proctor.router.clone().into_make_service_with_connect_info::<SocketAddr>()).with_graceful_shutdown(async move { stop.cancelled().await }).await;
        shutdown.cancel_now();
        let _ = supervisor.await;
        proctor.settler.settle_quietly().await;
        served.map_err(|error| ProctorError::Io(error.to_string()))
    }
}
//#endregion 🔖️Proctor

//#region 🔖️Middleware
/// 🔁️ Read-your-writes around the routes: a query is answered from read models that hold
/// everything committed before it arrived, and a command answers once what it queued for the sagas
/// has run.
async fn consistency(State(settler): State<Settler>, request: Request, next: Next) -> Response {
    let posted = request.method() == Method::POST;
    let (queries, commands) = (posted && request.uri().path() == "/queries", posted && request.uri().path() == "/commands");
    if queries {
        settler.settle_quietly().await;
    }
    let response = next.run(request).await;
    if commands {
        settler.relay_quietly().await;
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
        return preflight(origin.as_ref(), &gate.origins);
    }
    let mut response = next.run(request).await;
    grant(response.headers_mut(), origin.as_ref(), &gate.origins);
    response
}

/// ✈️ The answer to a CORS preflight: `204` with the grant, and — when the origin is admitted —
/// `Access-Control-Max-Age` so the browser reuses it instead of asking before every request.
fn preflight(origin: Option<&HeaderValue>, policy: &CrossOriginPolicy) -> Response {
    let mut response = StatusCode::NO_CONTENT.into_response();
    if grant(response.headers_mut(), origin, policy) {
        response.headers_mut().insert(header::ACCESS_CONTROL_MAX_AGE, HeaderValue::from_static(PREFLIGHT_MAX_AGE));
    }
    response
}

/// 🌍️ Write the cross-origin grant of one response: the caller's own origin only when the policy
/// admits it — never with credentials, the client sends none — together with leave to read
/// `Retry-After`, and `Vary: Origin` whenever an origin was presented; `true` when granted.
fn grant(headers: &mut HeaderMap, origin: Option<&HeaderValue>, policy: &CrossOriginPolicy) -> bool {
    headers.remove(header::ACCESS_CONTROL_ALLOW_ORIGIN);
    headers.remove(header::ACCESS_CONTROL_ALLOW_CREDENTIALS);
    let admitted = origin.filter(|origin| origin.to_str().is_ok_and(|value| policy.admits(value)));
    if origin.is_some() {
        headers.insert(header::VARY, HeaderValue::from_static("Origin"));
    }
    if let Some(origin) = admitted {
        headers.insert(header::ACCESS_CONTROL_ALLOW_ORIGIN, origin.clone());
        headers.insert(header::ACCESS_CONTROL_EXPOSE_HEADERS, HeaderValue::from_static("retry-after"));
    }
    headers.insert(header::ACCESS_CONTROL_ALLOW_METHODS, HeaderValue::from_static("GET, POST, HEAD, OPTIONS"));
    headers.insert(header::ACCESS_CONTROL_ALLOW_HEADERS, HeaderValue::from_static("content-type"));
    admitted.is_some()
}
//#endregion 🔖️Middleware

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
