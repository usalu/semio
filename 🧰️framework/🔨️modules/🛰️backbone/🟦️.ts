/** 🧵️ The backbone root: the sync controller id, the backbone endpoint and stream mux paths, backbone URIs and document pack byte helpers, the framework sync utilities and the blob endpoint path.
 *
 * 🧱️ The children own the rest: `🧭️opening-relay`, `🌐️envelope-io`, `🔀️message`, `🪟️desktop-window`, `🔑️persistence-binding`, `🎭️artifact-actor`, the backbone worker protocol (`🧱️worker-wire`, `🔬️worker-parsers`, `📨️worker-messages`) and `🗺️workflow-planner`.
 */

import type { UtilityLeaf } from "@semio-tech/framework";
import { readVarintU64, writeVarintU64 } from "@semio-tech/framework-replication";
import { decodePackValue, encodePackValue } from "../🎁️packs/🟦️.ts";

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

export function remoteEnvelopeUrl(remote: { readonly hostPort: string; readonly spaceId: string; readonly documentId: string }): string {
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

export const BACKBONE_OCTET_STREAM = "application/octet-stream";

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

//#region 🔖️Blob
/** 📦️ Dev-server-proxied content-addressed blob endpoint: `PUT ${BLOB_ENDPOINT_PATH}?mediaType=` (raw
 * bytes body, returns `{"hash":"..."}`) and `GET ${BLOB_ENDPOINT_PATH}/:hash` (raw bytes response).
 * Shared with the dev host shim (`framework/os/dev/script.ts`'s `hostShimSource`) and the
 * browser blob cache (`🏪️store/👷️worker/🟦️.ts`) so all three stay in sync on the same literal. Backed by
 * `vcs::BlobStore`'s native counterpart; a hub-backed route is a later ticket. */
export const BLOB_ENDPOINT_PATH = "/semio-blob";
//#endregion 🔖️Blob
