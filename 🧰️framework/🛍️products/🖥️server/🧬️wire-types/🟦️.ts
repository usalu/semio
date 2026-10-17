/** 🧬️ The wire types of the server product: identities, commands and their outcomes, queries, event and ephemeral frames, presence, policy, the module manifest and the instance definition.
 *
 * @see ../🟦️.ts — the TypeScript twin of the server product's wire contract
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
//#region 🔖️Presence
/** 🧍️ One session of a presence room as every member sees it: the server-generated session id, its palette slot, the
 * surface it joined from and its latest shared state — the instance's opaque JSON, `null` until it sent one. */
export interface PresenceEntry {
  readonly session: string;
  readonly colour: number;
  readonly surface: string;
  readonly state: unknown;
}

/** 👥️ One text frame of the presence socket (`semio.presence.v1`). `welcome` (the joining session, its colour and the
 * roster, led by the session itself) and a `batch` at most once per tick (sessions whose state changed since the
 * socket's last frame, sessions that left) travel server → client — a second `welcome` replaces the roster wholesale;
 * `state` (latest wins) travels client → server; `refused` answers a frame the server dropped while the socket stays
 * open.
 *
 * Server frames are bounded: a roster larger than one frame may carry begins in the `welcome` and continues in the
 * `batch` frames that follow, and a burst of changes larger than a frame is spread over the next ticks, every changed
 * session once before any twice. Apply every frame as it comes — `welcome` replaces, `batch` removes `left` and upserts
 * `entries` — and the whole room is held once the frames have caught up.
 *
 * Watching is read-only presence in other rooms: `watch` (client → server) replaces the watched scopes (≤ 16; an empty
 * list stops watching) and the interval the client wants their changes at (clamped to the server's tick at least);
 * `watched` (server → client) carries changes of one scope since the socket was last sent that scope, or — flagged
 * `snapshot` — the beginning of its roster, sent when the scope becomes watched or the watcher fell too far behind,
 * replacing what the client held for it. `watched` frames are bounded the same way, over all watched scopes per
 * interval. */
export type PresenceFrame =
  | { readonly type: "welcome"; readonly session: string; readonly colour: number; readonly roster: readonly PresenceEntry[] }
  | { readonly type: "state"; readonly state: unknown }
  | { readonly type: "batch"; readonly entries: readonly PresenceEntry[]; readonly left: readonly string[] }
  | { readonly type: "refused"; readonly reason: string }
  | { readonly type: "watch"; readonly scopes: readonly Scope[]; readonly intervalMs: number }
  | { readonly type: "watched"; readonly scope: Scope; readonly entries: readonly PresenceEntry[]; readonly left: readonly string[]; readonly snapshot?: boolean };

/** 🔌️ The websocket subprotocol the presence socket speaks. */
export const PRESENCE_PROTOCOL = "semio.presence.v1";

//#endregion 🔖️Presence
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
