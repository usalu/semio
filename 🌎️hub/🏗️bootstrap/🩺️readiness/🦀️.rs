//! 🩺️ `readiness` — hub mode, readiness reports, checkpoint policy, liveness and health routes.
//!
//! Carved out of `🏗️bootstrap/🦀️.rs`; mounted from its root with `#[path]`.

use axum::extract::State;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::routing::get;
use axum::{Json, Router};
use futures::{SinkExt, StreamExt};
use semio_framework_trace::record::Tracer;
use semio_hub::artifact_authority::chunk_cas::ArtifactChunkCasStorage;
use semio_hub::artifact_authority::trusted_catalog::schema::TrustedCatalogLoadProgressV1;
use semio_hub::directory::model::DocumentScope;
use semio_hub::directory::LocalBootstrapTransport;
use serde::Serialize;
use std::net::SocketAddr;
use std::sync::{Arc, Mutex};
use crate::startup::{HubError, StartupProgressCellV1};
use crate::state::HubState;
use crate::transport::{ForwardedTlsTrustV1, refusal_middleware, transport_security_middleware, CrossOriginPolicyV1, cors_middleware};
use db::db_storage::PayloadStorage as _;
use server::storage::SessionStore as _;

//#region 🔖️AdminAuth
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum HubMode {
    Production,
    Development,
}

impl HubMode {
    pub(crate) fn from_environment(bind: std::net::IpAddr) -> Result<Self, HubError> {
        match std::env::var("OS_HUB_MODE").ok().as_deref() {
            Some("production") => Ok(Self::Production),
            Some("development") => Ok(Self::Development),
            Some(_) => Err(HubError::UnsafeAuthConfiguration("OS_HUB_MODE must be development or production".into())),
            None if bind.is_loopback() => Ok(Self::Development),
            None => Ok(Self::Production),
        }
    }
}

#[derive(Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct HubReadinessV1 {
    pub(crate) schema: &'static str,
    pub(crate) status: &'static str,
    pub(crate) run_id: String,
    pub(crate) mode: &'static str,
    pub(crate) bind_scope: &'static str,
    pub(crate) authentication: HubAuthenticationReadinessV1,
    pub(crate) directory: HubComponentReadinessV1,
    pub(crate) storage: HubComponentReadinessV1,
    artifact_cas_barrier: HubComponentReadinessV1,
    pub(crate) artifact_publication: HubComponentReadinessV1,
    pub(crate) artifact_cas_sweeper: HubArtifactCasSweeperReadinessV1,
    pub(crate) artifact_authority: HubComponentReadinessV1,
    admin_assets: HubComponentReadinessV1,
    pub(crate) features: HubFeatureReadinessV1,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub(crate) blocked_by: Vec<HubClosedReadinessGateV1>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) startup: Option<HubStartupProgressV1>,
}

/// 📈️ How far a booting hub's startup work has come: the stage its trusted catalog load last
/// reported and that stage's units, and the catalog's per-package phases, bytes and rows once it has
/// selected its packages. Present only while the hub starts.
#[derive(Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct HubStartupProgressV1 {
    pub(crate) stage: &'static str,
    pub(crate) completed_units: u64,
    pub(crate) total_units: u64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) catalog: Option<TrustedCatalogLoadProgressV1>,
}

/// 🧭️ One named readiness gate that is holding `status` at `not-ready`, with the stable reason code a
/// waiting operator, probe or launcher reads instead of polling a silent boolean. `/readyz` omits the
/// list entirely once every required gate is open, so a ready hub's body is unchanged.
#[derive(Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct HubClosedReadinessGateV1 {
    pub(crate) gate: &'static str,
    pub(crate) reason: &'static str,
}

#[derive(Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct HubAuthenticationReadinessV1 {
    pub(crate) kind: &'static str,
    pub(crate) bootstrap_ready: bool,
    pub(crate) public_session_issuance: bool,
}

#[derive(Clone, Serialize)]
pub(crate) struct HubComponentReadinessV1 {
    pub(crate) ready: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) reason: Option<&'static str>,
}

impl HubComponentReadinessV1 {
    /// 🚦️ One gate: open gates carry no reason, a closed gate always names why it is closed.
    pub(crate) fn gate(ready: bool, reason: &'static str) -> Self {
        Self { ready, reason: if ready { None } else { Some(reason) } }
    }
}

#[derive(Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct HubArtifactCasSweeperReadinessV1 {
    pub(crate) ready: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) reason: Option<&'static str>,
    pub(crate) execute: bool,
    default_mode: &'static str,
}

#[derive(Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct HubFeatureReadinessV1 {
    pub(crate) open_plan: bool,
    pub(crate) open_plan_exchange: bool,
    pub(crate) rebootstrap: bool,
    pub(crate) mcp_workspace: bool,
    inference_services: Vec<HubInferenceServiceReadinessV1>,
    pub(crate) checkpoint_policy: HubCheckpointPolicyV1,
}

/// 📌️ `features.checkpointPolicy` (`LocalBootstrapReadinessV1`): the hub checks a document in by itself once the edits,
/// or the diff and inverse payload bytes, committed over its document sockets since it last did reach either bound — on behalf
/// of the author whose batch reached it, through the ordinary Check In. A client that seeds from the active checkpoint then
/// replays at most about one policy's worth of tail when it opens the document (ticket 26/09/23 session 14c: a 13 201-edit
/// document without a checkpoint could no longer be opened).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct HubCheckpointPolicyV1 {
    pub(crate) edits: u64,
    pub(crate) payload_bytes: u64,
}

/// 📌️ The declared default policy: 1 024 edits or 512 KiB of payload, well inside a browser client's pre-activation tail
/// retention (64 frames, 1 MiB).
pub(crate) const HUB_CHECKPOINT_POLICY: HubCheckpointPolicyV1 = HubCheckpointPolicyV1 { edits: 1_024, payload_bytes: 512 * 1024 };

pub(crate) const CHECKPOINT_POLICY_EDITS_ENV: &str = "OS_HUB_CHECKPOINT_POLICY_EDITS";

pub(crate) const CHECKPOINT_POLICY_BYTES_ENV: &str = "OS_HUB_CHECKPOINT_POLICY_BYTES";

impl HubCheckpointPolicyV1 {
    /// 🎚️ The operator's policy: each bound decimal within the schema's range, absent means the declared default, anything else
    /// fails boot.
    pub(crate) fn configured(edits: Option<&str>, payload_bytes: Option<&str>) -> Result<Self, String> {
        let bounded = |value: Option<&str>, name: &str, bounds: std::ops::RangeInclusive<u64>, default: u64| match value.map(str::trim).filter(|text| !text.is_empty()) {
            None => Ok(default),
            Some(text) => text.parse::<u64>().ok().filter(|bound| bounds.contains(bound)).ok_or_else(|| format!("{name} must be a decimal integer from {} to {}", bounds.start(), bounds.end())),
        };
        Ok(Self { edits: bounded(edits, CHECKPOINT_POLICY_EDITS_ENV, 1..=1_048_576, HUB_CHECKPOINT_POLICY.edits)?, payload_bytes: bounded(payload_bytes, CHECKPOINT_POLICY_BYTES_ENV, 4_096..=1_073_741_824, HUB_CHECKPOINT_POLICY.payload_bytes)? })
    }
}

/// 🧮️ What one written document committed since the hub last weighed its checkpoint policy, and whether a policy
/// Check In of it runs.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub(crate) struct CheckpointPolicyTallyV1 {
    pub(crate) edits: u64,
    pub(crate) payload_bytes: u64,
    pub(crate) running: bool,
}

/// 🧮️ How many documents one hub tallies at once; past it the tallies of documents with no Check In running restart.
const CHECKPOINT_POLICY_DOCUMENTS_MAX: usize = 65_536;

/// 🧮️ The per-document tallies of the checkpoint policy.
#[derive(Default)]
pub(crate) struct CheckpointPolicyLedgerV1 {
    pub(crate) documents: Mutex<std::collections::HashMap<DocumentScope, CheckpointPolicyTallyV1>>,
}

impl CheckpointPolicyLedgerV1 {
    /// ➕️ Adds one committed batch to `scope`'s tally: `true` when the tally reached `policy` and no policy Check In of the
    /// document runs — the caller starts one, the tally restarts and stays running until [`Self::finish`].
    pub(crate) fn observe(&self, scope: &DocumentScope, edits: u64, payload_bytes: u64, policy: HubCheckpointPolicyV1) -> bool {
        let mut documents = self.documents.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        if !documents.contains_key(scope) && documents.len() >= CHECKPOINT_POLICY_DOCUMENTS_MAX {
            documents.retain(|_, tally| tally.running);
            if documents.len() >= CHECKPOINT_POLICY_DOCUMENTS_MAX {
                return false;
            }
        }
        let tally = documents.entry(scope.clone()).or_default();
        tally.edits = tally.edits.saturating_add(edits);
        tally.payload_bytes = tally.payload_bytes.saturating_add(payload_bytes);
        if tally.running || (tally.edits < policy.edits && tally.payload_bytes < policy.payload_bytes) {
            return false;
        }
        *tally = CheckpointPolicyTallyV1 { edits: 0, payload_bytes: 0, running: true };
        true
    }

    /// 🏁️ Ends `scope`'s policy Check In; the edits committed meanwhile stay tallied.
    pub(crate) fn finish(&self, scope: &DocumentScope) {
        if let Some(tally) = self.documents.lock().unwrap_or_else(std::sync::PoisonError::into_inner).get_mut(scope) {
            tally.running = false;
        }
    }

    #[cfg(test)]
    pub(crate) fn tally(&self, scope: &DocumentScope) -> Option<CheckpointPolicyTallyV1> {
        self.documents.lock().unwrap_or_else(std::sync::PoisonError::into_inner).get(scope).copied()
    }
}

/// 💡️ One inference service THIS hub executes server-side, and the document-relative route family
/// its jobs are served under. Empty when the hub froze no inference binding; every other declared
/// service is executed by the client that holds the plugin's guest and committed through the
/// document ledger like any other edit.
#[derive(Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct HubInferenceServiceReadinessV1 {
    service_id: &'static str,
    pub(crate) route: &'static str,
}

/// 🤖️ The sentinel scope the agent-delegation readiness probe reads. It is deliberately not a
/// well-formed space or user id, so it can never collide with a real row in any data root, and the
/// probe's answer is therefore always the empty page — what is being read is whether the backend
/// implements the family at all, never what it holds.
pub(crate) const AGENT_DELEGATION_READINESS_PROBE_SCOPE: &str = "readiness-probe/mcp-workspace";

/// 🤖️ Whether a headless `semio-os-mcp --hub <url> --space <id> --credential-file <path>` workspace
/// can be served by THIS hub, as opposed to being compiled into it.
///
/// Both halves are what such a gateway actually asks for, in the order it asks:
/// `POST /auth/agent-sessions` exchanges the human's delegation for the agent's own session — which
/// needs a directory backend that implements the delegation family at all (sqlite does; postgres and
/// neo4j carry erroring defaults) — and then the authenticated descriptor/catalog binding resolves
/// a document open target, which is exactly what `open_plan` already answers for.
///
/// 🔒️ Never a constant. It read a hard-coded `false` from the day the field was introduced until
/// ticket 26/09/18 slice M8, which made a hub that could serve an agent and one that could not
/// publish the identical body — so `📓️c3-…` scored its "AI agent as third participant" step
/// **not run** on a hub that was in fact ready for the exchange, and the one honest consumer of the
/// field had no way to tell the two apart.
const fn mcp_workspace_ready(agent_delegation_ready: bool, open_plan_ready: bool) -> bool {
    agent_delegation_ready && open_plan_ready
}

pub(crate) fn hub_readiness(
    mode: HubMode,
    bind_scope: &'static str,
    run_id: String,
    bootstrap_ready: bool,
    artifact_authority_ready: bool,
    open_plan_ready: bool,
    agent_delegation_ready: bool,
    admin_assets_ready: bool,
    artifact_cas_barrier_ready: bool,
    artifact_cas_sweep_execute: bool,
    inference_ready: bool,
    artifact_authority_reason: &'static str,
) -> HubReadinessV1 {
    let inference_services = if inference_ready { vec![HubInferenceServiceReadinessV1 { service_id: semio_hub::inference::schema::GIS_SERVICE_ID, route: semio_hub::inference::schema::GIS_SERVICE_ROUTE }] } else { Vec::new() };
    let authentication_kind = match mode {
        HubMode::Development => "local-bootstrap-pipe-v1",
        HubMode::Production => "identity-assertion-verifier",
    };
    let bootstrap_reason = match mode {
        HubMode::Development => "local-bootstrap-pipe-handshake-incomplete",
        HubMode::Production => "identity-assertion-verifier-not-configured",
    };
    let required_ready = bootstrap_ready && artifact_authority_ready && admin_assets_ready && artifact_cas_barrier_ready;
    let blocked_by = [
        (bootstrap_ready, "authentication.bootstrapReady", bootstrap_reason),
        (artifact_cas_barrier_ready, "artifactCasBarrier", "artifact-cas-coordinator-barrier-closed"),
        (artifact_authority_ready, "artifactAuthority", artifact_authority_reason),
        (admin_assets_ready, "adminAssets", "admin-spa-dist-missing-run-os-hub-admin-build"),
    ]
    .into_iter()
    .filter(|(ready, ..)| !*ready)
    .map(|(_, gate, reason)| HubClosedReadinessGateV1 { gate, reason })
    .collect();
    HubReadinessV1 {
        schema: "semio.hub.readiness/v1",
        status: if required_ready { "ready" } else { "not-ready" },
        run_id,
        mode: match mode {
            HubMode::Development => "development",
            HubMode::Production => "production",
        },
        bind_scope,
        authentication: HubAuthenticationReadinessV1 { kind: authentication_kind, bootstrap_ready, public_session_issuance: false },
        directory: HubComponentReadinessV1::gate(true, "directory-backend-unavailable"),
        storage: HubComponentReadinessV1::gate(true, "document-storage-backend-unavailable"),
        artifact_cas_barrier: HubComponentReadinessV1::gate(artifact_cas_barrier_ready, "artifact-cas-coordinator-barrier-closed"),
        artifact_publication: HubComponentReadinessV1::gate(artifact_cas_barrier_ready, "artifact-cas-coordinator-barrier-closed"),
        artifact_cas_sweeper: HubArtifactCasSweeperReadinessV1 {
            ready: artifact_cas_barrier_ready,
            reason: if artifact_cas_barrier_ready { None } else { Some("artifact-cas-coordinator-barrier-closed") },
            execute: artifact_cas_sweep_execute,
            default_mode: "dry-run",
        },
        artifact_authority: HubComponentReadinessV1::gate(artifact_authority_ready, artifact_authority_reason),
        admin_assets: HubComponentReadinessV1::gate(admin_assets_ready, "admin-spa-dist-missing-run-os-hub-admin-build"),
        features: HubFeatureReadinessV1 {
            open_plan: open_plan_ready,
            open_plan_exchange: open_plan_ready,
            rebootstrap: true,
            mcp_workspace: mcp_workspace_ready(agent_delegation_ready, open_plan_ready),
            inference_services,
            checkpoint_policy: HUB_CHECKPOINT_POLICY,
        },
        blocked_by,
        startup: None,
    }
}

/// 🌅️ The readiness of a hub that has bound its socket but not yet opened its stores or loaded
/// its trusted catalog: every component gate closed with `hub-starting`, the artifact authority with
/// `trusted-catalog-loading`, and no feature served yet.
pub(crate) fn hub_starting_readiness(mode: HubMode, bind_scope: &'static str, run_id: String, bootstrap_ready: bool, credential_sign_in_enabled: bool, artifact_cas_sweep_execute: bool) -> HubReadinessV1 {
    let readiness = declare_public_session_issuance(hub_readiness(mode, bind_scope, run_id, bootstrap_ready, false, false, false, false, false, artifact_cas_sweep_execute, false, HUB_STARTING_CATALOG_REASON), credential_sign_in_enabled);
    let mut blocked_by = vec![HubClosedReadinessGateV1 { gate: "directory", reason: HUB_STARTING_REASON }, HubClosedReadinessGateV1 { gate: "storage", reason: HUB_STARTING_REASON }];
    blocked_by.extend(readiness.blocked_by.iter().filter(|closed| closed.gate == "authentication.bootstrapReady").cloned());
    blocked_by.extend([
        HubClosedReadinessGateV1 { gate: "artifactCasBarrier", reason: HUB_STARTING_REASON },
        HubClosedReadinessGateV1 { gate: "artifactAuthority", reason: HUB_STARTING_CATALOG_REASON },
        HubClosedReadinessGateV1 { gate: "adminAssets", reason: HUB_STARTING_REASON },
    ]);
    HubReadinessV1 {
        status: "not-ready",
        directory: HubComponentReadinessV1::gate(false, HUB_STARTING_REASON),
        storage: HubComponentReadinessV1::gate(false, HUB_STARTING_REASON),
        artifact_cas_barrier: HubComponentReadinessV1::gate(false, HUB_STARTING_REASON),
        artifact_publication: HubComponentReadinessV1::gate(false, HUB_STARTING_REASON),
        artifact_cas_sweeper: HubArtifactCasSweeperReadinessV1 { ready: false, reason: Some(HUB_STARTING_REASON), ..readiness.artifact_cas_sweeper.clone() },
        admin_assets: HubComponentReadinessV1::gate(false, HUB_STARTING_REASON),
        features: HubFeatureReadinessV1 { rebootstrap: false, ..readiness.features.clone() },
        blocked_by,
        ..readiness
    }
}

/// 🌅️ The closed-gate reason of a component a booting hub has not opened yet.
const HUB_STARTING_REASON: &str = "hub-starting";

/// 🌅️ The artifact authority's closed-gate reason while the trusted catalog loads.
const HUB_STARTING_CATALOG_REASON: &str = "trusted-catalog-loading";

/// 🗣️ The one line a hub prints about its own readiness at startup — `[INFO]` naming the bound
/// address when every required gate is open, `[WARN]` naming each closed gate and its stable reason
/// code otherwise. A hub that binds but never becomes ready must say which gate is holding it, since
/// an operator (and the collaboration harness) otherwise sees only an endless `/readyz` poll.
///
/// 🔒️ The `[INFO]` branch is gated on the **served `status`**, not only on the derived gate list, so
/// the printed line can never disagree with the `/readyz` body a probe reads. A 2026-09-20 runtime
/// probe caught a hub printing `[INFO] os-hub ready at …` while its own `/readyz` answered
/// `503 not-ready`; the two facts are now the same fact by construction, and a `not-ready` status
/// with no named gate still warns rather than claiming readiness.
pub(crate) fn startup_readiness_line(readiness: &HubReadinessV1, addr: &SocketAddr) -> String {
    if readiness.status == "ready" && readiness.blocked_by.is_empty() {
        return format!("[INFO] os-hub ready at http://{addr}");
    }
    let gates = if readiness.blocked_by.is_empty() {
        format!("status={}", readiness.status)
    } else {
        readiness.blocked_by.iter().map(|closed| format!("{}={}", closed.gate, closed.reason)).collect::<Vec<_>>().join(" ")
    };
    format!("[WARN] os-hub listening at http://{addr} but /readyz reports not-ready — closed gates: {gates}")
}

/// 📝️ The same readiness fact as [`startup_readiness_line`], in the stable-code shape a
/// `server.readiness` record's `detail` carries: `addr`, the bind scope and each closed gate by its
/// own reason code, never a prose sentence a collector would have to parse.
pub(crate) fn readiness_trace_detail(readiness: &HubReadinessV1, addr: &SocketAddr, bind_scope: &str) -> String {
    let mut detail = format!("addr={addr} scope={bind_scope}");
    for closed in &readiness.blocked_by {
        let _ = std::fmt::Write::write_fmt(&mut detail, format_args!(" blocked:{}={}", closed.gate, closed.reason));
    }
    detail
}

/// 🔐️ Stamps the ONE truthful answer to "can a browser obtain a session from this hub by
/// presenting a credential" onto a readiness snapshot. `hub_readiness` itself never claims it: a
/// hub only issues publicly when `OS_HUB_CREDENTIAL_SIGN_IN` enabled `POST /auth/sessions`, and the
/// local-bootstrap contract (`🚀️local-bootstrap/🧬️schema/🔣️.json`) requires a development hub
/// running the pipe to keep reporting `false`.
/// 📌️ Declares the operator's checkpoint policy in `features.checkpointPolicy`.
pub(crate) fn with_checkpoint_policy(mut readiness: HubReadinessV1, policy: HubCheckpointPolicyV1) -> HubReadinessV1 {
    readiness.features.checkpoint_policy = policy;
    readiness
}

pub(crate) fn declare_public_session_issuance(readiness: HubReadinessV1, credential_sign_in_enabled: bool) -> HubReadinessV1 {
    HubReadinessV1 { authentication: HubAuthenticationReadinessV1 { public_session_issuance: credential_sign_in_enabled, ..readiness.authentication }, ..readiness }
}
//#endregion 🔖️AdminAuth

//#region 🔖️AdminPage
/// 🫀️ Process liveness (`LocalBootstrapLivenessV1`, `🚀️local-bootstrap/🧬️schema`), deliberately independent of every
/// subsystem `/readyz` reports: an orchestrator restarts a process that stops answering here, and must NOT restart one that
/// is merely still warming up (a not-ready hub is a healthy hub that has not finished booting). The body therefore reads
/// nothing but the run identity and the process clock, and is answered `200` for as long as the axum task is scheduled at
/// all — by the boot server and by the full router alike.
#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct HubLivenessV1 {
    pub(crate) schema: &'static str,
    pub(crate) status: &'static str,
    pub(crate) run_id: String,
    uptime_ms: u64,
}

/// 💓️ The liveness body of the run `run_id` now.
pub(crate) fn hub_liveness(run_id: &str) -> HubLivenessV1 {
    HubLivenessV1 { schema: "semio.hub.liveness/v1", status: "live", run_id: run_id.to_string(), uptime_ms: HUB_PROCESS_START.elapsed().as_millis().min(u128::from(u64::MAX)) as u64 }
}

pub(crate) static HUB_PROCESS_START: std::sync::LazyLock<std::time::Instant> = std::sync::LazyLock::new(std::time::Instant::now);

/// 🌅️ What a hub answers while it boots. The listener is bound before the stores open and the trusted
/// catalog loads; until the full router takes over the very same socket, `/healthz` answers live,
/// `/readyz` answers `503 not-ready` with the catalog load's `startup` progress, and every other route
/// a signed `503 unavailable` with `Retry-After` — a caller sees a starting hub, never a refused
/// connection, and a load balancer or launcher can tell a slow boot from a dead one.
pub(crate) struct BootReadinessServerV1 {
    stop: Option<tokio::sync::oneshot::Sender<()>>,
    pub(crate) task: Option<tokio::task::JoinHandle<()>>,
}

#[derive(Clone)]
struct BootReadinessStateV1 {
    readiness: Arc<HubReadinessV1>,
    progress: StartupProgressCellV1,
}

impl BootReadinessServerV1 {
    /// 🌅️ Serves the booting hub on `listener` under the same transport, cross-origin and refusal
    /// layers as the full router.
    pub(crate) fn start(listener: tokio::net::TcpListener, readiness: HubReadinessV1, progress: StartupProgressCellV1, tracer: Tracer, cross_origin: CrossOriginPolicyV1, forwarded_tls: ForwardedTlsTrustV1) -> Self {
        std::sync::LazyLock::force(&HUB_PROCESS_START);
        let (stop, stopped) = tokio::sync::oneshot::channel::<()>();
        let app = Router::new()
            .route("/healthz", get(boot_healthz))
            .route("/readyz", get(boot_readyz))
            .fallback(boot_unavailable)
            .layer(axum::middleware::from_fn_with_state(cross_origin, cors_middleware))
            .layer(axum::middleware::from_fn_with_state(forwarded_tls, transport_security_middleware))
            .layer(axum::middleware::from_fn_with_state(tracer, refusal_middleware))
            .with_state(BootReadinessStateV1 { readiness: Arc::new(readiness), progress });
        let task = tokio::spawn(async move {
            let _ = axum::serve(listener, app.into_make_service_with_connect_info::<SocketAddr>())
                .with_graceful_shutdown(async move {
                    let _ = stopped.await;
                })
                .await;
        });
        Self { stop: Some(stop), task: Some(task) }
    }

    /// 🤝️ Stops accepting, lets the booting answers in flight finish, and releases the socket to the
    /// full router, which accepts on the same listener next.
    pub(crate) async fn hand_over(mut self) {
        if let Some(stop) = self.stop.take() {
            let _ = stop.send(());
        }
        if let Some(task) = self.task.take() {
            let _ = task.await;
        }
    }
}

impl Drop for BootReadinessServerV1 {
    fn drop(&mut self) {
        if let Some(task) = self.task.take() {
            task.abort();
        }
    }
}

async fn boot_healthz(State(state): State<BootReadinessStateV1>) -> impl IntoResponse {
    Json(hub_liveness(&state.readiness.run_id))
}

async fn boot_readyz(State(state): State<BootReadinessStateV1>) -> impl IntoResponse {
    let readiness = HubReadinessV1 { startup: state.progress.latest(), ..(*state.readiness).clone() };
    (StatusCode::SERVICE_UNAVAILABLE, Json(readiness))
}

async fn boot_unavailable() -> impl IntoResponse {
    (StatusCode::SERVICE_UNAVAILABLE, [(axum::http::header::RETRY_AFTER, "5")])
}

pub(crate) async fn get_healthz(State(state): State<HubState>) -> impl IntoResponse {
    Json(hub_liveness(&state.readiness.run_id))
}

pub(crate) async fn get_readyz(State(state): State<HubState>) -> impl IntoResponse {
    let mut readiness = (*state.readiness).clone();
    readiness.artifact_cas_sweeper.ready = state.artifact_maintenance.healthy();
    if !readiness.artifact_cas_sweeper.ready {
        readiness.status = "not-ready";
        readiness.artifact_cas_sweeper.reason = Some("artifact-cas-maintenance-supervisor-failed-closed");
        readiness.blocked_by.push(HubClosedReadinessGateV1 { gate: "artifactCasSweeper", reason: "artifact-cas-maintenance-supervisor-failed-closed" });
    }
    let status = if readiness.status == "ready" { StatusCode::OK } else { StatusCode::SERVICE_UNAVAILABLE };
    (status, Json(readiness))
}
//#endregion 🔖️AdminPage
