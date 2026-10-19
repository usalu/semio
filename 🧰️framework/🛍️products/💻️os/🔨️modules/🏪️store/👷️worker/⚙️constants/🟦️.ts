import { retryWithJitteredBackoff } from "@semio-tech/framework";

export const workerScope = typeof self !== "undefined" && !Reflect.has(self, "document") ? self : null;

//#region 🔖️Constants
/** 🛰️ Must match `framework/os/core/js/index.ts`'s `BACKBONE_ENDPOINT_PATH`. */
export const FOLDER_ENDPOINT_PATH = "/semio-backbone";
export const CANONICAL_BOOTSTRAP_FOLDER_MIRROR_PATH = `${FOLDER_ENDPOINT_PATH}/canonical-bootstrap`;
/** 🛟️ Sanity-fallback poll cadence (finding 1): the folder's change stream is the primary wake signal, so this only
 * ever fires while {@link ArtifactState.watchHealthy} is `false` — a slow, jittered self-heal while no stream is open, not
 * the primary path. Jittered per tick (not a fixed `setInterval`) so many documents self-healing together never
 * synchronize into a request burst. */
export const SANITY_POLL_MIN_MS = 24_000;
export const SANITY_POLL_MAX_MS = 36_000;
/** ⏱️ Caps how long any single folder/blob fetch can hang (finding 3) — composed with
 * {@link fetchWithTimeout} so a stalled dev-middleware response can never pin a document forever. */
export const FOLDER_FETCH_TIMEOUT_MS = 15_000;
export const BLOB_FETCH_TIMEOUT_MS = 15_000;
/** 🗃️ Bounded local outbound-mutation queue (finding 5) — see {@link rejectMutationQueueOverflow}
 * for the overflow contract: reject and report, never silently drop. */
export const PENDING_MUTATIONS_QUEUE_LIMIT = 2_000;
export const ARTIFACT_BOOTSTRAP_DEADLINE_MS = 15_000;
export const ARTIFACT_BOOTSTRAP_DIAGNOSTIC_MAX_BYTES = 4_096;
export const EXECUTION_TARGET_DIAGNOSTIC_MAX_BYTES = 1_024;
// 🔁️ HUB_RECONNECT_MIN_MS/MAX_MS moved to `🟦️.ts`'s `🔖️HubBinding` region (imported above)
// — single source of truth shared with `DirectoryClient.stream`'s reconnect loop.
/** ♻️ Coordinator follow-up (finding 4b): how long a hub connection must stay open before a
 * SUBSEQUENT drop is allowed to reset that transport's backoff back near its floor, instead of
 * continuing to grow from whatever `retryWithJitteredBackoff` attempt count it was already on.
 * Deliberately NOT "the socket opened" — a server that accepts a connection and immediately drops
 * it in a fast loop must still see the backoff climb (that IS the failure mode the backoff exists
 * for), so the threshold has to be comfortably longer than any such instant-drop cycle. Half of
 * {@link HUB_RECONNECT_MAX_MS} (30s): long enough that no
 * single accept-then-drop attempt could plausibly cross it, short enough that a connection which
 * has been genuinely healthy for a modest stretch still gets credit before its next blip. */
export const SUSTAINED_HEALTHY_MS = 15_000;

//#endregion 🔖️Constants

/**
 * ♻️ Coordinator follow-up (finding 4b): drives `attempt` (one physical connection's full
 * lifecycle — connect, stay open, eventually close) through {@link retryWithJitteredBackoff}
 * forever, but as a LOOP of fresh calls rather than one long-lived call. `attempt` resolving is
 * this loop's signal that the connection stayed open long enough to count as sustainedly healthy
 * before it (ordinarily) closed — see {@link connectHubOnce} — so the NEXT
 * cycle starts a brand-new {@link retryWithJitteredBackoff} call with its own zeroed internal
 * attempt/backoff state, rather than inheriting a large accumulated delay from earlier, already-
 * resolved blips. `attempt` rejecting (a close before sustained health) is absorbed entirely
 * inside the SAME `retryWithJitteredBackoff` call, so its backoff keeps growing across those —
 * exactly the "rapid accept-then-drop cycling still backs off" case the sustained-health gate
 * exists to protect.
 *
 * `retryWithJitteredBackoff`'s own signature has no notion of "reset now" — it only stops
 * retrying on success or abort — so this reset cannot be expressed by calling it once; looping
 * fresh calls from here is the only way to get a real reset without editing
 * `🧰️framework/📦️packages/🟦️typescript/🟦️.ts` (outside this packet's owned path).
 */
export async function reconnectForever(signal: AbortSignal, attempt: () => Promise<void>, minMs: number, maxMs: number): Promise<void> {
  while (!signal.aborted) {
    try {
      await retryWithJitteredBackoff(attempt, { minMs, maxMs, signal });
    } catch {
      return; // 🛑 only reachable via abort — `attempt` never lets a real failure escape the retry loop above.
    }
  }
}

export const SOCKET_GRANT_REQUEST_TIMEOUT_MS = 10_000;

/** 📶️ Bounded streaming progress unit for a verified execution-target body. */
export const EXECUTION_TARGET_PROGRESS_UNIT_BYTES = 64 * 1024;
/** 🧯️ Bound on the strict lease manifest JSON body. */
export const EXECUTION_TARGET_MANIFEST_MAX_BYTES = 8 * 1024;

export const documentExecutionTargetLeaseMintToken = Symbol("semio.os.document-execution-target-lease.mint/v1");
export const documentExecutionTargetLeaseBrand = Symbol("semio.os.document-execution-target-lease/v1");
