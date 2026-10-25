/** 🔤️ The codec of the server product's wire contract: the primitive readers and the decode and encode functions that convert every wire type between its JSON and its owned TypeScript value.
 *
 * @see ../🟦️.ts — the TypeScript twin of the server product's wire contract
 */
import type { ActorKey, CommandEnvelope, CommandOutcome, CommandReceipt, EphemeralFrame, EventRecord, FrontierSummary, HybridLogicalClock, ModuleManifest, Notice, OfflinePolicy, PolicyPoint, PolicyTemplate, PresenceEntry, PresenceFrame, Principal, QueryConsistency, QueryEnvelope, QueryResult, Rejection, ServerInstanceDefinition, TraceContext } from "../🧬️wire-types/🟦️.ts";
import { type AppInstall, type BlobReceipt, type ErrorBody, WireError } from "../🚚️transport/🟦️.ts";

//#region 🔖️Primitives
function fail(message: string, path: string): never {
  throw new WireError(message, path);
}

function asObject(value: unknown, path: string): Record<string, unknown> {
  if (value === null || typeof value !== "object" || Array.isArray(value)) fail("expected an object", path);
  return value as Record<string, unknown>;
}

export function asArray(value: unknown, path: string): unknown[] {
  if (!Array.isArray(value)) fail("expected an array", path);
  return value;
}

export function asString(value: unknown, path: string): string {
  if (typeof value !== "string") fail("expected a string", path);
  return value;
}

export function asNumber(value: unknown, path: string): number {
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

/** 🔑️ Encode one [`ActorKey`]. */
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

/** 👤️ Encode one [`Principal`]. */
export function encodePrincipal(value: Principal): unknown {
  return value.kind === "anonymous" ? { kind: "anonymous" } : { kind: value.kind, id: value.id };
}

/** 🕰️ Decode one [`HybridLogicalClock`]. */
export function decodeHybridLogicalClock(value: unknown, path = "hlc"): HybridLogicalClock {
  const row = asObject(value, path);
  return { millis: asNumber(row.millis, `${path}.millis`), counter: asNumber(row.counter, `${path}.counter`) };
}

/** ⏱️ Encode one [`HybridLogicalClock`]. */
export function encodeHybridLogicalClock(value: HybridLogicalClock): unknown {
  return { millis: value.millis, counter: value.counter };
}

/** 🔍️ Decode one [`TraceContext`] from its snake_case wire shape. */
export function decodeTraceContext(value: unknown, path = "trace"): TraceContext {
  const row = asObject(value, path);
  return { traceId: asString(row.trace_id, `${path}.trace_id`), spanId: asString(row.span_id, `${path}.span_id`) };
}

/** 🕵️ Encode one [`TraceContext`] into its snake_case wire shape. */
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

/** 🏁️ Encode one [`FrontierSummary`] into its snake_case wire shape. */
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

/** 📹️ Encode one [`EventRecord`]. */
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

/** 🎞️ Encode one [`EphemeralFrame`]. */
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

/** ✉️ Encode one [`CommandEnvelope`]. */
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

/** 🎫️ Encode one [`CommandReceipt`]. */
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

/** ⛔️ Encode one [`Rejection`]. */
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

/** 🏆️ Encode one [`CommandOutcome`]. */
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

/** ⚖️ Encode one [`QueryConsistency`]. */
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

/** 📩️ Encode one [`QueryEnvelope`]. */
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

/** 📈️ Encode one [`QueryResult`]. */
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

/** ☁️ Encode one [`ServerInstanceDefinition`]. */
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

/** ❌️ Decode one [`ErrorBody`]. */
export function decodeErrorBody(value: unknown, path = "error"): ErrorBody {
  const row = asObject(value, path);
  const body = { kind: asString(row.kind, `${path}.kind`), message: asString(row.message, `${path}.message`) };
  const waiting = row.retryAfterMs === undefined || row.retryAfterMs === null ? body : { ...body, retryAfterMs: asInterval(row.retryAfterMs, `${path}.retryAfterMs`) };
  return row.allowance === undefined || row.allowance === null ? waiting : { ...waiting, allowance: asString(row.allowance, `${path}.allowance`) };
}

/** ⚠️ Encode one [`ErrorBody`]. */
export function encodeErrorBody(value: ErrorBody): unknown {
  return { kind: value.kind, message: value.message, ...(value.retryAfterMs === undefined ? {} : { retryAfterMs: value.retryAfterMs }), ...(value.allowance === undefined ? {} : { allowance: value.allowance }) };
}

/** 🪪️ Decode one [`BlobReceipt`]. */
export function decodeBlobReceipt(value: unknown, path = "blobReceipt"): BlobReceipt {
  const row = asObject(value, path);
  return { hash: asString(row.hash, `${path}.hash`), size: asNumber(row.size, `${path}.size`) };
}

/** 🧩️ Decode one [`AppInstall`]. */
export function decodeAppInstall(value: unknown, path = "appInstall"): AppInstall {
  const row = asObject(value, path);
  return { id: asString(row.id, `${path}.id`), manifest: row.manifest };
}

function asJson(value: unknown, path: string): unknown {
  if (value === undefined) fail("expected a JSON value", path);
  return value;
}

function asColour(value: unknown, path: string): number {
  const colour = asNumber(value, path);
  if (!Number.isInteger(colour) || colour < 0 || colour > 255) fail("expected a palette slot in 0..=255", path);
  return colour;
}

function decodePresenceEntry(value: unknown, path: string): PresenceEntry {
  const row = asObject(value, path);
  return { session: asString(row.session, `${path}.session`), colour: asColour(row.colour, `${path}.colour`), surface: asString(row.surface, `${path}.surface`), state: asJson(row.state, `${path}.state`) };
}

function encodePresenceEntry(value: PresenceEntry): unknown {
  return { session: value.session, colour: value.colour, surface: value.surface, state: value.state };
}

/** 👥️ Decode one [`PresenceFrame`] (already parsed from the socket's text). */
export function decodePresenceFrame(value: unknown, path = "presenceFrame"): PresenceFrame {
  const row = asObject(value, path);
  const type = tag(row, "type", path);
  switch (type) {
    case "welcome":
      return { type, session: asString(row.session, `${path}.session`), colour: asColour(row.colour, `${path}.colour`), roster: asArray(row.roster, `${path}.roster`).map((entry, index) => decodePresenceEntry(entry, `${path}.roster[${index}]`)) };
    case "state":
      return { type, state: asJson(row.state, `${path}.state`) };
    case "batch":
      return { type, entries: asArray(row.entries, `${path}.entries`).map((entry, index) => decodePresenceEntry(entry, `${path}.entries[${index}]`)), left: asArray(row.left, `${path}.left`).map((entry, index) => asString(entry, `${path}.left[${index}]`)) };
    case "refused":
      return { type, reason: asString(row.reason, `${path}.reason`) };
    case "watch":
      return { type, scopes: asArray(row.scopes, `${path}.scopes`).map((scope, index) => asString(scope, `${path}.scopes[${index}]`)), intervalMs: asInterval(row.intervalMs, `${path}.intervalMs`) };
    case "watched": {
      const snapshot = row.snapshot === undefined ? false : asBoolean(row.snapshot, `${path}.snapshot`);
      const scope = asString(row.scope, `${path}.scope`);
      const entries = asArray(row.entries, `${path}.entries`).map((entry, index) => decodePresenceEntry(entry, `${path}.entries[${index}]`));
      const left = asArray(row.left, `${path}.left`).map((entry, index) => asString(entry, `${path}.left[${index}]`));
      return snapshot ? { type, scope, entries, left, snapshot } : { type, scope, entries, left };
    }
    default:
      return fail(`unknown presence frame type ${JSON.stringify(type)}`, path);
  }
}

function asInterval(value: unknown, path: string): number {
  const interval = asNumber(value, path);
  if (!Number.isSafeInteger(interval) || interval < 0) fail("expected a whole number of milliseconds", path);
  return interval;
}

/** 👁️ Encode one [`PresenceFrame`] into its wire JSON (stringify it to send). */
export function encodePresenceFrame(value: PresenceFrame): unknown {
  switch (value.type) {
    case "welcome":
      return { type: value.type, session: value.session, colour: value.colour, roster: value.roster.map(encodePresenceEntry) };
    case "state":
      return { type: value.type, state: value.state };
    case "batch":
      return { type: value.type, entries: value.entries.map(encodePresenceEntry), left: [...value.left] };
    case "refused":
      return { type: value.type, reason: value.reason };
    case "watch":
      return { type: value.type, scopes: [...value.scopes], intervalMs: value.intervalMs };
    case "watched":
      return { type: value.type, scope: value.scope, entries: value.entries.map(encodePresenceEntry), left: [...value.left], ...(value.snapshot ? { snapshot: true } : {}) };
  }
}
//#endregion 🔖️Codec
