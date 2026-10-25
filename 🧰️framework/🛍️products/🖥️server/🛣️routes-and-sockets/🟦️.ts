/** 🛍️ The sockets and routes of the server product: the event stream and document frames, the route table, the error kinds, the capability header, the lanes and the presence socket URL.
 *
 * @see ../🟦️.ts — the TypeScript twin of the server product's wire contract
 */
import type { ActorKey, CapabilityProof, EventRecord, Scope } from "../🧬️wire-types/🟦️.ts";
import { decodeEventRecord } from "../🔤️codec/🟦️.ts";

//#region 🔖️Sockets
/** 📡️ One frame of the durable event-stream socket: the gateway sends each [`EventRecord`] as JSON
 * text, replay and live through the same seam, gap-free and duplicate-free. */
export function decodeEventStreamFrame(text: string): EventRecord {
  return decodeEventRecord(JSON.parse(text), "eventStreamFrame");
}

/** 📄️ One frame of the document socket. Binary frames are the engine's own opaque bytes — the
 * welcome frame first, then whatever the authority answers or relays — and a text frame is the
 * gateway reporting an error on that socket rather than closing it. */
export type DocumentFrame = { readonly kind: "engine"; readonly bytes: Uint8Array } | { readonly kind: "error"; readonly message: string };

/** 🎞️ Classify one document-socket message. */
export function decodeDocumentFrame(message: string | Uint8Array): DocumentFrame {
  return typeof message === "string" ? { kind: "error", message } : { kind: "engine", bytes: message };
}

/** 💡️ Non-identity join hints for a document socket. Actor and session are bound server-side. */
export interface DocumentJoin {
  readonly surface?: string;
  readonly resume?: string;
}

//#endregion 🔖️Sockets
//#region 🔖️Routes
/** 🗂️ The group a gateway route is mounted with: `core` on every instance, `ephemeral`, `blobs` and `apps` only on
 * an instance that asks for them, `documents` only on an instance that hosts a document authority. A route of a group
 * an instance does not mount answers `404`. */
export type ServerRouteGroup = "core" | "ephemeral" | "blobs" | "apps" | "documents";

/** 🛣️ One route of the gateway's own router, before any module adds its own. */
export interface ServerRoute {
  readonly method: string;
  readonly path: string;
  readonly group: ServerRouteGroup;
}

/** 🧭️ The routes `base_router` mounts, in its own order, each with its group.
 *
 * Held against the Rust source itself by this module's suite rather than trusted: the fixture
 * document carries the same table, and the test reads `base_router` out of `📡️gateway/🦀️.rs` and
 * fails if either drifts. A typed client whose route table is a copy of a route table is a client
 * that silently 404s one release later. */
export const SERVER_ROUTES: readonly ServerRoute[] = [
  { method: "GET", path: "/instance", group: "core" },
  { method: "POST", path: "/commands", group: "core" },
  { method: "POST", path: "/queries", group: "core" },
  { method: "GET", path: "/scopes/{scope}/presence/ws", group: "core" },
  { method: "GET", path: "/actors/{tenant}/{kind}/{id}/events", group: "core" },
  { method: "GET", path: "/actors/{tenant}/{kind}/{id}/events/ws", group: "core" },
  { method: "POST", path: "/scopes/{scope}/ephemeral", group: "ephemeral" },
  { method: "GET", path: "/blobs/{hash}", group: "blobs" },
  { method: "HEAD", path: "/blobs/{hash}", group: "blobs" },
  { method: "PUT", path: "/blobs/{hash}", group: "blobs" },
  { method: "GET", path: "/apps", group: "apps" },
  { method: "GET", path: "/apps/{app}/installs", group: "apps" },
  { method: "GET", path: "/apps/{app}", group: "apps" },
  { method: "GET", path: "/apps/{app}/{*rest}", group: "apps" },
  { method: "GET", path: "/scopes/{scope}/document/ws", group: "documents" },
];

/** 🚫️ One error the gateway answers with: its status and the `kind` of its [`ErrorBody`]. */
export interface ServerErrorKind {
  readonly status: number;
  readonly kind: string;
}

/** 🚨️ Every error the gateway answers with. `throttled` (this client address spent an allowance: its allowance of
 * requests, or the named one its body's `allowance` states) and `overloaded` (the instance is at a cap) pass: their
 * body names `retryAfterMs` and the response carries `Retry-After`. `stalled` (the request body did not arrive in
 * time) is worth sending again as well. */
export const SERVER_ERRORS: readonly ServerErrorKind[] = [
  { status: 401, kind: "unauthorized" },
  { status: 403, kind: "forbidden" },
  { status: 404, kind: "notFound" },
  { status: 409, kind: "conflict" },
  { status: 400, kind: "badRequest" },
  { status: 500, kind: "internal" },
  { status: 413, kind: "payloadTooLarge" },
  { status: 408, kind: "stalled" },
  { status: 429, kind: "throttled" },
  { status: 503, kind: "overloaded" },
];

/** 🎟️ The header a caller presents a capability proof in. */
export const CAPABILITY_HEADER = "x-semio-capability";

/** 🪪️ What a client presents to identify itself on every call. */
export interface ServerCredential {
  readonly bearer?: string;
  readonly capability?: CapabilityProof;
}

/** 🌊️ The durable lane key of one actor, as the gateway names it. */
export function streamLane(actor: ActorKey): string {
  return `stream:${actor.tenant}/${actor.kind}/${actor.id}`;
}

/** 🔑️ The document lane key of one scope. */
export function documentLane(scope: Scope): string {
  return `document:${scope}`;
}

/** 💨️ The ephemeral lane key of one scope. */
export function ephemeralLane(scope: Scope): string {
  return `ephemeral:${scope}`;
}

/** 🧍️ The presence lane key of one scope: its room and its batch lane. */
export function presenceLane(scope: Scope): string {
  return `presence:${scope}`;
}

/** 👥️ The websocket URL of one scope's presence room, joined from `surface` (≤ 64 characters). Open it with the
 * [`PRESENCE_PROTOCOL`] subprotocol. */
export function presenceSocketUrl(baseUrl: string, scope: Scope, surface: string): string {
  return `${socketRoot(baseUrl)}/scopes/${encodeURIComponent(scope)}/presence/ws?${new URLSearchParams({ surface }).toString()}`;
}

//#endregion 🔖️Routes
//#region 🔖️Client
export function encodePathSegment(value: string): string {
  return encodeURIComponent(value);
}

/** 🔌️ The websocket origin of an http base URL. */
export function socketRoot(baseUrl: string): string {
  return baseUrl.replace(/\/+$/u, "").replace(/^http/u, "ws");
}
//#endregion 🔖️Client
