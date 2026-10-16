/** 🧵️ The backbone: artifact opening relay, backbone URIs and envelope transport, binary backbone messages, desktop window controls, persistence bindings, the backbone worker wire protocol, and the workflow planner.
 *
 * 🧱️ Child module of {@link ../../🟦️.ts | os root}, which re-exports every public name below so import specifiers stay unchanged.
 */

import { parseRetainedCloneGrant, type RetainedCloneGrant } from "@semio-tech/value-ts/retained-clone/wire";
import { canonicalDirectoryCommandV1, directoryCommandRequestJson, parseDirectoryEventV1, sealDirectoryCommandRequestV1 } from "../📇️directory/🚪️io/📝️text/🟦️.ts";
import {
  parseInstalledServiceContributionsV1,
  parseInstalledServiceOperationV1,
  parseInstalledServiceStatusV1,
  parseInstalledServiceHistoryStatusV1,
  type InstalledServiceOperationV1,
  type InstalledServiceStatusV1,
  type InstalledServiceHistoryStatusV1
} from "../💡️inference/🔌️service/🟦️.ts";
import type {
  AppRef,
  AppRole,
  AppRouter,
  ArtifactDialect,
  FetchTimeoutResponse,
  OpeningPreferences,
  UtilityLeaf
} from "@semio-tech/framework";
import {
  dialectCoordinate,
  fetchWithTimeout,
  parseDialectCoordinate,
  parseSurfaceAppId,
  resolveOpeningApp,
  retryWithJitteredBackoff
} from "@semio-tech/framework";
import type { DirectoryCommand, DirectoryEvent, DirectoryStreamMessage } from "../📇️directory/🟦️.ts";
import { documentCheckInStatusFromValueV1, type DocumentCheckInStatusV1 } from "../📄️document/📌️check-in/🟦️.ts";
import {
  parseInferencePortClosedV1,
  parseInferencePortOpeningResultV1,
  type InferencePortClosedV1,
  type InferencePortOpeningResultV1
} from "../💡️inference/🚪️opening/🟦️.ts";
import type {
  ArtifactFrontier,
  DirectoryCommandErrorCodeV1,
  DirectoryCommandOutcomeV1,
  DirectoryCommandReceiptV1,
  DocumentExecutionTargetLeaseFieldsV1,
  DocumentExecutionTargetProgressV1,
  DocumentExecutionTargetStatusCodeV1
} from "../📇️directory/🧬️schema/🟦️.ts";
import {
  DIRECTORY_EVENT_PAGE_MAX_RAW_ROWS,
  DIRECTORY_SPACE_ADMINISTRATION_CURSOR_MAX_BYTES,
  artifactFrontierIsEditedForV1,
  artifactFrontierIsGenesisForV1
} from "../📇️directory/🧬️schema/🟦️.ts";
import type { ArtifactPresencePeer, ExactWireMutationEnvelope, MutationEnvelope } from "@semio-tech/framework-replication";
import {
  decodeCausalEnvelopeBatch,
  decodeDocumentBackboneEnvelopeBatchExact,
  encodeCausalEnvelopeBatch,
  readVarintU64,
  writeVarintU64
} from "@semio-tech/framework-replication";
import {
  parseBrowserActorUiPatchOfferV1,
  parseBrowserActorUiPatchResultV1,
  type BrowserActorUiPatchOfferV1,
  type BrowserActorUiPatchResultV1
} from "../🌐️browser-bundle/🩹️patch-handoff/🟦️.ts";
import {
  parseBrowserActorActionRequestV1,
  parseBrowserActorActionResultV1,
  type BrowserActorActionRequestV1,
  type BrowserActorActionResultV1
} from "../🌐️browser-bundle/🎯️action-handoff/🟦️.ts";
import { parseBrowserActorViewStateRequest, type BrowserActorViewStateRequest } from "../🌐️browser-bundle/🪟️view-context/🟦️.ts";
import type { DocumentScope } from "../📇️directory/🟦️.ts";
import {
  decodePackValue,
  encodePackValue,
  PACK_TAG_LIST,
  PACK_TAG_STR,
  PACK_TAG_STR_INLINE,
  packBuildSymbols,
  packEncodeString,
  packPushBytes,
  packReadVarintBigInt,
  replicationPackCodec
} from "../🎁️packs/🟦️.ts";
import {
  BACKBONE_GENESIS_MAXIMUM_BYTES,
  BACKBONE_HOT_MESSAGE_MAXIMUM_BYTES,
  type CommandAckOutcome,
  commandAckOutcomeOfValueV1,
  commandBatchIdOfValueV1,
  decodeDocumentArchiveBytes,
  DOCUMENT_ARCHIVE_MAXIMUM_BYTES
} from "../🪢️wire-contracts/🟦️.ts";
import { type DirectoryEventPageAckV1 } from "../📞️directory-client/🟦️.ts";

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

  const documentId = openingRelayString(record, "artifactId") ?? openingRelayString(record, "documentId");
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

/** 🔀️ The dev serve's stream channel (`semio.io.stream-mux/v1`): the ONE WebSocket a page holds to its serve origin for every
 * long-lived stream, so none of them occupies one of the six HTTP/1.1 connections a browser allows per origin. */
export const STREAM_MUX_PATH = "/semio-stream-mux";

/** 📣️ The `import.meta.env` key a serve that hosts {@link STREAM_MUX_PATH} defines for its pages (the dev serve's backbone
 * plugin does, during `serve` only); a page without it — a production bundle, a test harness — has no channel to dial. */
export const STREAM_MUX_ANNOUNCEMENT = "VITE_SEMIO_STREAM_MUX_PATH";

/** 🛣️ The routes the dev serve publishes on {@link STREAM_MUX_PATH}: staged plugin availability (snapshot + `built`),
 * installed extensions (snapshot + `installed`/`uninstalled`), one plugin's activation job (progress, cancellation) and the
 * change notices of one `folder://` backbone. */
export const DEV_STREAM_ROUTES = Object.freeze({
  pluginModules: "plugin-modules.watch",
  extensionModules: "extension-modules.watch",
  pluginActivation: "plugin-modules.activation",
  backboneFolder: "backbone.folder",
} as const);

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

/** 🔌️ `store::os_store::serialization::encode_document_pack_bytes` — length-prefixed `pack` then raw `spr`. */
export function encodeDocumentPackBytes(pack: Uint8Array, spr: Uint8Array): Uint8Array {
  const out: number[] = [];
  writeVarintU64(out, pack.length);
  for (const byte of pack) out.push(byte);
  for (const byte of spr) out.push(byte);
  return new Uint8Array(out);
}

/** 🎯️ Inverse of {@link encodeDocumentPackBytes}. */
export function decodeDocumentPackBytes(bytes: Uint8Array): { readonly pack: Uint8Array; readonly spr: Uint8Array } {
  const pos: [number] = [0];
  const packLen = readVarintU64(bytes, pos);
  const packEnd = pos[0] + packLen;
  if (packEnd > bytes.length) throw new Error("document pack bytes truncated");
  const pack = bytes.subarray(pos[0], packEnd);
  pos[0] = packEnd;
  return { pack, spr: bytes.subarray(pos[0]) };
}

/** 📦️ Packs a snapshot value into a document bundle (`pack` + `spr`). */
export function encodeDocumentPackBundle(snapshot: unknown, spr: Uint8Array = new Uint8Array()): Uint8Array {
  return encodeDocumentPackBytes(encodePackValue(snapshot), spr);
}

/** 📥️ Decodes the snapshot from a document bundle (ignores `spr` history). */
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
//#endregion 🌐️BackboneEnvelopeIo

//#region 🔀️BackboneMessage
export type BinaryBackboneMessage =
  | { readonly kind: "genesis"; readonly pack: Uint8Array }
  | { readonly kind: "mutations"; readonly envelopes: Uint8Array }
  | { readonly kind: "ack"; readonly opIds: readonly string[] }
  | { readonly kind: "retract"; readonly mutationIds: readonly string[] };

/** 🏷️ The Rust `BackboneMessage` variant ordinal of each kind this twin speaks; `Member` (3) is a Rust-only lane. */
const BACKBONE_MESSAGE_ORDINALS = { genesis: 0, mutations: 1, ack: 2, retract: 4 } as const;

/** 📤️ Canonical Store OpBinary: version, variant, symbol table and exact record fields. */
export function encodeBackboneMessage(message: BinaryBackboneMessage): Uint8Array {
  const maximum = message.kind === "genesis" ? BACKBONE_GENESIS_MAXIMUM_BYTES : BACKBONE_HOT_MESSAGE_MAXIMUM_BYTES;
  const encoder = new TextEncoder();
  const checkedBytes = (value: Uint8Array) => {
    if (!(value instanceof Uint8Array) || value.length > maximum) throw new Error("backbone message: byte limit");
    return value;
  };
  const ids = message.kind === "ack" ? message.opIds : message.kind === "retract" ? message.mutationIds : null;
  if (ids !== null) {
    if (ids.length > maximum / 2) throw new Error("backbone message: item limit");
    for (const value of ids) {
      if (typeof value !== "string" || value.length > maximum || encoder.encode(value).length > maximum || new TextDecoder("utf-8", { fatal: true }).decode(encoder.encode(value)) !== value) throw new Error("backbone message: invalid string");
    }
  } else if (message.kind === "genesis") checkedBytes(message.pack);
  else if (message.kind === "mutations") checkedBytes(message.envelopes);
  else throw new Error("backbone message: unknown kind");
  const symbols = ids !== null ? packBuildSymbols(ids) : [];
  const symbolIndex = new Map(symbols.map((value, index) => [value, index] as const));
  const out: number[] = [1, BACKBONE_MESSAGE_ORDINALS[message.kind]];
  const bounded = () => { if (out.length > maximum) throw new Error("backbone message: byte limit"); };
  writeVarintU64(out, symbols.length);
  for (const symbol of symbols) {
    const bytes = encoder.encode(symbol);
    writeVarintU64(out, bytes.length);
    packPushBytes(out, bytes);
    bounded();
  }
  writeVarintU64(out, 1);
  const byteField = (id: number, bytes: Uint8Array) => {
    writeVarintU64(out, id);
    out.push(0x08);
    writeVarintU64(out, bytes.length);
    packPushBytes(out, bytes);
    bounded();
  };
  if (message.kind === "genesis") {
    byteField(0, message.pack);
  } else if (message.kind === "mutations") {
    byteField(0, message.envelopes);
  } else {
    out.push(0, PACK_TAG_LIST);
    writeVarintU64(out, ids!.length);
    for (const value of ids!) { packEncodeString(value, symbolIndex, out); bounded(); }
  }
  bounded();
  return new Uint8Array(out);
}

/** 📥️ Bounded exact record decoding, with canonical bytes checked before admission. */
export function decodeBackboneMessage(bytes: Uint8Array): BinaryBackboneMessage {
  if (!(bytes instanceof Uint8Array) || bytes.length === 0 || bytes.length > BACKBONE_GENESIS_MAXIMUM_BYTES) throw new Error("backbone message: byte limit");
  if (bytes[0] !== 1) throw new Error("backbone message: invalid format");
  const pos: [number] = [1];
  const natural = (maximum: number) => {
    const start = pos[0];
    const value = packReadVarintBigInt(bytes, pos);
    if (value > BigInt(maximum) || (pos[0] - start > 1 && bytes[pos[0] - 1] === 0)) throw new Error("backbone message: invalid count");
    return Number(value);
  };
  const tag = natural(BACKBONE_MESSAGE_ORDINALS.retract);
  if (tag === 3) throw new Error("backbone message: member lanes are Rust-only");
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
  if (natural(1) !== 1) throw new Error("backbone message: field count");
  const byteField = (id: number) => {
    if (natural(1) !== id) throw new Error("backbone message: field id");
    expectTag(0x08);
    return take(natural(bytes.length - pos[0])).slice();
  };
  let message: BinaryBackboneMessage;
  if (tag === 0) {
    message = { kind: "genesis", pack: byteField(0) };
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
    message = tag === BACKBONE_MESSAGE_ORDINALS.retract ? { kind: "retract", mutationIds: opIds } : { kind: "ack", opIds };
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

/** 🔙️ Admits one canonical hot message an actor hands its store: a mutation batch, exactly as
 * {@link parseDocumentBackboneMessage} admits it, or the retraction of history transitions the hub refused. */
export function parseInboundDocumentBackboneMessage(message: Uint8Array): Uint8Array {
  if (!(message instanceof Uint8Array) || message.length > BACKBONE_HOT_MESSAGE_MAXIMUM_BYTES) throw new Error("document backbone: hot byte limit");
  if (decodeBackboneMessage(message).kind === "retract") return message.slice();
  return parseDocumentBackboneMessage(message).message;
}
//#endregion 🔀️BackboneMessage

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
 * browser blob cache (`🏪️store/👷️worker/🟦️.ts`) so all three stay in sync on the same literal. Backed by
 * `vcs::BlobStore`'s native counterpart; a hub-backed route is a later ticket. */
export const BLOB_ENDPOINT_PATH = "/semio-blob";
//#endregion 🔖️Blob

//#region 🔖️BackboneWorkerProtocol
export type DocumentRuntimeScopeV1 = Readonly<{ kind: "hub", dataClass: "persistedShared"; spaceId: string; documentId: string }> | Readonly<{ kind: "local"; documentId: string }>;

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

/** 🗃️ Durability × sharing class for persistence bindings and replication lanes.
 * Preview/presence are always `ephemeralShared` and must never be treated as durable WAL state. */
export type PersistenceDataClass = "persistedLocalOnly" | "persistedShared" | "ephemeralLocalOnly" | "ephemeralShared";

export type ClassifiedWireLane =
  | { readonly lane: "command"; readonly dataClass: "persistedLocalOnly" | "persistedShared" }
  | { readonly lane: "preview"; readonly dataClass: "ephemeralShared" }
  | { readonly lane: "presence"; readonly dataClass: "ephemeralShared" };

export function wireLaneDataClass(lane: "command" | "preview" | "presence"): PersistenceDataClass {
  if (lane === "preview" || lane === "presence") return "ephemeralShared";
  return "persistedShared";
}

export function bindingsDataClass(bindings: readonly PersistenceBinding[]): PersistenceDataClass {
  const first = bindings[0];
  if (!first) return "ephemeralLocalOnly";
  return first.dataClass;
}

export function folderPersistenceBinding(path: string): Extract<PersistenceBinding, { kind: "folder", dataClass: "persistedLocalOnly" }> {
  return { kind: "folder", path, dataClass: "persistedLocalOnly" };
}

export function ephemeralLocalPersistenceBinding(): Extract<PersistenceBinding, { kind: "ephemeral"; dataClass: "ephemeralLocalOnly" }> {
  return { kind: "ephemeral", dataClass: "ephemeralLocalOnly" };
}

export function ephemeralSharedPersistenceBinding(lane: "preview" | "presence" = "preview"): Extract<PersistenceBinding, { kind: "ephemeral"; dataClass: "ephemeralShared" }> {
  return { kind: "ephemeral", dataClass: "ephemeralShared", lane };
}

export function hubPersistenceBinding(
  baseUrl: string,
  spaceId: string,
  extras?: Readonly<{ requestedSurfaceId?: string; installedTarget?: DocumentExecutionTargetLeaseFieldsV1 }>,
): Extract<PersistenceBinding, { kind: "hub", dataClass: "persistedShared" }> {
  return { kind: "hub", baseUrl, spaceId, dataClass: "persistedShared", ...extras };
}

export type PersistenceBinding =
  | { readonly kind: "folder"; readonly path: string; readonly dataClass: "persistedLocalOnly" }
  /** ☁️ A hub binding states which surface it would like (`requestedSurfaceId`) and, when a previous
   * verified installation is already known, the complete {@link DocumentExecutionTargetLeaseFieldsV1}
   * to compare the next plan against. Neither is byte ownership: a non-`react` renderer target is
   * admitted only through a live private lease minted from server-verified bytes. */
  | { readonly kind: "hub"; readonly baseUrl: string; readonly spaceId: string; readonly dataClass: "persistedShared"; readonly requestedSurfaceId?: string; readonly installedTarget?: DocumentExecutionTargetLeaseFieldsV1 }
  | { readonly kind: "ephemeral"; readonly dataClass: "ephemeralLocalOnly" }
  | { readonly kind: "ephemeral"; readonly dataClass: "ephemeralShared"; readonly lane?: "preview" | "presence" };

/** 🧾️ Everything the worker needs to open one artifact's actor — mirrors `ArtifactActorConfig`. */
export type ArtifactActorConfig = {
  readonly documentId: string;
  readonly schema: string;
  readonly bindings: readonly PersistenceBinding[];
  readonly watchExternal?: boolean;
  readonly actor: string;
  readonly actorIdentityGrant: RetainedCloneGrant;
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
  | { readonly kind: "localDocumentArchive"; readonly archive: readonly number[] }
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

/** 📬️ Actor→subscriber events — mirrors Rust `ArtifactEvent`. */
export type ArtifactEvent =
  | { readonly kind: "remoteMutations"; readonly envelopes: readonly MutationEnvelope[] }
  | { readonly kind: "documentBackbone"; readonly message: Uint8Array }
  | { readonly kind: "documentArchiveReplaced"; readonly archive: readonly number[] }
  /** 🫙️ The bound folder holds no archive of this document: once per open, from the bootstrap folder read alone, never
   * after an archive was read or written — mirrors Rust `ArtifactEvent::DocumentArchiveAbsent`. */
  | { readonly kind: "documentArchiveAbsent" }
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

/** 📤️ Main thread → `🏪️store/👷️worker/🟦️.ts` — `bytes` is a UTF-8 worker wire payload (see {@link encodeBackboneWorkerRequest}). */
export type BackboneWorkerWireMessage = { readonly wire: Uint8Array };

/** 🧵️ Worker wire magic — must match `store_sync::backbone_worker_wire::MAGIC`. */
export const BACKBONE_WORKER_WIRE_MAGIC = 0x01;

function parseBackboneWorkerWire<T>(wire: Uint8Array, decode: (value: unknown) => T): T {
  if (wire.length === 0 || wire[0] !== BACKBONE_WORKER_WIRE_MAGIC) {
    throw new Error("backbone worker wire: unknown or empty payload");
  }
  return decode(decodePackValue(wire.subarray(1)));
}

/** 📇️ The most events one `directory-space-events` message carries: one sealed directory page's raw rows. */
export const DIRECTORY_SPACE_EVENTS_MAX_PER_MESSAGE = DIRECTORY_EVENT_PAGE_MAX_RAW_ROWS;

/** 🧵️ Encodes a {@link BackboneWorkerRequest} for the wasm `store_worker` (`handleRequestBytes`). */
export function encodeBackboneWorkerRequest(request: BackboneWorkerRequest): Uint8Array {
  const wire =
    request.kind === "send"
      ? { ...request, message: wireArtifactActorMsg(request.message) }
      : request;
  const packed = encodePackValue(wire);
  return new Uint8Array([BACKBONE_WORKER_WIRE_MAGIC, ...packed]);
}

/** 🧵️ Decodes a {@link BackboneWorkerRequest} from the wasm actor or structured-clone twin. */
export function decodeBackboneWorkerRequest(wire: Uint8Array): BackboneWorkerRequest {
  const parsed = parseBackboneWorkerWire(wire, (value) => value as Record<string, unknown>);
  if (parsed.kind === "service-contributions") return parseInstalledServiceContributionsV1(parsed);
  if (parsed.kind === "service-operation") return parseInstalledServiceOperationV1(parsed);
  if (typeof parsed.kind === "string" && parsed.kind.startsWith("directory-administration-")) return parseDirectoryAdministrationWorkerRequestV1(parsed);
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
  if (parsed.kind === "document-check-in" || parsed.kind === "document-check-in-cancel") {
    const start = parsed.kind === "document-check-in";
    if (Object.keys(parsed).sort().join(",") !== (start ? "clientInstanceId,kind,requestId,scope" : "kind,requestId,scope")) throw new Error("backbone worker request: invalid check-in fields");
    const requestId = workerWireCreationRequestIdV1(parsed.requestId);
    const documentId = typeof parsed.scope === "object" && parsed.scope !== null ? workerWireIdV1((parsed.scope as Record<string, unknown>).documentId) : null;
    const scope = documentId === null ? null : workerWireScopeV1(parsed.scope, documentId);
    if (requestId === null || scope === null) throw new Error("backbone worker request: invalid check-in owner");
    if (!start) return { kind: "document-check-in-cancel", requestId, scope };
    const clientInstanceId = workerWireClientInstanceIdV1(parsed.clientInstanceId);
    if (clientInstanceId === null) throw new Error("backbone worker request: invalid check-in client instance id");
    return { kind: "document-check-in", requestId, clientInstanceId, scope };
  }
  if (parsed.kind === "directory-space-open" || parsed.kind === "directory-space-close") {
    const open = parsed.kind === "directory-space-open";
    if (Object.keys(parsed).sort().join(",") !== (open ? "baseUrl,kind,spaceId" : "kind,spaceId")) throw new Error("backbone worker request: invalid space directory fields");
    const spaceId = workerWireCreationIdentityV1(parsed.spaceId);
    if (spaceId === null || (open && (typeof parsed.baseUrl !== "string" || parsed.baseUrl.length === 0 || parsed.baseUrl.length > 2048))) throw new Error("backbone worker request: invalid space directory owner");
    return open ? { kind: "directory-space-open", baseUrl: parsed.baseUrl as string, spaceId } : { kind: "directory-space-close", spaceId };
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
    if(parsed.kind === "open")parseRetainedCloneGrant(parsed.actorIdentityGrant);
    const clientInstanceId = parsed.clientInstanceId === undefined ? undefined : workerWireClientInstanceIdV1(parsed.clientInstanceId);
    if (parsed.clientInstanceId !== undefined && clientInstanceId === null) throw new Error("backbone worker request: invalid client instance id");
  }
  if (parsed.kind === "directory-command") {
    if (Object.keys(parsed).sort().join(",") !== "command,kind,requestId") throw new Error("backbone worker request: invalid directory command fields");
    const requestId = workerWireCreationRequestIdV1(parsed.requestId);
    if (requestId === null) throw new Error("backbone worker request: invalid directory command request id");
    return { kind: "directory-command", requestId, command: canonicalDirectoryCommandV1(parsed.command) };
  }

  return parsed as BackboneWorkerRequest;
}

/** 🧵️ Encodes a {@link BackboneWorkerResponse} from the wasm actor / TS fallback. */
export function encodeBackboneWorkerResponse(response: BackboneWorkerResponse): Uint8Array {
  const wire =
    response.kind === "event" ? { ...response, event: wireArtifactEvent(response.event) } : response;
  const packed = encodePackValue(wire);
  return new Uint8Array([BACKBONE_WORKER_WIRE_MAGIC, ...packed]);
}

/** 🧵️ Decodes a worker response/event wire payload from the wasm actor. */
export function decodeBackboneWorkerResponse(wire: Uint8Array): BackboneWorkerResponse {
  const parsed = parseBackboneWorkerWire(wire, (value) => value as Record<string, unknown>);
  if (parsed.kind === "browser-actor-action-result") {
    const clientInstanceId = workerWireClientInstanceIdV1(parsed.clientInstanceId);
    if (clientInstanceId === null) throw new Error("backbone worker response: invalid client instance id");
    const { clientInstanceId: _client, ...response } = parsed;
    return { ...parseBrowserActorActionResultV1(response), clientInstanceId };
  }
  if (parsed.kind === "space-artifact-creation-status") return parseSpaceArtifactCreationStatusV1(parsed);
  if (parsed.kind === "directory-space-events") {
    if (Object.keys(parsed).sort().join(",") !== "events,kind,spaceId") throw new Error("backbone worker response: invalid space directory fields");
    const spaceId = workerWireCreationIdentityV1(parsed.spaceId);
    if (spaceId === null || !Array.isArray(parsed.events) || parsed.events.length === 0 || parsed.events.length > DIRECTORY_SPACE_EVENTS_MAX_PER_MESSAGE) throw new Error("backbone worker response: invalid space directory owner");
    const events = parsed.events.map((value) => parseDirectoryEventV1(value));
    if (events.some((event, index) => event.spaceId !== spaceId || (index > 0 && events[index - 1]!.seq >= event.seq))) throw new Error("backbone worker response: invalid space directory events");
    return { kind: "directory-space-events", spaceId, events };
  }
  if (parsed.kind === "document-check-in-status") {
    if (Object.keys(parsed).sort().join(",") !== "clientInstanceId,kind,scope,status") throw new Error("backbone worker response: invalid check-in status fields");
    const clientInstanceId = workerWireClientInstanceIdV1(parsed.clientInstanceId);
    const documentId = typeof parsed.scope === "object" && parsed.scope !== null ? workerWireIdV1((parsed.scope as Record<string, unknown>).documentId) : null;
    const scope = documentId === null ? null : workerWireScopeV1(parsed.scope, documentId);
    const status = documentCheckInStatusFromValueV1(parsed.status);
    if (clientInstanceId === null || scope === null || status === null) throw new Error("backbone worker response: invalid check-in status");
    return { kind: "document-check-in-status", clientInstanceId, scope, status };
  }
  if (parsed.kind === "space-artifact-creation-catalog-refresh-required") return parseSpaceArtifactCreationCatalogRefreshRequiredV1(parsed);
  if (parsed.kind === "space-artifact-creation-catalog") return parseSpaceArtifactCreationCatalogV1(parsed);
  if (parsed.kind === "space-artifact-creation-catalog-status") return parseSpaceArtifactCreationCatalogStatusV1(parsed);
  if (parsed.kind === "browser-actor-ui-patch") {
    const clientInstanceId = workerWireClientInstanceIdV1(parsed.clientInstanceId);
    if (clientInstanceId === null) throw new Error("backbone worker response: invalid client instance id");
    const { clientInstanceId: _clientInstanceId, ...offer } = parsed;
    return { ...parseBrowserActorUiPatchOfferV1(offer), clientInstanceId };
  }
  if (parsed.kind === "browser-actor-ui-mounted") {
    const fields = ["activationGeneration", "activeCheckpointId", "browserActorSha256", "catalogGenerationId", "clientInstanceId", "componentSha256", "descriptorDigestV1", "descriptorSha256", "frontier", "instanceId", "kind", "scope", "uiRevision", "verifiedSurfaceId", "windowKindId"];
    if (Object.keys(parsed).sort().join(",") !== fields.sort().join(",")) throw new Error("backbone worker response: invalid mounted UI fields");
    const scopeRow = parsed.scope as Record<string, unknown> | undefined;
    const documentId = scopeRow === undefined ? null : workerWireIdV1(scopeRow.documentId);
    const scope = documentId === null ? null : workerWireScopeV1(parsed.scope, documentId);
    const clientInstanceId = workerWireClientInstanceIdV1(parsed.clientInstanceId);
    const verifiedSurfaceId = workerWireIdV1(parsed.verifiedSurfaceId);
    const windowKindId = workerWireIdV1(parsed.windowKindId);
    const activationGeneration = typeof parsed.activationGeneration === "string" && /^[1-9][0-9]{0,19}$/u.test(parsed.activationGeneration) && BigInt(parsed.activationGeneration) <= 0xffffffffffffffffn ? parsed.activationGeneration : null;
    if (scope === null || clientInstanceId === null || verifiedSurfaceId === null || windowKindId === null || activationGeneration === null || !Number.isSafeInteger(parsed.instanceId) || (parsed.instanceId as number) < 0 || !Number.isSafeInteger(parsed.uiRevision) || (parsed.uiRevision as number) < 1) throw new Error("backbone worker response: invalid mounted UI owner");
    const catalogGenerationId = workerWireSha256V1(parsed.catalogGenerationId),
      componentSha256 = workerWireSha256V1(parsed.componentSha256),
      descriptorSha256 = workerWireSha256V1(parsed.descriptorSha256),
      browserActorSha256 = workerWireSha256V1(parsed.browserActorSha256),
      activeCheckpointId = workerWireSha256V1(parsed.activeCheckpointId),
      descriptorDigestV1 = workerWireSha256V1(parsed.descriptorDigestV1),
      frontier = workerWireArtifactFrontierV1(parsed.frontier, scope);
    if (catalogGenerationId === null || componentSha256 === null || descriptorSha256 === null || browserActorSha256 === null || activeCheckpointId === null || descriptorDigestV1 === null || frontier === null) throw new Error("backbone worker response: invalid mounted UI identity");
    return { kind: "browser-actor-ui-mounted", scope, clientInstanceId, activationGeneration, instanceId: parsed.instanceId as number, verifiedSurfaceId, windowKindId, catalogGenerationId, componentSha256, descriptorSha256, browserActorSha256, activeCheckpointId, descriptorDigestV1, frontier, uiRevision: parsed.uiRevision as number };
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
    return { kind: "inference-port-status", operationEpoch: parsed.operationEpoch as number, scope, status: parseInstalledServiceStatusV1(parsed.status) };
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
    return { owner: typeof parsed.owner === "string" ? parsed.owner : (() => { throw new Error("service history: owner required"); })(), serviceId: typeof parsed.serviceId === "string" ? parsed.serviceId : (() => { throw new Error("service history: service required"); })(), kind: "inference-history-status", historyEpoch: parsed.historyEpoch as number, clientInstanceId, scope, status: parseInstalledServiceHistoryStatusV1(parsed.status) };
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

/** 🪆️ A DIALECT component on the same wire. `subset` is the one component whose canonical value is
 * not an identity at all: `SubsetId::ANY` is the literal `*` (`🚪️io/🧬️schema/🦀️.rs`), the
 * unconstrained base subset every standard carries and the subset of every kind this repository
 * ships. Validating it with `workerWireCreationIdentityV1` therefore rejected EVERY catalog the hub
 * can send: the main thread threw on decode, the shell kept `null`, and `createArtifact`'s kind
 * chooser rendered disabled with zero options on every space of every hub (measured 2026-09-22 on
 * hub 7651, whose own `GET /spaces/{id}/artifact-creations` answers two kinds, both `subset: "*"`). */
function workerWireDialectComponentV1(value: unknown): string | null {
  return value === "*" ? value : workerWireCreationIdentityV1(value);
}

function workerWireCreationRequestIdV1(value: unknown): string | null {
  return typeof value === "string" && /^(?!0{32}$)[0-9a-f]{32}$/u.test(value) ? value : null;
}

function workerWireCatalogGenerationIdV1(value: unknown): string | null {
  return typeof value === "string" && /^(?!0{64}$)[0-9a-f]{64}$/u.test(value) ? value : null;
}

function parseDirectoryAdministrationWorkerRequestV1(parsed: Readonly<Record<string, unknown>>): Extract<BackboneWorkerRequest, { readonly kind: `directory-administration-${string}` }> {
  const operationEpoch = parsed.operationEpoch;
  if (typeof operationEpoch !== "number" || !Number.isSafeInteger(operationEpoch) || operationEpoch < 0) throw new Error("backbone worker request: invalid administration operation epoch");
  const fields: Readonly<Record<string, readonly string[]>> = {
    "directory-administration-open": ["kind", "operationEpoch", "spaceId"],
    "directory-administration-refresh": ["kind", "operationEpoch", ...(parsed.cursor === undefined ? [] : ["cursor"])],
    "directory-administration-submit": ["kind", "operationEpoch", "requestId", "command"],
    "directory-administration-capability-request": ["kind", "operationEpoch"],
    "directory-administration-capability-result": ["kind", "operationEpoch", "transferEpoch", "copied"],
    "directory-administration-close": ["kind", "operationEpoch"],
  };
  const accepted = fields[String(parsed.kind)];
  if (accepted === undefined || Object.keys(parsed).sort().join(",") !== [...accepted].sort().join(",")) throw new Error("backbone worker request: invalid administration fields");
  switch (parsed.kind) {
    case "directory-administration-open": {
      const spaceId = typeof parsed.spaceId === "string" && new TextEncoder().encode(parsed.spaceId).length <= 256 && parsed.spaceId.length > 0 && !/\p{Cc}/u.test(parsed.spaceId) ? parsed.spaceId : null;
      if (spaceId === null) throw new Error("backbone worker request: invalid administration space");
      return { kind: parsed.kind, operationEpoch, spaceId };
    }
    case "directory-administration-refresh": {
      if (parsed.cursor === undefined) return { kind: parsed.kind, operationEpoch };
      if (typeof parsed.cursor !== "string" || parsed.cursor.length === 0 || parsed.cursor.length > DIRECTORY_SPACE_ADMINISTRATION_CURSOR_MAX_BYTES || !/^[A-Za-z0-9._-]+$/u.test(parsed.cursor)) throw new Error("backbone worker request: invalid administration cursor");
      return { kind: parsed.kind, operationEpoch, cursor: parsed.cursor };
    }
    case "directory-administration-submit": {
      const requestId = workerWireCreationRequestIdV1(parsed.requestId);
      if (requestId === null) throw new Error("backbone worker request: invalid administration request id");
      const command = canonicalDirectoryCommandV1(parsed.command);
      directoryCommandRequestJson(sealDirectoryCommandRequestV1(requestId, command));
      if (command.kind === "record-user-preference") throw new Error("backbone worker request: a user preference is not an administration command");
      const spaceId = command.kind === "create-space" ? undefined : command.kind === "announce-document" ? command.descriptor.spaceId : command.spaceId;
      if (spaceId !== undefined && workerWireIdV1(spaceId) === null) throw new Error("backbone worker request: invalid administration command space");
      return { kind: parsed.kind, operationEpoch, requestId, command };
    }
    case "directory-administration-capability-request": return { kind: parsed.kind, operationEpoch };
    case "directory-administration-capability-result": {
      if (typeof parsed.transferEpoch !== "number" || !Number.isSafeInteger(parsed.transferEpoch) || parsed.transferEpoch < 1 || typeof parsed.copied !== "boolean") throw new Error("backbone worker request: invalid administration capability result");
      return { kind: parsed.kind, operationEpoch, transferEpoch: parsed.transferEpoch, copied: parsed.copied };
    }
    case "directory-administration-close": return { kind: parsed.kind, operationEpoch };
    default: throw new Error("backbone worker request: invalid administration kind");
  }
}

function parseSpaceArtifactCreationWorkerRequestV1(parsed: Readonly<Record<string, unknown>>): Extract<BackboneWorkerRequest, { readonly kind: "space-artifact-create" | "space-artifact-create-cancel" }> {
  const create = parsed.kind === "space-artifact-create";
  const expected = create ? "expectedCatalogGenerationId,kind,kindId,name,requestId,spaceId" : "kind,requestId,spaceId";
  if (Object.keys(parsed).sort().join(",") !== expected) throw new Error("backbone worker request: invalid space artifact creation fields");
  const requestId = workerWireCreationRequestIdV1(parsed.requestId),
    spaceId = workerWireCreationIdentityV1(parsed.spaceId);
  if (requestId === null || spaceId === null) throw new Error("backbone worker request: invalid space artifact creation owner");
  if (!create) return { kind: "space-artifact-create-cancel", requestId, spaceId };
  const expectedCatalogGenerationId = workerWireCatalogGenerationIdV1(parsed.expectedCatalogGenerationId),
    kindId = workerWireCreationIdentityV1(parsed.kindId),
    name = workerWireTextV1(parsed.name);
  if (expectedCatalogGenerationId === null || kindId === null || name === null) throw new Error("backbone worker request: invalid space artifact creation intent");
  return { kind: "space-artifact-create", requestId, spaceId, expectedCatalogGenerationId, kindId, name };
}

/** 🔄️ Validates the request-bound local refresh disposition emitted for an initial POST conflict. */
export function parseSpaceArtifactCreationCatalogRefreshRequiredV1(value: unknown): SpaceArtifactCreationCatalogRefreshRequiredV1 {
  if (typeof value !== "object" || value === null || Array.isArray(value)) throw new Error("space artifact creation catalog refresh: invalid record");
  const row = value as Readonly<Record<string, unknown>>;
  if (Object.keys(row).sort().join(",") !== "catalogGenerationId,kind,requestId,spaceId") throw new Error("space artifact creation catalog refresh: invalid fields");
  const requestId = workerWireCreationRequestIdV1(row.requestId),
    spaceId = workerWireCreationIdentityV1(row.spaceId),
    catalogGenerationId = workerWireCatalogGenerationIdV1(row.catalogGenerationId);
  if (row.kind !== "space-artifact-creation-catalog-refresh-required" || requestId === null || spaceId === null || catalogGenerationId === null) throw new Error("space artifact creation catalog refresh: invalid owner");
  return { kind: "space-artifact-creation-catalog-refresh-required", requestId, spaceId, catalogGenerationId };
}

/** 🛡️ Validates the exact worker-visible projection of the Hub's durable creation status. */
export function parseSpaceArtifactCreationStatusV1(value: unknown): SpaceArtifactCreationStatusV1 {
  if (typeof value !== "object" || value === null || Array.isArray(value)) throw new Error("space artifact creation status: invalid record");
  const row = value as Readonly<Record<string, unknown>>;
  const phases: readonly SpaceArtifactCreationPhaseV1[] = ["accepted", "preparing", "ready", "indeterminate", "failed", "cancelled"];
  const phase = phases.includes(row.phase as SpaceArtifactCreationPhaseV1) ? (row.phase as SpaceArtifactCreationPhaseV1) : null;
  const requestId = workerWireCreationRequestIdV1(row.requestId),
    spaceId = workerWireCreationIdentityV1(row.spaceId),
    catalogGenerationId = workerWireCatalogGenerationIdV1(row.catalogGenerationId);
  if (row.kind !== "space-artifact-creation-status" || phase === null || requestId === null || spaceId === null || catalogGenerationId === null) throw new Error("space artifact creation status: invalid owner");
  const expected = phase === "ready" ? "catalogGenerationId,kind,phase,ready,requestId,spaceId" : "catalogGenerationId,kind,phase,requestId,spaceId";
  if (Object.keys(row).sort().join(",") !== expected) throw new Error("space artifact creation status: invalid fields");
  if (phase !== "ready") return { kind: "space-artifact-creation-status", requestId, spaceId, catalogGenerationId, phase };
  if (typeof row.ready !== "object" || row.ready === null || Array.isArray(row.ready)) throw new Error("space artifact creation status: invalid ready record");
  const ready = row.ready as Readonly<Record<string, unknown>>;
  if (Object.keys(ready).sort().join(",") !== "artifactId,artifactSchema,kindId,parentDialect") throw new Error("space artifact creation status: invalid ready fields");
  if (typeof ready.parentDialect !== "object" || ready.parentDialect === null || Array.isArray(ready.parentDialect)) throw new Error("space artifact creation status: invalid parent dialect");
  const parentDialect = ready.parentDialect as Readonly<Record<string, unknown>>;
  if (Object.keys(parentDialect).sort().join(",") !== "artifactKind,standard,subset") throw new Error("space artifact creation status: invalid parent dialect fields");
  const artifactId = typeof ready.artifactId === "string" && /^artifact-(?!0{32}$)[0-9a-f]{32}$/u.test(ready.artifactId) ? ready.artifactId : null,
    kindId = workerWireCreationIdentityV1(ready.kindId),
    artifactSchema = workerWireCreationIdentityV1(ready.artifactSchema),
    artifactKind = workerWireCreationIdentityV1(parentDialect.artifactKind),
    standard = workerWireDialectComponentV1(parentDialect.standard),
    subset = workerWireDialectComponentV1(parentDialect.subset);
  if (artifactId === null || kindId === null || artifactSchema === null || artifactKind === null || standard === null || subset === null) throw new Error("space artifact creation status: invalid ready identity");
  return { kind: "space-artifact-creation-status", requestId, spaceId, catalogGenerationId, phase, ready: { artifactId, kindId, artifactSchema, parentDialect: { artifactKind, standard, subset } } };
}

/** 🗂️ Validates the presentation-only selected-current creation catalog from the worker. A member's creation kind
 * (`kindId`, e.g. `2d.drawing`) and the dialect whose surface opens it (`dialect.artifactKind`, e.g. `s.draw.drawing`)
 * are the hub catalog's two independent names; neither is derived from the other here. */
export function parseSpaceArtifactCreationCatalogV1(value: unknown): SpaceArtifactCreationCatalogV1 {
  if (typeof value !== "object" || value === null || Array.isArray(value)) throw new Error("space artifact creation catalog: invalid record");
  const row = value as Readonly<Record<string, unknown>>;
  if (Object.keys(row).sort().join(",") !== "catalogGenerationId,clientInstanceId,kind,kinds,spaceId") throw new Error("space artifact creation catalog: invalid fields");
  const spaceId = workerWireCreationIdentityV1(row.spaceId),
    clientInstanceId = workerWireClientInstanceIdV1(row.clientInstanceId);
  const catalogGenerationId = workerWireCatalogGenerationIdV1(row.catalogGenerationId);
  if (row.kind !== "space-artifact-creation-catalog" || spaceId === null || clientInstanceId === null || catalogGenerationId === null || !Array.isArray(row.kinds) || row.kinds.length === 0 || row.kinds.length > 64)
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
      standard = workerWireDialectComponentV1(dialect.standard),
      subset = workerWireDialectComponentV1(dialect.subset),
      en = workerWireTextV1(label.en),
      de = workerWireTextV1(label.de);
    if (kindId === null || schema === null || artifactKind === null || standard === null || subset === null || en === null || de === null) throw new Error("space artifact creation catalog: invalid kind identity");
    return { kindId, schema, dialect: { artifactKind, standard, subset }, label: { en, de } };
  });
  if (kinds.some((entry, index) => index > 0 && kinds[index - 1]!.kindId >= entry.kindId)) throw new Error("space artifact creation catalog: invalid order");
  return { kind: "space-artifact-creation-catalog", clientInstanceId, spaceId, catalogGenerationId, kinds };
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
  | "deleted"
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
  artifactId: string;
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

/** 🔄️ Local disposition for an ambiguous initial POST conflict. It requests a fresh selected
 * catalog without asserting a Hub mismatch or retrying the completed creation request. */
export type SpaceArtifactCreationCatalogRefreshRequiredV1 = Readonly<{
  kind: "space-artifact-creation-catalog-refresh-required";
  requestId: string;
  spaceId: string;
  catalogGenerationId: string;
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

/** 📤️ Main thread → `🏪️store/👷️worker/🟦️.ts` messages (structured clone or {@link BackboneWorkerWireMessage}).
 * The `directory-*` kinds (contract-freeze §C6) are the shell's ONLY way to reach the directory hub
 * — plugin surfaces never talk to the network, and the shell never opens a directory socket on the
 * UI thread; see `🏪️store/👷️worker/🟦️.ts`'s `🔖️Directory` region. */
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
  /** 🌐️ The signed-in user's preference lane (`DIRECTORY_PREFERENCE_PAGE_PATH_V1`): pages from `after`, then a wake on
   * every directory head that passes the lane's frontier. Recording a change is a `record-user-preference` directory
   * command whose `requestId` is the change's own, so a resend is the same command. */
  | { readonly kind: "preference-lane-open"; readonly baseUrl: string; readonly userId: string; readonly after: number }
  | { readonly kind: "preference-lane-record"; readonly requestId: string; readonly schema: string; readonly mutation: string }
  | { readonly kind: "preference-lane-close" }
  | { readonly kind: "directory-scope-open"; readonly baseUrl: string; readonly scope: DocumentScope; readonly since: number }
  | { readonly kind: "directory-scope-close"; readonly scope: DocumentScope }
  /** 📇️ A mounted space index's directory lane: its space's full event history (sealed pages from the first event),
   * then every live event of that space. The space index is the shell's projection of a space's directory, not a hub
   * document, so it has no document scope and no document-scoped stream. */
  | { readonly kind: "directory-space-open"; readonly baseUrl: string; readonly spaceId: string }
  | { readonly kind: "directory-space-close"; readonly spaceId: string }
  | { readonly kind: "directory-command"; readonly requestId: string; readonly command: DirectoryCommand }
  | { readonly kind: "directory-command-cancel"; readonly requestId: string }
  | { readonly kind: "space-artifact-creation-catalog-open"; readonly clientInstanceId: string; readonly spaceId: string }
  | { readonly kind: "space-artifact-create"; readonly requestId: string; readonly spaceId: string; readonly expectedCatalogGenerationId: string; readonly kindId: string; readonly name: string }
  | { readonly kind: "space-artifact-create-cancel"; readonly requestId: string; readonly spaceId: string }
  /** 📌️ Hub Check In of one mounted hub document at the head its socket had acknowledged once every
   * local edit is acknowledged; the worker names the head, the host never does. */
  | { readonly kind: "document-check-in"; readonly requestId: string; readonly clientInstanceId: string; readonly scope: DocumentScope }
  | { readonly kind: "document-check-in-cancel"; readonly requestId: string; readonly scope: DocumentScope }
  | { readonly kind: "directory-administration-open"; readonly operationEpoch: number; readonly spaceId: string }
  | { readonly kind: "directory-administration-refresh"; readonly operationEpoch: number; readonly cursor?: string }
  | { readonly kind: "directory-administration-submit"; readonly operationEpoch: number; readonly requestId: string; readonly command: DirectoryCommand }
  | { readonly kind: "directory-administration-capability-request"; readonly operationEpoch: number }
  | { readonly kind: "directory-administration-capability-result"; readonly operationEpoch: number; readonly transferEpoch: number; readonly copied: boolean }
  | { readonly kind: "directory-administration-close"; readonly operationEpoch: number }
  | { readonly kind: "directory-close" }
  | ReturnType<typeof parseInstalledServiceContributionsV1>
  | (InstalledServiceOperationV1 & { readonly kind: "service-operation" })
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
      readonly code: "cancelled" | "deadline-exceeded" | "invalid-bootstrap" | "transport-failure" | "recovery-exhausted";
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

/** 📥️ `🏪️store/👷️worker/🟦️.ts` → main thread messages. `directory-status.pendingCommands` is the
 * bounded, in-memory offline queue's length (contract-freeze §C6 "commands queue... and flush on
 * reconnect"). */
export type BackboneWorkerResponse =
  | (BrowserActorActionResultV1 & { readonly clientInstanceId: string })
  | { readonly kind: "event"; readonly documentId: string; readonly clientInstanceId: string; readonly event: ArtifactEvent; readonly scope?: DocumentScope; readonly verifiedSurfaceId?: string }
  | ArtifactBootstrapWorkerEvent
  | { readonly kind: "ready" }
  | { readonly kind: "directory-message"; readonly message: DirectoryStreamMessage }
  | ({ readonly kind: "directory-event-page"; readonly canonicalJson: string } & DirectoryEventPageAckV1 & { readonly afterSeqExclusive: number; readonly hasMore: boolean })
  | { readonly kind: "preference-lane-page"; readonly userId: string; readonly afterSeqExclusive: number; readonly throughSeqInclusive: number; readonly hasMore: boolean; readonly events: readonly { readonly seq: number; readonly schema: string; readonly mutation: string }[] }
  | { readonly kind: "preference-lane-recorded"; readonly requestId: string; readonly outcome: string }
  | { readonly kind: "preference-lane-failed"; readonly requestId: string | null; readonly code: string; readonly retryable: boolean }
  | { readonly kind: "directory-bootstrap-failed"; readonly bootstrapEpoch: number; readonly code: "unauthorized" | "cancelled" | "transport" | "invalid-page"; readonly retryable: boolean }
  | { readonly kind: "directory-scope-revoked"; readonly scope: DocumentScope }
  /** 📇️ Events of one space for its mounted index lane, in ascending `seq` (history first, then live). */
  | { readonly kind: "directory-space-events"; readonly spaceId: string; readonly events: readonly DirectoryEvent[] }
  | { readonly kind: "directory-command-receipt"; readonly requestId: string; readonly receipt: DirectoryCommandReceiptV1 }
  | { readonly kind: "directory-command-failed"; readonly requestId: string; readonly code: DirectoryCommandErrorCodeV1 }
  | SpaceArtifactCreationCatalogV1
  | SpaceArtifactCreationCatalogStatusV1
  | SpaceArtifactCreationStatusV1
  | SpaceArtifactCreationCatalogRefreshRequiredV1
  | { readonly kind: "socket-actor"; readonly documentId: string; readonly clientInstanceId: string; readonly scope?: DocumentScope; readonly actorId: string }
  /** 📌️ One hub Check In status, exactly as the hub answered it (or a local `failed` when the
   * document had no acknowledged head to name). */
  | { readonly kind: "document-check-in-status"; readonly clientInstanceId: string; readonly scope: DocumentScope; readonly status: DocumentCheckInStatusV1 }
  | { readonly kind: "socket-actor-failed"; readonly documentId: string; readonly clientInstanceId: string; readonly scope?: DocumentScope; readonly code: "installed-target-unavailable" | "session-mismatch" }
  /** 🪪️ Bounded execution-target install status for the React host's localized live region. It
   * carries a status code and byte counters only — never bytes, an origin, a path, a module URL, a
   * receipt, a grant or a digest. `diagnostic` is the one exception and never reaches the localized
   * text: it is the bounded failure line the owner itself named, surfaced beside the notice so a
   * terminal outcome is readable instead of anonymous. */
  | { readonly kind: "execution-target-status"; readonly documentId: string; readonly clientInstanceId: string; readonly spaceId: string; readonly scope?: DocumentScope; readonly code: DocumentExecutionTargetStatusCodeV1; readonly progress?: DocumentExecutionTargetProgressV1; readonly diagnostic?: string }
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
  | { readonly kind: "inference-port-status"; readonly operationEpoch: number; readonly scope: DocumentScope; readonly status: InstalledServiceStatusV1 }
  | InferencePortOpeningResultV1
  | InferencePortClosedV1
  | { readonly owner: string; readonly serviceId: string; readonly kind: "inference-history-status"; readonly historyEpoch: number; readonly clientInstanceId: string; readonly scope: DocumentScope; readonly status: InstalledServiceHistoryStatusV1 }
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
  /** 🪟️ The lease's window: the one whose acknowledged revision `uiRevision` is. */
  windowKindId: string;
  catalogGenerationId: string;
  componentSha256: string;
  descriptorSha256: string;
  browserActorSha256: string;
  activeCheckpointId: string;
  descriptorDigestV1: string;
  frontier: ArtifactFrontier;
  uiRevision: number;
}>;

function wireArtifactActorMsg(message: ArtifactActorMsg): unknown {
  if (message.kind === "documentBackbone") {
    return { kind: "documentBackbone", message: Array.from(parseDocumentBackboneMessage(message.message).message) };
  }
  if (message.kind === "localMutations") {
    return { kind: "localMutations", envelopes: encodeCausalEnvelopeBatch(message.envelopes, replicationPackCodec) };
  }
  if (message.kind === "localDocumentArchive") {
    const archive = Uint8Array.from(message.archive);
    decodeDocumentArchiveBytes(archive);
    return { kind: "localDocumentArchive", archive: Array.from(archive) };
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
  if (message.kind === "localDocumentArchive") {
    if (Object.keys(message).sort().join(",") !== "archive,kind" || !Array.isArray(message.archive) || message.archive.length === 0 || message.archive.length > DOCUMENT_ARCHIVE_MAXIMUM_BYTES || !message.archive.every((entry) => Number.isInteger(entry) && entry >= 0 && entry <= 255)) throw new Error("backbone worker request: invalid document archive");
    const archive = Uint8Array.from(message.archive as readonly number[]);
    decodeDocumentArchiveBytes(archive);
    return { kind: "localDocumentArchive", archive: Array.from(archive) };
  }
  return message as ArtifactActorMsg;
}

function wireArtifactEvent(event: ArtifactEvent): unknown {
  if (event.kind === "documentBackbone") {
    return { kind: "documentBackbone", message: Array.from(parseInboundDocumentBackboneMessage(event.message)) };
  }
  if (event.kind === "remoteMutations") {
    return { kind: "remoteMutations", envelopes: encodeCausalEnvelopeBatch(event.envelopes, replicationPackCodec) };
  }
  if (event.kind === "documentArchiveReplaced") {
    const archive = Uint8Array.from(event.archive);
    decodeDocumentArchiveBytes(archive);
    return { kind: "documentArchiveReplaced", archive: Array.from(archive) };
  }
  return event;
}

function parseArtifactEvent(event: Record<string, unknown>): ArtifactEvent {
  if (event.kind === "documentBackbone") {
    if (Object.keys(event).sort().join(",") !== "kind,message" || !Array.isArray(event.message) || event.message.length > BACKBONE_HOT_MESSAGE_MAXIMUM_BYTES || !event.message.every((entry) => Number.isInteger(entry) && entry >= 0 && entry <= 255)) throw new Error("backbone worker response: invalid document backbone message");
    return { kind: "documentBackbone", message: parseInboundDocumentBackboneMessage(Uint8Array.from(event.message as readonly number[])) };
  }
  if (event.kind === "remoteMutations" && Array.isArray(event.envelopes) && event.envelopes.every((entry) => typeof entry === "number")) {
    return { kind: "remoteMutations", envelopes: decodeCausalEnvelopeBatch(event.envelopes as readonly number[], replicationPackCodec) };
  }
  if (event.kind === "documentArchiveReplaced") {
    if (Object.keys(event).sort().join(",") !== "archive,kind" || !Array.isArray(event.archive) || event.archive.length === 0 || event.archive.length > DOCUMENT_ARCHIVE_MAXIMUM_BYTES || !event.archive.every((entry) => Number.isInteger(entry) && entry >= 0 && entry <= 255)) throw new Error("backbone worker response: invalid document archive");
    const archive = Uint8Array.from(event.archive as readonly number[]);
    decodeDocumentArchiveBytes(archive);
    return { kind: "documentArchiveReplaced", archive: Array.from(archive) };
  }
  if (event.kind === "documentArchiveAbsent") {
    if (Object.keys(event).join(",") !== "kind") throw new Error("backbone worker response: invalid document archive absence");
    return { kind: "documentArchiveAbsent" };
  }
  if (event.kind === "commandOutcome") {
    const batchId = commandBatchIdOfValueV1(event.batchId);
    const outcome = commandAckOutcomeOfValueV1(event.outcome);
    if (Object.keys(event).sort().join(",") !== "batchId,kind,outcome" || batchId === null || outcome === null) throw new Error("backbone worker response: invalid command outcome");
    return { kind: "commandOutcome", batchId, outcome };
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
