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

import type { UtilityLeaf } from "@semio-tech/framework-core";

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

export async function readBackboneEnvelope(uri: string): Promise<string | null> {
  if (uri.startsWith("remote://")) {
    const remote = parseRemoteBackboneUri(uri);
    if (!remote) return null;
    const response = await fetch(remoteEnvelopeUrl(remote));
    if (response.status === 404) return null;
    if (!response.ok) throw new Error(`remote backbone read failed (${response.status})`);
    const body = (await response.json()) as { envelope?: unknown };
    return JSON.stringify(body.envelope ?? body);
  }
  const response = await fetch(`${BACKBONE_ENDPOINT_PATH}?uri=${encodeURIComponent(uri)}`);
  if (response.status === 404) return null;
  if (!response.ok) throw new Error(`backbone read failed (${response.status})`);
  return response.text();
}

export async function writeBackboneEnvelope(uri: string, envelopeJson: string): Promise<void> {
  if (uri.startsWith("remote://")) {
    const remote = parseRemoteBackboneUri(uri);
    if (!remote) throw new Error(`invalid remote backbone uri: ${uri}`);
    const current = await fetch(remoteEnvelopeUrl(remote));
    const version = current.ok ? Number(((await current.json()) as { version?: number }).version ?? 0) : 0;
    const response = await fetch(remoteEnvelopeUrl(remote), {
      method: "PUT",
      headers: { "content-type": "application/json" },
      body: JSON.stringify({ version, envelope: JSON.parse(envelopeJson) }),
    });
    if (!response.ok) throw new Error(`remote backbone write failed (${response.status})`);
    console.log("[DEBUG] remote backbone synced", uri);
    return;
  }
  const response = await fetch(`${BACKBONE_ENDPOINT_PATH}?uri=${encodeURIComponent(uri)}`, {
    method: "PUT",
    headers: { "content-type": "application/json" },
    body: envelopeJson,
  });
  if (!response.ok) throw new Error(`backbone write failed (${response.status})`);
  console.log("[DEBUG] backbone synced", uri);
}

export function documentFromEnvelopeJson(envelopeJson: string): unknown {
  const parsed = JSON.parse(envelopeJson) as { projection?: unknown; document?: unknown; vcs?: unknown };
  if (parsed.projection != null) return parsed.projection;
  if (parsed.document != null) return parsed.document;
  return parsed;
}

export function wrapDocumentEnvelope(document: unknown, documentId: string, uri: string): string {
  if (document && typeof document === "object" && "vcs" in (document as Record<string, unknown>)) {
    const envelope = { ...(document as Record<string, unknown>), backbone: documentBackboneRef(uri) };
    return JSON.stringify(envelope);
  }
  return JSON.stringify({
    schema: "document/v1",
    id: documentId,
    projection: document,
    vcs: { edits: [], changes: [], checkpoints: [], alternatives: [], operations: [] },
    backbone: documentBackboneRef(uri),
  });
}

//#region 🔀️ApplyBackboneMessage
export type BackboneOpEnvelope = { readonly diff?: { readonly payload?: { readonly id?: string } & Record<string, unknown> } };

export type BackboneMessage = { readonly kind: "snapshot"; readonly envelopeJson: string } | { readonly kind: "operations"; readonly envelopes?: readonly BackboneOpEnvelope[] };

/**
 * 🔀️ Mirrors `vcs::storage_send` — applies an incoming backbone message on top of a previously
 * stored envelope: a `snapshot` message overwrites, an `operations` message appends into `vcs.edits`
 * deduped by id. This is the canonical implementation; the dev host shim's generated JS
 * (`hostShimSource` in `framework/os/dev/script.ts`) hand-ports the same algorithm and
 * must be kept in sync until a build-time inlining step exists.
 */
export function applyBackboneMessage(storedEnvelopeJson: string | null, messageJson: string): string {
  const message = JSON.parse(messageJson) as BackboneMessage;
  if (message.kind === "snapshot") return message.envelopeJson;
  if (message.kind === "operations") {
    if (storedEnvelopeJson == null) throw new Error("cannot append operations before a snapshot exists");
    const envelope = JSON.parse(storedEnvelopeJson) as { vcs?: { edits?: unknown[] } };
    const edits = envelope?.vcs?.edits;
    if (!Array.isArray(edits)) throw new Error("stored envelope missing vcs.edits");
    const seen = new Set(edits.map((edit) => (edit as { id?: unknown })?.id).filter((id): id is string => typeof id === "string"));
    for (const operationEnvelope of message.envelopes ?? []) {
      const editJson = operationEnvelope?.diff?.payload;
      const id = editJson?.id;
      if (typeof id === "string") {
        if (seen.has(id)) continue;
        seen.add(id);
      }
      edits.push(editJson);
    }
    return JSON.stringify(envelope);
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
  | { readonly kind: "localSnapshot"; readonly envelopeJson: string }
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
  | { readonly kind: "snapshotReplaced"; readonly envelopeJson: string }
  | ({ readonly kind: "status" } & DocumentSyncStatus)
  | { readonly kind: "presence"; readonly peers: readonly DocumentPresencePeer[] }
  | { readonly kind: "preview"; readonly actor: string; readonly key: string; readonly seq: number; readonly payload: readonly number[] }
  | { readonly kind: "commandOutcome"; readonly batchId: number; readonly outcome: CommandAckOutcome }
  | ({ readonly kind: "conflict" } & SyncConflict);

/** 📤️ Main thread → `🟦️backbone-worker.ts` messages. */
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

    it("wraps and unwraps document envelopes", () => {
      const envelopeJson = wrapDocumentEnvelope({ nodes: [] }, "doc-1", "file:///tmp/a.json");
      const envelope = JSON.parse(envelopeJson) as { schema: string; id: string; projection: unknown; backbone: unknown };
      expect(envelope.schema).toBe("document/v1");
      expect(envelope.id).toBe("doc-1");
      expect(documentFromEnvelopeJson(envelopeJson)).toEqual({ nodes: [] });
    });

    it("preserves an existing vcs envelope instead of re-wrapping it", () => {
      const existing = { vcs: { edits: [], changes: [], checkpoints: [], alternatives: [], operations: [] }, projection: { a: 1 } };
      const envelopeJson = wrapDocumentEnvelope(existing, "doc-1", "file:///tmp/a.json");
      const envelope = JSON.parse(envelopeJson) as { projection: unknown; vcs: unknown };
      expect(envelope.projection).toEqual({ a: 1 });
    });

    it("applies a snapshot message by overwriting the stored envelope", () => {
      const messageJson = JSON.stringify({ kind: "snapshot", envelopeJson: '{"vcs":{"edits":[]}}' });
      expect(applyBackboneMessage(null, messageJson)).toBe('{"vcs":{"edits":[]}}');
    });

    it("applies an operations message by appending deduped edits into vcs.edits", () => {
      const stored = JSON.stringify({ vcs: { edits: [{ id: "e1" }] } });
      const messageJson = JSON.stringify({
        kind: "operations",
        envelopes: [{ diff: { payload: { id: "e1" } } }, { diff: { payload: { id: "e2" } } }],
      });
      const result = JSON.parse(applyBackboneMessage(stored, messageJson)) as { vcs: { edits: Array<{ id: string }> } };
      expect(result.vcs.edits.map((edit) => edit.id)).toEqual(["e1", "e2"]);
    });

    it("throws when applying an operations message before a snapshot exists", () => {
      const messageJson = JSON.stringify({ kind: "operations", envelopes: [] });
      expect(() => applyBackboneMessage(null, messageJson)).toThrow("cannot append operations before a snapshot exists");
    });

    it("throws on an unsupported backbone message kind", () => {
      const messageJson = JSON.stringify({ kind: "bogus" });
      expect(() => applyBackboneMessage(null, messageJson)).toThrow("unsupported backbone message kind: bogus");
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
      const fixturesDir = join(here, "..", "fixtures");
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
        const spkBytes = new Uint8Array(readFileSync(join(fixturesDir, dslFile.replace(/\.dsl$/, ".spk"))));
        const viaDsl = wasmModule.parseWorkflowFixtureDsl(dslText);
        const viaPack = wasmModule.decodeWorkflowFixturePack(spkBytes);
        expect(viaDsl).toEqual(viaPack);
        const deliveries = planWorkflow(viaDsl.graph, new Set(viaDsl.dirtyInstanceIds));
        expect(deliveries).toEqual(viaDsl.expectedDeliveries);
      }
    });
  });
}
//#endregion 🧪️Tests
