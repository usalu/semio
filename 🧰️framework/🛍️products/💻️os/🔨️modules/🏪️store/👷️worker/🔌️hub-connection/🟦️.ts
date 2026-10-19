import type { PersistenceBinding } from "../../../../🟦️";
import { closeHubSocketV1 } from "@semio-tech/framework/directory/client/socket-close";
import { HUB_RECONNECT_MIN_MS, HUB_RECONNECT_MAX_MS } from "../../../../🟦️";
import { extractServerCommandsDocumentBackboneBatchExact, decodeServerFrame } from "@semio-tech/framework-replication";
import { type ArtifactState, emitEvent } from "../🧾️document-state/🟦️.ts";
import { requestDocumentSocketAuthority, setRemote, suspendDocumentBrowserActorLink } from "../🎭️actor-lifecycle/🟦️.ts";
import { hubSessionCapability } from "../🔐️hub-session/🟦️.ts";
import { SUSTAINED_HEALTHY_MS, reconnectForever } from "../⚙️constants/🟦️.ts";
import { sendWireFrame } from "../🛰️hub-relay/🟦️.ts";
import { handleHubFrame } from "../📡️hub-frames/🟦️.ts";
import { rejectArtifactBootstrap } from "../🚀️artifact-bootstrap/🟦️.ts";
import { retireMountedDocumentChild } from "../📥️execution-target-read/🟦️.ts";
import { abortArtifactBootstrap } from "../🛑️session-teardown/🟦️.ts";
import { requeuePendingBatches } from "../📮️hub-outbox/🟦️.ts";

/** 🔌️ One hub WebSocket connection attempt (finding 4) — resolves either once
 * {@link ArtifactState.docAbort} fires (a clean shutdown) OR once an ordinary close follows at
 * least {@link SUSTAINED_HEALTHY_MS} of unbroken uptime (coordinator follow-up, finding 4b: tells
 * {@link reconnectForever} this cycle counts as healthy, so the NEXT reconnect starts with a
 * fresh, reset backoff instead of inheriting this session's earlier accumulated delay); rejects
 * on every OTHER close (closed before reaching sustained health) so the caller's
 * {@link retryWithJitteredBackoff} loop keeps backing off within the SAME call, never resetting,
 * against a server that accepts and immediately drops connections in a loop — resetting on
 * "socket opened" alone would defeat the backoff entirely against exactly that failure mode. Any
 * batch the dying socket never acked is moved back into {@link ArtifactState.outbox} before the
 * retry either way, rather than left stranded in `pendingBatches` forever (finding 5 — a dead
 * socket will never deliver that `Ack`). */
export async function connectHubOnce(state: ArtifactState, binding: Extract<PersistenceBinding, { kind: "hub", dataClass: "persistedShared" }>): Promise<void> {
  const authority = await requestDocumentSocketAuthority(state, binding);
  const receipt = authority.receipt;
  if (receipt.expiresAtMs <= Date.now()) throw new Error("backbone-worker: expired socket grant");
  return new Promise<void>((resolve, reject) => {
    if (state.docAbort.signal.aborted) {
      reject(state.docAbort.signal.reason ?? new Error("backbone-worker: document closed"));
      return;
    }
    state.actor = "";
    state.hubActorReady = false;
    state.pendingSocketActorId = receipt.actorId;
    setRemote(state, { kind: "connecting" });
    const wsBase = binding.baseUrl.replace(/^http/, "ws");
    const scope = `${binding.spaceId}/${state.config.documentId}`;
    const query = new URLSearchParams();
    if (authority.surfaceId) query.set("surface", authority.surfaceId);
    if (hubSessionCapability === undefined) throw new Error("backbone-worker: missing hub session");
    const encoded = query.toString();
    const suffix = encoded.length > 0 ? `?${encoded}` : "";
    const socket = new WebSocket(`${wsBase}/scopes/${encodeURIComponent(scope)}/document/ws${suffix}`, ["semio.session.v1", hubSessionCapability]);
    const presenceCandidate = authority.surfaceId ? { socket, scope: { spaceId: binding.spaceId, documentId: state.config.documentId }, verifiedSurfaceId: authority.surfaceId } : null;
    socket.binaryType = "arraybuffer";
    state.socket = socket;
    let sustainedHealthTimer: ReturnType<typeof setTimeout> | null = null;
    let sustainedHealthReached = false;
    const onAbort = (): void => socket.close();
    state.docAbort.signal.addEventListener("abort", onAbort, { once: true });
    socket.onopen = () => {
      if (socket.protocol !== "semio.session.v1") {
        closeHubSocketV1(socket, "protocolMismatch");
        return;
      }
      state.reconnectDelayMs = HUB_RECONNECT_MIN_MS;
      sustainedHealthTimer = setTimeout(() => {
        sustainedHealthReached = true;
      }, SUSTAINED_HEALTHY_MS);
      sendWireFrame(
        state,
        {
          SocketHelloV1: {
            wire_version: 1,
            protocol_version: 1,
            schema: authority.schema,
            pack_schema_hash: [...authority.packSchemaHash],
            resume_token: state.resumeToken,
            frontier: state.frontier,
          },
        },
        "command",
      );
    };
    socket.onmessage = (messageEvent) => {
      state.hubFrameChain = state.hubFrameChain.then(async () => {
        if (state.socket !== socket) return;
        try {
          const bytes = new Uint8Array(messageEvent.data as ArrayBuffer);
          const commandBatch = extractServerCommandsDocumentBackboneBatchExact(bytes);
          await handleHubFrame(state, decodeServerFrame(bytes).frame, presenceCandidate, socket, commandBatch);
        } catch (error) {
          console.error("[backbone-worker] malformed hub frame", state.config.documentId, error);
          rejectArtifactBootstrap(state, error);
        }
      });
    };
    socket.onclose = () => {
      state.docAbort.signal.removeEventListener("abort", onAbort);
      if (sustainedHealthTimer != null) clearTimeout(sustainedHealthTimer);
      if (state.socket !== socket) {
        resolve();
        return;
      }
      if (state.socket === socket) {
        if (state.presenceAuthority?.socket === socket) emitEvent(state, { kind: "presence", peers: [] });
        state.presenceAuthority = null;
        state.socket = null;
        state.actor = "";
        state.hubActorReady = false;
        state.pendingSocketActorId = null;
        if (!suspendDocumentBrowserActorLink(state, binding)) retireMountedDocumentChild(state);
        abortArtifactBootstrap(state);
        requeuePendingBatches(state);
      }
      if (state.docAbort.signal.aborted) {
        resolve();
        return;
      }
      if (sustainedHealthReached || state.artifactRebootstrapRequired || state.actorRecoveryRequested) {
        state.actorRecoveryRequested = false;
        state.reconnectDelayMs = HUB_RECONNECT_MIN_MS;
        setRemote(state, { kind: "backoff", retryInMs: 0 });
        resolve();
        return;
      }
      setRemote(state, { kind: "backoff", retryInMs: state.reconnectDelayMs });
      state.reconnectDelayMs = Math.min(state.reconnectDelayMs * 2, HUB_RECONNECT_MAX_MS);
      reject(new Error("backbone-worker: hub socket closed"));
    };
    socket.onerror = () => socket.close();
  });
}

/** 🔁️ Reconnect loop entry point — one call per document lifetime (from {@link openArtifact}),
 * looping via {@link reconnectForever} until {@link ArtifactState.docAbort} fires. Full jitter
 * (finding 4) avoids a thundering herd when several documents' hub connections drop together
 * (e.g. a hub restart); the sustained-health reset (finding 4b, {@link connectHubOnce}) keeps a
 * long-healthy session from inheriting a large accumulated backoff on its next ordinary blip. */
export function connectHub(state: ArtifactState, binding: Extract<PersistenceBinding, { kind: "hub", dataClass: "persistedShared" }>): void {
  if (state.closed) return;
  void reconnectForever(state.docAbort.signal, () => connectHubOnce(state, binding), HUB_RECONNECT_MIN_MS, HUB_RECONNECT_MAX_MS);
}
