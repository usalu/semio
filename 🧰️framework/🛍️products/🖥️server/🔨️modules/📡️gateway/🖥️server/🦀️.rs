//! 🖥️ The route groups, the builder and the running server.

use super::*;

//#region 🔖️Server
/// 🛣️ Which of the gateway's own route groups an instance mounts. The core — the instance document,
/// commands, queries, the presence socket and the durable lane — is always mounted; a group an
/// instance does not use is not a route it answers `403` on but a path that does not exist, so it
/// reads no body, asks no policy and is no surface.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct RouteGroups {
    /// 💨️ `POST /scopes/{scope}/ephemeral`.
    pub ephemeral: bool,
    /// 🧱️ `GET`, `HEAD` and `PUT /blobs/{hash}`.
    pub blobs: bool,
    /// 🧩️ `GET /apps` and everything under it.
    pub apps: bool,
}

impl RouteGroups {
    /// 🌐️ Every group: the router of an instance that hosts blobs, static apps and ephemeral frames.
    pub const ALL: Self = Self { ephemeral: true, blobs: true, apps: true };
    /// 🎯️ The core alone: the router of an instance that is commands, queries and sockets.
    pub const CORE: Self = Self { ephemeral: false, blobs: false, apps: false };
}

/// 🪟️ How much of its definition an instance hands to whoever asks `GET /instance`.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum InstanceDisclosure {
    /// 📖️ The whole definition, policy templates included: right for an instance only its own
    /// operators reach.
    #[default]
    Whole,
    /// 🪟️ [`ServerInstanceDefinition::public`]: what a client needs to address the instance and
    /// nothing of how it authorizes.
    Public,
}

/// 🏗️ Assembles one server out of a storage profile and one [`ServerInstance`]'s modules.
pub struct ServerBuilder<I: ServerInstance> {
    pub(super) profile: StorageProfile,
    pub(super) modules: Vec<I::Modules>,
    pub(super) queries: Vec<I::Queries>,
    pub(super) sagas: Vec<I::Sagas>,
    pub(super) apps: Vec<(String, PathBuf)>,
    pub(super) documents: Option<Arc<I::Documents>>,
    pub(super) admin_token: Option<String>,
    pub(super) presence: PresenceSettings,
    pub(super) origin_admission: Option<OriginAdmission>,
    pub(super) limits: Limits,
    pub(super) addressing: ClientAddressing,
    pub(super) routes: RouteGroups,
    pub(super) disclosure: InstanceDisclosure,
    activations: usize,
    snapshot_interval: u64,
    pub(super) id: String,
    pub(super) version: String,
}

impl<I: ServerInstance> ServerBuilder<I> {
    /// ⏱️ Tune the presence socket (tick, idle timeout, keepalive, limits).
    pub fn presence(mut self, settings: PresenceSettings) -> Self {
        self.presence = settings;
        self
    }

    /// 🌍️ Name the browser origins that may open a presence socket.
    pub fn origin_admission(mut self, admits: OriginAdmission) -> Self {
        self.origin_admission = Some(admits);
        self
    }

    /// 🚦️ Bound the edge: rates per client address, sockets, requests in flight and the request
    /// body. An instance that sets none bounds nothing ([`Limits::open`]) — the shape of one that
    /// sits behind an edge of its own; a public instance passes its sizing, [`Limits::default`]
    /// being the one for a class of three hundred behind one address.
    pub fn limits(mut self, limits: Limits) -> Self {
        self.limits = limits;
        self
    }

    /// 🧭️ Say where a client's address comes from; the peer of the connection unless stated.
    pub fn addressing(mut self, addressing: ClientAddressing) -> Self {
        self.addressing = addressing;
        self
    }

    /// 🛣️ Mount only these of the gateway's own route groups; every group unless stated.
    pub fn routes(mut self, groups: RouteGroups) -> Self {
        self.routes = groups;
        self
    }

    /// 🪟️ Choose what `GET /instance` hands out; the whole definition unless stated.
    pub fn disclosure(mut self, disclosure: InstanceDisclosure) -> Self {
        self.disclosure = disclosure;
        self
    }

    /// 🧮️ Keep at most `capacity` actors placed at once ([`DIRECTORY_CAPACITY`] unless stated).
    pub fn activations(mut self, capacity: usize) -> Self {
        self.activations = capacity;
        self
    }

    /// 📸️ Snapshot an actor every `interval` committed events ([`SNAPSHOT_INTERVAL`] unless stated;
    /// `0` writes none).
    pub fn snapshots(mut self, interval: u64) -> Self {
        self.snapshot_interval = interval;
        self
    }

    /// 🧩️ Register one module. Its deciders, templates, resolvers and routes are collected at
    /// [`build`](Self::build) time, in registration order.
    pub fn module(mut self, module: I::Modules) -> Self {
        self.modules.push(module);
        self
    }

    /// ❓️ Register one query handler under the kind it declares.
    pub fn query(mut self, handler: I::Queries) -> Self {
        self.queries.push(handler);
        self
    }

    /// 🧵️ Register one cross-actor workflow on this instance's saga runner, for a workflow that
    /// belongs to the deployment rather than to one of its modules.
    pub fn saga(mut self, saga: I::Sagas) -> Self {
        self.sagas.push(saga);
        self
    }

    /// 📄️ Supply the replication engine backing the document websocket.
    pub fn document_authority(mut self, documents: Arc<I::Documents>) -> Self {
        self.documents = Some(documents);
        self
    }

    /// 🗂️ Host a directory as a static app under `name`.
    pub fn app(mut self, name: &str, dir: impl Into<PathBuf>) -> Self {
        self.apps.push((name.to_string(), dir.into()));
        self
    }

    /// 🚪️ Configure the administration gate. `None` leaves it in loopback-only mode.
    pub fn admin_token(mut self, token: Option<String>) -> Self {
        self.admin_token = token;
        self
    }

    /// 🏷️ Name this instance, as reported by `GET /instance`.
    pub fn identity(mut self, id: &str, version: &str) -> Self {
        self.id = id.to_string();
        self.version = version.to_string();
        self
    }

    /// 🔨️ Collect every module's contribution into one shared engine, bus, ladder and router, over
    /// the storage the instance opens for this profile.
    pub async fn build(self) -> Result<Server<I>, ServerError> {
        let policy = Arc::new(RwLock::new(PolicyEngine::new()));
        let mut chain = ResolverChain::<I::Resolvers>::new();
        let mut definition = ServerInstanceDefinition { id: self.id.clone(), version: self.version.clone(), modules: Vec::new() };
        let mut deciders: Vec<I::Deciders> = Vec::new();
        let mut workflows: Vec<I::Sagas> = Vec::new();
        let modules = Arc::new(self.modules);
        for module in modules.iter() {
            let manifest = module.manifest().await;
            let templates: Vec<PolicyTemplate> = manifest.policies.iter().cloned().chain(module.templates().await).collect();
            if let Ok(mut engine) = policy.write() {
                for template in templates {
                    if template.auto_apply {
                        engine.set_authenticated_template(template.name.clone());
                    }
                    engine.register_template(template);
                }
            }
            definition.modules.push(manifest);
            deciders.extend(module.deciders().await);
            workflows.extend(module.sagas().await);
            for resolver in module.resolvers().await {
                chain.push(resolver);
            }
        }

        let hook: PolicyHook = {
            let policy = Arc::clone(&policy);
            Box::new(move |envelope: &CommandEnvelope| match policy.read() {
                Ok(engine) => engine.evaluate(&admission_request(envelope)),
                Err(_) => PolicyDecision::Deny { reason: "policy engine poisoned".to_string() },
            })
        };
        let admission: AdmissionHook = {
            let modules = Arc::clone(&modules);
            Box::new(move |envelope: &CommandEnvelope| modules.iter().try_for_each(|module| module.command_admission(envelope)))
        };

        let stores = I::open(&self.profile).await?;
        let mut bus = CommandBus::new(AuthorityDirectory::bounded(self.activations), stores.authority, hook).admitting(admission).snapshotting(self.snapshot_interval);
        for decider in deciders {
            bus.register(decider).await;
        }
        let mut runner = ServerSagas::<I>::new();
        for saga in workflows.into_iter().chain(self.sagas) {
            runner.register(saga);
        }

        let apps = AppRegistry::new();
        for (name, dir) in &self.apps {
            apps.register(name, dir.clone());
        }
        let queries: ShardedMap<String, Arc<I::Queries>> = ShardedMap::new();
        for handler in self.queries {
            queries.insert(handler.kind().await.to_string(), Arc::new(handler));
        }

        let state = ServerState {
            store: Arc::clone(bus.store()),
            authority: Arc::new(Mutex::new(bus)),
            sagas: Arc::new(Mutex::new(runner)),
            projections: Arc::new(tokio::sync::RwLock::new(stores.projections)),
            blobs: Arc::new(Mutex::new(stores.blobs)),
            sessions: Arc::new(Mutex::new(stores.sessions)),
            policy,
            resolvers: Arc::new(chain),
            admin: Arc::new(AdminGate::new(self.admin_token.clone())),
            fanout: Arc::new(Fanout::new()),
            presence: Arc::new(Presence::new()),
            kicks: Arc::new(KickMap::new()),
            apps: Arc::new(apps),
            queries: Arc::new(queries),
            documents: self.documents.clone(),
            profile: Arc::new(self.profile.clone()),
            modules: Arc::clone(&modules),
            presence_rooms: Arc::new(PresenceRooms::new(&self.presence, Instant::now())),
            presence_settings: Arc::new(self.presence),
            origin_admission: self.origin_admission,
            throttle: Throttle::new(self.limits.clone(), std::time::Instant::now()),
            addressing: self.addressing,
            lane: Arc::new(CommandLane::default()),
            clock: Arc::new(StdMutex::new(HybridLogicalClock::default())),
        };

        let disclosed = match self.disclosure {
            InstanceDisclosure::Whole => definition.clone(),
            InstanceDisclosure::Public => definition.public(),
        };
        let mut router = base_router(disclosed, self.routes, self.documents.is_some());
        for module in modules.iter() {
            router = module.routes(router).await;
        }
        if let Some(bytes) = self.limits.body_bytes {
            router = router.layer(DefaultBodyLimit::max(bytes));
        }
        let router = router.layer(axum::middleware::from_fn(cors_middleware)).with_state(state.clone());
        Ok(Server { state, router, definition })
    }
}

/// 🚦️ The admission question one command envelope asks of the policy engine.
pub(super) fn admission_request(envelope: &CommandEnvelope) -> PolicyRequest {
    PolicyRequest { point: PolicyPoint::CommandAdmission, principal: envelope.principal.clone(), scope: Some(envelope.scope.clone()), resource: format!("{}/{}", envelope.target.kind, envelope.target.id), action: envelope.kind.clone() }
}

/// 🛣️ Every route the framework itself owns, before any module adds its own. Each handler is
/// instantiated at the instance being built, so a route is monomorphic even though the set of
/// backends behind it is chosen downstream. The core is always mounted; each further group only for
/// an instance that asked for it ([`RouteGroups`]), and the document socket only for an instance
/// that registered a [`DocumentAuthority`], leaving the path to an instance that owns it itself.
fn base_router<I: ServerInstance>(definition: ServerInstanceDefinition, groups: RouteGroups, hosts_documents: bool) -> Router<ServerState<I>> {
    let mut router = Router::new()
        .route("/instance", get(move || instance_body(definition.clone())))
        .route("/commands", post(post_command::<I>))
        .route("/queries", post(post_query::<I>))
        .route("/scopes/{scope}/presence/ws", get(get_presence_ws::<I>))
        .route("/actors/{tenant}/{kind}/{id}/events", get(get_events::<I>))
        .route("/actors/{tenant}/{kind}/{id}/events/ws", get(get_event_stream_ws::<I>));
    if groups.ephemeral {
        router = router.route("/scopes/{scope}/ephemeral", post(post_ephemeral::<I>));
    }
    if groups.blobs {
        router = router.route("/blobs/{hash}", get(get_blob::<I>).head(head_blob::<I>).put(put_blob::<I>));
    }
    if groups.apps {
        router = router
            .route("/apps", get(get_apps::<I>))
            .route("/apps/{app}/installs", get(get_app_installs::<I>))
            .route("/apps/{app}", get(get_app_root::<I>))
            .route("/apps/{app}/{*rest}", get(get_app_asset::<I>));
    }
    if hosts_documents {
        router = router.route("/scopes/{scope}/document/ws", get(get_document_ws::<I>));
    }
    router
}

async fn instance_body(definition: ServerInstanceDefinition) -> Json<ServerInstanceDefinition> {
    Json(definition)
}

/// 🖥️ One built server: its shared state, its router and the definition it reports.
pub struct Server<I: ServerInstance> {
    pub(super) state: ServerState<I>,
    pub(super) router: Router,
    pub(super) definition: ServerInstanceDefinition,
}

impl<I: ServerInstance> Server<I> {
    /// 🏗️ Start assembling a server over one storage profile.
    pub fn builder(profile: StorageProfile) -> ServerBuilder<I> {
        ServerBuilder {
            profile,
            modules: Vec::new(),
            queries: Vec::new(),
            sagas: Vec::new(),
            apps: Vec::new(),
            documents: None,
            admin_token: None,
            presence: PresenceSettings::default(),
            origin_admission: None,
            limits: Limits::open(),
            addressing: ClientAddressing::default(),
            routes: RouteGroups::ALL,
            disclosure: InstanceDisclosure::default(),
            activations: DIRECTORY_CAPACITY,
            snapshot_interval: SNAPSHOT_INTERVAL,
            id: "server".to_string(),
            version: env!("CARGO_PKG_VERSION").to_string(),
        }
    }

    /// 🛣️ The wired router, ready to be mounted or wrapped: the routes, the body limit and the
    /// cross-origin grant. The throttle is not on it — whoever composes the final router puts
    /// [`throttled`](Self::throttled) around everything that does work per request.
    pub fn router(&self) -> Router {
        self.router.clone()
    }

    /// 🚦️ `router` behind this instance's throttle ([`throttle_middleware`]): the layer a composer
    /// applies outside its own per-request layers and inside only what must answer even a refused
    /// request (a transport gate, the cross-origin grant of an edge that owns it).
    pub fn throttled(&self, router: Router) -> Router {
        router.layer(axum::middleware::from_fn_with_state(self.state.edge(), throttle_middleware))
    }

    /// 🧠️ The shared state every handler runs against.
    pub fn state(&self) -> &ServerState<I> {
        &self.state
    }

    /// 🏛️ What this instance declares itself to be.
    pub fn definition(&self) -> &ServerInstanceDefinition {
        &self.definition
    }

    /// ▶️ Bind `addr` and serve the throttled router until the process is stopped. Connection info
    /// is carried into every handler so the loopback fact behind [`AdminGate`] and the peer address
    /// behind the throttle stay transport facts, and every connection sends without delay
    /// ([`undelayed`]).
    pub async fn run(self, addr: SocketAddr) -> Result<(), ServerError> {
        let listener = tokio::net::TcpListener::bind(addr).await.map_err(|error| ServerError::Internal(error.to_string()))?;
        let router = self.throttled(self.router.clone());
        axum::serve(listener.tap_io(undelayed), router.into_make_service_with_connect_info::<SocketAddr>()).await.map_err(|error| ServerError::Internal(error.to_string()))
    }
}
//#endregion 🔖️Server
