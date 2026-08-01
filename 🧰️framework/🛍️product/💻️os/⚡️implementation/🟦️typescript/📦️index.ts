// #region Header
/**
 * 🖥️ `@semio-tech/framework-os-core` — JS sync/backbone protocol surface (backbone URIs, document
 * envelopes, `🟦️backbone-worker.ts` request/response wire types, `PersistenceBinding`/`OperationEnvelope`,
 * {@link buildFrameworkSyncUtilities}) consumed by `framework/os/renderer/js/react/index.tsx` and
 * `framework/os/dev/script.ts`. The OS kernel's *stateful* logic (operation application, program
 * registry) is Rust/wasm-only, hosted by the s-plugin wasm — this file is not a JS port of that. The
 * one exception is {@link planWorkflow}: a pure, side-effect-free scheduling function has no state
 * to keep in sync with a live wasm host, so it's hand-mirrored here against the Rust `plan_workflow`
 * (`framework/os/core/rs/lib.rs`) with shared fixtures (`framework/os/core/fixtures/`)
 * asserting parity. This file still exposes a small legacy `osBaselineArtifact`/
 * `mergeOsWorkflowDefinition`/`registerAppVcsHandler` app-registration shim kept alive only because
 * `compose/client/lib/sketchpad/js/index.ts` still calls it; do not extend that shim further.
 */
// #endregion Header

import type { PluginWasmHandle, UtilityLeaf } from "@semio-tech/framework-core";

export type OsPluginArtifactMap = Readonly<Record<string, { readonly kind: string; readonly id: string; readonly label: string }>>;

const programDefinitions = new Map<string, unknown>();
const vcsHandlers = new Set<() => void>();

export function osBaselineArtifact(kind: string, id: string, label: string) {
  return { kind, id, label };
}

export function mergeOsWorkflowDefinition(pluginId: string, definition: unknown, resources?: OsPluginArtifactMap): void {
  programDefinitions.set(pluginId, { definition, resources });
}

export function registerAppVcsHandler(handler: () => void): void {
  vcsHandlers.add(handler);
}

//#region 🔖️Backbone
export const FRAMEWORK_SYNC_CONTROLLER_ID = "framework.sync";

/** 🛰️ Dev-server-proxied backbone endpoint path for `file://`/`folder://` uris; shared with the dev host shim (`framework/os/dev/script.ts`) so both stay in sync on the same literal. */
export const BACKBONE_ENDPOINT_PATH = "/semio-backbone";

export type BackboneKind = "file" | "folder" | "remote" | "unknown";

export type DocumentBackboneRef = {
  readonly kind: BackboneKind;
  readonly uri: string;
};

export function backboneKindFromUri(uri: string): BackboneKind {
  if (uri.startsWith("file://")) return "file";
  if (uri.startsWith("folder://")) return "folder";
  if (uri.startsWith("remote://")) return "remote";
  return "unknown";
}

export function documentBackboneRef(uri: string): DocumentBackboneRef {
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

/** @emoji 📦️ Packs a projection value into a document bundle (`pack` + `spr`). */
export function encodeDocumentPackBundle(projection: unknown, spr: Uint8Array = new Uint8Array()): Uint8Array {
  return encodeDocumentPackBytes(encodePackValue(projection), spr);
}

/** @emoji 📥️ Decodes the projection from a document bundle (ignores `spr` history). */
export function decodeDocumentPackProjection(bundle: Uint8Array): unknown {
  const { pack } = decodeDocumentPackBytes(bundle);
  return decodePackValue(pack);
}

const BACKBONE_OCTET_STREAM = "application/octet-stream";

export async function readBackboneEnvelope(uri: string): Promise<Uint8Array | null> {
  if (uri.startsWith("remote://")) {
    const remote = parseRemoteBackboneUri(uri);
    if (!remote) return null;
    const response = await fetch(remoteEnvelopeUrl(remote));
    if (response.status === 404) return null;
    if (!response.ok) throw new Error(`remote backbone read failed (${response.status})`);
    return new Uint8Array(await response.arrayBuffer());
  }
  const response = await fetch(`${BACKBONE_ENDPOINT_PATH}?uri=${encodeURIComponent(uri)}`);
  if (response.status === 404) return null;
  if (!response.ok) throw new Error(`backbone read failed (${response.status})`);
  return new Uint8Array(await response.arrayBuffer());
}

export async function writeBackboneEnvelope(uri: string, bundle: Uint8Array): Promise<void> {
  if (uri.startsWith("remote://")) {
    const remote = parseRemoteBackboneUri(uri);
    if (!remote) throw new Error(`invalid remote backbone uri: ${uri}`);
    const response = await fetch(remoteEnvelopeUrl(remote), {
      method: "PUT",
      headers: { "content-type": BACKBONE_OCTET_STREAM },
      body: bundle,
    });
    if (!response.ok) throw new Error(`remote backbone write failed (${response.status})`);
    console.log("[DEBUG] remote backbone synced", uri);
    return;
  }
  const response = await fetch(`${BACKBONE_ENDPOINT_PATH}?uri=${encodeURIComponent(uri)}`, {
    method: "PUT",
    headers: { "content-type": BACKBONE_OCTET_STREAM },
    body: bundle,
  });
  if (!response.ok) throw new Error(`backbone write failed (${response.status})`);
  console.log("[DEBUG] backbone synced", uri);
}

/** @deprecated Use {@link decodeDocumentPackProjection}. */
export function documentFromEnvelopeJson(_envelopeJson: string): unknown {
  throw new Error("documentFromEnvelopeJson removed — use decodeDocumentPackProjection on binary bundle bytes");
}

/** @deprecated Use {@link encodeDocumentPackBundle}. */
export function wrapDocumentEnvelope(_document: unknown, _documentId: string, _uri: string): string {
  throw new Error("wrapDocumentEnvelope removed — use encodeDocumentPackBundle");
}

//#region 🔀️ApplyBackboneMessage
export type BinaryBackboneMessage =
  | { readonly kind: "snapshot"; readonly pack: Uint8Array; readonly spr: Uint8Array }
  | { readonly kind: "operations"; readonly envelopes: readonly WireOperationEnvelope[] }
  | { readonly kind: "ack"; readonly opIds: readonly string[] };

/** @emoji 🎯️ TS twin of `store::encode_backbone_message`. */
export function encodeBackboneMessage(message: BinaryBackboneMessage): Uint8Array {
  const out: number[] = [];
  if (message.kind === "snapshot") {
    out.push(0);
    writeBytes(out, Array.from(message.pack));
    writeBytes(out, Array.from(message.spr));
  } else if (message.kind === "operations") {
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
    return { kind: "operations", envelopes: readVecEnvelope(bytes, pos) };
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
  if (message.kind === "operations") {
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

//#region 🔖️SyncProtocol
/**
 * 🔁️ TS mirror of `store_sync`'s Rust actor protocol (`DocumentActorConfig`/`DocumentActorMsg`/
 * `DocumentEvent`/`DocumentSyncStatus`/`RemoteState`/`PersistenceBinding`) — the wire/postMessage
 * shapes `🟦️backbone-worker.ts` speaks, kept camelCase-tag-identical to the Rust side (`#[serde(tag =
 * "kind", rename_all = "camelCase")]`) so a shared JSON fixture suite (`store/sync/fixtures/`)
 * stays plausible across both runtimes even though this file is a deliberately dumb TS twin (no
 * materialization — it only relays queues, exactly like the Rust actor's `ChannelBackbone` side).
 */
export type OperationEnvelope = {
  readonly id: string;
  readonly actor: string;
  readonly document: string;
  readonly schemaVersion: string;
  readonly deps?: readonly string[];
  readonly payloadHash: string;
  readonly diff: { readonly schemaId: string; readonly payload: unknown };
  readonly inverse: {
    readonly targetOperation: string;
    readonly inverseDiff: { readonly schemaId: string; readonly payload: unknown };
    readonly baseVersion: number;
    readonly dependencies?: readonly string[];
    readonly undoPolicy: string;
  };
};

/** 🌉️ Maps the actor-protocol {@link OperationEnvelope} into a {@link WireOperationEnvelope}. */
export function operationEnvelopeToWire(envelope: OperationEnvelope, timestamp: WireOperationEnvelope["timestamp"]): WireOperationEnvelope {
  const packPayload = (value: unknown) => Array.from(encodePackValue(value));
  return {
    operation_id: envelope.id,
    document_id: envelope.document,
    actor: envelope.actor,
    dependencies: [...(envelope.deps ?? [])],
    diff: { schema: envelope.diff.schemaId, payload: packPayload(envelope.diff.payload) },
    inverse: { schema: envelope.inverse.inverseDiff.schemaId, payload: packPayload(envelope.inverse.inverseDiff.payload) },
    timestamp,
  };
}

/** 🌉️ Inverse of {@link operationEnvelopeToWire}. */
export function operationEnvelopeFromWire(envelope: WireOperationEnvelope): OperationEnvelope {
  const decodePayload = (bytes: readonly number[]) => decodePackValue(new Uint8Array(bytes));
  const payload = decodePayload(envelope.diff.payload);
  const sequenceNumber = payload !== null && typeof payload === "object" && "sequenceNumber" in payload ? Number((payload as Record<string, unknown>).sequenceNumber) : 0;
  return {
    id: envelope.operation_id,
    actor: envelope.actor,
    document: envelope.document_id,
    schemaVersion: envelope.diff.schema,
    deps: [...envelope.dependencies],
    payloadHash: "",
    diff: { schemaId: envelope.diff.schema, payload },
    inverse: {
      targetOperation: envelope.operation_id,
      inverseDiff: { schemaId: envelope.inverse.schema, payload: decodePayload(envelope.inverse.payload) },
      baseVersion: Number.isFinite(sequenceNumber) ? Math.max(0, sequenceNumber) : 0,
      dependencies: [],
      undoPolicy: "exactBaseOnly",
    },
  };
}

/** 📡️ Wire-protocol presence identity — distinct from the UI-rendering {@link PresencePeer} scene prop. */
export type DocumentPresencePeer = {
  readonly actor: string;
  readonly label?: string;
  readonly selectionJson?: string;
  readonly connectedAtMs: number;
  readonly userId?: string;
  readonly role?: string;
  readonly cursor?: { readonly x: number; readonly y: number };
  readonly viewport?: { readonly x: number; readonly y: number; readonly zoom: number };
  readonly dragGhostJson?: string;
};

/** 🌐️ One causally-ordered operation crossing the wire — mirrors Rust `protocol_causal::
 * OperationEnvelope` byte-for-byte. Wire-only shape, distinct from {@link OperationEnvelope} (this
 * file's postMessage/actor-protocol shape, camelCase-tagged): this type crosses `protocol_wire`'s
 * binary codec (see `encodeClientFrame`/`decodeClientFrame` below), where Rust field names are
 * plain (not renamed), so it stays snake_case like the Rust source. 🎯️ W5: `diff`/`inverse` payloads
 * are opaque bytes now (a JSON number array here, matching every other `Vec<u8>` field on this
 * boundary), not a schema-erased JSON value — `protocol_causal::DocumentDiff`/`InverseOperation`
 * both flipped from `serde_json::Value` to `Vec<u8>`. */
export type WireOperationEnvelope = {
  readonly operation_id: string;
  readonly document_id: string;
  readonly actor: string;
  readonly dependencies: readonly string[];
  readonly diff: { readonly schema: string; readonly payload: readonly number[] };
  readonly inverse: { readonly schema: string; readonly payload: readonly number[] };
  readonly timestamp: { readonly actor: number; readonly physical_ms: number; readonly logical: number };
};

/** 🏔️ Runtime/wire frontier summary — mirrors Rust `protocol_causal::FrontierSummary`
 * (`protocol::RuntimeFrontierSummary`). */
export type WireFrontierSummary = {
  readonly document_id: string;
  readonly head_edit_ordinal: number;
  readonly head_edit_id: string;
  readonly last_commit_seq: number;
  readonly chain_hash: readonly number[];
};

/** 🛣️ Which logical channel a wire frame travels on — mirrors Rust `protocol_wire::Lane`. */
export type WireLane = "command" | "preview";

/** 🚀️ How a `ServerFrame.Welcome` seeds a client — mirrors Rust `protocol_wire::Bootstrap`. */
export type WireBootstrap = "None" | { readonly Snapshot: { readonly pack_hash: readonly number[]; readonly inline: readonly number[] | null } } | "Tail";

/** ⚖️ How the hub resolved one submitted batch against concurrent history — mirrors Rust
 * `protocol_wire::ApplyOutcome`. */
export type WireApplyOutcome = "Accepted" | { readonly Transformed: { readonly envelope: WireOperationEnvelope } } | { readonly Rejected: { readonly reason: string } };

/** 🪜️ One stage of a submitted batch's lifecycle — mirrors Rust `protocol_wire::AckStage`. */
export type WireAckStage = "Received" | "Persisted" | { readonly Applied: { readonly outcome: WireApplyOutcome } };

/** 📨️ Client→server hub wire frames — mirrors Rust `protocol_wire::ClientFrame` byte-for-byte.
 * Externally-tagged plain enum (serde's default representation, no `#[serde(tag = ...)]` on the
 * Rust side): a struct variant serializes as `{ VariantName: { ...fields } }`, a unit variant as
 * the bare string `"VariantName"`. Encode/decode with {@link encodeClientFrame}/
 * {@link decodeClientFrame} below — never hand-construct the JSON. */
export type ClientFrame =
  | {
      readonly Hello: {
        readonly wire_version: number;
        readonly protocol_version: number;
        readonly schema: string;
        readonly pack_schema_hash: readonly number[];
        readonly actor: string;
        readonly token: string | null;
        readonly resume_token: string | null;
        readonly frontier: WireFrontierSummary | null;
      };
    }
  | { readonly Commands: { readonly batch_id: number; readonly envelopes: readonly WireOperationEnvelope[] } }
  | { readonly FrontierAdvertise: { readonly frontier: WireFrontierSummary } }
  | { readonly PreviewPublish: { readonly key: string; readonly seq: number; readonly payload: readonly number[] } }
  | { readonly Presence: { readonly peer: readonly number[] } }
  | { readonly CreditGrant: { readonly n: number } }
  | "Bye";

/** 📬️ Server→client hub wire frames — mirrors Rust `protocol_wire::ServerFrame` byte-for-byte. See
 * {@link ClientFrame}'s doc comment for the externally-tagged encoding this shares. */
export type ServerFrame =
  | { readonly Welcome: { readonly session_id: string; readonly resume_token: string; readonly server_frontier: WireFrontierSummary; readonly bootstrap: WireBootstrap } }
  | { readonly SnapshotChunk: { readonly seq: number; readonly bytes: readonly number[] } }
  | { readonly SnapshotDone: { readonly seq_count: number } }
  | { readonly Commands: { readonly envelopes: readonly WireOperationEnvelope[]; readonly origin: string; readonly frontier: WireFrontierSummary } }
  | { readonly Ack: { readonly batch_id: number; readonly stages: readonly WireAckStage[]; readonly frontier: WireFrontierSummary } }
  | { readonly Preview: { readonly actor: string; readonly key: string; readonly seq: number; readonly payload: readonly number[] } }
  | { readonly Presence: { readonly peers: readonly (readonly number[])[] } }
  | { readonly CreditGrant: { readonly n: number } }
  | { readonly Error: { readonly code: string; readonly message: string } };

/** 🎞️ Writes an unsigned LEB128 varint (minimal length) — a byte-for-byte TS twin of
 * `protocol_core`'s `write_varint_u64` (`protocol/core/rs/lib.rs` `🔖️WireCodec`). */
function writeVarintU64(out: number[], value: number): void {
  let remaining = value;
  for (;;) {
    const byte = remaining & 0x7f;
    remaining = Math.floor(remaining / 128);
    if (remaining === 0) {
      out.push(byte);
      return;
    }
    out.push(byte | 0x80);
  }
}

/** 🎞️ Reads an unsigned LEB128 varint starting at `pos[0]`, advancing it past it — the TS twin of
 * `protocol_core`'s `read_varint_u64`. */
function readVarintU64(bytes: Uint8Array, pos: [number]): number {
  let result = 0;
  let shift = 1;
  for (let i = 0; i < 10; i++) {
    const byte = bytes[pos[0]];
    if (byte === undefined) throw new Error("wire frame varint: truncated");
    pos[0] += 1;
    result += (byte & 0x7f) * shift;
    if ((byte & 0x80) === 0) return result;
    shift *= 128;
  }
  throw new Error("wire frame varint: overlong varint (exceeds 10 bytes)");
}

/** 🎞️ `varint-u64 len | utf8 bytes` — the TS twin of `protocol_core::write_str`. */
function writeStr(out: number[], value: string): void {
  const bytes = new TextEncoder().encode(value);
  writeVarintU64(out, bytes.length);
  for (const byte of bytes) out.push(byte);
}

/** 🎞️ The inverse of {@link writeStr} — the TS twin of `protocol_core::read_str`. */
function readStr(bytes: Uint8Array, pos: [number]): string {
  const len = readVarintU64(bytes, pos);
  const slice = bytes.subarray(pos[0], pos[0] + len);
  if (slice.length !== len) throw new Error("wire str: truncated");
  pos[0] += len;
  return new TextDecoder().decode(slice);
}

/** 🎞️ `varint-u64 len | raw bytes` — the TS twin of `protocol_core::write_bytes`. */
function writeBytes(out: number[], value: readonly number[]): void {
  writeVarintU64(out, value.length);
  for (const byte of value) out.push(byte);
}

/** 🎞️ The inverse of {@link writeBytes} — the TS twin of `protocol_core::read_bytes`. */
function readBytes(bytes: Uint8Array, pos: [number]): number[] {
  const len = readVarintU64(bytes, pos);
  const slice = bytes.subarray(pos[0], pos[0] + len);
  if (slice.length !== len) throw new Error("wire bytes: truncated");
  pos[0] += len;
  return Array.from(slice);
}

/** 🎞️ 32 raw bytes, no length prefix — the TS twin of `protocol_core::write_hash32`. */
function writeHash32(out: number[], value: readonly number[]): void {
  if (value.length !== 32) throw new Error("wire hash32: expected 32 bytes");
  for (const byte of value) out.push(byte);
}

/** 🎞️ The inverse of {@link writeHash32} — the TS twin of `protocol_core::read_hash32`. */
function readHash32(bytes: Uint8Array, pos: [number]): number[] {
  const slice = bytes.subarray(pos[0], pos[0] + 32);
  if (slice.length !== 32) throw new Error("wire hash32: truncated");
  pos[0] += 32;
  return Array.from(slice);
}

/** 🎞️ One byte, `0`/`1` — the TS twin of `protocol_core::write_bool`. */
function writeBool(out: number[], value: boolean): void {
  out.push(value ? 1 : 0);
}

/** 🎞️ The inverse of {@link writeBool} — the TS twin of `protocol_core::read_bool`. */
function readBool(bytes: Uint8Array, pos: [number]): boolean {
  const byte = bytes[pos[0]];
  if (byte === undefined) throw new Error("wire bool: truncated");
  pos[0] += 1;
  return byte !== 0;
}

/** 🎞️ 8 raw little-endian bytes — the TS twin of `protocol_core::write_f64`. */
function writeF64(out: number[], value: number): void {
  const buffer = new ArrayBuffer(8);
  new DataView(buffer).setFloat64(0, value, true);
  for (const byte of new Uint8Array(buffer)) out.push(byte);
}

/** 🎞️ The inverse of {@link writeF64} — the TS twin of `protocol_core::read_f64`. */
function readF64(bytes: Uint8Array, pos: [number]): number {
  const slice = bytes.subarray(pos[0], pos[0] + 8);
  if (slice.length !== 8) throw new Error("wire f64: truncated");
  pos[0] += 8;
  return new DataView(slice.buffer, slice.byteOffset, 8).getFloat64(0, true);
}

/** 🎞️ `varint-u64 len | raw bytes` per entry — the TS twin of `protocol_wire::write_vec_bytes`. */
function writeVecBytes(out: number[], values: readonly (readonly number[])[]): void {
  writeVarintU64(out, values.length);
  for (const value of values) writeBytes(out, value);
}

/** 🎞️ The inverse of {@link writeVecBytes} — the TS twin of `protocol_wire::read_vec_bytes`. */
function readVecBytes(bytes: Uint8Array, pos: [number]): number[][] {
  const count = readVarintU64(bytes, pos);
  const result: number[][] = [];
  for (let i = 0; i < count; i++) result.push(readBytes(bytes, pos));
  return result;
}

/** 🎯️ `actor str | presence bitmask u8 | connected_at_ms varint | fields present per bitmask
 * (label str? | selection_json str? | user_id str? | role str? | cursor f64,f64? | viewport
 * f64,f64,f64? | drag_ghost_json str?)` — the TS twin of `semio_framework_core::encode_presence_peer`
 * (`framework/core/rs/lib.rs`). This is what `ClientFrame::Presence.peer`/`ServerFrame::Presence.
 * peers[]` actually carry — real binary, not JSON bytes. */
export function encodePresencePeer(peer: DocumentPresencePeer): number[] {
  const out: number[] = [];
  writeStr(out, peer.actor);
  let presence = 0;
  if (peer.label !== undefined) presence |= 1 << 0;
  if (peer.selectionJson !== undefined) presence |= 1 << 1;
  if (peer.userId !== undefined) presence |= 1 << 2;
  if (peer.role !== undefined) presence |= 1 << 3;
  if (peer.cursor !== undefined) presence |= 1 << 4;
  if (peer.viewport !== undefined) presence |= 1 << 5;
  if (peer.dragGhostJson !== undefined) presence |= 1 << 6;
  out.push(presence);
  writeVarintU64(out, peer.connectedAtMs);
  if (peer.label !== undefined) writeStr(out, peer.label);
  if (peer.selectionJson !== undefined) writeStr(out, peer.selectionJson);
  if (peer.userId !== undefined) writeStr(out, peer.userId);
  if (peer.role !== undefined) writeStr(out, peer.role);
  if (peer.cursor !== undefined) {
    writeF64(out, peer.cursor.x);
    writeF64(out, peer.cursor.y);
  }
  if (peer.viewport !== undefined) {
    writeF64(out, peer.viewport.x);
    writeF64(out, peer.viewport.y);
    writeF64(out, peer.viewport.zoom);
  }
  if (peer.dragGhostJson !== undefined) writeStr(out, peer.dragGhostJson);
  return out;
}

/** 🎯️ The inverse of {@link encodePresencePeer} — the TS twin of
 * `semio_framework_core::decode_presence_peer`. */
export function decodePresencePeer(bytes: Uint8Array, pos: [number]): DocumentPresencePeer {
  const actor = readStr(bytes, pos);
  const presence = bytes[pos[0]];
  if (presence === undefined) throw new Error("presence peer: truncated");
  pos[0] += 1;
  const connectedAtMs = readVarintU64(bytes, pos);
  const label = presence & (1 << 0) ? readStr(bytes, pos) : undefined;
  const selectionJson = presence & (1 << 1) ? readStr(bytes, pos) : undefined;
  const userId = presence & (1 << 2) ? readStr(bytes, pos) : undefined;
  const role = presence & (1 << 3) ? readStr(bytes, pos) : undefined;
  const cursor = presence & (1 << 4) ? { x: readF64(bytes, pos), y: readF64(bytes, pos) } : undefined;
  const viewport = presence & (1 << 5) ? { x: readF64(bytes, pos), y: readF64(bytes, pos), zoom: readF64(bytes, pos) } : undefined;
  const dragGhostJson = presence & (1 << 6) ? readStr(bytes, pos) : undefined;
  return { actor, label, selectionJson, connectedAtMs, userId, role, cursor, viewport, dragGhostJson };
}

//#region 🔖️Combinators
function writeOptStr(out: number[], value: string | null): void {
  writeBool(out, value !== null);
  if (value !== null) writeStr(out, value);
}
function readOptStr(bytes: Uint8Array, pos: [number]): string | null {
  return readBool(bytes, pos) ? readStr(bytes, pos) : null;
}
function writeOptBytes(out: number[], value: readonly number[] | null): void {
  writeBool(out, value !== null);
  if (value !== null) writeBytes(out, value);
}
function readOptBytes(bytes: Uint8Array, pos: [number]): number[] | null {
  return readBool(bytes, pos) ? readBytes(bytes, pos) : null;
}
function writeOptFrontier(out: number[], value: WireFrontierSummary | null): void {
  writeBool(out, value !== null);
  if (value !== null) encodeFrontier(out, value);
}
function readOptFrontier(bytes: Uint8Array, pos: [number]): WireFrontierSummary | null {
  return readBool(bytes, pos) ? decodeFrontier(bytes, pos) : null;
}
function writeVecStr(out: number[], values: readonly string[]): void {
  writeVarintU64(out, values.length);
  for (const value of values) writeStr(out, value);
}
function readVecStr(bytes: Uint8Array, pos: [number]): string[] {
  const count = readVarintU64(bytes, pos);
  const result: string[] = [];
  for (let i = 0; i < count; i++) result.push(readStr(bytes, pos));
  return result;
}
function writeVecEnvelope(out: number[], values: readonly WireOperationEnvelope[]): void {
  writeVarintU64(out, values.length);
  for (const value of values) encodeEnvelope(out, value);
}
function readVecEnvelope(bytes: Uint8Array, pos: [number]): WireOperationEnvelope[] {
  const count = readVarintU64(bytes, pos);
  const result: WireOperationEnvelope[] = [];
  for (let i = 0; i < count; i++) result.push(decodeEnvelope(bytes, pos));
  return result;
}
//#endregion 🔖️Combinators

//#region 🔖️EnvelopeCodec
/** 🎞️ `actor varint | physical_ms varint | logical varint` — the TS twin of `protocol_causal`'s
 * private `encode_hlc`. */
function encodeHlc(out: number[], hlc: { readonly actor: number; readonly physical_ms: number; readonly logical: number }): void {
  writeVarintU64(out, hlc.actor);
  writeVarintU64(out, hlc.physical_ms);
  writeVarintU64(out, hlc.logical);
}
function decodeHlc(bytes: Uint8Array, pos: [number]): { readonly actor: number; readonly physical_ms: number; readonly logical: number } {
  const actor = readVarintU64(bytes, pos);
  const physical_ms = readVarintU64(bytes, pos);
  const logical = readVarintU64(bytes, pos);
  return { actor, physical_ms, logical };
}

/** 🎯️ `operation_id str | document_id str | actor str | dependencies vec<str> | diff.schema str |
 * diff.payload bytes | inverse.schema str | inverse.payload bytes | hlc` — the TS twin of Rust
 * `protocol_causal::encode_envelope`. */
function encodeEnvelope(out: number[], envelope: WireOperationEnvelope): void {
  writeStr(out, envelope.operation_id);
  writeStr(out, envelope.document_id);
  writeStr(out, envelope.actor);
  writeVecStr(out, envelope.dependencies);
  writeStr(out, envelope.diff.schema);
  writeBytes(out, envelope.diff.payload);
  writeStr(out, envelope.inverse.schema);
  writeBytes(out, envelope.inverse.payload);
  encodeHlc(out, envelope.timestamp);
}

/** 🎯️ Inverse of {@link encodeEnvelope} — the TS twin of Rust `protocol_causal::decode_envelope`. */
function decodeEnvelope(bytes: Uint8Array, pos: [number]): WireOperationEnvelope {
  const operation_id = readStr(bytes, pos);
  const document_id = readStr(bytes, pos);
  const actor = readStr(bytes, pos);
  const dependencies = readVecStr(bytes, pos);
  const diffSchema = readStr(bytes, pos);
  const diffPayload = readBytes(bytes, pos);
  const inverseSchema = readStr(bytes, pos);
  const inversePayload = readBytes(bytes, pos);
  const timestamp = decodeHlc(bytes, pos);
  return { operation_id, document_id, actor, dependencies, diff: { schema: diffSchema, payload: diffPayload }, inverse: { schema: inverseSchema, payload: inversePayload }, timestamp };
}

/** 🎯️ `document_id str | head_edit_ordinal varint | head_edit_id str | last_commit_seq varint |
 * chain_hash 32` — the TS twin of Rust `protocol_causal::encode_frontier`. */
function encodeFrontier(out: number[], frontier: WireFrontierSummary): void {
  writeStr(out, frontier.document_id);
  writeVarintU64(out, frontier.head_edit_ordinal);
  writeStr(out, frontier.head_edit_id);
  writeVarintU64(out, frontier.last_commit_seq);
  writeHash32(out, frontier.chain_hash);
}

/** 🎯️ Inverse of {@link encodeFrontier} — the TS twin of Rust `protocol_causal::decode_frontier`. */
function decodeFrontier(bytes: Uint8Array, pos: [number]): WireFrontierSummary {
  const document_id = readStr(bytes, pos);
  const head_edit_ordinal = readVarintU64(bytes, pos);
  const head_edit_id = readStr(bytes, pos);
  const last_commit_seq = readVarintU64(bytes, pos);
  const chain_hash = readHash32(bytes, pos);
  return { document_id, head_edit_ordinal, head_edit_id, last_commit_seq, chain_hash };
}
//#endregion 🔖️EnvelopeCodec

//#region 🔖️NestedEnums
function encodeBootstrap(out: number[], bootstrap: WireBootstrap): void {
  if (bootstrap === "None") {
    out.push(0);
    return;
  }
  if (bootstrap === "Tail") {
    out.push(2);
    return;
  }
  out.push(1);
  writeHash32(out, bootstrap.Snapshot.pack_hash);
  writeOptBytes(out, bootstrap.Snapshot.inline);
}
function decodeBootstrap(bytes: Uint8Array, pos: [number]): WireBootstrap {
  const tag = bytes[pos[0]];
  if (tag === undefined) throw new Error("wire bootstrap tag: truncated");
  pos[0] += 1;
  if (tag === 0) return "None";
  if (tag === 2) return "Tail";
  if (tag === 1) return { Snapshot: { pack_hash: readHash32(bytes, pos), inline: readOptBytes(bytes, pos) } };
  throw new Error(`wire bootstrap tag: unknown tag ${tag}`);
}

function encodeApplyOutcome(out: number[], outcome: WireApplyOutcome): void {
  if (outcome === "Accepted") {
    out.push(0);
    return;
  }
  if ("Transformed" in outcome) {
    out.push(1);
    encodeEnvelope(out, outcome.Transformed.envelope);
    return;
  }
  out.push(2);
  writeStr(out, outcome.Rejected.reason);
}
function decodeApplyOutcome(bytes: Uint8Array, pos: [number]): WireApplyOutcome {
  const tag = bytes[pos[0]];
  if (tag === undefined) throw new Error("wire apply-outcome tag: truncated");
  pos[0] += 1;
  if (tag === 0) return "Accepted";
  if (tag === 1) return { Transformed: { envelope: decodeEnvelope(bytes, pos) } };
  if (tag === 2) return { Rejected: { reason: readStr(bytes, pos) } };
  throw new Error(`wire apply-outcome tag: unknown tag ${tag}`);
}

function encodeAckStage(out: number[], stage: WireAckStage): void {
  if (stage === "Received") {
    out.push(0);
    return;
  }
  if (stage === "Persisted") {
    out.push(1);
    return;
  }
  out.push(2);
  encodeApplyOutcome(out, stage.Applied.outcome);
}
function decodeAckStage(bytes: Uint8Array, pos: [number]): WireAckStage {
  const tag = bytes[pos[0]];
  if (tag === undefined) throw new Error("wire ack-stage tag: truncated");
  pos[0] += 1;
  if (tag === 0) return "Received";
  if (tag === 1) return "Persisted";
  if (tag === 2) return { Applied: { outcome: decodeApplyOutcome(bytes, pos) } };
  throw new Error(`wire ack-stage tag: unknown tag ${tag}`);
}
function writeVecAckStage(out: number[], values: readonly WireAckStage[]): void {
  writeVarintU64(out, values.length);
  for (const value of values) encodeAckStage(out, value);
}
function readVecAckStage(bytes: Uint8Array, pos: [number]): WireAckStage[] {
  const count = readVarintU64(bytes, pos);
  const result: WireAckStage[] = [];
  for (let i = 0; i < count; i++) result.push(decodeAckStage(bytes, pos));
  return result;
}
//#endregion 🔖️NestedEnums

const WIRE_LANE_BYTES: Record<WireLane, number> = { command: 0, preview: 1 };
const WIRE_BYTE_LANES: readonly WireLane[] = ["command", "preview"];

/** 📤️ Encodes one `ClientFrame` on the given lane: `lane u8 | tag u8 | fields` — the TS twin of
 * `protocol_wire::encode_client_frame` (see that module's doc comment: W5 flipped the whole wire
 * codec from a JSON body to this hand-rolled binary layout, byte-for-byte with the Rust side). */
export function encodeClientFrame(frame: ClientFrame, lane: WireLane): Uint8Array {
  const out: number[] = [WIRE_LANE_BYTES[lane]];
  if (frame === "Bye") {
    out.push(6);
    return new Uint8Array(out);
  }
  if ("Hello" in frame) {
    out.push(0);
    const hello = frame.Hello;
    writeVarintU64(out, hello.wire_version);
    writeVarintU64(out, hello.protocol_version);
    writeStr(out, hello.schema);
    writeHash32(out, hello.pack_schema_hash);
    writeStr(out, hello.actor);
    writeOptStr(out, hello.token);
    writeOptStr(out, hello.resume_token);
    writeOptFrontier(out, hello.frontier);
  } else if ("Commands" in frame) {
    out.push(1);
    writeVarintU64(out, frame.Commands.batch_id);
    writeVecEnvelope(out, frame.Commands.envelopes);
  } else if ("FrontierAdvertise" in frame) {
    out.push(2);
    encodeFrontier(out, frame.FrontierAdvertise.frontier);
  } else if ("PreviewPublish" in frame) {
    out.push(3);
    writeStr(out, frame.PreviewPublish.key);
    writeVarintU64(out, frame.PreviewPublish.seq);
    writeBytes(out, frame.PreviewPublish.payload);
  } else if ("Presence" in frame) {
    out.push(4);
    writeBytes(out, frame.Presence.peer);
  } else if ("CreditGrant" in frame) {
    out.push(5);
    writeVarintU64(out, frame.CreditGrant.n);
  } else {
    throw new Error("encodeClientFrame: unrecognized frame variant");
  }
  return new Uint8Array(out);
}

/** 📥️ Decodes one `ClientFrame` — the TS twin of `protocol_wire::decode_client_frame`. */
export function decodeClientFrame(bytes: Uint8Array): { readonly lane: WireLane; readonly frame: ClientFrame } {
  if (bytes.length === 0) throw new Error("wire frame: empty frame");
  const lane = WIRE_BYTE_LANES[bytes[0]];
  if (lane === undefined) throw new Error(`wire frame lane byte: unknown lane ${bytes[0]}`);
  const pos: [number] = [1];
  const tag = bytes[pos[0]];
  if (tag === undefined) throw new Error("wire client-frame tag: truncated");
  pos[0] += 1;
  let frame: ClientFrame;
  switch (tag) {
    case 0: {
      const wire_version = readVarintU64(bytes, pos);
      const protocol_version = readVarintU64(bytes, pos);
      const schema = readStr(bytes, pos);
      const pack_schema_hash = readHash32(bytes, pos);
      const actor = readStr(bytes, pos);
      const token = readOptStr(bytes, pos);
      const resume_token = readOptStr(bytes, pos);
      const frontier = readOptFrontier(bytes, pos);
      frame = { Hello: { wire_version, protocol_version, schema, pack_schema_hash, actor, token, resume_token, frontier } };
      break;
    }
    case 1:
      frame = { Commands: { batch_id: readVarintU64(bytes, pos), envelopes: readVecEnvelope(bytes, pos) } };
      break;
    case 2:
      frame = { FrontierAdvertise: { frontier: decodeFrontier(bytes, pos) } };
      break;
    case 3:
      frame = { PreviewPublish: { key: readStr(bytes, pos), seq: readVarintU64(bytes, pos), payload: readBytes(bytes, pos) } };
      break;
    case 4:
      frame = { Presence: { peer: readBytes(bytes, pos) } };
      break;
    case 5:
      frame = { CreditGrant: { n: readVarintU64(bytes, pos) } };
      break;
    case 6:
      frame = "Bye";
      break;
    default:
      throw new Error(`wire client-frame tag: unknown tag ${tag}`);
  }
  return { lane, frame };
}

/** 📤️ Encodes one `ServerFrame` on the given lane: `lane u8 | tag u8 | fields` — the TS twin of
 * `protocol_wire::encode_server_frame`. */
export function encodeServerFrame(frame: ServerFrame, lane: WireLane): Uint8Array {
  const out: number[] = [WIRE_LANE_BYTES[lane]];
  if ("Welcome" in frame) {
    out.push(0);
    writeStr(out, frame.Welcome.session_id);
    writeStr(out, frame.Welcome.resume_token);
    encodeFrontier(out, frame.Welcome.server_frontier);
    encodeBootstrap(out, frame.Welcome.bootstrap);
  } else if ("SnapshotChunk" in frame) {
    out.push(1);
    writeVarintU64(out, frame.SnapshotChunk.seq);
    writeBytes(out, frame.SnapshotChunk.bytes);
  } else if ("SnapshotDone" in frame) {
    out.push(2);
    writeVarintU64(out, frame.SnapshotDone.seq_count);
  } else if ("Commands" in frame) {
    out.push(3);
    writeVecEnvelope(out, frame.Commands.envelopes);
    writeStr(out, frame.Commands.origin);
    encodeFrontier(out, frame.Commands.frontier);
  } else if ("Ack" in frame) {
    out.push(4);
    writeVarintU64(out, frame.Ack.batch_id);
    writeVecAckStage(out, frame.Ack.stages);
    encodeFrontier(out, frame.Ack.frontier);
  } else if ("Preview" in frame) {
    out.push(5);
    writeStr(out, frame.Preview.actor);
    writeStr(out, frame.Preview.key);
    writeVarintU64(out, frame.Preview.seq);
    writeBytes(out, frame.Preview.payload);
  } else if ("Presence" in frame) {
    out.push(6);
    writeVecBytes(out, frame.Presence.peers);
  } else if ("CreditGrant" in frame) {
    out.push(7);
    writeVarintU64(out, frame.CreditGrant.n);
  } else if ("Error" in frame) {
    out.push(8);
    writeStr(out, frame.Error.code);
    writeStr(out, frame.Error.message);
  } else {
    throw new Error("encodeServerFrame: unrecognized frame variant");
  }
  return new Uint8Array(out);
}

/** 📥️ Decodes one `ServerFrame` — the TS twin of `protocol_wire::decode_server_frame`. */
export function decodeServerFrame(bytes: Uint8Array): { readonly lane: WireLane; readonly frame: ServerFrame } {
  if (bytes.length === 0) throw new Error("wire frame: empty frame");
  const lane = WIRE_BYTE_LANES[bytes[0]];
  if (lane === undefined) throw new Error(`wire frame lane byte: unknown lane ${bytes[0]}`);
  const pos: [number] = [1];
  const tag = bytes[pos[0]];
  if (tag === undefined) throw new Error("wire server-frame tag: truncated");
  pos[0] += 1;
  let frame: ServerFrame;
  switch (tag) {
    case 0:
      frame = { Welcome: { session_id: readStr(bytes, pos), resume_token: readStr(bytes, pos), server_frontier: decodeFrontier(bytes, pos), bootstrap: decodeBootstrap(bytes, pos) } };
      break;
    case 1:
      frame = { SnapshotChunk: { seq: readVarintU64(bytes, pos), bytes: readBytes(bytes, pos) } };
      break;
    case 2:
      frame = { SnapshotDone: { seq_count: readVarintU64(bytes, pos) } };
      break;
    case 3:
      frame = { Commands: { envelopes: readVecEnvelope(bytes, pos), origin: readStr(bytes, pos), frontier: decodeFrontier(bytes, pos) } };
      break;
    case 4:
      frame = { Ack: { batch_id: readVarintU64(bytes, pos), stages: readVecAckStage(bytes, pos), frontier: decodeFrontier(bytes, pos) } };
      break;
    case 5:
      frame = { Preview: { actor: readStr(bytes, pos), key: readStr(bytes, pos), seq: readVarintU64(bytes, pos), payload: readBytes(bytes, pos) } };
      break;
    case 6:
      frame = { Presence: { peers: readVecBytes(bytes, pos) } };
      break;
    case 7:
      frame = { CreditGrant: { n: readVarintU64(bytes, pos) } };
      break;
    case 8:
      frame = { Error: { code: readStr(bytes, pos), message: readStr(bytes, pos) } };
      break;
    default:
      throw new Error(`wire server-frame tag: unknown tag ${tag}`);
  }
  return { lane, frame };
}

/** 🗃️ A durable place a document synchronizes with — mirrors Rust `PersistenceBinding`. */
export type PersistenceBinding = { readonly kind: "folder"; readonly path: string } | { readonly kind: "hub"; readonly baseUrl: string; readonly spaceId: string; readonly token?: string };

/** 🧾️ Everything the worker needs to open one document's actor — mirrors `DocumentActorConfig`. */
export type DocumentActorConfig = {
  readonly documentId: string;
  readonly schema: string;
  readonly bindings: readonly PersistenceBinding[];
  readonly watchExternal?: boolean;
  readonly actor: string;
  /** 🧬️ W5.7: this document kind's `store::DocumentCodec.pack_schema_hash`, for hub schema-hash
   * validation (`ClientFrame::Hello.pack_schema_hash`) — the shell fills this from the wasm
   * renderer's `document_pack_schema_hash(schema)` export before calling `openDocument`. Omitted
   * (or all-zero) means "schema-agnostic client", which the hub never validates. */
  readonly packSchemaHash?: readonly number[];
};

/** 📨️ Caller→actor control messages — mirrors Rust `DocumentActorMsg`. */
export type DocumentActorMsg =
  | { readonly kind: "localOperations"; readonly envelopes: readonly OperationEnvelope[] }
  | { readonly kind: "localSnapshot"; readonly pack: readonly number[]; readonly spr: readonly number[] }
  | { readonly kind: "presenceHeartbeat"; readonly peer: DocumentPresencePeer }
  | { readonly kind: "publishPreview"; readonly key: string; readonly seq: number; readonly payload: readonly number[] }
  | { readonly kind: "externalChanged" }
  | { readonly kind: "detach" };

/** 📶️ Connection state of a document's remote (hub) transport — mirrors Rust `RemoteState`. */
export type RemoteState = { readonly kind: "detached" } | { readonly kind: "connecting" } | { readonly kind: "live"; readonly peerCount: number } | { readonly kind: "backoff"; readonly retryInMs: number };

/** 🚦️ Sync health snapshot for status badges — mirrors Rust `DocumentSyncStatus`. */
export type DocumentSyncStatus = {
  readonly persisted: boolean;
  readonly pendingOperations: number;
  readonly remote: RemoteState;
};

/** ⚠️ A structural sync conflict — loosely typed pending a full mirror of `vcs::SpaceConflict`; the
 * shell only needs enough to render a conflict card / offer "fork alternative" vs "take theirs". */
export type SyncConflict = { readonly message?: string } & Record<string, unknown>;

/** 📮️ The client-side twin of `protocol_wire::ApplyOutcome`, minus the `Transformed` envelope
 * payload (already delivered separately as a `remoteOperations` event by the time this fires) —
 * mirrors Rust `CommandAckOutcome`. */
export type CommandAckOutcome = { readonly kind: "accepted" } | { readonly kind: "transformed" } | { readonly kind: "rejected"; readonly reason: string };

/** 📬️ Actor→subscriber events — mirrors Rust `DocumentEvent`. */
export type DocumentEvent =
  | { readonly kind: "remoteOperations"; readonly envelopes: readonly OperationEnvelope[] }
  | { readonly kind: "snapshotReplaced"; readonly pack: readonly number[]; readonly spr: readonly number[] }
  | ({ readonly kind: "status" } & DocumentSyncStatus)
  | { readonly kind: "presence"; readonly peers: readonly DocumentPresencePeer[] }
  | { readonly kind: "preview"; readonly actor: string; readonly key: string; readonly seq: number; readonly payload: readonly number[] }
  | { readonly kind: "commandOutcome"; readonly batchId: number; readonly outcome: CommandAckOutcome }
  | ({ readonly kind: "conflict" } & SyncConflict);

/** 📤️ Main thread → `🟦️backbone-worker.ts` — `bytes` is a UTF-8 worker wire payload (see {@link encodeBackboneWorkerRequest}). */
export type BackboneWorkerWireMessage = { readonly wire: Uint8Array };

/** @emoji 🧵️ Encodes a {@link BackboneWorkerRequest} for the wasm `store_worker` (`handleRequestBytes`). */
export function encodeBackboneWorkerRequest(request: BackboneWorkerRequest): Uint8Array {
  return new TextEncoder().encode(JSON.stringify(request));
}

/** @emoji 🧵️ Encodes a {@link BackboneWorkerResponse} from the wasm actor / TS fallback. */
export function encodeBackboneWorkerResponse(response: BackboneWorkerResponse): Uint8Array {
  return new TextEncoder().encode(JSON.stringify(response));
}

/** @emoji 🧵️ Decodes a worker response/event wire payload from the wasm actor. */
export function decodeBackboneWorkerResponse(wire: Uint8Array): BackboneWorkerResponse {
  return JSON.parse(new TextDecoder().decode(wire)) as BackboneWorkerResponse;
}

/** 📤️ Main thread → `🟦️backbone-worker.ts` messages (structured clone or {@link BackboneWorkerWireMessage}). */
export type BackboneWorkerRequest = ({ readonly kind: "open" } & DocumentActorConfig) | { readonly kind: "close"; readonly documentId: string } | { readonly kind: "send"; readonly documentId: string; readonly message: DocumentActorMsg };

/** 📥️ `🟦️backbone-worker.ts` → main thread messages. */
export type BackboneWorkerResponse = { readonly kind: "event"; readonly documentId: string; readonly event: DocumentEvent } | { readonly kind: "ready" };
//#endregion 🔖️SyncProtocol

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
//#endregion 🔖️PublicApi
//#endregion 🔖️PackValueCodec

//#region 🔖️AppChannelCodec
/**
 * 📡️ TS mirror of the `protocol_channel` crate's `AppCommand`/`AppFrame` binary frame protocol
 * (`tag u8 | fields`, built on `protocol_core`'s varint/string/bytes primitives — the same ones
 * {@link encodeClientFrame}/{@link decodeClientFrame} above use). `protocol_channel` is being
 * built in parallel (WP-0A of `HEADLESS-APP-ENGINE-BINARY-COMMAND-PROTOCOL-FOUNDATIONS`) and may
 * not exist on disk yet, so this mirrors the AGREED WIRE CONTRACT, not that crate's source —
 * `envelopes`/`config`/`command`/`descriptor`/etc. all stay OPAQUE `bytes` here (never a decoded
 * `protocol_causal::OperationEnvelope` or app-specific payload shape), exactly like
 * {@link WireOperationEnvelope}'s `diff`/`inverse` payloads above. `Option<T>` fields use the same
 * `0x00`/`0x01` presence-byte convention as {@link writeOptStr}/{@link writeOptBytes} elsewhere in
 * this file. ⚠️ Round-trip-tested against itself only (no `protocol_channel` Rust crate exists yet
 * to source hex fixtures from) — cross-language hex-fixture reconciliation is follow-up work once
 * WP-0A lands (see this file's `🧪️Tests` region for the tracking note).
 */

//#region 🔖️Types
/** 🔍️ One UI section's cache-probe entry inside `AppCommand.RefreshUi` — `kind u8 | key str |
 * hash: (u64 | null)`. */
export type SectionProbe = { readonly kind: number; readonly key: string; readonly hash: number | null };

export type AppCommandValue =
  | { readonly Hello: { readonly channel_version: number; readonly app_id: string; readonly actor: string; readonly config: readonly number[] } }
  | { readonly ConfigCommand: { readonly seq: number; readonly command: readonly number[] } }
  | { readonly Command: { readonly seq: number; readonly command: readonly number[]; readonly view_state: readonly number[] } }
  | { readonly CommandText: { readonly seq: number; readonly line: string } }
  | { readonly RefreshUi: { readonly seq: number; readonly sections: readonly SectionProbe[]; readonly view_state: readonly number[] } }
  | { readonly ContextMenu: { readonly seq: number; readonly request: readonly number[] } }
  | { readonly DocumentCommand: { readonly seq: number; readonly command: readonly number[] } }
  | { readonly ApplyEnvelopes: { readonly seq: number; readonly envelopes: readonly (readonly number[])[] } }
  | { readonly LoadDocument: { readonly seq: number; readonly pack: readonly number[]; readonly spr: readonly number[] } }
  | { readonly ReadDocument: { readonly seq: number } }
  | { readonly LoadConfig: { readonly seq: number; readonly pack: readonly number[]; readonly spr: readonly number[] } }
  | { readonly ReadConfig: { readonly seq: number } }
  | { readonly AttachBackbone: { readonly seq: number; readonly uri: string } }
  | { readonly DetachBackbone: { readonly seq: number } }
  | { readonly MediaIn: { readonly seq: number; readonly port: string; readonly descriptor: readonly number[]; readonly data: readonly number[] } }
  | { readonly MediaOut: { readonly seq: number; readonly port: string; readonly request: readonly number[] } }
  | { readonly MediaFingerprint: { readonly seq: number; readonly port: string } }
  | "Bye";

export type AppFrameValue =
  | { readonly Welcome: { readonly channel_version: number; readonly instance: number; readonly manifest: readonly number[] } }
  | { readonly Done: { readonly in_reply_to: number } }
  | { readonly Invocation: { readonly in_reply_to: number; readonly output: readonly number[]; readonly diagnostics: readonly number[] } }
  | { readonly UiSection: { readonly in_reply_to: number | null; readonly kind: number; readonly key: string; readonly hash: number; readonly body: readonly number[] | null } }
  | { readonly Effects: { readonly in_reply_to: number | null; readonly effects: readonly (readonly number[])[] } }
  | { readonly Events: { readonly in_reply_to: number | null; readonly events: readonly (readonly number[])[] } }
  | { readonly DocumentChanged: { readonly envelopes: readonly (readonly number[])[]; readonly origin: string } }
  | { readonly Document: { readonly in_reply_to: number; readonly pack: readonly number[]; readonly spr: readonly number[]; readonly ops: string } }
  | { readonly Config: { readonly in_reply_to: number; readonly pack: readonly number[]; readonly spr: readonly number[]; readonly ops: string } }
  | { readonly ConfigChanged: { readonly envelopes: readonly (readonly number[])[]; readonly origin: string } }
  | { readonly ContextMenu: { readonly in_reply_to: number; readonly items: readonly number[] } }
  | { readonly Media: { readonly in_reply_to: number; readonly port: string; readonly descriptor: readonly number[]; readonly data: readonly number[] } }
  | { readonly MediaFingerprint: { readonly in_reply_to: number; readonly port: string; readonly fingerprint: readonly number[] } }
  | { readonly Error: { readonly in_reply_to: number | null; readonly code: string; readonly message: string } };
//#endregion 🔖️Types

//#region 🔖️Combinators
/** 🎞️ `presence u8 | varint` — an `Option<u64>` (e.g. `SectionProbe.hash`,
 * `AppFrame.*.in_reply_to`), the same convention {@link writeOptStr}/{@link writeOptBytes} use. */
function writeOptU64(out: number[], value: number | null): void {
  writeBool(out, value !== null);
  if (value !== null) writeVarintU64(out, value);
}
function readOptU64(bytes: Uint8Array, pos: [number]): number | null {
  return readBool(bytes, pos) ? readVarintU64(bytes, pos) : null;
}
function writeSectionProbe(out: number[], probe: SectionProbe): void {
  out.push(probe.kind);
  writeStr(out, probe.key);
  writeOptU64(out, probe.hash);
}
function readSectionProbe(bytes: Uint8Array, pos: [number]): SectionProbe {
  const kind = bytes[pos[0]]!;
  pos[0] += 1;
  const key = readStr(bytes, pos);
  const hash = readOptU64(bytes, pos);
  return { kind, key, hash };
}
function writeVecSectionProbe(out: number[], values: readonly SectionProbe[]): void {
  writeVarintU64(out, values.length);
  for (const value of values) writeSectionProbe(out, value);
}
function readVecSectionProbe(bytes: Uint8Array, pos: [number]): SectionProbe[] {
  const count = readVarintU64(bytes, pos);
  const result: SectionProbe[] = [];
  for (let i = 0; i < count; i++) result.push(readSectionProbe(bytes, pos));
  return result;
}
//#endregion 🔖️Combinators

//#region 🔖️Codec
const APP_COMMAND_TAGS = {
  Hello: 0, ConfigCommand: 1, Command: 2, CommandText: 3, RefreshUi: 4, ContextMenu: 5, DocumentCommand: 6, ApplyEnvelopes: 7,
  LoadDocument: 8, ReadDocument: 9, LoadConfig: 10, ReadConfig: 11, AttachBackbone: 12, DetachBackbone: 13, MediaIn: 14, MediaOut: 15,
  MediaFingerprint: 16, Bye: 17,
} as const;
const APP_FRAME_TAGS = {
  Welcome: 0, Done: 1, Invocation: 2, UiSection: 3, Effects: 4, Events: 5, DocumentChanged: 6, Document: 7,
  Config: 8, ConfigChanged: 9, ContextMenu: 10, Media: 11, MediaFingerprint: 12, Error: 13,
} as const;

/** 📤️ `tag u8 | fields` — the TS twin of `protocol_channel::encode_app_command` (agreed contract). */
export function encodeAppCommand(cmd: AppCommandValue): Uint8Array {
  const out: number[] = [];
  if (cmd === "Bye") {
    out.push(APP_COMMAND_TAGS.Bye);
    return new Uint8Array(out);
  }
  if ("Hello" in cmd) {
    out.push(APP_COMMAND_TAGS.Hello);
    writeVarintU64(out, cmd.Hello.channel_version);
    writeStr(out, cmd.Hello.app_id);
    writeStr(out, cmd.Hello.actor);
    writeBytes(out, cmd.Hello.config);
  } else if ("ConfigCommand" in cmd) {
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
  } else if ("RefreshUi" in cmd) {
    out.push(APP_COMMAND_TAGS.RefreshUi);
    writeVarintU64(out, cmd.RefreshUi.seq);
    writeVecSectionProbe(out, cmd.RefreshUi.sections);
    writeBytes(out, cmd.RefreshUi.view_state);
  } else if ("ContextMenu" in cmd) {
    out.push(APP_COMMAND_TAGS.ContextMenu);
    writeVarintU64(out, cmd.ContextMenu.seq);
    writeBytes(out, cmd.ContextMenu.request);
  } else if ("DocumentCommand" in cmd) {
    out.push(APP_COMMAND_TAGS.DocumentCommand);
    writeVarintU64(out, cmd.DocumentCommand.seq);
    writeBytes(out, cmd.DocumentCommand.command);
  } else if ("ApplyEnvelopes" in cmd) {
    out.push(APP_COMMAND_TAGS.ApplyEnvelopes);
    writeVarintU64(out, cmd.ApplyEnvelopes.seq);
    writeVecBytes(out, cmd.ApplyEnvelopes.envelopes);
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
  } else if ("AttachBackbone" in cmd) {
    out.push(APP_COMMAND_TAGS.AttachBackbone);
    writeVarintU64(out, cmd.AttachBackbone.seq);
    writeStr(out, cmd.AttachBackbone.uri);
  } else if ("DetachBackbone" in cmd) {
    out.push(APP_COMMAND_TAGS.DetachBackbone);
    writeVarintU64(out, cmd.DetachBackbone.seq);
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
    case APP_COMMAND_TAGS.Hello: {
      const channel_version = readVarintU64(bytes, pos);
      const app_id = readStr(bytes, pos);
      const actor = readStr(bytes, pos);
      const config = readBytes(bytes, pos);
      return { Hello: { channel_version, app_id, actor, config } };
    }
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
    case APP_COMMAND_TAGS.RefreshUi: {
      const seq = readVarintU64(bytes, pos);
      const sections = readVecSectionProbe(bytes, pos);
      const view_state = readBytes(bytes, pos);
      return { RefreshUi: { seq, sections, view_state } };
    }
    case APP_COMMAND_TAGS.ContextMenu:
      return { ContextMenu: { seq: readVarintU64(bytes, pos), request: readBytes(bytes, pos) } };
    case APP_COMMAND_TAGS.DocumentCommand:
      return { DocumentCommand: { seq: readVarintU64(bytes, pos), command: readBytes(bytes, pos) } };
    case APP_COMMAND_TAGS.ApplyEnvelopes:
      return { ApplyEnvelopes: { seq: readVarintU64(bytes, pos), envelopes: readVecBytes(bytes, pos) } };
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
    case APP_COMMAND_TAGS.AttachBackbone:
      return { AttachBackbone: { seq: readVarintU64(bytes, pos), uri: readStr(bytes, pos) } };
    case APP_COMMAND_TAGS.DetachBackbone:
      return { DetachBackbone: { seq: readVarintU64(bytes, pos) } };
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
    case APP_COMMAND_TAGS.Bye:
      return "Bye";
    default:
      throw new Error(`decodeAppCommand: unknown tag ${bytes[0]}`);
  }
}

/** 📤️ `tag u8 | fields` — the TS twin of `protocol_channel::encode_app_frame` (agreed contract). */
export function encodeAppFrame(frame: AppFrameValue): Uint8Array {
  const out: number[] = [];
  if ("Welcome" in frame) {
    out.push(APP_FRAME_TAGS.Welcome);
    writeVarintU64(out, frame.Welcome.channel_version);
    writeVarintU64(out, frame.Welcome.instance);
    writeBytes(out, frame.Welcome.manifest);
  } else if ("Done" in frame) {
    out.push(APP_FRAME_TAGS.Done);
    writeVarintU64(out, frame.Done.in_reply_to);
  } else if ("Invocation" in frame) {
    out.push(APP_FRAME_TAGS.Invocation);
    writeVarintU64(out, frame.Invocation.in_reply_to);
    writeBytes(out, frame.Invocation.output);
    writeBytes(out, frame.Invocation.diagnostics);
  } else if ("UiSection" in frame) {
    out.push(APP_FRAME_TAGS.UiSection);
    writeOptU64(out, frame.UiSection.in_reply_to);
    out.push(frame.UiSection.kind);
    writeStr(out, frame.UiSection.key);
    writeVarintU64(out, frame.UiSection.hash);
    writeOptBytes(out, frame.UiSection.body);
  } else if ("Effects" in frame) {
    out.push(APP_FRAME_TAGS.Effects);
    writeOptU64(out, frame.Effects.in_reply_to);
    writeVecBytes(out, frame.Effects.effects);
  } else if ("Events" in frame) {
    out.push(APP_FRAME_TAGS.Events);
    writeOptU64(out, frame.Events.in_reply_to);
    writeVecBytes(out, frame.Events.events);
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
    writeStr(out, frame.Error.code);
    writeStr(out, frame.Error.message);
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
    case APP_FRAME_TAGS.Welcome: {
      const channel_version = readVarintU64(bytes, pos);
      const instance = readVarintU64(bytes, pos);
      const manifest = readBytes(bytes, pos);
      return { Welcome: { channel_version, instance, manifest } };
    }
    case APP_FRAME_TAGS.Done:
      return { Done: { in_reply_to: readVarintU64(bytes, pos) } };
    case APP_FRAME_TAGS.Invocation: {
      const in_reply_to = readVarintU64(bytes, pos);
      const output = readBytes(bytes, pos);
      const diagnostics = readBytes(bytes, pos);
      return { Invocation: { in_reply_to, output, diagnostics } };
    }
    case APP_FRAME_TAGS.UiSection: {
      const in_reply_to = readOptU64(bytes, pos);
      const kind = bytes[pos[0]]!;
      pos[0] += 1;
      const key = readStr(bytes, pos);
      const hash = readVarintU64(bytes, pos);
      const body = readOptBytes(bytes, pos);
      return { UiSection: { in_reply_to, kind, key, hash, body } };
    }
    case APP_FRAME_TAGS.Effects:
      return { Effects: { in_reply_to: readOptU64(bytes, pos), effects: readVecBytes(bytes, pos) } };
    case APP_FRAME_TAGS.Events:
      return { Events: { in_reply_to: readOptU64(bytes, pos), events: readVecBytes(bytes, pos) } };
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
      const code = readStr(bytes, pos);
      const message = readStr(bytes, pos);
      return { Error: { in_reply_to, code, message } };
    }
    default:
      throw new Error(`decodeAppFrame: unknown tag ${bytes[0]}`);
  }
}
//#endregion 🔖️Codec
//#endregion 🔖️AppChannelCodec

//#region 🔖️AppChannelClient
/**
 * 📡️ TS twin of `protocol_channel::CHANNEL_VERSION` (`🔨️module/📡️protocol/🧵️channel/⚡️implementation/🦀️rust/📦️lib.rs`)
 * — bump both sides together on a wire-incompatible frame change.
 */
const APP_CHANNEL_VERSION = 3;

/** 📡️ The slice of {@link PluginWasmHandle} {@link AppChannelClient} needs — deliberately narrower
 * than the full handle so a caller can hand in any `exchange`-shaped object (a real handle, a test
 * double, ...) without importing the rest of `@semio-tech/framework-core`'s plugin-loading surface. */
export type AppChannelHandle = Pick<PluginWasmHandle, "exchange">;

/**
 * 📡️ Typed facade over one plugin instance's `exchange` channel — encodes an {@link AppCommandValue},
 * sends it through {@link PluginWasmHandle.exchange} as the sole batched frame, and decodes every
 * {@link AppFrameValue} the call returns. This is the ONLY place `AppCommand`/`AppFrame` framing
 * happens on the host side; callers (a React renderer's dispatch/refresh loop, a headless workflow
 * runner) work with decoded frames and plain JS values, never raw bytes or wire tags. `seq` is a
 * per-client monotonic counter — the host has no other way to correlate a `Command`/`RefreshUi`/
 * `ConfigCommand`/`LoadDocument`/`ReadDocument`/`LoadConfig`/`ReadConfig` with the `Invocation`/`UiSection`/`Effects`/`Events`/
 * `Document` frame(s) it produced (`AppFrame.*.in_reply_to`).
 */
export class AppChannelClient {
  private seq = 0;
  private readonly handle: AppChannelHandle;
  private readonly instanceId: number;
  private readonly appId: string;
  private readonly actor: string;

  constructor(handle: AppChannelHandle, instanceId: number, appId: string, actor: string = "local") {
    this.handle = handle;
    this.instanceId = instanceId;
    this.appId = appId;
    this.actor = actor;
  }

  private nextSeq(): number {
    this.seq += 1;
    return this.seq;
  }

  /** 🔀️ Sends one encoded command, decodes every frame the batched `exchange` call returns. */
  private async exchangeOne(command: AppCommandValue): Promise<AppFrameValue[]> {
    const replies = await this.handle.exchange(this.instanceId, [encodeAppCommand(command)]);
    return replies.map(decodeAppFrame);
  }

  /** 👋️ The channel handshake — must be the first call on a freshly created instance. Returns the
   * (expected single) `Welcome` reply frame. */
  async hello(config: unknown): Promise<AppFrameValue> {
    const frames = await this.exchangeOne({
      Hello: { channel_version: APP_CHANNEL_VERSION, app_id: this.appId, actor: this.actor, config: Array.from(encodePackValue(config)) },
    });
    const frame = frames[0];
    if (!frame) throw new Error(`AppChannelClient.hello(${this.appId}): no reply frame`);
    return frame;
  }

  /** 🎛️ Forwards one opaque app-specific command (already encoded by the caller's own command
   * grammar) plus the current view state; may return several frames (`Invocation` + `Effects` +
   * `Events` + any dirtied `UiSection`s) — routing them is the caller's job. */
  async command(commandBytes: Uint8Array, viewState: unknown): Promise<AppFrameValue[]> {
    return this.exchangeOne({
      Command: { seq: this.nextSeq(), command: Array.from(commandBytes), view_state: Array.from(encodePackValue(viewState)) },
    });
  }

  /** 🔄️ Cache-probed UI section refresh — one `UiSection` frame per section whose `hash` changed. */
  async refreshUi(sections: readonly SectionProbe[], viewState: unknown = {}): Promise<AppFrameValue[]> {
    return this.exchangeOne({
      RefreshUi: { seq: this.nextSeq(), sections, view_state: Array.from(encodePackValue(viewState)) },
    });
  }

  async configure(config: unknown): Promise<AppFrameValue[]> {
    return this.exchangeOne({ ConfigCommand: { seq: this.nextSeq(), command: Array.from(encodePackValue(config)) } });
  }

  async readDocument(): Promise<AppFrameValue[]> {
    return this.exchangeOne({ ReadDocument: { seq: this.nextSeq() } });
  }

  async loadDocument(pack: Uint8Array, spr: Uint8Array): Promise<AppFrameValue[]> {
    return this.exchangeOne({ LoadDocument: { seq: this.nextSeq(), pack: Array.from(pack), spr: Array.from(spr) } });
  }

  /** 🔗️ Attaches this instance to a backbone `uri` — the channel twin of the old
   * `attach-backbone` WIT verb. */
  async attachBackbone(uri: string): Promise<AppFrameValue[]> {
    return this.exchangeOne({ AttachBackbone: { seq: this.nextSeq(), uri } });
  }

  /** 🔗️ Detaches this instance's backbone, if any. */
  async detachBackbone(): Promise<AppFrameValue[]> {
    return this.exchangeOne({ DetachBackbone: { seq: this.nextSeq() } });
  }

  /** 🖱️ On-demand context menu — one `ContextMenu` reply whose `in_reply_to` matches this call's `seq`. */
  async contextMenu(request: unknown): Promise<unknown> {
    const seq = this.nextSeq();
    const frames = await this.exchangeOne({
      ContextMenu: { seq, request: Array.from(encodePackValue(request)) },
    });
    const errorFrame = frames.find((frame): frame is Extract<AppFrameValue, { readonly Error: unknown }> => "Error" in frame);
    if (errorFrame) {
      throw new Error(`AppChannelClient.contextMenu(${this.appId}): ${errorFrame.Error.code}: ${errorFrame.Error.message}`);
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

  /** 💓️ The heartbeat — a pure drain (`exchange(id, [])`) for pending effects/events/backbone-ingested
   * `DocumentChanged` frames queued since the previous call. Replaces the old poll-backbone +
   * refresh-ui tick. */
  async drain(): Promise<AppFrameValue[]> {
    const replies = await this.handle.exchange(this.instanceId, []);
    return replies.map(decodeAppFrame);
  }
}
//#endregion 🔖️AppChannelClient

//#region 🧪️Tests
if (import.meta.vitest) {
  const { describe, expect, it } = import.meta.vitest;

  describe("@semio-tech/framework-os-core backbone", () => {
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
      expect(decodeDocumentPackProjection(bundle)).toEqual({ nodes: [] });
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
      const message = encodeBackboneMessage({ kind: "operations", envelopes: [] });
      expect(() => applyBackboneMessage(encodeDocumentPackBytes(new Uint8Array(), new Uint8Array()), message)).toThrow("native store");
    });

    it("throws when applying operations before a snapshot exists", () => {
      const message = encodeBackboneMessage({ kind: "operations", envelopes: [] });
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

  describe("@semio-tech/framework-os-core workflow", () => {
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
        schema: "s.workflow",
        nodes: [mediaNode("node-1", "app-1"), mediaNode("node-2", "app-2")],
        edges: [{ id: "edge-1", sourceNodeId: "node-1", sourcePortId: "app-1:out", targetNodeId: "node-2", targetPortId: "app-2:in", contract: mediaContract() }],
      };
      const deliveries = planWorkflow(graph, new Set(["app-1"]));
      expect(deliveries).toEqual([{ edgeId: "edge-1", producerInstanceId: "app-1", producerPortId: "app-1:out", consumerInstanceId: "app-2", consumerPortId: "app-2:in" }]);
    });

    it("plans a chain in topological order when only the root is dirty", () => {
      const graph: OsWorkflow = {
        schema: "s.workflow",
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
        schema: "s.workflow",
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
        schema: "s.workflow",
        nodes: [mediaNode("node-1", "app-1"), mediaNode("node-2", "app-2")],
        edges: [{ id: "edge-1", sourceNodeId: "node-1", sourcePortId: "app-1:out", targetNodeId: "node-2", targetPortId: "app-2:in", contract: mediaContract() }],
      };
      expect(planWorkflow(graph, new Set())).toEqual([]);
    });

    it("plans nothing for a dirty node with no outgoing edges", () => {
      const graph: OsWorkflow = { schema: "s.workflow", nodes: [mediaNode("node-1", "app-1")], edges: [] };
      expect(planWorkflow(graph, new Set(["app-1"]))).toEqual([]);
    });

    // 🔬️ Shared fixtures replay (`framework/os/core/fixtures/*.dsl`+`*.spk`) — the same files
    // drive the Rust harness's `workflow_fixtures_match_expected_deliveries` and
    // `workflow_fixture_dsl_and_spk_pairs_are_canonical_and_equivalent` tests. Both fixture faces are
    // decoded here through the crate's own wasm bindings (`rs/lib.rs`'s `wasm_exports` module) — no
    // JSON anywhere on this path. Node builtins and the wasm module are imported dynamically inside
    // this vitest-only block so neither reaches the browser bundle (this whole `if
    // (import.meta.vitest)` block is stripped from production builds).
    it("matches the Rust plan_workflow across shared fixtures decoded via wasm", async () => {
      const { readdirSync, readFileSync } = await import("node:fs");
      const { fileURLToPath, pathToFileURL } = await import("node:url");
      const { dirname, join } = await import("node:path");
      const here = dirname(fileURLToPath(import.meta.url));
      const fixturesDir = join(here, "..", "..", "🧫️fixtures");
      const rsPkgDir = join(here, "..", "🦀️rust", "pkg");

      const wasmModule = (await import(/* @vite-ignore */ pathToFileURL(join(rsPkgDir, "semio_framework_os.js")).href)) as {
        default: (opts: { module_or_path: Uint8Array }) => Promise<unknown>;
        decodeWorkflowFixturePack: (bytes: Uint8Array) => WorkflowFixture;
        parseWorkflowFixtureDsl: (text: string) => WorkflowFixture;
      };
      await wasmModule.default({ module_or_path: new Uint8Array(readFileSync(join(rsPkgDir, "semio_framework_os_bg.wasm"))) });

      const dslFiles = readdirSync(fixturesDir).filter((file) => file.endsWith(".dsl"));
      expect(dslFiles.length).toBeGreaterThanOrEqual(5);
      for (const dslFile of dslFiles) {
        const dslText = readFileSync(join(fixturesDir, dslFile), "utf8");
        const spkFile = dslFile.replace(/^🗣️?/, "📦️").replace(/\.dsl$/, ".spk");
        const spkBytes = new Uint8Array(readFileSync(join(fixturesDir, spkFile)));
        const viaDsl = wasmModule.parseWorkflowFixtureDsl(dslText);
        const viaPack = wasmModule.decodeWorkflowFixturePack(spkBytes);
        expect(viaDsl).toEqual(viaPack);
        const deliveries = planWorkflow(viaDsl.graph, new Set(viaDsl.dirtyInstanceIds));
        expect(deliveries).toEqual(viaDsl.expectedDeliveries);
      }
    });
  });

  describe("@semio-tech/framework-os-core PackValueCodec", () => {
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

  describe("@semio-tech/framework-os-core AppChannelCodec", () => {
    const sampleCommands: readonly AppCommandValue[] = [
      { Hello: { channel_version: 3, app_id: "app.demo", actor: "actor-1", config: [1, 2, 3] } },
      { ConfigCommand: { seq: 1, command: [4, 5] } },
      { Command: { seq: 2, command: [1], view_state: [2, 3] } },
      { CommandText: { seq: 3, line: "move 1 2" } },
      { RefreshUi: { seq: 4, sections: [{ kind: 1, key: "panel-a", hash: 42 }, { kind: 2, key: "panel-b", hash: null }], view_state: [] } },
      { ContextMenu: { seq: 5, request: [9, 9] } },
      { DocumentCommand: { seq: 6, command: [7] } },
      { ApplyEnvelopes: { seq: 7, envelopes: [[1, 2], [3, 4, 5], []] } },
      { LoadDocument: { seq: 8, pack: [1, 2, 3], spr: [4, 5, 6] } },
      { ReadDocument: { seq: 9 } },
      { LoadConfig: { seq: 10, pack: [1], spr: [2] } },
      { ReadConfig: { seq: 11 } },
      { AttachBackbone: { seq: 12, uri: "file:///tmp/a.json" } },
      { DetachBackbone: { seq: 13 } },
      { MediaIn: { seq: 14, port: "in-1", descriptor: [1], data: [2, 3] } },
      { MediaOut: { seq: 15, port: "out-1", request: [4] } },
      { MediaFingerprint: { seq: 16, port: "fp-1" } },
      "Bye",
    ];

    const sampleFrames: readonly AppFrameValue[] = [
      { Welcome: { channel_version: 3, instance: 7, manifest: [1, 2, 3] } },
      { Done: { in_reply_to: 1 } },
      { Invocation: { in_reply_to: 2, output: [1], diagnostics: [] } },
      { UiSection: { in_reply_to: 3, kind: 1, key: "panel-a", hash: 42, body: [1, 2] } },
      { UiSection: { in_reply_to: null, kind: 1, key: "panel-b", hash: 0, body: null } },
      { Effects: { in_reply_to: 4, effects: [[1], [2, 3]] } },
      { Effects: { in_reply_to: null, effects: [] } },
      { Events: { in_reply_to: 5, events: [[9]] } },
      { DocumentChanged: { envelopes: [[1, 2]], origin: "remote" } },
      { Document: { in_reply_to: 6, pack: [1, 2], spr: [3, 4], ops: "op-log" } },
      { ContextMenu: { in_reply_to: 7, items: [1, 2, 3] } },
      { Media: { in_reply_to: 8, port: "out-1", descriptor: [1], data: [2] } },
      { MediaFingerprint: { in_reply_to: 9, port: "fp-1", fingerprint: [1, 2, 3, 4] } },
      { Error: { in_reply_to: 10, code: "E_BAD", message: "boom" } },
      { Error: { in_reply_to: null, code: "E_BAD", message: "boom" } },
    ];

    it.each(sampleCommands.map((cmd) => [cmd] as const))("round-trips AppCommand %j", (cmd) => {
      expect(decodeAppCommand(encodeAppCommand(cmd))).toEqual(cmd);
    });

    it.each(sampleFrames.map((frame) => [frame] as const))("round-trips AppFrame %j", (frame) => {
      expect(decodeAppFrame(encodeAppFrame(frame))).toEqual(frame);
    });

    it("tags every AppCommand variant per the agreed contract order (Hello=0 ... Bye=17)", () => {
      expect(encodeAppCommand({ Hello: { channel_version: 0, app_id: "", actor: "", config: [] } })[0]).toBe(0);
      expect(encodeAppCommand({ ConfigCommand: { seq: 0, command: [] } })[0]).toBe(1);
      expect(encodeAppCommand("Bye")[0]).toBe(17);
    });

    it("tags every AppFrame variant per the agreed contract order (Welcome=0 ... Error=13)", () => {
      expect(encodeAppFrame({ Welcome: { channel_version: 0, instance: 0, manifest: [] } })[0]).toBe(0);
      expect(encodeAppFrame({ Error: { in_reply_to: null, code: "", message: "" } })[0]).toBe(13);
    });

    /**
     * 🔒️ Cross-language drift guard: the exact same fixture values and golden hex committed in
     * `protocol_channel`'s own `🔖️Corpus` region (`🔨️module/📡️protocol/🧵️channel/⚡️implementation/🦀️rust/📦️lib.rs`,
     * `channel_command_fixture_corpus`/`channel_command_fixture_hex` and their `AppFrame` twins) —
     * sourced by running the real `encode_app_command`/`encode_app_frame` and copying their
     * printed `[DEBUG] AppCommand::<label> = <hex>` output (`cargo test -p semio-protocol-channel
     * -- --nocapture`), NOT hand-computed. Any future change to either codec that shifts these
     * bytes fails on exactly one side, forcing a deliberate update of both this table and the Rust
     * golden hex in the same change.
     */
    it("matches protocol_channel's own golden hex fixture corpus, byte-for-byte", () => {
            const commandFixtures: readonly (readonly [string, AppCommandValue])[] = [
        ["Hello", { Hello: { channel_version: 3, app_id: "app", actor: "actor", config: [1, 2] } }],
        ["ConfigCommand", { ConfigCommand: { seq: 1, command: [9] } }],
        ["Command", { Command: { seq: 1, command: [1], view_state: [] } }],
        ["CommandText", { CommandText: { seq: 1, line: "go" } }],
        ["RefreshUi", { RefreshUi: { seq: 1, sections: [{ kind: 1, key: "a", hash: 1 }], view_state: [] } }],
        ["ContextMenu", { ContextMenu: { seq: 1, request: [1] } }],
        ["DocumentCommand", { DocumentCommand: { seq: 1, command: [1] } }],
        ["ApplyEnvelopes", { ApplyEnvelopes: { seq: 1, envelopes: [] } }],
        ["LoadDocument", { LoadDocument: { seq: 1, pack: [1], spr: [2] } }],
        ["ReadDocument", { ReadDocument: { seq: 1 } }],
        ["LoadConfig", { LoadConfig: { seq: 1, pack: [1], spr: [2] } }],
        ["ReadConfig", { ReadConfig: { seq: 1 } }],
        ["AttachBackbone", { AttachBackbone: { seq: 1, uri: "u" } }],
        ["DetachBackbone", { DetachBackbone: { seq: 1 } }],
        ["MediaIn", { MediaIn: { seq: 1, port: "p", descriptor: [1], data: [2] } }],
        ["MediaOut", { MediaOut: { seq: 1, port: "p", request: [1] } }],
        ["MediaFingerprint", { MediaFingerprint: { seq: 1, port: "p" } }],
        ["Bye", "Bye"],
      ];
            const commandGoldenHex: Readonly<Record<string, string>> = {
        Hello: "000303617070056163746f72020102",
        ConfigCommand: "01010109",
        Command: "0201010100",
        CommandText: "030102676f",
        RefreshUi: "040101010161010100",
        ContextMenu: "05010101",
        DocumentCommand: "06010101",
        ApplyEnvelopes: "070100",
        LoadDocument: "080101010102",
        ReadDocument: "0901",
        LoadConfig: "0a0101010102",
        ReadConfig: "0b01",
        AttachBackbone: "0c010175",
        DetachBackbone: "0d01",
        MediaIn: "0e01017001010102",
        MediaOut: "0f0101700101",
        MediaFingerprint: "10010170",
        Bye: "11",
      };
            const frameFixtures: readonly (readonly [string, AppFrameValue])[] = [
        ["Welcome", { Welcome: { channel_version: 3, instance: 1, manifest: [1] } }],
        ["Done", { Done: { in_reply_to: 1 } }],
        ["Invocation", { Invocation: { in_reply_to: 1, output: [1], diagnostics: [] } }],
        ["UiSection", { UiSection: { in_reply_to: 1, kind: 1, key: "k", hash: 1, body: null } }],
        ["Effects", { Effects: { in_reply_to: null, effects: [[1]] } }],
        ["Events", { Events: { in_reply_to: null, events: [] } }],
        ["DocumentChanged", { DocumentChanged: { envelopes: [], origin: "o" } }],
        ["Document", { Document: { in_reply_to: 1, pack: [1], spr: [2], ops: "o" } }],
        ["Config", { Config: { in_reply_to: 1, pack: [1], spr: [2], ops: "c" } }],
        ["ConfigChanged", { ConfigChanged: { envelopes: [], origin: "o" } }],
        ["ContextMenu", { ContextMenu: { in_reply_to: 1, items: [1] } }],
        ["Media", { Media: { in_reply_to: 1, port: "p", descriptor: [1], data: [2] } }],
        ["MediaFingerprint", { MediaFingerprint: { in_reply_to: 1, port: "p", fingerprint: [1] } }],
        ["Error", { Error: { in_reply_to: null, code: "c", message: "m" } }],
      ];
            const frameGoldenHex: Readonly<Record<string, string>> = {
        Welcome: "0003010101",
        Done: "0101",
        Invocation: "0201010100",
        UiSection: "03010101016b0100",
        Effects: "0400010101",
        Events: "050000",
        DocumentChanged: "0600016f",
        Document: "070101010102016f",
        Config: "0801010101020163",
        ConfigChanged: "0900016f",
        ContextMenu: "0a010101",
        Media: "0b01017001010102",
        MediaFingerprint: "0c0101700101",
        Error: "0d000163016d",
      };
      const hex = (bytes: Uint8Array) => Array.from(bytes, (b) => b.toString(16).padStart(2, "0")).join("");
      for (const [label, value] of commandFixtures) expect(hex(encodeAppCommand(value)), `AppCommand::${label}`).toBe(commandGoldenHex[label]);
      for (const [label, value] of frameFixtures) expect(hex(encodeAppFrame(value)), `AppFrame::${label}`).toBe(frameGoldenHex[label]);
    });
  });

  describe("@semio-tech/framework-os-core AppChannelClient", () => {
    /** 🧪️ A fake `exchange` that decodes whatever {@link AppChannelClient} encoded and replies with
     * caller-supplied frames — enough to assert the client frames/unframes correctly without a real
     * plugin instance. */
    function fakeHandle(reply: (instanceId: number, commands: AppCommandValue[]) => AppFrameValue[]): AppChannelHandle {
      return {
        exchange: async (instanceId, frames) => {
          const commands = frames.map(decodeAppCommand);
          return reply(instanceId, commands).map(encodeAppFrame);
        },
      };
    }

    it("hello() encodes channel_version/app_id/actor/config and returns the single Welcome reply", async () => {
      let seen: AppCommandValue[] = [];
      const handle = fakeHandle((instanceId, commands) => {
        seen = commands;
        expect(instanceId).toBe(7);
        return [{ Welcome: { channel_version: 3, instance: 7, manifest: [9, 9] } }];
      });
      const client = new AppChannelClient(handle, 7, "app.demo", "actor-1");
      const frame = await client.hello({ mode: "edit" });
      expect(seen).toEqual([{ Hello: { channel_version: 3, app_id: "app.demo", actor: "actor-1", config: Array.from(encodePackValue({ mode: "edit" })) } }]);
      expect(frame).toEqual({ Welcome: { channel_version: 3, instance: 7, manifest: [9, 9] } });
    });

    it("hello() throws when the exchange returns no frame", async () => {
      const client = new AppChannelClient(fakeHandle(() => []), 1, "app.demo");
      await expect(client.hello({})).rejects.toThrow(/no reply frame/);
    });

    it("command() allocates an incrementing seq and returns every frame the batch produced", async () => {
      const seqsSeen: number[] = [];
      const handle = fakeHandle((_instanceId, commands) => {
        const cmd = commands[0];
        if (cmd && cmd !== "Bye" && "Command" in cmd) seqsSeen.push(cmd.Command.seq);
        return [
          { Invocation: { in_reply_to: seqsSeen.at(-1) ?? 0, output: [1], diagnostics: [] } },
          { Effects: { in_reply_to: seqsSeen.at(-1) ?? 0, effects: [] } },
        ];
      });
      const client = new AppChannelClient(handle, 1, "app.demo");
      const first = await client.command(new Uint8Array([1, 2]), { cursor: 0 });
      const second = await client.command(new Uint8Array([3]), { cursor: 1 });
      expect(seqsSeen).toEqual([1, 2]);
      expect(first).toHaveLength(2);
      expect(second).toHaveLength(2);
    });

    it("refreshUi()/configure()/readDocument()/loadDocument() frame the right AppCommand variant", async () => {
      const seen: AppCommandValue[] = [];
      const handle = fakeHandle((_instanceId, commands) => {
        seen.push(...commands);
        return [{ Done: { in_reply_to: 1 } }];
      });
      const client = new AppChannelClient(handle, 1, "app.demo");
      await client.refreshUi([{ kind: 1, key: "panel-a", hash: null }]);
      await client.configure({ locale: "en" });
      await client.readDocument();
      await client.loadDocument(new Uint8Array([1]), new Uint8Array([2]));
      expect(seen[0]).toEqual({ RefreshUi: { seq: 1, sections: [{ kind: 1, key: "panel-a", hash: null }], view_state: Array.from(encodePackValue({})) } });
      expect(seen[1]).toEqual({ ConfigCommand: { seq: 2, command: Array.from(encodePackValue({ locale: "en" })) } });
      expect(seen[2]).toEqual({ ReadDocument: { seq: 3 } });
      expect(seen[3]).toEqual({ LoadDocument: { seq: 4, pack: [1], spr: [2] } });
    });

    it("drain() sends an empty batch and decodes whatever frames come back", async () => {
      const handle = fakeHandle((_instanceId, commands) => {
        expect(commands).toEqual([]);
        return [{ Events: { in_reply_to: null, events: [[1]] } }];
      });
      const client = new AppChannelClient(handle, 1, "app.demo");
      expect(await client.drain()).toEqual([{ Events: { in_reply_to: null, events: [[1]] } }]);
    });
  });
}
//#endregion 🧪️Tests
