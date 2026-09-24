/** 🖥️ `@semio-tech/framework-server` — the TypeScript twin of the server product's wire contract:
 * every type the gateway's twelve routes and two websockets carry, a codec that converts each one
 * between its wire JSON and an owned TypeScript value, and a typed client over an owned transport
 * interface.
 *
 * **Schema-first, and the schema is not this file.** The wire shape is owned by the Rust contract's
 * serde derives (`🔨️modules/🧬️contract/🦀️.rs`) and the gateway's route table
 * (`🔨️modules/📡️gateway/🦀️.rs`, `base_router`). Neither is expressible in TypeScript, so the shared
 * artifact both languages are held against is the language-agnostic fixture document
 * `🧫️fixtures/🔌️wire/🔣️.json`: it carries the route table and one canonical JSON vector per wire
 * type, and both twins round-trip every vector byte-for-byte in their own test suite. A rename on
 * either side fails the other side's suite, which is the only mechanism that keeps two
 * implementations of one contract honest.
 *
 * **Three serde spellings live in this contract and all three are deliberate.** Most structs carry
 * `rename_all = "camelCase"`. A few — `TraceContext`, `FrontierSummary` — carry none and stay
 * snake_case on the wire. And an internally tagged enum's `rename_all` renames its *variants*, never
 * the fields inside them, so `CommandOutcome::Transformed` carries `canonical_events` and
 * `Rejection::UnknownCommandKind` carries `command_kind` while their tags are camelCase. Every one
 * of those is pinned by a fixture vector rather than by a comment.
 *
 * **Bytes.** Rust's `Vec<u8>` serializes as a JSON array of numbers. The wire types below say
 * `number[]`; the owned types say `Uint8Array`, because a caller holding a payload wants bytes and
 * not a thousand-element array. The codec is the boundary between the two.
 *
 * **`u64` on the wire.** `seq`, `revision`, `millis` and the frontier counters are 64-bit in Rust and
 * `number` here. JavaScript is exact to 2^53, which every one of these is far below in practice; a
 * counter that ever approaches it needs a wire change on both sides, not a silent `bigint` here.
 *
 * @see 🔨️modules/🧬️contract/🦀️.rs
 * @see 🔨️modules/📡️gateway/🦀️.rs
 * @see 🧫️fixtures/🔌️wire/🔣️.json
 */

//#region 🔖️Identity
/** 🏢️ The instance-wide tenancy root — a newtype over a string in Rust, a bare string on the wire. */
export type TenantId = string;

/** 🗂️ A scope inside a tenant: the space a command or query is addressed within. */
export type Scope = string;

/** 🆔️ Client-minted identity of one command submission, stable across retries. */
export type CommandId = string;

/** 🔁️ Deduplication key: two submissions carrying it produce one effect and the same receipt. */
export type IdempotencyKey = string;

/** 🎫️ One authenticated session of a principal on one device. */
export type SessionId = string;

/** 📱️ A device a session runs on. */
export type DeviceId = string;

/** 🆔️ Identity of one query submission. */
export type QueryId = string;

/** 📑️ Opaque continuation token for a paged or subscribed read. */
export type QueryCursor = string;

/** 🔔️ Identity of an established live projection subscription. */
export type SubscriptionId = string;

/** 🛂️ A capability the caller presents to justify an action policy would otherwise deny. */
export type CapabilityProof = string;

/** 🔢️ An actor's monotonically increasing revision, used for optimistic concurrency. */
export type Revision = number;

/** 🎭️ The durable address of one authority actor. */
export interface ActorKey {
  readonly tenant: TenantId;
  readonly kind: string;
  readonly id: string;
}

/** 🙋️ Who is acting. Never a role — roles are policy templates evaluated against a principal. */
export type Principal =
  | { readonly kind: "user"; readonly id: string }
  | { readonly kind: "serviceAccount"; readonly id: string }
  | { readonly kind: "device"; readonly id: string }
  | { readonly kind: "anonymous" };

/** 🕰️ A hybrid-logical clock reading. */
export interface HybridLogicalClock {
  readonly millis: number;
  readonly counter: number;
}

/** 🔍️ Distributed-trace correlation for one submission. Snake_case on the wire, deliberately. */
export interface TraceContext {
  readonly traceId: string;
  readonly spanId: string;
}

/** 🏔️ The causal frontier summary the replication contract exchanges. Snake_case on the wire. */
export interface FrontierSummary {
  readonly documentId: string;
  readonly headEditOrdinal: number;
  readonly headEditId: string;
  readonly lastCommitSeq: number;
  readonly chainHash: Uint8Array;
}
//#endregion 🔖️Identity

//#region 🔖️Command
/** 📨️ One durable intent addressed to one authority actor; `payload` stays opaque to this contract. */
export interface CommandEnvelope {
  readonly commandId: CommandId;
  readonly kind: string;
  readonly version: number;
  readonly target: ActorKey;
  readonly scope: Scope;
  readonly principal: Principal;
  readonly session: SessionId | null;
  readonly device: DeviceId | null;
  readonly payload: Uint8Array;
  readonly causalFrontier: FrontierSummary | null;
  readonly clientHlc: HybridLogicalClock;
  readonly expectedRevision: Revision | null;
  readonly idempotencyKey: IdempotencyKey | null;
  readonly capabilityProof: CapabilityProof | null;
  readonly trace: TraceContext;
}

/** 🧾️ Proof the authority processed a command, returned identically on every retry of the same key. */
export interface CommandReceipt {
  readonly commandId: CommandId;
  readonly actor: ActorKey;
  readonly revision: Revision;
  readonly acceptedAt: HybridLogicalClock;
}

/** 🚫️ Why an authority refused a command. */
export type Rejection =
  | { readonly kind: "unauthorized"; readonly detail: string }
  | { readonly kind: "revisionConflict"; readonly expected: Revision; readonly actual: Revision }
  | { readonly kind: "invalid"; readonly detail: string }
  | { readonly kind: "unknownCommandKind"; readonly commandKind: string }
  | { readonly kind: "actorUnavailable"; readonly detail: string };

/** 💬️ A human-facing note attached to an outcome. */
export interface Notice {
  readonly code: string;
  readonly message: string;
}

/** 🧵️ A long-running workflow a command started. */
export type ProcessId = string;

/** 🎯️ What the authority decided. `transformed` is the collaborative case: accepted but rebased. */
export type CommandOutcome =
  | { readonly status: "accepted"; readonly receipt: CommandReceipt; readonly events: readonly EventRecord[]; readonly frontier: FrontierSummary | null }
  | { readonly status: "transformed"; readonly receipt: CommandReceipt; readonly canonicalEvents: readonly EventRecord[]; readonly frontier: FrontierSummary | null; readonly notices: readonly Notice[] }
  | { readonly status: "rejected"; readonly receipt: CommandReceipt; readonly reason: Rejection; readonly notices: readonly Notice[] }
  | { readonly status: "pending"; readonly receipt: CommandReceipt; readonly process: ProcessId };

/** 📴️ What a command kind may do while the replica is detached. */
export type OfflinePolicy = "optimistic" | "deferred" | "authorityRequired";
//#endregion 🔖️Command

//#region 🔖️Query
/** 🧭️ How fresh an answer must be. */
export type QueryConsistency = { readonly kind: "local" } | { readonly kind: "atFrontier"; readonly frontier: FrontierSummary } | { readonly kind: "authority" };

/** ❓️ One read addressed at a projection, never at an actor's private state. */
export interface QueryEnvelope {
  readonly queryId: QueryId;
  readonly kind: string;
  readonly version: number;
  readonly scope: Scope;
  readonly principal: Principal;
  readonly arguments: Uint8Array;
  readonly consistency: QueryConsistency;
  readonly cursor: QueryCursor | null;
}

/** 📤️ What a query returns: a whole value, one page, or a live subscription handle. */
export type QueryResult =
  | { readonly kind: "snapshot"; readonly value: Uint8Array; readonly frontier: FrontierSummary | null }
  | { readonly kind: "page"; readonly items: readonly Uint8Array[]; readonly next: QueryCursor | null; readonly frontier: FrontierSummary | null }
  | { readonly kind: "subscription"; readonly subscription: SubscriptionId; readonly initial: Uint8Array; readonly cursor: QueryCursor | null; readonly frontier: FrontierSummary | null };
//#endregion 🔖️Query

//#region 🔖️Lanes
/** 📚️ One durable, replayable fact an actor emitted. */
export interface EventRecord {
  readonly stream: ActorKey;
  readonly seq: number;
  readonly hlc: HybridLogicalClock;
  readonly kind: string;
  readonly payload: Uint8Array;
}

/** 💨️ One lossy, expiring frame — never replayed into durable state. */
export interface EphemeralFrame {
  readonly scope: Scope;
  readonly principal: Principal;
  readonly kind: string;
  readonly payload: Uint8Array;
}
//#endregion 🔖️Lanes

//#region 🔖️Policy
/** 🚦️ Every point authorization is actually decided at. */
export type PolicyPoint = "commandAdmission" | "commandExecution" | "queryAccess" | "subscription" | "eventDelivery" | "blobRead" | "blobWrite" | "effect" | "administration";

/** 🔑️ One grant inside a template: an action on a resource pattern at a policy point. */
export interface PolicyGrant {
  readonly point: PolicyPoint;
  readonly resource: string;
  readonly action: string;
}

/** 🎓️ A named bundle of grants; `admin`/`editor`/`viewer` are values of this type, never enums. */
export interface PolicyTemplate {
  readonly name: string;
  readonly autoApply: boolean;
  readonly grants: readonly PolicyGrant[];
}
//#endregion 🔖️Policy

//#region 🔖️Module
/** 📇️ What one command kind declares to the instance that registers it. */
export interface CommandDescriptor {
  readonly kind: string;
  readonly version: number;
  readonly actorKind: string;
  readonly offline: OfflinePolicy;
}

/** 📇️ What one query kind declares. */
export interface QueryDescriptor {
  readonly kind: string;
  readonly version: number;
  readonly projection: string;
}

/** 🧾️ The declarative half of a server module. */
export interface ModuleManifest {
  readonly id: string;
  readonly commands: readonly CommandDescriptor[];
  readonly queries: readonly QueryDescriptor[];
  readonly projections: readonly string[];
  readonly policies: readonly PolicyTemplate[];
  readonly actorKinds: readonly string[];
}

/** 🏛️ One deployable server: its identity plus the modules composing it. */
export interface ServerInstanceDefinition {
  readonly id: string;
  readonly version: string;
  readonly modules: readonly ModuleManifest[];
}
//#endregion 🔖️Module

//#region 🔖️Transport
/** 🧾️ The JSON body every gateway error renders as, so a client never parses a status line. */
export interface ErrorBody {
  readonly kind: string;
  readonly message: string;
}

/** 🧾️ What a successful blob upload answers with. */
export interface BlobReceipt {
  readonly hash: string;
  readonly size: number;
}

/** 🧩️ One installed entry discovered under an app root. */
export interface AppInstall {
  readonly id: string;
  readonly manifest: unknown;
}

/** 💥️ A wire value that does not match the contract. Thrown by every decoder in this module. */
export class WireError extends Error {
  readonly path: string;

  constructor(message: string, path: string) {
    super(path === "" ? message : `${path}: ${message}`);
    this.name = "WireError";
    this.path = path;
  }
}

/** 🚫️ A gateway answer that was not a success, carrying the tag a client branches on. */
export class ServerCallError extends Error {
  readonly status: number;
  readonly kind: string;

  constructor(status: number, body: ErrorBody) {
    super(`${body.kind}: ${body.message}`);
    this.name = "ServerCallError";
    this.status = status;
    this.kind = body.kind;
  }
}

/** 📨️ One request the client asks its transport to perform, in owned terms only. */
export interface HttpRequest {
  readonly method: "GET" | "HEAD" | "POST" | "PUT";
  readonly path: string;
  readonly query: Readonly<Record<string, string>>;
  readonly headers: Readonly<Record<string, string>>;
  readonly body?: string | Uint8Array;
}

/** 📬️ What a transport answers with. Bodies are pulled lazily so a `HEAD` never reads one. */
export interface HttpResponse {
  readonly status: number;
  text(): Promise<string>;
  bytes(): Promise<Uint8Array>;
}

/** 🚚️ The seam between this client and whatever performs the request.
 *
 * An interface rather than `fetch` directly, for the reason the repo states generally: nothing this
 * module exports may name a type it does not own. A host adapts its own `fetch`, its own test double
 * or its own instrumented pipeline with [`fetchTransport`] or by implementing this. */
export interface HttpTransport {
  send(request: HttpRequest): Promise<HttpResponse>;
}

/** 🌐️ Adapt the platform's `fetch` to [`HttpTransport`], rooted at one server's base URL. */
export function fetchTransport(baseUrl: string): HttpTransport {
  const root = baseUrl.replace(/\/+$/u, "");
  return {
    async send(request: HttpRequest): Promise<HttpResponse> {
      const query = new URLSearchParams(request.query as Record<string, string>).toString();
      const response = await fetch(`${root}${request.path}${query === "" ? "" : `?${query}`}`, {
        method: request.method,
        headers: { ...request.headers },
        body: request.body === undefined ? undefined : typeof request.body === "string" ? request.body : new Uint8Array(request.body),
      });
      return {
        status: response.status,
        text: () => response.text(),
        bytes: async () => new Uint8Array(await response.arrayBuffer()),
      };
    },
  };
}
//#endregion 🔖️Transport

//#region 🔖️Primitives
function fail(message: string, path: string): never {
  throw new WireError(message, path);
}

function asObject(value: unknown, path: string): Record<string, unknown> {
  if (value === null || typeof value !== "object" || Array.isArray(value)) fail("expected an object", path);
  return value as Record<string, unknown>;
}

function asArray(value: unknown, path: string): unknown[] {
  if (!Array.isArray(value)) fail("expected an array", path);
  return value;
}

function asString(value: unknown, path: string): string {
  if (typeof value !== "string") fail("expected a string", path);
  return value;
}

function asNumber(value: unknown, path: string): number {
  if (typeof value !== "number" || !Number.isFinite(value)) fail("expected a number", path);
  return value;
}

function asBoolean(value: unknown, path: string): boolean {
  if (typeof value !== "boolean") fail("expected a boolean", path);
  return value;
}

function asBytes(value: unknown, path: string): Uint8Array {
  const items = asArray(value, path);
  const bytes = new Uint8Array(items.length);
  for (const [index, item] of items.entries()) {
    const byte = asNumber(item, `${path}[${index}]`);
    if (!Number.isInteger(byte) || byte < 0 || byte > 255) fail("expected a byte in 0..=255", `${path}[${index}]`);
    bytes[index] = byte;
  }
  return bytes;
}

function optional<T>(value: unknown, path: string, decode: (inner: unknown, path: string) => T): T | null {
  return value === null || value === undefined ? null : decode(value, path);
}

function tag(value: Record<string, unknown>, field: string, path: string): string {
  return asString(value[field], `${path}.${field}`);
}

function bytesToWire(bytes: Uint8Array): number[] {
  return Array.from(bytes);
}
//#endregion 🔖️Primitives

//#region 🔖️Codec
/** 🎭️ Decode one [`ActorKey`]. */
export function decodeActorKey(value: unknown, path = "actorKey"): ActorKey {
  const row = asObject(value, path);
  return { tenant: asString(row.tenant, `${path}.tenant`), kind: asString(row.kind, `${path}.kind`), id: asString(row.id, `${path}.id`) };
}

/** 🎭️ Encode one [`ActorKey`]. */
export function encodeActorKey(value: ActorKey): unknown {
  return { tenant: value.tenant, kind: value.kind, id: value.id };
}

/** 🙋️ Decode one [`Principal`]. */
export function decodePrincipal(value: unknown, path = "principal"): Principal {
  const row = asObject(value, path);
  const kind = tag(row, "kind", path);
  switch (kind) {
    case "user":
    case "serviceAccount":
    case "device":
      return { kind, id: asString(row.id, `${path}.id`) };
    case "anonymous":
      return { kind };
    default:
      return fail(`unknown principal kind ${JSON.stringify(kind)}`, path);
  }
}

/** 🙋️ Encode one [`Principal`]. */
export function encodePrincipal(value: Principal): unknown {
  return value.kind === "anonymous" ? { kind: "anonymous" } : { kind: value.kind, id: value.id };
}

/** 🕰️ Decode one [`HybridLogicalClock`]. */
export function decodeHybridLogicalClock(value: unknown, path = "hlc"): HybridLogicalClock {
  const row = asObject(value, path);
  return { millis: asNumber(row.millis, `${path}.millis`), counter: asNumber(row.counter, `${path}.counter`) };
}

/** 🕰️ Encode one [`HybridLogicalClock`]. */
export function encodeHybridLogicalClock(value: HybridLogicalClock): unknown {
  return { millis: value.millis, counter: value.counter };
}

/** 🔍️ Decode one [`TraceContext`] from its snake_case wire shape. */
export function decodeTraceContext(value: unknown, path = "trace"): TraceContext {
  const row = asObject(value, path);
  return { traceId: asString(row.trace_id, `${path}.trace_id`), spanId: asString(row.span_id, `${path}.span_id`) };
}

/** 🔍️ Encode one [`TraceContext`] into its snake_case wire shape. */
export function encodeTraceContext(value: TraceContext): unknown {
  return { trace_id: value.traceId, span_id: value.spanId };
}

/** 🏔️ Decode one [`FrontierSummary`] from its snake_case wire shape. */
export function decodeFrontierSummary(value: unknown, path = "frontier"): FrontierSummary {
  const row = asObject(value, path);
  const chainHash = asBytes(row.chain_hash, `${path}.chain_hash`);
  if (chainHash.length !== 32) fail(`expected exactly 32 bytes, found ${chainHash.length}`, `${path}.chain_hash`);
  return {
    documentId: asString(row.document_id, `${path}.document_id`),
    headEditOrdinal: asNumber(row.head_edit_ordinal, `${path}.head_edit_ordinal`),
    headEditId: asString(row.head_edit_id, `${path}.head_edit_id`),
    lastCommitSeq: asNumber(row.last_commit_seq, `${path}.last_commit_seq`),
    chainHash,
  };
}

/** 🏔️ Encode one [`FrontierSummary`] into its snake_case wire shape. */
export function encodeFrontierSummary(value: FrontierSummary): unknown {
  return {
    document_id: value.documentId,
    head_edit_ordinal: value.headEditOrdinal,
    head_edit_id: value.headEditId,
    last_commit_seq: value.lastCommitSeq,
    chain_hash: bytesToWire(value.chainHash),
  };
}

/** 📚️ Decode one [`EventRecord`]. */
export function decodeEventRecord(value: unknown, path = "event"): EventRecord {
  const row = asObject(value, path);
  return {
    stream: decodeActorKey(row.stream, `${path}.stream`),
    seq: asNumber(row.seq, `${path}.seq`),
    hlc: decodeHybridLogicalClock(row.hlc, `${path}.hlc`),
    kind: asString(row.kind, `${path}.kind`),
    payload: asBytes(row.payload, `${path}.payload`),
  };
}

/** 📚️ Encode one [`EventRecord`]. */
export function encodeEventRecord(value: EventRecord): unknown {
  return { stream: encodeActorKey(value.stream), seq: value.seq, hlc: encodeHybridLogicalClock(value.hlc), kind: value.kind, payload: bytesToWire(value.payload) };
}

/** 💨️ Decode one [`EphemeralFrame`]. */
export function decodeEphemeralFrame(value: unknown, path = "ephemeralFrame"): EphemeralFrame {
  const row = asObject(value, path);
  return {
    scope: asString(row.scope, `${path}.scope`),
    principal: decodePrincipal(row.principal, `${path}.principal`),
    kind: asString(row.kind, `${path}.kind`),
    payload: asBytes(row.payload, `${path}.payload`),
  };
}

/** 💨️ Encode one [`EphemeralFrame`]. */
export function encodeEphemeralFrame(value: EphemeralFrame): unknown {
  return { scope: value.scope, principal: encodePrincipal(value.principal), kind: value.kind, payload: bytesToWire(value.payload) };
}

/** 📨️ Decode one [`CommandEnvelope`]. */
export function decodeCommandEnvelope(value: unknown, path = "commandEnvelope"): CommandEnvelope {
  const row = asObject(value, path);
  return {
    commandId: asString(row.commandId, `${path}.commandId`),
    kind: asString(row.kind, `${path}.kind`),
    version: asNumber(row.version, `${path}.version`),
    target: decodeActorKey(row.target, `${path}.target`),
    scope: asString(row.scope, `${path}.scope`),
    principal: decodePrincipal(row.principal, `${path}.principal`),
    session: optional(row.session, `${path}.session`, asString),
    device: optional(row.device, `${path}.device`, asString),
    payload: asBytes(row.payload, `${path}.payload`),
    causalFrontier: optional(row.causalFrontier, `${path}.causalFrontier`, decodeFrontierSummary),
    clientHlc: decodeHybridLogicalClock(row.clientHlc, `${path}.clientHlc`),
    expectedRevision: optional(row.expectedRevision, `${path}.expectedRevision`, asNumber),
    idempotencyKey: optional(row.idempotencyKey, `${path}.idempotencyKey`, asString),
    capabilityProof: optional(row.capabilityProof, `${path}.capabilityProof`, asString),
    trace: decodeTraceContext(row.trace, `${path}.trace`),
  };
}

/** 📨️ Encode one [`CommandEnvelope`]. */
export function encodeCommandEnvelope(value: CommandEnvelope): unknown {
  return {
    commandId: value.commandId,
    kind: value.kind,
    version: value.version,
    target: encodeActorKey(value.target),
    scope: value.scope,
    principal: encodePrincipal(value.principal),
    session: value.session,
    device: value.device,
    payload: bytesToWire(value.payload),
    causalFrontier: value.causalFrontier === null ? null : encodeFrontierSummary(value.causalFrontier),
    clientHlc: encodeHybridLogicalClock(value.clientHlc),
    expectedRevision: value.expectedRevision,
    idempotencyKey: value.idempotencyKey,
    capabilityProof: value.capabilityProof,
    trace: encodeTraceContext(value.trace),
  };
}

/** 🧾️ Decode one [`CommandReceipt`]. */
export function decodeCommandReceipt(value: unknown, path = "receipt"): CommandReceipt {
  const row = asObject(value, path);
  return {
    commandId: asString(row.commandId, `${path}.commandId`),
    actor: decodeActorKey(row.actor, `${path}.actor`),
    revision: asNumber(row.revision, `${path}.revision`),
    acceptedAt: decodeHybridLogicalClock(row.acceptedAt, `${path}.acceptedAt`),
  };
}

/** 🧾️ Encode one [`CommandReceipt`]. */
export function encodeCommandReceipt(value: CommandReceipt): unknown {
  return { commandId: value.commandId, actor: encodeActorKey(value.actor), revision: value.revision, acceptedAt: encodeHybridLogicalClock(value.acceptedAt) };
}

/** 🚫️ Decode one [`Rejection`]. Only the tag is camelCase; the fields inside a variant are not. */
export function decodeRejection(value: unknown, path = "rejection"): Rejection {
  const row = asObject(value, path);
  const kind = tag(row, "kind", path);
  switch (kind) {
    case "unauthorized":
    case "invalid":
    case "actorUnavailable":
      return { kind, detail: asString(row.detail, `${path}.detail`) };
    case "revisionConflict":
      return { kind, expected: asNumber(row.expected, `${path}.expected`), actual: asNumber(row.actual, `${path}.actual`) };
    case "unknownCommandKind":
      return { kind, commandKind: asString(row.command_kind, `${path}.command_kind`) };
    default:
      return fail(`unknown rejection kind ${JSON.stringify(kind)}`, path);
  }
}

/** 🚫️ Encode one [`Rejection`]. */
export function encodeRejection(value: Rejection): unknown {
  switch (value.kind) {
    case "revisionConflict":
      return { kind: value.kind, expected: value.expected, actual: value.actual };
    case "unknownCommandKind":
      return { kind: value.kind, command_kind: value.commandKind };
    default:
      return { kind: value.kind, detail: value.detail };
  }
}

function decodeNotice(value: unknown, path: string): Notice {
  const row = asObject(value, path);
  return { code: asString(row.code, `${path}.code`), message: asString(row.message, `${path}.message`) };
}

function decodeNotices(value: unknown, path: string): Notice[] {
  return asArray(value, path).map((entry, index) => decodeNotice(entry, `${path}[${index}]`));
}

function decodeEvents(value: unknown, path: string): EventRecord[] {
  return asArray(value, path).map((entry, index) => decodeEventRecord(entry, `${path}[${index}]`));
}

/** 🎯️ Decode one [`CommandOutcome`]. `canonical_events` keeps its snake_case: an internally tagged
 * enum's `rename_all` renames variants, never the fields inside them. */
export function decodeCommandOutcome(value: unknown, path = "outcome"): CommandOutcome {
  const row = asObject(value, path);
  const status = tag(row, "status", path);
  const receipt = decodeCommandReceipt(row.receipt, `${path}.receipt`);
  const frontier = () => optional(row.frontier, `${path}.frontier`, decodeFrontierSummary);
  switch (status) {
    case "accepted":
      return { status, receipt, events: decodeEvents(row.events, `${path}.events`), frontier: frontier() };
    case "transformed":
      return { status, receipt, canonicalEvents: decodeEvents(row.canonical_events, `${path}.canonical_events`), frontier: frontier(), notices: decodeNotices(row.notices, `${path}.notices`) };
    case "rejected":
      return { status, receipt, reason: decodeRejection(row.reason, `${path}.reason`), notices: decodeNotices(row.notices, `${path}.notices`) };
    case "pending":
      return { status, receipt, process: asString(row.process, `${path}.process`) };
    default:
      return fail(`unknown command outcome status ${JSON.stringify(status)}`, path);
  }
}

/** 🎯️ Encode one [`CommandOutcome`]. */
export function encodeCommandOutcome(value: CommandOutcome): unknown {
  const receipt = encodeCommandReceipt(value.receipt);
  switch (value.status) {
    case "accepted":
      return { status: value.status, receipt, events: value.events.map(encodeEventRecord), frontier: value.frontier === null ? null : encodeFrontierSummary(value.frontier) };
    case "transformed":
      return {
        status: value.status,
        receipt,
        canonical_events: value.canonicalEvents.map(encodeEventRecord),
        frontier: value.frontier === null ? null : encodeFrontierSummary(value.frontier),
        notices: value.notices.map((notice) => ({ code: notice.code, message: notice.message })),
      };
    case "rejected":
      return { status: value.status, receipt, reason: encodeRejection(value.reason), notices: value.notices.map((notice) => ({ code: notice.code, message: notice.message })) };
    case "pending":
      return { status: value.status, receipt, process: value.process };
  }
}

/** 🧭️ Decode one [`QueryConsistency`]. */
export function decodeQueryConsistency(value: unknown, path = "consistency"): QueryConsistency {
  const row = asObject(value, path);
  const kind = tag(row, "kind", path);
  switch (kind) {
    case "local":
    case "authority":
      return { kind };
    case "atFrontier":
      return { kind, frontier: decodeFrontierSummary(row.frontier, `${path}.frontier`) };
    default:
      return fail(`unknown query consistency ${JSON.stringify(kind)}`, path);
  }
}

/** 🧭️ Encode one [`QueryConsistency`]. */
export function encodeQueryConsistency(value: QueryConsistency): unknown {
  return value.kind === "atFrontier" ? { kind: value.kind, frontier: encodeFrontierSummary(value.frontier) } : { kind: value.kind };
}

/** ❓️ Decode one [`QueryEnvelope`]. */
export function decodeQueryEnvelope(value: unknown, path = "queryEnvelope"): QueryEnvelope {
  const row = asObject(value, path);
  return {
    queryId: asString(row.queryId, `${path}.queryId`),
    kind: asString(row.kind, `${path}.kind`),
    version: asNumber(row.version, `${path}.version`),
    scope: asString(row.scope, `${path}.scope`),
    principal: decodePrincipal(row.principal, `${path}.principal`),
    arguments: asBytes(row.arguments, `${path}.arguments`),
    consistency: decodeQueryConsistency(row.consistency, `${path}.consistency`),
    cursor: optional(row.cursor, `${path}.cursor`, asString),
  };
}

/** ❓️ Encode one [`QueryEnvelope`]. */
export function encodeQueryEnvelope(value: QueryEnvelope): unknown {
  return {
    queryId: value.queryId,
    kind: value.kind,
    version: value.version,
    scope: value.scope,
    principal: encodePrincipal(value.principal),
    arguments: bytesToWire(value.arguments),
    consistency: encodeQueryConsistency(value.consistency),
    cursor: value.cursor,
  };
}

/** 📤️ Decode one [`QueryResult`]. */
export function decodeQueryResult(value: unknown, path = "queryResult"): QueryResult {
  const row = asObject(value, path);
  const kind = tag(row, "kind", path);
  const frontier = () => optional(row.frontier, `${path}.frontier`, decodeFrontierSummary);
  switch (kind) {
    case "snapshot":
      return { kind, value: asBytes(row.value, `${path}.value`), frontier: frontier() };
    case "page":
      return { kind, items: asArray(row.items, `${path}.items`).map((entry, index) => asBytes(entry, `${path}.items[${index}]`)), next: optional(row.next, `${path}.next`, asString), frontier: frontier() };
    case "subscription":
      return { kind, subscription: asString(row.subscription, `${path}.subscription`), initial: asBytes(row.initial, `${path}.initial`), cursor: optional(row.cursor, `${path}.cursor`, asString), frontier: frontier() };
    default:
      return fail(`unknown query result kind ${JSON.stringify(kind)}`, path);
  }
}

/** 📤️ Encode one [`QueryResult`]. */
export function encodeQueryResult(value: QueryResult): unknown {
  switch (value.kind) {
    case "snapshot":
      return { kind: value.kind, value: bytesToWire(value.value), frontier: value.frontier === null ? null : encodeFrontierSummary(value.frontier) };
    case "page":
      return { kind: value.kind, items: value.items.map(bytesToWire), next: value.next, frontier: value.frontier === null ? null : encodeFrontierSummary(value.frontier) };
    case "subscription":
      return { kind: value.kind, subscription: value.subscription, initial: bytesToWire(value.initial), cursor: value.cursor, frontier: value.frontier === null ? null : encodeFrontierSummary(value.frontier) };
  }
}

function decodePolicyPoint(value: unknown, path: string): PolicyPoint {
  const point = asString(value, path);
  const points: readonly string[] = ["commandAdmission", "commandExecution", "queryAccess", "subscription", "eventDelivery", "blobRead", "blobWrite", "effect", "administration"];
  if (!points.includes(point)) fail(`unknown policy point ${JSON.stringify(point)}`, path);
  return point as PolicyPoint;
}

function decodeOfflinePolicy(value: unknown, path: string): OfflinePolicy {
  const policy = asString(value, path);
  if (policy !== "optimistic" && policy !== "deferred" && policy !== "authorityRequired") fail(`unknown offline policy ${JSON.stringify(policy)}`, path);
  return policy;
}

function decodePolicyTemplate(value: unknown, path: string): PolicyTemplate {
  const row = asObject(value, path);
  return {
    name: asString(row.name, `${path}.name`),
    autoApply: row.autoApply === undefined || row.autoApply === null ? false : asBoolean(row.autoApply, `${path}.autoApply`),
    grants: asArray(row.grants, `${path}.grants`).map((entry, index) => {
      const grant = asObject(entry, `${path}.grants[${index}]`);
      return { point: decodePolicyPoint(grant.point, `${path}.grants[${index}].point`), resource: asString(grant.resource, `${path}.grants[${index}].resource`), action: asString(grant.action, `${path}.grants[${index}].action`) };
    }),
  };
}

function decodeModuleManifest(value: unknown, path: string): ModuleManifest {
  const row = asObject(value, path);
  return {
    id: asString(row.id, `${path}.id`),
    commands: asArray(row.commands, `${path}.commands`).map((entry, index) => {
      const command = asObject(entry, `${path}.commands[${index}]`);
      return {
        kind: asString(command.kind, `${path}.commands[${index}].kind`),
        version: asNumber(command.version, `${path}.commands[${index}].version`),
        actorKind: asString(command.actorKind, `${path}.commands[${index}].actorKind`),
        offline: decodeOfflinePolicy(command.offline, `${path}.commands[${index}].offline`),
      };
    }),
    queries: asArray(row.queries, `${path}.queries`).map((entry, index) => {
      const query = asObject(entry, `${path}.queries[${index}]`);
      return { kind: asString(query.kind, `${path}.queries[${index}].kind`), version: asNumber(query.version, `${path}.queries[${index}].version`), projection: asString(query.projection, `${path}.queries[${index}].projection`) };
    }),
    projections: asArray(row.projections, `${path}.projections`).map((entry, index) => asString(entry, `${path}.projections[${index}]`)),
    policies: asArray(row.policies, `${path}.policies`).map((entry, index) => decodePolicyTemplate(entry, `${path}.policies[${index}]`)),
    actorKinds: asArray(row.actorKinds, `${path}.actorKinds`).map((entry, index) => asString(entry, `${path}.actorKinds[${index}]`)),
  };
}

/** 🏛️ Decode one [`ServerInstanceDefinition`], the body of `GET /instance`. */
export function decodeServerInstanceDefinition(value: unknown, path = "instance"): ServerInstanceDefinition {
  const row = asObject(value, path);
  return {
    id: asString(row.id, `${path}.id`),
    version: asString(row.version, `${path}.version`),
    modules: asArray(row.modules, `${path}.modules`).map((entry, index) => decodeModuleManifest(entry, `${path}.modules[${index}]`)),
  };
}

/** 🏛️ Encode one [`ServerInstanceDefinition`]. */
export function encodeServerInstanceDefinition(value: ServerInstanceDefinition): unknown {
  return {
    id: value.id,
    version: value.version,
    modules: value.modules.map((module) => ({
      id: module.id,
      commands: module.commands.map((command) => ({ kind: command.kind, version: command.version, actorKind: command.actorKind, offline: command.offline })),
      queries: module.queries.map((query) => ({ kind: query.kind, version: query.version, projection: query.projection })),
      projections: [...module.projections],
      policies: module.policies.map((template) => ({ name: template.name, autoApply: template.autoApply, grants: template.grants.map((grant) => ({ point: grant.point, resource: grant.resource, action: grant.action })) })),
      actorKinds: [...module.actorKinds],
    })),
  };
}

/** 🧾️ Decode one [`ErrorBody`]. */
export function decodeErrorBody(value: unknown, path = "error"): ErrorBody {
  const row = asObject(value, path);
  return { kind: asString(row.kind, `${path}.kind`), message: asString(row.message, `${path}.message`) };
}

/** 🧾️ Decode one [`BlobReceipt`]. */
export function decodeBlobReceipt(value: unknown, path = "blobReceipt"): BlobReceipt {
  const row = asObject(value, path);
  return { hash: asString(row.hash, `${path}.hash`), size: asNumber(row.size, `${path}.size`) };
}

/** 🧩️ Decode one [`AppInstall`]. */
export function decodeAppInstall(value: unknown, path = "appInstall"): AppInstall {
  const row = asObject(value, path);
  return { id: asString(row.id, `${path}.id`), manifest: row.manifest };
}
//#endregion 🔖️Codec

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

/** 📄️ Classify one document-socket message. */
export function decodeDocumentFrame(message: string | Uint8Array): DocumentFrame {
  return typeof message === "string" ? { kind: "error", message } : { kind: "engine", bytes: message };
}

/** 📄️ Non-identity join hints for a document socket. Actor and session are bound server-side. */
export interface DocumentJoin {
  readonly surface?: string;
  readonly resume?: string;
}
//#endregion 🔖️Sockets

//#region 🔖️Routes
/** 🛣️ One route of the gateway's own router, before any module adds its own. */
export interface ServerRoute {
  readonly method: string;
  readonly path: string;
}

/** 🛣️ The twelve routes `base_router` mounts, in its own order.
 *
 * Held against the Rust source itself by this module's suite rather than trusted: the fixture
 * document carries the same table, and the test reads `base_router` out of `📡️gateway/🦀️.rs` and
 * fails if either drifts. A typed client whose route table is a copy of a route table is a client
 * that silently 404s one release later. */
export const SERVER_ROUTES: readonly ServerRoute[] = [
  { method: "GET", path: "/instance" },
  { method: "POST", path: "/commands" },
  { method: "POST", path: "/queries" },
  { method: "POST", path: "/scopes/{scope}/ephemeral" },
  { method: "GET", path: "/actors/{tenant}/{kind}/{id}/events" },
  { method: "GET", path: "/actors/{tenant}/{kind}/{id}/events/ws" },
  { method: "GET", path: "/blobs/{hash}" },
  { method: "HEAD", path: "/blobs/{hash}" },
  { method: "PUT", path: "/blobs/{hash}" },
  { method: "GET", path: "/apps" },
  { method: "GET", path: "/apps/{app}/installs" },
  { method: "GET", path: "/apps/{app}" },
  { method: "GET", path: "/apps/{app}/{*rest}" },
  { method: "GET", path: "/scopes/{scope}/document/ws" },
];

/** 🎟️ The header a caller presents a capability proof in. */
export const CAPABILITY_HEADER = "x-semio-capability";

/** 🪪️ What a client presents to identify itself on every call. */
export interface ServerCredential {
  readonly bearer?: string;
  readonly capability?: CapabilityProof;
}

/** 🗂️ The durable lane key of one actor, as the gateway names it. */
export function streamLane(actor: ActorKey): string {
  return `stream:${actor.tenant}/${actor.kind}/${actor.id}`;
}

/** 📄️ The document lane key of one scope. */
export function documentLane(scope: Scope): string {
  return `document:${scope}`;
}

/** 💨️ The ephemeral lane key of one scope. */
export function ephemeralLane(scope: Scope): string {
  return `ephemeral:${scope}`;
}
//#endregion 🔖️Routes

//#region 🔖️Client
function encodePathSegment(value: string): string {
  return encodeURIComponent(value);
}

/** 🖥️ A typed client over the gateway's own routes.
 *
 * Every method names the route it calls and returns an owned, decoded value — a caller never sees a
 * raw JSON object, and a wire-shape drift surfaces as a [`WireError`] naming the field rather than
 * as `undefined` three layers later. Failures arrive as [`ServerCallError`] carrying the gateway's
 * own `kind` tag, which is the thing to branch on: the status code is a transport detail. */
export class ServerClient {
  private readonly transport: HttpTransport;
  private readonly credential: ServerCredential;

  constructor(transport: HttpTransport, credential: ServerCredential = {}) {
    this.transport = transport;
    this.credential = credential;
  }

  /** 🏛️ `GET /instance` — what this deployment declares itself to be. */
  async instance(): Promise<ServerInstanceDefinition> {
    return decodeServerInstanceDefinition(await this.json("GET", "/instance", {}));
  }

  /** 📨️ `POST /commands` — submit one command and read the authority's outcome. */
  async submitCommand(envelope: CommandEnvelope): Promise<CommandOutcome> {
    return decodeCommandOutcome(await this.json("POST", "/commands", {}, JSON.stringify(encodeCommandEnvelope(envelope))));
  }

  /** ❓️ `POST /queries` — answer one query from the read models. */
  async query(envelope: QueryEnvelope): Promise<QueryResult> {
    return decodeQueryResult(await this.json("POST", "/queries", {}, JSON.stringify(encodeQueryEnvelope(envelope))));
  }

  /** 💨️ `POST /scopes/{scope}/ephemeral` — publish one lossy frame; answers how many sockets got it. */
  async publishEphemeral(frame: EphemeralFrame): Promise<number> {
    const body = await this.json("POST", `/scopes/${encodePathSegment(frame.scope)}/ephemeral`, {}, JSON.stringify(encodeEphemeralFrame(frame)));
    return asNumber(body, "delivered");
  }

  /** 📜️ `GET /actors/{tenant}/{kind}/{id}/events` — one page of durable history after `since`. */
  async events(actor: ActorKey, since = 0): Promise<EventRecord[]> {
    const body = await this.json("GET", this.actorPath(actor, "/events"), { since: String(since) });
    return asArray(body, "events").map((entry, index) => decodeEventRecord(entry, `events[${index}]`));
  }

  /** 📦️ `GET /blobs/{hash}` — content-addressed bytes. */
  async blob(hash: string): Promise<Uint8Array> {
    const response = await this.transport.send({ method: "GET", path: `/blobs/${encodePathSegment(hash)}`, query: {}, headers: this.headers() });
    if (response.status >= 400) throw new ServerCallError(response.status, decodeErrorBody(JSON.parse(await response.text())));
    return response.bytes();
  }

  /** ❓️ `HEAD /blobs/{hash}` — the cheap half of an upload negotiation. */
  async hasBlob(hash: string): Promise<boolean> {
    const response = await this.transport.send({ method: "HEAD", path: `/blobs/${encodePathSegment(hash)}`, query: {}, headers: this.headers() });
    if (response.status === 200) return true;
    if (response.status === 404) return false;
    throw new ServerCallError(response.status, { kind: "unexpected", message: `HEAD /blobs answered ${response.status}` });
  }

  /** 💾️ `PUT /blobs/{hash}` — store bytes at a content address the server re-derives and verifies. */
  async putBlob(hash: string, bytes: Uint8Array): Promise<BlobReceipt> {
    const response = await this.transport.send({
      method: "PUT",
      path: `/blobs/${encodePathSegment(hash)}`,
      query: {},
      headers: { ...this.headers(), "content-type": "application/octet-stream" },
      body: bytes,
    });
    const text = await response.text();
    if (response.status >= 400) throw new ServerCallError(response.status, decodeErrorBody(JSON.parse(text)));
    return decodeBlobReceipt(JSON.parse(text));
  }

  /** 📋️ `GET /apps` — the names of every static app this instance hosts. */
  async apps(): Promise<string[]> {
    return asArray(await this.json("GET", "/apps", {}), "apps").map((entry, index) => asString(entry, `apps[${index}]`));
  }

  /** 🧩️ `GET /apps/{app}/installs` — the `install.json` entries under one app's root. */
  async appInstalls(app: string): Promise<AppInstall[]> {
    const body = await this.json("GET", `/apps/${encodePathSegment(app)}/installs`, {});
    return asArray(body, "installs").map((entry, index) => decodeAppInstall(entry, `installs[${index}]`));
  }

  /** 📡️ The websocket URL of one actor's durable lane, resuming after `since`. */
  eventStreamUrl(baseUrl: string, actor: ActorKey, since = 0): string {
    return `${socketRoot(baseUrl)}${this.actorPath(actor, "/events/ws")}?since=${encodeURIComponent(String(since))}`;
  }

  /** 📄️ The websocket URL of one scope's document lane. Actor identity is never a query parameter. */
  documentSocketUrl(baseUrl: string, scope: Scope, join: DocumentJoin = {}): string {
    const query = new URLSearchParams();
    if (join.surface !== undefined) query.set("surface", join.surface);
    if (join.resume !== undefined) query.set("resume", join.resume);
    const encoded = query.toString();
    const suffix = encoded.length > 0 ? `?${encoded}` : "";
    return `${socketRoot(baseUrl)}/scopes/${encodePathSegment(scope)}/document/ws${suffix}`;
  }

  private actorPath(actor: ActorKey, suffix: string): string {
    return `/actors/${encodePathSegment(actor.tenant)}/${encodePathSegment(actor.kind)}/${encodePathSegment(actor.id)}${suffix}`;
  }

  private headers(): Record<string, string> {
    const headers: Record<string, string> = {};
    if (this.credential.bearer !== undefined) headers.authorization = `Bearer ${this.credential.bearer}`;
    if (this.credential.capability !== undefined) headers[CAPABILITY_HEADER] = this.credential.capability;
    return headers;
  }

  private async json(method: HttpRequest["method"], path: string, query: Record<string, string>, body?: string): Promise<unknown> {
    const headers = body === undefined ? this.headers() : { ...this.headers(), "content-type": "application/json" };
    const response = await this.transport.send({ method, path, query, headers, body });
    const text = await response.text();
    const parsed: unknown = text === "" ? null : JSON.parse(text);
    if (response.status >= 400) throw new ServerCallError(response.status, decodeErrorBody(parsed));
    return parsed;
  }
}

/** 🔌️ The websocket origin of an http base URL. */
export function socketRoot(baseUrl: string): string {
  return baseUrl.replace(/\/+$/u, "").replace(/^http/u, "ws");
}
//#endregion 🔖️Client

//#region 🔖️Tests
if (import.meta.vitest) {
  const { registerServerWireTests } = await import("./🧪️tests/🔬️wire/🟦️.ts");
  await registerServerWireTests(import.meta.vitest, { SERVER_ROUTES, ServerClient, WireError, decodeActorKey, decodeCommandEnvelope, decodeCommandOutcome, decodeCommandReceipt, decodeDocumentFrame, decodeEphemeralFrame, decodeEventRecord, decodeEventStreamFrame, decodeFrontierSummary, decodeHybridLogicalClock, decodePrincipal, decodeQueryConsistency, decodeQueryEnvelope, decodeQueryResult, decodeRejection, decodeServerInstanceDefinition, decodeTraceContext, documentLane, encodeActorKey, encodeCommandEnvelope, encodeCommandOutcome, encodeCommandReceipt, encodeEphemeralFrame, encodeEventRecord, encodeFrontierSummary, encodeHybridLogicalClock, encodePrincipal, encodeQueryConsistency, encodeQueryEnvelope, encodeQueryResult, encodeRejection, encodeServerInstanceDefinition, encodeTraceContext, ephemeralLane, fetchTransport, socketRoot, streamLane }, { directory: import.meta.dir, url: import.meta.url });
}
//#endregion 🔖️Tests
