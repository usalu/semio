//! 🚦️ The hub's own per-principal and per-remote-address token buckets, in front of the credential
//! sign-in, directory-command, invite-redemption and socket-grant routes, and behind every agent session's document
//! commands (`agent-command`: the volume of committed tool calls an agent pushes through its document sockets).
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
