//! 🧩️ The server instance, module and store vocabulary a deployment implements.

use super::*;

//#region 🔖️Instance
/// 🪪️ One deployment of this product — hub, zentrale, a test profile — expressed as a **type**.
///
/// Every extension point of the server is an associated type here, and every one of them is chosen
/// by the deployment rather than by the framework. That is the whole design: a set of
/// implementations must be closed at the site that owns the implementations, so a downstream crate
/// names its own module set, its own deciders, its own storage backends, and — when it really has
/// several of one kind — closes them into one enum in its own crate with `dyn_enum_close!`, which
/// writes that enum for every port of this product, whether it is declared with plain `async fn` or
/// with `impl Future<..> + Send` (the macro delegates the latter as an `async fn` over the future's
/// `Output`, since one `match` cannot return two different opaque futures).
/// Closing those sets inside this crate instead would force this crate to name hub's types, which
/// inverts the dependency the product exists to avoid — and it is what made the document port
/// unusable before: an empty enum cannot be implemented from outside, so the document websocket
/// answered 404 unconditionally and the presence code behind it was unreachable.
///
/// Nothing in this crate implements `ServerInstance`. The reference profile that exercises it lives
/// with the tests (`🧪️tests/🧩️instance/🦀️.rs`), because a demo decider and an in-memory store are
/// fixtures, not product surface.
pub trait ServerInstance: Sized + Send + Sync + 'static {
    /// 📦️ The module set this deployment is assembled from.
    type Modules: ServerModule<Instance = Self> + 'static;
    /// ❓️ The read handlers it answers queries with; [`NoQueryHandler`] when it has none yet.
    type Queries: QueryHandler + 'static;
    /// 📄️ The replication engine behind its document websocket; [`NoDocumentAuthority`] when it
    /// hosts none.
    type Documents: DocumentAuthority + 'static;
    /// ⚖️ The deterministic cores its actors are decided by.
    type Deciders: Decider + 'static;
    /// 🧵️ The cross-actor workflows it runs off the outbox.
    type Sagas: Saga + 'static;
    /// 🪜️ The rungs of its authentication ladder.
    type Resolvers: PrincipalResolver + 'static;
    /// 🏛️ The backend holding its authoritative history.
    type AuthorityStore: AuthorityStore + 'static;
    /// 🔭️ The backend holding its rebuildable read models.
    type ProjectionStore: ProjectionStore + 'static;
    /// 🧱️ The backend holding its content-addressed bytes.
    type BlobStore: BlobStore + 'static;
    /// 🎫️ The backend holding its live authentication state.
    type SessionStore: SessionStore + 'static;

    /// 🗄️ Open the four durable roles for this deployment shape. The instance decides what
    /// [`StorageProfile::Embedded`]'s `data_dir` means — a directory to open, or nothing at all for
    /// a profile that keeps everything in memory — because the framework has no backend to impose.
    async fn open(profile: &StorageProfile) -> Result<InstanceStores<Self>, StorageError>;
}

/// 🏪️ The four durable roles of one instance, opened together so a half-open server is not a state
/// [`ServerBuilder::build`] has to handle.
pub struct InstanceStores<I: ServerInstance> {
    pub authority: I::AuthorityStore,
    pub projections: I::ProjectionStore,
    pub blobs: I::BlobStore,
    pub sessions: I::SessionStore,
}

/// 🎼️ The saga runner of one instance, over the workflows that instance declared.
pub type ServerSagas<I> = SagaRunner<<I as ServerInstance>::Sagas>;
//#endregion 🔖️Instance

//#region 🔖️Module
/// ☁️ The runtime half of a server module.
///
/// The declarative half is [`ModuleManifest`], which is pure data and lives in the contract so an
/// instance definition can be inspected, diffed and served without ever constructing a server. This
/// trait is the half that cannot be data: [`routes`](Self::routes) hands out a live
/// `Router<`[`ServerState`]`>` and therefore names the transport library, which is exactly why it
/// is declared here in layer 3 and not next to the manifest in layer 1. Keeping the two halves
/// apart is what lets a client, a CLI or a documentation generator read a manifest without axum.
///
/// A module belongs to exactly one [`ServerInstance`], named by [`Instance`](Self::Instance): its
/// routes take that instance's [`ServerState`], its deciders and rungs are that instance's decider
/// and resolver types. That binding is also why this trait is not `#[dyn_enum]`'d — the macro
/// refuses an associated type, because an enum has no single type to give it — so an instance with
/// several modules writes the delegating enum by hand, or gives each module its own server.
pub trait ServerModule: Send + Sync {
    /// 🚀️ The deployment this module is part of.
    type Instance: ServerInstance;

    /// 📇️ What this module declares to the instance registering it.
    async fn manifest(&self) -> ModuleManifest;

    /// 🚌️ The deciders this module registers on the command bus, one per actor kind it serves.
    async fn deciders(&self) -> Vec<<Self::Instance as ServerInstance>::Deciders> {
        Vec::new()
    }

    /// 🔀️ The cross-actor workflows this module reacts with, appended to the instance's one saga
    /// runner in module registration order. A module owns its workflows for the same reason it owns
    /// its deciders: the subsystem that emits an event is the subsystem that knows what must follow.
    async fn sagas(&self) -> Vec<<Self::Instance as ServerInstance>::Sagas> {
        Vec::new()
    }

    /// 🛣️ The routes this module mounts. Called once at build time with the router under
    /// construction; a module that serves no HTTP surface returns it untouched.
    async fn routes(&self, router: Router<ServerState<Self::Instance>>) -> Router<ServerState<Self::Instance>> {
        router
    }

    /// 🔑️ The authentication rungs this module contributes, appended to the shared ladder in
    /// module registration order.
    async fn resolvers(&self) -> Vec<<Self::Instance as ServerInstance>::Resolvers> {
        Vec::new()
    }

    /// 🎓️ The role definitions this module registers into the shared policy engine.
    async fn templates(&self) -> Vec<PolicyTemplate> {
        Vec::new()
    }

    /// 🧍️ Admit or refuse one state a presence session of `scope` shares. Every module is asked in
    /// registration order and the first refusal is sent back as a `refused` frame; the default
    /// admits, so an instance without presence rules shares any state. Synchronous on purpose: it
    /// runs inside every socket's receive loop and must neither suspend nor reach anything but the
    /// state it judges.
    fn presence_admission(&self, _scope: &Scope, _state: &OpaqueJson) -> Result<(), String> {
        Ok(())
    }

    /// 🛃️ Admit or refuse the shape of one command envelope before the bus reads or places anything
    /// for it: an id of a form this module's actors never have, a key or a kind out of bounds. Every
    /// module is asked in registration order and the first refusal is the command's rejection; the
    /// default admits. Synchronous and pure for the same reason as
    /// [`presence_admission`](Self::presence_admission): it runs inside every turn, ahead of the
    /// store, and judges nothing but the envelope.
    fn command_admission(&self, _envelope: &CommandEnvelope) -> Result<(), Rejection> {
        Ok(())
    }

    /// 💳️ The named allowance ([`Limits::allowances`]) one command posted by a client is counted
    /// against, besides the write allowance of its request: the class of commands that make the
    /// instance keep something of a finite store — a registration, an upload. Every module is asked
    /// in registration order and the first name is the command's class; the default names none. A
    /// command the instance issues itself (a saga's follow-up) is never asked about. Synchronous and
    /// pure like [`command_admission`](Self::command_admission), and asked before it: it judges by
    /// the envelope's kind, which the admission then holds the payload to.
    fn command_allowance(&self, _envelope: &CommandEnvelope) -> Option<&'static str> {
        None
    }
}

//#endregion 🔖️Module

//#region 🔖️Store
/// 🛡️ The bus shape this gateway serializes every command through: the instance's own authority
/// store and its own decider set, both known to the compiler at every call site — no boxed trait
/// object, and no enum this crate had to invent on the instance's behalf.
pub type ServerAuthority<I> = CommandBus<<I as ServerInstance>::AuthorityStore, <I as ServerInstance>::Deciders>;
//#endregion 🔖️Store
