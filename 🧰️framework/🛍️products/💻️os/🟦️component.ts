// #region Header
/**
 * 🖥️ `@semio-tech/framework-os` — JS sync/backbone protocol surface (backbone URIs, document
 * envelopes, `🟦️backbone-worker.ts` request/response wire types, `PersistenceBinding`/`MutationEnvelope`,
 * {@link buildFrameworkSyncUtilities}) consumed by `framework/os/renderer/js/react/index.tsx` and
 * `framework/os/dev/script.ts`. The OS kernel's *stateful* logic (operation application, program
 * registry) is Rust/wasm-only, hosted by the s-plugin wasm — this file is not a JS port of that. The
 * one exception is {@link planWorkflow}: a pure, side-effect-free scheduling function has no state
 * to keep in sync with a live wasm host, so it's hand-mirrored here against the Rust `plan_workflow`
 * (`framework/os/core/rs/lib.rs`) with shared fixtures (`framework/os/core/fixtures/`)
 * asserting parity.
 */
// #endregion Header

import type { Conflict, ConflictResolution, DispatchReport, Fault, FetchTimeoutResponse, MergePolicy, MergeReport, MutationMessage, PluginWasmHandle, TurnOutcome, UtilityLeaf } from "@semio-tech/framework";
import { conflictResolutionAsU8, createTurnOutcomeBroadcast, fetchWithTimeout, mergePolicyAsU8, retryWithJitteredBackoff } from "@semio-tech/framework";
/** 📇️ Directory event/command/DTO types (contract-freeze §C1/§C6) — imported once here for
 * {@link BackboneWorkerRequest}/{@link BackboneWorkerResponse}'s `directory-*` variants and this
 * file's `🔖️HubBinding` region; never redeclared (lane 0-A owns the type source). */
import type { DirectoryCommand, DirectoryEvent, DirectoryStreamMessage } from "./🔨️modules/📇️directory/🟦️component.ts";
/** 📡️ The replication wire contract lives in `🧰️framework/🔨️modules/📡️replication` — os speaks it,
 * it is not os-owned. Frames/envelopes/presence peers all come from there. */
import type { ArtifactPresencePeer, ClientFrame, LocalInteractionIdentity, LocalInteractionPage, LocalInteractionQueryCommand, LocalInteractionQueryReply, LocalInteractionQueryToken, MutationEnvelope, ServerFrame, WireAckStage, WireFrontierSummary, WireLane, WireMutationEnvelope } from "@semio-tech/framework-replication";
import { decodeClientFrame, decodeLocalInteractionQueryCommand, decodeLocalInteractionQueryReply, decodePresencePeer, decodeServerFrame, encodeClientFrame, encodeLocalInteractionQueryCommand, encodeLocalInteractionQueryReply, encodePresencePeer, encodeServerFrame, localInteractionIdentityEquals, mutationEnvelopeFromWire, mutationEnvelopeToWire } from "@semio-tech/framework-replication";
/** 🔢️ Shared byte-codec floor — the same primitives the wire frames are built from; os reuses them
 * for its backbone-envelope and app-channel codecs rather than keeping a second copy. */
import { decodeCausalEnvelopeBatch, encodeCausalEnvelopeBatch, readBool, readBytes, readF64, readHash32, readStr, readU8, readVarintU64, readVecBytes, readVecEnvelope, readVecStr, writeBool, writeBytes, writeF64, writeHash32, writeStr, writeVarintU64, writeVecBytes, writeVecEnvelope, writeVecStr } from "@semio-tech/framework-replication";

const replicationPackCodec = { encode: encodePackValue, decode: decodePackValue };

//#region 🔖️Backbone
export const FRAMEWORK_SYNC_CONTROLLER_ID = "framework.sync";

/** 🛰️ Dev-server-proxied backbone endpoint path for `file://`/`folder://` uris; shared with the dev host shim (`framework/os/dev/script.ts`) so both stay in sync on the same literal. */
export const BACKBONE_ENDPOINT_PATH = "/semio-backbone";

export type BackboneKind = "file" | "folder" | "remote" | "unknown";

export type ArtifactBackboneRef = {
  readonly kind: BackboneKind;
  readonly uri: string;
};

export function backboneKindFromUri(uri: string): BackboneKind {
  if (uri.startsWith("file://")) return "file";
  if (uri.startsWith("folder://")) return "folder";
  if (uri.startsWith("remote://")) return "remote";
  return "unknown";
}

export function artifactBackboneRef(uri: string): ArtifactBackboneRef {
  return { kind: backboneKindFromUri(uri), uri };
}

export function parseRemoteBackboneUri(uri: string): { readonly hostPort: string; readonly spaceId: string; readonly documentId: string } | null {
  if (!uri.startsWith("remote://")) return null;
  const rest = uri.slice("remote://".length);
  const firstSlash = rest.indexOf("/");
  if (firstSlash <= 0) return null;
  const secondSlash = rest.indexOf("/", firstSlash + 1);
  if (secondSlash <= 0) return null;
  return { hostPort: rest.slice(0, firstSlash), spaceId: rest.slice(firstSlash + 1, secondSlash), documentId: rest.slice(secondSlash + 1) };
}

export function buildRemoteBackboneUri(hostPort: string, spaceId: string, documentId: string): string {
  return `remote://${hostPort}/${spaceId}/${documentId}`;
}

export function buildFileBackboneUri(path: string): string {
  const normalized = path.startsWith("/") ? path : `/${path}`;
  return `file://${normalized}`;
}

export function buildFolderBackboneUri(path: string): string {
  const normalized = path.startsWith("/") ? path : `/${path}`;
  return `folder://${normalized}`;
}

function remoteEnvelopeUrl(remote: { readonly hostPort: string; readonly spaceId: string; readonly documentId: string }): string {
  return `http://${remote.hostPort}/spaces/${encodeURIComponent(remote.spaceId)}/documents/${encodeURIComponent(remote.documentId)}/envelope`;
}

/** @emoji 🔌️ `store::encode_document_pack_bytes` — length-prefixed `pack` then raw `spr`. */
export function encodeDocumentPackBytes(pack: Uint8Array, spr: Uint8Array): Uint8Array {
  const out: number[] = [];
  writeVarintU64(out, pack.length);
  for (const byte of pack) out.push(byte);
  for (const byte of spr) out.push(byte);
  return new Uint8Array(out);
}

/** @emoji 🎯️ Inverse of {@link encodeDocumentPackBytes}. */
export function decodeDocumentPackBytes(bytes: Uint8Array): { readonly pack: Uint8Array; readonly spr: Uint8Array } {
  const pos: [number] = [0];
  const packLen = readVarintU64(bytes, pos);
  const packEnd = pos[0] + packLen;
  if (packEnd > bytes.length) throw new Error("document pack bytes truncated");
  const pack = bytes.subarray(pos[0], packEnd);
  pos[0] = packEnd;
  return { pack, spr: bytes.subarray(pos[0]) };
}

/** @emoji 📦️ Packs a snapshot value into a document bundle (`pack` + `spr`). */
export function encodeDocumentPackBundle(snapshot: unknown, spr: Uint8Array = new Uint8Array()): Uint8Array {
  return encodeDocumentPackBytes(encodePackValue(snapshot), spr);
}

/** @emoji 📥️ Decodes the snapshot from a document bundle (ignores `spr` history). */
export function decodeDocumentPackSnapshot(bundle: Uint8Array): unknown {
  const { pack } = decodeDocumentPackBytes(bundle);
  return decodePackValue(pack);
}

const BACKBONE_OCTET_STREAM = "application/octet-stream";

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
const BACKBONE_ENVELOPE_RETRY_WINDOW_MS = 15_000;

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
  if (response.status === 404) return null;
  if (!response.ok) throw new BackboneEnvelopeResponseError(`backbone read failed (${response.status})`);
  return new Uint8Array(await response.arrayBuffer());
}

/** 🌐️ Reads the raw bundle bytes at `uri`, or `null` for a 404 (a real, final "nothing written here
 * yet" answer — never retried). Optional `signal` cancels the whole read, including any retry in
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
          // 🛟️ a definitive server response ends the retry loop immediately (via the abort reason)
          // instead of being retried like a transport failure — see `BackboneEnvelopeResponseError`'s
          // docstring.
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

if (import.meta.vitest) {
  const { afterEach, describe, expect, it, vi } = import.meta.vitest;

  describe("backbone envelope io", () => {
    const originalFetch = globalThis.fetch;
    afterEach(() => {
      globalThis.fetch = originalFetch;
      vi.useRealTimers();
    });

    it("readBackboneEnvelope retries a transient transport failure and then succeeds, with no real sleep", async () => {
      vi.useFakeTimers();
      let calls = 0;
      globalThis.fetch = vi.fn(async () => {
        calls += 1;
        if (calls < 3) throw new Error("connection refused");
        return { ok: true, status: 200, arrayBuffer: async () => new Uint8Array([1, 2, 3]).buffer } as unknown as Response;
      }) as unknown as typeof fetch;
      const promise = readBackboneEnvelope("folder:///doc");
      await vi.runAllTimersAsync();
      const result = await promise;
      expect(calls).toBe(3);
      expect(Array.from(result ?? [])).toEqual([1, 2, 3]);
    });

    it("readBackboneEnvelope does NOT retry a definitive non-404 server response", async () => {
      let calls = 0;
      globalThis.fetch = vi.fn(async () => {
        calls += 1;
        return { ok: false, status: 500, arrayBuffer: async () => new ArrayBuffer(0) } as unknown as Response;
      }) as unknown as typeof fetch;
      // 🪧️ no intervening `await` between creating the promise and `.rejects` consuming it — a
      // definitive failure settles in one microtask hop, with no timer involved at all.
      await expect(readBackboneEnvelope("folder:///doc")).rejects.toThrow("backbone read failed (500)");
      expect(calls).toBe(1);
    });

    it("readBackboneEnvelope gives up after its retry window instead of hanging forever", async () => {
      vi.useFakeTimers();
      globalThis.fetch = vi.fn(async () => {
        throw new Error("connection refused");
      }) as unknown as typeof fetch;
      const promise = readBackboneEnvelope("folder:///doc");
      let settled = false;
      promise.then(
        () => (settled = true),
        () => (settled = true),
      );
      await vi.advanceTimersByTimeAsync(BACKBONE_ENVELOPE_RETRY_WINDOW_MS + 1_000);
      expect(settled).toBe(true);
      await expect(promise).rejects.toThrow();
    });

    it("readBackboneEnvelope returns null on 404 without retrying", async () => {
      let calls = 0;
      globalThis.fetch = vi.fn(async () => {
        calls += 1;
        return { ok: false, status: 404, arrayBuffer: async () => new ArrayBuffer(0) } as unknown as Response;
      }) as unknown as typeof fetch;
      const result = await readBackboneEnvelope("folder:///doc");
      expect(result).toBeNull();
      expect(calls).toBe(1);
    });

    it("writeBackboneEnvelope does NOT retry on transport failure (duplicate-write safety)", async () => {
      let calls = 0;
      globalThis.fetch = vi.fn(async () => {
        calls += 1;
        throw new Error("connection refused");
      }) as unknown as typeof fetch;
      await expect(writeBackboneEnvelope("folder:///doc", new Uint8Array([1]))).rejects.toThrow("connection refused");
      expect(calls).toBe(1);
    });

    it("writeBackboneEnvelope propagates an external abort promptly with no leaked timer", async () => {
      const controller = new AbortController();
      globalThis.fetch = vi.fn((_url: string, init?: RequestInit) => {
        return new Promise((_resolve, reject) => {
          init?.signal?.addEventListener("abort", () => reject(init.signal!.reason ?? new Error("aborted")));
        });
      }) as unknown as typeof fetch;
      const promise = writeBackboneEnvelope("folder:///doc", new Uint8Array([1]), controller.signal);
      controller.abort(new Error("caller cancelled"));
      await expect(promise).rejects.toThrow("caller cancelled");
    });
  });
}
//#endregion 🌐️BackboneEnvelopeIo

/** @deprecated Use {@link decodeDocumentPackSnapshot}. */
export function documentFromEnvelopeJson(_envelopeJson: string): unknown {
  throw new Error("documentFromEnvelopeJson removed — use decodeDocumentPackSnapshot on binary bundle bytes");
}

/** @deprecated Use {@link encodeDocumentPackBundle}. */
export function wrapArtifactEnvelope(_document: unknown, _documentId: string, _uri: string): string {
  throw new Error("wrapArtifactEnvelope removed — use encodeDocumentPackBundle");
}

//#region 🔀️ApplyBackboneMessage
export type BinaryBackboneMessage =
  | { readonly kind: "snapshot"; readonly pack: Uint8Array; readonly spr: Uint8Array }
  | { readonly kind: "mutations"; readonly envelopes: readonly WireMutationEnvelope[] }
  | { readonly kind: "ack"; readonly opIds: readonly string[] };

/** @emoji 🎯️ TS twin of `store::encode_backbone_message`. */
export function encodeBackboneMessage(message: BinaryBackboneMessage): Uint8Array {
  const out: number[] = [];
  if (message.kind === "snapshot") {
    out.push(0);
    writeBytes(out, Array.from(message.pack));
    writeBytes(out, Array.from(message.spr));
  } else if (message.kind === "mutations") {
    out.push(1);
    writeVecEnvelope(out, message.envelopes);
  } else {
    out.push(2);
    writeVecStr(out, message.opIds);
  }
  return new Uint8Array(out);
}

/** @emoji 🎯️ Inverse of {@link encodeBackboneMessage}. */
export function decodeBackboneMessage(bytes: Uint8Array): BinaryBackboneMessage {
  if (bytes.length === 0) throw new Error("backbone message: empty");
  const tag = bytes[0]!;
  const pos: [number] = [1];
  if (tag === 0) {
    const pack = new Uint8Array(readBytes(bytes, pos));
    const spr = new Uint8Array(readBytes(bytes, pos));
    return { kind: "snapshot", pack, spr };
  }
  if (tag === 1) {
    return { kind: "mutations", envelopes: readVecEnvelope(bytes, pos) };
  }
  if (tag === 2) {
    return { kind: "ack", opIds: readVecStr(bytes, pos) };
  }
  throw new Error(`backbone message: unknown tag ${tag}`);
}

/**
 * 🔀️ Applies an incoming {@link encodeBackboneMessage} payload onto a stored document bundle.
 * Snapshot overwrites; operations require the native store (not implemented in this TS twin).
 */
export function applyBackboneMessage(storedBundle: Uint8Array | null, messageBytes: Uint8Array): Uint8Array {
  const message = decodeBackboneMessage(messageBytes);
  if (message.kind === "snapshot") return encodeDocumentPackBytes(message.pack, message.spr);
  if (message.kind === "mutations") {
    if (storedBundle == null) throw new Error("cannot append operations before a snapshot exists");
    throw new Error("backbone operations apply requires native store — ingest envelopes through the sync actor");
  }
  throw new Error(`unsupported backbone message kind: ${(message as { kind: string }).kind}`);
}
//#endregion 🔀️ApplyBackboneMessage

/** 🍃️ Sync-controller-scoped toggle leaf — narrows the canonical {@link UtilityLeaf} `"toggle"` variant instead of duplicating its fields. */
export type FrameworkSyncUtilityLeaf = Extract<UtilityLeaf, { readonly kind: "toggle" }> & {
  readonly category: "sync";
  readonly controllerId: typeof FRAMEWORK_SYNC_CONTROLLER_ID;
  readonly action: string;
};

export function buildFrameworkSyncUtilities(activeUri: string | null): readonly FrameworkSyncUtilityLeaf[] {
  const activeKind = activeUri ? backboneKindFromUri(activeUri) : null;
  const pressed = (kind: BackboneKind) => activeKind === kind;
  return [
    { id: "framework.sync.file", kind: "toggle", iconId: "file-json", label: "File", category: "sync", pressed: pressed("file"), order: 0, controllerId: FRAMEWORK_SYNC_CONTROLLER_ID, action: "selectFile" },
    { id: "framework.sync.folder", kind: "toggle", iconId: "folder", label: "Folder", category: "sync", pressed: pressed("folder"), order: 1, controllerId: FRAMEWORK_SYNC_CONTROLLER_ID, action: "selectFolder" },
    { id: "framework.sync.remote", kind: "toggle", iconId: "cloud", label: "Remote", category: "sync", pressed: pressed("remote"), order: 2, controllerId: FRAMEWORK_SYNC_CONTROLLER_ID, action: "selectRemote" },
  ];
}
//#endregion 🔖️Backbone

//#region 🔖️DesktopWindowChrome
/** 🪟️ IPC channel names for the desktop window chrome controls (minimize/maximize/close) — shared literal between a host's `ipcMain.handle` registration and the renderer's `invoke` bridge. */
export const DESKTOP_WINDOW_CONTROL_CHANNELS = {
  minimize: "framework.window.minimize",
  maximize: "framework.window.maximize",
  close: "framework.window.close",
} as const;

/** 🎛️ Renderer-facing surface for the three desktop window chrome controls. */
export type DesktopWindowControls = { minimize(): Promise<unknown>; maximize(): Promise<unknown>; close(): Promise<unknown> };

/**
 * 🔌️ Registers host-side handlers for {@link DESKTOP_WINDOW_CONTROL_CHANNELS} against a structural
 * `ipc.handle`-shaped port — no `electron` types leak into this signature; a real Electron app wires
 * its `ipcMain`/`BrowserWindow` in at the call site. `maximize` toggles based on `isMaximized()`;
 * a null `focusedWindow()` is a no-operation.
 */
export function registerDesktopWindowControlHandlers(
  ipc: { handle(channel: string, fn: () => void): void },
  focusedWindow: () => { minimize(): void; isMaximized(): boolean; maximize(): void; unmaximize(): void; close(): void } | null,
): void {
  ipc.handle(DESKTOP_WINDOW_CONTROL_CHANNELS.minimize, () => {
    focusedWindow()?.minimize();
  });
  ipc.handle(DESKTOP_WINDOW_CONTROL_CHANNELS.maximize, () => {
    const window = focusedWindow();
    if (!window) return;
    if (window.isMaximized()) window.unmaximize();
    else window.maximize();
  });
  ipc.handle(DESKTOP_WINDOW_CONTROL_CHANNELS.close, () => {
    focusedWindow()?.close();
  });
}

/** 🌉️ Renderer-side {@link DesktopWindowControls} backed by a structural `invoke`-shaped port (e.g. `electron`'s `ipcRenderer.invoke`). */
export function desktopWindowControlsBridge(invoke: (channel: string) => Promise<unknown>): DesktopWindowControls {
  return {
    minimize: () => invoke(DESKTOP_WINDOW_CONTROL_CHANNELS.minimize),
    maximize: () => invoke(DESKTOP_WINDOW_CONTROL_CHANNELS.maximize),
    close: () => invoke(DESKTOP_WINDOW_CONTROL_CHANNELS.close),
  };
}
//#endregion 🔖️DesktopWindowChrome

//#region 🔖️Blob
/** 📦️ Dev-server-proxied content-addressed blob endpoint: `PUT ${BLOB_ENDPOINT_PATH}?mediaType=` (raw
 * bytes body, returns `{"hash":"..."}`) and `GET ${BLOB_ENDPOINT_PATH}/:hash` (raw bytes response).
 * Shared with the dev host shim (`framework/os/dev/script.ts`'s `hostShimSource`) and the
 * browser blob cache (`🟦️backbone-worker.ts`) so all three stay in sync on the same literal. Backed by
 * `vcs::BlobStore`'s native counterpart; a hub-backed route is a later ticket. */
export const BLOB_ENDPOINT_PATH = "/semio-blob";
//#endregion 🔖️Blob

//#region 🔖️BackboneWorkerProtocol
export type PersistenceBinding = { readonly kind: "folder"; readonly path: string } | { readonly kind: "hub"; readonly baseUrl: string; readonly spaceId: string; readonly token?: string; readonly surface?: string };

/** 🧾️ Everything the worker needs to open one artifact's actor — mirrors `ArtifactActorConfig`. */
export type ArtifactActorConfig = {
  readonly documentId: string;
  readonly schema: string;
  readonly bindings: readonly PersistenceBinding[];
  readonly watchExternal?: boolean;
  readonly actor: string;
  /** 🧬️ W5.7: this document kind's `store::DocumentCodec.pack_schema_hash`, for hub schema-hash
   * validation (`ClientFrame::Hello.pack_schema_hash`) — the shell fills this from the wasm
   * renderer's `document_pack_schema_hash(schema)` export before calling `openArtifact`. Omitted
   * (or all-zero) means "schema-agnostic client", which the hub never validates. */
  readonly packSchemaHash?: readonly number[];
};

/** 📨️ Caller→actor control messages — mirrors Rust `ArtifactActorMsg`. */
export type ArtifactActorMsg =
  | { readonly kind: "localMutations"; readonly envelopes: readonly MutationEnvelope[] }
  | { readonly kind: "localSnapshot"; readonly pack: readonly number[]; readonly spr: readonly number[] }
  | { readonly kind: "presenceHeartbeat"; readonly peer: ArtifactPresencePeer }
  | { readonly kind: "publishPreview"; readonly key: string; readonly seq: number; readonly payload: readonly number[] }
  | { readonly kind: "externalChanged" }
  | { readonly kind: "detach" };

/** 📶️ Connection state of a document's remote (hub) transport — mirrors Rust `RemoteState`. */
export type RemoteState = { readonly kind: "detached" } | { readonly kind: "connecting" } | { readonly kind: "live"; readonly peerCount: number } | { readonly kind: "backoff"; readonly retryInMs: number };

/** 🚦️ Sync health snapshot for status badges — mirrors Rust `ArtifactSyncStatus`. */
export type ArtifactSyncStatus = {
  readonly persisted: boolean;
  readonly pendingMutations: number;
  readonly remote: RemoteState;
};

/** ⚠️ A structural sync conflict — `ArtifactEvent::Conflict` wraps a `MutationMessage` (contract
 * freeze `26/08/16/MUTATION-OUTCOMES-MERGE-POLICIES-AND-FIRST-CLASS-CONFLICTS` §C10's frozen
 * diagnostic-bag vocabulary, replacing the deleted transport-level conflict bag it used to carry),
 * loosely typed here rather than importing the full shape — the shell only needs enough to render a
 * conflict card / offer "fork alternative" vs "take theirs", and `message` alone already covers that. */
export type SyncConflict = { readonly message?: string } & Record<string, unknown>;

/** 📮️ The client-side twin of `protocol_wire::ApplyOutcome`, minus the `Transformed` envelope
 * payload (already delivered separately as a `remoteMutations` event by the time this fires) —
 * mirrors Rust `CommandAckOutcome`. */
export type CommandAckOutcome = { readonly kind: "accepted" } | { readonly kind: "transformed" } | { readonly kind: "rejected"; readonly reason: string; readonly messages: readonly number[] };

/** 📬️ Actor→subscriber events — mirrors Rust `ArtifactEvent`. */
export type ArtifactEvent =
  | { readonly kind: "remoteMutations"; readonly envelopes: readonly MutationEnvelope[] }
  | { readonly kind: "snapshotReplaced"; readonly pack: readonly number[]; readonly spr: readonly number[] }
  | ({ readonly kind: "status" } & ArtifactSyncStatus)
  | { readonly kind: "presence"; readonly peers: readonly ArtifactPresencePeer[] }
  /** 🎨️ The hub's one-time session color assignment (`ServerFrame::Session`) — mirrors Rust
   * `ArtifactEvent::Session`. Flows through the SAME generic `{kind:"event",documentId,event}`
   * wrapping every other `ArtifactEvent` variant gets (both the real wasm `👷️worker/🦀️component.rs`
   * host, which wraps every `ArtifactEvent` uniformly with zero per-variant special-casing, and this
   * file's TS fallback via `emitEvent`), never a separate top-level `BackboneWorkerResponse` member. */
  | { readonly kind: "session"; readonly actor: string; readonly color: number }
  | { readonly kind: "preview"; readonly actor: string; readonly key: string; readonly seq: number; readonly payload: readonly number[] }
  | { readonly kind: "commandOutcome"; readonly batchId: number; readonly outcome: CommandAckOutcome }
  | ({ readonly kind: "conflict" } & SyncConflict);

/** 📤️ Main thread → `🟦️backbone-worker.ts` — `bytes` is a UTF-8 worker wire payload (see {@link encodeBackboneWorkerRequest}). */
export type BackboneWorkerWireMessage = { readonly wire: Uint8Array };

/** @emoji 🧵️ Worker wire magic — must match `store_sync::backbone_worker_wire::MAGIC`. */
const BACKBONE_WORKER_WIRE_MAGIC = 0x01;

function parseBackboneWorkerWire<T>(wire: Uint8Array, decode: (value: unknown) => T): T {
  if (wire.length === 0 || wire[0] !== BACKBONE_WORKER_WIRE_MAGIC) {
    throw new Error("backbone worker wire: unknown or empty payload");
  }
  return decode(decodePackValue(wire.subarray(1)));
}

/** @emoji 🧵️ Encodes a {@link BackboneWorkerRequest} for the wasm `store_worker` (`handleRequestBytes`). */
export function encodeBackboneWorkerRequest(request: BackboneWorkerRequest): Uint8Array {
  const wire =
    request.kind === "send"
      ? { ...request, message: wireArtifactActorMsg(request.message) }
      : request;
  const packed = encodePackValue(wire);
  return new Uint8Array([BACKBONE_WORKER_WIRE_MAGIC, ...packed]);
}

/** @emoji 🧵️ Decodes a {@link BackboneWorkerRequest} from the wasm actor or structured-clone twin. */
export function decodeBackboneWorkerRequest(wire: Uint8Array): BackboneWorkerRequest {
  const parsed = parseBackboneWorkerWire(wire, (value) => value as Record<string, unknown>);
  if (parsed.kind === "send" && typeof parsed.message === "object" && parsed.message !== null) {
    return {
      kind: "send",
      documentId: String(parsed.documentId),
      message: parseArtifactActorMsg(parsed.message as Record<string, unknown>),
    };
  }
  return parsed as BackboneWorkerRequest;
}

/** @emoji 🧵️ Encodes a {@link BackboneWorkerResponse} from the wasm actor / TS fallback. */
export function encodeBackboneWorkerResponse(response: BackboneWorkerResponse): Uint8Array {
  const wire =
    response.kind === "event" ? { ...response, event: wireArtifactEvent(response.event) } : response;
  const packed = encodePackValue(wire);
  return new Uint8Array([BACKBONE_WORKER_WIRE_MAGIC, ...packed]);
}

/** @emoji 🧵️ Decodes a worker response/event wire payload from the wasm actor. */
export function decodeBackboneWorkerResponse(wire: Uint8Array): BackboneWorkerResponse {
  const parsed = parseBackboneWorkerWire(wire, (value) => value as Record<string, unknown>);
  if (parsed.kind === "event" && typeof parsed.event === "object" && parsed.event !== null) {
    return { kind: "event", documentId: String(parsed.documentId), event: parseArtifactEvent(parsed.event as Record<string, unknown>) };
  }
  return parsed as BackboneWorkerResponse;
}

/** 📤️ Main thread → `🟦️backbone-worker.ts` messages (structured clone or {@link BackboneWorkerWireMessage}).
 * The `directory-*` kinds (contract-freeze §C6) are the shell's ONLY way to reach the directory hub
 * — plugin surfaces never talk to the network, and the shell never opens a directory socket on the
 * UI thread; see `🟦️backbone-worker.ts`'s `🔖️Directory` region. */
export type BackboneWorkerRequest =
  | ({ readonly kind: "open" } & ArtifactActorConfig)
  | { readonly kind: "close"; readonly documentId: string }
  | { readonly kind: "send"; readonly documentId: string; readonly message: ArtifactActorMsg }
  | { readonly kind: "directory-open"; readonly baseUrl: string; readonly token?: string; readonly since: number }
  | { readonly kind: "directory-command"; readonly requestId: string; readonly command: DirectoryCommand }
  | { readonly kind: "directory-close" };

/** 📥️ `🟦️backbone-worker.ts` → main thread messages. `directory-status.pendingCommands` is the
 * bounded, in-memory offline queue's length (contract-freeze §C6 "commands queue... and flush on
 * reconnect"). */
export type BackboneWorkerResponse =
  | { readonly kind: "event"; readonly documentId: string; readonly event: ArtifactEvent }
  | { readonly kind: "ready" }
  | { readonly kind: "directory-message"; readonly message: DirectoryStreamMessage }
  | { readonly kind: "directory-command-result"; readonly requestId: string; readonly ok: boolean; readonly events?: readonly DirectoryEvent[]; readonly error?: string }
  | { readonly kind: "directory-status"; readonly pendingCommands: number };

function wireArtifactActorMsg(message: ArtifactActorMsg): unknown {
  if (message.kind === "localMutations") {
    return { kind: "localMutations", envelopes: encodeCausalEnvelopeBatch(message.envelopes, replicationPackCodec) };
  }
  return message;
}

function parseArtifactActorMsg(message: Record<string, unknown>): ArtifactActorMsg {
  if (message.kind === "localMutations" && Array.isArray(message.envelopes) && message.envelopes.every((entry) => typeof entry === "number")) {
    return { kind: "localMutations", envelopes: decodeCausalEnvelopeBatch(message.envelopes as readonly number[], replicationPackCodec) };
  }
  return message as ArtifactActorMsg;
}

function wireArtifactEvent(event: ArtifactEvent): unknown {
  if (event.kind === "remoteMutations") {
    return { kind: "remoteMutations", envelopes: encodeCausalEnvelopeBatch(event.envelopes, replicationPackCodec) };
  }
  return event;
}

function parseArtifactEvent(event: Record<string, unknown>): ArtifactEvent {
  if (event.kind === "remoteMutations" && Array.isArray(event.envelopes) && event.envelopes.every((entry) => typeof entry === "number")) {
    return { kind: "remoteMutations", envelopes: decodeCausalEnvelopeBatch(event.envelopes as readonly number[], replicationPackCodec) };
  }
  return event as ArtifactEvent;
}
//#endregion 🔖️EnvelopeCodec

//#endregion 🔖️BackboneWorkerProtocol


//#region 🔖️WorkflowPlanner
/**
 * 🎬️ TS mirror of `workflow::{OsMediaPort,OsWorkflowNode,OsWorkflowEdge,OsWorkflow}` (Rust,
 * `framework/os/core/rs/lib.rs`) — camelCase-field-identical (Rust: `#[serde(rename_all =
 * "camelCase")]`). See this file's header for why only this pure-planner slice is hand-mirrored.
 */
export type OsMediaPort = {
  readonly id: string;
  readonly artifactKind: string;
  readonly direction: string;
};

export type OsWorkflowNode = {
  readonly id: string;
  readonly instanceId: string;
  readonly x: number;
  readonly y: number;
  readonly width: number;
  readonly height: number;
  readonly inputs: readonly OsMediaPort[];
  readonly outputs: readonly OsMediaPort[];
};

/** 🤝️ TS twin of Rust `MediaContract` (`workflow::MediaContract`, hand-written `dsl::DslField`). */
export type MediaContract = {
  readonly kindId: string;
  readonly mediaType: { readonly class: string; readonly form: string };
  readonly wire: { readonly kind: "binary"; readonly format: string } | { readonly kind: "document"; readonly schema: string };
  readonly conversion?: readonly [string, string] | null;
};

export type OsWorkflowEdge = {
  readonly id: string;
  readonly sourceNodeId: string;
  readonly sourcePortId: string;
  readonly targetNodeId: string;
  readonly targetPortId: string;
  readonly contract: MediaContract;
};

export type OsWorkflow = {
  readonly schema: string;
  readonly nodes: readonly OsWorkflowNode[];
  readonly edges: readonly OsWorkflowEdge[];
};

/** 🚚️ TS twin of Rust `WorkflowDelivery`. */
export type WorkflowDelivery = {
  readonly edgeId: string;
  readonly producerInstanceId: string;
  readonly producerPortId: string;
  readonly consumerInstanceId: string;
  readonly consumerPortId: string;
};

/** 🔬️ TS twin of Rust `WorkflowFixture` — decoded from the shared `.dsl`/`.spk` fixture pairs via wasm, never JSON. */
export type WorkflowFixture = {
  readonly name: string;
  readonly graph: OsWorkflow;
  readonly dirtyInstanceIds: readonly string[];
  readonly expectedDeliveries: readonly WorkflowDelivery[];
};

/**
 * 🧭️ TS twin of Rust `workflow_topological_node_order` — DFS post-order reversed into a
 * topological node order (source before target); deterministic purely from `graph.nodes`/
 * `graph.edges` insertion order, so it matches the Rust side edge-for-edge.
 */
function mediaFlowTopologicalNodeOrder(graph: OsWorkflow): readonly string[] {
  const adjacency = new Map<string, string[]>();
  for (const edge of graph.edges) {
    const targets = adjacency.get(edge.sourceNodeId) ?? [];
    targets.push(edge.targetNodeId);
    adjacency.set(edge.sourceNodeId, targets);
  }
  const visited = new Set<string>();
  const order: string[] = [];
  const dfs = (nodeId: string): void => {
    if (visited.has(nodeId)) return;
    visited.add(nodeId);
    for (const next of adjacency.get(nodeId) ?? []) dfs(next);
    order.push(nodeId);
  };
  for (const node of graph.nodes) dfs(node.id);
  order.reverse();
  return order;
}

/**
 * 🚚️ TS twin of Rust `plan_workflow` — plans one {@link WorkflowDelivery} per edge in the
 * downstream closure of `dirtyInstanceIds`, propagating dirtiness onto each edge's consumer instance
 * so multi-hop chains (A→B→C) resolve in a single topological pass. Pure/side-effect-free.
 */
export function planWorkflow(graph: OsWorkflow, dirtyInstanceIds: ReadonlySet<string>): readonly WorkflowDelivery[] {
  const nodeById = new Map<string, OsWorkflowNode>(graph.nodes.map((node) => [node.id, node]));
  const edgesBySource = new Map<string, OsWorkflowEdge[]>();
  for (const edge of graph.edges) {
    const edges = edgesBySource.get(edge.sourceNodeId) ?? [];
    edges.push(edge);
    edgesBySource.set(edge.sourceNodeId, edges);
  }
  const order = mediaFlowTopologicalNodeOrder(graph);
  const dirty = new Set(dirtyInstanceIds);
  const deliveries: WorkflowDelivery[] = [];
  for (const nodeId of order) {
    const node = nodeById.get(nodeId);
    if (!node || !dirty.has(node.instanceId)) continue;
    for (const edge of edgesBySource.get(nodeId) ?? []) {
      const targetNode = nodeById.get(edge.targetNodeId);
      if (!targetNode) continue;
      deliveries.push({
        edgeId: edge.id,
        producerInstanceId: node.instanceId,
        producerPortId: edge.sourcePortId,
        consumerInstanceId: targetNode.instanceId,
        consumerPortId: edge.targetPortId,
      });
      dirty.add(targetNode.instanceId);
    }
  }
  return deliveries;
}
//#endregion 🔖️WorkflowPlanner

//#region 🔖️PackValueCodec
/**
 * 📦️ TS mirror of `store::pack_rt::encode_wire_value`/`decode_wire_value`
 * (`framework/product/os/module/store/rs/lib.rs`) — the schema-less `serde_json::Value` bridge
 * for per-message wire payloads (UI tree diffs, host effects, events, manifests), NOT whole
 * documents (that's `encode_json_value`/`decode_json_value`'s job, backed by
 * `pack::encode_document`'s full `.spk` container — 32-byte header, deflate-compressed segments,
 * an 84-byte footer with a BLAKE3 content hash, 200+ bytes of overhead per value, and
 * deflate-compressed bytes that are NOT portable byte-for-byte across a spec-compliant TS
 * deflate implementation). `encode_wire_value` instead calls `pack::encode_record_body` — the
 * container-less twin used by `dsl::op_rt::encode_op` — for a `symbol_count varint, (len varint,
 * utf8)*, record fields` grammar with no header, segments, manifest, or footer. Every JSON value
 * is still wrapped as a single `Shape::Value` field (id 1) of the same synthetic one-field
 * `json_bridge_spec()` record; only the outer framing changed. Fully deterministic and
 * byte-exact against real Rust output in both directions (no compression involved, unlike the
 * old container-backed encoding this replaces).
 */

//#region 🔖️PackContainerPrimitives
/** 🌱️ `store::pack_rt`'s synthetic single-field record spec (`{ id: 1, key: "value", shape:
 * Shape::Value }`) every JSON value is wrapped in before hitting `encode_record_body`. */
const JSON_BRIDGE_FIELD_ID = 1;

/** 🌱️ `pack_value`'s wire tags actually reachable from a `DslValue` (`encode_dsl_value`/
 * `decode_dsl_value`, `pack/value/rs/lib.rs`'s `🔖️Tags` region) — the subset `PackValueCodec`
 * needs (no `Int`/`UInt`/`Bytes64`/`Enum`/... — a JSON value never produces those). */
const PACK_TAG_FALSE = 0x01;
const PACK_TAG_TRUE = 0x02;
const PACK_TAG_F64 = 0x05;
const PACK_TAG_STR = 0x06;
const PACK_TAG_STR_INLINE = 0x07;
const PACK_TAG_LIST = 0x0c;
const PACK_TAG_MAP = 0x10;
const PACK_TAG_VALUE = 0x11;
const PACK_TAG_NULL = 0x12;

function packPushBytes(out: number[], bytes: Uint8Array): void {
  for (let index = 0; index < bytes.length; index++) out.push(bytes[index]!);
}
/** 🔤️ Byte-lexicographic string comparison — the TS twin of Rust `str`'s `Ord` (which compares
 * the UTF-8 byte sequence), used everywhere `pack_value` sorts by `.as_bytes()` (symbol table,
 * `DslValue::Object` keys). Differs from JS's default UTF-16-code-unit `<`/`.sort()` only outside
 * the BMP, but is implemented properly rather than assumed equivalent. */
function packByteCompare(a: string, b: string): number {
  const encoder = new TextEncoder();
  const ab = encoder.encode(a);
  const bb = encoder.encode(b);
  const len = Math.min(ab.length, bb.length);
  for (let index = 0; index < len; index++) {
    const diff = ab[index]! - bb[index]!;
    if (diff !== 0) return diff;
  }
  return ab.length - bb.length;
}
//#endregion 🔖️PackContainerPrimitives

//#region 🔖️JsonValueTags
/** 🔎️ `pack_value::build_symbols`, specialized to a JSON-bridge document (one `Shape::Value`
 * field — no `TableSoA`/`Statements` forced-symbol cases apply). Walks `value` counting only
 * STRING LEAVES (object/array keys are never counted — `pack_value::walk_dsl_value_for_symbols`'s
 * `DslValue::Object` case only walks entry VALUES); a string is interned (added to the symbol
 * table) iff its UTF-8 byte length is `<= 128` or it occurs `>= 2` times, matching `pack_value`'s
 * rule exactly (note: `.len()` on the Rust side is UTF-8 BYTE length, not char count). */
function packCollectStrings(value: unknown, counts: Map<string, number>): void {
  if (typeof value === "string") {
    counts.set(value, (counts.get(value) ?? 0) + 1);
    return;
  }
  if (Array.isArray(value)) {
    for (const item of value) packCollectStrings(item, counts);
    return;
  }
  if (value !== null && typeof value === "object") {
    for (const item of Object.values(value as Record<string, unknown>)) packCollectStrings(item, counts);
  }
}
function packBuildSymbols(value: unknown): string[] {
  const counts = new Map<string, number>();
  packCollectStrings(value, counts);
  const encoder = new TextEncoder();
  const symbols: string[] = [];
  for (const [text, count] of counts) if (encoder.encode(text).length <= 128 || count >= 2) symbols.push(text);
  symbols.sort(packByteCompare);
  return symbols;
}

/** ✍️ `pack_value::encode_string`: `TAG_STR + symref varint` if interned, else
 * `TAG_STR_INLINE + len varint + utf8 bytes`. */
function packEncodeString(text: string, symbolIndex: ReadonlyMap<string, number>, out: number[]): void {
  const index = symbolIndex.get(text);
  if (index !== undefined) {
    out.push(PACK_TAG_STR);
    writeVarintU64(out, index);
    return;
  }
  packEncodeStringInline(text, out);
}
/** ✍️ `pack_value::encode_string_inline` — forced, e.g. every `DslValue::Object` key. */
function packEncodeStringInline(text: string, out: number[]): void {
  const bytes = new TextEncoder().encode(text);
  out.push(PACK_TAG_STR_INLINE);
  writeVarintU64(out, bytes.length);
  packPushBytes(out, bytes);
}
/** 📖️ `pack_value::decode_string` — reads its OWN leading tag (`TAG_STR`/`TAG_STR_INLINE`), used
 * both for `Map`/object keys and inside {@link packDecodeValue}'s `TAG_STR` case. */
function packDecodeString(bytes: Uint8Array, symbols: readonly string[], pos: [number]): string {
  const tag = bytes[pos[0]]!;
  pos[0] += 1;
  if (tag === PACK_TAG_STR) {
    const index = readVarintU64(bytes, pos);
    const symbol = symbols[index];
    if (symbol === undefined) throw new Error(`decodePackValue: symref ${index} out of range for table of ${symbols.length}`);
    return symbol;
  }
  if (tag === PACK_TAG_STR_INLINE) {
    const len = readVarintU64(bytes, pos);
    const text = new TextDecoder().decode(bytes.subarray(pos[0], pos[0] + len));
    pos[0] += len;
    return text;
  }
  throw new Error(`decodePackValue: expected a string tag, found 0x${tag.toString(16)}`);
}

/** ✍️ `pack_value::encode_dsl_value` — the tag-prefixed encoding one JSON value recurses through.
 * `Number` always writes `TAG_F64` (`DslValue::Number` is always `f64`; `pack_rt`'s
 * `renormalize_whole_number_floats` is a SEPARATE opt-in helper for typed-struct consumers, never
 * called by `encode_json_value`/`decode_json_value`/`encode_wire_value`/`decode_wire_value`
 * themselves — verified empirically against real fixture bytes, see this region's header doc).
 * `-0` normalizes to `0` (byte-level parity
 * with Rust's `normalize_f64`; unobservable via `===` in JS either way). Object entries sort by
 * key BYTES with keys always forced inline, never a symref. */
function packEncodeValue(value: unknown, symbolIndex: ReadonlyMap<string, number>, out: number[]): void {
  if (value === null || value === undefined) {
    out.push(PACK_TAG_NULL);
    return;
  }
  if (typeof value === "boolean") {
    out.push(value ? PACK_TAG_TRUE : PACK_TAG_FALSE);
    return;
  }
  if (typeof value === "number") {
    out.push(PACK_TAG_F64);
    writeF64(out, value === 0 ? 0 : value);
    return;
  }
  if (typeof value === "string") {
    packEncodeString(value, symbolIndex, out);
    return;
  }
  if (Array.isArray(value)) {
    out.push(PACK_TAG_LIST);
    writeVarintU64(out, value.length);
    for (const item of value) packEncodeValue(item, symbolIndex, out);
    return;
  }
  if (typeof value === "object") {
    out.push(PACK_TAG_MAP);
    const entries = Object.entries(value as Record<string, unknown>).sort((a, b) => packByteCompare(a[0], b[0]));
    writeVarintU64(out, entries.length);
    for (const [key, entryValue] of entries) {
      packEncodeStringInline(key, out);
      packEncodeValue(entryValue, symbolIndex, out);
    }
    return;
  }
  throw new Error(`encodePackValue: unsupported JSON value of type ${typeof value}`);
}
/** 📖️ Inverse of {@link packEncodeValue} — the TS twin of `pack_value::decode_dsl_value`. */
function packDecodeValue(bytes: Uint8Array, symbols: readonly string[], pos: [number]): unknown {
  const tag = bytes[pos[0]]!;
  pos[0] += 1;
  switch (tag) {
    case PACK_TAG_NULL:
      return null;
    case PACK_TAG_FALSE:
      return false;
    case PACK_TAG_TRUE:
      return true;
    case PACK_TAG_F64:
      return readF64(bytes, pos);
    case PACK_TAG_STR: {
      const index = readVarintU64(bytes, pos);
      const symbol = symbols[index];
      if (symbol === undefined) throw new Error(`decodePackValue: symref ${index} out of range for table of ${symbols.length}`);
      return symbol;
    }
    case PACK_TAG_STR_INLINE: {
      const len = readVarintU64(bytes, pos);
      const text = new TextDecoder().decode(bytes.subarray(pos[0], pos[0] + len));
      pos[0] += len;
      return text;
    }
    case PACK_TAG_LIST: {
      const count = readVarintU64(bytes, pos);
      const items: unknown[] = [];
      for (let i = 0; i < count; i++) items.push(packDecodeValue(bytes, symbols, pos));
      return items;
    }
    case PACK_TAG_MAP: {
      const count = readVarintU64(bytes, pos);
      const entries: Record<string, unknown> = {};
      for (let i = 0; i < count; i++) {
        const key = packDecodeString(bytes, symbols, pos);
        entries[key] = packDecodeValue(bytes, symbols, pos);
      }
      return entries;
    }
    default:
      throw new Error(`decodePackValue: unrecognized dsl value tag 0x${tag.toString(16)}`);
  }
}
//#endregion 🔖️JsonValueTags

//#region 🔖️PublicApi
/** 📤️ TS twin of `store::pack_rt::encode_wire_value` — encodes any JSON-shaped `value` (null,
 * bool, number, string, array, nested object) as an `encode_record_body` payload: `symbol_count
 * varint, (len varint, utf8 bytes)*` (the symbol table, written inline — no `Symbols` segment)
 * followed directly by the synthetic one-field record's fields (`field_count=1, field_id=1,
 * TAG_VALUE, <value>`, matching `pack_value::encode_record_fields`'s grammar exactly). No header,
 * segments, manifest, or footer — byte-exact against real Rust output (verified against the
 * `pack_wire_value_fixture_corpus_hex_dump` fixture corpus, `store/rs/lib.rs`'s
 * `🔖️PackValueFixtures` region). */
export function encodePackValue(value: unknown): Uint8Array {
  const symbols = packBuildSymbols(value);
  const symbolIndex = new Map(symbols.map((symbol, index) => [symbol, index] as const));
  const encoder = new TextEncoder();

  const out: number[] = [];
  writeVarintU64(out, symbols.length);
  for (const symbol of symbols) {
    const bytes = encoder.encode(symbol);
    writeVarintU64(out, bytes.length);
    packPushBytes(out, bytes);
  }
  writeVarintU64(out, 1); // field_count
  writeVarintU64(out, JSON_BRIDGE_FIELD_ID);
  out.push(PACK_TAG_VALUE);
  packEncodeValue(value, symbolIndex, out);
  return new Uint8Array(out);
}

/** 📥️ TS twin of `store::pack_rt::decode_wire_value` — the inverse of {@link encodePackValue}. */
export function decodePackValue(bytes: Uint8Array): unknown {
  const pos: [number] = [0];
  const decoder = new TextDecoder();
  const symbolCount = readVarintU64(bytes, pos);
  const symbols: string[] = [];
  for (let i = 0; i < symbolCount; i++) {
    const len = readVarintU64(bytes, pos);
    symbols.push(decoder.decode(bytes.subarray(pos[0], pos[0] + len)));
    pos[0] += len;
  }

  const fieldCount = readVarintU64(bytes, pos);
  let result: unknown = null;
  for (let i = 0; i < fieldCount; i++) {
    const fieldId = readVarintU64(bytes, pos);
    const outerTag = bytes[pos[0]]!;
    pos[0] += 1;
    if (outerTag !== PACK_TAG_VALUE) throw new Error(`decodePackValue: unexpected field tag 0x${outerTag.toString(16)} for field ${fieldId}`);
    const value = packDecodeValue(bytes, symbols, pos);
    if (fieldId === JSON_BRIDGE_FIELD_ID) result = value;
  }
  return result;
}

const PACK_B64_PREFIX = "pk:";

/** @emoji 📦️ Lossless pack snapshot as a `pk:`-prefixed base64 string for `sessionStorage`/`ViewModel` string slots. */
export function packValueToBase64(value: unknown): string {
  const bytes = encodePackValue(value);
  let binary = "";
  for (let index = 0; index < bytes.length; index += 1) binary += String.fromCharCode(bytes[index]!);
  return `${PACK_B64_PREFIX}${btoa(binary)}`;
}

/** @emoji 📥️ Inverse of {@link packValueToBase64}. */
export function packValueFromBase64(encoded: string): unknown {
  if (!encoded.startsWith(PACK_B64_PREFIX)) throw new Error("packValueFromBase64: expected pk: prefix");
  const binary = atob(encoded.slice(PACK_B64_PREFIX.length));
  const bytes = new Uint8Array(binary.length);
  for (let index = 0; index < binary.length; index += 1) bytes[index] = binary.charCodeAt(index);
  return decodePackValue(bytes);
}

/** @emoji 🎯️ Plugin `handleAction` wire: pack-base64 `{ controllerId, action, args? }`. */
export type ActionWire = { readonly controllerId: string; readonly action: string; readonly args?: unknown };

export function encodeActionWire(descriptor: ActionWire): string {
  return packValueToBase64(descriptor);
}

/** @emoji 📥️ Inverse of {@link encodeActionWire}. */
export function decodeActionWire(wire: string): ActionWire {
  return packValueFromBase64(wire) as ActionWire;
}

/** @emoji 🎬️ Decodes a component-scene `*Json` field when it carries {@link packValueToBase64} bytes. */
export function decodeScenePackField(encoded: string): unknown {
  return packValueFromBase64(encoded);
}

/** @emoji 📤️ `protocol::encode_envelopes` batch as a {@link packValueToBase64} string for `applyMutations`. */
export function encodeMutationEnvelopesPack(envelopes: readonly MutationEnvelope[]): string {
  return packValueToBase64(Array.from(encodeCausalEnvelopeBatch(envelopes, replicationPackCodec)));
}

/** @emoji 📥️ Inverse of {@link encodeMutationEnvelopesPack}. */
export function decodeMutationEnvelopesPack(pack: string): MutationEnvelope[] {
  const wire = packValueFromBase64(pack);
  if (!Array.isArray(wire) || !wire.every((entry) => typeof entry === "number")) {
    throw new Error("decodeMutationEnvelopesPack: expected pack byte array");
  }
  return decodeCausalEnvelopeBatch(wire as readonly number[], replicationPackCodec);
}
//#endregion 🔖️PublicApi
//#endregion 🔖️PackValueCodec

//#region 🔖️ScenePackCodec
const SCENE_PACK_TAG_UNIT = 0;
const SCENE_PACK_TAG_FALSE = 1;
const SCENE_PACK_TAG_TRUE = 2;
const SCENE_PACK_TAG_U64 = 3;
const SCENE_PACK_TAG_I64 = 4;
const SCENE_PACK_TAG_F64 = 5;
const SCENE_PACK_TAG_STR = 6;
const SCENE_PACK_TAG_BYTES = 7;
const SCENE_PACK_TAG_NONE = 8;
const SCENE_PACK_TAG_SOME = 9;
const SCENE_PACK_TAG_SEQ = 10;
const SCENE_PACK_TAG_CHAR = 11;
const SCENE_PACK_TAG_VARIANT = 12;
const SCENE_PACK_TAG_MAP = 13;
const SCENE_PACK_UNIT = Symbol("scene-pack-unit");

function readScenePackVarint(bytes: Uint8Array, position: { value: number }): bigint {
  let value = 0n;
  for (let shift = 0n; shift < 70n; shift += 7n) {
    const byte = bytes[position.value];
    if (byte === undefined) throw new Error("decodeScenePackValue: truncated varint");
    position.value += 1;
    value |= BigInt(byte & 0x7f) << shift;
    if ((byte & 0x80) === 0) return value;
  }
  throw new Error("decodeScenePackValue: varint exceeds u64");
}

function scenePackNumber(value: bigint): number {
  const number = Number(value);
  if (!Number.isSafeInteger(number)) throw new Error("decodeScenePackValue: integer exceeds JavaScript's safe range");
  return number;
}

function readScenePackLength(bytes: Uint8Array, position: { value: number }): number {
  const length = scenePackNumber(readScenePackVarint(bytes, position));
  if (length > bytes.length - position.value) throw new Error("decodeScenePackValue: declared length exceeds remaining bytes");
  return length;
}

function decodeScenePackItem(bytes: Uint8Array, position: { value: number }): unknown | typeof SCENE_PACK_UNIT {
  const tag = bytes[position.value];
  if (tag === undefined) throw new Error("decodeScenePackValue: truncated value");
  position.value += 1;
  if (tag === SCENE_PACK_TAG_UNIT) return SCENE_PACK_UNIT;
  if (tag === SCENE_PACK_TAG_FALSE) return false;
  if (tag === SCENE_PACK_TAG_TRUE) return true;
  if (tag === SCENE_PACK_TAG_U64) return scenePackNumber(readScenePackVarint(bytes, position));
  if (tag === SCENE_PACK_TAG_I64) {
    const raw = readScenePackVarint(bytes, position);
    return scenePackNumber((raw >> 1n) ^ -(raw & 1n));
  }
  if (tag === SCENE_PACK_TAG_F64) {
    if (bytes.length - position.value < 8) throw new Error("decodeScenePackValue: truncated f64");
    const value = new DataView(bytes.buffer, bytes.byteOffset + position.value, 8).getFloat64(0, true);
    position.value += 8;
    return value;
  }
  if (tag === SCENE_PACK_TAG_STR) {
    const length = readScenePackLength(bytes, position);
    const value = new TextDecoder("utf-8", { fatal: true }).decode(bytes.subarray(position.value, position.value + length));
    position.value += length;
    return value;
  }
  if (tag === SCENE_PACK_TAG_BYTES) {
    const length = readScenePackLength(bytes, position);
    const value = Array.from(bytes.subarray(position.value, position.value + length));
    position.value += length;
    return value;
  }
  if (tag === SCENE_PACK_TAG_NONE) return null;
  if (tag === SCENE_PACK_TAG_SOME) return decodeScenePackItem(bytes, position);
  if (tag === SCENE_PACK_TAG_SEQ) {
    const count = readScenePackLength(bytes, position);
    const value: unknown[] = [];
    for (let index = 0; index < count; index += 1) {
      const item = decodeScenePackItem(bytes, position);
      value.push(item === SCENE_PACK_UNIT ? null : item);
    }
    return value;
  }
  if (tag === SCENE_PACK_TAG_CHAR) {
    const codePoint = scenePackNumber(readScenePackVarint(bytes, position));
    if (codePoint > 0x10ffff || (codePoint >= 0xd800 && codePoint <= 0xdfff)) throw new Error("decodeScenePackValue: invalid Unicode scalar");
    return String.fromCodePoint(codePoint);
  }
  if (tag === SCENE_PACK_TAG_VARIANT) {
    const name = decodeScenePackItem(bytes, position);
    if (typeof name !== "string") throw new Error("decodeScenePackValue: variant name is not a string");
    const payload = decodeScenePackItem(bytes, position);
    return payload === SCENE_PACK_UNIT ? name : { [name]: payload };
  }
  if (tag === SCENE_PACK_TAG_MAP) {
    const count = readScenePackLength(bytes, position);
    const value: Record<string, unknown> = {};
    for (let index = 0; index < count; index += 1) {
      const key = decodeScenePackItem(bytes, position);
      if (typeof key !== "string") throw new Error("decodeScenePackValue: map key is not a string");
      if (Object.hasOwn(value, key)) throw new Error(`decodeScenePackValue: duplicate map key ${key}`);
      const item = decodeScenePackItem(bytes, position);
      value[key] = item === SCENE_PACK_UNIT ? null : item;
    }
    return value;
  }
  throw new Error(`decodeScenePackValue: invalid tag ${tag}`);
}

/** @emoji 🎬️ Decodes the self-describing serde packet emitted by `semio-framework-ui-scene`. */
export function decodeScenePackValue(bytes: Uint8Array): unknown {
  const position = { value: 0 };
  const value = decodeScenePackItem(bytes, position);
  if (position.value !== bytes.length) throw new Error(`decodeScenePackValue: ${bytes.length - position.value} trailing bytes`);
  return value === SCENE_PACK_UNIT ? null : value;
}
//#endregion 🔖️ScenePackCodec

//#region 🔖️AppChannelCodec
/**
 * 📡️ TS mirror of the `protocol_channel` crate's `AppCommand`/`AppFrame` binary frame protocol
 * (`tag u8 | fields`, built on `protocol_core`'s varint/string/bytes primitives — the same ones
 * {@link encodeClientFrame}/{@link decodeClientFrame} above use). Channel v12
 * (`📓️design-abi.md` §2's handshake-collapse section) retired the `Hello`/`Bye`/`AttachBackbone`/
 * `DetachBackbone`/`RefreshUi` commands and the `Welcome`/`UiSection`/`Effects`/`Events` frames —
 * the reactor ABI wakes guests on events/timers/`next-wake` and carries lifecycle through
 * `Event::InstanceOpen`/`InstanceClose` instead of a wire handshake, and UI updates are now a
 * revisioned `UiPatch` push rather than a cache-probed `RefreshUi`/`UiSection` round trip.
 * `envelopes`/`config`/`command`/`descriptor`/etc. all stay OPAQUE `bytes` here (never a decoded
 * `protocol_causal::MutationEnvelope` or app-specific payload shape), exactly like
 * {@link WireMutationEnvelope}'s `diff`/`inverse` payloads above. `Option<T>` fields use the same
 * `0x00`/`0x01` presence-byte convention as {@link writeOptStr}/{@link writeOptBytes} elsewhere in
 * this file.
 */

//#region 🔖️Types
export type ChildPackEntry = { readonly slot: string; readonly child_id: string; readonly dialect: string; readonly envelope_pack: readonly number[] };

export type AppCommandValue =
  | { readonly LocalInteractionQuery: { readonly seq: number; readonly command: LocalInteractionQueryCommand } }
  | { readonly ConfigCommand: { readonly seq: number; readonly command: readonly number[] } }
  | { readonly Command: { readonly seq: number; readonly command: readonly number[]; readonly view_state: readonly number[] } }
  | { readonly CommandText: { readonly seq: number; readonly line: string } }
  | { readonly ContextMenu: { readonly seq: number; readonly request: readonly number[] } }
  | { readonly ArtifactCommand: { readonly seq: number; readonly command: readonly number[] } }
  | { readonly ApplyEnvelopes: { readonly seq: number; readonly envelopes: readonly MutationEnvelope[] } }
  | { readonly LoadDocument: { readonly seq: number; readonly pack: readonly number[]; readonly spr: readonly number[] } }
  | { readonly ReadDocument: { readonly seq: number } }
  | { readonly LoadConfig: { readonly seq: number; readonly pack: readonly number[]; readonly spr: readonly number[] } }
  | { readonly ReadConfig: { readonly seq: number } }
  | { readonly MediaIn: { readonly seq: number; readonly port: string; readonly descriptor: readonly number[]; readonly data: readonly number[] } }
  | { readonly MediaOut: { readonly seq: number; readonly port: string; readonly request: readonly number[] } }
  | { readonly MediaFingerprint: { readonly seq: number; readonly port: string } }
  | { readonly PureCommand: { readonly seq: number; readonly command: readonly number[]; readonly document: readonly number[]; readonly document_spr: readonly number[]; readonly config: readonly number[]; readonly config_spr: readonly number[]; readonly draft: readonly number[]; readonly draft_spr: readonly number[] } }
  | { readonly LoadChildren: { readonly seq: number; readonly entries: readonly ChildPackEntry[] } }
  | { readonly ReadChildren: { readonly seq: number } }
  | { readonly ReadHistory: { readonly seq: number } }
  | {
      readonly transactionPrepare: {
        readonly seq: number;
        readonly txn_id: string;
        readonly mutation_id: string;
        readonly payload: readonly number[];
        readonly prepared_ops: readonly (readonly number[])[];
        readonly label: string;
        readonly origin: readonly number[];
      };
    }
  | { readonly transactionCommit: { readonly seq: number; readonly txn_id: string } }
  | { readonly transactionRollback: { readonly seq: number; readonly txn_id: string } }
  | { readonly transactionUndo: { readonly seq: number; readonly group_id: string } }
  | { readonly transactionRedo: { readonly seq: number; readonly group_id: string } }
  | { readonly openArtifact: { readonly seq: number; readonly artifact_ref: string; readonly role: number; readonly plugin_id: string; readonly app_id: string } }
  | {
      readonly setDefaultApp: {
        readonly seq: number;
        readonly artifact_kind: string;
        readonly standard: string;
        readonly subset: string;
        readonly role: number;
        readonly plugin_id: string;
        readonly app_id: string;
      };
    }
  | { readonly clearDefaultApp: { readonly seq: number; readonly artifact_kind: string; readonly standard: string; readonly subset: string; readonly role: number } }
  | { readonly setMergePolicy: { readonly seq: number; readonly policy: number } }
  | { readonly resolveConflict: { readonly seq: number; readonly conflict_id: string; readonly resolution: number } }
  | { readonly readConflicts: { readonly seq: number } }
  /** 👥️ Pushes the document-wide presence roster into this app instance — the ONLY plugin ingress
   * for peers (contract-freeze §C7.6). `own_color` is this actor's hub-assigned palette index
   * (`null` for a folder-only session with no hub); `peers` are `encodePresencePeer` blobs, the
   * whole roster with the wrapper's own actor already dropped. CHANNEL_VERSION 12 wire addition. */
  | { readonly presence: { readonly seq: number; readonly own_color: number | null; readonly peers: readonly (readonly number[])[] } };

export type AppFrameValue =
  | { readonly LocalInteractionQuery: { readonly reply: LocalInteractionQueryReply } }
  | { readonly Done: { readonly in_reply_to: number } }
  | {
      readonly Invocation: {
        readonly in_reply_to: number;
        readonly output: readonly number[];
        readonly diagnostics: readonly number[];
        readonly ui_scope: readonly number[];
        readonly history_patch: readonly number[];
        readonly messages: readonly number[];
      };
    }
  | { readonly DocumentChanged: { readonly envelopes: readonly (readonly number[])[]; readonly origin: string } }
  | { readonly Document: { readonly in_reply_to: number; readonly pack: readonly number[]; readonly spr: readonly number[]; readonly ops: string } }
  | { readonly Config: { readonly in_reply_to: number; readonly pack: readonly number[]; readonly spr: readonly number[]; readonly ops: string } }
  | { readonly ConfigChanged: { readonly envelopes: readonly (readonly number[])[]; readonly origin: string } }
  | { readonly ContextMenu: { readonly in_reply_to: number; readonly items: readonly number[] } }
  | { readonly Media: { readonly in_reply_to: number; readonly port: string; readonly descriptor: readonly number[]; readonly data: readonly number[] } }
  | { readonly MediaFingerprint: { readonly in_reply_to: number; readonly port: string; readonly fingerprint: readonly number[] } }
  | { readonly Error: { readonly in_reply_to: number | null; readonly fault: readonly number[]; readonly report: readonly number[] } }
  | { readonly Emit: { readonly in_reply_to: number; readonly document_ops: readonly number[]; readonly config_ops: readonly number[]; readonly draft_ops: readonly number[]; readonly output: readonly number[]; readonly diagnostics: readonly number[] } }
  | { readonly Draft: { readonly in_reply_to: number; readonly pack: readonly number[]; readonly spr: readonly number[]; readonly ops: string } }
  | { readonly Children: { readonly in_reply_to: number; readonly entries: readonly ChildPackEntry[] } }
  | { readonly Ephemeral: { readonly presence: readonly number[]; readonly presence_generation: number; readonly transient_generation: number; readonly interaction: readonly number[] } }
  | { readonly HistorySnapshot: { readonly in_reply_to: number; readonly history_patch: readonly number[] } }
  | {
      readonly transactionProposal: {
        readonly in_reply_to: number;
        readonly proposal_id: string;
        readonly local_ops: readonly (readonly number[])[];
        readonly description: string;
        readonly coalesce_key: string;
        readonly foreign: readonly (readonly number[])[];
      };
    }
  | { readonly transactionPrepared: { readonly txn_id: string; readonly foreign: readonly (readonly number[])[]; readonly rejection: readonly number[] } }
  | { readonly transactionCommitted: { readonly txn_id: string; readonly edit_id: string } }
  | { readonly transactionRolledBack: { readonly txn_id: string } }
  | { readonly MergeReport: { readonly in_reply_to: number | null; readonly report: readonly number[] } }
  | { readonly Conflicts: { readonly in_reply_to: number | null; readonly conflicts: readonly number[] } }
  /** 🎨️ Revisioned UI patch batch for one surface — replaces `UiSection`'s cache-probe push.
   * `ops` is `store::pack_rt::encode_wire_value`-encoded `Vec<kernel::PatchOp>` (reused from
   * `semio_framework::kernel`, never redefined here). CHANNEL_VERSION 12 wire addition. */
  | { readonly UiPatch: { readonly in_reply_to: number | null; readonly surface: string; readonly kind: string; readonly revision: number; readonly base_revision: number; readonly ops: readonly number[] } }
  /** 🏁️ Marks the end of one surface's initial full-body snapshot burst. CHANNEL_VERSION 12 wire addition. */
  | { readonly UiSnapshotEnd: { readonly revision: number } };
//#endregion 🔖️Types

//#region 🔖️Combinators
/** 🎞️ `presence u8 | varint` — an `Option<u64>` (e.g. `AppFrame.*.in_reply_to`), the same
 * convention {@link writeOptStr}/{@link writeOptBytes} use. */
function writeOptU64(out: number[], value: number | null): void {
  writeBool(out, value !== null);
  if (value !== null) writeVarintU64(out, value);
}
function readOptU64(bytes: Uint8Array, pos: [number]): number | null {
  return readBool(bytes, pos) ? readVarintU64(bytes, pos) : null;
}
/** 🎞️ `presence u8 | byte` — an `Option<u8>` (`AppCommand.presence.own_color`), the same
 * presence-byte convention as {@link writeOptU64} above. */
function writeOptU8(out: number[], value: number | null): void {
  writeBool(out, value !== null);
  if (value !== null) out.push(value);
}
function readOptU8(bytes: Uint8Array, pos: [number]): number | null {
  if (!readBool(bytes, pos)) return null;
  const byte = bytes[pos[0]]!;
  pos[0] += 1;
  return byte;
}
function writeChildPackEntry(out: number[], entry: ChildPackEntry): void {
  writeStr(out, entry.slot);
  writeStr(out, entry.child_id);
  writeStr(out, entry.dialect);
  writeBytes(out, entry.envelope_pack);
}
function readChildPackEntry(bytes: Uint8Array, pos: [number]): ChildPackEntry {
  return { slot: readStr(bytes, pos), child_id: readStr(bytes, pos), dialect: readStr(bytes, pos), envelope_pack: readBytes(bytes, pos) };
}
function writeVecChildPackEntry(out: number[], entries: readonly ChildPackEntry[]): void {
  writeVarintU64(out, entries.length);
  for (const entry of entries) writeChildPackEntry(out, entry);
}
function readVecChildPackEntry(bytes: Uint8Array, pos: [number]): ChildPackEntry[] {
  const count = readVarintU64(bytes, pos);
  return Array.from({ length: count }, () => readChildPackEntry(bytes, pos));
}
//#endregion 🔖️Combinators

//#region 🔖️Codec
const APP_COMMAND_TAGS = {
  ConfigCommand: 0, Command: 1, CommandText: 2, ContextMenu: 3, ArtifactCommand: 4, ApplyEnvelopes: 5,
  LoadDocument: 6, ReadDocument: 7, LoadConfig: 8, ReadConfig: 9, MediaIn: 10, MediaOut: 11,
  MediaFingerprint: 12, PureCommand: 13, LoadChildren: 14, ReadChildren: 15, ReadHistory: 16,
  transactionPrepare: 17, transactionCommit: 18, transactionRollback: 19, transactionUndo: 20, transactionRedo: 21,
  openArtifact: 22, setDefaultApp: 23, clearDefaultApp: 24,
  setMergePolicy: 25, resolveConflict: 26, readConflicts: 27,
  presence: 28, LocalInteractionQuery: 29,
} as const;
const APP_FRAME_TAGS = {
  Done: 0, Invocation: 1, DocumentChanged: 2, Document: 3,
  Config: 4, ConfigChanged: 5, ContextMenu: 6, Media: 7, MediaFingerprint: 8, Error: 9, Emit: 10, Draft: 11, Children: 12, Ephemeral: 13, HistorySnapshot: 14,
  transactionProposal: 15, transactionPrepared: 16, transactionCommitted: 17, transactionRolledBack: 18,
  MergeReport: 19, Conflicts: 20, UiPatch: 21, UiSnapshotEnd: 22, LocalInteractionQuery: 23,
} as const;

/** 📤️ `tag u8 | fields` — the TS twin of `protocol_channel::encode_app_command` (agreed contract). */
export function encodeAppCommand(cmd: AppCommandValue): Uint8Array {
  const out: number[] = [];
  if ("ConfigCommand" in cmd) {
    out.push(APP_COMMAND_TAGS.ConfigCommand);
    writeVarintU64(out, cmd.ConfigCommand.seq);
    writeBytes(out, cmd.ConfigCommand.command);
  } else if ("Command" in cmd) {
    out.push(APP_COMMAND_TAGS.Command);
    writeVarintU64(out, cmd.Command.seq);
    writeBytes(out, cmd.Command.command);
    writeBytes(out, cmd.Command.view_state);
  } else if ("CommandText" in cmd) {
    out.push(APP_COMMAND_TAGS.CommandText);
    writeVarintU64(out, cmd.CommandText.seq);
    writeStr(out, cmd.CommandText.line);
  } else if ("ContextMenu" in cmd) {
    out.push(APP_COMMAND_TAGS.ContextMenu);
    writeVarintU64(out, cmd.ContextMenu.seq);
    writeBytes(out, cmd.ContextMenu.request);
  } else if ("ArtifactCommand" in cmd) {
    out.push(APP_COMMAND_TAGS.ArtifactCommand);
    writeVarintU64(out, cmd.ArtifactCommand.seq);
    writeBytes(out, cmd.ArtifactCommand.command);
  } else if ("ApplyEnvelopes" in cmd) {
    out.push(APP_COMMAND_TAGS.ApplyEnvelopes);
    writeVarintU64(out, cmd.ApplyEnvelopes.seq);
    writeVecEnvelope(
      out,
      cmd.ApplyEnvelopes.envelopes.map((envelope, index) => mutationEnvelopeToWire(envelope, { actor: 0, physical_ms: 0, logical: index + 1 }, replicationPackCodec)),
    );
  } else if ("LoadDocument" in cmd) {
    out.push(APP_COMMAND_TAGS.LoadDocument);
    writeVarintU64(out, cmd.LoadDocument.seq);
    writeBytes(out, cmd.LoadDocument.pack);
    writeBytes(out, cmd.LoadDocument.spr);
  } else if ("ReadDocument" in cmd) {
    out.push(APP_COMMAND_TAGS.ReadDocument);
    writeVarintU64(out, cmd.ReadDocument.seq);
  } else if ("LoadConfig" in cmd) {
    out.push(APP_COMMAND_TAGS.LoadConfig);
    writeVarintU64(out, cmd.LoadConfig.seq);
    writeBytes(out, cmd.LoadConfig.pack);
    writeBytes(out, cmd.LoadConfig.spr);
  } else if ("ReadConfig" in cmd) {
    out.push(APP_COMMAND_TAGS.ReadConfig);
    writeVarintU64(out, cmd.ReadConfig.seq);
  } else if ("MediaIn" in cmd) {
    out.push(APP_COMMAND_TAGS.MediaIn);
    writeVarintU64(out, cmd.MediaIn.seq);
    writeStr(out, cmd.MediaIn.port);
    writeBytes(out, cmd.MediaIn.descriptor);
    writeBytes(out, cmd.MediaIn.data);
  } else if ("MediaOut" in cmd) {
    out.push(APP_COMMAND_TAGS.MediaOut);
    writeVarintU64(out, cmd.MediaOut.seq);
    writeStr(out, cmd.MediaOut.port);
    writeBytes(out, cmd.MediaOut.request);
  } else if ("MediaFingerprint" in cmd) {
    out.push(APP_COMMAND_TAGS.MediaFingerprint);
    writeVarintU64(out, cmd.MediaFingerprint.seq);
    writeStr(out, cmd.MediaFingerprint.port);
  } else if ("PureCommand" in cmd) {
    out.push(APP_COMMAND_TAGS.PureCommand);
    writeVarintU64(out, cmd.PureCommand.seq);
    writeBytes(out, cmd.PureCommand.command);
    writeBytes(out, cmd.PureCommand.document);
    writeBytes(out, cmd.PureCommand.document_spr);
    writeBytes(out, cmd.PureCommand.config);
    writeBytes(out, cmd.PureCommand.config_spr);
    writeBytes(out, cmd.PureCommand.draft);
    writeBytes(out, cmd.PureCommand.draft_spr);
  } else if ("LoadChildren" in cmd) {
    out.push(APP_COMMAND_TAGS.LoadChildren);
    writeVarintU64(out, cmd.LoadChildren.seq);
    writeVecChildPackEntry(out, cmd.LoadChildren.entries);
  } else if ("ReadChildren" in cmd) {
    out.push(APP_COMMAND_TAGS.ReadChildren);
    writeVarintU64(out, cmd.ReadChildren.seq);
  } else if ("ReadHistory" in cmd) {
    out.push(APP_COMMAND_TAGS.ReadHistory);
    writeVarintU64(out, cmd.ReadHistory.seq);
  } else if ("transactionPrepare" in cmd) {
    out.push(APP_COMMAND_TAGS.transactionPrepare);
    writeVarintU64(out, cmd.transactionPrepare.seq);
    writeStr(out, cmd.transactionPrepare.txn_id);
    writeStr(out, cmd.transactionPrepare.mutation_id);
    writeBytes(out, cmd.transactionPrepare.payload);
    writeVecBytes(out, cmd.transactionPrepare.prepared_ops);
    writeStr(out, cmd.transactionPrepare.label);
    writeBytes(out, cmd.transactionPrepare.origin);
  } else if ("transactionCommit" in cmd) {
    out.push(APP_COMMAND_TAGS.transactionCommit);
    writeVarintU64(out, cmd.transactionCommit.seq);
    writeStr(out, cmd.transactionCommit.txn_id);
  } else if ("transactionRollback" in cmd) {
    out.push(APP_COMMAND_TAGS.transactionRollback);
    writeVarintU64(out, cmd.transactionRollback.seq);
    writeStr(out, cmd.transactionRollback.txn_id);
  } else if ("transactionUndo" in cmd) {
    out.push(APP_COMMAND_TAGS.transactionUndo);
    writeVarintU64(out, cmd.transactionUndo.seq);
    writeStr(out, cmd.transactionUndo.group_id);
  } else if ("transactionRedo" in cmd) {
    out.push(APP_COMMAND_TAGS.transactionRedo);
    writeVarintU64(out, cmd.transactionRedo.seq);
    writeStr(out, cmd.transactionRedo.group_id);
  } else if ("openArtifact" in cmd) {
    out.push(APP_COMMAND_TAGS.openArtifact);
    writeVarintU64(out, cmd.openArtifact.seq);
    writeStr(out, cmd.openArtifact.artifact_ref);
    out.push(cmd.openArtifact.role);
    writeStr(out, cmd.openArtifact.plugin_id);
    writeStr(out, cmd.openArtifact.app_id);
  } else if ("setDefaultApp" in cmd) {
    out.push(APP_COMMAND_TAGS.setDefaultApp);
    writeVarintU64(out, cmd.setDefaultApp.seq);
    writeStr(out, cmd.setDefaultApp.artifact_kind);
    writeStr(out, cmd.setDefaultApp.standard);
    writeStr(out, cmd.setDefaultApp.subset);
    out.push(cmd.setDefaultApp.role);
    writeStr(out, cmd.setDefaultApp.plugin_id);
    writeStr(out, cmd.setDefaultApp.app_id);
  } else if ("clearDefaultApp" in cmd) {
    out.push(APP_COMMAND_TAGS.clearDefaultApp);
    writeVarintU64(out, cmd.clearDefaultApp.seq);
    writeStr(out, cmd.clearDefaultApp.artifact_kind);
    writeStr(out, cmd.clearDefaultApp.standard);
    writeStr(out, cmd.clearDefaultApp.subset);
    out.push(cmd.clearDefaultApp.role);
  } else if ("setMergePolicy" in cmd) {
    out.push(APP_COMMAND_TAGS.setMergePolicy);
    writeVarintU64(out, cmd.setMergePolicy.seq);
    out.push(cmd.setMergePolicy.policy);
  } else if ("resolveConflict" in cmd) {
    out.push(APP_COMMAND_TAGS.resolveConflict);
    writeVarintU64(out, cmd.resolveConflict.seq);
    writeStr(out, cmd.resolveConflict.conflict_id);
    out.push(cmd.resolveConflict.resolution);
  } else if ("readConflicts" in cmd) {
    out.push(APP_COMMAND_TAGS.readConflicts);
    writeVarintU64(out, cmd.readConflicts.seq);
  } else if ("LocalInteractionQuery" in cmd) {
    out.push(APP_COMMAND_TAGS.LocalInteractionQuery);
    writeVarintU64(out, cmd.LocalInteractionQuery.seq);
    writeBytes(out, Array.from(encodeLocalInteractionQueryCommand(cmd.LocalInteractionQuery.command)));
  } else if ("presence" in cmd) {
    out.push(APP_COMMAND_TAGS.presence);
    writeVarintU64(out, cmd.presence.seq);
    writeOptU8(out, cmd.presence.own_color);
    writeVecBytes(out, cmd.presence.peers);
  } else {
    throw new Error("encodeAppCommand: unrecognized command variant");
  }
  return new Uint8Array(out);
}

/** 📥️ Inverse of {@link encodeAppCommand} — the TS twin of `protocol_channel::decode_app_command`. */
export function decodeAppCommand(bytes: Uint8Array): AppCommandValue {
  if (bytes.length === 0) throw new Error("decodeAppCommand: empty frame");
  const pos: [number] = [1];
  switch (bytes[0]) {
    case APP_COMMAND_TAGS.ConfigCommand:
      return { ConfigCommand: { seq: readVarintU64(bytes, pos), command: readBytes(bytes, pos) } };
    case APP_COMMAND_TAGS.Command: {
      const seq = readVarintU64(bytes, pos);
      const command = readBytes(bytes, pos);
      const view_state = readBytes(bytes, pos);
      return { Command: { seq, command, view_state } };
    }
    case APP_COMMAND_TAGS.CommandText:
      return { CommandText: { seq: readVarintU64(bytes, pos), line: readStr(bytes, pos) } };
    case APP_COMMAND_TAGS.ContextMenu:
      return { ContextMenu: { seq: readVarintU64(bytes, pos), request: readBytes(bytes, pos) } };
    case APP_COMMAND_TAGS.ArtifactCommand:
      return { ArtifactCommand: { seq: readVarintU64(bytes, pos), command: readBytes(bytes, pos) } };
    case APP_COMMAND_TAGS.ApplyEnvelopes: {
      const seq = readVarintU64(bytes, pos);
      const wire = readVecEnvelope(bytes, pos);
      return { ApplyEnvelopes: { seq, envelopes: wire.map((envelope) => mutationEnvelopeFromWire(envelope, replicationPackCodec)) } };
    }
    case APP_COMMAND_TAGS.LoadDocument: {
      const seq = readVarintU64(bytes, pos);
      const pack = readBytes(bytes, pos);
      const spr = readBytes(bytes, pos);
      return { LoadDocument: { seq, pack, spr } };
    }
    case APP_COMMAND_TAGS.ReadDocument:
      return { ReadDocument: { seq: readVarintU64(bytes, pos) } };
    case APP_COMMAND_TAGS.LoadConfig: {
      const seq = readVarintU64(bytes, pos);
      const pack = readBytes(bytes, pos);
      const spr = readBytes(bytes, pos);
      return { LoadConfig: { seq, pack, spr } };
    }
    case APP_COMMAND_TAGS.ReadConfig:
      return { ReadConfig: { seq: readVarintU64(bytes, pos) } };
    case APP_COMMAND_TAGS.MediaIn: {
      const seq = readVarintU64(bytes, pos);
      const port = readStr(bytes, pos);
      const descriptor = readBytes(bytes, pos);
      const data = readBytes(bytes, pos);
      return { MediaIn: { seq, port, descriptor, data } };
    }
    case APP_COMMAND_TAGS.MediaOut: {
      const seq = readVarintU64(bytes, pos);
      const port = readStr(bytes, pos);
      const request = readBytes(bytes, pos);
      return { MediaOut: { seq, port, request } };
    }
    case APP_COMMAND_TAGS.MediaFingerprint:
      return { MediaFingerprint: { seq: readVarintU64(bytes, pos), port: readStr(bytes, pos) } };
    case APP_COMMAND_TAGS.PureCommand:
      return { PureCommand: { seq: readVarintU64(bytes, pos), command: readBytes(bytes, pos), document: readBytes(bytes, pos), document_spr: readBytes(bytes, pos), config: readBytes(bytes, pos), config_spr: readBytes(bytes, pos), draft: readBytes(bytes, pos), draft_spr: readBytes(bytes, pos) } };
    case APP_COMMAND_TAGS.LoadChildren:
      return { LoadChildren: { seq: readVarintU64(bytes, pos), entries: readVecChildPackEntry(bytes, pos) } };
    case APP_COMMAND_TAGS.ReadChildren:
      return { ReadChildren: { seq: readVarintU64(bytes, pos) } };
    case APP_COMMAND_TAGS.ReadHistory:
      return { ReadHistory: { seq: readVarintU64(bytes, pos) } };
    case APP_COMMAND_TAGS.transactionPrepare: {
      const seq = readVarintU64(bytes, pos);
      const txn_id = readStr(bytes, pos);
      const mutation_id = readStr(bytes, pos);
      const payload = readBytes(bytes, pos);
      const prepared_ops = readVecBytes(bytes, pos);
      const label = readStr(bytes, pos);
      const origin = readBytes(bytes, pos);
      return { transactionPrepare: { seq, txn_id, mutation_id, payload, prepared_ops, label, origin } };
    }
    case APP_COMMAND_TAGS.transactionCommit:
      return { transactionCommit: { seq: readVarintU64(bytes, pos), txn_id: readStr(bytes, pos) } };
    case APP_COMMAND_TAGS.transactionRollback:
      return { transactionRollback: { seq: readVarintU64(bytes, pos), txn_id: readStr(bytes, pos) } };
    case APP_COMMAND_TAGS.transactionUndo:
      return { transactionUndo: { seq: readVarintU64(bytes, pos), group_id: readStr(bytes, pos) } };
    case APP_COMMAND_TAGS.transactionRedo:
      return { transactionRedo: { seq: readVarintU64(bytes, pos), group_id: readStr(bytes, pos) } };
    case APP_COMMAND_TAGS.openArtifact: {
      const seq = readVarintU64(bytes, pos);
      const artifact_ref = readStr(bytes, pos);
      const role = bytes[pos[0]]!;
      pos[0] += 1;
      const plugin_id = readStr(bytes, pos);
      const app_id = readStr(bytes, pos);
      return { openArtifact: { seq, artifact_ref, role, plugin_id, app_id } };
    }
    case APP_COMMAND_TAGS.setDefaultApp: {
      const seq = readVarintU64(bytes, pos);
      const artifact_kind = readStr(bytes, pos);
      const standard = readStr(bytes, pos);
      const subset = readStr(bytes, pos);
      const role = bytes[pos[0]]!;
      pos[0] += 1;
      const plugin_id = readStr(bytes, pos);
      const app_id = readStr(bytes, pos);
      return { setDefaultApp: { seq, artifact_kind, standard, subset, role, plugin_id, app_id } };
    }
    case APP_COMMAND_TAGS.clearDefaultApp: {
      const seq = readVarintU64(bytes, pos);
      const artifact_kind = readStr(bytes, pos);
      const standard = readStr(bytes, pos);
      const subset = readStr(bytes, pos);
      const role = bytes[pos[0]]!;
      pos[0] += 1;
      return { clearDefaultApp: { seq, artifact_kind, standard, subset, role } };
    }
    case APP_COMMAND_TAGS.setMergePolicy: {
      const seq = readVarintU64(bytes, pos);
      const policy = bytes[pos[0]]!;
      pos[0] += 1;
      return { setMergePolicy: { seq, policy } };
    }
    case APP_COMMAND_TAGS.resolveConflict: {
      const seq = readVarintU64(bytes, pos);
      const conflict_id = readStr(bytes, pos);
      const resolution = bytes[pos[0]]!;
      pos[0] += 1;
      return { resolveConflict: { seq, conflict_id, resolution } };
    }
    case APP_COMMAND_TAGS.readConflicts:
      return { readConflicts: { seq: readVarintU64(bytes, pos) } };
    case APP_COMMAND_TAGS.presence: {
      const seq = readVarintU64(bytes, pos);
      const own_color = readOptU8(bytes, pos);
      const peers = readVecBytes(bytes, pos);
      return { presence: { seq, own_color, peers } };
    }
    case APP_COMMAND_TAGS.LocalInteractionQuery: {
      const seq = readVarintU64(bytes, pos);
      const length = readVarintU64(bytes, pos);
      if (length > 142 || pos[0] + length !== bytes.length) throw new Error("local-interaction.command-envelope");
      const command = decodeLocalInteractionQueryCommand(bytes.subarray(pos[0]));
      return { LocalInteractionQuery: { seq, command } };
    }
    default:
      throw new Error(`decodeAppCommand: unknown tag ${bytes[0]}`);
  }
}

/** 📤️ `tag u8 | fields` — the TS twin of `protocol_channel::encode_app_frame` (agreed contract). */
export function encodeAppFrame(frame: AppFrameValue): Uint8Array {
  const out: number[] = [];
  if ("Done" in frame) {
    out.push(APP_FRAME_TAGS.Done);
    writeVarintU64(out, frame.Done.in_reply_to);
  } else if ("Invocation" in frame) {
    out.push(APP_FRAME_TAGS.Invocation);
    writeVarintU64(out, frame.Invocation.in_reply_to);
    writeBytes(out, frame.Invocation.output);
    writeBytes(out, frame.Invocation.diagnostics);
    writeBytes(out, frame.Invocation.ui_scope);
    writeBytes(out, frame.Invocation.history_patch);
    writeBytes(out, frame.Invocation.messages);
  } else if ("DocumentChanged" in frame) {
    out.push(APP_FRAME_TAGS.DocumentChanged);
    writeVecBytes(out, frame.DocumentChanged.envelopes);
    writeStr(out, frame.DocumentChanged.origin);
  } else if ("Document" in frame) {
    out.push(APP_FRAME_TAGS.Document);
    writeVarintU64(out, frame.Document.in_reply_to);
    writeBytes(out, frame.Document.pack);
    writeBytes(out, frame.Document.spr);
    writeStr(out, frame.Document.ops);
  } else if ("Config" in frame) {
    out.push(APP_FRAME_TAGS.Config);
    writeVarintU64(out, frame.Config.in_reply_to);
    writeBytes(out, frame.Config.pack);
    writeBytes(out, frame.Config.spr);
    writeStr(out, frame.Config.ops);
  } else if ("ConfigChanged" in frame) {
    out.push(APP_FRAME_TAGS.ConfigChanged);
    writeVecBytes(out, frame.ConfigChanged.envelopes);
    writeStr(out, frame.ConfigChanged.origin);
  } else if ("ContextMenu" in frame) {
    out.push(APP_FRAME_TAGS.ContextMenu);
    writeVarintU64(out, frame.ContextMenu.in_reply_to);
    writeBytes(out, frame.ContextMenu.items);
  } else if ("Media" in frame) {
    out.push(APP_FRAME_TAGS.Media);
    writeVarintU64(out, frame.Media.in_reply_to);
    writeStr(out, frame.Media.port);
    writeBytes(out, frame.Media.descriptor);
    writeBytes(out, frame.Media.data);
  } else if ("MediaFingerprint" in frame) {
    out.push(APP_FRAME_TAGS.MediaFingerprint);
    writeVarintU64(out, frame.MediaFingerprint.in_reply_to);
    writeStr(out, frame.MediaFingerprint.port);
    writeBytes(out, frame.MediaFingerprint.fingerprint);
  } else if ("Error" in frame) {
    out.push(APP_FRAME_TAGS.Error);
    writeOptU64(out, frame.Error.in_reply_to);
    writeBytes(out, frame.Error.fault);
    writeBytes(out, frame.Error.report);
  } else if ("Emit" in frame) {
    out.push(APP_FRAME_TAGS.Emit);
    writeVarintU64(out, frame.Emit.in_reply_to);
    writeBytes(out, frame.Emit.document_ops);
    writeBytes(out, frame.Emit.config_ops);
    writeBytes(out, frame.Emit.draft_ops);
    writeBytes(out, frame.Emit.output);
    writeBytes(out, frame.Emit.diagnostics);
  } else if ("Draft" in frame) {
    out.push(APP_FRAME_TAGS.Draft);
    writeVarintU64(out, frame.Draft.in_reply_to);
    writeBytes(out, frame.Draft.pack);
    writeBytes(out, frame.Draft.spr);
    writeStr(out, frame.Draft.ops);
  } else if ("Children" in frame) {
    out.push(APP_FRAME_TAGS.Children);
    writeVarintU64(out, frame.Children.in_reply_to);
    writeVecChildPackEntry(out, frame.Children.entries);
  } else if ("Ephemeral" in frame) {
    out.push(APP_FRAME_TAGS.Ephemeral);
    writeBytes(out, frame.Ephemeral.presence);
    writeVarintU64(out, frame.Ephemeral.presence_generation);
    writeVarintU64(out, frame.Ephemeral.transient_generation);
    writeBytes(out, frame.Ephemeral.interaction);
  } else if ("HistorySnapshot" in frame) {
    out.push(APP_FRAME_TAGS.HistorySnapshot);
    writeVarintU64(out, frame.HistorySnapshot.in_reply_to);
    writeBytes(out, frame.HistorySnapshot.history_patch);
  } else if ("transactionProposal" in frame) {
    out.push(APP_FRAME_TAGS.transactionProposal);
    writeVarintU64(out, frame.transactionProposal.in_reply_to);
    writeStr(out, frame.transactionProposal.proposal_id);
    writeVecBytes(out, frame.transactionProposal.local_ops);
    writeStr(out, frame.transactionProposal.description);
    writeStr(out, frame.transactionProposal.coalesce_key);
    writeVecBytes(out, frame.transactionProposal.foreign);
  } else if ("transactionPrepared" in frame) {
    out.push(APP_FRAME_TAGS.transactionPrepared);
    writeStr(out, frame.transactionPrepared.txn_id);
    writeVecBytes(out, frame.transactionPrepared.foreign);
    writeBytes(out, frame.transactionPrepared.rejection);
  } else if ("transactionCommitted" in frame) {
    out.push(APP_FRAME_TAGS.transactionCommitted);
    writeStr(out, frame.transactionCommitted.txn_id);
    writeStr(out, frame.transactionCommitted.edit_id);
  } else if ("transactionRolledBack" in frame) {
    out.push(APP_FRAME_TAGS.transactionRolledBack);
    writeStr(out, frame.transactionRolledBack.txn_id);
  } else if ("MergeReport" in frame) {
    out.push(APP_FRAME_TAGS.MergeReport);
    writeOptU64(out, frame.MergeReport.in_reply_to);
    writeBytes(out, frame.MergeReport.report);
  } else if ("Conflicts" in frame) {
    out.push(APP_FRAME_TAGS.Conflicts);
    writeOptU64(out, frame.Conflicts.in_reply_to);
    writeBytes(out, frame.Conflicts.conflicts);
  } else if ("UiPatch" in frame) {
    out.push(APP_FRAME_TAGS.UiPatch);
    writeOptU64(out, frame.UiPatch.in_reply_to);
    writeStr(out, frame.UiPatch.surface);
    writeStr(out, frame.UiPatch.kind);
    writeVarintU64(out, frame.UiPatch.revision);
    writeVarintU64(out, frame.UiPatch.base_revision);
    writeBytes(out, frame.UiPatch.ops);
  } else if ("LocalInteractionQuery" in frame) {
    out.push(APP_FRAME_TAGS.LocalInteractionQuery);
    writeBytes(out, Array.from(encodeLocalInteractionQueryReply(frame.LocalInteractionQuery.reply)));
  } else if ("UiSnapshotEnd" in frame) {
    out.push(APP_FRAME_TAGS.UiSnapshotEnd);
    writeVarintU64(out, frame.UiSnapshotEnd.revision);
  } else {
    throw new Error("encodeAppFrame: unrecognized frame variant");
  }
  return new Uint8Array(out);
}

/** 📥️ Inverse of {@link encodeAppFrame} — the TS twin of `protocol_channel::decode_app_frame`. */
export function decodeAppFrame(bytes: Uint8Array): AppFrameValue {
  if (bytes.length === 0) throw new Error("decodeAppFrame: empty frame");
  const pos: [number] = [1];
  switch (bytes[0]) {
    case APP_FRAME_TAGS.Done:
      return { Done: { in_reply_to: readVarintU64(bytes, pos) } };
    case APP_FRAME_TAGS.Invocation: {
      const in_reply_to = readVarintU64(bytes, pos);
      const output = readBytes(bytes, pos);
      const diagnostics = readBytes(bytes, pos);
      const ui_scope = readBytes(bytes, pos);
      const history_patch = readBytes(bytes, pos);
      const messages = readBytes(bytes, pos);
      return { Invocation: { in_reply_to, output, diagnostics, ui_scope, history_patch, messages } };
    }
    case APP_FRAME_TAGS.DocumentChanged: {
      const envelopes = readVecBytes(bytes, pos);
      const origin = readStr(bytes, pos);
      return { DocumentChanged: { envelopes, origin } };
    }
    case APP_FRAME_TAGS.Document: {
      const in_reply_to = readVarintU64(bytes, pos);
      const pack = readBytes(bytes, pos);
      const spr = readBytes(bytes, pos);
      const ops = readStr(bytes, pos);
      return { Document: { in_reply_to, pack, spr, ops } };
    }
    case APP_FRAME_TAGS.Config: {
      const in_reply_to = readVarintU64(bytes, pos);
      const pack = readBytes(bytes, pos);
      const spr = readBytes(bytes, pos);
      const ops = readStr(bytes, pos);
      return { Config: { in_reply_to, pack, spr, ops } };
    }
    case APP_FRAME_TAGS.ConfigChanged: {
      const envelopes = readVecBytes(bytes, pos);
      const origin = readStr(bytes, pos);
      return { ConfigChanged: { envelopes, origin } };
    }
    case APP_FRAME_TAGS.ContextMenu:
      return { ContextMenu: { in_reply_to: readVarintU64(bytes, pos), items: readBytes(bytes, pos) } };
    case APP_FRAME_TAGS.Media: {
      const in_reply_to = readVarintU64(bytes, pos);
      const port = readStr(bytes, pos);
      const descriptor = readBytes(bytes, pos);
      const data = readBytes(bytes, pos);
      return { Media: { in_reply_to, port, descriptor, data } };
    }
    case APP_FRAME_TAGS.MediaFingerprint: {
      const in_reply_to = readVarintU64(bytes, pos);
      const port = readStr(bytes, pos);
      const fingerprint = readBytes(bytes, pos);
      return { MediaFingerprint: { in_reply_to, port, fingerprint } };
    }
    case APP_FRAME_TAGS.Error: {
      const in_reply_to = readOptU64(bytes, pos);
      const fault = readBytes(bytes, pos);
      const report = readBytes(bytes, pos);
      return { Error: { in_reply_to, fault, report } };
    }
    case APP_FRAME_TAGS.Emit:
      return { Emit: { in_reply_to: readVarintU64(bytes, pos), document_ops: readBytes(bytes, pos), config_ops: readBytes(bytes, pos), draft_ops: readBytes(bytes, pos), output: readBytes(bytes, pos), diagnostics: readBytes(bytes, pos) } };
    case APP_FRAME_TAGS.Draft:
      return { Draft: { in_reply_to: readVarintU64(bytes, pos), pack: readBytes(bytes, pos), spr: readBytes(bytes, pos), ops: readStr(bytes, pos) } };
    case APP_FRAME_TAGS.Children:
      return { Children: { in_reply_to: readVarintU64(bytes, pos), entries: readVecChildPackEntry(bytes, pos) } };
    case APP_FRAME_TAGS.Ephemeral:
      return {
        Ephemeral: { presence: readBytes(bytes, pos), presence_generation: readVarintU64(bytes, pos), transient_generation: readVarintU64(bytes, pos), interaction: readBytes(bytes, pos) },
      };
    case APP_FRAME_TAGS.HistorySnapshot:
      return { HistorySnapshot: { in_reply_to: readVarintU64(bytes, pos), history_patch: readBytes(bytes, pos) } };
    case APP_FRAME_TAGS.transactionProposal: {
      const in_reply_to = readVarintU64(bytes, pos);
      const proposal_id = readStr(bytes, pos);
      const local_ops = readVecBytes(bytes, pos);
      const description = readStr(bytes, pos);
      const coalesce_key = readStr(bytes, pos);
      const foreign = readVecBytes(bytes, pos);
      return { transactionProposal: { in_reply_to, proposal_id, local_ops, description, coalesce_key, foreign } };
    }
    case APP_FRAME_TAGS.transactionPrepared: {
      const txn_id = readStr(bytes, pos);
      const foreign = readVecBytes(bytes, pos);
      const rejection = readBytes(bytes, pos);
      return { transactionPrepared: { txn_id, foreign, rejection } };
    }
    case APP_FRAME_TAGS.transactionCommitted:
      return { transactionCommitted: { txn_id: readStr(bytes, pos), edit_id: readStr(bytes, pos) } };
    case APP_FRAME_TAGS.transactionRolledBack:
      return { transactionRolledBack: { txn_id: readStr(bytes, pos) } };
    case APP_FRAME_TAGS.MergeReport:
      return { MergeReport: { in_reply_to: readOptU64(bytes, pos), report: readBytes(bytes, pos) } };
    case APP_FRAME_TAGS.Conflicts:
      return { Conflicts: { in_reply_to: readOptU64(bytes, pos), conflicts: readBytes(bytes, pos) } };
    case APP_FRAME_TAGS.UiPatch: {
      const in_reply_to = readOptU64(bytes, pos);
      const surface = readStr(bytes, pos);
      const kind = readStr(bytes, pos);
      const revision = readVarintU64(bytes, pos);
      const base_revision = readVarintU64(bytes, pos);
      const ops = readBytes(bytes, pos);
      return { UiPatch: { in_reply_to, surface, kind, revision, base_revision, ops } };
    }
    case APP_FRAME_TAGS.UiSnapshotEnd:
      return { UiSnapshotEnd: { revision: readVarintU64(bytes, pos) } };
    case APP_FRAME_TAGS.LocalInteractionQuery: {
      const length = readVarintU64(bytes, pos);
      if (length > 4256 || pos[0] + length !== bytes.length) throw new Error("local-interaction.reply-envelope");
      return { LocalInteractionQuery: { reply: decodeLocalInteractionQueryReply(bytes.subarray(pos[0])) } };
    }
    default:
      throw new Error(`decodeAppFrame: unknown tag ${bytes[0]}`);
  }
}
//#endregion 🔖️Codec
//#endregion 🔖️AppChannelCodec

//#region 🔖️AppChannelClient
/**
 * 📡️ TS twin of `protocol_channel::CHANNEL_VERSION` (`🔨️modules/📡️protocol/🧵️channel/📦️packages/🦀️rust/📦️lib.rs`)
 * — bump both sides together on a wire-incompatible frame change.
 */
/** @emoji 📥️ Decodes a pack-encoded {@link Fault} from an app-channel error frame. */
export function decodeFaultFromWire(faultBytes: readonly number[], decodePackValue: (bytes: Uint8Array) => unknown): Fault | null {
  try {
    const raw = decodePackValue(new Uint8Array(faultBytes));
    if (!raw || typeof raw !== "object" || !("message" in raw)) return null;
    return raw as Fault;
  } catch {
    return null;
  }
}

export function faultDisplayMessage(faultBytes: readonly number[], decodePackValue: (bytes: Uint8Array) => unknown): string {
  const fault = decodeFaultFromWire(faultBytes, decodePackValue);
  if (!fault) return "unknown fault";
  const code = typeof fault.code === "string" ? fault.code : String(fault.code);
  return `${code}: ${fault.message}`;
}

/** @emoji 📥️ Decodes a pack-encoded {@link DispatchReport} from an app-channel wire blob —
 * `AppFrame::Invocation.messages` (a successful dispatch's report) or `AppFrame::Error.report` (the
 * rejected dispatch's report, `Fault.code == "mutation.rejected"`). `null` for an empty blob (the
 * trailing field's zero value before every dispatch path was updated to populate it). */
export function decodeDispatchReportFromWire(reportBytes: readonly number[], decodePackValue: (bytes: Uint8Array) => unknown): DispatchReport | null {
  if (reportBytes.length === 0) return null;
  try {
    return decodePackValue(new Uint8Array(reportBytes)) as DispatchReport;
  } catch {
    return null;
  }
}

/** @emoji 📨️ Decodes an `AppFrame::Error.report` blob into its typed `MutationMessage`s — so a
 * caller reacting to a rejected dispatch (contract-freeze §C8/§C9) gets structured messages instead
 * of parsing {@link faultDisplayMessage}'s prose string. Empty array for an empty/undecodable blob. */
export function faultMessages(reportBytes: readonly number[], decodePackValue: (bytes: Uint8Array) => unknown): readonly MutationMessage[] {
  return decodeDispatchReportFromWire(reportBytes, decodePackValue)?.messages ?? [];
}

/** @emoji 📥️ Decodes a pack-encoded {@link MergeReport} from an `AppFrame::MergeReport.report`
 * blob — pushed unsolicited after every `ingest_remote`/`merge_remote_snapshot`/`resolve_conflict`,
 * alongside `DocumentChanged`. */
export function decodeMergeReportFromWire(reportBytes: readonly number[], decodePackValue: (bytes: Uint8Array) => unknown): MergeReport | null {
  if (reportBytes.length === 0) return null;
  try {
    return decodePackValue(new Uint8Array(reportBytes)) as MergeReport;
  } catch {
    return null;
  }
}

/** @emoji 📥️ Decodes a pack-encoded {@link Conflict}[] projection from an `AppFrame::
 * Conflicts.conflicts` blob — pushed unsolicited after every ingest (alongside `DocumentChanged`)
 * and in reply to `AppCommand::ReadConflicts`. */
export function decodeConflictsFromWire(conflictsBytes: readonly number[], decodePackValue: (bytes: Uint8Array) => unknown): readonly Conflict[] {
  if (conflictsBytes.length === 0) return [];
  try {
    return decodePackValue(new Uint8Array(conflictsBytes)) as readonly Conflict[];
  } catch {
    return [];
  }
}

/** 📡️ TS twin of `protocol_channel::CHANNEL_VERSION`. Both constants are pinned against
 * `🧫️fixtures/📡️channel/channel-version.json`, which owns the number — this one sat at 8 while Rust
 * had moved to 10, so the pin exists to make a half-done bump fail a test instead of a session.
 * Channel v12 retired the `Hello`/`Welcome` handshake this constant used to be carried on — it now
 * exists purely for the drift-guard test below. */
const APP_CHANNEL_VERSION = 13;

/** 📡️ The slice of {@link PluginWasmHandle} {@link AppChannelClient} needs — deliberately narrower
 * than the full handle so a caller can hand in any object shaped like it (a real handle, a test
 * double, ...) without importing the rest of `@semio-tech/framework`'s plugin-loading surface. */
export type AppChannelHandle = Pick<PluginWasmHandle, "enqueue" | "outcomes">;

//#region 🏠️LocalQueryOwnership
type LocalInteractionClientQuery = {
  readonly requestId: string;
  readonly cancelSequence: number;
  readonly consume: (page: LocalInteractionPage) => Promise<void>;
  readonly resolve: (identity: LocalInteractionIdentity) => void;
  readonly reject: (error: unknown) => void;
  readonly signal: AbortSignal | undefined;
  readonly abort: () => void;
  token: LocalInteractionQueryToken | null;
  nextOrdinal: bigint;
  consuming: boolean;
  cancelled: boolean;
  terminalConsumed: boolean;
  failure: unknown;
};

function sameLocalInteractionQuery(left: LocalInteractionQueryToken, right: LocalInteractionQueryToken): boolean {
  return left.requestId === right.requestId && left.queryGeneration === right.queryGeneration && localInteractionIdentityEquals(left.identity, right.identity);
}
type AppChannelTransactionReply = { readonly kind: "prepared" | "committed" | "rolledBack"; readonly id: string };

function appChannelTransactionReply(command: AppCommandValue): AppChannelTransactionReply | null {
  if ("transactionPrepare" in command) return { kind: "prepared", id: command.transactionPrepare.txn_id };
  if ("transactionCommit" in command) return { kind: "committed", id: command.transactionCommit.txn_id };
  if ("transactionRollback" in command) return { kind: "rolledBack", id: command.transactionRollback.txn_id };
  return null;
}

function appChannelReplySequence(frame: AppFrameValue): number | null {
  const value = Object.values(frame)[0];
  return value && "in_reply_to" in value && typeof value.in_reply_to === "number" ? value.in_reply_to : null;
}

function appChannelFrameBelongsTo(frame: AppFrameValue, sequence: number, transaction: AppChannelTransactionReply | null): boolean {
  const replySequence = appChannelReplySequence(frame);
  if (replySequence !== null) return replySequence === sequence;
  if ("transactionPrepared" in frame) return transaction?.kind === "prepared" && transaction.id === frame.transactionPrepared.txn_id;
  if ("transactionCommitted" in frame) return transaction?.kind === "committed" && transaction.id === frame.transactionCommitted.txn_id;
  if ("transactionRolledBack" in frame) return transaction?.kind === "rolledBack" && transaction.id === frame.transactionRolledBack.txn_id;
  return true;
}
/** 🪪️ One checked handle-lifetime owner spans every client recreation; query admission reserves its cancellation receipt identity. */
export class AppChannelRequestSequence {
  constructor(private sequence = 0, private request = 0n) {
    if (!Number.isSafeInteger(sequence) || sequence < 0 || request < 0n || request > 0xffffffffffffffffn) throw new Error("app-channel.invalid-sequence-owner");
  }

  nextSequence(): number {
    if (this.sequence === Number.MAX_SAFE_INTEGER) throw new Error("app-channel.sequence-exhausted");
    return ++this.sequence;
  }

  nextQuery(): { readonly sequence: number; readonly cancelSequence: number; readonly request: string } {
    if (this.sequence > Number.MAX_SAFE_INTEGER - 2) throw new Error("app-channel.sequence-exhausted");
    if (this.request === 0xffffffffffffffffn) throw new Error("local-interaction.request-exhausted");
    const sequence = this.sequence + 1;
    this.sequence += 2;
    return { sequence, cancelSequence: this.sequence, request: (++this.request).toString() };
  }

  checkpoint(): { readonly sequence: number; readonly request: string } {
    return { sequence: this.sequence, request: this.request.toString() };
  }
}
//#endregion 🏠️LocalQueryOwnership

/**
 * 📡️ Typed facade over one plugin instance's app channel — encodes an {@link AppCommandValue}, queues
 * it via {@link PluginWasmHandle.enqueue}, and decodes every {@link AppFrameValue} the matching
 * {@link TurnOutcome} carries. This is the ONLY place `AppCommand`/`AppFrame` framing happens on the
 * host side; callers (a React renderer's dispatch/refresh loop, a headless workflow runner) work with
 * decoded frames and plain JS values, never raw bytes or wire tags. `seq` is a handle-owned monotonic
 * counter — the host has no other way to correlate a `Command`/`ConfigCommand`/`LoadDocument`/
 * `ReadDocument`/`LoadConfig`/`ReadConfig` with the `Invocation`/`Document` frame(s) it produced
 * (`AppFrame.*.in_reply_to`). Channel v12 retired the `hello()`/`refreshUi()`/`attachBackbone()`/
 * `detachBackbone()`/`drain()` surface this class used to expose — the handshake, cache-probed UI
 * refresh, and empty-batch drain all disappeared with the reactor ABI (lifecycle now arrives via
 * `Event::InstanceOpen`/`InstanceClose`, UI updates are a `UiPatch` push, and guests wake on
 * events/timers/`next-wake` rather than a poll).
 *
 * `📌️important.md`'s "Replace, never wrap" list: the old handle's synchronous
 * per-call method used to hand this class its reply directly; the handle is now fire-and-forget
 * ({@link PluginWasmHandle.enqueue}) and replies arrive on the handle-wide {@link
 * PluginWasmHandle.outcomes} stream instead, so THIS class is what turns that stream back into the
 * one-reply-per-call shape every method below still returns. One background loop
 * ({@link pumpOutcomes}) owns the handle's async iterator for this instance's lifetime and routes
 * every explicit numeric receipt to its exact waiter. Empty outcomes and uncorrelated notifications
 * cannot complete a command. Transaction payloads accompany their numeric receipt and additionally
 * match the pending transaction identity. One outcome may complete several distinct commands.
 */
export class AppChannelClient {
  private localQuery: LocalInteractionClientQuery | null = null;
  private disposed = false;
  private readonly handle: AppChannelHandle;
  private readonly instanceId: number;
  private readonly appId: string;
  private readonly actor: string;
  private readonly outcomeIterator: AsyncIterator<TurnOutcome>;
  private readonly pending: { readonly seq: number; readonly queryReceipt: boolean; readonly transaction: AppChannelTransactionReply | null; readonly resolve: (frames: AppFrameValue[]) => void; readonly reject: (error: unknown) => void }[] = [];
  /** 📦️ Per-instance document-pack cache (ticket
   * 26/08/16/PLUGIN-DEPENDENCIES-ARTIFACT-CONTRIBUTIONS-AND-COMPOSITE-MUTATIONS, scout-1 §4: "the
   * browser host keeps NO document pack per instance today"). Populated from BOTH directions —
   * {@link loadDocument}'s own arguments (no round trip needed to know what we just sent) and every
   * `AppFrame::Document` reply any sent command's outcome carries (`ReadDocument`, `LoadDocument`'s
   * own echo, or any future command that happens to include one) — so a transaction coordinator can
   * ask "what does this instance's document look like right now" without a dedicated round trip. */
  private cachedPack: Uint8Array | null = null;
  private cachedSpr: Uint8Array | null = null;

  constructor(handle: AppChannelHandle, private readonly sequenceOwner: AppChannelRequestSequence, instanceId: number, appId: string, actor: string = "local") {
    this.handle = handle;
    this.instanceId = instanceId;
    this.appId = appId;
    this.actor = actor;
    this.outcomeIterator = handle.outcomes[Symbol.asyncIterator]();
    void this.pumpOutcomes();
  }

  /** 🔁️ Owns this client's subscription against the handle-wide {@link PluginWasmHandle.outcomes}
   * stream for its whole lifetime — runs until {@link dispose} calls the iterator's own `return()`
   * (what breaks this loop) or the handle itself completes it. Every outcome for a DIFFERENT
   * `instanceId` is silently skipped (it belongs to a sibling `AppChannelClient` on the same handle);
   * stale or duplicate numeric receipts do not consume another command's waiter. Query pages retain
   * their separate ACK-owned lifecycle even when coalesced with ordinary replies or notifications. */
  private async pumpOutcomes(): Promise<void> {
    for (;;) {
      const step = await this.outcomeIterator.next();
      if (step.done) {
        this.finishLocalInteractionQuery(new Error("local-interaction.channel-closed"));
        for (const waiter of this.pending.splice(0)) waiter.reject(new Error("app-channel.closed"));
        return;
      }
      const outcome = step.value;
      if (outcome.instanceId !== this.instanceId) continue;
      if ("error" in outcome) {
        this.finishLocalInteractionQuery(outcome.error);
        this.pending.shift()?.reject(outcome.error);
        continue;
      }
      const frames: AppFrameValue[] = [];
      for (const encoded of outcome.frames) {
        try { frames.push(decodeAppFrame(encoded)); }
        catch (error) {
          this.cancelLocalInteractionQuery(error);
          for (let index = this.pending.length - 1; index >= 0; index -= 1) {
            if (!this.pending[index]!.queryReceipt) this.pending.splice(index, 1)[0]!.reject(error);
          }
        }
      }
      const ordinary: AppFrameValue[] = [];
      for (const frame of frames) {
        if ("LocalInteractionQuery" in frame) this.receiveLocalInteractionQuery(frame.LocalInteractionQuery.reply);
        else ordinary.push(frame);
      }
      const correlated = new Set(ordinary.flatMap((frame) => { const sequence = appChannelReplySequence(frame); return sequence === null ? [] : [sequence]; }));
      for (let index = 0; index < this.pending.length;) {
        const waiter = this.pending[index]!;
        if (!correlated.has(waiter.seq)) { index += 1; continue; }
        this.pending.splice(index, 1);
        const reply = ordinary.filter((frame) => appChannelFrameBelongsTo(frame, waiter.seq, waiter.transaction));
        this.captureDocumentFrames(reply);
        waiter.resolve(reply);
      }
      this.finishDisposal();
    }
  }

  /** 🔌️ Ends this client's background {@link pumpOutcomes} subscription — call once from
   * `destroyApp` (`PluginRuntime/🟦️component.tsx`) so a torn-down instance doesn't leak a live
   * subscriber against the handle-wide outcome stream for the rest of the handle's lifetime. */
  dispose(): void {
    this.disposed = true;
    for (let index = this.pending.length - 1; index >= 0; index -= 1) {
      if (!this.pending[index]!.queryReceipt) this.pending.splice(index, 1)[0]!.reject(new Error("app-channel.disposed"));
    }
    if (this.localQuery) this.cancelLocalInteractionQuery(new Error("local-interaction.disposed"));
    this.finishDisposal();
  }

  private finishDisposal(): void {
    if (this.disposed && this.localQuery === null && this.pending.length === 0) void this.outcomeIterator.return?.();
  }

  private nextSeq(): number {
    return this.sequenceOwner.nextSequence();
  }

  /** 📦️ Scans every frame one sent command's outcome carried for `AppFrame::Document` and refreshes
   * the pack cache — the "every `AppFrame::Document` reply" half of the cache-population contract. */
  private captureDocumentFrames(frames: readonly AppFrameValue[]): void {
    for (const frame of frames) {
      if ("Document" in frame) {
        this.cachedPack = new Uint8Array(frame.Document.pack);
        this.cachedSpr = new Uint8Array(frame.Document.spr);
      }
    }
  }

  /** 📦️ The cached `{pack, spr}` for this instance's document, or `null` before any
   * {@link loadDocument} call or `AppFrame::Document` reply has been observed. Surfaced to the
   * transaction coordinator through the `PluginWasmHandle` adapter's own `documentPack` accessor
   * (`PluginRuntime/🟦️component.tsx`) — a contributor plan call needs the target's current snapshot
   * pack, and this is the only place that snapshot is retained host-side. */
  documentPack(): { readonly pack: Uint8Array; readonly spr: Uint8Array } | null {
    return this.cachedPack && this.cachedSpr ? { pack: this.cachedPack, spr: this.cachedSpr } : null;
  }

  /** 🔀️ Queues one encoded command and resolves with every frame its matching {@link TurnOutcome}
   * carries — see this class's own header doc for how the reply gets correlated back to this call. */
  private sendCommand(command: AppCommandValue): Promise<AppFrameValue[]> {
    if (this.disposed) return Promise.reject(new Error("app-channel.disposed"));
    return new Promise<AppFrameValue[]>((resolve, reject) => {
      const seq = Object.values(command)[0]!.seq;
      this.pending.push({ seq, queryReceipt: false, transaction: appChannelTransactionReply(command), resolve, reject });
      this.handle.enqueue(this.instanceId, [encodeAppCommand(command)]);
    });
  }

  //#region 🏠️LocalInteractionQuery
  /** 📃️ Each page remains native-owned until its consumer resolves; completion waits for exact native root retirement. */
  readLocalInteractionPages(consume: (page: LocalInteractionPage) => Promise<void>, signal?: AbortSignal): Promise<LocalInteractionIdentity> {
    if (this.disposed) return Promise.reject(new Error("app-channel.disposed"));
    if (this.localQuery) return Promise.reject(new Error("local-interaction.busy"));
    if (signal?.aborted) return Promise.reject(new Error("local-interaction.cancelled"));
    let admission: ReturnType<AppChannelRequestSequence["nextQuery"]>;
    try { admission = this.sequenceOwner.nextQuery(); }
    catch (error) { return Promise.reject(error); }
    const requestId = admission.request;
    return new Promise((resolve, reject) => {
      const abort = () => this.cancelLocalInteractionQuery(new Error("local-interaction.cancelled"));
      this.localQuery = { requestId, cancelSequence: admission.cancelSequence, consume, resolve, reject, signal, abort, token: null, nextOrdinal: 0n, consuming: false, cancelled: false, terminalConsumed: false, failure: null };
      signal?.addEventListener("abort", abort, { once: true });
      this.sendLocalInteractionQuery(admission.sequence, { kind: "read", requestId });
    });
  }

  private sendLocalInteractionQuery(seq: number, command: LocalInteractionQueryCommand): void {
    this.pending.push({ seq, queryReceipt: true, transaction: null, resolve: () => {}, reject: (error: unknown) => this.finishLocalInteractionQuery(error) });
    try { this.handle.enqueue(this.instanceId, [encodeAppCommand({ LocalInteractionQuery: { seq, command } })]); }
    catch (error) {
      const index = this.pending.findIndex((waiter) => waiter.seq === seq);
      if (index !== -1) this.pending.splice(index, 1);
      this.finishLocalInteractionQuery(error);
    }
  }

  private cancelLocalInteractionQuery(error: unknown): void {
    const query = this.localQuery;
    if (!query || query.cancelled) return;
    query.cancelled = true;
    query.failure = error;
    if (query.token) this.sendLocalInteractionQuery(query.cancelSequence, { kind: "cancel", token: query.token });
  }

  private finishLocalInteractionQuery(error: unknown = null): void {
    const query = this.localQuery;
    if (!query) return;
    this.localQuery = null;
    for (let index = this.pending.length - 1; index >= 0; index -= 1) {
      if (this.pending[index]!.queryReceipt) this.pending.splice(index, 1);
    }
    query.signal?.removeEventListener("abort", query.abort);
    const failure = query.failure ?? error;
    if (failure !== null) query.reject(failure);
    else if (query.token && query.terminalConsumed) query.resolve(query.token.identity);
    else query.reject(new Error("local-interaction.incomplete-close"));
    this.finishDisposal();
  }

  private receiveLocalInteractionQuery(reply: LocalInteractionQueryReply): void {
    const query = this.localQuery;
    if (!query) return;
    if (reply.kind === "rejected") {
      if (reply.requestId === query.requestId) this.finishLocalInteractionQuery(new Error(`local-interaction.${reply.code}`));
      return;
    }
    if (reply.kind === "started") {
      if (reply.token.requestId !== query.requestId || reply.token.identity.appInstanceId !== this.instanceId || query.token !== null) return;
      query.token = reply.token;
      if (query.cancelled) this.sendLocalInteractionQuery(query.cancelSequence, { kind: "cancel", token: query.token });
      return;
    }
    const token = reply.kind === "page" ? reply.page : reply.token;
    if (!query.token || !sameLocalInteractionQuery(query.token, token)) return;
    if (reply.kind === "closed") {
      if (!query.cancelled && (!query.terminalConsumed || token.ordinal !== query.token.ordinal)) return;
      this.finishLocalInteractionQuery(reply.cancelled ? new Error("local-interaction.cancelled") : null);
      return;
    }
    if (query.cancelled || query.consuming || token.ordinal !== query.nextOrdinal.toString()) return;
    query.token = { requestId: token.requestId, queryGeneration: token.queryGeneration, identity: token.identity, ordinal: token.ordinal };
    query.consuming = true;
    void Promise.resolve().then(() => query.consume(reply.page)).then(() => {
      if (this.localQuery !== query) return;
      query.consuming = false;
      if (query.cancelled) return;
      query.terminalConsumed = reply.page.terminal;
      query.nextOrdinal += 1n;
      let seq: number;
      try { seq = this.nextSeq(); }
      catch (error) { this.cancelLocalInteractionQuery(error); return; }
      this.sendLocalInteractionQuery(seq, { kind: "acknowledge", token: query.token! });
    }, (error: unknown) => {
      if (this.localQuery !== query) return;
      query.consuming = false;
      this.cancelLocalInteractionQuery(error);
    });
  }
  //#endregion 🏠️LocalInteractionQuery

  /** 🎛️ Forwards one opaque app-specific command (already encoded by the caller's own command
   * grammar) plus the current view state; may return several frames (`Invocation` + any dirtied
   * `UiPatch`es) — routing them is the caller's job. */
  async command(commandBytes: Uint8Array, viewState: unknown): Promise<AppFrameValue[]> {
    return this.sendCommand({
      Command: { seq: this.nextSeq(), command: Array.from(commandBytes), view_state: Array.from(encodePackValue(viewState)) },
    });
  }

  async configure(config: unknown): Promise<AppFrameValue[]> {
    return this.sendCommand({ ConfigCommand: { seq: this.nextSeq(), command: Array.from(encodePackValue(config)) } });
  }

  async readDocument(): Promise<AppFrameValue[]> {
    return this.sendCommand({ ReadDocument: { seq: this.nextSeq() } });
  }

  async loadDocument(pack: Uint8Array, spr: Uint8Array): Promise<AppFrameValue[]> {
    // 📦️ Cache from the call's own arguments too — the "both directions" half of the cache-population
    // contract, so a caller doesn't have to wait for an echoed `AppFrame::Document` to know what it
    // just loaded (and `plugin_exchange`'s `LoadDocument` handling is not guaranteed to echo one).
    this.cachedPack = pack;
    this.cachedSpr = spr;
    return this.sendCommand({ LoadDocument: { seq: this.nextSeq(), pack: Array.from(pack), spr: Array.from(spr) } });
  }

  /** 🧾️ Retrieves the complete history projection for initial load or cursor-gap recovery. */
  async readHistory(): Promise<AppFrameValue[]> {
    return this.sendCommand({ ReadHistory: { seq: this.nextSeq() } });
  }

  /** 📂️ Opens an artifact in its resolved (or explicitly named) viewer/editor surface —
   * `os.open-artifact` (contract-freeze §3 of
   * `.🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️16/ARTIFACT-VIEWERS-AND-EDITORS-PER-SUBSET/`). Empty
   * `pluginId`/`appId` means "resolve via the `OpeningResolver`". `role` is `0` Viewer, `1`
   * Editor — declaration order of `AppRole` (kernel `🔖️AppRouter` region). */
  async openArtifact(artifactRef: string, role: number, pluginId = "", appId = ""): Promise<AppFrameValue[]> {
    return this.sendCommand({ openArtifact: { seq: this.nextSeq(), artifact_ref: artifactRef, role, plugin_id: pluginId, app_id: appId } });
  }

  /** 🎚️ Pins a viewer/editor default for one `(artifactKind, standard, subset, role)` coordinate,
   * persisted event-sourced in the OS `os.config.opening` facet — `os.set-default-viewer`/
   * `os.set-default-editor`. */
  async setDefaultApp(artifactKind: string, standard: string, subset: string, role: number, pluginId: string, appId: string): Promise<AppFrameValue[]> {
    return this.sendCommand({ setDefaultApp: { seq: this.nextSeq(), artifact_kind: artifactKind, standard, subset, role, plugin_id: pluginId, app_id: appId } });
  }

  /** 🎚️ Clears a previously pinned default, falling back to the `OpeningResolver`'s owner/router
   * order — `os.clear-default-app`. */
  async clearDefaultApp(artifactKind: string, standard: string, subset: string, role: number): Promise<AppFrameValue[]> {
    return this.sendCommand({ clearDefaultApp: { seq: this.nextSeq(), artifact_kind: artifactKind, standard, subset, role } });
  }

  /** 🖱️ On-demand context menu — one `ContextMenu` reply whose `in_reply_to` matches this call's `seq`. */
  async contextMenu(request: unknown): Promise<unknown> {
    const seq = this.nextSeq();
    const frames = await this.sendCommand({
      ContextMenu: { seq, request: Array.from(encodePackValue(request)) },
    });
    const errorFrame = frames.find((frame): frame is Extract<AppFrameValue, { readonly Error: unknown }> => "Error" in frame);
    if (errorFrame) {
      throw new Error(`AppChannelClient.contextMenu(${this.appId}): ${faultDisplayMessage(errorFrame.Error.fault, decodePackValue)}`);
    }
    const menuFrame = frames.find(
      (frame): frame is Extract<AppFrameValue, { readonly ContextMenu: { readonly in_reply_to: number; readonly items: readonly number[] } }> =>
        "ContextMenu" in frame && frame.ContextMenu.in_reply_to === seq,
    );
    if (!menuFrame) {
      throw new Error(`AppChannelClient.contextMenu(${this.appId}): missing ContextMenu frame for seq ${seq}`);
    }
    return decodePackValue(new Uint8Array(menuFrame.ContextMenu.items));
  }

  /** @emoji 📥️ Force-applies remote `MutationEnvelope`s through `AppCommand::ApplyEnvelopes`. */
  async applyEnvelopes(envelopes: readonly MutationEnvelope[]): Promise<AppFrameValue[]> {
    return this.sendCommand({ ApplyEnvelopes: { seq: this.nextSeq(), envelopes } });
  }

  //#region 🔖️Merge
  /** ⚖️ Sets this instance's local merge-policy authority (`os.set-merge-policy`, C6/C9) — a `Done`
   * reply, never a `MergeReport`/`Conflicts` (those only follow an ingest). */
  async setMergePolicy(policy: MergePolicy): Promise<AppFrameValue[]> {
    return this.sendCommand({ setMergePolicy: { seq: this.nextSeq(), policy: mergePolicyAsU8(policy) } });
  }

  /** ⚔️ Accepts or discards an `Open` {@link Conflict} (`os.resolve-conflict`) — replays it under
   * `LaissezFaire` on `accept` (Quarantined) or acks it in place (Degraded); `discard` on a
   * Quarantined conflict seeds the DAG as already-seen without ever relaying it, on a Degraded
   * conflict it is rejected (never rewrites shared history, C6 §`resolve_conflict`). Returns the
   * authoritative `MergeReport` + `Conflicts` frames. */
  async resolveConflict(conflictId: string, resolution: ConflictResolution): Promise<AppFrameValue[]> {
    return this.sendCommand({ resolveConflict: { seq: this.nextSeq(), conflict_id: conflictId, resolution: conflictResolutionAsU8(resolution) } });
  }

  /** 📖️ Reads the open-conflict projection (`os.read-conflicts`) — one `Conflicts` reply frame. */
  async readConflicts(): Promise<AppFrameValue[]> {
    return this.sendCommand({ readConflicts: { seq: this.nextSeq() } });
  }
  //#endregion 🔖️Merge

  //#region 🔖️Presence
  /** 👥️ Pushes the document-wide presence roster into this instance's plugin app — the ONLY plugin
   * ingress for peers (contract-freeze §C7.6). Encodes each {@link ArtifactPresencePeer} via
   * {@link encodePresencePeer}; the caller has already dropped its own actor from `peers` before
   * calling this (`ownColor` carries this actor's own hub-assigned palette index separately, `null`
   * for a folder-only session with no hub). A plain `Done` reply, never decoded further here. */
  async pushPresence(ownColor: number | null, peers: readonly ArtifactPresencePeer[]): Promise<AppFrameValue[]> {
    return this.sendCommand({ presence: { seq: this.nextSeq(), own_color: ownColor, peers: peers.map((peer) => encodePresencePeer(peer)) } });
  }
  //#endregion 🔖️Presence

  //#region 🔖️Transaction
  /** 🎫️ `TransactionPrepare`, owner-mutation wire form (contract freeze §2/§5.3): `mutationId` +
   * `payload` set, `preparedOps` empty. Sent when `ArtifactMutationRouter` resolves the mutation to
   * its OWNING plugin. */
  async transactionPrepareOwner(txnId: string, mutationId: string, payload: Uint8Array): Promise<AppFrameValue[]> {
    return this.sendCommand({
      transactionPrepare: { seq: this.nextSeq(), txn_id: txnId, mutation_id: mutationId, payload: Array.from(payload), prepared_ops: [], label: "", origin: [] },
    });
  }

  /** 🎫️ `TransactionPrepare`, pre-planned wire form: `preparedOps`/`label`/`origin` set, `mutationId`
   * empty. Sent to a CONTRIBUTED-mutation target (after the host has already called the contributor's
   * `contributor.artifact-mutation-plan`) or to any member the coordinator is re-batching several
   * already-known ops onto in one call — see `PluginRuntime/🟦️component.tsx`'s `TransactionCoordinator`. */
  async transactionPreparePlanned(txnId: string, preparedOps: readonly Uint8Array[], label: string, origin: Uint8Array): Promise<AppFrameValue[]> {
    return this.sendCommand({
      transactionPrepare: {
        seq: this.nextSeq(),
        txn_id: txnId,
        mutation_id: "",
        payload: [],
        prepared_ops: preparedOps.map((op) => Array.from(op)),
        label,
        origin: Array.from(origin),
      },
    });
  }

  async transactionCommit(txnId: string): Promise<AppFrameValue[]> {
    return this.sendCommand({ transactionCommit: { seq: this.nextSeq(), txn_id: txnId } });
  }

  async transactionRollback(txnId: string): Promise<AppFrameValue[]> {
    return this.sendCommand({ transactionRollback: { seq: this.nextSeq(), txn_id: txnId } });
  }

  /** 🎁️ Group undo — fans out to every member of `groupId` (contract freeze §5.7); this call is one
   * member's half, the coordinator drives the fan-out. */
  async transactionUndo(groupId: string): Promise<AppFrameValue[]> {
    return this.sendCommand({ transactionUndo: { seq: this.nextSeq(), group_id: groupId } });
  }

  async transactionRedo(groupId: string): Promise<AppFrameValue[]> {
    return this.sendCommand({ transactionRedo: { seq: this.nextSeq(), group_id: groupId } });
  }
  //#endregion 🔖️Transaction
}
//#endregion 🔖️AppChannelClient

//#region 🧪️Tests
if (import.meta.vitest) {
  const { describe, expect, it } = import.meta.vitest;

  describe("@semio-tech/framework-os backbone", () => {
    it("classifies backbone uri kinds", () => {
      expect(backboneKindFromUri("file:///tmp/a.json")).toBe("file");
      expect(backboneKindFromUri("folder:///tmp")).toBe("folder");
      expect(backboneKindFromUri("remote://host:1234/doc-1")).toBe("remote");
      expect(backboneKindFromUri("other://x")).toBe("unknown");
    });

    it("builds and parses backbone uris", () => {
      expect(buildFileBackboneUri("tmp/a.json")).toBe("file:///tmp/a.json");
      expect(buildFolderBackboneUri("tmp")).toBe("folder:///tmp");
      expect(buildRemoteBackboneUri("localhost:1234", "studio-1", "doc-1")).toBe("remote://localhost:1234/studio-1/doc-1");
      expect(parseRemoteBackboneUri("remote://localhost:1234/studio-1/doc-1")).toEqual({ hostPort: "localhost:1234", spaceId: "studio-1", documentId: "doc-1" });
      expect(parseRemoteBackboneUri("remote://localhost:1234/doc-1")).toBeNull();
      expect(parseRemoteBackboneUri("file:///tmp/a.json")).toBeNull();
    });

    it("packs and unpacks document bundles", () => {
      const bundle = encodeDocumentPackBundle({ nodes: [] });
      expect(decodeDocumentPackSnapshot(bundle)).toEqual({ nodes: [] });
    });

    it("round-trips backbone snapshot messages", () => {
      const message: BinaryBackboneMessage = { kind: "snapshot", pack: new Uint8Array([1, 2]), spr: new Uint8Array([3]) };
      const round = decodeBackboneMessage(encodeBackboneMessage(message));
      expect(round.kind).toBe("snapshot");
      if (round.kind !== "snapshot") return;
      expect(Array.from(round.pack)).toEqual([1, 2]);
      expect(Array.from(round.spr)).toEqual([3]);
    });

    it("applies a snapshot backbone message by overwriting the stored bundle", () => {
      const snapshot = encodeBackboneMessage({ kind: "snapshot", pack: new Uint8Array([9]), spr: new Uint8Array() });
      const result = applyBackboneMessage(encodeDocumentPackBytes(new Uint8Array([1]), new Uint8Array()), snapshot);
      expect(decodeDocumentPackBytes(result).pack).toEqual(new Uint8Array([9]));
    });

    it("throws when applying operations without native store", () => {
      const message = encodeBackboneMessage({ kind: "mutations", envelopes: [] });
      expect(() => applyBackboneMessage(encodeDocumentPackBytes(new Uint8Array(), new Uint8Array()), message)).toThrow("native store");
    });

    it("throws when applying operations before a snapshot exists", () => {
      const message = encodeBackboneMessage({ kind: "mutations", envelopes: [] });
      expect(() => applyBackboneMessage(null, message)).toThrow("cannot append operations before a snapshot exists");
    });

    it("throws on an unknown backbone message tag", () => {
      expect(() => decodeBackboneMessage(new Uint8Array([99]))).toThrow("unknown tag");
    });

    it("builds sync utilities reflecting the active backbone kind", () => {
      const utilities = buildFrameworkSyncUtilities("folder:///tmp");
      expect(utilities.map((utility) => utility.id)).toEqual(["framework.sync.file", "framework.sync.folder", "framework.sync.remote"]);
      expect(utilities.find((utility) => utility.id === "framework.sync.folder")?.pressed).toBe(true);
      expect(utilities.find((utility) => utility.id === "framework.sync.file")?.pressed).toBe(false);
    });
  });

  describe("@semio-tech/framework-os workflow", () => {
    const mediaContract = (): MediaContract => ({ kindId: "2d.drawing", mediaType: { class: "data", form: "value" }, wire: { kind: "document", schema: "2d.drawing" } });
    const mediaNode = (id: string, instanceId: string): OsWorkflowNode => ({
      id,
      instanceId,
      x: 0,
      y: 0,
      width: 160,
      height: 72,
      inputs: [{ id: `${instanceId}:in`, artifactKind: "2d.drawing", direction: "in" }],
      outputs: [{ id: `${instanceId}:out`, artifactKind: "2d.drawing", direction: "out" }],
    });

    it("plans a single delivery across one dirty edge", () => {
      const graph: OsWorkflow = {
        schema: "os.workflow",
        nodes: [mediaNode("node-1", "app-1"), mediaNode("node-2", "app-2")],
        edges: [{ id: "edge-1", sourceNodeId: "node-1", sourcePortId: "app-1:out", targetNodeId: "node-2", targetPortId: "app-2:in", contract: mediaContract() }],
      };
      const deliveries = planWorkflow(graph, new Set(["app-1"]));
      expect(deliveries).toEqual([{ edgeId: "edge-1", producerInstanceId: "app-1", producerPortId: "app-1:out", consumerInstanceId: "app-2", consumerPortId: "app-2:in" }]);
    });

    it("plans a chain in topological order when only the root is dirty", () => {
      const graph: OsWorkflow = {
        schema: "os.workflow",
        nodes: [mediaNode("node-1", "app-1"), mediaNode("node-2", "app-2"), mediaNode("node-3", "app-3")],
        edges: [
          { id: "edge-ab", sourceNodeId: "node-1", sourcePortId: "app-1:out", targetNodeId: "node-2", targetPortId: "app-2:in", contract: mediaContract() },
          { id: "edge-bc", sourceNodeId: "node-2", sourcePortId: "app-2:out", targetNodeId: "node-3", targetPortId: "app-3:in", contract: mediaContract() },
        ],
      };
      const deliveries = planWorkflow(graph, new Set(["app-1"]));
      expect(deliveries.map((delivery) => delivery.edgeId)).toEqual(["edge-ab", "edge-bc"]);
    });

    it("plans a diamond with one delivery per incoming edge", () => {
      const graph: OsWorkflow = {
        schema: "os.workflow",
        nodes: [mediaNode("node-1", "app-a"), mediaNode("node-2", "app-b"), mediaNode("node-3", "app-c"), mediaNode("node-4", "app-d")],
        edges: [
          { id: "edge-ab", sourceNodeId: "node-1", sourcePortId: "app-a:out", targetNodeId: "node-2", targetPortId: "app-b:in", contract: mediaContract() },
          { id: "edge-ac", sourceNodeId: "node-1", sourcePortId: "app-a:out", targetNodeId: "node-3", targetPortId: "app-c:in", contract: mediaContract() },
          { id: "edge-bd", sourceNodeId: "node-2", sourcePortId: "app-b:out", targetNodeId: "node-4", targetPortId: "app-d:in", contract: mediaContract() },
          { id: "edge-cd", sourceNodeId: "node-3", sourcePortId: "app-c:out", targetNodeId: "node-4", targetPortId: "app-d:in", contract: mediaContract() },
        ],
      };
      const deliveries = planWorkflow(graph, new Set(["app-a"]));
      const edgeIds = deliveries.map((delivery) => delivery.edgeId);
      expect(edgeIds).toHaveLength(4);
      expect(edgeIds.indexOf("edge-bd")).toBeGreaterThan(edgeIds.indexOf("edge-ab"));
      expect(edgeIds.indexOf("edge-cd")).toBeGreaterThan(edgeIds.indexOf("edge-ac"));
    });

    it("plans nothing when no instance is dirty", () => {
      const graph: OsWorkflow = {
        schema: "os.workflow",
        nodes: [mediaNode("node-1", "app-1"), mediaNode("node-2", "app-2")],
        edges: [{ id: "edge-1", sourceNodeId: "node-1", sourcePortId: "app-1:out", targetNodeId: "node-2", targetPortId: "app-2:in", contract: mediaContract() }],
      };
      expect(planWorkflow(graph, new Set())).toEqual([]);
    });

    it("plans nothing for a dirty node with no outgoing edges", () => {
      const graph: OsWorkflow = { schema: "os.workflow", nodes: [mediaNode("node-1", "app-1")], edges: [] };
      expect(planWorkflow(graph, new Set(["app-1"]))).toEqual([]);
    });

    // 🔬️ Rust owns semantic DSL/SPK decoding and canonical equivalence. This language-neutral check
    // keeps the source corpus paired without depending on a browser ABI or a generated wasm package.
    it("pairs every shared workflow DSL fixture with a pack fixture", async () => {
      const { readdirSync, readFileSync } = await import("node:fs");
      const { fileURLToPath } = await import("node:url");
      const { dirname, join } = await import("node:path");
      const here = dirname(fileURLToPath(import.meta.url));
      const fixturesDir = join(here, "🧫️fixtures");
      const dslFiles = readdirSync(fixturesDir).filter((file) => file.endsWith(".dsl"));
      expect(dslFiles.length).toBeGreaterThanOrEqual(5);
      for (const dslFile of dslFiles) {
        expect(readFileSync(join(fixturesDir, dslFile), "utf8").length).toBeGreaterThan(0);
        const spkFile = dslFile.replace(/^🗣️?/, "📦️").replace(/\.dsl$/, ".spk");
        expect(readFileSync(join(fixturesDir, spkFile)).byteLength).toBeGreaterThan(0);
      }
    });
  });

  describe("@semio-tech/framework-os PackValueCodec", () => {
    function bytesToHex(bytes: Uint8Array): string {
      return Array.from(bytes)
        .map((byte) => byte.toString(16).padStart(2, "0"))
        .join("");
    }
    function hexToBytes(hex: string): Uint8Array {
      const out = new Uint8Array(hex.length / 2);
      for (let i = 0; i < out.length; i++) out[i] = Number.parseInt(hex.substring(i * 2, i * 2 + 2), 16);
      return out;
    }

    // 🔬️ Ground truth captured verbatim from `cargo test -p semio-framework-os-kernel-store
    // pack_wire_value_fixture_corpus_hex_dump -- --nocapture` (`store/rs/lib.rs`'s
    // `🔖️PackValueFixtures` region) — the REAL bytes `pack_rt::encode_wire_value` produces (the
    // `encode_record_body`-backed sibling of `encode_json_value`, see this file's
    // `🔖️PackValueCodec` header doc for why the container-backed encoding was replaced).
    // `encode_record_body`'s grammar has no compression anywhere it is fully deterministic, so
    // both `encodePackValue` and `decodePackValue` are asserted BYTE-EXACT against these, unlike
    // the old DEFLATE-backed encoding this replaced (which was only decode-exact).
    const packValueFixtures: ReadonlyArray<readonly [string, unknown, string]> = [
      ["null", null, "0001011112"],
      ["bool_true", true, "0001011102"],
      ["bool_false", false, "0001011101"],
      ["int_zero", 0, "00010111050000000000000000"],
      ["int_negative_one", -1, "0001011105000000000000f0bf"],
      ["float_pi", 3.14, "00010111051f85eb51b81e0940"],
      ["float_whole_number", 2.0, "00010111050000000000000040"],
      ["string_empty", "", "01000101110600"],
      ["string_escapes", 'hello\nworld with "quotes"', "011968656c6c6f0a776f726c642077697468202271756f746573220101110600"],
      ["array_empty", [], "000101110c00"],
      ["array_ints", [1, 2, 3], "000101110c0305000000000000f03f050000000000000040050000000000000840"],
      ["object_empty", {}, "000101111000"],
      ["object_mixed", { a: 1, b: [true, null] }, "00010111100207016105000000000000f03f0701620c020212"],
      [
        "nested_deep",
        { a: { b: { c: [1, 2, { d: "leaf" }] } } },
        "01046c6561660101111001070161100107016210010701630c0305000000000000f03f05000000000000004010010701640600",
      ],
    ];

    it.each(packValueFixtures)("decodes real Rust encode_wire_value bytes for %s", (_name, expected, hex) => {
      expect(decodePackValue(hexToBytes(hex))).toEqual(expected);
    });

    it.each(packValueFixtures)("encodes byte-exact against real Rust encode_wire_value output for %s", (_name, value, hex) => {
      expect(bytesToHex(encodePackValue(value))).toBe(hex);
    });

    it.each(packValueFixtures)("round-trips %s through encodePackValue/decodePackValue", (_name, value) => {
      expect(decodePackValue(encodePackValue(value))).toEqual(value);
    });
  });

  describe("@semio-tech/framework-os ScenePackCodec", () => {
    it("decodes the byte-exact Rust TableScene fixture without a schema-specific field mirror", () => {
      const hex = "0d02060b636f6c756d6e734a736f6e060f5b7b226964223a226e616d65227d5d0608726f77734a736f6e06025b5d";
      const bytes = new Uint8Array(hex.length / 2);
      for (let index = 0; index < bytes.length; index += 1) bytes[index] = Number.parseInt(hex.slice(index * 2, index * 2 + 2), 16);
      expect(decodeScenePackValue(bytes)).toEqual({ columnsJson: '[{"id":"name"}]', rowsJson: "[]" });
    });
  });

  describe("@semio-tech/framework-os AppChannelCodec", () => {
    it("round-trips the app-typed presence pack through the document-presence wire", () => {
      const peer: ArtifactPresencePeer = {
        actor: "actor-1",
        connectedAtMs: 42,
        label: "One",
        presencePack: [1, 2, 3],
        color: 4,
        surface: "s.space.home@1/*#editor",
        views: [{ windowId: "w1", space: "canvas", kind: { kind: "canvas", x: 1, y: 2, zoom: 1.5 }, size: [800, 600], pointer: [10, 20, 0] }],
        ui: { hoveredPath: "row[0]#a" },
      };
      expect(decodePresencePeer(new Uint8Array(encodePresencePeer(peer)), [0])).toEqual(peer);
    });

    const sampleCommands: readonly AppCommandValue[] = [
      { ConfigCommand: { seq: 1, command: [4, 5] } },
      { Command: { seq: 2, command: [1], view_state: [2, 3] } },
      { CommandText: { seq: 3, line: "move 1 2" } },
      { ContextMenu: { seq: 5, request: [9, 9] } },
      { ArtifactCommand: { seq: 6, command: [7] } },
      { ApplyEnvelopes: { seq: 7, envelopes: [] } },
      { LoadDocument: { seq: 8, pack: [1, 2, 3], spr: [4, 5, 6] } },
      { ReadDocument: { seq: 9 } },
      { LoadConfig: { seq: 10, pack: [1], spr: [2] } },
      { ReadConfig: { seq: 11 } },
      { MediaIn: { seq: 14, port: "in-1", descriptor: [1], data: [2, 3] } },
      { MediaOut: { seq: 15, port: "out-1", request: [4] } },
      { MediaFingerprint: { seq: 16, port: "fp-1" } },
      { PureCommand: { seq: 17, command: [1], document: [2], document_spr: [3], config: [4], config_spr: [5], draft: [6], draft_spr: [7] } },
      { LoadChildren: { seq: 18, entries: [{ slot: "s", child_id: "c", dialect: "d", envelope_pack: [1] }] } },
      { ReadChildren: { seq: 19 } },
      { ReadHistory: { seq: 20 } },
      { transactionPrepare: { seq: 21, txn_id: "txn-1", mutation_id: "s.demo#kind", payload: [1, 2], prepared_ops: [], label: "", origin: [] } },
      { transactionPrepare: { seq: 22, txn_id: "txn-1", mutation_id: "", payload: [], prepared_ops: [[1], [2, 2]], label: "step-1", origin: [9] } },
      { transactionCommit: { seq: 23, txn_id: "txn-1" } },
      { transactionRollback: { seq: 24, txn_id: "txn-1" } },
      { transactionUndo: { seq: 25, group_id: "grp-1" } },
      { transactionRedo: { seq: 26, group_id: "grp-1" } },
      { openArtifact: { seq: 27, artifact_ref: "s.cad.cad@1/*#viewer", role: 0, plugin_id: "", app_id: "" } },
      { openArtifact: { seq: 28, artifact_ref: "s.cad.cad@1/*#editor", role: 1, plugin_id: "cad", app_id: "s.cad.cad@1/*#editor" } },
      { setDefaultApp: { seq: 29, artifact_kind: "s.cad.cad", standard: "1", subset: "*", role: 1, plugin_id: "cad", app_id: "s.cad.cad@1/*#editor" } },
      { clearDefaultApp: { seq: 30, artifact_kind: "s.cad.cad", standard: "1", subset: "*", role: 0 } },
      { setMergePolicy: { seq: 31, policy: 1 } },
      { resolveConflict: { seq: 32, conflict_id: "conflict-1", resolution: 0 } },
      { readConflicts: { seq: 33 } },
      { presence: { seq: 34, own_color: 3, peers: [[1, 2], [9]] } },
      { presence: { seq: 35, own_color: null, peers: [] } },
    ];

    const sampleFrames: readonly AppFrameValue[] = [
      { Done: { in_reply_to: 1 } },
      { Invocation: { in_reply_to: 2, output: [1], diagnostics: [], ui_scope: [], history_patch: [], messages: [] } },
      { Invocation: { in_reply_to: 2, output: [1], diagnostics: [], ui_scope: [], history_patch: [], messages: [9] } },
      { DocumentChanged: { envelopes: [[1, 2]], origin: "remote" } },
      { Document: { in_reply_to: 6, pack: [1, 2], spr: [3, 4], ops: "op-log" } },
      { ContextMenu: { in_reply_to: 7, items: [1, 2, 3] } },
      { Media: { in_reply_to: 8, port: "out-1", descriptor: [1], data: [2] } },
      { MediaFingerprint: { in_reply_to: 9, port: "fp-1", fingerprint: [1, 2, 3, 4] } },
      { Error: { in_reply_to: 10, fault: [1, 2, 3], report: [6] } },
      { Error: { in_reply_to: null, fault: [4, 5], report: [] } },
      { Emit: { in_reply_to: 11, document_ops: [1], config_ops: [2], draft_ops: [3], output: [4], diagnostics: [5] } },
      { Draft: { in_reply_to: 12, pack: [1], spr: [2], ops: "d" } },
      { Children: { in_reply_to: 13, entries: [{ slot: "s", child_id: "c", dialect: "d", envelope_pack: [1] }] } },
      { Ephemeral: { presence: [1, 2], presence_generation: 3, transient_generation: 4, interaction: [7] } },
      { Ephemeral: { presence: [1, 2], presence_generation: 3, transient_generation: 4, interaction: [] } },
      { HistorySnapshot: { in_reply_to: 14, history_patch: [1] } },
      { transactionProposal: { in_reply_to: 15, proposal_id: "prop-1", local_ops: [[1]], description: "move", coalesce_key: "k-1", foreign: [[2, 3]] } },
      { transactionPrepared: { txn_id: "txn-1", foreign: [[1]], rejection: [] } },
      { transactionPrepared: { txn_id: "txn-1", foreign: [], rejection: [1, 2] } },
      { transactionCommitted: { txn_id: "txn-1", edit_id: "edit-1" } },
      { transactionRolledBack: { txn_id: "txn-1" } },
      { MergeReport: { in_reply_to: 16, report: [1, 2] } },
      { MergeReport: { in_reply_to: null, report: [] } },
      { Conflicts: { in_reply_to: 17, conflicts: [3] } },
      { Conflicts: { in_reply_to: null, conflicts: [] } },
      { UiPatch: { in_reply_to: 18, surface: "1:body", kind: "window", revision: 2, base_revision: 1, ops: [3] } },
      { UiPatch: { in_reply_to: null, surface: "1:body", kind: "window", revision: 1, base_revision: 0, ops: [] } },
      { UiSnapshotEnd: { revision: 5 } },
    ];

    it.each(sampleCommands.map((cmd) => [cmd] as const))("round-trips AppCommand %j", (cmd) => {
      expect(decodeAppCommand(encodeAppCommand(cmd))).toEqual(cmd);
    });

    it.each(sampleFrames.map((frame) => [frame] as const))("round-trips AppFrame %j", (frame) => {
      expect(decodeAppFrame(encodeAppFrame(frame))).toEqual(frame);
    });

    it("tags every AppCommand variant per the agreed contract order (ConfigCommand=0 ... presence=28)", () => {
      expect(encodeAppCommand({ ConfigCommand: { seq: 0, command: [] } })[0]).toBe(0);
      expect(encodeAppCommand({ Command: { seq: 0, command: [], view_state: [] } })[0]).toBe(1);
      expect(encodeAppCommand({ ReadChildren: { seq: 0 } })[0]).toBe(15);
      expect(encodeAppCommand({ ReadHistory: { seq: 0 } })[0]).toBe(16);
      expect(encodeAppCommand({ transactionPrepare: { seq: 0, txn_id: "", mutation_id: "", payload: [], prepared_ops: [], label: "", origin: [] } })[0]).toBe(17);
      expect(encodeAppCommand({ transactionCommit: { seq: 0, txn_id: "" } })[0]).toBe(18);
      expect(encodeAppCommand({ transactionRollback: { seq: 0, txn_id: "" } })[0]).toBe(19);
      expect(encodeAppCommand({ transactionUndo: { seq: 0, group_id: "" } })[0]).toBe(20);
      expect(encodeAppCommand({ transactionRedo: { seq: 0, group_id: "" } })[0]).toBe(21);
      expect(encodeAppCommand({ openArtifact: { seq: 0, artifact_ref: "", role: 0, plugin_id: "", app_id: "" } })[0]).toBe(22);
      expect(encodeAppCommand({ setDefaultApp: { seq: 0, artifact_kind: "", standard: "", subset: "", role: 0, plugin_id: "", app_id: "" } })[0]).toBe(23);
      expect(encodeAppCommand({ clearDefaultApp: { seq: 0, artifact_kind: "", standard: "", subset: "", role: 0 } })[0]).toBe(24);
      expect(encodeAppCommand({ setMergePolicy: { seq: 0, policy: 0 } })[0]).toBe(25);
      expect(encodeAppCommand({ resolveConflict: { seq: 0, conflict_id: "", resolution: 0 } })[0]).toBe(26);
      expect(encodeAppCommand({ readConflicts: { seq: 0 } })[0]).toBe(27);
      expect(encodeAppCommand({ presence: { seq: 0, own_color: null, peers: [] } })[0]).toBe(28);
    });

    it("tags every AppFrame variant per the agreed contract order (Done=0 ... UiSnapshotEnd=22)", () => {
      expect(encodeAppFrame({ Done: { in_reply_to: 0 } })[0]).toBe(0);
      expect(encodeAppFrame({ Invocation: { in_reply_to: 0, output: [], diagnostics: [], ui_scope: [], history_patch: [], messages: [] } })[0]).toBe(1);
      expect(encodeAppFrame({ Error: { in_reply_to: null, fault: [], report: [] } })[0]).toBe(9);
      expect(encodeAppFrame({ Ephemeral: { presence: [], presence_generation: 0, transient_generation: 0, interaction: [] } })[0]).toBe(13);
      expect(encodeAppFrame({ HistorySnapshot: { in_reply_to: 0, history_patch: [] } })[0]).toBe(14);
      expect(encodeAppFrame({ transactionProposal: { in_reply_to: 0, proposal_id: "", local_ops: [], description: "", coalesce_key: "", foreign: [] } })[0]).toBe(15);
      expect(encodeAppFrame({ transactionPrepared: { txn_id: "", foreign: [], rejection: [] } })[0]).toBe(16);
      expect(encodeAppFrame({ transactionCommitted: { txn_id: "", edit_id: "" } })[0]).toBe(17);
      expect(encodeAppFrame({ transactionRolledBack: { txn_id: "" } })[0]).toBe(18);
      expect(encodeAppFrame({ MergeReport: { in_reply_to: null, report: [] } })[0]).toBe(19);
      expect(encodeAppFrame({ Conflicts: { in_reply_to: null, conflicts: [] } })[0]).toBe(20);
      expect(encodeAppFrame({ UiPatch: { in_reply_to: null, surface: "", kind: "", revision: 0, base_revision: 0, ops: [] } })[0]).toBe(21);
      expect(encodeAppFrame({ UiSnapshotEnd: { revision: 0 } })[0]).toBe(22);
    });

    /**
     * 🔒️ Cross-language drift guard: the exact same fixture values and golden hex committed in
     * `protocol_channel`'s own `🔖️Corpus` region (`🔨️modules/📡️protocol/🧵️channel/📦️packages/🦀️rust/📦️lib.rs`,
     * `channel_command_fixture_corpus`/`channel_command_fixture_hex` and their `AppFrame` twins) —
     * sourced by running the real `encode_app_command`/`encode_app_frame` and copying their
     * printed `[DEBUG] AppCommand::<label> = <hex>` output (`cargo test -p semio-protocol-channel
     * -- --nocapture`), NOT hand-computed. Any future change to either codec that shifts these
     * bytes fails on exactly one side, forcing a deliberate update of both this table and the Rust
     * golden hex in the same change.
     */
    it("matches protocol_channel's own golden hex fixture corpus, byte-for-byte", () => {
            const commandFixtures: readonly (readonly [string, AppCommandValue])[] = [
        ["ConfigCommand", { ConfigCommand: { seq: 1, command: [9] } }],
        ["Command", { Command: { seq: 1, command: [1], view_state: [] } }],
        ["CommandText", { CommandText: { seq: 1, line: "go" } }],
        ["ContextMenu", { ContextMenu: { seq: 1, request: [1] } }],
        ["ArtifactCommand", { ArtifactCommand: { seq: 1, command: [1] } }],
        ["ApplyEnvelopes", { ApplyEnvelopes: { seq: 1, envelopes: [] } }],
        ["LoadDocument", { LoadDocument: { seq: 1, pack: [1], spr: [2] } }],
        ["ReadDocument", { ReadDocument: { seq: 1 } }],
        ["LoadConfig", { LoadConfig: { seq: 1, pack: [1], spr: [2] } }],
        ["ReadConfig", { ReadConfig: { seq: 1 } }],
        ["MediaIn", { MediaIn: { seq: 1, port: "p", descriptor: [1], data: [2] } }],
        ["MediaOut", { MediaOut: { seq: 1, port: "p", request: [1] } }],
        ["MediaFingerprint", { MediaFingerprint: { seq: 1, port: "p" } }],
        ["PureCommand", { PureCommand: { seq: 1, command: [1], document: [2], document_spr: [3], config: [4], config_spr: [5], draft: [6], draft_spr: [7] } }],
        ["LoadChildren", { LoadChildren: { seq: 1, entries: [{ slot: "s", child_id: "c", dialect: "d", envelope_pack: [1] }] } }],
        ["ReadChildren", { ReadChildren: { seq: 1 } }],
        ["ReadHistory", { ReadHistory: { seq: 1 } }],
      ];
            const commandGoldenHex: Readonly<Record<string, string>> = {
        ConfigCommand: "00010109",
        Command: "0101010100",
        CommandText: "020102676f",
        ContextMenu: "03010101",
        ArtifactCommand: "04010101",
        ApplyEnvelopes: "050100",
        LoadDocument: "060101010102",
        ReadDocument: "0701",
        LoadConfig: "080101010102",
        ReadConfig: "0901",
        MediaIn: "0a01017001010102",
        MediaOut: "0b0101700101",
        MediaFingerprint: "0c010170",
        PureCommand: "0d010101010201030104010501060107",
        LoadChildren: "0e01010173016301640101",
        ReadChildren: "0f01",
        ReadHistory: "1001",
      };
            const frameFixtures: readonly (readonly [string, AppFrameValue])[] = [
        ["Done", { Done: { in_reply_to: 1 } }],
        ["Invocation", { Invocation: { in_reply_to: 1, output: [1], diagnostics: [], ui_scope: [], history_patch: [], messages: [] } }],
        ["DocumentChanged", { DocumentChanged: { envelopes: [], origin: "o" } }],
        ["Document", { Document: { in_reply_to: 1, pack: [1], spr: [2], ops: "o" } }],
        ["Config", { Config: { in_reply_to: 1, pack: [1], spr: [2], ops: "c" } }],
        ["ConfigChanged", { ConfigChanged: { envelopes: [], origin: "o" } }],
        ["ContextMenu", { ContextMenu: { in_reply_to: 1, items: [1] } }],
        ["Media", { Media: { in_reply_to: 1, port: "p", descriptor: [1], data: [2] } }],
        ["MediaFingerprint", { MediaFingerprint: { in_reply_to: 1, port: "p", fingerprint: [1] } }],
        ["Error", { Error: { in_reply_to: null, fault: [99], report: [] } }],
        ["Emit", { Emit: { in_reply_to: 1, document_ops: [1], config_ops: [], draft_ops: [], output: [2], diagnostics: [] } }],
        ["Draft", { Draft: { in_reply_to: 1, pack: [1], spr: [2], ops: "d" } }],
        ["Children", { Children: { in_reply_to: 1, entries: [{ slot: "s", child_id: "c", dialect: "d", envelope_pack: [1] }] } }],
        ["Ephemeral", { Ephemeral: { presence: [1, 2], presence_generation: 3, transient_generation: 4, interaction: [] } }],
        ["HistorySnapshot", { HistorySnapshot: { in_reply_to: 1, history_patch: [1] } }],
        ["UiPatch", { UiPatch: { in_reply_to: 1, surface: "1:body", kind: "window", revision: 3, base_revision: 2, ops: [9] } }],
        ["UiSnapshotEnd", { UiSnapshotEnd: { revision: 6 } }],
      ];
            const frameGoldenHex: Readonly<Record<string, string>> = {
        Done: "0001",
        Invocation: "0101010100000000",
        DocumentChanged: "0200016f",
        Document: "030101010102016f",
        Config: "0401010101020163",
        ConfigChanged: "0500016f",
        ContextMenu: "06010101",
        Media: "0701017001010102",
        MediaFingerprint: "080101700101",
        Error: "0900016300",
        Emit: "0a0101010000010200",
        Draft: "0b01010101020164",
        Children: "0c01010173016301640101",
        Ephemeral: "0d020102030400",
        HistorySnapshot: "0e010101",
        UiPatch: "15010106313a626f64790677696e646f7703020109",
        UiSnapshotEnd: "1606",
      };
      const hex = (bytes: Uint8Array) => Array.from(bytes, (b) => b.toString(16).padStart(2, "0")).join("");
      for (const [label, value] of commandFixtures) expect(hex(encodeAppCommand(value)), `AppCommand::${label}`).toBe(commandGoldenHex[label]);
      for (const [label, value] of frameFixtures) expect(hex(encodeAppFrame(value)), `AppFrame::${label}`).toBe(frameGoldenHex[label]);
    });

    /**
     * 🔗️ Cross-language drift guard for the M2 transaction variants (tags 22-26/19-22): both this
     * suite and `protocol_channel`'s `channel_transaction_fixtures_match_shared_cross_language_json_vectors`
     * Rust test load the SAME two JSON files under `🧫️fixtures/📡️channel/` — there is exactly one
     * committed hex string per label, not a copy per language, so a codec change that shifts these
     * bytes on either side fails here or there, never silently in both at once.
     */
    it("pins APP_CHANNEL_VERSION against the shared cross-language channel version", async () => {
      const { readFileSync } = await import("node:fs");
      const { fileURLToPath } = await import("node:url");
      const { dirname, join } = await import("node:path");
      const here = dirname(fileURLToPath(import.meta.url));
      const pin = JSON.parse(readFileSync(join(here, "🧫️fixtures", "📡️channel", "channel-version.json"), "utf8")) as { channelVersion: number };
      expect(APP_CHANNEL_VERSION).toBe(pin.channelVersion);
    });

    it("matches the shared cross-language transaction fixture vectors, byte-for-byte", async () => {
      const { readFileSync } = await import("node:fs");
      const { fileURLToPath } = await import("node:url");
      const { dirname, join } = await import("node:path");
      const here = dirname(fileURLToPath(import.meta.url));
      const channelFixturesDir = join(here, "🧫️fixtures", "📡️channel");
      const commandVectors = JSON.parse(readFileSync(join(channelFixturesDir, "app-command-transaction.json"), "utf8")) as Record<string, string>;
      const frameVectors = JSON.parse(readFileSync(join(channelFixturesDir, "app-frame-transaction.json"), "utf8")) as Record<string, string>;
      const hex = (bytes: Uint8Array) => Array.from(bytes, (b) => b.toString(16).padStart(2, "0")).join("");

      const commandCases: Readonly<Record<string, AppCommandValue>> = {
        TransactionPrepareOwner: { transactionPrepare: { seq: 1, txn_id: "t", mutation_id: "m", payload: [9], prepared_ops: [], label: "", origin: [] } },
        TransactionPreparePrePlanned: { transactionPrepare: { seq: 2, txn_id: "t", mutation_id: "", payload: [], prepared_ops: [[1], [2, 2]], label: "l", origin: [9] } },
        TransactionCommit: { transactionCommit: { seq: 3, txn_id: "t" } },
        TransactionRollback: { transactionRollback: { seq: 4, txn_id: "t" } },
        TransactionUndo: { transactionUndo: { seq: 5, group_id: "g" } },
        TransactionRedo: { transactionRedo: { seq: 6, group_id: "g" } },
      };
      const frameCases: Readonly<Record<string, AppFrameValue>> = {
        TransactionProposal: { transactionProposal: { in_reply_to: 1, proposal_id: "p", local_ops: [[1]], description: "d", coalesce_key: "k", foreign: [] } },
        TransactionPrepared: { transactionPrepared: { txn_id: "t", foreign: [[1]], rejection: [] } },
        TransactionCommitted: { transactionCommitted: { txn_id: "t", edit_id: "e" } },
        TransactionRolledBack: { transactionRolledBack: { txn_id: "t" } },
      };

      expect(Object.keys(commandVectors).sort()).toEqual(Object.keys(commandCases).sort());
      expect(Object.keys(frameVectors).sort()).toEqual(Object.keys(frameCases).sort());
      for (const [label, value] of Object.entries(commandCases)) {
        expect(hex(encodeAppCommand(value)), `AppCommand::${label}`).toBe(commandVectors[label]);
        expect(decodeAppCommand(new Uint8Array(Buffer.from(commandVectors[label]!, "hex")))).toEqual(value);
      }
      for (const [label, value] of Object.entries(frameCases)) {
        expect(hex(encodeAppFrame(value)), `AppFrame::${label}`).toBe(frameVectors[label]);
        expect(decodeAppFrame(new Uint8Array(Buffer.from(frameVectors[label]!, "hex")))).toEqual(value);
      }
    });

    /**
     * 🔗️ Cross-language drift guard for the C3 opening variants (tags 27-29): both this suite and
     * `protocol_channel`'s `channel_opening_fixtures_match_shared_cross_language_json_vectors` Rust
     * test load the SAME JSON file under `🧫️fixtures/📡️channel/` — no `AppFrame` variants were added
     * for opening, so only the command-side vector file exists.
     */
    it("matches the shared cross-language opening fixture vectors, byte-for-byte", async () => {
      const { readFileSync } = await import("node:fs");
      const { fileURLToPath } = await import("node:url");
      const { dirname, join } = await import("node:path");
      const here = dirname(fileURLToPath(import.meta.url));
      const channelFixturesDir = join(here, "🧫️fixtures", "📡️channel");
      const commandVectors = JSON.parse(readFileSync(join(channelFixturesDir, "app-command-opening.json"), "utf8")) as Record<string, string>;
      const hex = (bytes: Uint8Array) => Array.from(bytes, (b) => b.toString(16).padStart(2, "0")).join("");

      const commandCases: Readonly<Record<string, AppCommandValue>> = {
        OpenArtifactResolve: { openArtifact: { seq: 1, artifact_ref: "s.cad.cad@1/*#viewer", role: 0, plugin_id: "", app_id: "" } },
        OpenArtifactExplicit: { openArtifact: { seq: 2, artifact_ref: "s.cad.cad@1/*#editor", role: 1, plugin_id: "cad", app_id: "s.cad.cad@1/*#editor" } },
        SetDefaultApp: { setDefaultApp: { seq: 3, artifact_kind: "s.cad.cad", standard: "1", subset: "*", role: 1, plugin_id: "cad", app_id: "s.cad.cad@1/*#editor" } },
        ClearDefaultApp: { clearDefaultApp: { seq: 4, artifact_kind: "s.cad.cad", standard: "1", subset: "*", role: 0 } },
      };

      expect(Object.keys(commandVectors).sort()).toEqual(Object.keys(commandCases).sort());
      for (const [label, value] of Object.entries(commandCases)) {
        expect(hex(encodeAppCommand(value)), `AppCommand::${label}`).toBe(commandVectors[label]);
        expect(decodeAppCommand(new Uint8Array(Buffer.from(commandVectors[label]!, "hex")))).toEqual(value);
      }
    });

    /**
     * 🔗️ Cross-language drift guard for the C8 merge-policy/conflict variants (tags 30-32/23-24)
     * plus the extended `Invocation`/`Error` frames: both this suite and `protocol_channel`'s
     * `channel_merge_fixtures_match_shared_cross_language_json_vectors` Rust test load the SAME two
     * JSON files under `🧫️fixtures/📡️channel/` — see contract-freeze.md §C8 of
     * `.🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️16/MUTATION-OUTCOMES-MERGE-POLICIES-AND-FIRST-CLASS-CONFLICTS/`.
     */
    it("matches the shared cross-language merge fixture vectors, byte-for-byte", async () => {
      const { readFileSync } = await import("node:fs");
      const { fileURLToPath } = await import("node:url");
      const { dirname, join } = await import("node:path");
      const here = dirname(fileURLToPath(import.meta.url));
      const channelFixturesDir = join(here, "🧫️fixtures", "📡️channel");
      const commandVectors = JSON.parse(readFileSync(join(channelFixturesDir, "app-command-merge.json"), "utf8")) as Record<string, string>;
      const frameVectors = JSON.parse(readFileSync(join(channelFixturesDir, "app-frame-merge.json"), "utf8")) as Record<string, string>;
      const hex = (bytes: Uint8Array) => Array.from(bytes, (b) => b.toString(16).padStart(2, "0")).join("");

      const commandCases: Readonly<Record<string, AppCommandValue>> = {
        SetMergePolicy: { setMergePolicy: { seq: 5, policy: 1 } },
        ResolveConflict: { resolveConflict: { seq: 6, conflict_id: "conflict-1", resolution: 0 } },
        ReadConflicts: { readConflicts: { seq: 7 } },
      };
      const frameCases: Readonly<Record<string, AppFrameValue>> = {
        MergeReport: { MergeReport: { in_reply_to: 1, report: [1] } },
        Conflicts: { Conflicts: { in_reply_to: null, conflicts: [2] } },
        Invocation: { Invocation: { in_reply_to: 1, output: [1], diagnostics: [], ui_scope: [], history_patch: [], messages: [9] } },
        Error: { Error: { in_reply_to: null, fault: [99], report: [7] } },
      };

      expect(Object.keys(commandVectors).sort()).toEqual(Object.keys(commandCases).sort());
      expect(Object.keys(frameVectors).sort()).toEqual(Object.keys(frameCases).sort());
      for (const [label, value] of Object.entries(commandCases)) {
        expect(hex(encodeAppCommand(value)), `AppCommand::${label}`).toBe(commandVectors[label]);
        expect(decodeAppCommand(new Uint8Array(Buffer.from(commandVectors[label]!, "hex")))).toEqual(value);
      }
      for (const [label, value] of Object.entries(frameCases)) {
        expect(hex(encodeAppFrame(value)), `AppFrame::${label}`).toBe(frameVectors[label]);
        expect(decodeAppFrame(new Uint8Array(Buffer.from(frameVectors[label]!, "hex")))).toEqual(value);
      }
    });
  });

  describe("@semio-tech/framework-os AppChannelClient", () => {
    it("local interaction outer wire matches strict fixtures and the independent LEB128 oracle", async () => {
      const { readFileSync } = await import("node:fs");
      const { default: Ajv } = await import("ajv");
      const oracleModule = "@webassemblyjs/leb128/lib/leb.js";
      const imported: unknown = await import(oracleModule);
      if (!imported || typeof imported !== "object") throw new Error("invalid LEB128 oracle module");
      const oracle: unknown = Reflect.get(imported, "default");
      if (!oracle || typeof oracle !== "object") throw new Error("invalid LEB128 oracle interface");
      const encodeUnsigned: unknown = Reflect.get(oracle, "encodeUIntBuffer");
      if (typeof encodeUnsigned !== "function") throw new Error("missing LEB128 oracle encoder");
      const fixture = JSON.parse(readFileSync(new URL("./🧫️fixtures/🏠️local-interaction/🔣️query.json", import.meta.url), "utf8"));
      const schema = JSON.parse(readFileSync(new URL("./🧫️fixtures/🏠️local-interaction/🧬️schema.json", import.meta.url), "utf8"));
      const validate = new Ajv({ strict: true }).compile(schema);
      expect(validate(fixture)).toBe(true);
      expect(validate({ ...fixture, lateTokenAccepted: true })).toBe(false);
      expect(validate({ ...fixture, terminalBeforeClosed: true })).toBe(false);
      const read: AppCommandValue = { LocalInteractionQuery: { seq: 9, command: { kind: "read", requestId: "13" } } };
      const rejected: AppFrameValue = { LocalInteractionQuery: { reply: { kind: "rejected", requestId: "13", code: "busy" } } };
      const u64 = (value: bigint): number[] => {
        const bytes = Buffer.alloc(8); bytes.writeBigUInt64LE(value);
        const encoded: unknown = encodeUnsigned(bytes);
        if (!(encoded instanceof Uint8Array)) throw new Error("invalid LEB128 oracle bytes");
        return Array.from(encoded);
      };
      expect([...encodeAppCommand(read)]).toEqual([29, ...u64(9n), ...u64(2n), 0, ...u64(13n)]);
      expect(Buffer.from(encodeAppCommand(read)).toString("hex")).toBe(fixture.outerReadHex);
      expect(Buffer.from(encodeAppFrame(rejected)).toString("hex")).toBe(fixture.outerRejectedHex);
      const receipt: AppFrameValue = { Done: { in_reply_to: 2 } };
      expect([...encodeAppFrame(receipt)]).toEqual([0, ...u64(2n)]);
      expect(Buffer.from(encodeAppFrame(receipt)).toString("hex")).toBe(fixture.receiptHex);
      for (const row of fixture.sequenceCases) {
        if (row.result === null) continue;
        const sequence = row.result.sequence;
        expect([...encodeAppCommand({ ReadDocument: { seq: sequence } })]).toEqual([7, ...u64(BigInt(sequence))]);
        if (row.result.request !== null) {
          const inner = [0, ...u64(BigInt(row.result.request))];
          expect([...encodeAppCommand({ LocalInteractionQuery: { seq: sequence, command: { kind: "read", requestId: row.result.request } } })]).toEqual([29, ...u64(BigInt(sequence)), ...u64(BigInt(inner.length)), ...inner]);
        }
      }
      expect(decodeAppCommand(encodeAppCommand(read))).toEqual(read);
      expect(decodeAppFrame(encodeAppFrame(rejected))).toEqual(rejected);
      expect(() => decodeAppCommand(Uint8Array.from([...encodeAppCommand(read), 0]))).toThrow();
      expect(() => decodeAppFrame(Uint8Array.from([...encodeAppFrame(rejected), 0]))).toThrow();
    });

    it("local interaction client fixture lifecycles preserve ACK ownership and ordinary replies", async () => {
      const { readFileSync } = await import("node:fs");
      const fixture = JSON.parse(readFileSync(new URL("./🧫️fixtures/🏠️local-interaction/🔣️query.json", import.meta.url), "utf8"));
      for (const row of fixture.lifecycles) {
        const broadcast = createTurnOutcomeBroadcast<TurnOutcome>();
        const sent: AppCommandValue[] = [];
        const handle: AppChannelHandle = { enqueue: (_id, events) => {
          for (const command of events.map(decodeAppCommand)) {
            sent.push(command);
            if ("LocalInteractionQuery" in command && !(["disposal-before-read-receipt", "coalesced-query-and-ordinary-receipts"].includes(row.id) && command.LocalInteractionQuery.command.kind === "read")) broadcast.push({ instanceId: 7, frames: [encodeAppFrame({ Done: { in_reply_to: command.LocalInteractionQuery.seq } })] });
          }
        }, outcomes: broadcast.stream };
        const client = new AppChannelClient(handle, new AppChannelRequestSequence(), 7, "fixture");
        const abort = new AbortController();
        let consume!: () => void;
        const consumed = new Promise<void>((resolve) => { consume = resolve; });
        let complete = false;
        let queryResult: Promise<LocalInteractionIdentity | unknown> | undefined;
        let ordinary: Promise<AppFrameValue[]> | undefined;
        let ordinaryComplete = false;
        const token: LocalInteractionQueryToken = { requestId: "1", queryGeneration: "41", identity: { appInstanceId: 7, generation: "9007199254740993", revision: "11".repeat(32), documentRevision: "22".repeat(32), topologyRevision: "33".repeat(32) }, ordinal: "0" };
        const push = (reply: LocalInteractionQueryReply) => broadcast.push({ instanceId: 7, frames: [encodeAppFrame({ LocalInteractionQuery: { reply } })] });
        const flush = async () => { for (let index = 0; index < 12; index += 1) await Promise.resolve(); };
        for (const event of row.events) {
          if (event === "read") queryResult = client.readLocalInteractionPages(() => {
            if (row.id === "synchronous-consumer-failure") throw new Error("synchronous consumer fixture");
            if (row.id === "consumer-failure") return Promise.reject(new Error("consumer fixture"));
            return consumed;
          }, abort.signal).then((identity) => { complete = true; return identity; }, (error: unknown) => { complete = true; return error; });
          else if (event === "ordinary") ordinary = client.readDocument().then((frames) => { ordinaryComplete = true; return frames; });
          else if (event === "started") push({ kind: "started", token });
          else if (event === "abort") abort.abort();
          else if (event === "dispose") client.dispose();
          else if (event === "malformed") broadcast.push({ instanceId: 7, frames: [Uint8Array.from(Buffer.from(fixture.malformedFrameHex, "hex"))] });
          else if (event === "readReceipt") broadcast.push({ instanceId: 7, frames: [encodeAppFrame({ Done: { in_reply_to: 1 } })] });
          else if (event === "wrongReceipt" || event === "duplicateReadReceipt") {
            broadcast.push({ instanceId: 7, frames: [encodeAppFrame({ Done: { in_reply_to: event === "wrongReceipt" ? 999 : 1 } })] });
            await flush();
            expect(ordinaryComplete).toBe(false);
          }
          else if (event === "ordinaryDone") { broadcast.push({ instanceId: 7, frames: [encodeAppFrame({ Done: { in_reply_to: 3 } })] }); expect(await ordinary).toEqual([{ Done: { in_reply_to: 3 } }]); }
          else if (event === "ordinaryEmpty") { broadcast.push({ instanceId: 7, frames: [] }); await flush(); expect(ordinaryComplete).toBe(false); }
          else if (event === "mixedReceipts") {
            broadcast.push({ instanceId: 7, frames: [encodeAppFrame({ Done: { in_reply_to: 1 } }), encodeAppFrame({ Done: { in_reply_to: 3 } })] });
            await flush(); expect(ordinaryComplete).toBe(true);
            expect(await ordinary).toEqual([{ Done: { in_reply_to: 3 } }]);
          }
          else if (event === "startedWithNotice" || event === "pageWithNotice") {
            const reply: LocalInteractionQueryReply = event === "startedWithNotice" ? { kind: "started", token } : { kind: "page", page: { ...token, terminal: true, bytes: [0xe2, 0x9c, 0x93] } };
            broadcast.push({ instanceId: 7, frames: [encodeAppFrame({ LocalInteractionQuery: { reply } }), encodeAppFrame({ Ephemeral: { presence: [], presence_generation: 0, transient_generation: 0, interaction: [] } }), encodeAppFrame({ UiPatch: { in_reply_to: null, surface: "fixture", kind: "graph", revision: 1, base_revision: 0, ops: [] } })] });
            await flush(); expect(ordinaryComplete).toBe(false);
          }
          else if (event === "page") push({ kind: "page", page: { ...token, terminal: true, bytes: [0xe2, 0x9c, 0x93] } });
          else if (event === "consume") consume();
          else if (event === "consumeError") await flush();
          else if (event === "ack" || event === "cancel") {
            await flush();
            const last = sent.at(-1);
            expect(last && "LocalInteractionQuery" in last ? last.LocalInteractionQuery.command : null).toEqual({ kind: event === "ack" ? "acknowledge" : "cancel", token });
          } else if (event === "closed") {
            expect(complete).toBe(false);
            push({ kind: "closed", token, cancelled: row.cancelled });
          }
          await flush();
        }
        const result = await queryResult;
        expect(complete).toBe(true);
        if (!row.cancelled) expect(result).toEqual(token.identity);
        else expect(result).toBeInstanceOf(Error);
        client.dispose();
      }
    });
    it("local interaction sequence admission matches the checked shared-owner fixture", async () => {
      const { readFileSync } = await import("node:fs");
      const fixture = JSON.parse(readFileSync(new URL("./🧫️fixtures/🏠️local-interaction/🔣️query.json", import.meta.url), "utf8"));
      for (const row of fixture.sequenceCases) {
        const owner = new AppChannelRequestSequence(row.sequence, BigInt(row.request));
        const allocate = () => row.operation === "query" ? owner.nextQuery() : { sequence: owner.nextSequence(), cancelSequence: null, request: null };
        if (row.result === null) expect(allocate).toThrow();
        else expect(allocate()).toEqual(row.result);
        expect(owner.checkpoint()).toEqual(row.after);
      }
      for (const invalid of [-1, Number.MAX_SAFE_INTEGER + 1, NaN, 0.5]) expect(() => new AppChannelRequestSequence(invalid)).toThrow();
      for (const invalid of [-1n, 0x1_0000_0000_0000_0000n]) expect(() => new AppChannelRequestSequence(0, invalid)).toThrow();
    });

    it("local interaction reopened clients reject delayed Started pages and ordinary receipts", async () => {
      const { readFileSync } = await import("node:fs");
      const fixture = JSON.parse(readFileSync(new URL("./🧫️fixtures/🏠️local-interaction/🔣️query.json", import.meta.url), "utf8")).reopen;
      const owner = new AppChannelRequestSequence();
      const broadcast = createTurnOutcomeBroadcast<TurnOutcome>();
      const sent: AppCommandValue[] = [];
      const handle: AppChannelHandle = { enqueue: (_instance, bytes) => {
        for (const command of bytes.map(decodeAppCommand)) {
          sent.push(command);
          if ("LocalInteractionQuery" in command) broadcast.push({ instanceId: 7, frames: [encodeAppFrame({ Done: { in_reply_to: command.LocalInteractionQuery.seq } })] });
        }
      }, outcomes: broadcast.stream };
      const flush = async () => { for (let index = 0; index < 16; index += 1) await Promise.resolve(); };
      const push = (reply: LocalInteractionQueryReply) => broadcast.push({ instanceId: 7, frames: [encodeAppFrame({ LocalInteractionQuery: { reply } })] });
      const oldToken: LocalInteractionQueryToken = { requestId: fixture.requests[0], queryGeneration: "41", identity: { appInstanceId: 7, generation: "7", revision: "11".repeat(32), documentRevision: "22".repeat(32), topologyRevision: "33".repeat(32) }, ordinal: "0" };
      const newToken: LocalInteractionQueryToken = { ...oldToken, requestId: fixture.requests[1], queryGeneration: "42" };
      const first = new AppChannelClient(handle, owner, 7, "fixture");
      const firstResult = first.readLocalInteractionPages(async () => {}).catch((error: unknown) => error);
      first.dispose();
      await flush();
      push({ kind: "started", token: oldToken });
      await flush();
      push({ kind: "closed", token: oldToken, cancelled: true });
      expect(await firstResult).toBeInstanceOf(Error);
      const second = new AppChannelClient(handle, owner, 7, "fixture");
      let consumed = 0;
      let ordinaryComplete = false;
      const secondResult = second.readLocalInteractionPages(async () => { consumed += 1; });
      const ordinary = second.readDocument().then((frames) => { ordinaryComplete = true; return frames; });
      for (const sequence of [...fixture.readSequences.slice(0, 1), ...fixture.cancelSequences.slice(0, 1), 999]) broadcast.push({ instanceId: 7, frames: [encodeAppFrame({ Done: { in_reply_to: sequence } })] });
      push({ kind: "started", token: oldToken });
      push({ kind: "page", page: { ...oldToken, terminal: true, bytes: [1] } });
      push({ kind: "closed", token: oldToken, cancelled: false });
      await flush();
      expect(consumed).toBe(0);
      expect(ordinaryComplete).toBe(false);
      push({ kind: "started", token: newToken });
      push({ kind: "page", page: { ...newToken, terminal: true, bytes: [2] } });
      await flush();
      expect(consumed).toBe(1);
      expect(sent.at(-1)).toEqual({ LocalInteractionQuery: { seq: fixture.ackSequence, command: { kind: "acknowledge", token: newToken } } });
      broadcast.push({ instanceId: 7, frames: [encodeAppFrame({ Done: { in_reply_to: fixture.ordinarySequence } })] });
      expect(await ordinary).toEqual([{ Done: { in_reply_to: fixture.ordinarySequence } }]);
      push({ kind: "closed", token: newToken, cancelled: false });
      expect(await secondResult).toEqual(newToken.identity);
      expect(sent.filter((command) => "LocalInteractionQuery" in command && command.LocalInteractionQuery.command.kind === "read").map((command) => "LocalInteractionQuery" in command ? command.LocalInteractionQuery.seq : 0)).toEqual(fixture.readSequences);
      second.dispose();
    });

    it("local interaction exhausted admission leaves no query slot and retains a cancellation sequence", async () => {
      const broadcast = createTurnOutcomeBroadcast<TurnOutcome>();
      const sent: AppCommandValue[] = [];
      const handle: AppChannelHandle = { enqueue: (_instance, bytes) => {
        for (const command of bytes.map(decodeAppCommand)) {
          sent.push(command);
          broadcast.push({ instanceId: 7, frames: [encodeAppFrame({ Done: { in_reply_to: Object.values(command)[0]!.seq } })] });
        }
      }, outcomes: broadcast.stream };
      const exhausted = new AppChannelClient(handle, new AppChannelRequestSequence(Number.MAX_SAFE_INTEGER - 1), 7, "fixture");
      await expect(exhausted.readLocalInteractionPages(async () => {})).rejects.toThrow("sequence-exhausted");
      await expect(exhausted.readLocalInteractionPages(async () => {})).rejects.toThrow("sequence-exhausted");
      expect(sent).toHaveLength(0);
      await exhausted.readDocument();
      expect(sent).toEqual([{ ReadDocument: { seq: Number.MAX_SAFE_INTEGER } }]);
      exhausted.dispose();
      sent.length = 0;
      const last = new AppChannelClient(handle, new AppChannelRequestSequence(Number.MAX_SAFE_INTEGER - 2), 7, "fixture");
      const result = last.readLocalInteractionPages(async () => {}).catch((error: unknown) => error);
      const token: LocalInteractionQueryToken = { requestId: "1", queryGeneration: "43", identity: { appInstanceId: 7, generation: "7", revision: "11".repeat(32), documentRevision: "22".repeat(32), topologyRevision: "33".repeat(32) }, ordinal: "0" };
      broadcast.push({ instanceId: 7, frames: [encodeAppFrame({ LocalInteractionQuery: { reply: { kind: "started", token } } }), encodeAppFrame({ LocalInteractionQuery: { reply: { kind: "page", page: { ...token, terminal: true, bytes: [1] } } } })] });
      for (let index = 0; index < 16; index += 1) await Promise.resolve();
      expect(sent.at(-1)).toEqual({ LocalInteractionQuery: { seq: Number.MAX_SAFE_INTEGER, command: { kind: "cancel", token } } });
      broadcast.push({ instanceId: 7, frames: [encodeAppFrame({ LocalInteractionQuery: { reply: { kind: "closed", token, cancelled: true } } })] });
      expect(await result).toBeInstanceOf(Error);
      last.dispose();
    });
    /** 🧪️ A fake handle that decodes whatever {@link AppChannelClient} `enqueue`d and pushes
     * caller-supplied frames back as this SAME instance's next outcome — enough to assert the client
     * frames/unframes correctly (and correlates replies through the real `outcomes` broadcast) without
     * a real plugin instance. */
    function fakeHandle(reply: (instanceId: number, commands: AppCommandValue[]) => AppFrameValue[]): AppChannelHandle {
      const broadcast = createTurnOutcomeBroadcast<TurnOutcome>();
      return {
        enqueue: (instanceId, events) => {
          const commands = events.map(decodeAppCommand);
          const frames = reply(instanceId, commands).map(encodeAppFrame);
          broadcast.push({ instanceId, frames });
        },
        outcomes: broadcast.stream,
      };
    }

    it("command() allocates an incrementing seq and returns every frame the batch produced", async () => {
      const seqsSeen: number[] = [];
      const handle = fakeHandle((_instanceId, commands) => {
        const cmd = commands[0];
        if (cmd && "Command" in cmd) seqsSeen.push(cmd.Command.seq);
        return [
          { Invocation: { in_reply_to: seqsSeen.at(-1) ?? 0, output: [1], diagnostics: [], ui_scope: [], history_patch: [], messages: [] } },
          { UiPatch: { in_reply_to: seqsSeen.at(-1) ?? 0, surface: "1:body", kind: "window", revision: 1, base_revision: 0, ops: [] } },
        ];
      });
      const client = new AppChannelClient(handle, new AppChannelRequestSequence(), 1, "app.demo");
      const first = await client.command(new Uint8Array([1, 2]), { cursor: 0 });
      const second = await client.command(new Uint8Array([3]), { cursor: 1 });
      expect(seqsSeen).toEqual([1, 2]);
      expect(first).toHaveLength(2);
      expect(second).toHaveLength(2);
    });

    it("configure()/readDocument()/loadDocument() frame the right AppCommand variant", async () => {
      const seen: AppCommandValue[] = [];
      const handle = fakeHandle((_instanceId, commands) => {
        seen.push(...commands);
        return [{ Done: { in_reply_to: Object.values(commands[0]!)[0]!.seq } }];
      });
      const client = new AppChannelClient(handle, new AppChannelRequestSequence(), 1, "app.demo");
      await client.configure({ locale: "en" });
      await client.readDocument();
      await client.loadDocument(new Uint8Array([1]), new Uint8Array([2]));
      expect(seen[0]).toEqual({ ConfigCommand: { seq: 1, command: Array.from(encodePackValue({ locale: "en" })) } });
      expect(seen[1]).toEqual({ ReadDocument: { seq: 2 } });
      expect(seen[2]).toEqual({ LoadDocument: { seq: 3, pack: [1], spr: [2] } });
    });

    it("caches the document pack from loadDocument()'s own arguments before any reply arrives", async () => {
      const handle = fakeHandle(() => [{ Done: { in_reply_to: 1 } }]);
      const client = new AppChannelClient(handle, new AppChannelRequestSequence(), 1, "app.demo");
      expect(client.documentPack()).toBeNull();
      await client.loadDocument(new Uint8Array([1, 2]), new Uint8Array([3]));
      expect(client.documentPack()).toEqual({ pack: new Uint8Array([1, 2]), spr: new Uint8Array([3]) });
    });

    it("caches the document pack from every AppFrame::Document reply, most recent wins", async () => {
      const handle = fakeHandle((_instanceId, commands) => {
        const cmd = commands[0];
        if (cmd && "ReadDocument" in cmd) {
          return [{ Document: { in_reply_to: cmd.ReadDocument.seq, pack: [9, 9], spr: [8], ops: "" } }];
        }
        return [{ Done: { in_reply_to: 1 } }];
      });
      const client = new AppChannelClient(handle, new AppChannelRequestSequence(), 1, "app.demo");
      await client.readDocument();
      expect(client.documentPack()).toEqual({ pack: new Uint8Array([9, 9]), spr: new Uint8Array([8]) });
    });

    it("transactionPrepareOwner()/transactionPreparePlanned()/transactionCommit()/transactionRollback()/transactionUndo()/transactionRedo() frame the right AppCommand variant", async () => {
      const seen: AppCommandValue[] = [];
      const handle = fakeHandle((_instanceId, commands) => {
        seen.push(...commands);
        return [{ Done: { in_reply_to: Object.values(commands[0]!)[0]!.seq } }];
      });
      const client = new AppChannelClient(handle, new AppChannelRequestSequence(), 1, "app.demo");
      await client.transactionPrepareOwner("txn-1", "s.doc#kind", new Uint8Array([1]));
      await client.transactionPreparePlanned("txn-1", [new Uint8Array([2]), new Uint8Array([3])], "duplicate", new Uint8Array([4]));
      await client.transactionCommit("txn-1");
      await client.transactionRollback("txn-1");
      await client.transactionUndo("grp-1");
      await client.transactionRedo("grp-1");
      expect(seen[0]).toEqual({ transactionPrepare: { seq: 1, txn_id: "txn-1", mutation_id: "s.doc#kind", payload: [1], prepared_ops: [], label: "", origin: [] } });
      expect(seen[1]).toEqual({ transactionPrepare: { seq: 2, txn_id: "txn-1", mutation_id: "", payload: [], prepared_ops: [[2], [3]], label: "duplicate", origin: [4] } });
      expect(seen[2]).toEqual({ transactionCommit: { seq: 3, txn_id: "txn-1" } });
      expect(seen[3]).toEqual({ transactionRollback: { seq: 4, txn_id: "txn-1" } });
      expect(seen[4]).toEqual({ transactionUndo: { seq: 5, group_id: "grp-1" } });
      expect(seen[5]).toEqual({ transactionRedo: { seq: 6, group_id: "grp-1" } });
    });

    /**
     * 🔗️ Rust↔TS parity for the C8 merge/conflict surface, driven through {@link AppChannelClient}'s
     * OWN public methods rather than the raw `encodeAppCommand`/`decodeAppFrame` codec functions the
     * `AppChannelCodec` suite already asserts against the same two files — this is the "does the
     * CLIENT layer itself send/surface the new commands and frames correctly" half of the parity
     * story (contract-freeze §C8/§C9, ticket
     * `26/08/16/MUTATION-OUTCOMES-MERGE-POLICIES-AND-FIRST-CLASS-CONFLICTS`). Four throwaway
     * `configure({})` calls burn seq 1-4 so `setMergePolicy`/`resolveConflict`/`readConflicts` land on
     * the exact seq (5/6/7) the golden vectors were baked against.
     */
    it("setMergePolicy()/resolveConflict()/readConflicts() match the shared cross-language merge command vectors, byte-for-byte", async () => {
      const { readFileSync } = await import("node:fs");
      const { fileURLToPath } = await import("node:url");
      const { dirname, join } = await import("node:path");
      const here = dirname(fileURLToPath(import.meta.url));
      const commandVectors = JSON.parse(readFileSync(join(here, "🧫️fixtures", "📡️channel", "app-command-merge.json"), "utf8")) as Record<string, string>;
      const hex = (bytes: Uint8Array) => Array.from(bytes, (b) => b.toString(16).padStart(2, "0")).join("");

      const seen: AppCommandValue[] = [];
      const handle = fakeHandle((_instanceId, commands) => {
        seen.push(...commands);
        return [{ Done: { in_reply_to: Object.values(commands[0]!)[0]!.seq } }];
      });
      const client = new AppChannelClient(handle, new AppChannelRequestSequence(), 1, "app.demo");
      await client.configure({});
      await client.configure({});
      await client.configure({});
      await client.configure({});
      await client.setMergePolicy("Normal");
      await client.resolveConflict("conflict-1", "accept");
      await client.readConflicts();

      expect(seen[4]).toEqual({ setMergePolicy: { seq: 5, policy: 1 } });
      expect(seen[5]).toEqual({ resolveConflict: { seq: 6, conflict_id: "conflict-1", resolution: 0 } });
      expect(seen[6]).toEqual({ readConflicts: { seq: 7 } });
      expect(hex(encodeAppCommand(seen[4]!)), "AppCommand::SetMergePolicy").toBe(commandVectors.SetMergePolicy);
      expect(hex(encodeAppCommand(seen[5]!)), "AppCommand::ResolveConflict").toBe(commandVectors.ResolveConflict);
      expect(hex(encodeAppCommand(seen[6]!)), "AppCommand::ReadConflicts").toBe(commandVectors.ReadConflicts);
    });

    it("command() surfaces unsolicited MergeReport/Conflicts frames and the extended Invocation.messages/Error.report fields verbatim", async () => {
      const handle = fakeHandle(() => [
        { Invocation: { in_reply_to: 1, output: [], diagnostics: [], ui_scope: [], history_patch: [], messages: [9] } },
        { MergeReport: { in_reply_to: null, report: [1] } },
        { Conflicts: { in_reply_to: null, conflicts: [2] } },
      ]);
      const client = new AppChannelClient(handle, new AppChannelRequestSequence(), 1, "app.demo");
      const frames = await client.command(new Uint8Array([1]), {});
      expect(frames).toHaveLength(3);
      const invocation = frames.find((frame): frame is Extract<AppFrameValue, { readonly Invocation: unknown }> => "Invocation" in frame);
      const mergeReport = frames.find((frame): frame is Extract<AppFrameValue, { readonly MergeReport: unknown }> => "MergeReport" in frame);
      const conflicts = frames.find((frame): frame is Extract<AppFrameValue, { readonly Conflicts: unknown }> => "Conflicts" in frame);
      expect(invocation?.Invocation.messages).toEqual([9]);
      expect(mergeReport?.MergeReport.report).toEqual([1]);
      expect(conflicts?.Conflicts.conflicts).toEqual([2]);
    });

    /**
     * 🔗️ Round-trips the frozen TS shapes ({@link DispatchReport}/{@link MergeReport}/{@link
     * Conflict}) through {@link encodePackValue}/the new `decode*FromWire` helpers — proving the
     * field-name mapping this lane derived from Rust's `#[serde(rename_all = "camelCase")]`
     * (`policy`/`worst`/`messages`, `insertionIndex`, `editId`→`edit_id` NOT renamed inside
     * `ConflictKind`'s struct variants, `MergePolicy`'s bare un-camelCased variant names) actually
     * decodes the way the wire's `store::pack_rt::encode_wire_value`-backed `encode_wire_serialized`
     * would produce it, since no live Rust `DispatchReport`/`MergeReport`/`Conflict` pack bytes are
     * checked into a fixture yet (only the outer `AppFrame` framing is, in `app-frame-merge.json`).
     */
    it("faultMessages()/decodeDispatchReportFromWire()/decodeMergeReportFromWire()/decodeConflictsFromWire() decode the frozen TS report shapes", () => {
      const dispatchReport: DispatchReport = {
        policy: "Vigilant",
        worst: "warning",
        messages: [{ level: "warning", code: "mutation.clamped", message: "value clamped to range" }],
      };
      const reportBytes = Array.from(encodePackValue(dispatchReport));
      expect(decodeDispatchReportFromWire(reportBytes, decodePackValue)).toEqual(dispatchReport);
      expect(faultMessages(reportBytes, decodePackValue)).toEqual(dispatchReport.messages);
      expect(faultMessages([], decodePackValue)).toEqual([]);

      const mergeReport: MergeReport = {
        policy: "Normal",
        accepted: true,
        insertionIndex: 3,
        replayed: [{ edit_id: "e1", messages: [{ level: "info", code: "mutation.cascade", message: "cascaded" }] }],
        worst: "info",
        conflict: null,
      };
      expect(decodeMergeReportFromWire(Array.from(encodePackValue(mergeReport)), decodePackValue)).toEqual(mergeReport);
      expect(decodeMergeReportFromWire([], decodePackValue)).toBeNull();

      const conflicts: readonly Conflict[] = [
        {
          id: "conflict-abc",
          kind: { kind: "degraded", edit_ids: ["e1"] },
          status: "open",
          messages: [{ level: "error", code: "mutation.target-missing", message: "target missing" }],
          actors: ["actor-1"],
          timestamp: { actor: 1, physical_ms: 100, logical: 0 },
        },
      ];
      expect(decodeConflictsFromWire(Array.from(encodePackValue(conflicts)), decodePackValue)).toEqual(conflicts);
      expect(decodeConflictsFromWire([], decodePackValue)).toEqual([]);
    });
  });

  // 🕸️ `@semio-tech/framework`'s `PluginGraph`/`InstanceDirectory`/`ArtifactMutationRouter`/
  // `ArtifactInferenceRouter` (ticket 26/08/16/PLUGIN-DEPENDENCIES-ARTIFACT-CONTRIBUTIONS-AND-COMPOSITE-MUTATIONS,
  // W2-B) have no in-source-testing harness of their own (`@semio-tech/framework`'s vitest only
  // `includeSource`s `🟦️glue.ts`, not the module file they're defined in) — dynamically importing the
  // real workspace package here exercises them under a config that DOES run, without this file taking
  // on a static dependency on `@semio-tech/framework`'s runtime exports.
  describe("@semio-tech/framework PluginGraph", () => {
    it("validates a graph with every dependency present and version-satisfying", async () => {
      const { validatePluginDependencyGraph } = await import("@semio-tech/framework");
      expect(
        validatePluginDependencyGraph([
          { pluginId: "a", version: "1.2.3" },
          { pluginId: "b", version: "1.0.0", dependencies: [{ pluginId: "a", version: "^1.0.0" }] },
        ]),
      ).toEqual([]);
    });

    it("reports a missing dependency", async () => {
      const { validatePluginDependencyGraph } = await import("@semio-tech/framework");
      expect(validatePluginDependencyGraph([{ pluginId: "b", dependencies: [{ pluginId: "missing", version: "*" }] }])).toEqual([
        { code: "transaction.dependency-missing", pluginId: "b", dependsOn: "missing" },
      ]);
    });

    it("reports a version mismatch", async () => {
      const { validatePluginDependencyGraph } = await import("@semio-tech/framework");
      expect(
        validatePluginDependencyGraph([
          { pluginId: "a", version: "2.0.0" },
          { pluginId: "b", dependencies: [{ pluginId: "a", version: "^1.0.0" }] },
        ]),
      ).toEqual([{ code: "transaction.version-mismatch", pluginId: "b", dependsOn: "a", required: "^1.0.0", actual: "2.0.0" }]);
    });

    it("resolves a diamond load order deterministically, tie-broken lexicographically", async () => {
      const { resolvePluginLoadOrder } = await import("@semio-tech/framework");
      const result = resolvePluginLoadOrder([
        {
          pluginId: "d",
          dependencies: [
            { pluginId: "b", version: "*" },
            { pluginId: "c", version: "*" },
          ],
        },
        { pluginId: "c", dependencies: [{ pluginId: "a", version: "*" }] },
        { pluginId: "b", dependencies: [{ pluginId: "a", version: "*" }] },
        { pluginId: "a" },
      ]);
      expect(result.errors).toEqual([]);
      expect(result.order).toEqual(["a", "b", "c", "d"]);
    });

    it("names every member of a cycle", async () => {
      const { resolvePluginLoadOrder } = await import("@semio-tech/framework");
      const result = resolvePluginLoadOrder([
        { pluginId: "a", dependencies: [{ pluginId: "b", version: "*" }] },
        { pluginId: "b", dependencies: [{ pluginId: "a", version: "*" }] },
      ]);
      expect(result.order).toEqual([]);
      expect(result.errors).toEqual([{ code: "transaction.cycle", members: ["a", "b"] }]);
    });

    it("versionSatisfies matches the frozen grammar (*, =, ^, ~, >=), including caret's leading-zero tiers", async () => {
      const { versionSatisfies } = await import("@semio-tech/framework");
      expect(versionSatisfies("1.2.3", "*")).toBe(true);
      expect(versionSatisfies("1.2.3", "=1.2.3")).toBe(true);
      expect(versionSatisfies("1.2.4", "=1.2.3")).toBe(false);
      expect(versionSatisfies("1.9.0", "^1.2.3")).toBe(true);
      expect(versionSatisfies("2.0.0", "^1.2.3")).toBe(false);
      expect(versionSatisfies("0.2.9", "^0.2.3")).toBe(true);
      expect(versionSatisfies("0.3.0", "^0.2.3")).toBe(false);
      expect(versionSatisfies("0.0.9", "^0.0.3")).toBe(false);
      expect(versionSatisfies("0.0.3", "^0.0.3")).toBe(true);
      expect(versionSatisfies("1.2.9", "~1.2.3")).toBe(true);
      expect(versionSatisfies("1.3.0", "~1.2.3")).toBe(false);
      expect(versionSatisfies("1.2.3", ">=1.2.3")).toBe(true);
      expect(versionSatisfies("9.9.9", ">=1.2.3")).toBe(true);
      expect(versionSatisfies("1.2.2", ">=1.2.3")).toBe(false);
    });

    it("orderPluginRegistryEntries drops only the blocked entries, dependency-orders the rest", async () => {
      const { orderPluginRegistryEntries } = await import("@semio-tech/framework");
      const result = orderPluginRegistryEntries([
        { pluginId: "b", moduleUrl: "b.js", dependencies: [{ pluginId: "a", version: "*" }] },
        { pluginId: "a", moduleUrl: "a.js" },
        { pluginId: "broken", moduleUrl: "broken.js", dependencies: [{ pluginId: "missing", version: "*" }] },
      ]);
      expect(result.order.map((entry) => entry.pluginId)).toEqual(["a", "b"]);
      expect(result.errors).toEqual([{ code: "transaction.dependency-missing", pluginId: "broken", dependsOn: "missing" }]);
    });

    it("pluginGraphErrorMessage renders a real English and a real German message", async () => {
      const { pluginGraphErrorMessage } = await import("@semio-tech/framework");
      const error = { code: "transaction.dependency-missing" as const, pluginId: "b", dependsOn: "a" };
      expect(pluginGraphErrorMessage(error, "en")).toContain("needs");
      expect(pluginGraphErrorMessage(error, "de")).toContain("benötigt");
    });

    it("PluginGraph.canUnload refuses while a loaded dependent exists, allows once it's gone", async () => {
      const { PluginGraph } = await import("@semio-tech/framework");
      const graph = new PluginGraph([{ pluginId: "a" }, { pluginId: "b", dependencies: [{ pluginId: "a", version: "*" }] }]);
      expect(graph.canUnload("a", new Set(["a", "b"]))).toBe(false);
      expect(graph.canUnload("a", new Set(["a"]))).toBe(true);
    });
  });

  describe("@semio-tech/framework InstanceDirectory and ArtifactRouters", () => {
    it("InstanceDirectory registers, resolves, and unregisters", async () => {
      const { InstanceDirectory } = await import("@semio-tech/framework");
      const directory = new InstanceDirectory();
      directory.register("artifact-1", { pluginId: "cad", instanceId: 3, artifactKind: "s.cad.model" });
      expect(directory.resolve("artifact-1")).toEqual({ pluginId: "cad", instanceId: 3, artifactKind: "s.cad.model" });
      directory.unregister("artifact-1");
      expect(directory.resolve("artifact-1")).toBeUndefined();
    });

    it("ArtifactMutationRouter accepts a byte-identical re-registration, rejects a conflicting one", async () => {
      const { ArtifactMutationRouter } = await import("@semio-tech/framework");
      const router = new ArtifactMutationRouter();
      router.registerOwner("s.cad.model", "s.cad#add-wall");
      router.registerOwner("s.cad.model", "s.cad#add-wall");
      expect(router.resolve("s.cad.model", "s.cad#add-wall")).toEqual({ kind: "owner" });
      expect(() =>
        router.registerContributed(
          "s.cad.model",
          "aec-building",
          "cad",
          { mutationId: "s.cad#add-wall", semantics: { verb: "add", entity: "wall", kind: "add-wall", record: "Wall" }, schemaVersion: 1, algorithmVersion: 1 },
          true,
        ),
      ).toThrow(/conflict/);
    });

    it("ArtifactMutationRouter.registerContributed rejects a contributor that doesn't depend on the owner", async () => {
      const { ArtifactMutationRouter } = await import("@semio-tech/framework");
      const router = new ArtifactMutationRouter();
      expect(() =>
        router.registerContributed(
          "s.cad.model",
          "aec-building",
          "cad",
          { mutationId: "s.cad#aec-building:add-room", semantics: { verb: "add", entity: "room", kind: "add-room", record: "Room" }, schemaVersion: 1, algorithmVersion: 1 },
          false,
        ),
      ).toThrow(/not a direct dependency/);
    });

    it("ArtifactInferenceRouter enforces owner === contributor and orders the depends_on DAG", async () => {
      const { ArtifactInferenceRouter } = await import("@semio-tech/framework");
      const router = new ArtifactInferenceRouter();
      router.registerContributed(
        "s.cad.model",
        {
          owner: "aec-building",
          artifactKind: "s.cad.model",
          artifactSchema: "s.cad.model",
          artifactSchemaVersion: 1,
          documentSchema: "s.cad",
          documentSchemaVersion: 1,
          inferenceSchema: "s.aec-building.load-path",
          inferenceSchemaVersion: 1,
          algorithmVersion: 1,
          policyVersion: 1,
          contributor: "aec-building",
          dependsOn: [],
        },
        true,
      );
      expect(router.resolve("s.cad.model", "s.aec-building.load-path")).toEqual({ kind: "contributed", pluginId: "aec-building" });
      expect(router.dependencyOrder()).toEqual(["s.cad.model s.aec-building.load-path"]);
    });

    it("ArtifactInferenceRouter.registerContributed rejects owner !== contributor", async () => {
      const { ArtifactInferenceRouter } = await import("@semio-tech/framework");
      const router = new ArtifactInferenceRouter();
      expect(() =>
        router.registerContributed(
          "s.cad.model",
          {
            owner: "someone-else",
            artifactKind: "s.cad.model",
            artifactSchema: "s.cad.model",
            artifactSchemaVersion: 1,
            documentSchema: "s.cad",
            documentSchemaVersion: 1,
            inferenceSchema: "s.aec-building.load-path",
            inferenceSchemaVersion: 1,
            algorithmVersion: 1,
            policyVersion: 1,
            contributor: "aec-building",
            dependsOn: [],
          },
          true,
        ),
      ).toThrow(/owner\/contributor mismatch/);
    });
  });
}
//#endregion 🧪️Tests

//#region StdioFormatKinds
/** 🗄️ Normalize `stdio.dwg` / `dwg` to short stdio format kind id. */
export function normalizeStdioFormatKind(value: string): string {
  const trimmed = value.trim();
  const short = trimmed.startsWith("stdio.") ? trimmed.slice("stdio.".length) : trimmed;
  switch (short) {
    case "jpeg":
      return "jpg";
    case "tif":
      return "tiff";
    case "stp":
      return "step";
    case "markdown":
      return "md";
    default:
      return short;
  }
}

/** 🗂️ File-picker accept filter from stdio format kind ids. */
export function mediaAcceptFilterKinds(formatArtifactKinds: readonly string[]): string {
  return formatArtifactKinds
    .map((kind) => normalizeStdioFormatKind(kind))
    .filter(Boolean)
    .map((kind) => `.${kind}`)
    .join(",");
}
//#endregion StdioFormatKinds

//#region 🔖️Directory
// 📇️ ticket 26/08/16/HUB-SPACES-LIVE-PRESENCE-AND-COLLABORATIVE-STUDIOS C1 — explicit re-export
// (CLAUDE.md: no `export *`) of the directory event log schema + pure read model so
// `@semio-tech/framework-os` consumers get it from the package root. Logic lives in
// `🔨️modules/📇️directory/🟦️component.ts`; this region only imports/re-exports and, per this
// package's `🧪️vitest.config.ts` (`include`/`includeSource` list only THIS file and
// `🟦️backbone-worker.ts`), hosts the in-source parity test against the Rust twin's golden fixture.
import { emptyDirectoryReadModel, fold, foldAll, isDirectoryCommandKind, isDirectoryEventBodyKind, isDirectoryStreamMessageKind } from "./🔨️modules/📇️directory/🟦️component.ts";
import type { DirectoryReadModel } from "./🔨️modules/📇️directory/🟦️component.ts";

export type {
  ConnectionView,
  DirectoryActor,
  DirectoryActorKind,
  DirectoryCommand,
  DirectoryConnectionPhase,
  DirectoryEvent,
  DirectoryEventBody,
  DirectoryReadModel,
  DirectorySpace,
  DirectorySpaceKind,
  DirectorySpaceRole,
  DirectorySpaceVisibility,
  DirectoryStreamMessage,
  DocumentView,
  Hlc,
  InviteView,
  MemberView,
  SpaceView,
  UserView,
} from "./🔨️modules/📇️directory/🟦️component.ts";
export { emptyDirectoryReadModel, fold, foldAll, isDirectoryCommandKind, isDirectoryEventBodyKind, isDirectoryStreamMessageKind };

if (import.meta.vitest) {
  const { describe, expect, it } = import.meta.vitest;

  describe("@semio-tech/framework-os directory", () => {
    const loadFixtureEvents = async (): Promise<DirectoryEvent[]> => {
      const { readFileSync } = await import("node:fs");
      const { fileURLToPath } = await import("node:url");
      const { dirname, join } = await import("node:path");
      const here = dirname(fileURLToPath(import.meta.url));
      const raw = readFileSync(join(here, "🧫️fixtures", "📇️directory", "🧾️events.json"), "utf8");
      return (JSON.parse(raw) as { events: DirectoryEvent[] }).events;
    };

    it("folds the golden fixture into the expected projection (parity with the Rust twin)", async () => {
      const events = await loadFixtureEvents();
      const model: DirectoryReadModel = foldAll(emptyDirectoryReadModel(), events);

      expect(model.cursor).toBe(16);
      expect(model.spaces.size).toBe(1);
      expect(model.spaces.has("sp-atelier-amara")).toBe(false);

      const studio = model.spaces.get("sp-studio-fabrication");
      expect(studio?.view.name).toBe("Fabrication Studio");
      expect(studio?.view.visibility).toBe("public");
      expect(studio?.view.kind).toBe("archive");
      expect(studio?.view.memberCount).toBe(2);

      const roles = (studio?.members ?? []).map((member) => [member.userId, member.role]).sort();
      expect(roles).toEqual([
        ["u-amara", "spectator"],
        ["u-devon", "spectator"],
      ]);

      const devon = studio?.members.find((member) => member.userId === "u-devon");
      expect(devon?.email).toBe("devon@semio.dev");
    });

    it("is idempotent on replay", async () => {
      const events = await loadFixtureEvents();
      const once = foldAll(emptyDirectoryReadModel(), events);
      const twice = foldAll(once, events);
      expect(twice).toEqual(once);
    });
  });
}

//#region 🔖️HubBinding
// 📇️ ticket 26/08/16/HUB-SPACES-LIVE-PRESENCE-AND-COLLABORATIVE-STUDIOS C2/C6 — the shell's ONLY
// point of contact with the directory hub's HTTP/WS control plane. Plugin surfaces never talk to
// the network (contract §C6); `🟦️backbone-worker.ts`'s `🔖️Directory` region is the only caller, so
// the shell never opens a directory socket on the UI thread. `fetch`/`WebSocket` only — no external
// HTTP library (CLAUDE.md "no external libraries for runtime purposes").
import type { DocumentView, InviteView, MemberView, SpaceView } from "./🔨️modules/📇️directory/🟦️component.ts";

/** 🔁️ Reconnect backoff shared by every hub transport this package opens — `connectHub` in
 * `🟦️backbone-worker.ts` (artifact sync) and {@link DirectoryClient.stream} both import these
 * (single source of truth; the two used to carry independent copies of the same two numbers). */
export const HUB_RECONNECT_MIN_MS = 500;
export const HUB_RECONNECT_MAX_MS = 30_000;

// 🎫️ ticket 26/08/17/MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME, packet `web-directory`, coordinator
// follow-up on finding 2 — CLAUDE.md "support short connection-shortages [...] not freeze the app"
// means a session that has been healthy for a while must not inherit an escalated backoff from
// earlier, unrelated blips; see {@link DirectoryClient.stream}'s docstring for the mechanism.
/** 🩺️ How long {@link DirectoryClient.stream}'s socket must stay open before a drop is treated as
 * "this connection was genuinely healthy" and resets the reconnect backoff toward
 * {@link HUB_RECONNECT_MIN_MS}. Deliberately set equal to {@link HUB_RECONNECT_MAX_MS}: surviving
 * open for at least one full worst-case backoff cycle is comfortably longer than any legitimate
 * reconnect delay this client would ever impose, so a genuinely flapping server — one that accepts
 * and then immediately drops, on a cycle time far shorter than this — can never cross the threshold
 * by accident. The reset is therefore only ever reachable by a connection that was actually stable,
 * never as a side effect of the accept-then-immediately-drop failure mode the backoff exists to
 * guard against. */
export const HUB_HEALTHY_RESET_MS = HUB_RECONNECT_MAX_MS;

/** 🔌️ A live {@link DirectoryClient.stream} subscription handle. */
export type DirectoryStream = { readonly close: () => void };

export type DirectoryMintedSession = { readonly token: string; readonly userId: string };
export type DirectorySessionSummary = { readonly userId: string; readonly email: string; readonly displayName: string; readonly expiresAt: number };
/** 🏠️ `GET /directory/spaces/{id}` — `invites` is always an array (empty for a non-author, per
 * contract §C2 "invites(authors only)"), never omitted; mirrors the Rust twin's `SpaceDetail`
 * (`🔨️modules/📇️directory/🔌️client/🦀️component.rs`, `#[serde(default)]`, not `skip_serializing`). */
export type DirectorySpaceDetail = SpaceView & { readonly members: readonly MemberView[]; readonly documents: readonly DocumentView[]; readonly invites: readonly InviteView[] };
export type DirectoryCommandResult = { readonly events: readonly DirectoryEvent[]; readonly result?: unknown };

/** 🚨️ Thrown by every {@link DirectoryClient} REST method on a non-2xx response — `status` lets a
 * caller (this package's `🟦️backbone-worker.ts` directory lane) distinguish "the hub answered and
 * rejected this" (surface immediately) from a thrown network error with no `status` at all ("the
 * hub is unreachable" — queue and retry). */
export class DirectoryHttpError extends Error {
  readonly status: number;
  constructor(status: number, message: string) {
    super(message);
    this.status = status;
  }
}

// 🎫️ ticket 26/08/17/MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME, packet `web-directory`, finding 1 —
// every REST call is bounded by a timeout and cancellable by a caller `signal`, so a hung directory
// server degrades ("hub unreachable, staying offline" — the ShellHost boot effect's existing catch)
// instead of hanging the identity/boot path forever awaiting a response that never arrives.
/** ⏱️ Per-request timeout for every {@link DirectoryClient} REST call — generous for a real request,
 * short enough that a hung server still lets the boot path's existing offline fallback run instead of
 * hanging indefinitely. */
export const DIRECTORY_HTTP_TIMEOUT_MS = 10_000;

/** 🎛️ Per-call options every {@link DirectoryClient} method accepts — just a caller-cancellable
 * `signal`; the timeout itself is fixed ({@link DIRECTORY_HTTP_TIMEOUT_MS}) and not caller-tunable. */
export interface DirectoryRequestOptions {
  readonly signal?: AbortSignal;
}

/**
 * 📡️ Typed facade over the directory hub's REST/WS surface (contract-freeze §C2). Constructed per
 * identity (`baseUrl` + optional bearer `token`, mutated in place by {@link mintSession}), reused for
 * every call. No client-side caching or optimistic mutation of the read model — the hub log is the
 * single writer (contract §C6).
 */
export class DirectoryClient {
  private readonly baseUrl: string;
  private token: string | undefined;

  constructor(baseUrl: string, token?: string) {
    this.baseUrl = baseUrl.replace(/\/+$/, "");
    this.token = token;
  }

  private headers(json: boolean): Record<string, string> {
    const headers: Record<string, string> = {};
    if (json) headers["content-type"] = "application/json";
    if (this.token) headers.authorization = `Bearer ${this.token}`;
    return headers;
  }

  /** 📨️ {@link FetchTimeoutResponse} plus the one extra accessor this class needs — declared locally
   * per this module's own body accessing only `json()` beyond the base shape. */
  private async getJson<T>(path: string, options?: DirectoryRequestOptions): Promise<T> {
    const response = await fetchWithTimeout(`${this.baseUrl}${path}`, { headers: this.headers(false) }, { timeoutMs: DIRECTORY_HTTP_TIMEOUT_MS, signal: options?.signal });
    if (!response.ok) throw new DirectoryHttpError(response.status, `directory: GET ${path} failed (${response.status})`);
    return (await response.json()) as T;
  }

  private async postJson<T>(path: string, body: unknown, options?: DirectoryRequestOptions): Promise<T> {
    const response = await fetchWithTimeout(
      `${this.baseUrl}${path}`,
      { method: "POST", headers: this.headers(true), body: JSON.stringify(body) },
      { timeoutMs: DIRECTORY_HTTP_TIMEOUT_MS, signal: options?.signal },
    );
    if (!response.ok) throw new DirectoryHttpError(response.status, `directory: POST ${path} failed (${response.status})`);
    return (await response.json()) as T;
  }

  /** 🪪️ `POST /auth/sessions` — dev email mint (contract §C2 "unchanged"). Wire response is
   * `{ token, user_id }` (the hub's `CreateAuthSessionResponse`, never renamed camelCase); this
   * client normalizes it and remembers the token for subsequent calls. A timeout/abort throws a plain
   * (non-{@link DirectoryHttpError}) error — the identity boot effect's existing catch-all already
   * treats that as "hub unreachable, stay offline" rather than blocking startup. */
  async mintSession(email: string, options?: DirectoryRequestOptions): Promise<DirectoryMintedSession> {
    const body = await this.postJson<{ token: string; user_id: string }>("/auth/sessions", { email }, options);
    this.token = body.token;
    return { token: body.token, userId: body.user_id };
  }

  /** 🪪️ `GET /auth/sessions/me` — `null` on 401 (no/expired session, the normal "not signed in"
   * outcome the boot flow branches on), throws {@link DirectoryHttpError} on any other failure. A
   * timeout/abort surfaces as a plain `Error` (no `.status`) — same "hub unreachable" shape the
   * caller's `directoryRejectionStatus`/identity-bootstrap catch already treats as offline, not a
   * boot-blocking failure (finding 1: this is what stops a hung server from hanging the boot path). */
  async me(options?: DirectoryRequestOptions): Promise<DirectorySessionSummary | null> {
    try {
      return await this.getJson<DirectorySessionSummary>("/auth/sessions/me", options);
    } catch (error) {
      if (error instanceof DirectoryHttpError && error.status === 401) return null;
      throw error;
    }
  }

  async spaces(options?: DirectoryRequestOptions): Promise<readonly SpaceView[]> {
    return this.getJson<SpaceView[]>("/directory/spaces", options);
  }

  async space(id: string, options?: DirectoryRequestOptions): Promise<DirectorySpaceDetail> {
    return this.getJson<DirectorySpaceDetail>(`/directory/spaces/${encodeURIComponent(id)}`, options);
  }

  async command(command: DirectoryCommand, options?: DirectoryRequestOptions): Promise<DirectoryCommandResult> {
    return this.postJson<DirectoryCommandResult>("/directory/commands", command, options);
  }

  async events(since: number, options?: DirectoryRequestOptions): Promise<readonly DirectoryEvent[]> {
    return this.getJson<DirectoryEvent[]>(`/directory/events?since=${encodeURIComponent(String(since))}`, options);
  }

  /** 🔌️ `GET /directory/ws?token=&since=` — subscribes from `since`, replays gap-free, then goes
   * live; text (JSON) frames, one {@link DirectoryStreamMessage} each (contract §C2, unlike the
   * binary `protocol_wire` the artifact sync hub channel speaks). Auto-reconnects via
   * {@link retryWithJitteredBackoff} (finding 2 — full jitter avoids a thundering herd when many
   * shells' directory sockets drop together, e.g. a hub restart), resuming from the highest
   * `seq`/`headSeq` this subscription has actually observed — never the caller's original `since` —
   * so a reconnect never replays a gap or a duplicate; this resume-from-`lastSeq` behaviour is
   * unchanged from before. Never throws into the caller: a malformed frame is dropped, a socket
   * error/close only feeds the reconnect loop.
   *
   * 🩺️ Coordinator follow-up: reconnecting after a connection that proved itself open for at least
   * {@link HUB_HEALTHY_RESET_MS} (see its docstring for why that threshold specifically can't be
   * crossed by a flapping server) resets the backoff — the next redial lands near
   * {@link HUB_RECONNECT_MIN_MS} instead of wherever the delay had climbed to from earlier, unrelated
   * blips (CLAUDE.md "support short connection-shortages"). Mechanism: `retryWithJitteredBackoff`'s
   * attempt counter lives inside ONE call and cannot be reset from outside it, so instead of one
   * call for the stream's whole life, {@link runCycles} below starts a FRESH call (fresh counter)
   * each time a cycle ends because its connection proved healthy before dropping — `connectOnce`
   * resolves (instead of rejecting) in exactly that case to end the current call as a "success". The
   * very first `fn()` of the next cycle is a synthetic, immediate rejection (never opens a socket) so
   * `retryWithJitteredBackoff`'s own jitter still inserts a `[MIN, 2·MIN]` pause before the real
   * redial — reusing its jitter math for that pause rather than reinventing it, and avoiding an
   * instant reconnect that would defeat jitter's whole point of not synchronizing many clients onto
   * the same instant. Mirrors `🟦️backbone-worker.ts`'s `connectHubOnce`/`connectHub` idiom for the
   * base reconnect loop; the health-reset addition here has no counterpart there (routed to that
   * file's own owning packet). */
  stream(since: number, onMessage: (message: DirectoryStreamMessage) => void): DirectoryStream {
    const abort = new AbortController();
    let socket: WebSocket | null = null;
    let lastSeq = since;
    let healthy = false; // 🩺️ set once THIS cycle's socket has been open for HUB_HEALTHY_RESET_MS.

    const wsUrl = (): string => {
      const wsBase = this.baseUrl.replace(/^http/, "ws");
      const query = new URLSearchParams();
      if (this.token) query.set("token", this.token);
      query.set("since", String(lastSeq));
      return `${wsBase}/directory/ws?${query.toString()}`;
    };

    /** 🔌️ One WS connection attempt. Resolves once {@link close} aborts (a clean shutdown) OR once
     * the socket closes after having been open for {@link HUB_HEALTHY_RESET_MS} (a proven-healthy
     * drop — ends this cycle as a "success" so {@link runCycles} starts a fresh, reset one); rejects
     * on every other close/error/construct-throw, feeding the current cycle's growing jitter exactly
     * like before. The health timer is armed on open and always cleared on close, whichever reason —
     * never left pending past this promise settling. */
    const connectOnce = (): Promise<void> =>
      new Promise<void>((resolve, reject) => {
        if (abort.signal.aborted) {
          reject(abort.signal.reason ?? new Error("directory stream: closed"));
          return;
        }
        let ws: WebSocket;
        try {
          ws = new WebSocket(wsUrl());
        } catch (error) {
          reject(error);
          return;
        }
        socket = ws;
        const onAbort = (): void => ws.close();
        abort.signal.addEventListener("abort", onAbort, { once: true });
        let healthyTimer: ReturnType<typeof setTimeout> | null = null;
        ws.onopen = () => {
          healthyTimer = setTimeout(() => {
            healthy = true;
          }, HUB_HEALTHY_RESET_MS);
        };
        ws.onmessage = (event: unknown) => {
          try {
            const data = (event as { data: unknown }).data;
            const message = JSON.parse(String(data)) as DirectoryStreamMessage;
            if (message.kind === "event") lastSeq = Math.max(lastSeq, message.event.seq);
            if (message.kind === "heartbeat") lastSeq = Math.max(lastSeq, message.headSeq);
            onMessage(message);
          } catch {
            // 🛟️ malformed frame — dropped, never thrown into the caller.
          }
        };
        ws.onclose = () => {
          abort.signal.removeEventListener("abort", onAbort);
          if (healthyTimer != null) clearTimeout(healthyTimer);
          if (socket === ws) socket = null;
          if (abort.signal.aborted || healthy) {
            resolve();
            return;
          }
          reject(new Error("directory stream: socket closed"));
        };
        ws.onerror = () => {
          try {
            ws.close();
          } catch {
            // 🛟️ already closing.
          }
        };
      });

    /** 🔁️ Runs {@link connectOnce} through {@link retryWithJitteredBackoff} for one cycle at a time,
     * forever, until `close()` aborts. A cycle ends in success only when `connectOnce` resolves —
     * either the manual-close case, or the health-reset case — never by exhausting retries (the
     * underlying primitive retries forever on failure by design). On a health-reset success, the NEXT
     * cycle is built "primed": its `fn` synthetically rejects once, immediately, before ever touching
     * the network, purely so the retry primitive's own jitter inserts a `[MIN, 2·MIN]` pause ahead of
     * the real redial — see this method's docstring for why that beats both an instant reconnect and
     * hand-rolling a second jitter formula. */
    async function runCycles(): Promise<void> {
      let primeNextCycle = false;
      for (;;) {
        healthy = false;
        let primed = !primeNextCycle;
        const fn = (): Promise<void> => {
          if (!primed) {
            primed = true;
            return Promise.reject(new Error("directory stream: healthy-reset pause"));
          }
          return connectOnce();
        };
        try {
          await retryWithJitteredBackoff(fn, { minMs: HUB_RECONNECT_MIN_MS, maxMs: HUB_RECONNECT_MAX_MS, signal: abort.signal });
        } catch {
          return; // 🛑 only reachable via an aborted signal — `close()` was called.
        }
        if (abort.signal.aborted) return; // 🛑 resolved via the manual-close path above.
        primeNextCycle = healthy; // 🩺️ resolved via the health-reset path — start the next cycle primed.
      }
    }

    void runCycles();

    return {
      close: () => {
        abort.abort();
        socket?.close();
      },
    };
  }
}

if (import.meta.vitest) {
  const { describe, expect, it, vi } = import.meta.vitest;

  class FakeDirectoryWebSocket {
    static instances: FakeDirectoryWebSocket[] = [];
    readonly url: string;
    readyState = 0;
    onopen: (() => void) | null = null;
    onmessage: ((event: { data: string }) => void) | null = null;
    onclose: (() => void) | null = null;
    onerror: (() => void) | null = null;
    constructor(url: string) {
      this.url = url;
      FakeDirectoryWebSocket.instances.push(this);
    }
    send(): void {}
    close(): void {
      this.readyState = 3;
    }
    triggerOpen(): void {
      this.readyState = 1;
      this.onopen?.();
    }
    triggerMessage(message: DirectoryStreamMessage): void {
      this.onmessage?.({ data: JSON.stringify(message) });
    }
    triggerClose(): void {
      this.readyState = 3;
      this.onclose?.();
    }
  }

  function sampleDirectoryEvent(seq: number): DirectoryEvent {
    return {
      seq,
      id: `evt-${seq}`,
      hlc: { physicalMs: seq, logical: 0 },
      actor: { kind: "user", id: "u-1" },
      body: { kind: "space.renamed", spaceId: "sp-1", name: `space ${seq}` },
      recordedAtMs: seq,
    };
  }

  describe("DirectoryClient.stream", () => {
    it("replays then goes live with no gap and no duplicate", () => {
      FakeDirectoryWebSocket.instances = [];
      (globalThis as unknown as { WebSocket: unknown }).WebSocket = FakeDirectoryWebSocket;
      const received: DirectoryStreamMessage[] = [];
      const client = new DirectoryClient("http://hub.test", "tok-1");
      const handle = client.stream(0, (message) => received.push(message));
      const socket = FakeDirectoryWebSocket.instances[0]!;
      expect(socket.url).toBe("ws://hub.test/directory/ws?token=tok-1&since=0");
      socket.triggerOpen();
      socket.triggerMessage({ kind: "event", event: sampleDirectoryEvent(1) });
      socket.triggerMessage({ kind: "event", event: sampleDirectoryEvent(2) });
      socket.triggerMessage({ kind: "heartbeat", headSeq: 2 });
      expect(received.map((message) => message.kind)).toEqual(["event", "event", "heartbeat"]);
      expect(received).toHaveLength(3);
      handle.close();
    });

    it("reconnects resuming from the last seen seq, with jittered backoff within bounds", async () => {
      vi.useFakeTimers();
      const randomSpy = vi.spyOn(Math, "random");
      try {
        FakeDirectoryWebSocket.instances = [];
        (globalThis as unknown as { WebSocket: unknown }).WebSocket = FakeDirectoryWebSocket;
        randomSpy.mockReturnValue(0); // 🎯️ pins the jitter draw to the lower bound of its range.
        const client = new DirectoryClient("http://hub.test", "tok-1");
        const handle = client.stream(0, () => {});
        const first = FakeDirectoryWebSocket.instances[0]!;
        first.triggerMessage({ kind: "event", event: sampleDirectoryEvent(7) });
        first.triggerClose();
        await Promise.resolve(); // 🪧️ let the rejection's microtask reach retryWithJitteredBackoff's catch (which schedules the backoff timer) before advancing fake time — advanceTimersByTime does not itself drain microtasks.

        // attempt 1: cap = min(MAX, MIN·2¹) = 2·MIN; random()=0 ⇒ delay lands exactly on the lower
        // bound (MIN) — no reconnect fires a tick earlier, and one fires the instant it's due.
        await vi.advanceTimersByTimeAsync(HUB_RECONNECT_MIN_MS - 1);
        expect(FakeDirectoryWebSocket.instances).toHaveLength(1);
        await vi.advanceTimersByTimeAsync(1);
        expect(FakeDirectoryWebSocket.instances).toHaveLength(2);
        const second = FakeDirectoryWebSocket.instances[1]!;
        expect(second.url).toBe("ws://hub.test/directory/ws?token=tok-1&since=7"); // resumes from lastSeq, never the original `since`.

        randomSpy.mockReturnValue(1); // 🎯️ pins the jitter draw to the upper bound of its range.
        second.triggerClose();
        await Promise.resolve();

        // attempt 2: cap = min(MAX, MIN·2²) = 4·MIN; random()=1 ⇒ delay lands exactly on that upper
        // bound — proving the delay grows and stays within [MIN, cap], not a fixed exponential value.
        const attempt2Cap = Math.min(HUB_RECONNECT_MAX_MS, HUB_RECONNECT_MIN_MS * 2 ** 2);
        await vi.advanceTimersByTimeAsync(attempt2Cap - 1);
        expect(FakeDirectoryWebSocket.instances).toHaveLength(2);
        await vi.advanceTimersByTimeAsync(1);
        expect(FakeDirectoryWebSocket.instances).toHaveLength(3);

        handle.close();
      } finally {
        randomSpy.mockRestore();
        vi.useRealTimers();
      }
    });

    it("never throws into the caller on a malformed frame", () => {
      FakeDirectoryWebSocket.instances = [];
      (globalThis as unknown as { WebSocket: unknown }).WebSocket = FakeDirectoryWebSocket;
      const received: DirectoryStreamMessage[] = [];
      const client = new DirectoryClient("http://hub.test");
      const handle = client.stream(0, (message) => received.push(message));
      const socket = FakeDirectoryWebSocket.instances[0]!;
      expect(() => socket.onmessage?.({ data: "not json" })).not.toThrow();
      socket.triggerMessage({ kind: "heartbeat", headSeq: 0 });
      expect(received).toHaveLength(1);
      handle.close();
    });

    it("close() stops the reconnect loop — no further socket is ever opened", () => {
      vi.useFakeTimers();
      try {
        FakeDirectoryWebSocket.instances = [];
        (globalThis as unknown as { WebSocket: unknown }).WebSocket = FakeDirectoryWebSocket;
        const client = new DirectoryClient("http://hub.test");
        const handle = client.stream(0, () => {});
        const first = FakeDirectoryWebSocket.instances[0]!;
        handle.close();
        first.triggerClose();
        vi.advanceTimersByTime(HUB_RECONNECT_MAX_MS * 2);
        expect(FakeDirectoryWebSocket.instances).toHaveLength(1);
      } finally {
        vi.useRealTimers();
      }
    });

    it("(a) a drop after sustained health resets the backoff — reconnects near HUB_RECONNECT_MIN_MS, not at an escalated delay", async () => {
      vi.useFakeTimers();
      const randomSpy = vi.spyOn(Math, "random");
      try {
        FakeDirectoryWebSocket.instances = [];
        (globalThis as unknown as { WebSocket: unknown }).WebSocket = FakeDirectoryWebSocket;
        randomSpy.mockReturnValue(0); // 🎯️ pins every jittered delay to its lower bound.
        const client = new DirectoryClient("http://hub.test");
        const handle = client.stream(0, () => {});
        const first = FakeDirectoryWebSocket.instances[0]!;
        first.triggerOpen();

        // 🩺️ let it prove itself healthy, then drop it.
        await vi.advanceTimersByTimeAsync(HUB_HEALTHY_RESET_MS);
        first.triggerClose();
        await Promise.resolve(); // let the resolved connectOnce reach runCycles' next-cycle setup.

        // The next cycle is primed: its first (synthetic) failure is attempt 1 of a FRESH counter —
        // cap = min(MAX, MIN·2¹) = 2·MIN; random()=0 ⇒ delay lands exactly on the lower bound, MIN —
        // never the far larger delay an un-reset counter would have reached by this point.
        await vi.advanceTimersByTimeAsync(HUB_RECONNECT_MIN_MS - 1);
        expect(FakeDirectoryWebSocket.instances).toHaveLength(1);
        await vi.advanceTimersByTimeAsync(1);
        expect(FakeDirectoryWebSocket.instances).toHaveLength(2);

        handle.close();
      } finally {
        randomSpy.mockRestore();
        vi.useRealTimers();
      }
    });

    it("(b) rapid accept-then-drop cycling never crosses the health threshold — backoff keeps escalating, never resets", async () => {
      vi.useFakeTimers();
      const randomSpy = vi.spyOn(Math, "random");
      try {
        FakeDirectoryWebSocket.instances = [];
        (globalThis as unknown as { WebSocket: unknown }).WebSocket = FakeDirectoryWebSocket;
        randomSpy.mockReturnValue(1); // 🎯️ pins every jittered delay to its upper bound (== cap), so a reset shows up unmistakably as a delay dropping back down.
        const client = new DirectoryClient("http://hub.test");
        const handle = client.stream(0, () => {});

        let instanceCount = 1;
        for (let attempt = 1; attempt <= 3; attempt++) {
          const socket = FakeDirectoryWebSocket.instances[instanceCount - 1]!;
          socket.triggerOpen();
          socket.triggerClose(); // drops instantly — nowhere near HUB_HEALTHY_RESET_MS, so `healthy` stays false.
          await Promise.resolve();
          const cap = Math.min(HUB_RECONNECT_MAX_MS, HUB_RECONNECT_MIN_MS * 2 ** attempt);
          await vi.advanceTimersByTimeAsync(cap - 1);
          expect(FakeDirectoryWebSocket.instances).toHaveLength(instanceCount); // still escalated — a reset would have reconnected far sooner than this growing cap.
          await vi.advanceTimersByTimeAsync(1);
          instanceCount += 1;
          expect(FakeDirectoryWebSocket.instances).toHaveLength(instanceCount);
        }

        handle.close();
      } finally {
        randomSpy.mockRestore();
        vi.useRealTimers();
      }
    });

    it("(c) close() during a healthy-but-not-yet-reset connection cancels promptly, clears the health timer, and never reconnects", async () => {
      vi.useFakeTimers();
      try {
        FakeDirectoryWebSocket.instances = [];
        (globalThis as unknown as { WebSocket: unknown }).WebSocket = FakeDirectoryWebSocket;
        const client = new DirectoryClient("http://hub.test");
        const handle = client.stream(0, () => {});
        const first = FakeDirectoryWebSocket.instances[0]!;
        first.triggerOpen();
        await vi.advanceTimersByTimeAsync(HUB_HEALTHY_RESET_MS / 2); // health timer armed, not yet fired.
        handle.close();
        first.triggerClose();
        await vi.advanceTimersByTimeAsync(HUB_RECONNECT_MAX_MS * 2);
        expect(FakeDirectoryWebSocket.instances).toHaveLength(1); // no reconnect was ever attempted.
        expect(vi.getTimerCount()).toBe(0); // 🛟️ neither the health timer nor any backoff timer is left pending.
      } finally {
        vi.useRealTimers();
      }
    });
  });

  describe("DirectoryClient http (getJson/postJson timeout + abort)", () => {
    const originalFetch = globalThis.fetch;

    it("a hung server rejects at the timeout instead of hanging the caller forever", async () => {
      vi.useFakeTimers();
      try {
        globalThis.fetch = vi.fn((_url: string, init?: RequestInit) => {
          return new Promise((_resolve, reject) => {
            init?.signal?.addEventListener("abort", () => reject(init.signal!.reason ?? new Error("aborted")));
          });
        }) as unknown as typeof fetch;
        const client = new DirectoryClient("http://hub.test");
        const promise = client.me(); // 🪪️ the identity/boot-path call finding 1 is about.
        let settled = false;
        promise.then(
          () => (settled = true),
          () => (settled = true),
        );
        await vi.advanceTimersByTimeAsync(DIRECTORY_HTTP_TIMEOUT_MS + 1_000);
        expect(settled).toBe(true); // never hangs — this is what lets the boot path's own
        // catch-all ("hub unreachable, staying offline") actually run instead of awaiting forever.
        await expect(promise).rejects.toThrow();
      } finally {
        globalThis.fetch = originalFetch;
        vi.useRealTimers();
      }
    });

    it("an external abort cancels promptly, without ever waiting out the timeout", async () => {
      vi.useFakeTimers();
      try {
        globalThis.fetch = vi.fn((_url: string, init?: RequestInit) => {
          return new Promise((_resolve, reject) => {
            init?.signal?.addEventListener("abort", () => reject(init.signal!.reason ?? new Error("aborted")));
          });
        }) as unknown as typeof fetch;
        const client = new DirectoryClient("http://hub.test");
        const controller = new AbortController();
        const promise = client.spaces({ signal: controller.signal });
        controller.abort(new Error("caller cancelled"));
        // no `vi.advanceTimersByTime*` call at all: if this depended on the timeout timer firing,
        // fake timers would leave it pending forever and this await would hang the test.
        await expect(promise).rejects.toThrow("caller cancelled");
      } finally {
        globalThis.fetch = originalFetch;
        vi.useRealTimers();
      }
    });

    it("still resolves normally when the server answers promptly", async () => {
      globalThis.fetch = vi.fn(async () => ({ ok: true, status: 200, json: async () => [] })) as unknown as typeof fetch;
      try {
        const client = new DirectoryClient("http://hub.test");
        await expect(client.spaces()).resolves.toEqual([]);
      } finally {
        globalThis.fetch = originalFetch;
      }
    });
  });
}
//#endregion 🔖️HubBinding
//#endregion 🔖️Directory
