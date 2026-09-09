// #region Header
/**
 * 🖥️ `@semio-tech/framework-os` — JS sync/backbone protocol surface (backbone URIs, document
 * envelopes, `🧵️backbone-worker.ts` request/response wire types, `PersistenceBinding`/`MutationEnvelope`,
 * {@link buildFrameworkSyncUtilities}) consumed by `framework/os/renderer/js/react/index.tsx` and
 * `framework/os/dev/script.ts`. The OS kernel's *stateful* logic (operation application, program
 * registry) is Rust/wasm-only, hosted by the s-plugin wasm — this file is not a JS port of that. The
 * one exception is {@link planWorkflow}: a pure, side-effect-free scheduling function has no state
 * to keep in sync with a live wasm host, so it's hand-mirrored here against the Rust `plan_workflow`
 * (`framework/os/core/rs/lib.rs`) with shared fixtures (`framework/os/core/fixtures/`)
 * asserting parity.
 */
// #endregion Header

import type { AppRef, AppRole, AppRouter, ArtifactDialect, ArtifactDiff, Conflict, ConflictResolution, DispatchReport, Fault, FetchTimeoutResponse, InverseMutation, KernelMutation, MergePolicy, MergeReport, MutationMessage, OpeningPreferences, PluginWasmHandle, TurnOutcome, UndoGroup, UndoPolicy, UtilityLeaf } from "@semio-tech/framework";
import { conflictResolutionAsU8, createTurnOutcomeBroadcast, dialectCoordinate, fetchWithTimeout, mergePolicyAsU8, parseDialectCoordinate, parseSurfaceAppId, resolveOpeningApp, retryWithJitteredBackoff } from "@semio-tech/framework";
/** 📇️ Directory event/command/DTO types (contract-freeze §C1/§C6) — imported once here for
 * {@link BackboneWorkerRequest}/{@link BackboneWorkerResponse}'s `directory-*` variants and this
 * file's `🔖️HubBinding` region; never redeclared (lane 0-A owns the type source). */
import type { DirectoryCommand, DirectoryEvent, DirectoryStreamMessage } from "./🔨️modules/📇️directory/🟦️.ts";
import { parseDirectorySessionAuthorityJsonV1, type DirectorySessionAuthorityV1 } from "./🔨️modules/📇️directory/🧬️schema/🪪️session-authority-v1/🟦️.ts";
export { parseDirectorySessionAuthorityJsonV1, type DirectorySessionAuthorityV1 } from "./🔨️modules/📇️directory/🧬️schema/🪪️session-authority-v1/🟦️.ts";
import { parseInferencePortClosedV1, parseInferencePortOpeningRequestV1, parseInferencePortOpeningResultV1, type InferencePortClosedV1, type InferencePortOpeningResultV1 } from "./🔨️modules/💡️inference/🚪️opening/🟦️.ts";
import type { ArtifactFrontier, DirectoryCommandErrorCodeV1, DirectoryCommandOutcomeV1, DirectoryCommandReceiptV1, DirectoryCommandRequestV1, DirectoryEventPageV1, DocumentExecutionTargetLeaseFieldsV1, DocumentExecutionTargetProgressV1, DocumentExecutionTargetStatusCodeV1, GisMapInferencePortCodeV1, GisMapInferencePortStatusV1 } from "./🔨️modules/📇️directory/🧬️schema/🟦️.ts";
import { DIRECTORY_COMMAND_RECEIPT_MAX_BYTES, DIRECTORY_EVENT_PAGE_MAX_BYTES, GIS_MAP_INFERENCE_PORT_CODE_TEXT_V1, artifactFrontierIsEditedForV1, artifactFrontierIsGenesisForV1, directoryCommandErrorFromStatus, directoryCommandRequestJson, parseDirectoryCommandReceiptV1, parseDirectoryEventPageV1, parseGisMapInferencePortStatusV1 } from "./🔨️modules/📇️directory/🧬️schema/🟦️.ts";
export { artifactFrontierIsEditedForV1, artifactFrontierIsGenesisForV1 } from "./🔨️modules/📇️directory/🧬️schema/🟦️.ts";
/** 📡️ The replication wire contract lives in `🧰️framework/🔨️modules/📡️replication` — os speaks it,
 * it is not os-owned. Frames/envelopes/presence peers all come from there. */
import type { ArtifactPresencePeer, ClientFrame, ExactWireMutationEnvelope, LocalInteractionIdentity, LocalInteractionPage, LocalInteractionQueryCommand, LocalInteractionQueryReply, LocalInteractionQueryToken, MutationEnvelope, ServerFrame, WireAckStage, WireFrontierSummary, WireLane, WireMutationEnvelope } from "@semio-tech/framework-replication";
import { decodeClientFrame, decodeLocalInteractionQueryCommand, decodeLocalInteractionQueryReply, decodePresencePeer, decodeServerFrame, encodeClientFrame, encodeLocalInteractionQueryCommand, encodeLocalInteractionQueryReply, encodePresencePeer, encodeServerFrame, localInteractionIdentityEquals, mutationEnvelopeFromWire, mutationEnvelopeToWire } from "@semio-tech/framework-replication";
/** 🔢️ Shared byte-codec floor — the same primitives the wire frames are built from; os reuses them
 * for its backbone-envelope and app-channel codecs rather than keeping a second copy. */
import { decodeCausalEnvelopeBatch, decodeDocumentBackboneEnvelopeBatchExact, encodeCausalEnvelopeBatch, readBool, readBytes, readF64, readHash32, readStr, readU8, readVarintU64, readVecBytes, readVecEnvelope, readVecStr, writeBool, writeBytes, writeF64, writeHash32, writeStr, writeVarintU64, writeVecBytes, writeVecEnvelope, writeVecStr } from "@semio-tech/framework-replication";
import { parseBrowserActorUiPatchOfferV1, parseBrowserActorUiPatchResultV1, type BrowserActorUiPatchOfferV1, type BrowserActorUiPatchResultV1 } from "./🔨️modules/🔌️plugin/🌐️browser-bundle/🩹️patch-handoff/🟦️.ts";
import { parseBrowserActorActionRequestV1, parseBrowserActorActionResultV1, type BrowserActorActionRequestV1, type BrowserActorActionResultV1 } from "./🔨️modules/🔌️plugin/🌐️browser-bundle/🎯️action-handoff/🟦️.ts";
import { parseBrowserActorViewStateRequest, type BrowserActorViewStateRequest } from "./🔨️modules/🔌️plugin/🌐️browser-bundle/🪟️view-context/🟦️.ts";
export type { BrowserActorUiPatchOfferV1, BrowserActorUiPatchResultV1 };

const replicationPackCodec = { encode: encodePackValue, decode: decodePackValue };

//#region 🔖️ArtifactOpeningRelay
/** 📂️ Canonical shell relay after parsing role aliases, validating an exact "Open with…"
 * choice against the live router, and resolving a plain open through the event-sourced opening
 * preferences projection. A document is attachable only as the complete `(documentId, schema)`
 * pair; `spaceId` remains optional because local-only documents deliberately have none. */
export type ResolvedArtifactOpeningRelay = {
  readonly artifactRef: string;
  readonly dialect: ArtifactDialect;
  readonly role: AppRole;
  readonly app: AppRef;
  readonly documentId?: string;
  readonly spaceId?: string;
  readonly schema?: string;
};

function openingRelayError(code: string): never {
  throw new Error(code);
}

function openingRelayRole(value: unknown): AppRole | undefined {
  if (value === undefined) return undefined;
  if (value === 0 || value === "viewer") return "viewer";
  if (value === 1 || value === "editor") return "editor";
  return openingRelayError("opening.invalid-role");
}

function openingRelayString(record: Readonly<Record<string, unknown>>, field: string, errorCode = `opening.invalid-${field}`): string | undefined {
  const value = record[field];
  if (value === undefined) return undefined;
  if (typeof value !== "string" || value.trim() === "") return openingRelayError(errorCode);
  return value;
}

/** 🧭 Resolves `os.open-artifact`/`os.open-artifact-with` without mutating the router or
 * preferences. Missing role means editor, matching both shells' declared boot default. */
export function resolveArtifactOpeningRelay(
  actionId: string,
  args: unknown,
  router: AppRouter,
  preferences: OpeningPreferences,
): ResolvedArtifactOpeningRelay {
  if (actionId !== "os.open-artifact" && actionId !== "os.open-artifact-with") return openingRelayError("opening.invalid-action");
  if (!args || typeof args !== "object" || Array.isArray(args)) return openingRelayError("opening.invalid-args");
  const record = args as Readonly<Record<string, unknown>>;
  const rawArtifactRef = openingRelayString(record, "artifactRef", "opening.invalid-artifact-ref");
  if (!rawArtifactRef) return openingRelayError("opening.invalid-artifact-ref");

  let dialect: ArtifactDialect;
  let surfaceRole: AppRole | undefined;
  try {
    if (rawArtifactRef.includes("#")) {
      const surface = parseSurfaceAppId(rawArtifactRef);
      dialect = surface.dialect;
      surfaceRole = surface.role;
    } else {
      dialect = parseDialectCoordinate(rawArtifactRef);
    }
  } catch {
    return openingRelayError("opening.invalid-artifact-ref");
  }

  const wireRole = openingRelayRole(record.role);
  if (surfaceRole && wireRole && surfaceRole !== wireRole) return openingRelayError("opening.role-mismatch");
  const role = wireRole ?? surfaceRole ?? "editor";
  const pluginId = openingRelayString(record, "pluginId");
  const appId = openingRelayString(record, "appId");
  if (Boolean(pluginId) !== Boolean(appId)) return openingRelayError("opening.partial-app-ref");
  if (actionId === "os.open-artifact-with" && (!pluginId || !appId)) return openingRelayError("opening.explicit-app-required");

  let app: AppRef;
  if (pluginId && appId) {
    const explicit = { pluginId, appId };
    if (!router.entriesFor(dialect, role).some((entry) => entry.pluginId === explicit.pluginId && entry.appId === explicit.appId)) return openingRelayError("opening.app-mismatch");
    app = explicit;
  } else {
    app = resolveOpeningApp(router, dialect, role, preferences);
  }

  const documentId = openingRelayString(record, "documentId");
  const schema = openingRelayString(record, "schema");
  if (Boolean(documentId) !== Boolean(schema)) return openingRelayError("opening.partial-document-ref");
  const spaceId = openingRelayString(record, "spaceId");
  return {
    artifactRef: dialectCoordinate(dialect),
    dialect,
    role,
    app,
    ...(documentId && schema ? { documentId, schema } : {}),
    ...(spaceId ? { spaceId } : {}),
  };
}
//#endregion 🔖️ArtifactOpeningRelay

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
  const { registerTests1 } = await import("./🧪️tests/🧪️backbone-envelope-io/🟦️.ts");
  await registerTests1(import.meta.vitest, { BACKBONE_ENVELOPE_RETRY_WINDOW_MS, readBackboneEnvelope, writeBackboneEnvelope }, { directory: import.meta.dir, url: import.meta.url });
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
  | { readonly kind: "mutations"; readonly envelopes: Uint8Array }
  | { readonly kind: "ack"; readonly opIds: readonly string[] };

export const BACKBONE_HOT_MESSAGE_MAXIMUM_BYTES = 262_144;
export const BACKBONE_SNAPSHOT_MAXIMUM_BYTES = 4 * 1024 * 1024;

/** 📤️ Canonical Store OpBinary: version, variant, symbol table and exact record fields. */
export function encodeBackboneMessage(message: BinaryBackboneMessage): Uint8Array {
  const maximum = message.kind === "snapshot" ? BACKBONE_SNAPSHOT_MAXIMUM_BYTES : BACKBONE_HOT_MESSAGE_MAXIMUM_BYTES;
  const encoder = new TextEncoder();
  const checkedBytes = (value: Uint8Array) => {
    if (!(value instanceof Uint8Array) || value.length > maximum) throw new Error("backbone message: byte limit");
    return value;
  };
  if (message.kind === "ack") {
    if (message.opIds.length > maximum / 2) throw new Error("backbone message: item limit");
    for (const value of message.opIds) {
      if (typeof value !== "string" || value.length > maximum || encoder.encode(value).length > maximum || new TextDecoder("utf-8", { fatal: true }).decode(encoder.encode(value)) !== value) throw new Error("backbone message: invalid string");
    }
  } else if (message.kind === "snapshot") {
    if (checkedBytes(message.pack).length + checkedBytes(message.spr).length > maximum) throw new Error("backbone message: byte limit");
  } else if (message.kind === "mutations") checkedBytes(message.envelopes);
  else throw new Error("backbone message: unknown kind");
  const symbols = message.kind === "ack" ? packBuildSymbols(message.opIds) : [];
  const symbolIndex = new Map(symbols.map((value, index) => [value, index] as const));
  const out: number[] = [1, message.kind === "snapshot" ? 0 : message.kind === "mutations" ? 1 : 2];
  const bounded = () => { if (out.length > maximum) throw new Error("backbone message: byte limit"); };
  writeVarintU64(out, symbols.length);
  for (const symbol of symbols) {
    const bytes = encoder.encode(symbol);
    writeVarintU64(out, bytes.length);
    packPushBytes(out, bytes);
    bounded();
  }
  writeVarintU64(out, message.kind === "snapshot" ? 2 : 1);
  const byteField = (id: number, bytes: Uint8Array) => {
    writeVarintU64(out, id);
    out.push(0x08);
    writeVarintU64(out, bytes.length);
    packPushBytes(out, bytes);
    bounded();
  };
  if (message.kind === "snapshot") {
    byteField(0, message.pack);
    byteField(1, message.spr);
  } else if (message.kind === "mutations") {
    byteField(0, message.envelopes);
  } else {
    out.push(0, PACK_TAG_LIST);
    writeVarintU64(out, message.opIds.length);
    for (const value of message.opIds) { packEncodeString(value, symbolIndex, out); bounded(); }
  }
  bounded();
  return new Uint8Array(out);
}

/** 📥️ Bounded exact record decoding, with canonical bytes checked before admission. */
export function decodeBackboneMessage(bytes: Uint8Array): BinaryBackboneMessage {
  if (!(bytes instanceof Uint8Array) || bytes.length === 0 || bytes.length > BACKBONE_SNAPSHOT_MAXIMUM_BYTES) throw new Error("backbone message: byte limit");
  if (bytes[0] !== 1) throw new Error("backbone message: invalid format");
  const pos: [number] = [1];
  const natural = (maximum: number) => {
    const start = pos[0];
    const value = packReadVarintBigInt(bytes, pos);
    if (value > BigInt(maximum) || (pos[0] - start > 1 && bytes[pos[0] - 1] === 0)) throw new Error("backbone message: invalid count");
    return Number(value);
  };
  const tag = natural(2);
  if (tag !== 0 && bytes.length > BACKBONE_HOT_MESSAGE_MAXIMUM_BYTES) throw new Error("backbone message: hot byte limit");
  const take = (length: number) => {
    if (length > bytes.length - pos[0]) throw new Error("backbone message: truncated bytes");
    const value = bytes.subarray(pos[0], pos[0] + length);
    pos[0] += length;
    return value;
  };
  const expectTag = (expected: number) => {
    if (take(1)[0] !== expected) throw new Error("backbone message: unexpected tag");
  };
  const text = () => new TextDecoder("utf-8", { fatal: true }).decode(take(natural(bytes.length - pos[0])));
  const symbols: string[] = [];
  const symbolCount = natural(bytes.length - pos[0]);
  for (let index = 0; index < symbolCount; index++) symbols.push(text());
  if (natural(2) !== (tag === 0 ? 2 : 1)) throw new Error("backbone message: field count");
  const byteField = (id: number) => {
    if (natural(1) !== id) throw new Error("backbone message: field id");
    expectTag(0x08);
    return take(natural(bytes.length - pos[0])).slice();
  };
  let message: BinaryBackboneMessage;
  if (tag === 0) {
    message = { kind: "snapshot", pack: byteField(0), spr: byteField(1) };
  } else if (tag === 1) {
    message = { kind: "mutations", envelopes: byteField(0) };
  } else {
    if (natural(0) !== 0) throw new Error("backbone message: field id");
    expectTag(PACK_TAG_LIST);
    const count = natural(Math.floor((bytes.length - pos[0]) / 2));
    const opIds: string[] = [];
    for (let index = 0; index < count; index++) {
      const stringTag = take(1)[0];
      if (stringTag === PACK_TAG_STR_INLINE) opIds.push(text());
      else if (stringTag === PACK_TAG_STR) {
        const symbol = symbols[natural(symbols.length)];
        if (symbol === undefined) throw new Error("backbone message: symbol reference");
        opIds.push(symbol);
      } else throw new Error("backbone message: string tag");
    }
    message = { kind: "ack", opIds };
  }
  if (pos[0] !== bytes.length) throw new Error("backbone message: trailing bytes");
  const canonical = encodeBackboneMessage(message);
  if (canonical.length !== bytes.length || canonical.some((value, index) => value !== bytes[index])) throw new Error("backbone message: noncanonical record");
  return message;
}

export type DocumentBackboneMessage = Readonly<{ message: Uint8Array; envelopes: readonly ExactWireMutationEnvelope[] }>;

/** 🪢 Admits one canonical hot mutation message and its exact bounded causal batch. */
export function parseDocumentBackboneMessage(message: Uint8Array): DocumentBackboneMessage {
  if (!(message instanceof Uint8Array) || message.length > BACKBONE_HOT_MESSAGE_MAXIMUM_BYTES) throw new Error("document backbone: hot byte limit");
  const parsed = decodeBackboneMessage(message);
  if (parsed.kind !== "mutations") throw new Error("document backbone: mutations required");
  const envelopes = decodeDocumentBackboneEnvelopeBatchExact(parsed.envelopes);
  return { message: message.slice(), envelopes };
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
 * browser blob cache (`🧵️backbone-worker.ts`) so all three stay in sync on the same literal. Backed by
 * `vcs::BlobStore`'s native counterpart; a hub-backed route is a later ticket. */
export const BLOB_ENDPOINT_PATH = "/semio-blob";
//#endregion 🔖️Blob

//#region 🔖️BackboneWorkerProtocol
/** 🪪️ Hub documents compare a plan against the complete {@link DocumentExecutionTargetLeaseFieldsV1}
 * of a locally installed execution target — catalog generation, both scope ids, descriptor digest,
 * every package/component/descriptor digest with byte lengths, artifact, parent dialect, surface,
 * grant, checkpoint and revalidation. A non-`react` renderer target is admitted only when the
 * worker owns a live private lease that verified those exact bytes. */
export type { DirectoryCommandErrorCodeV1, DirectoryCommandOutcomeV1, DirectoryCommandReceiptV1, DirectoryCommandRequestV1, DirectoryCommandResultV1, DocumentExecutionTargetLeaseFieldsV1, DocumentExecutionTargetProgressV1, DocumentExecutionTargetStatusCodeV1 } from "./🔨️modules/📇️directory/🧬️schema/🟦️.ts";
export { DIRECTORY_COMMAND_RECEIPT_MAX_BYTES, DIRECTORY_COMMAND_REQUEST_MAX_BYTES, directoryCommandErrorIsTransient, directoryCommandSha256, parseDirectoryCommandReceiptV1, parseDirectoryCommandRequestV1, sealDirectoryCommandReceiptV1, sealDirectoryCommandRequestV1 } from "./🔨️modules/📇️directory/🧬️schema/🟦️.ts";
export { DOCUMENT_EXECUTION_TARGET_COMPONENT_MAX_BYTES, DOCUMENT_EXECUTION_TARGET_DESCRIPTOR_MAX_BYTES, DOCUMENT_EXECUTION_TARGET_STATUS_TEXT_V1, documentExecutionTargetStatusRoleV1, leaseFieldsFromPlanV1, parseDocumentExecutionTargetLeaseFieldsV1, sameLeaseFieldsV1 } from "./🔨️modules/📇️directory/🧬️schema/🟦️.ts";
/** 💡️ The host-owned ephemeral GIS Map inference port: its closed wire DTOs, its nine-phase state
 * machine, and its explicit EN/DE vocabulary. Nothing here is ever persisted into a document. */
export type {
  GisMapApprovalUndoHandleV1,
  GisMapApprovalUndoReceiptV1,
  GisMapApprovalUndoRequestV1,
  GisMapInferenceApprovalReceiptV1,
  GisMapInferenceApprovalRequestV1,
  GisMapInferenceEventPageV1,
  GisMapInferenceEventV1,
  GisMapInferenceJobReceiptV1,
  GisMapInferenceJobRequestV1,
  GisMapInferenceJobStateV1,
  GisMapInferencePortCodeV1,
  GisMapInferencePortEventV1,
  GisMapInferencePortPhaseV1,
  GisMapInferencePortStatusV1,
  GisMapInferencePreviewV1,
  GisMapInferenceProgressV1,
  GisMapInferenceProposalStateV1,
} from "./🔨️modules/📇️directory/🧬️schema/🟦️.ts";
export {
  GIS_MAP_INFERENCE_EVENT_PAGE_MAX_ITEMS,
  GIS_MAP_INFERENCE_JOB_MAX_LIFETIME_MS,
  GIS_MAP_INFERENCE_PORT_CODE_TEXT_V1,
  GIS_MAP_INFERENCE_PORT_CONTROL_TEXT_V1,
  GIS_MAP_INFERENCE_PORT_TEXT_V1,
  GIS_MAP_INFERENCE_PROGRESS_MAX_CURSOR,
  GIS_MAP_INFERENCE_REQUEST_MAX_BYTES,
  GIS_MAP_INFERENCE_RESPONSE_MAX_BYTES,
  GIS_MAP_INFERENCE_SERVICE_ID,
  gisMapInferenceCodeFromStatusV1,
  gisMapInferencePortRoleV1,
  gisMapInferencePortTerminalV1,
  idleGisMapInferencePortStatusV1,
  parseGisMapInferenceApprovalReceiptV1,
  parseGisMapApprovalUndoReceiptV1,
  parseGisMapApprovalUndoRequestV1,
  parseGisMapInferenceEventPageV1,
  parseGisMapInferenceJobReceiptV1,
  parseGisMapInferencePreviewV1,
  parseGisMapInferencePortStatusV1,
  reduceGisMapInferencePortV1,
  sealGisMapApprovalUndoRequestV1,
  sealGisMapInferenceApprovalRequestV1,
  sealGisMapInferenceJobRequestV1,
} from "./🔨️modules/📇️directory/🧬️schema/🟦️.ts";

export type DocumentRuntimeScopeV1 = Readonly<{ kind: "hub"; spaceId: string; documentId: string }> | Readonly<{ kind: "local"; documentId: string }>;

/** 🔑️ Canonical collision-free document runtime ownership key. Hub documents are keyed by their
 * complete authority scope; local documents occupy an explicitly separate namespace. */
export function documentRuntimeKeyV1(scope: DocumentRuntimeScopeV1): string {
  const encodeId = (value: string): Uint8Array => {
    const bytes = new TextEncoder().encode(value);
    if (bytes.length === 0 || bytes.length > 256 || /[\u0000-\u001f\u007f]/u.test(value)) throw new Error("document runtime scope: invalid id");
    return bytes;
  };
  const documentBytes = encodeId(scope.documentId);
  if (scope.kind === "local") return `local:v1:${documentBytes.length}:${scope.documentId}`;
  const spaceBytes = encodeId(scope.spaceId);
  return `v1:${spaceBytes.length}:${documentBytes.length}:${scope.spaceId}${scope.documentId}`;
}

export type PersistenceBinding =
  | { readonly kind: "folder"; readonly path: string }
  /** 🪪️ A hub binding states which surface it would like (`requestedSurfaceId`) and, when a previous
   * verified installation is already known, the complete {@link DocumentExecutionTargetLeaseFieldsV1}
   * to compare the next plan against. Neither is byte ownership: a non-`react` renderer target is
   * admitted only through a live private lease minted from server-verified bytes. */
  | { readonly kind: "hub"; readonly baseUrl: string; readonly spaceId: string; readonly requestedSurfaceId?: string; readonly installedTarget?: DocumentExecutionTargetLeaseFieldsV1 };

/** 🧾️ Everything the worker needs to open one artifact's actor — mirrors `ArtifactActorConfig`. */
export type ArtifactActorConfig = {
  readonly documentId: string;
  readonly schema: string;
  readonly bindings: readonly PersistenceBinding[];
  readonly watchExternal?: boolean;
  readonly actor: string;
  /** 🧬️ W5.7: this document kind's `store::DocumentCodec.pack_schema_hash`, for hub schema-hash
   * validation (`ClientFrame::SocketHelloV1.pack_schema_hash`) — the shell fills this from the wasm
   * renderer's `document_pack_schema_hash(schema)` export before calling `openArtifact`. Omitted
   * (or all-zero) means "schema-agnostic client", which the hub never validates. */
  readonly packSchemaHash?: readonly number[];
};

/** 📨️ Caller→actor control messages — mirrors Rust `ArtifactActorMsg`. */
export type ArtifactActorMsg =
  | { readonly kind: "localMutations"; readonly envelopes: readonly MutationEnvelope[] }
  | { readonly kind: "documentBackbone"; readonly message: Uint8Array }
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
  | { readonly kind: "documentBackbone"; readonly message: Uint8Array }
  | { readonly kind: "snapshotReplaced"; readonly pack: readonly number[]; readonly spr: readonly number[] }
  | ({ readonly kind: "status" } & ArtifactSyncStatus)
  | { readonly kind: "presence"; readonly peers: readonly ArtifactPresencePeer[] }
  /** 🎨️ The hub's one-time session color assignment (`ServerFrame::Session`) — mirrors Rust
   * `ArtifactEvent::Session`. Flows through the SAME generic `{kind:"event",documentId,event}`
   * wrapping every other `ArtifactEvent` variant gets (both the real wasm `👷️worker/🦀️.rs`
   * host, which wraps every `ArtifactEvent` uniformly with zero per-variant special-casing, and this
   * file's TS fallback via `emitEvent`), never a separate top-level `BackboneWorkerResponse` member. */
  | { readonly kind: "session"; readonly actor: string; readonly color: number }
  | { readonly kind: "preview"; readonly actor: string; readonly key: string; readonly seq: number; readonly payload: readonly number[] }
  | { readonly kind: "commandOutcome"; readonly batchId: number; readonly outcome: CommandAckOutcome }
  | ({ readonly kind: "conflict" } & SyncConflict);

/** 📤️ Main thread → `🧵️backbone-worker.ts` — `bytes` is a UTF-8 worker wire payload (see {@link encodeBackboneWorkerRequest}). */
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
  if (parsed.kind === "inference-open") return parseInferencePortOpeningRequestV1(parsed);
  if (parsed.kind === "browser-actor-action") {
    const clientInstanceId = workerWireClientInstanceIdV1(parsed.clientInstanceId);
    if (clientInstanceId === null) throw new Error("backbone worker request: invalid client instance id");
    const { clientInstanceId: _client, ...request } = parsed;
    return { ...parseBrowserActorActionRequestV1(request), clientInstanceId };
  }
  if (parsed.kind === "browser-actor-view-state") return parseBrowserActorViewStateRequest(parsed);
  if (parsed.kind === "space-artifact-create" || parsed.kind === "space-artifact-create-cancel") {
    return parseSpaceArtifactCreationWorkerRequestV1(parsed);
  }
  if (parsed.kind === "space-artifact-creation-catalog-open") {
    if (Object.keys(parsed).sort().join(",") !== "clientInstanceId,kind,spaceId") throw new Error("backbone worker request: invalid creation catalog fields");
    const spaceId = workerWireCreationIdentityV1(parsed.spaceId),
      clientInstanceId = workerWireClientInstanceIdV1(parsed.clientInstanceId);
    if (spaceId === null || clientInstanceId === null) throw new Error("backbone worker request: invalid creation catalog scope");
    return { kind: "space-artifact-creation-catalog-open", clientInstanceId, spaceId };
  }
  if (parsed.kind === "browser-actor-ui-patch-result") {
    const clientInstanceId = workerWireClientInstanceIdV1(parsed.clientInstanceId);
    if (clientInstanceId === null) throw new Error("backbone worker request: invalid client instance id");
    const { clientInstanceId: _clientInstanceId, ...result } = parsed;
    return { ...parseBrowserActorUiPatchResultV1(result), clientInstanceId };
  }
  if (parsed.kind === "send" && typeof parsed.message === "object" && parsed.message !== null) {
    const clientInstanceId = parsed.clientInstanceId === undefined ? undefined : workerWireClientInstanceIdV1(parsed.clientInstanceId);
    if (clientInstanceId === null) throw new Error("backbone worker request: invalid client instance id");
    const message = parseArtifactActorMsg(parsed.message as Record<string, unknown>);
    if (message.kind === "documentBackbone") {
      const fields = ["kind", "documentId", "clientInstanceId", "message", ...(parsed.spaceId === undefined ? [] : ["spaceId"])];
      const documentId = workerWireIdV1(parsed.documentId);
      const spaceId = parsed.spaceId === undefined ? undefined : workerWireIdV1(parsed.spaceId);
      if (Object.keys(parsed).sort().join(",") !== fields.sort().join(",") || documentId === null || clientInstanceId === undefined || spaceId === null) throw new Error("backbone worker request: invalid document backbone owner");
      return { kind: "send", documentId, clientInstanceId, ...(spaceId === undefined ? {} : { spaceId }), message };
    }
    return {
      kind: "send",
      documentId: String(parsed.documentId),
      ...(typeof parsed.spaceId === "string" ? { spaceId: parsed.spaceId } : {}),
      ...(clientInstanceId === undefined ? {} : { clientInstanceId }),
      message,
    };
  }
  if (parsed.kind === "open" || parsed.kind === "close") {
    const clientInstanceId = parsed.clientInstanceId === undefined ? undefined : workerWireClientInstanceIdV1(parsed.clientInstanceId);
    if (parsed.clientInstanceId !== undefined && clientInstanceId === null) throw new Error("backbone worker request: invalid client instance id");
  }
  if (parsed.kind === "inference-history-undo") {
    const clientInstanceId = workerWireClientInstanceIdV1(parsed.clientInstanceId);
    if (clientInstanceId === null || !Number.isSafeInteger(parsed.historyEpoch) || (parsed.historyEpoch as number) < 1) throw new Error("backbone worker request: invalid inference history owner");
    if (typeof parsed.scope !== "object" || parsed.scope === null || Array.isArray(parsed.scope)) throw new Error("backbone worker request: invalid inference history scope");
    const documentId = workerWireIdV1((parsed.scope as Record<string, unknown>).documentId);
    const scope = documentId === null ? null : workerWireScopeV1(parsed.scope, documentId);
    if (scope === null) throw new Error("backbone worker request: invalid inference history scope");
    return { kind: "inference-history-undo", historyEpoch: parsed.historyEpoch as number, clientInstanceId, scope };
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
  if (parsed.kind === "browser-actor-action-result") {
    const clientInstanceId = workerWireClientInstanceIdV1(parsed.clientInstanceId);
    if (clientInstanceId === null) throw new Error("backbone worker response: invalid client instance id");
    const { clientInstanceId: _client, ...response } = parsed;
    return { ...parseBrowserActorActionResultV1(response), clientInstanceId };
  }
  if (parsed.kind === "space-artifact-creation-status") return parseSpaceArtifactCreationStatusV1(parsed);
  if (parsed.kind === "space-artifact-creation-catalog") return parseSpaceArtifactCreationCatalogV1(parsed);
  if (parsed.kind === "space-artifact-creation-catalog-status") return parseSpaceArtifactCreationCatalogStatusV1(parsed);
  if (parsed.kind === "browser-actor-ui-patch") {
    const clientInstanceId = workerWireClientInstanceIdV1(parsed.clientInstanceId);
    if (clientInstanceId === null) throw new Error("backbone worker response: invalid client instance id");
    const { clientInstanceId: _clientInstanceId, ...offer } = parsed;
    return { ...parseBrowserActorUiPatchOfferV1(offer), clientInstanceId };
  }
  if (parsed.kind === "browser-actor-ui-mounted") {
    const fields = ["activationGeneration", "activeCheckpointId", "browserActorSha256", "catalogGenerationId", "clientInstanceId", "componentSha256", "descriptorDigestV1", "descriptorSha256", "frontier", "instanceId", "kind", "scope", "uiRevision", "verifiedSurfaceId"];
    if (Object.keys(parsed).sort().join(",") !== fields.sort().join(",")) throw new Error("backbone worker response: invalid mounted UI fields");
    const scopeRow = parsed.scope as Record<string, unknown> | undefined;
    const documentId = scopeRow === undefined ? null : workerWireIdV1(scopeRow.documentId);
    const scope = documentId === null ? null : workerWireScopeV1(parsed.scope, documentId);
    const clientInstanceId = workerWireClientInstanceIdV1(parsed.clientInstanceId);
    const verifiedSurfaceId = workerWireIdV1(parsed.verifiedSurfaceId);
    const activationGeneration = typeof parsed.activationGeneration === "string" && /^[1-9][0-9]{0,19}$/u.test(parsed.activationGeneration) && BigInt(parsed.activationGeneration) <= 0xffffffffffffffffn ? parsed.activationGeneration : null;
    if (scope === null || clientInstanceId === null || verifiedSurfaceId === null || activationGeneration === null || !Number.isSafeInteger(parsed.instanceId) || (parsed.instanceId as number) < 0 || !Number.isSafeInteger(parsed.uiRevision) || (parsed.uiRevision as number) < 1) throw new Error("backbone worker response: invalid mounted UI owner");
    const catalogGenerationId = workerWireSha256V1(parsed.catalogGenerationId),
      componentSha256 = workerWireSha256V1(parsed.componentSha256),
      descriptorSha256 = workerWireSha256V1(parsed.descriptorSha256),
      browserActorSha256 = workerWireSha256V1(parsed.browserActorSha256),
      activeCheckpointId = workerWireSha256V1(parsed.activeCheckpointId),
      descriptorDigestV1 = workerWireSha256V1(parsed.descriptorDigestV1),
      frontier = workerWireArtifactFrontierV1(parsed.frontier, scope);
    if (catalogGenerationId === null || componentSha256 === null || descriptorSha256 === null || browserActorSha256 === null || activeCheckpointId === null || descriptorDigestV1 === null || frontier === null) throw new Error("backbone worker response: invalid mounted UI identity");
    return { kind: "browser-actor-ui-mounted", scope, clientInstanceId, activationGeneration, instanceId: parsed.instanceId as number, verifiedSurfaceId, catalogGenerationId, componentSha256, descriptorSha256, browserActorSha256, activeCheckpointId, descriptorDigestV1, frontier, uiRevision: parsed.uiRevision as number };
  }
  if (parsed.kind === "event" && typeof parsed.event === "object" && parsed.event !== null) {
    const documentId = workerWireIdV1(parsed.documentId);
    if (documentId === null) throw new Error("backbone worker response: invalid document id");
    const clientInstanceId = workerWireClientInstanceIdV1(parsed.clientInstanceId);
    if (clientInstanceId === null) throw new Error("backbone worker response: invalid client instance id");
    const event = parseArtifactEvent(parsed.event as Record<string, unknown>);
    const scope = workerWireScopeV1(parsed.scope, documentId);
    if (event.kind === "documentBackbone") {
      const fields = ["clientInstanceId", "documentId", "event", "kind", ...(parsed.scope === undefined ? [] : ["scope"])];
      if (Object.keys(parsed).sort().join(",") !== fields.sort().join(",") || parsed.verifiedSurfaceId !== undefined || (parsed.scope !== undefined && scope === null)) throw new Error("backbone worker response: invalid document backbone owner");
      return { kind: "event", documentId, clientInstanceId, event, ...(scope === null ? {} : { scope }) };
    }
    if (event.kind !== "presence") return { kind: "event", documentId, clientInstanceId, event, ...(scope === null ? {} : { scope }) };
    const verifiedSurfaceId = workerWireIdV1(parsed.verifiedSurfaceId);
    if (scope === null || verifiedSurfaceId === null) return { kind: "event", documentId, clientInstanceId, event: { kind: "presence", peers: [] }, ...(scope === null ? {} : { scope }) };
    return { kind: "event", documentId, clientInstanceId, scope, verifiedSurfaceId, event };
  }
  if (
    parsed.kind === "socket-actor"
    || parsed.kind === "socket-actor-failed"
    || parsed.kind === "artifact-bootstrap-progress"
    || parsed.kind === "artifact-bootstrap-failed"
    || parsed.kind === "artifact-rebootstrap-required"
    || parsed.kind === "execution-target-status"
  ) {
    const documentId = workerWireIdV1(parsed.documentId);
    if (documentId === null) throw new Error("backbone worker response: invalid document id");
    const clientInstanceId = workerWireClientInstanceIdV1(parsed.clientInstanceId);
    if (clientInstanceId === null) throw new Error("backbone worker response: invalid client instance id");
    let scope = workerWireScopeV1(parsed.scope, documentId);
    if (parsed.kind === "execution-target-status" && scope !== null && parsed.spaceId !== scope.spaceId) scope = null;
    const { documentId: _documentId, clientInstanceId: _clientInstanceId, scope: _scope, ...rest } = parsed;
    return { ...rest, documentId, clientInstanceId, ...(scope === null ? {} : { scope }) } as BackboneWorkerResponse;
  }
  if (parsed.kind === "inference-port-status") {
    if (!Number.isSafeInteger(parsed.operationEpoch) || (parsed.operationEpoch as number) < 0 || typeof parsed.scope !== "object" || parsed.scope === null || Array.isArray(parsed.scope)) throw new Error("backbone worker response: invalid inference owner");
    const scopeRow = parsed.scope as Record<string, unknown>;
    const documentId = workerWireIdV1(scopeRow.documentId);
    if (documentId === null) throw new Error("backbone worker response: invalid inference document id");
    const scope = workerWireScopeV1(parsed.scope, documentId);
    if (scope === null) throw new Error("backbone worker response: invalid inference scope");
    return { kind: "inference-port-status", operationEpoch: parsed.operationEpoch as number, scope, status: parseGisMapInferencePortStatusV1(parsed.status) };
  }
  if (parsed.kind === "inference-port-opened") return parseInferencePortOpeningResultV1(parsed);
  if (parsed.kind === "inference-port-closed") return parseInferencePortClosedV1(parsed);
  if (parsed.kind === "inference-history-status") {
    const clientInstanceId = workerWireClientInstanceIdV1(parsed.clientInstanceId);
    if (clientInstanceId === null || !Number.isSafeInteger(parsed.historyEpoch) || (parsed.historyEpoch as number) < 1) throw new Error("backbone worker response: invalid inference history owner");
    if (typeof parsed.scope !== "object" || parsed.scope === null || Array.isArray(parsed.scope)) throw new Error("backbone worker response: invalid inference history scope");
    const documentId = workerWireIdV1((parsed.scope as Record<string, unknown>).documentId);
    const scope = documentId === null ? null : workerWireScopeV1(parsed.scope, documentId);
    if (scope === null) throw new Error("backbone worker response: invalid inference history scope");
    return { kind: "inference-history-status", historyEpoch: parsed.historyEpoch as number, clientInstanceId, scope, status: parseGisMapApprovalHistoryStatusV1(parsed.status) };
  }
  return parsed as BackboneWorkerResponse;
}

function workerWireIdV1(value: unknown): string | null {
  if (typeof value !== "string" || value.length === 0 || new TextEncoder().encode(value).length > 256 || /[\u0000-\u001f\u007f]/u.test(value)) return null;
  return value;
}

function workerWireTextV1(value: unknown): string | null {
  return typeof value === "string" && !value.startsWith(" ") && !value.endsWith(" ") && [...value].length <= 128 && value.length > 0 && !/[\u0000-\u001f\u007f-\u009f]/u.test(value) ? value : null;
}

function workerWireCreationIdentityV1(value: unknown): string | null {
  return typeof value === "string" && value.length <= 256 && /^[A-Za-z0-9][A-Za-z0-9._:/-]*$/u.test(value) ? value : null;
}

function workerWireCreationRequestIdV1(value: unknown): string | null {
  return typeof value === "string" && /^(?!0{32}$)[0-9a-f]{32}$/u.test(value) ? value : null;
}

function parseSpaceArtifactCreationWorkerRequestV1(parsed: Readonly<Record<string, unknown>>): Extract<BackboneWorkerRequest, { readonly kind: "space-artifact-create" | "space-artifact-create-cancel" }> {
  const create = parsed.kind === "space-artifact-create";
  const expected = create ? "expectedCatalogGenerationId,kind,kindId,name,requestId,spaceId" : "kind,requestId,spaceId";
  if (Object.keys(parsed).sort().join(",") !== expected) throw new Error("backbone worker request: invalid space artifact creation fields");
  const requestId = workerWireCreationRequestIdV1(parsed.requestId),
    spaceId = workerWireCreationIdentityV1(parsed.spaceId);
  if (requestId === null || spaceId === null) throw new Error("backbone worker request: invalid space artifact creation owner");
  if (!create) return { kind: "space-artifact-create-cancel", requestId, spaceId };
  const expectedCatalogGenerationId = workerWireSha256V1(parsed.expectedCatalogGenerationId),
    kindId = workerWireCreationIdentityV1(parsed.kindId),
    name = workerWireTextV1(parsed.name);
  if (expectedCatalogGenerationId === null || kindId === null || name === null) throw new Error("backbone worker request: invalid space artifact creation intent");
  return { kind: "space-artifact-create", requestId, spaceId, expectedCatalogGenerationId, kindId, name };
}

/** 🛡️ Validates the exact worker-visible projection of the Hub's durable creation status. */
export function parseSpaceArtifactCreationStatusV1(value: unknown): SpaceArtifactCreationStatusV1 {
  if (typeof value !== "object" || value === null || Array.isArray(value)) throw new Error("space artifact creation status: invalid record");
  const row = value as Readonly<Record<string, unknown>>;
  const phases: readonly SpaceArtifactCreationPhaseV1[] = ["accepted", "preparing", "ready", "indeterminate", "failed", "cancelled"];
  const phase = phases.includes(row.phase as SpaceArtifactCreationPhaseV1) ? (row.phase as SpaceArtifactCreationPhaseV1) : null;
  const requestId = workerWireCreationRequestIdV1(row.requestId),
    spaceId = workerWireCreationIdentityV1(row.spaceId),
    catalogGenerationId = workerWireSha256V1(row.catalogGenerationId);
  if (row.kind !== "space-artifact-creation-status" || phase === null || requestId === null || spaceId === null || catalogGenerationId === null) throw new Error("space artifact creation status: invalid owner");
  const expected = phase === "ready" ? "catalogGenerationId,kind,phase,ready,requestId,spaceId" : "catalogGenerationId,kind,phase,requestId,spaceId";
  if (Object.keys(row).sort().join(",") !== expected) throw new Error("space artifact creation status: invalid fields");
  if (phase !== "ready") return { kind: "space-artifact-creation-status", requestId, spaceId, catalogGenerationId, phase };
  if (typeof row.ready !== "object" || row.ready === null || Array.isArray(row.ready)) throw new Error("space artifact creation status: invalid ready record");
  const ready = row.ready as Readonly<Record<string, unknown>>;
  if (Object.keys(ready).sort().join(",") !== "artifactSchema,documentId,kindId,parentDialect") throw new Error("space artifact creation status: invalid ready fields");
  if (typeof ready.parentDialect !== "object" || ready.parentDialect === null || Array.isArray(ready.parentDialect)) throw new Error("space artifact creation status: invalid parent dialect");
  const parentDialect = ready.parentDialect as Readonly<Record<string, unknown>>;
  if (Object.keys(parentDialect).sort().join(",") !== "artifactKind,standard,subset") throw new Error("space artifact creation status: invalid parent dialect fields");
  const documentId = typeof ready.documentId === "string" && /^artifact-(?!0{32}$)[0-9a-f]{32}$/u.test(ready.documentId) ? ready.documentId : null,
    kindId = workerWireCreationIdentityV1(ready.kindId),
    artifactSchema = workerWireCreationIdentityV1(ready.artifactSchema),
    artifactKind = workerWireCreationIdentityV1(parentDialect.artifactKind),
    standard = workerWireCreationIdentityV1(parentDialect.standard),
    subset = workerWireCreationIdentityV1(parentDialect.subset);
  if (documentId === null || kindId === null || artifactSchema === null || artifactKind === null || standard === null || subset === null || artifactKind !== kindId) throw new Error("space artifact creation status: invalid ready identity");
  return { kind: "space-artifact-creation-status", requestId, spaceId, catalogGenerationId, phase, ready: { documentId, kindId, artifactSchema, parentDialect: { artifactKind, standard, subset } } };
}

/** 🗂️ Validates the presentation-only selected-current creation catalog from the worker. */
export function parseSpaceArtifactCreationCatalogV1(value: unknown): SpaceArtifactCreationCatalogV1 {
  if (typeof value !== "object" || value === null || Array.isArray(value)) throw new Error("space artifact creation catalog: invalid record");
  const row = value as Readonly<Record<string, unknown>>;
  if (Object.keys(row).sort().join(",") !== "catalogGenerationId,clientInstanceId,kind,kinds,spaceId") throw new Error("space artifact creation catalog: invalid fields");
  const spaceId = workerWireCreationIdentityV1(row.spaceId),
    clientInstanceId = workerWireClientInstanceIdV1(row.clientInstanceId);
  if (row.kind !== "space-artifact-creation-catalog" || spaceId === null || clientInstanceId === null || typeof row.catalogGenerationId !== "string" || !/^(?!0{64}$)[0-9a-f]{64}$/u.test(row.catalogGenerationId) || !Array.isArray(row.kinds) || row.kinds.length === 0 || row.kinds.length > 64)
    throw new Error("space artifact creation catalog: invalid owner");
  const kinds = row.kinds.map((value) => {
    if (typeof value !== "object" || value === null || Array.isArray(value)) throw new Error("space artifact creation catalog: invalid kind");
    const kind = value as Readonly<Record<string, unknown>>;
    if (Object.keys(kind).sort().join(",") !== "dialect,kindId,label,schema" || typeof kind.dialect !== "object" || kind.dialect === null || Array.isArray(kind.dialect) || typeof kind.label !== "object" || kind.label === null || Array.isArray(kind.label))
      throw new Error("space artifact creation catalog: invalid kind fields");
    const dialect = kind.dialect as Readonly<Record<string, unknown>>,
      label = kind.label as Readonly<Record<string, unknown>>;
    if (Object.keys(dialect).sort().join(",") !== "artifactKind,standard,subset" || Object.keys(label).sort().join(",") !== "de,en") throw new Error("space artifact creation catalog: invalid nested fields");
    const kindId = workerWireCreationIdentityV1(kind.kindId),
      schema = workerWireCreationIdentityV1(kind.schema),
      artifactKind = workerWireCreationIdentityV1(dialect.artifactKind),
      standard = workerWireCreationIdentityV1(dialect.standard),
      subset = workerWireCreationIdentityV1(dialect.subset),
      en = workerWireTextV1(label.en),
      de = workerWireTextV1(label.de);
    if (kindId === null || schema === null || artifactKind !== kindId || standard === null || subset === null || en === null || de === null) throw new Error("space artifact creation catalog: invalid kind identity");
    return { kindId, schema, dialect: { artifactKind, standard, subset }, label: { en, de } };
  });
  if (kinds.some((entry, index) => index > 0 && kinds[index - 1]!.kindId >= entry.kindId)) throw new Error("space artifact creation catalog: invalid order");
  return { kind: "space-artifact-creation-catalog", clientInstanceId, spaceId, catalogGenerationId: row.catalogGenerationId, kinds };
}

/** 🗂️ Validates a catalog presentation update without granting any catalog contents. */
export function parseSpaceArtifactCreationCatalogStatusV1(value: unknown): SpaceArtifactCreationCatalogStatusV1 {
  if (typeof value !== "object" || value === null || Array.isArray(value)) throw new Error("space artifact creation catalog status: invalid record");
  const row = value as Readonly<Record<string, unknown>>;
  if (Object.keys(row).sort().join(",") !== "clientInstanceId,kind,phase,spaceId") throw new Error("space artifact creation catalog status: invalid fields");
  const clientInstanceId = workerWireClientInstanceIdV1(row.clientInstanceId),
    spaceId = workerWireCreationIdentityV1(row.spaceId),
    phase = row.phase === "loading" || row.phase === "ready" || row.phase === "unavailable" ? row.phase : null;
  if (row.kind !== "space-artifact-creation-catalog-status" || clientInstanceId === null || spaceId === null || phase === null) throw new Error("space artifact creation catalog status: invalid owner");
  return { kind: "space-artifact-creation-catalog-status", clientInstanceId, spaceId, phase };
}

function workerWireClientInstanceIdV1(value: unknown): string | null {
  const id = workerWireIdV1(value);
  return id !== null && /^[0-9a-f]{8}-[0-9a-f]{4}-4[0-9a-f]{3}-[89ab][0-9a-f]{3}-[0-9a-f]{12}$/u.test(id) ? id : null;
}

function workerWireSha256V1(value: unknown): string | null {
  return typeof value === "string" && /^[0-9a-f]{64}$/u.test(value) ? value : null;
}

function workerWireArtifactFrontierV1(value: unknown, scope: DocumentScope): ArtifactFrontier | null {
  if (typeof value !== "object" || value === null || Array.isArray(value)) return null;
  const row = value as Record<string, unknown>;
  if (Object.keys(row).sort().join(",") !== "chainHash,documentId,headEditId,headEditOrdinal,lastCommitSeq") return null;
  const chainHash = row.chainHash;
  if (!Array.isArray(chainHash) || chainHash.length !== 32 || chainHash.some(byte => !Number.isInteger(byte) || (byte as number) < 0 || (byte as number) > 255)) return null;
  if (typeof row.documentId !== "string" || typeof row.headEditId !== "string" || !Number.isSafeInteger(row.headEditOrdinal) || !Number.isSafeInteger(row.lastCommitSeq)) return null;
  const frontier: ArtifactFrontier = { documentId: row.documentId, headEditOrdinal: row.headEditOrdinal as number, headEditId: row.headEditId, lastCommitSeq: row.lastCommitSeq as number, chainHash: chainHash as number[] };
  return (artifactFrontierIsGenesisForV1(scope, frontier) || artifactFrontierIsEditedForV1(scope, frontier)) && frontier.lastCommitSeq <= frontier.headEditOrdinal ? frontier : null;
}

function workerWireScopeV1(value: unknown, documentId: string): DocumentScope | null {
  if (typeof value !== "object" || value === null || Array.isArray(value)) return null;
  const row = value as Record<string, unknown>;
  if (Object.keys(row).sort().join(",") !== "documentId,spaceId") return null;
  const spaceId = workerWireIdV1(row.spaceId);
  const scopedDocumentId = workerWireIdV1(row.documentId);
  return spaceId === null || scopedDocumentId === null || scopedDocumentId !== documentId ? null : { spaceId, documentId };
}

/** 🏛️ Closed lifecycle of the one shell-owned retained space-administration operation.
 * `receipt` is only ever reached through an exact accepted server receipt; every terminal phase
 * has already erased the page, the receipt, and any invite capability. */
export type DirectoryAdministrationPhaseV1 =
  | "loading"
  | "ready"
  | "submitting"
  | "receipt"
  | "refreshing"
  | "cancelled"
  | "denied"
  | "stale"
  | "failed";

/** 🗂️ The one independently paged administration window a cursor may advance. */
export type DirectoryAdministrationSectionV1 = "members" | "invites" | "documents";

/** 📋️ Renderer-visible state of the worker-retained invite capability. */
export type DirectoryAdministrationInviteCapabilityStatusV1 = "available" | "copying" | "failed";

/** 🌱️ One server-owned artifact creation's durable, non-CRUD lifecycle. Only `ready` may
 * disclose the server-minted document tuple that the Shell can pass to its ordinary open path. */
export type SpaceArtifactCreationPhaseV1 = "accepted" | "preparing" | "ready" | "indeterminate" | "failed" | "cancelled";

/** 🔐️ The exact public artifact identity produced by a ready creation saga. Catalog authority,
 * descriptors, grants and filesystem details remain server-private and are re-resolved by open-plan. */
export type SpaceArtifactCreationReadyV1 = Readonly<{
  documentId: string;
  kindId: string;
  artifactSchema: string;
  parentDialect: ArtifactDialect;
}>;

/** 📡️ Renderer-visible status for one exact request id and Space. */
export type SpaceArtifactCreationStatusV1 = Readonly<{
  kind: "space-artifact-creation-status";
  requestId: string;
  spaceId: string;
  catalogGenerationId: string;
  phase: SpaceArtifactCreationPhaseV1;
  ready?: SpaceArtifactCreationReadyV1;
}>;

/** 🗣️ One selected descriptor's exact bilingual creation presentation. */
export type SpaceArtifactCreationKindV1 = Readonly<{
  kindId: string;
  schema: string;
  dialect: ArtifactDialect;
  label: Readonly<{ en: string; de: string }>;
}>;

/** 🗂️ Current trusted creation choices for one authenticated Space. */
export type SpaceArtifactCreationCatalogV1 = Readonly<{
  kind: "space-artifact-creation-catalog";
  clientInstanceId: string;
  spaceId: string;
  catalogGenerationId: string;
  kinds: readonly SpaceArtifactCreationKindV1[];
}>;

export type SpaceArtifactCreationCatalogPhaseV1 = "loading" | "ready" | "unavailable";

/** 🗂️ Exact mounted-Space presentation state for the selected creation catalog. */
export type SpaceArtifactCreationCatalogStatusV1 = Readonly<{
  kind: "space-artifact-creation-catalog-status";
  clientInstanceId: string;
  spaceId: string;
  phase: SpaceArtifactCreationCatalogPhaseV1;
}>;

/** 📤️ Main thread → `🧵️backbone-worker.ts` messages (structured clone or {@link BackboneWorkerWireMessage}).
 * The `directory-*` kinds (contract-freeze §C6) are the shell's ONLY way to reach the directory hub
 * — plugin surfaces never talk to the network, and the shell never opens a directory socket on the
 * UI thread; see `🧵️backbone-worker.ts`'s `🔖️Directory` region. */
export type BackboneWorkerRequest =
  | (BrowserActorActionRequestV1 & { readonly clientInstanceId: string })
  | BrowserActorViewStateRequest
  | ({ readonly kind: "open"; readonly clientInstanceId?: string } & ArtifactActorConfig)
  | { readonly kind: "close"; readonly documentId: string; readonly spaceId?: string; readonly clientInstanceId?: string }
  | { readonly kind: "send"; readonly documentId: string; readonly spaceId?: string; readonly clientInstanceId?: string; readonly message: ArtifactActorMsg }
  | { readonly kind: "directory-open"; readonly baseUrl: string; readonly since: number }
  | { readonly kind: "directory-bootstrap-open"; readonly baseUrl: string; readonly after: number; readonly bootstrapEpoch: number }
  | ({ readonly kind: "directory-bootstrap-ack" } & DirectoryEventPageAckV1)
  | { readonly kind: "directory-bootstrap-reject"; readonly bootstrapEpoch: number; readonly receiptSha256: string }
  | { readonly kind: "directory-bootstrap-close"; readonly bootstrapEpoch: number }
  | { readonly kind: "directory-scope-open"; readonly baseUrl: string; readonly scope: DocumentScope; readonly since: number }
  | { readonly kind: "directory-scope-close"; readonly scope: DocumentScope }
  | { readonly kind: "directory-command"; readonly requestId: string; readonly command: DirectoryCommand }
  | { readonly kind: "directory-command-cancel"; readonly requestId: string }
  | { readonly kind: "space-artifact-creation-catalog-open"; readonly clientInstanceId: string; readonly spaceId: string }
  | { readonly kind: "space-artifact-create"; readonly requestId: string; readonly spaceId: string; readonly expectedCatalogGenerationId: string; readonly kindId: string; readonly name: string }
  | { readonly kind: "space-artifact-create-cancel"; readonly requestId: string; readonly spaceId: string }
  | { readonly kind: "directory-administration-open"; readonly operationEpoch: number; readonly spaceId: string }
  | { readonly kind: "directory-administration-refresh"; readonly operationEpoch: number; readonly cursor?: string }
  | { readonly kind: "directory-administration-submit"; readonly operationEpoch: number; readonly requestId: string; readonly command: DirectoryCommand }
  | { readonly kind: "directory-administration-capability-request"; readonly operationEpoch: number }
  | { readonly kind: "directory-administration-capability-result"; readonly operationEpoch: number; readonly transferEpoch: number; readonly copied: boolean }
  | { readonly kind: "directory-administration-close"; readonly operationEpoch: number }
  | { readonly kind: "directory-close" }
  /** 💡️ The host-owned ephemeral inference port's only transport. Every request names the exact
   * document scope the shell already owns and the operation epoch that owns the port; no bearer,
   * origin, path or receipt ever crosses this boundary, and no request reaches the document socket
   * or any generic document command. */
  | { readonly kind: "inference-open"; readonly operationEpoch: number; readonly scope: DocumentScope }
  | { readonly kind: "inference-propose"; readonly operationEpoch: number; readonly requestId: string }
  | { readonly kind: "inference-poll"; readonly operationEpoch: number }
  | { readonly kind: "inference-cancel"; readonly operationEpoch: number }
  | { readonly kind: "inference-approve"; readonly operationEpoch: number }
  | { readonly kind: "inference-close"; readonly operationEpoch: number }
  /** ↩️ Ordinary Shell history undo names only its current mounted owner. The Hub-minted target,
   * frontier and stable idempotency key remain worker-private. */
  | { readonly kind: "inference-history-undo"; readonly historyEpoch: number; readonly clientInstanceId: string; readonly scope: DocumentScope }
  | (BrowserActorUiPatchResultV1 & { readonly clientInstanceId: string });

/** 🛰️ Worker-local P2-C recovery lifecycle. These are not persisted artifact events: they describe
 * one bounded public bootstrap transfer and therefore remain explicit top-level worker responses. */
export type ArtifactBootstrapWorkerEvent =
  | {
      readonly kind: "artifact-bootstrap-progress";
      readonly documentId: string;
      readonly clientInstanceId: string;
      readonly receivedBytes: number;
      readonly totalBytes: number;
      readonly receivedChunks: number;
      readonly totalChunks: number;
      readonly scope?: DocumentScope;
    }
  | {
      readonly kind: "artifact-bootstrap-failed";
      readonly documentId: string;
      readonly clientInstanceId: string;
      readonly code: "cancelled" | "deadline-exceeded" | "invalid-bootstrap" | "transport-failure";
      readonly message: string;
      readonly retryable: boolean;
      readonly scope?: DocumentScope;
    }
  | {
      readonly kind: "artifact-rebootstrap-required";
      readonly documentId: string;
      readonly clientInstanceId: string;
      readonly message: string;
      readonly retryable: true;
      readonly scope?: DocumentScope;
    };

/** 📥️ `🧵️backbone-worker.ts` → main thread messages. `directory-status.pendingCommands` is the
 * bounded, in-memory offline queue's length (contract-freeze §C6 "commands queue... and flush on
 * reconnect"). */
export type BackboneWorkerResponse =
  | (BrowserActorActionResultV1 & { readonly clientInstanceId: string })
  | { readonly kind: "event"; readonly documentId: string; readonly clientInstanceId: string; readonly event: ArtifactEvent; readonly scope?: DocumentScope; readonly verifiedSurfaceId?: string }
  | ArtifactBootstrapWorkerEvent
  | { readonly kind: "ready" }
  | { readonly kind: "directory-message"; readonly message: DirectoryStreamMessage }
  | ({ readonly kind: "directory-event-page"; readonly canonicalJson: string } & DirectoryEventPageAckV1 & { readonly afterSeqExclusive: number; readonly hasMore: boolean })
  | { readonly kind: "directory-bootstrap-failed"; readonly bootstrapEpoch: number; readonly code: "unauthorized" | "cancelled" | "transport" | "invalid-page"; readonly retryable: boolean }
  | { readonly kind: "directory-scope-revoked"; readonly scope: DocumentScope }
  | { readonly kind: "directory-command-receipt"; readonly requestId: string; readonly receipt: DirectoryCommandReceiptV1 }
  | { readonly kind: "directory-command-failed"; readonly requestId: string; readonly code: DirectoryCommandErrorCodeV1 }
  | SpaceArtifactCreationCatalogV1
  | SpaceArtifactCreationCatalogStatusV1
  | SpaceArtifactCreationStatusV1
  | { readonly kind: "socket-actor"; readonly documentId: string; readonly clientInstanceId: string; readonly scope?: DocumentScope; readonly actorId: string }
  | { readonly kind: "socket-actor-failed"; readonly documentId: string; readonly clientInstanceId: string; readonly scope?: DocumentScope; readonly code: "installed-target-unavailable" | "session-mismatch" }
  /** 🪪️ Bounded execution-target install status for the React host's localized live region. It
   * carries a status code and byte counters only — never bytes, an origin, a path, a module URL, a
   * receipt, a grant or a digest. */
  | { readonly kind: "execution-target-status"; readonly documentId: string; readonly clientInstanceId: string; readonly spaceId: string; readonly scope?: DocumentScope; readonly code: DocumentExecutionTargetStatusCodeV1; readonly progress?: DocumentExecutionTargetProgressV1 }
  /** 🏛️ The complete renderer-visible administration state. `canonicalJson` is the exact page the
   * hub sealed; `inviteCapabilityPending` says a one-shot invite token remains held by the worker
   * until exact clipboard success. No session identity, bearer, or invite token ever appears here. */
  | {
      readonly kind: "directory-administration-state";
      readonly operationEpoch: number;
      readonly spaceId: string;
      readonly phase: DirectoryAdministrationPhaseV1;
      readonly canonicalJson?: string;
      readonly receiptSha256?: string;
      readonly outcome?: DirectoryCommandOutcomeV1;
      readonly code?: DirectoryCommandErrorCodeV1;
      readonly inviteCapabilityPending?: boolean;
      readonly inviteCapabilityStatus?: DirectoryAdministrationInviteCapabilityStatusV1;
    }
  /** 🎁️ One operation-bound clipboard offer. The worker retains the capability until the renderer
   * reports that this exact transfer epoch was copied. */
  | { readonly kind: "directory-administration-capability"; readonly operationEpoch: number; readonly transferEpoch: number; readonly inviteToken: string }
  /** 🛡️ A duplicate or stale request/result was rejected without redisclosing the capability. */
  | { readonly kind: "directory-administration-capability-rejected"; readonly operationEpoch: number; readonly transferEpoch?: number; readonly code: "capacity" | "already-settled" | "mismatch" }
  /** 💡️ The complete renderer-visible state of one document's inference port. It carries the
   * phase, the server's own job id, the bounded progress cursor and the hash the server published —
   * never a receipt, bearer, origin, path, base pack, proposal body or user identity. */
  | { readonly kind: "inference-port-status"; readonly operationEpoch: number; readonly scope: DocumentScope; readonly status: GisMapInferencePortStatusV1 }
  | InferencePortOpeningResultV1
  | InferencePortClosedV1
  | { readonly kind: "inference-history-status"; readonly historyEpoch: number; readonly clientInstanceId: string; readonly scope: DocumentScope; readonly status: GisMapApprovalHistoryStatusV1 }
  /** 🔬️ Public checkpoint/frontier identity for one exact actor UI revision, emitted only after
   * the guest accepts the Shell's transactional patch acknowledgement. It carries no plan, grant,
   * credential, receipt or action. */
  | BrowserActorUiMountedV1
  | (BrowserActorUiPatchOfferV1 & { readonly clientInstanceId: string })
  | { readonly kind: "directory-status"; readonly pendingCommands: number };

export type BrowserActorUiMountedV1 = Readonly<{
  kind: "browser-actor-ui-mounted";
  scope: DocumentScope;
  clientInstanceId: string;
  activationGeneration: string;
  instanceId: number;
  verifiedSurfaceId: string;
  catalogGenerationId: string;
  componentSha256: string;
  descriptorSha256: string;
  browserActorSha256: string;
  activeCheckpointId: string;
  descriptorDigestV1: string;
  frontier: ArtifactFrontier;
  uiRevision: number;
}>;

/** ↩️ Renderer-visible projection of the private durable approval-undo owner. It intentionally
 * carries no job, mutation, command, proposal, target, frontier, bearer or idempotency value. */
export type GisMapApprovalHistoryPhaseV1 = "unavailable" | "available" | "submitting" | "applied" | "failed";
export type GisMapApprovalHistoryStatusV1 = Readonly<{ phase: GisMapApprovalHistoryPhaseV1; canUndo: boolean; code: GisMapInferencePortCodeV1 | null }>;

function parseGisMapApprovalHistoryStatusV1(value: unknown): GisMapApprovalHistoryStatusV1 {
  if (typeof value !== "object" || value === null || Array.isArray(value)) throw new Error("backbone worker: invalid inference history status");
  const row = value as Record<string, unknown>;
  if (Object.keys(row).sort().join(",") !== "canUndo,code,phase") throw new Error("backbone worker: invalid inference history fields");
  const phase = row.phase;
  if (phase !== "unavailable" && phase !== "available" && phase !== "submitting" && phase !== "applied" && phase !== "failed") throw new Error("backbone worker: invalid inference history phase");
  if (typeof row.canUndo !== "boolean" || (phase !== "available" && phase !== "failed" && row.canUndo)) throw new Error("backbone worker: invalid inference history availability");
  if (phase === "available" && !row.canUndo) throw new Error("backbone worker: invalid inference history availability");
  if (row.code !== null && (typeof row.code !== "string" || !(row.code in GIS_MAP_INFERENCE_PORT_CODE_TEXT_V1))) throw new Error("backbone worker: invalid inference history code");
  if ((phase === "failed") !== (row.code !== null)) throw new Error("backbone worker: invalid inference history failure");
  return { phase, canUndo: row.canUndo, code: row.code as GisMapInferencePortCodeV1 | null };
}

function wireArtifactActorMsg(message: ArtifactActorMsg): unknown {
  if (message.kind === "documentBackbone") {
    return { kind: "documentBackbone", message: Array.from(parseDocumentBackboneMessage(message.message).message) };
  }
  if (message.kind === "localMutations") {
    return { kind: "localMutations", envelopes: encodeCausalEnvelopeBatch(message.envelopes, replicationPackCodec) };
  }
  return message;
}

function parseArtifactActorMsg(message: Record<string, unknown>): ArtifactActorMsg {
  if (message.kind === "documentBackbone") {
    if (Object.keys(message).sort().join(",") !== "kind,message" || !Array.isArray(message.message) || message.message.length > BACKBONE_HOT_MESSAGE_MAXIMUM_BYTES || !message.message.every((entry) => Number.isInteger(entry) && entry >= 0 && entry <= 255)) throw new Error("backbone worker request: invalid document backbone message");
    return { kind: "documentBackbone", message: parseDocumentBackboneMessage(Uint8Array.from(message.message as readonly number[])).message };
  }
  if (message.kind === "localMutations" && Array.isArray(message.envelopes) && message.envelopes.every((entry) => typeof entry === "number")) {
    return { kind: "localMutations", envelopes: decodeCausalEnvelopeBatch(message.envelopes as readonly number[], replicationPackCodec) };
  }
  return message as ArtifactActorMsg;
}

function wireArtifactEvent(event: ArtifactEvent): unknown {
  if (event.kind === "documentBackbone") {
    return { kind: "documentBackbone", message: Array.from(parseDocumentBackboneMessage(event.message).message) };
  }
  if (event.kind === "remoteMutations") {
    return { kind: "remoteMutations", envelopes: encodeCausalEnvelopeBatch(event.envelopes, replicationPackCodec) };
  }
  return event;
}

function parseArtifactEvent(event: Record<string, unknown>): ArtifactEvent {
  if (event.kind === "documentBackbone") {
    if (Object.keys(event).sort().join(",") !== "kind,message" || !Array.isArray(event.message) || event.message.length > BACKBONE_HOT_MESSAGE_MAXIMUM_BYTES || !event.message.every((entry) => Number.isInteger(entry) && entry >= 0 && entry <= 255)) throw new Error("backbone worker response: invalid document backbone message");
    return { kind: "documentBackbone", message: parseDocumentBackboneMessage(Uint8Array.from(event.message as readonly number[])).message };
  }
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
 * needs (no `Bytes64`/`Enum`/... — a dynamic value never produces those). `TAG_INT`/`TAG_UINT`
 * carry a whole 64-bit integer exactly; a JS `number` can only ever be `TAG_F64`. */
const PACK_TAG_FALSE = 0x01;
const PACK_TAG_TRUE = 0x02;
const PACK_TAG_INT = 0x03;
const PACK_TAG_UINT = 0x04;
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

//#region 🔖️PackInteger
/** 🔢️ A whole 64-bit dynamic integer, carrying the writer's `int`/`uint` form as well as its
 * magnitude. A bare `bigint` would lose the form — Rust `Int(7)` and `UInt(7)` are equal under
 * `Number::PartialEq` yet emit different tags and therefore different canonical bytes and hashes —
 * and a JS `number` would lose magnitude past 2^53. Construct one only through {@link packInt} or
 * {@link packUInt}: the codec accepts nothing else as an integer. */
export type PackInteger = Readonly<{ readonly kind: "int" | "uint"; readonly value: bigint }>;

/** 🌱️ Everything the dynamic pack grammar can carry. `number` is always `TAG_F64`; an exact
 * integer is always a {@link PackInteger}. */
export type PackValue = null | boolean | number | string | PackInteger | readonly PackValue[] | { readonly [key: string]: PackValue };

const PACK_U64_MAX = (1n << 64n) - 1n;
const PACK_I64_MIN = -(1n << 63n);
const PACK_I64_MAX = (1n << 63n) - 1n;
/** 🔒️ Module-private mint. Membership, not shape, is what makes a carrier an integer — an
 * ordinary dynamic map `{ kind, value }` is otherwise indistinguishable from one. */
const packIntegerMint = new WeakSet<object>();

function packMintInteger(kind: "int" | "uint", value: bigint): PackInteger {
  const carrier = Object.freeze({ kind, value });
  packIntegerMint.add(carrier);
  return carrier;
}

/** @emoji 🔢️ Mints a signed 64-bit dynamic integer (`-2^63 <= value <= 2^63-1`). */
export function packInt(value: bigint): PackInteger {
  if (typeof value !== "bigint" || value < PACK_I64_MIN || value > PACK_I64_MAX) throw new Error(`packInt: ${String(value)} is outside the exact i64 range`);
  return packMintInteger("int", value);
}

/** @emoji 🔢️ Mints an unsigned 64-bit dynamic integer (`0 <= value <= 2^64-1`). */
export function packUInt(value: bigint): PackInteger {
  if (typeof value !== "bigint" || value < 0n || value > PACK_U64_MAX) throw new Error(`packUInt: ${String(value)} is outside the exact u64 range`);
  return packMintInteger("uint", value);
}

/** @emoji 🔎️ True only for a carrier this module minted — never for a look-alike literal. */
export function isPackInteger(value: unknown): value is PackInteger {
  return typeof value === "object" && value !== null && packIntegerMint.has(value);
}

/** @emoji 🧬️ Structural-clone replacement. The mint is a `WeakSet`, so a raw `structuredClone`
 * silently degrades every integer carrier into an ambiguous `{ kind, value }` map; this rebuilds
 * them, and rejects the look-alikes a clone would have produced. */
export function clonePackValue(value: PackValue): PackValue {
  if (isPackInteger(value)) return value.kind === "uint" ? packUInt(value.value) : packInt(value.value);
  if (Array.isArray(value)) return (value as readonly PackValue[]).map(clonePackValue);
  if (value !== null && typeof value === "object") {
    packRejectIntegerLookAlike(value);
    return Object.fromEntries(Object.entries(value as Record<string, PackValue>).map(([key, entry]) => [key, clonePackValue(entry)]));
  }
  if (typeof value === "bigint") throw new Error("clonePackValue: a bare bigint is not a PackValue — mint it with packInt/packUInt");
  return value;
}

function packRejectIntegerLookAlike(value: object): void {
  if (typeof (value as { value?: unknown }).value === "bigint") throw new Error("PackValue: an unminted { value: bigint } object is ambiguous — mint it with packInt/packUInt or remove the bigint");
}

/** ✍️ Canonical unsigned LEB128 over the whole `u64` range — Pack-local on purpose, because
 * `writeVarintU64` computes in `number` for bounded framing/counts and cannot reach 2^64. */
function packWriteVarintBigInt(out: number[], value: bigint): void {
  let remaining = value;
  for (;;) {
    const byte = Number(remaining & 0x7fn);
    remaining >>= 7n;
    if (remaining === 0n) {
      out.push(byte);
      return;
    }
    out.push(byte | 0x80);
  }
}

/** 📖️ Inverse of {@link packWriteVarintBigInt} with `codec::read_varint_u64`'s exact ten-byte
 * overflow rule: the tenth byte may not continue and may not carry a payload above 1. */
function packReadVarintBigInt(bytes: Uint8Array, pos: [number]): bigint {
  let result = 0n;
  for (let index = 0; index < 10; index++) {
    const byte = bytes[pos[0]];
    if (byte === undefined) throw new Error("decodePackValue: truncated integer varint");
    pos[0] += 1;
    const payload = byte & 0x7f;
    const more = (byte & 0x80) !== 0;
    if (index === 9 && (more || payload > 1)) throw new Error("decodePackValue: overlong integer varint (exceeds 10 bytes / 64 bits)");
    result |= BigInt(payload) << BigInt(index * 7);
    if (!more) return result;
  }
  throw new Error("decodePackValue: overlong integer varint (exceeds 10 bytes)");
}

const packZigzagEncode = (value: bigint): bigint => (value < 0n ? (-value << 1n) - 1n : value << 1n);
const packZigzagDecode = (raw: bigint): bigint => (raw >> 1n) ^ -(raw & 1n);
//#endregion 🔖️PackInteger

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
  if (isPackInteger(value)) return;
  if (Array.isArray(value)) {
    for (const item of value as readonly PackValue[]) packCollectStrings(item, counts);
    return;
  }
  if (value !== null && typeof value === "object") {
    for (const item of Object.values(value as Record<string, PackValue>)) packCollectStrings(item, counts);
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

/** ✍️ `pack_value::encode_dsl_value` — the tag-prefixed encoding one dynamic value recurses
 * through. A JS `number` is `TAG_F64` because that is exactly what it is; an exact 64-bit integer
 * arrives as a {@link PackInteger} and writes `TAG_UINT`/`TAG_INT` plus a canonical
 * unsigned/zig-zag LEB128. `-0` keeps its sign bit, byte-for-byte with Rust's `normalize_f64`
 * (which only folds `NaN`). Object entries sort by key BYTES with keys always forced inline,
 * never a symref. */
function packEncodeValue(value: unknown, symbolIndex: ReadonlyMap<string, number>, out: number[]): void {
  if (value === null || value === undefined) {
    out.push(PACK_TAG_NULL);
    return;
  }
  if (typeof value === "boolean") {
    out.push(value ? PACK_TAG_TRUE : PACK_TAG_FALSE);
    return;
  }
  if (isPackInteger(value)) {
    out.push(value.kind === "uint" ? PACK_TAG_UINT : PACK_TAG_INT);
    packWriteVarintBigInt(out, value.kind === "uint" ? value.value : packZigzagEncode(value.value));
    return;
  }
  if (typeof value === "number") {
    out.push(PACK_TAG_F64);
    writeF64(out, value);
    return;
  }
  if (typeof value === "string") {
    packEncodeString(value, symbolIndex, out);
    return;
  }
  if (Array.isArray(value)) {
    out.push(PACK_TAG_LIST);
    writeVarintU64(out, value.length);
    for (const item of value as readonly PackValue[]) packEncodeValue(item, symbolIndex, out);
    return;
  }
  if (typeof value === "object") {
    packRejectIntegerLookAlike(value);
    out.push(PACK_TAG_MAP);
    const entries = Object.entries(value as Record<string, PackValue>).sort((a, b) => packByteCompare(a[0], b[0]));
    writeVarintU64(out, entries.length);
    for (const [key, entryValue] of entries) {
      packEncodeStringInline(key, out);
      packEncodeValue(entryValue, symbolIndex, out);
    }
    return;
  }
  throw new Error(`encodePackValue: unsupported dynamic value of type ${typeof value}`);
}
/** 📖️ Inverse of {@link packEncodeValue} — the TS twin of `pack_value::decode_dsl_value`. */
function packDecodeValue(bytes: Uint8Array, symbols: readonly string[], pos: [number]): PackValue {
  const tag = bytes[pos[0]]!;
  pos[0] += 1;
  switch (tag) {
    case PACK_TAG_NULL:
      return null;
    case PACK_TAG_FALSE:
      return false;
    case PACK_TAG_TRUE:
      return true;
    case PACK_TAG_UINT:
      return packUInt(packReadVarintBigInt(bytes, pos));
    case PACK_TAG_INT:
      return packInt(packZigzagDecode(packReadVarintBigInt(bytes, pos)));
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
      const items: PackValue[] = [];
      for (let i = 0; i < count; i++) items.push(packDecodeValue(bytes, symbols, pos));
      return items;
    }
    case PACK_TAG_MAP: {
      const count = readVarintU64(bytes, pos);
      const entries: Record<string, PackValue> = {};
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
 * `🔖️PackValueFixtures` region). Input is runtime-checked; decoded {@link PackValue} retains
 * the closed grammar without requiring callers' typed objects to declare an index signature. */
export function encodePackValue(value: unknown): Uint8Array<ArrayBuffer> {
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
export function decodePackValue(bytes: Uint8Array): PackValue {
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
  let result: PackValue = null;
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
export function packValueToBase64(value: PackValue): string {
  const bytes = encodePackValue(value);
  let binary = "";
  for (let index = 0; index < bytes.length; index += 1) binary += String.fromCharCode(bytes[index]!);
  return `${PACK_B64_PREFIX}${btoa(binary)}`;
}

/** @emoji 📥️ Inverse of {@link packValueToBase64}. */
export function packValueFromBase64(encoded: string): PackValue {
  if (!encoded.startsWith(PACK_B64_PREFIX)) throw new Error("packValueFromBase64: expected pk: prefix");
  const binary = atob(encoded.slice(PACK_B64_PREFIX.length));
  const bytes = new Uint8Array(binary.length);
  for (let index = 0; index < binary.length; index += 1) bytes[index] = binary.charCodeAt(index);
  return decodePackValue(bytes);
}

/** @emoji 🎯️ Plugin `handleAction` wire: pack-base64 `{ controllerId, action, args? }`. */
export type ActionWire = { readonly controllerId: string; readonly action: string; readonly args?: PackValue };

export function encodeActionWire(descriptor: ActionWire): string {
  return packValueToBase64(descriptor);
}

/** @emoji 📥️ Inverse of {@link encodeActionWire}. */
export function decodeActionWire(wire: string): ActionWire {
  const wireValue = packValueFromBase64(wire);
  if (wireValue === null || typeof wireValue !== "object" || Array.isArray(wireValue) || isPackInteger(wireValue)) throw new Error("decodeActionWire: expected a dynamic map");
  const record = wireValue as Readonly<Record<string, PackValue>>;
  if (typeof record.controllerId !== "string" || typeof record.action !== "string") throw new Error("decodeActionWire: controllerId and action are required strings");
  return record.args === undefined ? { controllerId: record.controllerId, action: record.action } : { controllerId: record.controllerId, action: record.action, args: record.args };
}

/** @emoji 🎬️ Decodes a component-scene `*Json` field when it carries {@link packValueToBase64} bytes. */
export function decodeScenePackField(encoded: string): PackValue {
  return packValueFromBase64(encoded);
}

/** @emoji 📤️ `protocol::encode_envelopes` batch as a {@link packValueToBase64} string for `applyMutations`. */
export function encodeMutationEnvelopesPack(envelopes: readonly MutationEnvelope[]): string {
  return packValueToBase64(Array.from(encodeCausalEnvelopeBatch(envelopes, replicationPackCodec)));
}

/** @emoji 📥️ Inverse of {@link encodeMutationEnvelopesPack}. */
export function decodeMutationEnvelopesPack(pack: string): MutationEnvelope[] {
  const wire = packValueFromBase64(pack);
  if (!isPackByteVector(wire)) throw new Error("decodeMutationEnvelopesPack: expected pack byte array");
  return decodeCausalEnvelopeBatch(wire, replicationPackCodec);
}

/** @emoji 🧮️ Projects a decoded {@link PackValue} onto strict JSON. An integer carrier survives
 * only when its `bigint` fits a safe JS integer exactly, and then as a `number`; anything else
 * throws rather than rounding. The descriptor pipeline is deliberately JSON-only — canonical-check
 * and hash the raw `PackValue`, then project through this for JSON schema/pair comparison and JSON
 * writing. Never the other way round: JSON never learns to carry an integer carrier. */
export function packValueToExactJson(value: PackValue, path = "$"): unknown {
  if (isPackInteger(value)) {
    if (value.value < BigInt(Number.MIN_SAFE_INTEGER) || value.value > BigInt(Number.MAX_SAFE_INTEGER)) throw new Error(`packValueToExactJson: ${path} carries ${value.value}, which no JSON number represents exactly`);
    return Number(value.value);
  }
  if (Array.isArray(value)) return (value as readonly PackValue[]).map((item, index) => packValueToExactJson(item, `${path}[${index}]`));
  if (value !== null && typeof value === "object") {
    packRejectIntegerLookAlike(value);
    return Object.fromEntries(Object.entries(value as Record<string, PackValue>).map(([key, entry]) => [key, packValueToExactJson(entry, `${path}.${key}`)]));
  }
  if (typeof value === "number" && !Number.isFinite(value)) throw new Error(`packValueToExactJson: ${path} carries ${value}, which no JSON number represents`);
  if (value !== null && typeof value !== "boolean" && typeof value !== "number" && typeof value !== "string") throw new Error(`packValueToExactJson: ${path} carries an unsupported ${typeof value}`);
  return value;
}

/** @emoji 🧱️ The one byte-vector boundary every artifact/envelope byte parser shares: each item
 * must be a finite, safe, integral JS `number` in `0..255`. A `PackInteger` and a fractional or
 * out-of-range `number` are both rejected rather than coerced. */
export function isPackByteVector(value: PackValue): value is readonly number[] {
  return Array.isArray(value) && (value as readonly PackValue[]).every((entry) => typeof entry === "number" && Number.isSafeInteger(entry) && entry >= 0 && entry <= 255);
}

/** @emoji 🔢️ Narrows one declared unsigned field to an exact JS integer, or `null`. A `TAG_UINT`
 * carrier and a plain integral `number` both qualify; a signed carrier, a fraction, and anything
 * beyond `Number.MAX_SAFE_INTEGER` do not — an out-of-range value is a rejection, never a rounding. */
export function packUIntSafeOrNull(value: PackValue): number | null {
  if (isPackInteger(value)) return value.kind === "uint" && value.value <= BigInt(Number.MAX_SAFE_INTEGER) ? Number(value.value) : null;
  return typeof value === "number" && Number.isSafeInteger(value) && value >= 0 ? value : null;
}

/** @emoji 🔍️ Renders one raw wasm-boundary value for a fault message, `bigint`s included, bounded so a
 * whole decoded node never lands in a log line. */
export function describePackWireValue(raw: unknown): string {
  if (typeof raw === "bigint") return `${raw}n`;
  try {
    return JSON.stringify(raw, (_key, value) => (typeof value === "bigint" ? `${value}n` : value))?.slice(0, 400) ?? String(raw);
  } catch {
    return String(raw);
  }
}

/** @emoji 🔢️ One natural number off an actor WIT/pack boundary: a WIT `u64` arrives as a `bigint`, a
 * pack-decoded integer as a lossless integer carrier, an absent value means zero. Anything not exactly
 * representable as a safe non-negative JS integer is a fault naming the field and the value, never a
 * rounding. Every renderer's wire decoder shares this one implementation. */
export function packWireNatural(raw: unknown, field = "natural"): number {
  if (raw === undefined || raw === null) return 0;
  const value = typeof raw === "bigint" ? (raw >= 0n && raw <= BigInt(Number.MAX_SAFE_INTEGER) ? Number(raw) : Number.NaN) : typeof raw === "number" ? raw : packUIntSafeOrNull(raw as PackValue) ?? Number.NaN;
  if (!Number.isSafeInteger(value) || value < 0) throw new Error(`actor WIT natural "${field}" is not a safe non-negative integer: ${describePackWireValue(raw)}`);
  return value;
}

/** @emoji 📦️ Decodes one pack-encoded wasm-boundary byte payload and projects its lossless integer
 * carriers onto exact JSON numbers — the only shape the UI/effect contract twins declare. The one
 * decoder every renderer injects wherever a `pack`-typed WIT field crosses into TypeScript; decoding
 * without this projection leaks `{kind, value}` carriers into node ids, revisions and effect params. */
export function decodePackWire(bytes: Uint8Array, path = "$"): unknown {
  return packValueToExactJson(decodePackValue(bytes) as PackValue, path);
}

function invocationResultObject(value: unknown, path: string, required: readonly string[], optional: readonly string[] = []): Readonly<Record<string, unknown>> {
  if (value === null || typeof value !== "object" || Array.isArray(value)) throw new Error(`${path}: expected an object`);
  const record = value as Readonly<Record<string, unknown>>;
  const allowed = new Set([...required, ...optional]);
  if (Object.keys(record).some((key) => !allowed.has(key)) || required.some((key) => !(key in record))) throw new Error(`${path}: invalid exact fields`);
  return record;
}

function invocationResultString(value: unknown, path: string): string {
  if (typeof value !== "string" || value.length === 0) throw new Error(`${path}: expected a nonempty string`);
  return value;
}

function invocationResultNatural(value: unknown, path: string): number {
  if (typeof value !== "number" || !Number.isSafeInteger(value) || value < 0) throw new Error(`${path}: expected a safe natural number`);
  return value;
}

function invocationResultBytes(value: unknown, path: string): readonly number[] {
  if (!Array.isArray(value) || value.some((byte) => typeof byte !== "number" || !Number.isInteger(byte) || byte < 0 || byte > 255)) throw new Error(`${path}: expected bytes`);
  return value;
}

function invocationResultStrings(value: unknown, path: string): readonly string[] {
  if (!Array.isArray(value)) throw new Error(`${path}: expected an array`);
  return value.map((entry, index) => invocationResultString(entry, `${path}[${index}]`));
}

function invocationResultDiff(value: unknown, path: string): ArtifactDiff {
  const record = invocationResultObject(value, path, ["schema", "payload"]);
  return { schema: invocationResultString(record.schema, `${path}.schema`), payload: invocationResultBytes(record.payload, `${path}.payload`) };
}

function invocationResultUndoPolicy(value: unknown, path: string): UndoPolicy {
  if (value === "ExactBaseOnly" || value === "TransformAgainstConcurrent" || value === "SemanticUndo" || value === "CompensatingAction") return value;
  throw new Error(`${path}: invalid undo policy`);
}

function invocationResultInverse(value: unknown, path: string): InverseMutation {
  const record = invocationResultObject(value, path, ["targetMutation", "inverseDiff", "baseVersion", "undoPolicy"], ["dependencies"]);
  return {
    targetMutation: invocationResultString(record.targetMutation, `${path}.targetMutation`),
    inverseDiff: invocationResultDiff(record.inverseDiff, `${path}.inverseDiff`),
    baseVersion: invocationResultNatural(record.baseVersion, `${path}.baseVersion`),
    ...(record.dependencies === undefined ? {} : { dependencies: invocationResultStrings(record.dependencies, `${path}.dependencies`) }),
    undoPolicy: invocationResultUndoPolicy(record.undoPolicy, `${path}.undoPolicy`),
  };
}

function invocationResultMutation(value: unknown, path: string): KernelMutation {
  const record = invocationResultObject(value, path, ["id", "document", "baseVersion", "invocationId", "diff", "inverse", "author", "timestamp"], ["dependencies"]);
  const timestamp = invocationResultObject(record.timestamp, `${path}.timestamp`, ["actor", "physical_ms", "logical"]);
  return {
    id: invocationResultString(record.id, `${path}.id`),
    document: invocationResultString(record.document, `${path}.document`),
    baseVersion: invocationResultNatural(record.baseVersion, `${path}.baseVersion`),
    invocationId: invocationResultString(record.invocationId, `${path}.invocationId`),
    diff: invocationResultDiff(record.diff, `${path}.diff`),
    inverse: invocationResultInverse(record.inverse, `${path}.inverse`),
    ...(record.dependencies === undefined ? {} : { dependencies: invocationResultStrings(record.dependencies, `${path}.dependencies`) }),
    author: invocationResultString(record.author, `${path}.author`),
    timestamp: {
      actor: invocationResultNatural(timestamp.actor, `${path}.timestamp.actor`),
      physical_ms: invocationResultNatural(timestamp.physical_ms, `${path}.timestamp.physical_ms`),
      logical: invocationResultNatural(timestamp.logical, `${path}.timestamp.logical`),
    },
  };
}

function invocationResultUndoGroup(value: unknown, path: string): UndoGroup {
  const record = invocationResultObject(value, path, ["invocationId", "mutations", "inverseMutations"], ["memberEdits"]);
  if (typeof record.invocationId !== "string") throw new Error(`${path}.invocationId: expected a string`);
  if (!Array.isArray(record.inverseMutations)) throw new Error(`${path}.inverseMutations: expected an array`);
  if (record.memberEdits !== undefined && !Array.isArray(record.memberEdits)) throw new Error(`${path}.memberEdits: expected an array`);
  const mutations = invocationResultStrings(record.mutations, `${path}.mutations`);
  if (record.invocationId.length === 0 && (mutations.length !== 0 || record.inverseMutations.length !== 0 || (record.memberEdits?.length ?? 0) !== 0)) throw new Error(`${path}.invocationId: empty identity cannot own mutation results`);
  return {
    invocationId: record.invocationId,
    mutations,
    inverseMutations: record.inverseMutations.map((entry, index) => invocationResultInverse(entry, `${path}.inverseMutations[${index}]`)),
    ...(record.memberEdits === undefined
      ? {}
      : {
          memberEdits: record.memberEdits.map((entry, index) => {
            const edit = invocationResultObject(entry, `${path}.memberEdits[${index}]`, ["document", "editId"]);
            return { document: invocationResultString(edit.document, `${path}.memberEdits[${index}].document`), editId: invocationResultString(edit.editId, `${path}.memberEdits[${index}].editId`) };
          }),
        }),
  };
}

/** 📥️ Decodes the exact packed mutation and inverse-group fields retained by an Invocation frame. */
export function decodeInvocationResultPacks(frame: { readonly mutations: ArrayLike<number>; readonly inverse_group: ArrayLike<number> }): Pick<import("@semio-tech/framework").InvocationResponse, "mutations" | "inverseGroup"> {
  if (frame.mutations.length > INVOCATION_RESULT_PACK_MAXIMUM_BYTES || frame.inverse_group.length > INVOCATION_RESULT_PACK_MAXIMUM_BYTES) throw new Error("invocation result: packed field exceeds command transport authority");
  if (frame.mutations.length === 0 && frame.inverse_group.length === 0) return { mutations: [], inverseGroup: { invocationId: "", mutations: [], inverseMutations: [] } };
  if (frame.mutations.length === 0 || frame.inverse_group.length === 0) throw new Error("invocation result: mutations and inverse group must be published together");
  const mutations = decodePackWire(new Uint8Array(frame.mutations), "invocation.mutations");
  if (!Array.isArray(mutations)) throw new Error("invocation.mutations: expected an array");
  return { mutations: mutations.map((entry, index) => invocationResultMutation(entry, `invocation.mutations[${index}]`)), inverseGroup: invocationResultUndoGroup(decodePackWire(new Uint8Array(frame.inverse_group), "invocation.inverseGroup"), "invocation.inverseGroup") };
}

/** @emoji 🔢️ Throwing form of {@link packUIntSafeOrNull} for a required schema field. */
export function asPackUIntSafe(value: PackValue, name: string): number {
  const parsed = packUIntSafeOrNull(value);
  if (parsed === null) throw new Error(`${name}: expected an exact unsigned integer within the safe JS range`);
  return parsed;
}

/** @emoji 🔢️ {@link asPackUIntSafe} additionally bounded to `u32`. */
export function asPackUInt32(value: PackValue, name: string): number {
  const parsed = asPackUIntSafe(value, name);
  if (parsed > 0xffff_ffff) throw new Error(`${name}: ${parsed} exceeds the declared u32 range`);
  return parsed;
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
export type WindowConfigPackEntry = { readonly window_id: string; readonly window_kind_id: string; readonly envelope_pack: readonly number[] };

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
  | { readonly LoadWindowConfig: { readonly seq: number; readonly entry: WindowConfigPackEntry } }
  | { readonly ReadWindowConfigs: { readonly seq: number } }
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
        readonly mutations: readonly number[];
        readonly inverse_group: readonly number[];
      };
    }
  | { readonly DocumentChanged: { readonly envelopes: readonly (readonly number[])[]; readonly origin: string } }
  | { readonly Document: { readonly in_reply_to: number; readonly pack: readonly number[]; readonly spr: readonly number[]; readonly ops: string } }
  | { readonly Config: { readonly in_reply_to: number; readonly pack: readonly number[]; readonly spr: readonly number[]; readonly ops: string } }
  | { readonly WindowConfigs: { readonly in_reply_to: number; readonly entries: readonly WindowConfigPackEntry[] } }
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

export const INVOCATION_RESULT_PACK_MAXIMUM_BYTES = 4_096 * 64;
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
function writeWindowConfigPackEntry(out: number[], entry: WindowConfigPackEntry): void {
  writeStr(out, entry.window_id);
  writeStr(out, entry.window_kind_id);
  writeBytes(out, entry.envelope_pack);
}
function readWindowConfigPackEntry(bytes: Uint8Array, pos: [number]): WindowConfigPackEntry {
  return { window_id: readStr(bytes, pos), window_kind_id: readStr(bytes, pos), envelope_pack: readBytes(bytes, pos) };
}
function writeVecWindowConfigPackEntry(out: number[], entries: readonly WindowConfigPackEntry[]): void {
  writeVarintU64(out, entries.length);
  for (const entry of entries) writeWindowConfigPackEntry(out, entry);
}
function readVecWindowConfigPackEntry(bytes: Uint8Array, pos: [number]): WindowConfigPackEntry[] {
  const count = readVarintU64(bytes, pos);
  return Array.from({ length: count }, () => readWindowConfigPackEntry(bytes, pos));
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
  presence: 28, LocalInteractionQuery: 29, LoadWindowConfig: 30, ReadWindowConfigs: 31,
} as const;
const APP_FRAME_TAGS = {
  Done: 0, Invocation: 1, DocumentChanged: 2, Document: 3,
  Config: 4, ConfigChanged: 5, ContextMenu: 6, Media: 7, MediaFingerprint: 8, Error: 9, Emit: 10, Draft: 11, Children: 12, Ephemeral: 13, HistorySnapshot: 14,
  transactionProposal: 15, transactionPrepared: 16, transactionCommitted: 17, transactionRolledBack: 18,
  MergeReport: 19, Conflicts: 20, UiPatch: 21, UiSnapshotEnd: 22, LocalInteractionQuery: 23, WindowConfigs: 24,
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
  } else if ("LoadWindowConfig" in cmd) {
    out.push(APP_COMMAND_TAGS.LoadWindowConfig);
    writeVarintU64(out, cmd.LoadWindowConfig.seq);
    writeWindowConfigPackEntry(out, cmd.LoadWindowConfig.entry);
  } else if ("ReadWindowConfigs" in cmd) {
    out.push(APP_COMMAND_TAGS.ReadWindowConfigs);
    writeVarintU64(out, cmd.ReadWindowConfigs.seq);
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
    case APP_COMMAND_TAGS.LoadWindowConfig:
      return { LoadWindowConfig: { seq: readVarintU64(bytes, pos), entry: readWindowConfigPackEntry(bytes, pos) } };
    case APP_COMMAND_TAGS.ReadWindowConfigs:
      return { ReadWindowConfigs: { seq: readVarintU64(bytes, pos) } };
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
    if (frame.Invocation.mutations.length > INVOCATION_RESULT_PACK_MAXIMUM_BYTES || frame.Invocation.inverse_group.length > INVOCATION_RESULT_PACK_MAXIMUM_BYTES) throw new Error("encodeAppFrame: invocation result pack exceeds command transport authority");
    out.push(APP_FRAME_TAGS.Invocation);
    writeVarintU64(out, frame.Invocation.in_reply_to);
    writeBytes(out, frame.Invocation.output);
    writeBytes(out, frame.Invocation.diagnostics);
    writeBytes(out, frame.Invocation.ui_scope);
    writeBytes(out, frame.Invocation.history_patch);
    writeBytes(out, frame.Invocation.messages);
    writeBytes(out, frame.Invocation.mutations);
    writeBytes(out, frame.Invocation.inverse_group);
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
  } else if ("WindowConfigs" in frame) {
    out.push(APP_FRAME_TAGS.WindowConfigs);
    writeVarintU64(out, frame.WindowConfigs.in_reply_to);
    writeVecWindowConfigPackEntry(out, frame.WindowConfigs.entries);
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
      const readInvocationResultPack = (): readonly number[] => {
        const length = readVarintU64(bytes, pos);
        if (length > INVOCATION_RESULT_PACK_MAXIMUM_BYTES) throw new Error("decodeAppFrame: invocation result pack exceeds command transport authority");
        const end = pos[0] + length;
        if (!Number.isSafeInteger(end) || end > bytes.length) throw new Error("decodeAppFrame: invocation result pack is truncated");
        const value = Array.from(bytes.subarray(pos[0], end));
        pos[0] = end;
        return value;
      };
      const mutations = readInvocationResultPack();
      const inverse_group = readInvocationResultPack();
      return { Invocation: { in_reply_to, output, diagnostics, ui_scope, history_patch, messages, mutations, inverse_group } };
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
    case APP_FRAME_TAGS.WindowConfigs:
      return { WindowConfigs: { in_reply_to: readVarintU64(bytes, pos), entries: readVecWindowConfigPackEntry(bytes, pos) } };
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
 * `🧫️fixtures/📡️channel/🔖️channel-version.json`, which owns the number — this one sat at 8 while Rust
 * had moved to 10, so the pin exists to make a half-done bump fail a test instead of a session.
 * Channel v12 retired the `Hello`/`Welcome` handshake this constant used to be carried on — it now
 * exists purely for the drift-guard test below. */
export const APP_CHANNEL_VERSION = 15;

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
  private readonly pending: { readonly seq: number; readonly queryReceipt: boolean; readonly transaction: AppChannelTransactionReply | null; readonly document: { readonly pack: Uint8Array; readonly spr: Uint8Array } | null; readonly resolve: (frames: AppFrameValue[]) => void; readonly reject: (error: unknown) => void }[] = [];
  /** 📦️ Per-instance document-pack cache (ticket
   * 26/08/16/PLUGIN-DEPENDENCIES-ARTIFACT-CONTRIBUTIONS-AND-COMPOSITE-MUTATIONS, scout-1 §4: "the
   * browser host keeps NO document pack per instance today"). Populated from BOTH directions —
   * {@link loadDocument}'s accepted arguments and every
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
        this.cachedPack = null;
        this.cachedSpr = null;
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
        this.captureDocumentFrames(reply, waiter.document);
        waiter.resolve(reply);
      }
      this.finishDisposal();
    }
  }

  /** 🔌️ Ends this client's background {@link pumpOutcomes} subscription — call once from
   * `destroyApp` (`PluginRuntime/🟦️.tsx`) so a torn-down instance doesn't leak a live
   * subscriber against the handle-wide outcome stream for the rest of the handle's lifetime. */
  dispose(): void {
    this.disposed = true;
    this.cachedPack = null;
    this.cachedSpr = null;
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
  private captureDocumentFrames(frames: readonly AppFrameValue[], candidate: { readonly pack: Uint8Array; readonly spr: Uint8Array } | null): void {
    if (this.disposed || frames.some(frame => "Error" in frame)) return;
    if (candidate && frames.some(frame => "Done" in frame)) {
      this.cachedPack = candidate.pack;
      this.cachedSpr = candidate.spr;
    }
    for (const frame of frames) {
      if ("Document" in frame) {
        this.cachedPack = new Uint8Array(frame.Document.pack);
        this.cachedSpr = new Uint8Array(frame.Document.spr);
      }
    }
  }

  /** 📦️ The cached `{pack, spr}` for this instance's document, or `null` before any
   * accepted {@link loadDocument} call or `AppFrame::Document` reply has been observed. Surfaced to the
   * transaction coordinator through the `PluginWasmHandle` adapter's own `documentPack` accessor
   * (`PluginRuntime/🟦️.tsx`) — a contributor plan call needs the target's current snapshot
   * pack, and this is the only place that snapshot is retained host-side. */
  documentPack(): { readonly pack: Uint8Array; readonly spr: Uint8Array } | null {
    return this.cachedPack && this.cachedSpr ? { pack: this.cachedPack.slice(), spr: this.cachedSpr.slice() } : null;
  }

  /** 🔀️ Queues one encoded command and resolves with every frame its matching {@link TurnOutcome}
   * carries — see this class's own header doc for how the reply gets correlated back to this call. */
  private sendCommand(command: AppCommandValue): Promise<AppFrameValue[]> {
    if (this.disposed) return Promise.reject(new Error("app-channel.disposed"));
    return new Promise<AppFrameValue[]>((resolve, reject) => {
      const seq = Object.values(command)[0]!.seq;
      const document = "LoadDocument" in command ? { pack: Uint8Array.from(command.LoadDocument.pack), spr: Uint8Array.from(command.LoadDocument.spr) } : null;
      const waiter = { seq, queryReceipt: false, transaction: appChannelTransactionReply(command), document, resolve, reject };
      this.pending.push(waiter);
      try { this.handle.enqueue(this.instanceId, [encodeAppCommand(command)]); }
      catch (error) {
        const index = this.pending.indexOf(waiter);
        if (index !== -1) this.pending.splice(index, 1);
        reject(error);
      }
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
    this.pending.push({ seq, queryReceipt: true, transaction: null, document: null, resolve: () => {}, reject: (error: unknown) => this.finishLocalInteractionQuery(error) });
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
    return this.sendCommand({ LoadDocument: { seq: this.nextSeq(), pack: Array.from(pack), spr: Array.from(spr) } });
  }

  /** 🪟️ Restores one exact concrete window's persisted-local config envelope. */
  async loadWindowConfig(entry: WindowConfigPackEntry): Promise<void> {
    const seq = this.nextSeq();
    const frames = await this.sendCommand({
      LoadWindowConfig: {
        seq,
        entry: { window_id: entry.window_id, window_kind_id: entry.window_kind_id, envelope_pack: Array.from(entry.envelope_pack) },
      },
    });
    const error = frames.find((frame): frame is Extract<AppFrameValue, { readonly Error: unknown }> => "Error" in frame);
    if (error) throw new Error(`AppChannelClient.loadWindowConfig(${this.appId}): ${faultDisplayMessage(error.Error.fault, decodePackValue)}`);
    if (!frames.some((frame) => "Done" in frame && frame.Done.in_reply_to === seq)) {
      throw new Error(`AppChannelClient.loadWindowConfig(${this.appId}): missing Done frame for seq ${seq}`);
    }
  }

  /** 🪟️ Reads every exact concrete window config envelope for persisted-local replay. */
  async readWindowConfigs(): Promise<readonly WindowConfigPackEntry[]> {
    const seq = this.nextSeq();
    const frames = await this.sendCommand({ ReadWindowConfigs: { seq } });
    const error = frames.find((frame): frame is Extract<AppFrameValue, { readonly Error: unknown }> => "Error" in frame);
    if (error) throw new Error(`AppChannelClient.readWindowConfigs(${this.appId}): ${faultDisplayMessage(error.Error.fault, decodePackValue)}`);
    const configs = frames.find(
      (frame): frame is Extract<AppFrameValue, { readonly WindowConfigs: unknown }> =>
        "WindowConfigs" in frame && frame.WindowConfigs.in_reply_to === seq,
    );
    if (!configs) throw new Error(`AppChannelClient.readWindowConfigs(${this.appId}): missing WindowConfigs frame for seq ${seq}`);
    return configs.WindowConfigs.entries.map((entry) => ({
      window_id: entry.window_id,
      window_kind_id: entry.window_kind_id,
      envelope_pack: Array.from(entry.envelope_pack),
    }));
  }

  /** 🧾️ Retrieves the complete history projection for initial load or cursor-gap recovery. */
  async readHistory(): Promise<AppFrameValue[]> {
    return this.sendCommand({ ReadHistory: { seq: this.nextSeq() } });
  }

  /** 📂️ Opens an artifact in its resolved (or explicitly named) viewer/editor surface —
   * `os.open-artifact` (contract-freeze §3 of
   * `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️16/ARTIFACT-VIEWERS-AND-EDITORS-PER-SUBSET/`). Empty
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
   * already-known ops onto in one call — see `PluginRuntime/🟦️.tsx`'s `TransactionCoordinator`. */
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
  const { registerTests2 } = await import("./🧪️tests/🧪️backbone-envelope-io/🟦️.ts");
  await registerTests2(import.meta.vitest, { APP_CHANNEL_VERSION, AppChannelClient, AppChannelRequestSequence, INVOCATION_RESULT_PACK_MAXIMUM_BYTES, applyBackboneMessage, backboneKindFromUri, buildFileBackboneUri, buildFolderBackboneUri, buildFrameworkSyncUtilities, buildRemoteBackboneUri, clonePackValue, createTurnOutcomeBroadcast, decodeAppCommand, decodeAppFrame, decodeBackboneMessage, decodeConflictsFromWire, decodeDispatchReportFromWire, decodeDocumentPackBytes, decodeDocumentPackSnapshot, decodeInvocationResultPacks, decodeMergeReportFromWire, decodePackValue, decodePresencePeer, decodeScenePackValue, encodeAppCommand, encodeAppFrame, encodeBackboneMessage, encodeDocumentPackBundle, encodeDocumentPackBytes, encodePackValue, encodePresencePeer, faultMessages, isPackByteVector, isPackInteger, packInt, packUInt, packValueToExactJson, parseRemoteBackboneUri, planWorkflow }, { directory: import.meta.dir, url: import.meta.url });
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
// `🔨️modules/📇️directory/🟦️.ts`; this region only imports/re-exports and, per this
// package's `🧪️tests/🟦️.ts` (`include`/`includeSource` list only THIS file and
// `🧵️backbone-worker.ts`), hosts the in-source parity test against the Rust twin's golden fixture.
import { descriptorDigestEncodingV1, descriptorDigestV1, emptyDirectoryReadModel, fold, foldAll, isDirectoryCommandKind, isDirectoryEventBodyKind, isDirectoryStreamMessageKind, parseDirectorySpaceAdministrationPageV1 } from "./🔨️modules/📇️directory/🟦️.ts";
import type { DirectoryReadModel, DocumentDescriptor } from "./🔨️modules/📇️directory/🟦️.ts";

export type {
  AdminConnectionSnapshotV1,
  AdminIntentOutcomeV1,
  AdminIntentReceiptV1,
  AdminIntentResultV1,
  AdminIntentStateV1,
  AdminIntentV1,
  AdminOperationAuditPhaseV1,
  AdminOperationAuditV1,
  AdminOperationProgressV1,
  AdminOperationStatusV1,
  AdminPageV1,
  AdminRecordedConnectionV1,
  ArtifactBlobRef,
  ArtifactCheckpoint,
  ArtifactFrontier,
  ArtifactHash,
  ArtifactRetention,
  CheckpointId,
  ConnectionView,
  DirectoryActor,
  DirectoryActorKind,
  DirectoryCommand,
  DirectoryConnectionPhase,
  DirectoryEvent,
  DirectoryEventBody,
  DirectoryReadModel,
  DirectorySpace,
  DirectorySpaceAdministrationCapabilitiesV1,
  DirectorySpaceAdministrationInviteRowV1,
  DirectorySpaceAdministrationMemberRowV1,
  DirectorySpaceAdministrationPageV1,
  DirectorySpaceAdministrationSectionV1,
  DirectorySpaceAdministrationWindowV1,
  DirectorySpaceKind,
  DirectorySpaceListEntryV1,
  DirectorySpaceRole,
  DirectorySpaceVisibility,
  DirectoryStreamMessage,
  DocumentDescriptor,
  DocumentFrontier,
  DocumentOwner,
  DocumentScope,
  DocumentView,
  Hlc,
  InviteView,
  MemberSpaceViewV1,
  MemberView,
  PublicDocumentCatalogEntryV1,
  PublicSpaceViewV1,
  SpaceView,
  UserView,
} from "./🔨️modules/📇️directory/🟦️.ts";
export { decodeServerFrame, descriptorDigestEncodingV1, descriptorDigestV1, emptyDirectoryReadModel, encodeServerFrame, fold, foldAll, isDirectoryCommandKind, isDirectoryEventBodyKind, isDirectoryStreamMessageKind, parseDirectorySpaceAdministrationPageV1 };

if (import.meta.vitest) {
  const { registerTests3 } = await import("./🧪️tests/🧪️backbone-envelope-io/🟦️.ts");
  await registerTests3(import.meta.vitest, { descriptorDigestEncodingV1, descriptorDigestV1, emptyDirectoryReadModel, foldAll }, { directory: import.meta.dir, url: import.meta.url });
}

//#region 🔖️HubBinding
// 📇️ ticket 26/08/16/HUB-SPACES-LIVE-PRESENCE-AND-COLLABORATIVE-STUDIOS C2/C6 — the shell's ONLY
// point of contact with the directory hub's HTTP/WS control plane. Plugin surfaces never talk to
// the network (contract §C6); `🧵️backbone-worker.ts`'s `🔖️Directory` region is the only caller, so
// the shell never opens a directory socket on the UI thread. `fetch`/`WebSocket` only — no external
// HTTP library (CLAUDE.md "no external libraries for runtime purposes").
import type { DirectorySpaceAdministrationPageV1, DirectorySpaceListEntryV1, DocumentScope } from "./🔨️modules/📇️directory/🟦️.ts";
import { DIRECTORY_SPACE_ADMINISTRATION_CURSOR_MAX_BYTES, DIRECTORY_SPACE_ADMINISTRATION_PAGE_MAX_BYTES } from "./🔨️modules/📇️directory/🟦️.ts";

/** 🏛️ One administration page plus the exact response bytes its receipt covers. */
export interface CanonicalDirectorySpaceAdministrationPageV1 {
  readonly canonicalJson: string;
  readonly page: DirectorySpaceAdministrationPageV1;
}

/** 🔁️ Reconnect backoff shared by every hub transport this package opens — `connectHub` in
 * `🧵️backbone-worker.ts` (artifact sync) and {@link DirectoryClient.stream} both import these
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

/** 📌️ A global stream whose reconnect cursor advances only after a Home page acknowledgement. */
export type DirectoryAcknowledgedStream = DirectoryStream & { readonly acknowledge: (through: number) => void };

/** 🚨️ One closed command-transport denial. It never carries a raw response body or server text. */
export class DirectoryCommandError extends Error {
  readonly code: DirectoryCommandErrorCodeV1;

  constructor(code: DirectoryCommandErrorCodeV1) {
    super(`directory command: ${code}`);
    this.name = "DirectoryCommandError";
    this.code = code;
  }
}

/** 📄️ Exact canonical page bytes plus the bounded header a shell needs for ACK ordering. */
export type CanonicalDirectoryEventPageV1 = Readonly<{
  canonicalJson: string;
  sessionBindingSha256: string;
  authorizationGeneration: number;
  afterSeqExclusive: number;
  throughSeqInclusive: number;
  hasMore: boolean;
  receiptSha256: string;
}>;

/** ✅️ Exact retained-Home acknowledgement for one still-owned directory page. */
export type DirectoryEventPageAckV1 = Readonly<{
  bootstrapEpoch: number;
  sessionBindingSha256: string;
  authorizationGeneration: number;
  throughSeqInclusive: number;
  receiptSha256: string;
}>;

/** 🚨️ Thrown by every {@link DirectoryClient} REST method on a non-2xx response — `status` lets a
 * caller (this package's `🧵️backbone-worker.ts` directory lane) distinguish "the hub answered and
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

/** 🎫 One-use, non-persistable authority for exactly one socket upgrade. */
export type SocketGrantReceiptV1 = Readonly<{
  schema: "semio.hub.socket-grant/v1";
  protocol: "semio.socket.v1";
  grant: string;
  actorId: string;
  expiresAtMs: number;
}>;

export type BrowserBrokerPortRequestV1 =
  | Readonly<{ kind: "initialize"; proof: string }>
  | Readonly<{ kind: "request"; requestId: string; operation: "me" }>
  | Readonly<{ kind: "cancel"; requestId: string }>
  | Readonly<{ kind: "close" }>;

export type BrowserBrokerPortResponseV1 =
  | Readonly<{ kind: "initialized"; ok: boolean }>
  | Readonly<{ kind: "response"; requestId: string; status: number; body: string }>;

function exactRecordKeys(value: Readonly<Record<string, unknown>>, expected: readonly string[]): boolean {
  const keys = Object.keys(value).sort();
  return keys.length === expected.length && keys.every((key, index) => key === [...expected].sort()[index]);
}

function directoryProjectionRecord(value: unknown, keys: readonly string[]): Readonly<Record<string, unknown>> | undefined {
  if (!value || typeof value !== "object" || Array.isArray(value)) return undefined;
  const record = value as Readonly<Record<string, unknown>>;
  return exactRecordKeys(record, keys) ? record : undefined;
}

function directoryProjectionInteger(value: unknown): value is number {
  return typeof value === "number" && Number.isSafeInteger(value) && value >= 0;
}

function directoryProjectionSignedInteger(value: unknown): value is number {
  return typeof value === "number" && Number.isSafeInteger(value);
}

function directoryProjectionText(value: unknown): value is string {
  return typeof value === "string";
}

function directoryProjectionSpaceBase(record: Readonly<Record<string, unknown>>): boolean {
  return directoryProjectionText(record.id)
    && directoryProjectionText(record.name)
    && (record.kind === "atelier" || record.kind === "studio" || record.kind === "archive")
    && (record.visibility === "private" || record.visibility === "public")
    && directoryProjectionInteger(record.memberCount)
    && directoryProjectionInteger(record.documentCount)
    && directoryProjectionSignedInteger(record.createdAtMs)
    && directoryProjectionSignedInteger(record.updatedAtMs);
}

function directoryPublicSpace(value: unknown): boolean {
  const record = directoryProjectionRecord(value, ["createdAtMs", "documentCount", "id", "kind", "memberCount", "name", "updatedAtMs", "visibility"]);
  return record !== undefined && directoryProjectionSpaceBase(record) && record.visibility === "public";
}

function directoryMemberSpace(value: unknown): boolean {
  const record = directoryProjectionRecord(value, ["activeConnections", "createdAtMs", "documentCount", "id", "kind", "memberCount", "name", "ownerUserId", "role", "updatedAtMs", "visibility"]);
  return record !== undefined
    && directoryProjectionSpaceBase(record)
    && directoryProjectionText(record.ownerUserId)
    && (record.role === "author" || record.role === "spectator")
    && directoryProjectionInteger(record.activeConnections);
}

function directoryMemberSpaceRole(value: unknown, role: "author" | "spectator"): boolean {
  return directoryMemberSpace(value) && (value as Readonly<Record<string, unknown>>).role === role;
}

function directoryOwner(value: unknown): boolean {
  const record = directoryProjectionRecord(value, ["packageHash", "packageId", "pluginId", "version"]);
  return record !== undefined
    && directoryProjectionText(record.pluginId)
    && directoryProjectionText(record.packageId)
    && directoryProjectionText(record.version)
    && typeof record.packageHash === "string"
    && /^(?!0{64}$)[0-9a-f]{64}$/u.test(record.packageHash);
}

function directoryPublicDocument(value: unknown): boolean {
  const record = directoryProjectionRecord(value, ["artifactKind", "artifactSchema", "documentId", "owner", "packSchemaHash"]);
  return record !== undefined
    && directoryProjectionText(record.documentId)
    && directoryProjectionText(record.artifactKind)
    && directoryProjectionText(record.artifactSchema)
    && directoryOwner(record.owner)
    && typeof record.packSchemaHash === "string"
    && /^(?!0{64}$)[0-9a-f]{64}$/u.test(record.packSchemaHash);
}

function directoryMember(value: unknown): boolean {
  const record = directoryProjectionRecord(value, ["displayName", "email", "role", "userId"]);
  return record !== undefined
    && directoryProjectionText(record.userId)
    && directoryProjectionText(record.email)
    && directoryProjectionText(record.displayName)
    && (record.role === "author" || record.role === "spectator");
}

function directoryFrontier(value: unknown): boolean {
  const record = directoryProjectionRecord(value, ["commitSeq", "epoch", "headSeq"]);
  return record !== undefined && directoryProjectionInteger(record.headSeq) && directoryProjectionInteger(record.commitSeq) && directoryProjectionInteger(record.epoch);
}

function directoryDescriptor(value: unknown): boolean {
  const record = directoryProjectionRecord(value, ["artifactKind", "artifactSchema", "bootstrapFrontier", "bootstrapSnapshotHash", "bootstrapVersion", "documentId", "owner", "packSchemaHash", "spaceId"]);
  return record !== undefined
    && directoryProjectionText(record.spaceId)
    && directoryProjectionText(record.documentId)
    && directoryProjectionText(record.artifactKind)
    && directoryProjectionText(record.artifactSchema)
    && directoryOwner(record.owner)
    && typeof record.packSchemaHash === "string"
    && /^(?!0{64}$)[0-9a-f]{64}$/u.test(record.packSchemaHash)
    && directoryProjectionInteger(record.bootstrapVersion)
    && directoryFrontier(record.bootstrapFrontier)
    && typeof record.bootstrapSnapshotHash === "string"
    && /^(?!0{64}$)[0-9a-f]{64}$/u.test(record.bootstrapSnapshotHash);
}

function directoryDocument(value: unknown): boolean {
  const record = directoryProjectionRecord(value, ["commitSeq", "descriptor", "epoch", "headSeq"]);
  return record !== undefined && directoryDescriptor(record.descriptor) && directoryProjectionInteger(record.headSeq) && directoryProjectionInteger(record.commitSeq) && directoryProjectionInteger(record.epoch);
}

/** 🛡️ Strictly decodes one discriminated list projection. */
export function parseDirectorySpaceListEntryV1(value: unknown): DirectorySpaceListEntryV1 {
  const record = directoryProjectionRecord(value, ["access", "space"]);
  if (record?.access === "public" && directoryPublicSpace(record.space)) return record as DirectorySpaceListEntryV1;
  if (record?.access === "member" && directoryMemberSpaceRole(record.space, "spectator")) return record as DirectorySpaceListEntryV1;
  if (record?.access === "author" && directoryMemberSpaceRole(record.space, "author")) return record as DirectorySpaceListEntryV1;
  throw new Error("directory: invalid space list projection");
}

export function parseBrowserBrokerPortRequestV1(value: unknown): BrowserBrokerPortRequestV1 | undefined {
  if (!value || typeof value !== "object" || Array.isArray(value)) return undefined;
  const record = value as Readonly<Record<string, unknown>>;
  if (record.kind === "initialize" && exactRecordKeys(record, ["kind", "proof"]) && typeof record.proof === "string" && /^[0-9a-f]{64}$/u.test(record.proof)) return record as BrowserBrokerPortRequestV1;
  if (record.kind === "request" && exactRecordKeys(record, ["kind", "operation", "requestId"]) && record.operation === "me" && typeof record.requestId === "string" && /^[0-9a-f-]{36}$/u.test(record.requestId)) return record as BrowserBrokerPortRequestV1;
  if (record.kind === "cancel" && exactRecordKeys(record, ["kind", "requestId"]) && typeof record.requestId === "string" && /^[0-9a-f-]{36}$/u.test(record.requestId)) return record as BrowserBrokerPortRequestV1;
  if (record.kind === "close" && exactRecordKeys(record, ["kind"])) return record as BrowserBrokerPortRequestV1;
  return undefined;
}

export function parseBrowserBrokerPortResponseV1(value: unknown): BrowserBrokerPortResponseV1 | undefined {
  if (!value || typeof value !== "object" || Array.isArray(value)) return undefined;
  const record = value as Readonly<Record<string, unknown>>;
  if (record.kind === "initialized" && exactRecordKeys(record, ["kind", "ok"]) && typeof record.ok === "boolean") return record as BrowserBrokerPortResponseV1;
  if (record.kind !== "response" || !exactRecordKeys(record, ["body", "kind", "requestId", "status"]) || typeof record.requestId !== "string" || !/^[0-9a-f-]{36}$/u.test(record.requestId) || typeof record.status !== "number" || !Number.isSafeInteger(record.status) || record.status < 100 || record.status > 599 || typeof record.body !== "string" || new TextEncoder().encode(record.body).byteLength > 1024 * 1024) return undefined;
  return record as BrowserBrokerPortResponseV1;
}

/** 🔐 Narrow authority boundary that alone may retain an upstream session/share credential. */
export interface SocketGrantIssuerV1 {
  issueDirectory(options?: DirectoryRequestOptions): Promise<SocketGrantReceiptV1>;
  issueDirectoryScoped(scope: DocumentScope, options?: DirectoryRequestOptions): Promise<SocketGrantReceiptV1>;
  issueDocument(spaceId: string, documentId: string, options?: DirectoryRequestOptions): Promise<SocketGrantReceiptV1>;
}

/** 🌉 Credential-owning request port. Browser implementations target the local BFF; native
 * implementations inject the inherited-envelope bearer without exposing it to persisted bindings. */
export interface SocketGrantRequestPortV1 {
  post(path: string, options?: DirectoryRequestOptions): Promise<unknown>;
}

/** 🏗️ Builds the only socket issuer from a credential-owning request port. */
export function createSocketGrantIssuerV1(port: SocketGrantRequestPortV1): SocketGrantIssuerV1 {
  return {
    issueDirectory: async (options) => parseSocketGrantReceiptV1(await port.post("/directory/socket-grants", options)),
    issueDirectoryScoped: async (scope, options) => parseSocketGrantReceiptV1(await port.post(`/directory/spaces/${encodeURIComponent(scope.spaceId)}/documents/${encodeURIComponent(scope.documentId)}/socket-grants`, options)),
    issueDocument: async (spaceId, documentId, options) => parseSocketGrantReceiptV1(await port.post(`/spaces/${encodeURIComponent(spaceId)}/documents/${encodeURIComponent(documentId)}/socket-grants`, options)),
  };
}

const SOCKET_GRANT_PATTERN = /^socket\.v1\.[0-9a-f]{32}\.[0-9a-f]{64}$/;
const SOCKET_ACTOR_PATTERN = /^hub\.v1\.[0-9a-f]{64}$/;

/** 🛡️ Validates the schema-first receipt before any grant reaches a WebSocket API. */
export function parseSocketGrantReceiptV1(value: unknown): SocketGrantReceiptV1 {
  if (typeof value !== "object" || value === null) throw new Error("socket grant: invalid receipt");
  const receipt = value as Record<string, unknown>;
  if (
    receipt.schema !== "semio.hub.socket-grant/v1" ||
    receipt.protocol !== "semio.socket.v1" ||
    typeof receipt.grant !== "string" ||
    receipt.grant.length !== 107 ||
    !SOCKET_GRANT_PATTERN.test(receipt.grant) ||
    typeof receipt.actorId !== "string" ||
    !SOCKET_ACTOR_PATTERN.test(receipt.actorId) ||
    !Number.isSafeInteger(receipt.expiresAtMs)
  ) throw new Error("socket grant: invalid receipt");
  return receipt as SocketGrantReceiptV1;
}

/** 📡 Exact ordered subprotocol offer required by the v1 upgrade boundary. */
export function socketGrantProtocolsV1(receipt: SocketGrantReceiptV1): readonly ["semio.socket.v1", string] {
  return [receipt.protocol, receipt.grant];
}

/**
 * 📡️ Typed facade over the directory hub's REST/WS surface (contract-freeze §C2). Constructed per
 * identity (`baseUrl` + optional bearer `token`, mutated in place by {@link mintSession}), reused for
 * every call. No client-side caching or optimistic mutation of the read model — the hub log is the
 * single writer (contract §C6).
 */
export class DirectoryClient {
  private readonly baseUrl: string;
  private readonly requestBaseUrl: string;
  private readonly socketGrantIssuer: SocketGrantIssuerV1 | undefined;
  private readonly request: typeof fetchWithTimeout;

  constructor(baseUrl: string, options: { readonly requestBaseUrl?: string; readonly socketGrantIssuer?: SocketGrantIssuerV1; readonly request?: typeof fetchWithTimeout } = {}) {
    this.baseUrl = baseUrl.replace(/\/+$/, "");
    this.requestBaseUrl = (options.requestBaseUrl ?? baseUrl).replace(/\/+$/, "");
    this.socketGrantIssuer = options.socketGrantIssuer;
    this.request = options.request ?? fetchWithTimeout;
  }

  private headers(json: boolean): Record<string, string> {
    const headers: Record<string, string> = {};
    if (json) headers["content-type"] = "application/json";
    return headers;
  }

  /** 📨️ {@link FetchTimeoutResponse} plus the one extra accessor this class needs — declared locally
   * per this module's own body accessing only `json()` beyond the base shape. */
  private async getJson<T>(path: string, options?: DirectoryRequestOptions): Promise<T> {
    const response = await this.request(`${this.requestBaseUrl}${path}`, { credentials: "include", headers: this.headers(false) }, { timeoutMs: DIRECTORY_HTTP_TIMEOUT_MS, signal: options?.signal });
    if (!response.ok) throw new DirectoryHttpError(response.status, `directory: GET ${path} failed (${response.status})`);
    return (await response.json()) as T;
  }

  private async postJson<T>(path: string, body: unknown, options?: DirectoryRequestOptions): Promise<T> {
    const response = await this.request(
      `${this.requestBaseUrl}${path}`,
      { method: "POST", credentials: "include", headers: this.headers(true), body: JSON.stringify(body) },
      { timeoutMs: DIRECTORY_HTTP_TIMEOUT_MS, signal: options?.signal },
    );
    if (!response.ok) throw new DirectoryHttpError(response.status, `directory: POST ${path} failed (${response.status})`);
    return (await response.json()) as T;
  }

  /** 🪪️ `GET /auth/sessions/me` — `null` on 401 (no/expired session, the normal "not signed in"
   * outcome the boot flow branches on), throws {@link DirectoryHttpError} on any other failure. A
   * timeout/abort surfaces as a plain `Error` (no `.status`) — same "hub unreachable" shape the
   * caller's `directoryRejectionStatus`/identity-bootstrap catch already treats as offline, not a
   * boot-blocking failure (finding 1: this is what stops a hung server from hanging the boot path). */
  async me(options?: DirectoryRequestOptions): Promise<DirectorySessionAuthorityV1 | null> {
    try {
      const response = await this.request(`${this.requestBaseUrl}/auth/sessions/me`, { credentials: "include", headers: this.headers(false) }, { timeoutMs: DIRECTORY_HTTP_TIMEOUT_MS, signal: options?.signal });
      if (!response.ok) throw new DirectoryHttpError(response.status, `directory: GET /auth/sessions/me failed (${response.status})`);
      return parseDirectorySessionAuthorityJsonV1(await response.text());
    } catch (error) {
      if (error instanceof DirectoryHttpError && error.status === 401) return null;
      throw error;
    }
  }

  async spaces(options?: DirectoryRequestOptions): Promise<readonly DirectorySpaceListEntryV1[]> {
    const value = await this.getJson<unknown>("/directory/spaces", options);
    if (!Array.isArray(value)) throw new Error("directory: invalid space list projection");
    return value.map(parseDirectorySpaceListEntryV1);
  }

  /** 🏛️ Fetches one bounded canonical administration page, preserving the exact receipt bytes.
   * `cursor` advances precisely the window it was issued for; every other window restarts. */
  async spaceAdministrationPage(id: string, cursor?: string, options?: DirectoryRequestOptions): Promise<CanonicalDirectorySpaceAdministrationPageV1> {
    if (cursor !== undefined && (cursor.length === 0 || cursor.length > DIRECTORY_SPACE_ADMINISTRATION_CURSOR_MAX_BYTES || !/^[A-Za-z0-9._-]+$/u.test(cursor))) throw new Error("directory space administration: invalid cursor");
    const path = cursor === undefined ? `/directory/spaces/${encodeURIComponent(id)}` : `/directory/spaces/${encodeURIComponent(id)}?cursor=${cursor}`;
    const response = await this.request(`${this.requestBaseUrl}${path}`, { credentials: "include", headers: this.headers(false) }, { timeoutMs: DIRECTORY_HTTP_TIMEOUT_MS, signal: options?.signal });
    if (!response.ok) throw new DirectoryHttpError(response.status, `directory: GET ${path} failed (${response.status})`);
    const canonicalJson = await response.text();
    if (options?.signal?.aborted) throw options.signal.reason ?? new Error("directory space administration: cancelled");
    if (new TextEncoder().encode(canonicalJson).byteLength > DIRECTORY_SPACE_ADMINISTRATION_PAGE_MAX_BYTES) throw new Error("directory space administration: response too large");
    let page: DirectorySpaceAdministrationPageV1;
    try {
      page = await parseDirectorySpaceAdministrationPageV1(canonicalJson);
    } catch {
      throw new Error("directory space administration: invalid canonical response");
    }
    if (options?.signal?.aborted) throw options.signal.reason ?? new Error("directory space administration: cancelled");
    if (page.spaceId !== id) throw new Error("directory space administration: response space mismatch");
    return { canonicalJson, page };
  }

  /** 🧾️ Posts one sealed V1 request and parses only a raw-byte-capped canonical receipt bound to it.
   * A non-2xx becomes a closed {@link DirectoryCommandError} code; the response body is never read,
   * logged, or surfaced. The caller's `signal` cancels the HTTP wait only — a command already past
   * its server linearization point stays committed, so the operation becomes indeterminate. */
  async command(request: DirectoryCommandRequestV1, options?: DirectoryRequestOptions): Promise<DirectoryCommandReceiptV1> {
    const body = directoryCommandRequestJson(request);
    let response: FetchTimeoutResponse;
    try {
      response = await this.request(`${this.requestBaseUrl}/directory/commands`, { method: "POST", credentials: "include", headers: this.headers(true), body }, { timeoutMs: DIRECTORY_HTTP_TIMEOUT_MS, signal: options?.signal });
    } catch {
      throw new DirectoryCommandError(options?.signal?.aborted === true ? "cancelled" : "transport");
    }
    if (!response.ok) throw new DirectoryCommandError(directoryCommandErrorFromStatus(response.status));
    const canonicalJson = await response.text();
    if (options?.signal?.aborted === true) throw new DirectoryCommandError("cancelled");
    if (new TextEncoder().encode(canonicalJson).byteLength > DIRECTORY_COMMAND_RECEIPT_MAX_BYTES) throw new DirectoryCommandError("too-large");
    try {
      return await parseDirectoryCommandReceiptV1(canonicalJson, request);
    } catch {
      throw new DirectoryCommandError("invalid");
    }
  }

  async events(since: number, options?: DirectoryRequestOptions): Promise<readonly DirectoryEvent[]> {
    return this.getJson<DirectoryEvent[]>(`/directory/events?since=${encodeURIComponent(String(since))}`, options);
  }

  /** 📄️ Fetches and validates the original canonical page text without a JSON parse/reserialize gap. */
  async eventPage(after: number, options?: DirectoryRequestOptions): Promise<CanonicalDirectoryEventPageV1> {
    if (!Number.isSafeInteger(after) || after < 0) throw new Error("directory event page: invalid after frontier");
    const path = `/directory/event-page/v1?after=${after}`;
    const response = await this.request(`${this.requestBaseUrl}${path}`, { credentials: "include", headers: this.headers(false) }, { timeoutMs: DIRECTORY_HTTP_TIMEOUT_MS, signal: options?.signal });
    if (!response.ok) throw new DirectoryHttpError(response.status, `directory: GET ${path} failed (${response.status})`);
    const canonicalJson = await response.text();
    if (options?.signal?.aborted) throw options.signal.reason ?? new Error("directory event page: cancelled");
    if (new TextEncoder().encode(canonicalJson).byteLength > DIRECTORY_EVENT_PAGE_MAX_BYTES) throw new Error("directory event page: response too large");
    let page: DirectoryEventPageV1;
    try {
      page = await parseDirectoryEventPageV1(canonicalJson);
    } catch {
      throw new Error("directory event page: invalid canonical response");
    }
    if (options?.signal?.aborted) throw options.signal.reason ?? new Error("directory event page: cancelled");
    if (page.afterSeqExclusive !== after) throw new Error("directory event page: response frontier mismatch");
    return {
      canonicalJson,
      sessionBindingSha256: page.sessionBindingSha256,
      authorizationGeneration: page.authorizationGeneration,
      afterSeqExclusive: page.afterSeqExclusive,
      throughSeqInclusive: page.throughSeqInclusive,
      hasMore: page.hasMore,
      receiptSha256: page.receiptSha256,
    };
  }

  /** 🔌️ Opens the explicitly global directory stream, which is not document-scope authority. */
  stream(since: number, onMessage: (message: DirectoryStreamMessage) => void): DirectoryStream {
    return this.streamFor(undefined, since, onMessage, undefined, true);
  }

  /** 📌️ Opens a global wakeup stream that never treats observed events or heartbeats as committed. */
  streamAcknowledged(since: number, onMessage: (message: DirectoryStreamMessage) => void): DirectoryAcknowledgedStream {
    if (!Number.isSafeInteger(since) || since < 0) throw new Error("directory stream: invalid acknowledged frontier");
    return this.streamFor(undefined, since, onMessage, undefined, false);
  }

  /** 🎯️ Opens one exact document-scoped directory stream and terminally reports membership revocation. */
  streamScoped(scope: DocumentScope, since: number, onMessage: (message: DirectoryStreamMessage) => void, onRevoked?: () => void): DirectoryStream {
    if (!scope.spaceId || !scope.documentId || new TextEncoder().encode(scope.spaceId).byteLength > 4096 || new TextEncoder().encode(scope.documentId).byteLength > 4096) throw new Error("directory stream: invalid scope");
    return this.streamFor(scope, since, onMessage, onRevoked, true);
  }

  /** 🔌️ Fresh protected grant + scoped/global socket — subscribes from `since`, replays gap-free, then goes
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
   * the same instant. Mirrors `🧵️backbone-worker.ts`'s `connectHubOnce`/`connectHub` idiom for the
   * base reconnect loop; the health-reset addition here has no counterpart there (routed to that
   * file's own owning packet). */
  private streamFor(scope: DocumentScope | undefined, since: number, onMessage: (message: DirectoryStreamMessage) => void, onRevoked: (() => void) | undefined, trackObservedFrontier: boolean): DirectoryAcknowledgedStream {
    const abort = new AbortController();
    let socket: WebSocket | null = null;
    let lastSeq = since;
    let healthy = false; // 🩺️ set once THIS cycle's socket has been open for HUB_HEALTHY_RESET_MS.

    const wsUrl = (): string => {
      const wsBase = this.baseUrl.replace(/^http/, "ws");
      const query = new URLSearchParams();
      query.set("since", String(lastSeq));
      const path = scope
        ? `/directory/spaces/${encodeURIComponent(scope.spaceId)}/documents/${encodeURIComponent(scope.documentId)}/socket/v1`
        : "/directory/socket/v1";
      return `${wsBase}${path}?${query.toString()}`;
    };

    /** 🔌️ One WS connection attempt. Resolves once {@link close} aborts (a clean shutdown) OR once
     * the socket closes after having been open for {@link HUB_HEALTHY_RESET_MS} (a proven-healthy
     * drop — ends this cycle as a "success" so {@link runCycles} starts a fresh, reset one); rejects
     * on every other close/error/construct-throw, feeding the current cycle's growing jitter exactly
     * like before. The health timer is armed on open and always cleared on close, whichever reason —
     * never left pending past this promise settling. */
    const connectOnce = async (): Promise<void> => {
      const issuer = this.socketGrantIssuer;
      if (!issuer) throw new Error("directory stream: socket grant issuer unavailable");
      const receipt = parseSocketGrantReceiptV1(await (scope ? issuer.issueDirectoryScoped(scope, { signal: abort.signal }) : issuer.issueDirectory({ signal: abort.signal })));
      if (receipt.expiresAtMs <= Date.now()) throw new Error("directory stream: expired socket grant");
      return new Promise<void>((resolve, reject) => {
        if (abort.signal.aborted) {
          reject(abort.signal.reason ?? new Error("directory stream: closed"));
          return;
        }
        let ws: WebSocket;
        try {
          ws = new WebSocket(wsUrl(), [...socketGrantProtocolsV1(receipt)]);
        } catch (error) {
          reject(error);
          return;
        }
        socket = ws;
        const onAbort = (): void => ws.close();
        abort.signal.addEventListener("abort", onAbort, { once: true });
        let healthyTimer: ReturnType<typeof setTimeout> | null = null;
        ws.onopen = () => {
          if (ws.protocol !== "semio.socket.v1") {
            ws.close(1002, "socket protocol mismatch");
            return;
          }
          ws.send(encodeClientFrame({ SocketHelloV1: { wire_version: 1, protocol_version: 1, schema: "semio.directory.v1", pack_schema_hash: new Array(32).fill(0), resume_token: null, frontier: null } }, "command"));
          healthyTimer = setTimeout(() => {
            healthy = true;
          }, HUB_HEALTHY_RESET_MS);
        };
        ws.onmessage = (event: unknown) => {
          try {
            const data = (event as { data: unknown }).data;
            const message = JSON.parse(String(data)) as DirectoryStreamMessage;
            if (trackObservedFrontier && message.kind === "event") lastSeq = Math.max(lastSeq, message.event.seq);
            if (trackObservedFrontier && !scope && message.kind === "heartbeat") lastSeq = Math.max(lastSeq, message.headSeq);
            onMessage(message);
          } catch {
            // 🛟️ malformed frame — dropped, never thrown into the caller.
          }
        };
        ws.onclose = (event: CloseEvent) => {
          abort.signal.removeEventListener("abort", onAbort);
          if (healthyTimer != null) clearTimeout(healthyTimer);
          if (socket === ws) socket = null;
          if (scope && event.code === 4401) {
            onRevoked?.();
            abort.abort(new Error("directory stream: scope revoked"));
            resolve();
            return;
          }
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
    };

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
      acknowledge: (through: number) => {
        if (!Number.isSafeInteger(through) || through < lastSeq) throw new Error("directory stream: invalid acknowledged frontier");
        lastSeq = through;
      },
      close: () => {
        abort.abort();
        socket?.close();
      },
    };
  }
}

if (import.meta.vitest) {
  const { registerTests4 } = await import("./🧪️tests/🧪️backbone-envelope-io/🟦️.ts");
  await registerTests4(import.meta.vitest, { BACKBONE_WORKER_WIRE_MAGIC, DIRECTORY_HTTP_TIMEOUT_MS, DirectoryClient, HUB_HEALTHY_RESET_MS, HUB_RECONNECT_MAX_MS, HUB_RECONNECT_MIN_MS, decodeBackboneWorkerRequest, decodeBackboneWorkerResponse, decodePackValue, encodeBackboneMessage, encodeBackboneWorkerRequest, encodeBackboneWorkerResponse, encodePackValue, fetchWithTimeout, parseBrowserActorUiPatchOfferV1, parseBrowserActorUiPatchResultV1, parseDirectorySpaceAdministrationPageV1, parseDocumentBackboneMessage }, { directory: import.meta.dir, url: import.meta.url });
}
//#endregion 🔖️HubBinding
//#endregion 🔖️Directory
