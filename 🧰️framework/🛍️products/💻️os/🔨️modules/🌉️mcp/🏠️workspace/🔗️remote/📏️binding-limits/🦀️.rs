//! 📏️ Hub binding limits, the network-budget wait, the refresh retry law and the component stream constants.

use semio_framework_os_mcp_hub_pair::binding_errors::HubBindingError;

pub const HUB_DESCRIPTOR_INDEX_MAX_DOCUMENTS: usize = 4_096;

/// 👥️ The most member rows a descriptor refresh reads while it looks for the authenticated principal's own row.
pub const HUB_SPACE_MEMBERS_MAX: usize = 4_096;

pub const HUB_VERIFIED_CATALOG_MAX_PACKAGES: usize = 256;

pub const HUB_VERIFIED_CATALOG_MAX_DESCRIPTOR_BYTES: usize = 32 * 1024 * 1024;

pub const HUB_BINDING_DIAGNOSTIC_MAX_BYTES: usize = 4_096;

pub const HUB_BINDING_ID_MAX_BYTES: usize = 512;

pub const HUB_BINDING_OPERATION_TIMEOUT_MS: u64 = 10_000;

/// ⏳️ How long a hub-bound call waits for a descriptor refresh already in flight before it fails
/// closed: one refresh turn's own deadline. Every directory event in the space (another member's
/// new document, this agent's own commit) invalidates the authority, and without the wait every
/// tool call in that window answered a retryable `PLUGIN_UNAVAILABLE`, including an approval whose
/// edit had already landed.
pub const HUB_AUTHORITY_SETTLE_WAIT_MS: u64 = HUB_BINDING_OPERATION_TIMEOUT_MS;

/// ⏳️ How long a hub-bound call keeps waiting while the binding waits for its network byte budget to refill: one refill
/// interval on top of the ordinary settle wait, so a call spans the refill instead of refusing inside it.
pub const HUB_NETWORK_BUDGET_SETTLE_WAIT_MS: u64 = semio_framework_os_services::HTTP_BUCKET_REFILL_INTERVAL_MS + HUB_AUTHORITY_SETTLE_WAIT_MS;

/// 💰️ The budget one authority refresh waits for before it runs again after its budget ran out: the session, the space page
/// and one manifest per document, plus any descriptor it has not verified before.
pub const HUB_REFRESH_NETWORK_BUDGET_BYTES: u64 = 16 * 1024 * 1024;

/// 🔁️ How often a network budget wait re-reads the budget.
pub const HUB_NETWORK_BUDGET_POLL_MS: u64 = 250;

/// 💰️ Waits until `read` reports at least `needed` bytes of budget (a whole allowance when `needed` exceeds it), reporting
/// each wait to `report` as `(remaining, needed)`: a spent budget is a wait for its next refill turn, never a refusal.
/// Only `cancel` ends the wait early.
pub fn await_network_budget(
    read: impl Fn() -> semio_framework_os_services::HttpPackageBudget,
    needed: u64,
    cancel: &semio_framework_async::CancelToken,
    mut report: impl FnMut(u64, u64),
) -> Result<(), HubBindingError> {
    loop {
        let budget = read();
        let needed = needed.min(budget.capacity_bytes);
        if budget.remaining_bytes >= needed {
            return Ok(());
        }
        if cancel.is_cancelled_now() {
            return Err(HubBindingError::Cancelled);
        }
        report(budget.remaining_bytes, needed);
        pause_unless_cancelled(cancel, HUB_NETWORK_BUDGET_POLL_MS);
    }
}

/// 🔁️ First and largest pause between failed authority refreshes. A refresh costs the hub two
/// requests plus two per document, and one that failed (a busy hub answering `503
/// deadline-exceeded`) used to be retried back-to-back for as long as it kept failing.
pub const HUB_REFRESH_RETRY_BASE_MS: u64 = 100;

pub const HUB_REFRESH_RETRY_MAX_MS: u64 = 5_000;

/// 🔁️ `min(base · 2^(failures-1), max)` — the pause after the `failures`th refresh in a row failed.
pub fn hub_refresh_retry_ms(failures: u32) -> u64 {
    HUB_REFRESH_RETRY_BASE_MS.saturating_mul(1u64 << failures.saturating_sub(1).min(16)).min(HUB_REFRESH_RETRY_MAX_MS)
}

/// 😴️ Sleeps `ms` in short steps so a cancelled binding actor stops within one step.
pub(super) fn pause_unless_cancelled(cancel: &semio_framework_async::CancelToken, ms: u64) {
    let deadline = std::time::Instant::now() + std::time::Duration::from_millis(ms);
    while !cancel.is_cancelled_now() {
        let Some(remaining) = deadline.checked_duration_since(std::time::Instant::now()).filter(|remaining| !remaining.is_zero()) else { return };
        std::thread::sleep(remaining.min(std::time::Duration::from_millis(25)));
    }
}

/// 🧩️ Execution-target components are static assets — immutable, catalog-verified, content-addressed — so a gateway keeps
/// them in the per-user execution-target store and fetches a missing one over a pool of its own that meters no bytes: the
/// per-minute budget is for the directory's API requests, and component traffic is bounded by [`HUB_COMPONENT_STREAMS`]
/// here and by the hub's own execution-target asset stream rule (see [`HubComponentAssets`]).
pub const HUB_COMPONENT_ASSET_BYTES_PER_MINUTE: u64 = u64::MAX;

/// 🚰️ Component streams one gateway runs at once: one stream already takes the whole link, a second only splits it.
pub const HUB_COMPONENT_STREAMS: u32 = 1;

/// ⏱️ One component stream's deadline is this allowance plus its declared length at [`HUB_COMPONENT_STREAM_FLOOR_BYTES_PER_SECOND`].
pub const HUB_COMPONENT_STREAM_ALLOWANCE_MS: u64 = 60_000;

/// 🐢️ The slowest transfer rate a component stream is still waited for.
pub const HUB_COMPONENT_STREAM_FLOOR_BYTES_PER_SECOND: u64 = 256 * 1024;

/// 🔁️ How often one component is asked for across interrupted streams and hub shortages (`502`/`503`/`504`) before the
/// open fails, retryably.
pub const HUB_COMPONENT_STREAM_ATTEMPTS: u32 = 4;

/// ⏳️ The wait a busy hub's asset refusal stands for when its body names none, and the longest wait any refusal is held to.
pub const HUB_COMPONENT_STREAM_RETRY_MS: u64 = 1_000;

pub const HUB_COMPONENT_STREAM_RETRY_MAX_MS: u64 = 30_000;

/// 📈️ The longest a component transfer stays silent: an unchanged progress is published again after this long.
pub const HUB_COMPONENT_PROGRESS_KEEPALIVE_MS: u64 = 2_000;

/// 💰️ The budget one execution-target lease request waits for: its ≈3 KB JSON with generous headroom.
pub const HUB_EXECUTION_TARGET_LEASE_BUDGET_BYTES: u64 = 64 * 1024;

pub const CANONICAL_CHECKPOINT_RESOURCE_SCHEMA: &str = "semio.mcp.canonical-checkpoint-resource/v1";

pub const CANONICAL_CHECKPOINT_RESOURCE_MAX_TEXT_BYTES: usize = 6 * 1024 * 1024;

pub(super) const CANONICAL_CHECKPOINT_RESOURCE_METADATA_MAX_BYTES: usize = 16 * 1024;
