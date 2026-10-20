/** 🌐️ Backbone envelope IO: bounded, jittered-retry reads and writes of document envelopes over the backbone HTTP endpoint. */

import type { FetchTimeoutResponse } from "@semio-tech/framework";
import { fetchWithTimeout } from "../../🚪️io/🌐️fetch-timeout/🟦️.ts";
import { retryWithJitteredBackoff } from "../../⏳️async/🔁️jittered-backoff/🟦️.ts";
import { BACKBONE_ENDPOINT_PATH, BACKBONE_OCTET_STREAM, parseRemoteBackboneUri, remoteEnvelopeUrl } from "../🟦️.ts";

//#region 🔖️Backbone
//#region 🌐️BackboneEnvelopeIo
// 🎫️ ticket 26/08/17/MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME, packet `web-directory`, finding 3 —
// bounded timeout + jittered retry so a hung/unreachable backbone degrades instead of hanging the
// caller forever; see the read/write docstrings below for which of the two is actually safe to retry
// and why.
const BACKBONE_ENVELOPE_HTTP_TIMEOUT_MS = 10_000;
const BACKBONE_ENVELOPE_RETRY_MIN_MS = 500;
const BACKBONE_ENVELOPE_RETRY_MAX_MS = 5_000;
/** 🪟️ Overall ceiling on {@link readBackboneEnvelope}'s retry loop — `retryWithJitteredBackoff` on
 * its own retries forever until its `signal` aborts, which is wrong for something a caller is
 * awaiting: a permanently unreachable backbone must eventually surface as a rejection (local-first —
 * the backbone is an enhancement, not a blocking prerequisite) rather than hang the caller alongside
 * whatever real outage is happening. */
export const BACKBONE_ENVELOPE_RETRY_WINDOW_MS = 15_000;

/** 📨️ {@link FetchTimeoutResponse} plus the one extra accessor this module needs (binary bodies) —
 * declared locally rather than widening the shared glue type, per this module's own body accessing
 * only what it uses. `fetchWithTimeout`'s actual runtime value is a real `fetch` `Response`, so the
 * cast is safe. */
interface BackboneFetchResponse extends FetchTimeoutResponse {
  arrayBuffer(): Promise<ArrayBuffer>;
}

/** 🚨️ Marks a backbone envelope failure as "the server answered, definitively" (any status code) —
 * as opposed to a transport-level failure (thrown by `fetch`/{@link fetchWithTimeout} itself: DNS,
 * connection refused, or {@link BACKBONE_ENVELOPE_HTTP_TIMEOUT_MS} timing out) where the request
 * never definitively reached or was answered by the server. Only the latter is safe to retry blindly
 * — a real response, even an error one, is a final answer and retrying it would just repeat the same
 * definitive outcome while burning the retry window. */
class BackboneEnvelopeResponseError extends Error {}

async function readBackboneEnvelopeOnce(uri: string, signal: AbortSignal): Promise<Uint8Array | null> {
  if (uri.startsWith("remote://")) {
    const remote = parseRemoteBackboneUri(uri);
    if (!remote) return null;
    const response = (await fetchWithTimeout(remoteEnvelopeUrl(remote), undefined, { timeoutMs: BACKBONE_ENVELOPE_HTTP_TIMEOUT_MS, signal })) as BackboneFetchResponse;
    if (response.status === 404) return null;
    if (!response.ok) throw new BackboneEnvelopeResponseError(`remote backbone read failed (${response.status})`);
    return new Uint8Array(await response.arrayBuffer());
  }
  const response = (await fetchWithTimeout(`${BACKBONE_ENDPOINT_PATH}?uri=${encodeURIComponent(uri)}`, undefined, { timeoutMs: BACKBONE_ENVELOPE_HTTP_TIMEOUT_MS, signal })) as BackboneFetchResponse;
  if (response.status === 204 || response.status === 404) return null;
  if (!response.ok) throw new BackboneEnvelopeResponseError(`backbone read failed (${response.status})`);
  return new Uint8Array(await response.arrayBuffer());
}

/** 🌐️ Reads the raw bundle bytes at `uri`, or `null` for a 204 (a real, final "nothing written here
 * yet" answer — never retried) or a 404 (no backbone at this origin). Optional `signal` cancels the whole read, including any retry in
 * progress. RETRY-SAFE, and retried: a read has no side effect, so re-issuing it on a transport-level
 * failure (see {@link BackboneEnvelopeResponseError}) can never duplicate an effect — only a
 * definitive server response (any status) or the caller's `signal` skips further retries. Retries are
 * jittered ({@link retryWithJitteredBackoff}) and bounded by {@link BACKBONE_ENVELOPE_RETRY_WINDOW_MS}
 * overall, so a permanently unreachable backbone rejects instead of hanging the caller forever.
 * Liest die rohen Bundle-Bytes für `uri`, oder `null` bei 404. Ein Lesevorgang hat keinen Seiteneffekt
 * und wird deshalb bei einem Transportfehler sicher wiederholt (mit Jitter, zeitlich begrenzt). */
export async function readBackboneEnvelope(uri: string, signal?: AbortSignal): Promise<Uint8Array | null> {
  const retryAbort = new AbortController();
  if (signal?.aborted) retryAbort.abort(signal.reason);
  const onCallerAbort = (): void => retryAbort.abort(signal!.reason);
  signal?.addEventListener("abort", onCallerAbort, { once: true });
  const windowTimer = setTimeout(() => retryAbort.abort(new Error(`backbone read: retry window exceeded after ${BACKBONE_ENVELOPE_RETRY_WINDOW_MS}ms`)), BACKBONE_ENVELOPE_RETRY_WINDOW_MS);
  try {
    return await retryWithJitteredBackoff(
      async () => {
        try {
          return await readBackboneEnvelopeOnce(uri, retryAbort.signal);
        } catch (error) {
          if (error instanceof BackboneEnvelopeResponseError && !retryAbort.signal.aborted) retryAbort.abort(error);
          throw error;
        }
      },
      { minMs: BACKBONE_ENVELOPE_RETRY_MIN_MS, maxMs: BACKBONE_ENVELOPE_RETRY_MAX_MS, signal: retryAbort.signal },
    );
  } finally {
    clearTimeout(windowTimer);
    signal?.removeEventListener("abort", onCallerAbort);
  }
}

/** 🌐️ Writes the full bundle bytes for `uri`, replacing whatever was there. Optional `signal` bounds
 * the request via {@link fetchWithTimeout} ({@link BACKBONE_ENVELOPE_HTTP_TIMEOUT_MS}) so a hung
 * backbone cannot freeze a caller — but, deliberately, this call is NOT retried on failure. A `PUT`
 * here always carries the caller's complete current bundle, which makes a same-bytes retry look
 * idempotent at first glance, but this function has no visibility into the server's actual write
 * semantics (a pure last-write-wins slot vs. one that appends a history/audit entry per accepted
 * write), and "the request timed out" gives no way to distinguish "never arrived" from "arrived and
 * applied, only the response never came back". Retrying blindly risks silently double-applying a
 * write whose effect this client cannot observe well enough to rule that out — worse than surfacing
 * the failure and letting the caller (which already treats a rejected write as "still local-only, try
 * the whole save again later") decide. If the server's replace semantics are ever made provably
 * idempotent end-to-end, add {@link retryWithJitteredBackoff} here to match {@link
 * readBackboneEnvelope} — not before.
 * Schreibt die vollständigen Bundle-Bytes für `uri`; wird bei einem Fehler bewusst NICHT wiederholt,
 * da ein doppelt angewandter Schreibvorgang nicht sicher ausgeschlossen werden kann. */
export async function writeBackboneEnvelope(uri: string, bundle: Uint8Array, signal?: AbortSignal): Promise<void> {
  const body = Uint8Array.from(bundle).buffer;
  if (uri.startsWith("remote://")) {
    const remote = parseRemoteBackboneUri(uri);
    if (!remote) throw new Error(`invalid remote backbone uri: ${uri}`);
    const response = await fetchWithTimeout(
      remoteEnvelopeUrl(remote),
      { method: "PUT", headers: { "content-type": BACKBONE_OCTET_STREAM }, body },
      { timeoutMs: BACKBONE_ENVELOPE_HTTP_TIMEOUT_MS, signal },
    );
    if (!response.ok) throw new Error(`remote backbone write failed (${response.status})`);
    return;
  }
  const response = await fetchWithTimeout(
    `${BACKBONE_ENDPOINT_PATH}?uri=${encodeURIComponent(uri)}`,
    { method: "PUT", headers: { "content-type": BACKBONE_OCTET_STREAM }, body },
    { timeoutMs: BACKBONE_ENVELOPE_HTTP_TIMEOUT_MS, signal },
  );
  if (!response.ok) throw new Error(`backbone write failed (${response.status})`);
}
//#endregion 🌐️BackboneEnvelopeIo
//#endregion 🔖️Backbone
