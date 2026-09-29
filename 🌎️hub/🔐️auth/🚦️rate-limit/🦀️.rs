//! 🚦️ The hub's own per-principal and per-remote-address token buckets, in front of the credential
//! sign-in, directory-command, invite-redemption and socket-grant routes, and behind every agent session's document
//! commands (`agent-command`: the volume of committed tool calls an agent pushes through its document sockets) — and the
//! stream rule behind the execution-target asset routes ([`HubStreamLimiterV1`]), whose immutable, content-addressed
//! bodies no request bucket meters: they are bounded by how many stream at once.
//!
//! Schema authority: [`🔣️.json`](../🧬️schema/🔣️.json) `$defs/AuthRateLimitPolicyV1` and
//! `$defs/AuthRateLimitClassV1`. The bucket is a millisecond-budget formulation: a class costs
//! `cost_ms` of budget per admitted request, budget accrues one millisecond per elapsed
//! millisecond and saturates at `burst * cost_ms`, so `burst` requests may arrive at once and the
//! sustained rate is exactly one per `cost_ms`. All arithmetic is integer — two runs over the same
//! clock readings always decide identically.
//!
//! The clock is injected ([`RateLimitClockV1`]) so the laws in `🧪️tests/🔬️unit/🦀️.rs` step time
//! by hand instead of sleeping.

use std::collections::HashMap;
use std::sync::Mutex;

use semio_framework_hash::Sha256;

/// 🕰️ The limiter's only notion of time.
pub trait RateLimitClockV1: Send + Sync + 'static {
    fn now_ms(&self) -> i64;
}

/// ⏱️ The production clock: milliseconds since the Unix epoch.
pub struct SystemRateLimitClockV1;

impl RateLimitClockV1 for SystemRateLimitClockV1 {
    fn now_ms(&self) -> i64 {
        i64::try_from(std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|elapsed| elapsed.as_millis()).unwrap_or_default()).unwrap_or(i64::MAX)
    }
}

/// 🏷️ The five rate-limited families: four routes and the agent document-command lane.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum RateLimitClassV1 {
    Auth,
    DirectoryCommand,
    InviteRedemption,
    SocketGrant,
    AgentCommand,
}

/// 🧾️ Every class, in the schema enum's order — what the laws hold the schema and the policy table against.
pub const RATE_LIMIT_CLASSES: [RateLimitClassV1; 5] = [RateLimitClassV1::Auth, RateLimitClassV1::DirectoryCommand, RateLimitClassV1::InviteRedemption, RateLimitClassV1::SocketGrant, RateLimitClassV1::AgentCommand];

impl RateLimitClassV1 {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Auth => "auth",
            Self::DirectoryCommand => "directory-command",
            Self::InviteRedemption => "invite-redemption",
            Self::SocketGrant => "socket-grant",
            Self::AgentCommand => "agent-command",
        }
    }

    /// 📐️ The shipped policy for this class — the table `📓️au1-…` §1.4 publishes.
    pub fn policy(self) -> RateLimitPolicyV1 {
        match self {
            Self::Auth => RateLimitPolicyV1 { burst: 10, cost_ms: 6_000 },
            Self::DirectoryCommand => RateLimitPolicyV1 { burst: 60, cost_ms: 100 },
            Self::InviteRedemption => RateLimitPolicyV1 { burst: 10, cost_ms: 6_000 },
            Self::SocketGrant => RateLimitPolicyV1 { burst: 30, cost_ms: 200 },
            Self::AgentCommand => RateLimitPolicyV1 { burst: 120, cost_ms: 250 },
        }
    }
}

/// 🏷️ `RateLimitRefusalV1.schema`.
pub const RATE_LIMIT_REFUSAL_SCHEMA: &str = "semio.hub.rate-limit-refusal/v1";

/// 🗣️ `RateLimitRefusalMessageV1`: the notice of a rate-limited request, en and de.
#[derive(Clone, Copy, Debug, PartialEq, Eq, serde::Serialize)]
pub struct RateLimitRefusalMessageV1 {
    pub en: &'static str,
    pub de: &'static str,
}

/// 🗣️ The one `RateLimitRefusalMessageV1` every refusal carries.
pub const RATE_LIMIT_REFUSAL_MESSAGE: RateLimitRefusalMessageV1 = RateLimitRefusalMessageV1 { en: "Too many requests in a short time. Wait a moment, then try again.", de: "Zu viele Anfragen in kurzer Zeit. Bitte einen Moment warten und es dann erneut versuchen." };

/// 🚦️ `RateLimitRefusalV1`: the body of the `429` a non-auth rate-limited route family answers when its bucket is empty, or a
/// stream-limited one when its streams stay taken — the class, the wait until one request is admitted again, the notice; the
/// client resends the same request after the wait.
#[derive(Clone, Copy, Debug, PartialEq, Eq, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RateLimitRefusalV1 {
    pub schema: &'static str,
    pub code: &'static str,
    pub class: &'static str,
    pub retry_after_ms: u64,
    pub message: RateLimitRefusalMessageV1,
}

impl RateLimitRefusalV1 {
    pub fn new(class: RateLimitClassV1, retry_after_ms: u64) -> Self {
        Self::named(class.as_str(), retry_after_ms)
    }

    /// 🚰️ The refusal of a stream-limited class whose streams stayed taken for its whole admission wait.
    pub fn streams(class: StreamLimitClassV1, retry_after_ms: u64) -> Self {
        Self::named(class.as_str(), retry_after_ms)
    }

    fn named(class: &'static str, retry_after_ms: u64) -> Self {
        Self { schema: RATE_LIMIT_REFUSAL_SCHEMA, code: "rate-limited", class, retry_after_ms: retry_after_ms.max(1), message: RATE_LIMIT_REFUSAL_MESSAGE }
    }
}

/// 📐️ One class's admitted burst and sustained cost.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct RateLimitPolicyV1 {
    pub burst: u32,
    pub cost_ms: u32,
}

impl RateLimitPolicyV1 {
    pub fn capacity_ms(self) -> u64 {
        u64::from(self.burst) * u64::from(self.cost_ms)
    }
}

/// 🧭️ What a request is charged against: never the raw address or capability, always its digest.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct RateLimitSubjectV1([u8; 32]);

impl RateLimitSubjectV1 {
    /// 🌐️ The peer's network address, domain-separated from principal subjects.
    pub fn remote_address(address: &std::net::IpAddr) -> Self {
        Self::digest(b"semio/hub/rate-limit/remote/v1\0", address.to_string().as_bytes())
    }

    /// 🪪️ An authenticated principal — the session capability's public selector, never its secret.
    pub fn principal(selector: &str) -> Self {
        Self::digest(b"semio/hub/rate-limit/principal/v1\0", selector.as_bytes())
    }

    /// 📧️ A sign-in attempt's claimed identity, so one address cannot spray many accounts.
    pub fn claimed_identity(email: &str) -> Self {
        Self::digest(b"semio/hub/rate-limit/identity/v1\0", email.to_lowercase().as_bytes())
    }

    fn digest(domain: &[u8], value: &[u8]) -> Self {
        let mut hash = Sha256::new();
        hash.update(domain);
        hash.update(value);
        Self(hash.finalize())
    }
}

/// ✅️ One admission decision. `Refused` carries the exact wait the client is told to observe.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RateLimitDecisionV1 {
    Admitted,
    Refused { retry_after_ms: u64 },
}

impl RateLimitDecisionV1 {
    pub fn is_admitted(self) -> bool {
        matches!(self, Self::Admitted)
    }

    /// ⏳️ `retry-after` in whole seconds, never below one, as HTTP requires.
    pub fn retry_after_secs(self) -> u64 {
        match self {
            Self::Admitted => 0,
            Self::Refused { retry_after_ms } => retry_after_ms.div_ceil(1_000).max(1),
        }
    }
}

#[derive(Clone, Copy)]
struct Bucket {
    available_ms: u64,
    last_ms: i64,
}

/// 🚦️ Every class's buckets, bounded in memory and safe to share across connections.
pub struct HubRateLimiterV1 {
    clock: std::sync::Arc<dyn RateLimitClockV1>,
    buckets: Mutex<HashMap<(RateLimitClassV1, RateLimitSubjectV1), Bucket>>,
    capacity: usize,
}

/// 🧮️ How many distinct subjects the limiter tracks before it sheds fully recovered entries.
pub const SUBJECT_CAPACITY: usize = 16_384;

impl HubRateLimiterV1 {
    pub fn new(clock: std::sync::Arc<dyn RateLimitClockV1>) -> Self {
        Self::with_capacity(clock, SUBJECT_CAPACITY)
    }

    pub fn with_capacity(clock: std::sync::Arc<dyn RateLimitClockV1>, capacity: usize) -> Self {
        Self { clock, buckets: Mutex::new(HashMap::new()), capacity: capacity.max(1) }
    }

    /// 🕰️ The production limiter, over the system clock.
    pub fn system() -> Self {
        Self::new(std::sync::Arc::new(SystemRateLimitClockV1))
    }

    /// 🔎️ The number of live subjects — a memory-bound witness for the laws, not a rate signal.
    pub fn tracked_subjects(&self) -> usize {
        self.buckets.lock().map(|buckets| buckets.len()).unwrap_or(0)
    }

    /// ✅️ Charges one request against every subject it belongs to; ALL must admit.
    /// The refusal reported is the longest wait any subject demands.
    pub fn admit(&self, class: RateLimitClassV1, subjects: &[RateLimitSubjectV1]) -> RateLimitDecisionV1 {
        let policy = class.policy();
        let now_ms = self.clock.now_ms();
        let Ok(mut buckets) = self.buckets.lock() else {
            return RateLimitDecisionV1::Refused { retry_after_ms: u64::from(policy.cost_ms) };
        };
        let mut worst = 0u64;
        let mut charged: Vec<(RateLimitClassV1, RateLimitSubjectV1)> = Vec::with_capacity(subjects.len());
        for subject in subjects {
            let key = (class, *subject);
            if !buckets.contains_key(&key) && buckets.len() >= self.capacity {
                prune_recovered(&mut buckets, now_ms);
            }
            if !buckets.contains_key(&key) && buckets.len() >= self.capacity {
                worst = worst.max(u64::from(policy.cost_ms));
                break;
            }
            let bucket = buckets.entry(key).or_insert(Bucket { available_ms: policy.capacity_ms(), last_ms: now_ms });
            let elapsed = now_ms.saturating_sub(bucket.last_ms).max(0);
            bucket.available_ms = bucket.available_ms.saturating_add(u64::try_from(elapsed).unwrap_or(0)).min(policy.capacity_ms());
            bucket.last_ms = now_ms;
            if bucket.available_ms < u64::from(policy.cost_ms) {
                worst = worst.max(u64::from(policy.cost_ms) - bucket.available_ms);
            } else {
                bucket.available_ms -= u64::from(policy.cost_ms);
                charged.push(key);
            }
        }
        if worst == 0 {
            return RateLimitDecisionV1::Admitted;
        }
        for key in charged {
            if let Some(bucket) = buckets.get_mut(&key) {
                bucket.available_ms = bucket.available_ms.saturating_add(u64::from(policy.cost_ms)).min(policy.capacity_ms());
            }
        }
        RateLimitDecisionV1::Refused { retry_after_ms: worst }
    }

    /// ⏳️ Paces one request instead of refusing it outright: admits it as soon as every subject's bucket holds its cost,
    /// waiting through `wait(ms)` for exactly the refill each refusal names, at most `patience_ms` in total. A request
    /// the buckets cannot admit within that patience is refused with the wait it still needs.
    pub async fn admit_paced<W, F>(&self, class: RateLimitClassV1, subjects: &[RateLimitSubjectV1], patience_ms: u64, mut wait: W) -> RateLimitDecisionV1
    where
        W: FnMut(u64) -> F,
        F: std::future::Future<Output = ()>,
    {
        let mut waited_ms = 0u64;
        loop {
            match self.admit(class, subjects) {
                RateLimitDecisionV1::Admitted => return RateLimitDecisionV1::Admitted,
                RateLimitDecisionV1::Refused { retry_after_ms } if waited_ms.saturating_add(retry_after_ms) <= patience_ms => {
                    wait(retry_after_ms).await;
                    waited_ms = waited_ms.saturating_add(retry_after_ms);
                }
                refused => return refused,
            }
        }
    }
}

/// 🧹️ Drops every subject whose budget has fully recovered: such an entry decides exactly as a
/// freshly created one would, so forgetting it is observationally free and bounds the map.
fn prune_recovered(buckets: &mut HashMap<(RateLimitClassV1, RateLimitSubjectV1), Bucket>, now_ms: i64) {
    buckets.retain(|(class, _), bucket| {
        let capacity_ms = class.policy().capacity_ms();
        let elapsed = u64::try_from(now_ms.saturating_sub(bucket.last_ms).max(0)).unwrap_or(0);
        bucket.available_ms.saturating_add(elapsed) < capacity_ms
    });
}

//#region 🚰️StreamLimit
/// 🏷️ The stream-limited families (schema `AuthStreamLimitClassV1`): `execution-target-asset`, the component and browser
/// actor bodies of a document's execution target — immutable, catalog-verified, content-addressed and tens of megabytes
/// long, so no per-request bucket meters them; what bounds them is how many stream at once.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum StreamLimitClassV1 {
    ExecutionTargetAsset,
}

/// 🧾️ Every stream-limited class, in the schema enum's order.
pub const STREAM_LIMIT_CLASSES: [StreamLimitClassV1; 1] = [StreamLimitClassV1::ExecutionTargetAsset];

impl StreamLimitClassV1 {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::ExecutionTargetAsset => "execution-target-asset",
        }
    }

    /// 📐️ The shipped bound of this class: a whole catalog's components for a few clients at once hub-wide, a document's
    /// component and browser actor twice over per principal.
    pub fn policy(self) -> StreamLimitPolicyV1 {
        match self {
            Self::ExecutionTargetAsset => StreamLimitPolicyV1 { hub_streams: 32, principal_streams: 4, admission_wait_ms: 5_000, retry_after_ms: 1_000 },
        }
    }
}

/// 📐️ One stream class's bound (schema `AuthStreamLimitPolicyV1`): at most `hub_streams` streams at once hub-wide and
/// `principal_streams` per principal. A request waits first come, first served for `admission_wait_ms`, then is refused
/// with `retry_after_ms`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct StreamLimitPolicyV1 {
    pub hub_streams: u32,
    pub principal_streams: u32,
    pub admission_wait_ms: u64,
    pub retry_after_ms: u64,
}

type StreamGates = std::sync::Arc<Mutex<HashMap<(StreamLimitClassV1, RateLimitSubjectV1), std::sync::Arc<tokio::sync::Semaphore>>>>;

/// 🚰️ The live streams of every stream-limited class: one hub-wide gate per class and one gate per principal streaming
/// now, forgotten as soon as its last stream ends, so the map holds only principals with a live or waiting stream.
pub struct HubStreamLimiterV1 {
    classes: HashMap<StreamLimitClassV1, (StreamLimitPolicyV1, std::sync::Arc<tokio::sync::Semaphore>)>,
    principals: StreamGates,
}

impl Default for HubStreamLimiterV1 {
    fn default() -> Self {
        Self::with_policies(STREAM_LIMIT_CLASSES.map(|class| (class, class.policy())))
    }
}

impl HubStreamLimiterV1 {
    /// 📐️ A limiter over the given policies — the laws' narrow bounds; production runs [`Default`], the shipped ones.
    pub fn with_policies(policies: impl IntoIterator<Item = (StreamLimitClassV1, StreamLimitPolicyV1)>) -> Self {
        let classes = policies.into_iter().map(|(class, policy)| (class, (policy, std::sync::Arc::new(tokio::sync::Semaphore::new(policy.hub_streams as usize))))).collect();
        Self { classes, principals: std::sync::Arc::default() }
    }

    /// 🎫️ Admits one stream of `class` for `principal`, waiting first come, first served for its principal's slot and then
    /// a hub-wide one, at most the policy's admission wait. The permit is the stream's slot: it is held for as long as the
    /// body streams and released when the body ends or the client drops it. A refusal names the policy's wait.
    pub async fn admit(&self, class: StreamLimitClassV1, principal: RateLimitSubjectV1) -> Result<StreamPermitV1, RateLimitRefusalV1> {
        let (policy, hub) = self.classes.get(&class).map(|(policy, hub)| (*policy, hub.clone())).unwrap_or_else(|| (class.policy(), std::sync::Arc::new(tokio::sync::Semaphore::new(0))));
        let key = (class, principal);
        let gate = match self.principals.lock() {
            Ok(mut principals) => principals.entry(key).or_insert_with(|| std::sync::Arc::new(tokio::sync::Semaphore::new(policy.principal_streams as usize))).clone(),
            Err(_) => return Err(RateLimitRefusalV1::streams(class, policy.retry_after_ms)),
        };
        let admitted = tokio::time::timeout(std::time::Duration::from_millis(policy.admission_wait_ms), async {
            let principal = gate.clone().acquire_owned().await.ok()?;
            let hub = hub.acquire_owned().await.ok()?;
            Some((principal, hub))
        })
        .await
        .ok()
        .flatten();
        match admitted {
            Some(permits) => Ok(StreamPermitV1 { permits: Some(permits), key, gate, principals: self.principals.clone(), principal_streams: policy.principal_streams }),
            None => {
                forget_idle_gate(&self.principals, &key, &gate, policy.principal_streams);
                Err(RateLimitRefusalV1::streams(class, policy.retry_after_ms))
            }
        }
    }

    /// 🔎️ How many streams of `class` are live hub-wide now — a witness for the laws, not a rate signal.
    pub fn live_streams(&self, class: StreamLimitClassV1) -> usize {
        self.classes.get(&class).map_or(0, |(policy, hub)| (policy.hub_streams as usize).saturating_sub(hub.available_permits()))
    }

    /// 🔎️ How many principals have a live or waiting stream — the memory bound's witness.
    pub fn tracked_principals(&self) -> usize {
        self.principals.lock().map(|principals| principals.len()).unwrap_or(0)
    }
}

/// 🎫️ One admitted stream's slots, released when the stream ends; the principal's gate is forgotten with its last stream.
pub struct StreamPermitV1 {
    permits: Option<(tokio::sync::OwnedSemaphorePermit, tokio::sync::OwnedSemaphorePermit)>,
    key: (StreamLimitClassV1, RateLimitSubjectV1),
    gate: std::sync::Arc<tokio::sync::Semaphore>,
    principals: StreamGates,
    principal_streams: u32,
}

impl Drop for StreamPermitV1 {
    fn drop(&mut self) {
        self.permits.take();
        forget_idle_gate(&self.principals, &self.key, &self.gate, self.principal_streams);
    }
}

/// 🧹️ Forgets `gate` when nothing streams through it and nothing waits on it: only the map and this caller hold it and
/// every slot is free. Checked and removed under the map's lock, which every admission clones a gate under.
fn forget_idle_gate(principals: &StreamGates, key: &(StreamLimitClassV1, RateLimitSubjectV1), gate: &std::sync::Arc<tokio::sync::Semaphore>, principal_streams: u32) {
    if let Ok(mut principals) = principals.lock() {
        if principals.get(key).is_some_and(|current| std::sync::Arc::ptr_eq(current, gate)) && std::sync::Arc::strong_count(gate) == 2 && gate.available_permits() == principal_streams as usize {
            principals.remove(key);
        }
    }
}
//#endregion 🚰️StreamLimit
