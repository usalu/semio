//! 🛰️ Shard transports and test doubles: the shared thread transport, the shard transports enum with its conversions, the loopback transport and the fixture identity and probe helpers. See [`super`].

use super::*;

//#region 🚚️ShardTransports
/// 🔌️ Local newtype so `ShardExecutor::new` can hand [`ShardLoop::new`] an `Arc<ThreadTransport>`
/// as a [`ShardTransports::SharedThread`] variant while the executor ALSO keeps its own
/// [`ThreadTransport`] end (the mirror of the one wrapped here) to inject frames and drain outcomes —
/// `impl ShardTransport for Arc<ThreadTransport>` directly would hit `E0117` (neither type is local
/// to this crate; `🧵️shard/🦀️.rs`'s own `LoopbackProbe` doc comment already names this same
/// orphan-rule constraint for `Arc<LoopbackTransport>`).
pub struct SharedThreadTransport(pub Arc<semio_framework_actor::ThreadTransport>);

impl ShardTransport for SharedThreadTransport {
    async fn send(&self, bytes: &[u8]) {
        self.0.send(bytes).await;
    }
    async fn recv(&self) -> Option<Vec<u8>> {
        self.0.try_recv_now()
    }
    async fn heartbeat(&self) -> u64 {
        self.0.heartbeat().await
    }
    async fn kill(&self) {
        self.0.kill().await;
    }
}

/// 🚚️ MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME (host-dedyn): the closed-set enum replacing every
/// `Box<dyn ShardTransport>` in this crate. `ShardTransport` (O1/R1: `async fn` in a trait cannot be
/// `dyn`-dispatched) is declared OUTSIDE this crate (`semio_framework_actor::ShardTransport`, this
/// packet's path scope forbids touching `🔌️plugin/**` outside `🖥️host/`), so `#[dyn_enum]` cannot be
/// applied to it — the macro's bare-invocation mechanism (`📓️terra-dyn-enum-macro-report.md`,
/// finding 1) only works when the trait's OWN crate emits the captured delegation macro. This family
/// is therefore hand-written unconditionally, not merely because of cfg-gated variants (`GuestRuntimes`'
/// own reason, next region) — a second, independent reason the macro doesn't apply here, worth
/// recording for the ~50 remaining families since some of them will hit this exact wall.
pub enum ShardTransports {
    SharedThread(SharedThreadTransport),
    Process(semio_framework_plugin_host_process_transport::ProcessTransport),
    Stdio(semio_framework_plugin_host_process_transport::StdioTransport),
    #[cfg(any(test, feature = "testkit"))]
    Loopback(LoopbackTransport),
}

impl ShardTransport for ShardTransports {
    async fn send(&self, bytes: &[u8]) {
        match self {
            Self::SharedThread(t) => t.send(bytes).await,
            Self::Process(t) => t.send(bytes).await,
            Self::Stdio(t) => t.send(bytes).await,
            #[cfg(any(test, feature = "testkit"))]
            Self::Loopback(t) => t.send(bytes).await,
        }
    }

    async fn recv(&self) -> Option<Vec<u8>> {
        match self {
            Self::SharedThread(t) => t.recv().await,
            Self::Process(t) => t.recv().await,
            Self::Stdio(t) => t.recv().await,
            #[cfg(any(test, feature = "testkit"))]
            Self::Loopback(t) => t.recv().await,
        }
    }

    async fn heartbeat(&self) -> u64 {
        match self {
            Self::SharedThread(t) => t.heartbeat().await,
            Self::Process(t) => t.heartbeat().await,
            Self::Stdio(t) => t.heartbeat().await,
            #[cfg(any(test, feature = "testkit"))]
            Self::Loopback(t) => t.heartbeat().await,
        }
    }

    async fn kill(&self) {
        match self {
            Self::SharedThread(t) => t.kill().await,
            Self::Process(t) => t.kill().await,
            Self::Stdio(t) => t.kill().await,
            #[cfg(any(test, feature = "testkit"))]
            Self::Loopback(t) => t.kill().await,
        }
    }
}

impl From<SharedThreadTransport> for ShardTransports {
    fn from(t: SharedThreadTransport) -> Self {
        Self::SharedThread(t)
    }
}

impl From<semio_framework_plugin_host_process_transport::ProcessTransport> for ShardTransports {
    fn from(t: semio_framework_plugin_host_process_transport::ProcessTransport) -> Self {
        Self::Process(t)
    }
}

impl From<semio_framework_plugin_host_process_transport::StdioTransport> for ShardTransports {
    fn from(t: semio_framework_plugin_host_process_transport::StdioTransport) -> Self {
        Self::Stdio(t)
    }
}

#[cfg(any(test, feature = "testkit"))]
impl From<LoopbackTransport> for ShardTransports {
    fn from(t: LoopbackTransport) -> Self {
        Self::Loopback(t)
    }
}
//#endregion 🚚️ShardTransports

//#region 🧪️TestDoubles
/// 🧵️ In-process, single-actor loopback transport — an `mpsc`-free stand-in for
/// `design-runtime.md`'s `ThreadTransport`, precise enough to exercise `ShardLoop::pump`'s real
/// drain/group/dispatch/send logic end to end without needing a real thread. Its two buffers are
/// `Arc<Mutex<..>>` INTERNALLY (not the struct itself behind an `Arc`) so `LoopbackProbe::new` can
/// hand `ShardLoop::new` sole ownership of a [`ShardTransports::Loopback`] while keeping a separate
/// handle that can still inspect `outbound` afterward — `impl ShardTransport for
/// Arc<LoopbackTransport>` would hit `E0117` (neither `Arc` nor `ShardTransport` is local to this
/// crate, and `Arc` is not `#[fundamental]` the way `Box` is). `pub(crate)`, not private — moved out
/// of `mod tests` (below) so [`ShardTransports`] can name it in its own `#[cfg(any(test, feature = "testkit"))]` variant.
#[cfg(any(test, feature = "testkit"))]
#[derive(Default)]
pub struct LoopbackTransport {
    inbound: Arc<Mutex<Vec<Vec<u8>>>>,
    pub outbound: Arc<Mutex<Vec<Vec<u8>>>>,
}

#[cfg(any(test, feature = "testkit"))]
impl LoopbackTransport {
    /// Returns `(the transport ShardLoop::new takes ownership of, a probe this test keeps)`.
    pub async fn paired() -> (Self, crate::LoopbackProbe) {
        let transport = Self::default();
        let probe = crate::LoopbackProbe { inbound: transport.inbound.clone(), outbound: transport.outbound.clone() };
        (transport, probe)
    }
}

#[cfg(any(test, feature = "testkit"))]
impl ShardTransport for LoopbackTransport {
    async fn send(&self, bytes: &[u8]) {
        self.outbound.lock().expect("loopback lock").push(bytes.to_vec());
    }
    async fn recv(&self) -> Option<Vec<u8>> {
        self.inbound.lock().expect("loopback lock").pop()
    }
    async fn heartbeat(&self) -> u64 {
        0
    }
    async fn kill(&self) {}
}
//#endregion 🧪️TestDoubles

#[cfg(any(test, feature = "testkit"))]
pub fn fixture_instance_close_request() -> semio_framework::kernel::ActorInstanceCloseRequest {
    semio_framework::kernel::ActorInstanceCloseRequest { lifetime: semio_framework::kernel::ActorInstanceLifetime { activation_generation: 1, instance_id: 7, guest_lifetime: 13 }, request_sequence: 9 }
}

#[cfg(any(test, feature = "testkit"))]
pub fn fixture_instance_close_event() -> Event {
    Event::InstanceClose(fixture_instance_close_request())
}

#[cfg(any(test, feature = "testkit"))]
pub fn test_identity_issuer()->OriginalShardIdentityIssuer{
 let policy:ShardIdentityPolicy=serde_json::from_value(serde_json::from_str::<serde_json::Value>(include_str!("../../⚡️effects/🧪️tests/🧫️fixtures/🔣️.json")).unwrap()["identityPolicy"].clone()).unwrap();
 Box::new(move |_|OriginalShardIdentity::issue(policy,Box::new(|_|true),Box::new(move |request|{if request.bytes>policy.grant.maximum_capacity_bytes||policy.grant.maximum_items==0||policy.grant.maximum_depth==0{Err(semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::OwnershipLimit,"original fixture refuses native shard allocation"))}else{Ok(())}})))
}

/// 🧪️ [`LoopbackTransport`] (module level, above — moved there so [`ShardTransports`] can name
/// it) hands this back from `paired()`; `pub(super)` so the parent `shard` module's own
/// `LoopbackTransport::paired` can construct it.
#[cfg(any(test, feature = "testkit"))]
pub struct LoopbackProbe {
    pub(crate) inbound: Arc<Mutex<Vec<Vec<u8>>>>,
    pub(crate) outbound: Arc<Mutex<Vec<Vec<u8>>>>,
}

#[cfg(any(test, feature = "testkit"))]
impl LoopbackProbe {
    pub async fn push_inbound(&self, bytes: Vec<u8>) {
        self.inbound.lock().expect("loopback lock").push(bytes);
    }
    pub async fn take_outbound(&self) -> Vec<Vec<u8>> {
        std::mem::take(&mut *self.outbound.lock().expect("loopback lock"))
    }
}
