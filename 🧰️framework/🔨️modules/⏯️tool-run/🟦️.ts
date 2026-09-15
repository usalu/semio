/**
 * ⏯️ TypeScript mirror of the domain-neutral tool run contract (`🦀️.rs`): identity, the pure
 * lifecycle reducer, step ring, columnar trace pages with their resident store, trace delta and tick
 * codecs over the pack record body wire, framework actions, chords and EN/DE labels. u64 values are
 * `bigint`. Schema of record: `🧬️schema/🔣️.json`; contract:
 * `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️13/INTERACTIVE-TOOLS-VISIBLE-PROCESS/📋️tool-run-contract.md` §2, §3.1, §3.2.
 */

//#region 🔖️Limits
export const TOOL_RUN_STEP_RING_CAPACITY = 64;
export const TOOL_RUN_STEP_ARGS_MAX = 4;
export const TOOL_RUN_COUNTERS_MAX = 8;
export const TOOL_RUN_PROVISIONAL_OPS_MAX = 65_536;
export const TOOL_RUN_TRACE_RESIDENT_RECORDS = 1_048_576;
export const TOOL_RUN_TRACE_PAGE_OPS_MAX = 4_096;
export const TOOL_RUN_TRACE_PAGE_BYTES_MAX = 262_144;
export const TOOL_RUN_TICK_BYTES_MAX = 262_144;
export const TOOL_RUN_TRACE_LOG_COMPACT_FLOOR = 4_096;
export const TOOL_RUN_STATUS_ANNOUNCE_INTERVAL_MS = 2_000;
export const TOOL_RUN_RESERVED_REASON_FLOOR = 0xff00;
export const TOOL_RUN_REASON_REBASING = 0xff00;
export const TOOL_RUN_REASON_CONFLICT = 0xff01;
export const TOOL_RUN_REASON_TRACE_TRUNCATED = 0xff02;
export const TOOL_RUN_REASON_PROVISIONAL_CAP = 0xff03;
export const TOOL_RUN_GROUP_ID_PREFIX = "toolRun:";
export const TOOL_RUN_TRACE_PAGE_OVERHEAD_BYTES = 64;
//#endregion 🔖️Limits

//#region 🔖️Identity
/** 🪪️ Per-instance run id; `run` is monotone and never reused. */
export type ToolRunId = { readonly appInstanceId: number; readonly run: bigint };

/** 🧿️ Run id plus staleness generation and the 32-byte committed base revision. */
export type ToolRunIdentity = { readonly id: ToolRunId; readonly generation: number; readonly baseRevision: Uint8Array };

/** 🏷️ The `group_id` of the single `Edit` a finalized run publishes. */
export function toolRunGroupId(id: ToolRunId): string {
  return `${TOOL_RUN_GROUP_ID_PREFIX}${id.run}`;
}

/** 🥇️ Lexicographic `(run, generation, sequence)` comparison; positive when `a` is newer. */
export function compareToolRunFreshness(a: { run: bigint; generation: number; sequence: bigint }, b: { run: bigint; generation: number; sequence: bigint }): number {
  if (a.run !== b.run) return a.run > b.run ? 1 : -1;
  if (a.generation !== b.generation) return a.generation > b.generation ? 1 : -1;
  return a.sequence === b.sequence ? 0 : a.sequence > b.sequence ? 1 : -1;
}
//#endregion 🔖️Identity

//#region 🔖️Lifecycle
export const TOOL_RUN_STATES = ["starting", "running", "paused", "complete", "finalizing", "finalized", "aborting", "aborted", "faulted"] as const;
export type ToolRunState = (typeof TOOL_RUN_STATES)[number];

/** 🏁️ `finalized`, `aborted` and `faulted`. */
export function isToolRunTerminal(state: ToolRunState): boolean {
  return state === "finalized" || state === "aborted" || state === "faulted";
}

export type ToolRunSlot = { readonly run: bigint; readonly generation: number; readonly state: ToolRunState };

export type ToolRunEvent =
  | { readonly type: "start"; readonly run: bigint }
  | { readonly type: "jobAdmitted" | "pause" | "resume" | "step" | "jobComplete" | "jobFault" | "finalize" | "publicationComplete" | "revalidationConflicts" | "storeRejected" | "abortComplete"; readonly run: bigint; readonly generation: number }
  | { readonly type: "abort"; readonly run: bigint; readonly generation: number; readonly publishing: boolean }
  | { readonly type: "settingsChanged" | "baseChanged" | "dismiss"; readonly run: bigint }
  | { readonly type: "closed" };

export const TOOL_RUN_EVENT_KEYS = ["start", "jobAdmitted", "pause", "resume", "step", "jobComplete", "jobFault", "settingsChanged", "baseChanged", "finalize", "publicationComplete", "revalidationConflicts", "storeRejected", "abort", "abortWhilePublishing", "abortComplete", "dismiss", "closed"] as const;
export type ToolRunEventKey = (typeof TOOL_RUN_EVENT_KEYS)[number];

/** 🗝️ Lifecycle-law matrix column of an event. */
export function toolRunEventKey(event: ToolRunEvent): ToolRunEventKey {
  return event.type === "abort" && event.publishing ? "abortWhilePublishing" : event.type;
}

export const TOOL_RUN_EFFECTS = ["spawnJob", "schedule", "stopScheduling", "driveOneUnit", "holdResult", "reconfigure", "refold", "beginFinalize", "releaseProvisional", "retractConflicts", "keepProvisional", "closeJob", "cancelBatch", "retireProvisional", "discardProvisional", "clearTrace", "retireAll"] as const;
export type ToolRunEffect = (typeof TOOL_RUN_EFFECTS)[number];

/** 💾️ True only for the one transition that changes the store generation. */
export function toolRunEffectCommits(effect: ToolRunEffect): boolean {
  return effect === "releaseProvisional";
}

export type ToolRunRejection = "toolRun.stale" | "toolRun.busy" | "toolRun.illegal";
export type ToolRunTransition = { readonly slot: ToolRunSlot | null; readonly effect: ToolRunEffect };
export type ToolRunApplyResult = { readonly ok: true; readonly transition: ToolRunTransition } | { readonly ok: false; readonly rejection: ToolRunRejection };

type ToolRunRow = readonly [ToolRunState | null, ToolRunEffect, boolean];

const TOOL_RUN_TABLE: Readonly<Partial<Record<ToolRunState, Partial<Record<ToolRunEventKey, ToolRunRow | ToolRunRejection>>>>> = {
  starting: { jobAdmitted: ["running", "schedule", false], abort: ["aborting", "closeJob", false], abortWhilePublishing: ["aborting", "closeJob", false], jobFault: ["faulted", "discardProvisional", false] },
  running: { pause: ["paused", "stopScheduling", false], jobComplete: ["complete", "holdResult", false], settingsChanged: ["running", "reconfigure", true], baseChanged: ["running", "refold", true], abort: ["aborting", "closeJob", false], abortWhilePublishing: ["aborting", "closeJob", false], jobFault: ["faulted", "discardProvisional", false] },
  paused: { resume: ["running", "schedule", false], step: ["paused", "driveOneUnit", false], jobComplete: ["complete", "holdResult", false], settingsChanged: ["paused", "reconfigure", true], baseChanged: ["paused", "refold", true], abort: ["aborting", "closeJob", false], abortWhilePublishing: ["aborting", "closeJob", false], jobFault: ["faulted", "discardProvisional", false] },
  complete: { settingsChanged: ["running", "reconfigure", true], baseChanged: ["complete", "refold", true], finalize: ["finalizing", "beginFinalize", false], abort: ["aborting", "closeJob", false], abortWhilePublishing: ["aborting", "closeJob", false] },
  finalizing: { publicationComplete: ["finalized", "releaseProvisional", false], revalidationConflicts: ["complete", "retractConflicts", true], storeRejected: ["complete", "keepProvisional", true], abort: ["aborting", "cancelBatch", false], abortWhilePublishing: "toolRun.stale" },
  aborting: { abortComplete: ["aborted", "retireProvisional", false] },
  finalized: { dismiss: [null, "clearTrace", false] },
  aborted: { dismiss: [null, "clearTrace", false] },
  faulted: { dismiss: [null, "clearTrace", false] },
};

/** 🛣️ What a run occupies on its document instance for its local actor (§2.2 invariant 5). */
export type ToolRunLane = { readonly mutating: boolean; readonly toolId: string; readonly windowId?: string };

/** 🤝️ Two mutating runs always share a lane; two read-only runs share it on the same tool and window; a mutating and a read-only run never do. */
export function toolRunLanesShare(left: ToolRunLane, right: ToolRunLane): boolean {
  if (left.mutating !== right.mutating) return false;
  return left.mutating || (left.toolId === right.toolId && left.windowId === right.windowId);
}

/** 🚦️ `toolRun.busy` while a run on the same lane is non-terminal, else the indices of the terminal runs the start replaces. */
export function toolRunAdmit(lane: ToolRunLane, live: readonly (readonly [ToolRunLane, ToolRunState])[]): { readonly ok: true; readonly replaces: readonly number[] } | { readonly ok: false; readonly rejection: ToolRunRejection } {
  const shared = live.flatMap(([other], index) => (toolRunLanesShare(other, lane) ? [index] : []));
  if (shared.some((index) => !isToolRunTerminal(live[index]![1]))) return { ok: false, rejection: "toolRun.busy" };
  return { ok: true, replaces: shared };
}

/** ⚙️ The pure §2.2 reducer of one run slot; {@link toolRunAdmit} decides which slot a start may take. */
export const ToolRunMachine = {
  /** ⚖️ Staleness guard first, then the transition table. */
  apply(slot: ToolRunSlot | null, event: ToolRunEvent): ToolRunApplyResult {
    if (event.type === "start") {
      if (slot !== null && !isToolRunTerminal(slot.state)) return { ok: false, rejection: "toolRun.busy" };
      if (slot !== null && event.run <= slot.run) return { ok: false, rejection: "toolRun.illegal" };
      return { ok: true, transition: { slot: { run: event.run, generation: 0, state: "starting" }, effect: "spawnJob" } };
    }
    if (event.type === "closed") return { ok: true, transition: { slot: null, effect: "retireAll" } };
    if (slot === null || event.run !== slot.run || ("generation" in event && event.generation !== slot.generation)) return { ok: false, rejection: "toolRun.stale" };
    const row = TOOL_RUN_TABLE[slot.state]?.[toolRunEventKey(event)];
    if (row === undefined) return { ok: false, rejection: "toolRun.illegal" };
    if (typeof row === "string") return { ok: false, rejection: row };
    const [to, effect, increment] = row;
    return { ok: true, transition: { slot: to === null ? null : { run: slot.run, generation: increment ? (slot.generation + 1) >>> 0 : slot.generation, state: to }, effect } };
  },
};
//#endregion 🔖️Lifecycle

//#region 🔖️Progress
export const TOOL_RUN_VERDICTS = ["testing", "success", "warning", "danger"] as const;
export type ToolRunVerdict = (typeof TOOL_RUN_VERDICTS)[number];

/** 🗑️ `warning` and `danger` records are the only ones residency may evict. */
export function isToolRunVerdictRejected(verdict: ToolRunVerdict): boolean {
  return verdict === "warning" || verdict === "danger";
}

export const TOOL_RUN_STEP_KINDS = ["info", "success", "warning", "danger"] as const;
export type ToolRunStepKind = (typeof TOOL_RUN_STEP_KINDS)[number];

export type ToolRunStepArg = { readonly unsigned: bigint } | { readonly float: number };

export type ToolRunStep = {
  readonly sequence: bigint;
  readonly kind: ToolRunStepKind;
  readonly stage: number;
  readonly reason: number;
  readonly subject?: bigint;
  readonly repeat: number;
  readonly args: readonly ToolRunStepArg[];
};

function toolRunStepArgEquals(a: ToolRunStepArg, b: ToolRunStepArg): boolean {
  if ("unsigned" in a) return "unsigned" in b && a.unsigned === b.unsigned;
  return "float" in b && Object.is(a.float, b.float);
}

/** 🧲️ Identical apart from `sequence` and `repeat`, so consecutive pushes coalesce. */
export function toolRunStepsCoalesce(a: ToolRunStep, b: ToolRunStep): boolean {
  return a.kind === b.kind && a.stage === b.stage && a.reason === b.reason && a.subject === b.subject && a.args.length === b.args.length && a.args.every((arg, index) => toolRunStepArgEquals(arg, b.args[index]!));
}

/** 💍️ Newest ≤ 64 steps, overwrite-oldest, identical consecutive steps coalesced. */
export class ToolRunStepRing {
  #steps: ToolRunStep[] = [];

  static fromSteps(steps: readonly ToolRunStep[]): ToolRunStepRing {
    if (steps.length > TOOL_RUN_STEP_RING_CAPACITY) throw new ToolRunCodecError("limit", "step ring capacity");
    const ring = new ToolRunStepRing();
    ring.#steps = [...steps];
    return ring;
  }

  push(step: ToolRunStep): void {
    const newest = this.#steps[this.#steps.length - 1];
    if (newest !== undefined && toolRunStepsCoalesce(newest, step)) {
      this.#steps[this.#steps.length - 1] = { ...newest, sequence: step.sequence, repeat: Math.min(newest.repeat + step.repeat, 0xffff_ffff) };
      return;
    }
    if (this.#steps.length === TOOL_RUN_STEP_RING_CAPACITY) this.#steps.shift();
    this.#steps.push(step);
  }

  get length(): number {
    return this.#steps.length;
  }

  steps(): readonly ToolRunStep[] {
    return this.#steps;
  }

  oldest(): ToolRunStep | undefined {
    return this.#steps[0];
  }

  newest(): ToolRunStep | undefined {
    return this.#steps[this.#steps.length - 1];
  }
}

export type ToolRunCounter = { readonly counter: number; readonly value: bigint };

export type ToolRunProgress = {
  readonly identity: ToolRunIdentity;
  readonly sequence: bigint;
  readonly state: ToolRunState;
  readonly stage: number;
  readonly completed: bigint;
  readonly total?: bigint;
  readonly counters: readonly ToolRunCounter[];
  readonly unitsPerSecond: number;
  readonly conflicts: number;
  readonly steps: readonly ToolRunStep[];
};

/** ➗️ `completed / total`; `undefined` while indeterminate. */
export function toolRunProgressFraction(progress: ToolRunProgress): number | undefined {
  return progress.total === undefined || progress.total === 0n ? undefined : Number(progress.completed) / Number(progress.total);
}
//#endregion 🔖️Progress

//#region 🔖️Trace
export type ToolRunTraceSubject =
  | { readonly kind: "instance3d"; readonly mesh: number; readonly position: readonly [number, number, number]; readonly rotation: readonly [number, number, number, number]; readonly scale: number }
  | { readonly kind: "placement2d"; readonly shape: number; readonly position: readonly [number, number]; readonly rotation: number }
  | { readonly kind: "entity"; readonly entity: bigint };

export type ToolRunTraceOp = { readonly op: "upsert"; readonly key: bigint; readonly verdict: ToolRunVerdict; readonly reason: number; readonly subject: ToolRunTraceSubject } | { readonly op: "retire"; readonly key: bigint } | { readonly op: "clear" };

export type ToolRunTracePage = { readonly identity: ToolRunIdentity; readonly page: number; readonly ops: readonly ToolRunTraceOp[] };
export type ToolRunTraceDelta = { readonly identity: ToolRunIdentity; readonly clear: boolean; readonly next: number; readonly pages: readonly ToolRunTracePage[] };
export type ToolRunTraceCursor = { readonly run: bigint; readonly generation: number; readonly page: number };
export type ToolRunTraceRecord = { readonly verdict: ToolRunVerdict; readonly reason: number; readonly subject: ToolRunTraceSubject; readonly stamp: number };
export type ToolRunTraceApply = { readonly logged: number; readonly evicted: number; readonly overflowed: number; readonly compacted: boolean };

/** 📏️ Deterministic column-byte estimate of one op. */
export function toolRunTraceOpWireBytes(op: ToolRunTraceOp): number {
  if (op.op === "clear") return 1;
  if (op.op === "retire") return 9;
  return 13 + (op.subject.kind === "instance3d" ? 36 : op.subject.kind === "placement2d" ? 16 : 8);
}

/** 📐️ Delivery budget estimate of a page. */
export function toolRunTracePageWireBytes(ops: readonly ToolRunTraceOp[]): number {
  let bytes = TOOL_RUN_TRACE_PAGE_OVERHEAD_BYTES;
  for (const op of ops) bytes += toolRunTraceOpWireBytes(op);
  return bytes;
}

type ToolRunTraceLogPage = { readonly ops: readonly ToolRunTraceOp[]; readonly bytes: number };

/** 🗄️ Resident trace records plus the page log windows read through cursors (§3.2). */
export class ToolRunTraceStore {
  #identity: ToolRunIdentity;
  readonly #capacity: number;
  readonly #compactFloor: number;
  #records = new Map<bigint, ToolRunTraceRecord>();
  #rejected: [bigint, number][] = [];
  #rejectedHead = 0;
  #nextStamp = 0;
  #log: ToolRunTraceLogPage[] = [];
  #logBase = 0;
  #nextPage = 0;
  #loggedOps = 0;

  constructor(identity: ToolRunIdentity, capacity = TOOL_RUN_TRACE_RESIDENT_RECORDS, compactFloor = TOOL_RUN_TRACE_LOG_COMPACT_FLOOR) {
    this.#identity = identity;
    this.#capacity = Math.max(1, capacity);
    this.#compactFloor = Math.max(1, compactFloor);
  }

  get identity(): ToolRunIdentity {
    return this.#identity;
  }

  get size(): number {
    return this.#records.size;
  }

  get logBase(): number {
    return this.#logBase;
  }

  get nextPage(): number {
    return this.#nextPage;
  }

  /** 🔗️ Adopts a new generation (records kept) or a new run (everything dropped). */
  rebind(identity: ToolRunIdentity): void {
    if (identity.id.run !== this.#identity.id.run || identity.id.appInstanceId !== this.#identity.id.appInstanceId) {
      this.#records.clear();
      this.#rejected = [];
      this.#rejectedHead = 0;
      this.#nextStamp = 0;
      this.#log = [];
      this.#logBase = 0;
      this.#nextPage = 0;
      this.#loggedOps = 0;
    }
    this.#identity = identity;
  }

  record(key: bigint): ToolRunTraceRecord | undefined {
    return this.#records.get(key);
  }

  records(): IterableIterator<[bigint, ToolRunTraceRecord]> {
    return this.#records.entries();
  }

  logPage(page: number): readonly ToolRunTraceOp[] | undefined {
    return this.#log[page - this.#logBase]?.ops;
  }

  /** 📥️ Applies a job page; pages of another run or generation are stale. */
  applyPage(page: ToolRunTracePage): ToolRunTraceApply | "toolRun.stale" {
    if (page.identity.id.run !== this.#identity.id.run || page.identity.id.appInstanceId !== this.#identity.id.appInstanceId || page.identity.generation !== this.#identity.generation) return "toolRun.stale";
    return this.applyOps(page.ops);
  }

  /** 📨️ Applies ops with residency eviction and logs what renderers must replay. */
  applyOps(ops: readonly ToolRunTraceOp[]): ToolRunTraceApply {
    let evicted = 0;
    let overflowed = 0;
    const logged: ToolRunTraceOp[] = [];
    for (const op of ops) {
      if (op.op === "clear") {
        this.#records.clear();
        this.#rejected = [];
        this.#rejectedHead = 0;
        logged.push(op);
      } else if (op.op === "retire") {
        if (this.#records.delete(op.key)) logged.push(op);
      } else {
        if (!this.#records.has(op.key) && this.#records.size >= this.#capacity) {
          const victim = this.#evictOldestRejected();
          if (victim !== undefined) {
            logged.push({ op: "retire", key: victim });
            evicted += 1;
          } else if (isToolRunVerdictRejected(op.verdict)) {
            evicted += 1;
            continue;
          } else {
            overflowed += 1;
            continue;
          }
        }
        const stamp = this.#nextStamp++;
        this.#records.set(op.key, { verdict: op.verdict, reason: op.reason, subject: op.subject, stamp });
        if (isToolRunVerdictRejected(op.verdict)) this.#rejected.push([op.key, stamp]);
        logged.push(op);
      }
    }
    const pages = this.#logOps(logged);
    if (this.#rejected.length - this.#rejectedHead > 2 * this.#records.size + TOOL_RUN_STEP_RING_CAPACITY) this.#rebuildRejected();
    let compacted = false;
    if (this.#loggedOps > 2 * Math.max(this.#records.size, this.#compactFloor)) {
      this.#compact();
      compacted = true;
    }
    return { logged: pages, evicted, overflowed, compacted };
  }

  /** 📬️ Pages after `cursor` within `byteBudget` (always at least one pending page). */
  deltaAfter(cursor: ToolRunTraceCursor | null, byteBudget: number): ToolRunTraceDelta {
    const resend = cursor === null || cursor.run !== this.#identity.id.run || cursor.generation !== this.#identity.generation || cursor.page < this.#logBase || cursor.page > this.#nextPage;
    const start = resend ? this.#logBase : cursor.page;
    const pages: ToolRunTracePage[] = [];
    let bytes = 0;
    for (let index = start - this.#logBase; index < this.#log.length; index += 1) {
      const logged = this.#log[index]!;
      if (pages.length > 0 && bytes + logged.bytes > byteBudget) break;
      bytes += logged.bytes;
      pages.push({ identity: this.#identity, page: this.#logBase + index, ops: logged.ops });
    }
    return { identity: this.#identity, clear: resend, next: start + pages.length, pages };
  }

  #logOps(ops: readonly ToolRunTraceOp[]): number {
    let pages = 0;
    for (let offset = 0; offset < ops.length; offset += TOOL_RUN_TRACE_PAGE_OPS_MAX) {
      const chunk = ops.slice(offset, offset + TOOL_RUN_TRACE_PAGE_OPS_MAX);
      this.#loggedOps += chunk.length;
      this.#log.push({ ops: chunk, bytes: toolRunTracePageWireBytes(chunk) });
      this.#nextPage += 1;
      pages += 1;
    }
    return pages;
  }

  #evictOldestRejected(): bigint | undefined {
    while (this.#rejectedHead < this.#rejected.length) {
      const [key, stamp] = this.#rejected[this.#rejectedHead++]!;
      const record = this.#records.get(key);
      if (record !== undefined && record.stamp === stamp && isToolRunVerdictRejected(record.verdict)) {
        this.#records.delete(key);
        return key;
      }
    }
    return undefined;
  }

  #liveByStamp(): [bigint, ToolRunTraceRecord][] {
    return [...this.#records.entries()].sort((a, b) => a[1].stamp - b[1].stamp);
  }

  #rebuildRejected(): void {
    this.#rejected = this.#liveByStamp()
      .filter(([, record]) => isToolRunVerdictRejected(record.verdict))
      .map(([key, record]) => [key, record.stamp]);
    this.#rejectedHead = 0;
  }

  #compact(): void {
    const live = this.#liveByStamp();
    const snapshot: ToolRunTraceOp[] = [{ op: "clear" }, ...live.map(([key, record]): ToolRunTraceOp => ({ op: "upsert", key, verdict: record.verdict, reason: record.reason, subject: record.subject }))];
    this.#rejected = live.filter(([, record]) => isToolRunVerdictRejected(record.verdict)).map(([key, record]) => [key, record.stamp]);
    this.#rejectedHead = 0;
    this.#log = [];
    this.#loggedOps = 0;
    this.#logBase = this.#nextPage;
    this.#logOps(snapshot);
  }
}
//#endregion 🔖️Trace

//#region 🔖️Tick
export type ToolRunTick = {
  readonly identity: ToolRunIdentity;
  readonly sequence: bigint;
  readonly progress?: ToolRunProgress;
  readonly steps: readonly ToolRunStep[];
  readonly trace: readonly ToolRunTracePage[];
  readonly appendOps: readonly Uint8Array[];
  readonly appendEntities: readonly bigint[];
  readonly retractTo?: number;
  /** 📨️ Opaque plugin bytes the run's windows read back — the newest tick carrying one wins. */
  readonly payload?: Uint8Array;
};
//#endregion 🔖️Tick

//#region 🔖️Codec
/** 🧨️ Encode or decode failure of a tool run wire value. */
export class ToolRunCodecError extends Error {
  constructor(
    readonly kind: "pack" | "malformed" | "limit",
    readonly what: string,
  ) {
    super(`toolRun.codec.${kind}: ${what}`);
  }
}

const TAG_ABSENT = 0x00;
const TAG_FALSE = 0x01;
const TAG_TRUE = 0x02;
const TAG_UINT = 0x04;
const TAG_F64 = 0x05;
const TAG_BYTES = 0x08;
const TAG_LIST = 0x0c;
const TAG_RECORD = 0x0d;

type PackValue = { readonly t: "uint"; readonly v: bigint } | { readonly t: "bool"; readonly v: boolean } | { readonly t: "f64"; readonly v: number } | { readonly t: "bytes"; readonly v: Uint8Array } | { readonly t: "record"; readonly v: PackRecord } | { readonly t: "list"; readonly v: readonly PackValue[] };
type PackRecord = Map<number, PackValue>;

class PackBuffer {
  bytes = new Uint8Array(256);
  length = 0;

  reserve(extra: number): void {
    if (this.length + extra <= this.bytes.length) return;
    const next = new Uint8Array(Math.max(this.bytes.length * 2, this.length + extra));
    next.set(this.bytes.subarray(0, this.length));
    this.bytes = next;
  }

  u8(value: number): void {
    this.reserve(1);
    this.bytes[this.length++] = value;
  }

  varint(value: bigint | number): void {
    let rest = BigInt(value);
    this.reserve(10);
    while (rest >= 0x80n) {
      this.bytes[this.length++] = Number(rest & 0x7fn) | 0x80;
      rest >>= 7n;
    }
    this.bytes[this.length++] = Number(rest);
  }

  raw(bytes: Uint8Array): void {
    this.reserve(bytes.length);
    this.bytes.set(bytes, this.length);
    this.length += bytes.length;
  }

  finish(): Uint8Array {
    return this.bytes.slice(0, this.length);
  }
}

function writePackValue(out: PackBuffer, value: PackValue): void {
  switch (value.t) {
    case "uint":
      out.u8(TAG_UINT);
      out.varint(value.v);
      return;
    case "bool":
      out.u8(value.v ? TAG_TRUE : TAG_FALSE);
      return;
    case "f64": {
      out.u8(TAG_F64);
      const cell = new DataView(new ArrayBuffer(8));
      if (Number.isNaN(value.v)) cell.setBigUint64(0, 0x7ff8_0000_0000_0000n, true);
      else cell.setFloat64(0, value.v, true);
      out.raw(new Uint8Array(cell.buffer));
      return;
    }
    case "bytes":
      out.u8(TAG_BYTES);
      out.varint(value.v.length);
      out.raw(value.v);
      return;
    case "record":
      out.u8(TAG_RECORD);
      writePackFields(out, value.v);
      return;
    case "list":
      out.u8(TAG_LIST);
      out.varint(value.v.length);
      for (const item of value.v) writePackValue(out, item);
  }
}

function writePackFields(out: PackBuffer, record: PackRecord): void {
  const ids = [...record.keys()].sort((a, b) => a - b);
  out.varint(ids.length);
  for (const id of ids) {
    out.varint(id);
    writePackValue(out, record.get(id)!);
  }
}

function encodePackBody(record: PackRecord): Uint8Array {
  const out = new PackBuffer();
  out.varint(0);
  writePackFields(out, record);
  return out.finish();
}

class PackReader {
  offset = 0;
  constructor(readonly bytes: Uint8Array) {}

  u8(): number {
    const byte = this.bytes[this.offset];
    if (byte === undefined) throw new ToolRunCodecError("pack", "truncated");
    this.offset += 1;
    return byte;
  }

  varint(): bigint {
    let result = 0n;
    for (let shift = 0n; shift < 70n; shift += 7n) {
      const byte = this.u8();
      result |= BigInt(byte & 0x7f) << shift;
      if ((byte & 0x80) === 0) {
        if (result > 0xffff_ffff_ffff_ffffn) throw new ToolRunCodecError("pack", "varint exceeds u64");
        return result;
      }
    }
    throw new ToolRunCodecError("pack", "varint exceeds u64");
  }

  length(): number {
    const value = this.varint();
    if (value > BigInt(this.bytes.length - this.offset)) throw new ToolRunCodecError("pack", "declared length exceeds remaining bytes");
    return Number(value);
  }

  raw(length: number): Uint8Array {
    const slice = this.bytes.slice(this.offset, this.offset + length);
    this.offset += length;
    return slice;
  }
}

function readPackValue(reader: PackReader, depth: number): PackValue | undefined {
  if (depth > 64) throw new ToolRunCodecError("pack", "max depth exceeded");
  const tag = reader.u8();
  switch (tag) {
    case TAG_ABSENT:
      return undefined;
    case TAG_FALSE:
      return { t: "bool", v: false };
    case TAG_TRUE:
      return { t: "bool", v: true };
    case TAG_UINT:
      return { t: "uint", v: reader.varint() };
    case TAG_F64:
      return { t: "f64", v: new DataView(reader.raw(8).buffer).getFloat64(0, true) };
    case TAG_BYTES:
      return { t: "bytes", v: reader.raw(reader.length()) };
    case TAG_RECORD:
      return { t: "record", v: readPackFields(reader, depth + 1) };
    case TAG_LIST: {
      const count = reader.length();
      const items: PackValue[] = [];
      for (let index = 0; index < count; index += 1) {
        const item = readPackValue(reader, depth + 1);
        if (item === undefined) throw new ToolRunCodecError("malformed", "absent list item");
        items.push(item);
      }
      return { t: "list", v: items };
    }
    default:
      throw new ToolRunCodecError("malformed", `unexpected tag 0x${tag.toString(16)}`);
  }
}

function readPackFields(reader: PackReader, depth: number): PackRecord {
  const count = reader.length();
  const record: PackRecord = new Map();
  for (let index = 0; index < count; index += 1) {
    const id = reader.varint();
    if (id > 0xffffn) throw new ToolRunCodecError("pack", "field id exceeds u16");
    const value = readPackValue(reader, depth + 1);
    if (value !== undefined) record.set(Number(id), value);
  }
  return record;
}

function decodePackBody(bytes: Uint8Array, knownIds: readonly number[]): PackRecord {
  const reader = new PackReader(bytes);
  const symbols = reader.length();
  for (let index = 0; index < symbols; index += 1) reader.raw(reader.length());
  const record = readPackFields(reader, 0);
  if (reader.offset !== bytes.length) throw new ToolRunCodecError("pack", "trailing bytes after the terminal record");
  assertKnown(record, knownIds);
  return record;
}

function assertKnown(record: PackRecord, knownIds: readonly number[]): void {
  for (const id of record.keys()) if (!knownIds.includes(id)) throw new ToolRunCodecError("pack", `unknown field ${id}`);
}

const IDENTITY_IDS = [1, 2, 3, 4];
const STEP_IDS = [1, 2, 3, 4, 5, 6, 7];
const PROGRESS_IDS = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
const PAGE_IDS = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11];
const DELTA_IDS = [1, 2, 3, 4];
const TICK_IDS = [1, 2, 3, 4, 5, 6, 7, 8, 9];

const uintValue = (v: bigint | number): PackValue => ({ t: "uint", v: BigInt(v) });
const bytesValue = (v: Uint8Array): PackValue => ({ t: "bytes", v });

function getUint(record: PackRecord, id: number): bigint {
  const value = getOptionalUint(record, id);
  if (value === undefined) throw new ToolRunCodecError("malformed", "required unsigned field");
  return value;
}

function getOptionalUint(record: PackRecord, id: number): bigint | undefined {
  const value = record.get(id);
  if (value === undefined) return undefined;
  if (value.t !== "uint") throw new ToolRunCodecError("malformed", "unsigned field");
  return value.v;
}

function getBytes(record: PackRecord, id: number): Uint8Array {
  const value = record.get(id);
  if (value === undefined) return new Uint8Array(0);
  if (value.t !== "bytes") throw new ToolRunCodecError("malformed", "bytes field");
  return value.v;
}

function getList(record: PackRecord, id: number): readonly PackValue[] {
  const value = record.get(id);
  if (value === undefined) return [];
  if (value.t !== "list") throw new ToolRunCodecError("malformed", "list field");
  return value.v;
}

function getRecord(value: PackValue | undefined, what: string, knownIds: readonly number[]): PackRecord {
  if (value?.t !== "record") throw new ToolRunCodecError("malformed", what);
  assertKnown(value.v, knownIds);
  return value.v;
}

function itemBytes(item: PackValue): Uint8Array {
  if (item.t !== "bytes") throw new ToolRunCodecError("malformed", "bytes item");
  return item.v;
}

function narrow(value: bigint, max: number, what: string): number {
  if (value > BigInt(max)) throw new ToolRunCodecError("malformed", what);
  return Number(value);
}

function view(bytes: Uint8Array): DataView {
  return new DataView(bytes.buffer, bytes.byteOffset, bytes.byteLength);
}

function identityRecord(identity: ToolRunIdentity): PackValue {
  if (identity.baseRevision.length !== 32) throw new ToolRunCodecError("malformed", "baseRevision");
  return { t: "record", v: new Map([[1, uintValue(identity.id.appInstanceId)], [2, uintValue(identity.id.run)], [3, uintValue(identity.generation)], [4, bytesValue(identity.baseRevision)]]) };
}

function identityFrom(value: PackValue | undefined): ToolRunIdentity {
  const record = getRecord(value, "identity", IDENTITY_IDS);
  const baseRevision = getBytes(record, 4);
  if (baseRevision.length !== 32) throw new ToolRunCodecError("malformed", "baseRevision");
  return { id: { appInstanceId: narrow(getUint(record, 1), 0xffff_ffff, "appInstanceId"), run: getUint(record, 2) }, generation: narrow(getUint(record, 3), 0xffff_ffff, "generation"), baseRevision };
}

function stepRecord(step: ToolRunStep): PackValue {
  if (step.args.length > TOOL_RUN_STEP_ARGS_MAX) throw new ToolRunCodecError("limit", "step args");
  if (step.repeat === 0) throw new ToolRunCodecError("malformed", "repeat");
  const record: PackRecord = new Map([[1, uintValue(step.sequence)], [2, uintValue(TOOL_RUN_STEP_KINDS.indexOf(step.kind))], [3, uintValue(step.stage)], [4, uintValue(step.reason)], [6, uintValue(step.repeat)]]);
  if (step.subject !== undefined) record.set(5, uintValue(step.subject));
  if (step.args.length > 0) {
    const args = new Uint8Array(9 * step.args.length);
    step.args.forEach((arg, index) => {
      if ("unsigned" in arg) view(args).setBigUint64(9 * index + 1, arg.unsigned, true);
      else {
        args[9 * index] = 1;
        view(args).setFloat64(9 * index + 1, arg.float, true);
      }
    });
    record.set(7, bytesValue(args));
  }
  return { t: "record", v: record };
}

function stepFrom(value: PackValue): ToolRunStep {
  const record = getRecord(value, "record item", STEP_IDS);
  const rawArgs = getBytes(record, 7);
  if (rawArgs.length % 9 !== 0 || rawArgs.length / 9 > TOOL_RUN_STEP_ARGS_MAX) throw new ToolRunCodecError("malformed", "step args");
  const args: ToolRunStepArg[] = [];
  for (let offset = 0; offset < rawArgs.length; offset += 9) {
    if (rawArgs[offset] === 0) args.push({ unsigned: view(rawArgs).getBigUint64(offset + 1, true) });
    else if (rawArgs[offset] === 1) args.push({ float: view(rawArgs).getFloat64(offset + 1, true) });
    else throw new ToolRunCodecError("malformed", "step arg kind");
  }
  const repeat = narrow(getUint(record, 6), 0xffff_ffff, "repeat");
  if (repeat === 0) throw new ToolRunCodecError("malformed", "repeat");
  const kind = TOOL_RUN_STEP_KINDS[narrow(getUint(record, 2), 0xff, "step kind")];
  if (kind === undefined) throw new ToolRunCodecError("malformed", "step kind");
  const subject = getOptionalUint(record, 5);
  return { sequence: getUint(record, 1), kind, stage: narrow(getUint(record, 3), 0xffff, "stage"), reason: narrow(getUint(record, 4), 0xffff, "reason"), ...(subject === undefined ? {} : { subject }), repeat, args };
}

function progressRecord(progress: ToolRunProgress): PackValue {
  if (progress.counters.length > TOOL_RUN_COUNTERS_MAX) throw new ToolRunCodecError("limit", "counters");
  const record: PackRecord = new Map([[1, identityRecord(progress.identity)], [2, uintValue(progress.sequence)], [3, uintValue(TOOL_RUN_STATES.indexOf(progress.state))], [4, uintValue(progress.stage)], [5, uintValue(progress.completed)], [8, { t: "f64", v: Math.fround(progress.unitsPerSecond) }], [9, uintValue(progress.conflicts)]]);
  if (progress.total !== undefined) record.set(6, uintValue(progress.total));
  if (progress.counters.length > 0) {
    const counters = new Uint8Array(10 * progress.counters.length);
    progress.counters.forEach((counter, index) => {
      view(counters).setUint16(10 * index, counter.counter, true);
      view(counters).setBigUint64(10 * index + 2, counter.value, true);
    });
    record.set(7, bytesValue(counters));
  }
  if (progress.steps.length > 0) record.set(10, { t: "list", v: progress.steps.map(stepRecord) });
  return { t: "record", v: record };
}

function progressFrom(value: PackValue): ToolRunProgress {
  const record = getRecord(value, "progress", PROGRESS_IDS);
  const rawCounters = getBytes(record, 7);
  if (rawCounters.length % 10 !== 0 || rawCounters.length / 10 > TOOL_RUN_COUNTERS_MAX) throw new ToolRunCodecError("malformed", "counters");
  const counters: ToolRunCounter[] = [];
  for (let offset = 0; offset < rawCounters.length; offset += 10) counters.push({ counter: view(rawCounters).getUint16(offset, true), value: view(rawCounters).getBigUint64(offset + 2, true) });
  const units = record.get(8);
  if (units?.t !== "f64") throw new ToolRunCodecError("malformed", "unitsPerSecond");
  const state = TOOL_RUN_STATES[narrow(getUint(record, 3), 0xff, "state")];
  if (state === undefined) throw new ToolRunCodecError("malformed", "state");
  const steps = getList(record, 10).map(stepFrom);
  if (steps.length > TOOL_RUN_STEP_RING_CAPACITY) throw new ToolRunCodecError("limit", "step ring capacity");
  const total = getOptionalUint(record, 6);
  return { identity: identityFrom(record.get(1)), sequence: getUint(record, 2), state, stage: narrow(getUint(record, 4), 0xffff, "stage"), completed: getUint(record, 5), ...(total === undefined ? {} : { total }), counters, unitsPerSecond: Math.fround(units.v), conflicts: narrow(getUint(record, 9), 0xffff_ffff, "conflicts"), steps };
}

/** 🎒️ Columnar pack record body of a trace page. */
export function encodeToolRunTracePage(page: ToolRunTracePage): Uint8Array {
  if (page.ops.length > TOOL_RUN_TRACE_PAGE_OPS_MAX) throw new ToolRunCodecError("limit", "trace page ops");
  const kinds: number[] = [];
  const keys: bigint[] = [];
  const verdicts: number[] = [];
  const reasons: number[] = [];
  const subjectKinds: number[] = [];
  const meshes: number[] = [];
  const shapes: number[] = [];
  const entities: bigint[] = [];
  const floats: number[] = [];
  for (const op of page.ops) {
    if (op.op === "clear") {
      kinds.push(2);
      continue;
    }
    keys.push(op.key);
    if (op.op === "retire") {
      kinds.push(1);
      continue;
    }
    kinds.push(0);
    verdicts.push(TOOL_RUN_VERDICTS.indexOf(op.verdict));
    reasons.push(op.reason);
    const subject = op.subject;
    if (subject.kind === "instance3d") {
      subjectKinds.push(0);
      meshes.push(subject.mesh);
      floats.push(...subject.position, ...subject.rotation, subject.scale);
    } else if (subject.kind === "placement2d") {
      subjectKinds.push(1);
      shapes.push(subject.shape);
      floats.push(...subject.position, subject.rotation);
    } else {
      subjectKinds.push(2);
      entities.push(subject.entity);
    }
  }
  const column = (count: number, width: number, write: (cell: DataView, index: number) => void): Uint8Array => {
    const bytes = new Uint8Array(count * width);
    const cell = view(bytes);
    for (let index = 0; index < count; index += 1) write(cell, index);
    return bytes;
  };
  const columns: [number, Uint8Array][] = [
    [3, Uint8Array.from(kinds)],
    [4, column(keys.length, 8, (cell, index) => cell.setBigUint64(8 * index, keys[index]!, true))],
    [5, Uint8Array.from(verdicts)],
    [6, column(reasons.length, 2, (cell, index) => cell.setUint16(2 * index, reasons[index]!, true))],
    [7, Uint8Array.from(subjectKinds)],
    [8, column(meshes.length, 4, (cell, index) => cell.setUint32(4 * index, meshes[index]!, true))],
    [9, column(shapes.length, 4, (cell, index) => cell.setUint32(4 * index, shapes[index]!, true))],
    [10, column(entities.length, 8, (cell, index) => cell.setBigUint64(8 * index, entities[index]!, true))],
    [11, column(floats.length, 4, (cell, index) => cell.setFloat32(4 * index, floats[index]!, true))],
  ];
  const record: PackRecord = new Map([[1, identityRecord(page.identity)], [2, uintValue(page.page)]]);
  for (const [id, bytes] of columns) if (bytes.length > 0) record.set(id, bytesValue(bytes));
  const bytes = encodePackBody(record);
  if (bytes.length > TOOL_RUN_TRACE_PAGE_BYTES_MAX) throw new ToolRunCodecError("limit", "trace page bytes");
  return bytes;
}

export function decodeToolRunTracePage(bytes: Uint8Array): ToolRunTracePage {
  if (bytes.length > TOOL_RUN_TRACE_PAGE_BYTES_MAX) throw new ToolRunCodecError("limit", "trace page bytes");
  return tracePageFrom(decodePackBody(bytes, PAGE_IDS));
}

function tracePageFrom(record: PackRecord): ToolRunTracePage {
  const kinds = getBytes(record, 3);
  if (kinds.length > TOOL_RUN_TRACE_PAGE_OPS_MAX) throw new ToolRunCodecError("limit", "trace page ops");
  const keys = getBytes(record, 4);
  const verdicts = getBytes(record, 5);
  const reasons = getBytes(record, 6);
  const subjectKinds = getBytes(record, 7);
  const meshes = getBytes(record, 8);
  const shapes = getBytes(record, 9);
  const entities = getBytes(record, 10);
  const floats = getBytes(record, 11);
  const count = (bytes: Uint8Array, value: number) => bytes.reduce((sum, byte) => sum + (byte === value ? 1 : 0), 0);
  const upserts = count(kinds, 0);
  const keyed = upserts + count(kinds, 1);
  const instances = count(subjectKinds, 0);
  const placements = count(subjectKinds, 1);
  const entitySubjects = count(subjectKinds, 2);
  if (kinds.some((kind) => kind > 2) || keys.length % 8 !== 0 || keys.length / 8 !== keyed || verdicts.length !== upserts || reasons.length !== 2 * upserts || subjectKinds.length !== upserts || instances + placements + entitySubjects !== upserts || meshes.length % 4 !== 0 || meshes.length / 4 !== instances || shapes.length % 4 !== 0 || shapes.length / 4 !== placements || entities.length % 8 !== 0 || entities.length / 8 !== entitySubjects || floats.length % 4 !== 0 || floats.length / 4 !== 8 * instances + 3 * placements) {
    throw new ToolRunCodecError("malformed", "trace page columns");
  }
  const keyCells = view(keys);
  const reasonCells = view(reasons);
  const meshCells = view(meshes);
  const shapeCells = view(shapes);
  const entityCells = view(entities);
  const floatCells = view(floats);
  const float = (index: number) => floatCells.getFloat32(4 * index, true);
  let keyAt = 0;
  let upsertAt = 0;
  let meshAt = 0;
  let shapeAt = 0;
  let entityAt = 0;
  let floatAt = 0;
  const ops: ToolRunTraceOp[] = [];
  for (const kind of kinds) {
    if (kind === 2) {
      ops.push({ op: "clear" });
      continue;
    }
    const key = keyCells.getBigUint64(8 * keyAt++, true);
    if (kind === 1) {
      ops.push({ op: "retire", key });
      continue;
    }
    const verdict = TOOL_RUN_VERDICTS[verdicts[upsertAt]!];
    if (verdict === undefined) throw new ToolRunCodecError("malformed", "verdict");
    let subject: ToolRunTraceSubject;
    const subjectKind = subjectKinds[upsertAt]!;
    if (subjectKind === 0) {
      subject = { kind: "instance3d", mesh: meshCells.getUint32(4 * meshAt++, true), position: [float(floatAt), float(floatAt + 1), float(floatAt + 2)], rotation: [float(floatAt + 3), float(floatAt + 4), float(floatAt + 5), float(floatAt + 6)], scale: float(floatAt + 7) };
      floatAt += 8;
    } else if (subjectKind === 1) {
      subject = { kind: "placement2d", shape: shapeCells.getUint32(4 * shapeAt++, true), position: [float(floatAt), float(floatAt + 1)], rotation: float(floatAt + 2) };
      floatAt += 3;
    } else {
      subject = { kind: "entity", entity: entityCells.getBigUint64(8 * entityAt++, true) };
    }
    ops.push({ op: "upsert", key, verdict, reason: reasonCells.getUint16(2 * upsertAt, true), subject });
    upsertAt += 1;
  }
  return { identity: identityFrom(record.get(1)), page: narrow(getUint(record, 2), 0xffff_ffff, "page"), ops };
}

export function encodeToolRunTraceDelta(delta: ToolRunTraceDelta): Uint8Array {
  const record: PackRecord = new Map([[1, identityRecord(delta.identity)], [2, { t: "bool", v: delta.clear }], [3, uintValue(delta.next)]]);
  if (delta.pages.length > 0) record.set(4, { t: "list", v: delta.pages.map((page) => bytesValue(encodeToolRunTracePage(page))) });
  return encodePackBody(record);
}

export function decodeToolRunTraceDelta(bytes: Uint8Array): ToolRunTraceDelta {
  const record = decodePackBody(bytes, DELTA_IDS);
  const clear = record.get(2);
  if (clear?.t !== "bool") throw new ToolRunCodecError("malformed", "boolean field");
  return { identity: identityFrom(record.get(1)), clear: clear.v, next: narrow(getUint(record, 3), 0xffff_ffff, "next"), pages: getList(record, 4).map((item) => decodeToolRunTracePage(itemBytes(item))) };
}

/** 🧳️ Pack record body of a tick; rejects ticks over `TOOL_RUN_TICK_BYTES_MAX`. */
export function encodeToolRunTick(tick: ToolRunTick): Uint8Array {
  const record: PackRecord = new Map([[1, identityRecord(tick.identity)], [2, uintValue(tick.sequence)]]);
  if (tick.progress !== undefined) record.set(3, progressRecord(tick.progress));
  if (tick.steps.length > 0) record.set(4, { t: "list", v: tick.steps.map(stepRecord) });
  if (tick.trace.length > 0) record.set(5, { t: "list", v: tick.trace.map((page) => bytesValue(encodeToolRunTracePage(page))) });
  if (tick.appendOps.length > 0) record.set(6, { t: "list", v: tick.appendOps.map(bytesValue) });
  if (tick.appendEntities.length > 0) {
    const entities = new Uint8Array(8 * tick.appendEntities.length);
    tick.appendEntities.forEach((entity, index) => view(entities).setBigUint64(8 * index, entity, true));
    record.set(7, bytesValue(entities));
  }
  if (tick.retractTo !== undefined) record.set(8, uintValue(tick.retractTo));
  if (tick.payload !== undefined) record.set(9, bytesValue(tick.payload));
  const bytes = encodePackBody(record);
  if (bytes.length > TOOL_RUN_TICK_BYTES_MAX) throw new ToolRunCodecError("limit", "tick bytes");
  return bytes;
}

export function decodeToolRunTick(bytes: Uint8Array): ToolRunTick {
  if (bytes.length > TOOL_RUN_TICK_BYTES_MAX) throw new ToolRunCodecError("limit", "tick bytes");
  const record = decodePackBody(bytes, TICK_IDS);
  const rawEntities = getBytes(record, 7);
  if (rawEntities.length % 8 !== 0) throw new ToolRunCodecError("malformed", "appendEntities");
  const appendEntities: bigint[] = [];
  for (let offset = 0; offset < rawEntities.length; offset += 8) appendEntities.push(view(rawEntities).getBigUint64(offset, true));
  const progress = record.get(3);
  const retractTo = getOptionalUint(record, 8);
  const payload = record.has(9) ? getBytes(record, 9) : undefined;
  return {
    identity: identityFrom(record.get(1)),
    sequence: getUint(record, 2),
    ...(progress === undefined ? {} : { progress: progressFrom(progress) }),
    steps: getList(record, 4).map(stepFrom),
    trace: getList(record, 5).map((item) => decodeToolRunTracePage(itemBytes(item))),
    appendOps: getList(record, 6).map(itemBytes),
    appendEntities,
    ...(retractTo === undefined ? {} : { retractTo: narrow(retractTo, 0xffff_ffff, "retractTo") }),
    ...(payload === undefined ? {} : { payload }),
  };
}
//#endregion 🔖️Codec

//#region 🔖️Json
/** 🧾️ Schema JSON form (`🧬️schema/🔣️.json`): u64 as integers, bytes as lowercase hex. */
export type ToolRunJson = null | boolean | number | string | readonly ToolRunJson[] | { readonly [key: string]: ToolRunJson };
type JsonObject = { readonly [key: string]: any };

export function toolRunHexToBytes(hex: string): Uint8Array {
  if (!/^([0-9a-f]{2})*$/.test(hex)) throw new ToolRunCodecError("malformed", "hex");
  const bytes = new Uint8Array(hex.length / 2);
  for (let index = 0; index < bytes.length; index += 1) bytes[index] = parseInt(hex.slice(2 * index, 2 * index + 2), 16);
  return bytes;
}

export function toolRunBytesToHex(bytes: Uint8Array): string {
  return Array.from(bytes, (byte) => byte.toString(16).padStart(2, "0")).join("");
}

const u64Json = (value: bigint): number => {
  if (value > BigInt(Number.MAX_SAFE_INTEGER)) throw new ToolRunCodecError("limit", "u64 beyond JSON safe integers");
  return Number(value);
};

export function toolRunIdentityFromJson(json: JsonObject): ToolRunIdentity {
  return { id: { appInstanceId: json.id.appInstanceId, run: BigInt(json.id.run) }, generation: json.generation, baseRevision: toolRunHexToBytes(json.baseRevision) };
}

export function toolRunIdentityToJson(identity: ToolRunIdentity): ToolRunJson {
  return { id: { appInstanceId: identity.id.appInstanceId, run: u64Json(identity.id.run) }, generation: identity.generation, baseRevision: toolRunBytesToHex(identity.baseRevision) };
}

export function toolRunSlotFromJson(json: JsonObject | null): ToolRunSlot | null {
  return json === null ? null : { run: BigInt(json.run), generation: json.generation, state: json.state };
}

export function toolRunSlotToJson(slot: ToolRunSlot | null): ToolRunJson {
  return slot === null ? null : { run: u64Json(slot.run), generation: slot.generation, state: slot.state };
}

export function toolRunEventFromJson(json: JsonObject): ToolRunEvent {
  return (json.type === "closed" ? { type: "closed" } : { ...json, run: BigInt(json.run) }) as ToolRunEvent;
}

export function toolRunStepFromJson(json: JsonObject): ToolRunStep {
  return { sequence: BigInt(json.sequence), kind: json.kind, stage: json.stage, reason: json.reason, ...(json.subject === undefined ? {} : { subject: BigInt(json.subject) }), repeat: json.repeat, args: json.args.map((arg: JsonObject) => ("unsigned" in arg ? { unsigned: BigInt(arg.unsigned) } : { float: arg.float })) };
}

export function toolRunStepToJson(step: ToolRunStep): ToolRunJson {
  return { sequence: u64Json(step.sequence), kind: step.kind, stage: step.stage, reason: step.reason, ...(step.subject === undefined ? {} : { subject: u64Json(step.subject) }), repeat: step.repeat, args: step.args.map((arg): ToolRunJson => ("unsigned" in arg ? { unsigned: u64Json(arg.unsigned) } : { float: arg.float })) };
}

export function toolRunTraceOpFromJson(json: JsonObject): ToolRunTraceOp {
  if (json.op === "clear") return { op: "clear" };
  if (json.op === "retire") return { op: "retire", key: BigInt(json.key) };
  const subject: ToolRunTraceSubject = json.subject.kind === "entity" ? { kind: "entity", entity: BigInt(json.subject.entity) } : json.subject;
  return { op: "upsert", key: BigInt(json.key), verdict: json.verdict, reason: json.reason, subject };
}

export function toolRunTraceOpToJson(op: ToolRunTraceOp): ToolRunJson {
  if (op.op === "clear") return { op: "clear" };
  if (op.op === "retire") return { op: "retire", key: u64Json(op.key) };
  const subject: ToolRunJson = op.subject.kind === "entity" ? { kind: "entity", entity: u64Json(op.subject.entity) } : { ...op.subject, position: [...op.subject.position], rotation: op.subject.kind === "instance3d" ? [...op.subject.rotation] : op.subject.rotation };
  return { op: "upsert", key: u64Json(op.key), verdict: op.verdict, reason: op.reason, subject };
}

export function toolRunTracePageFromJson(json: JsonObject): ToolRunTracePage {
  return { identity: toolRunIdentityFromJson(json.identity), page: json.page, ops: json.ops.map(toolRunTraceOpFromJson) };
}

export function toolRunTracePageToJson(page: ToolRunTracePage): ToolRunJson {
  return { identity: toolRunIdentityToJson(page.identity), page: page.page, ops: page.ops.map(toolRunTraceOpToJson) };
}

export function toolRunTraceDeltaFromJson(json: JsonObject): ToolRunTraceDelta {
  return { identity: toolRunIdentityFromJson(json.identity), clear: json.clear, next: json.next, pages: json.pages.map(toolRunTracePageFromJson) };
}

export function toolRunTraceDeltaToJson(delta: ToolRunTraceDelta): ToolRunJson {
  return { identity: toolRunIdentityToJson(delta.identity), clear: delta.clear, next: delta.next, pages: delta.pages.map(toolRunTracePageToJson) };
}

export function toolRunProgressFromJson(json: JsonObject): ToolRunProgress {
  return {
    identity: toolRunIdentityFromJson(json.identity),
    sequence: BigInt(json.sequence),
    state: json.state,
    stage: json.stage,
    completed: BigInt(json.completed),
    ...(json.total === undefined ? {} : { total: BigInt(json.total) }),
    counters: json.counters.map((counter: JsonObject) => ({ counter: counter.counter, value: BigInt(counter.value) })),
    unitsPerSecond: json.unitsPerSecond,
    conflicts: json.conflicts,
    steps: json.steps.map(toolRunStepFromJson),
  };
}

export function toolRunProgressToJson(progress: ToolRunProgress): ToolRunJson {
  return {
    identity: toolRunIdentityToJson(progress.identity),
    sequence: u64Json(progress.sequence),
    state: progress.state,
    stage: progress.stage,
    completed: u64Json(progress.completed),
    ...(progress.total === undefined ? {} : { total: u64Json(progress.total) }),
    counters: progress.counters.map((counter) => ({ counter: counter.counter, value: u64Json(counter.value) })),
    unitsPerSecond: progress.unitsPerSecond,
    conflicts: progress.conflicts,
    steps: progress.steps.map(toolRunStepToJson),
  };
}

export function toolRunTickFromJson(json: JsonObject): ToolRunTick {
  return {
    identity: toolRunIdentityFromJson(json.identity),
    sequence: BigInt(json.sequence),
    ...(json.progress === undefined ? {} : { progress: toolRunProgressFromJson(json.progress) }),
    steps: json.steps.map(toolRunStepFromJson),
    trace: json.trace.map(toolRunTracePageFromJson),
    appendOps: json.appendOps.map(toolRunHexToBytes),
    appendEntities: json.appendEntities.map((entity: number) => BigInt(entity)),
    ...(json.retractTo === undefined ? {} : { retractTo: json.retractTo }),
    ...(json.payload === undefined ? {} : { payload: toolRunHexToBytes(json.payload) }),
  };
}

export function toolRunTickToJson(tick: ToolRunTick): ToolRunJson {
  return {
    identity: toolRunIdentityToJson(tick.identity),
    sequence: u64Json(tick.sequence),
    ...(tick.progress === undefined ? {} : { progress: toolRunProgressToJson(tick.progress) }),
    steps: tick.steps.map(toolRunStepToJson),
    trace: tick.trace.map(toolRunTracePageToJson),
    appendOps: tick.appendOps.map(toolRunBytesToHex),
    appendEntities: tick.appendEntities.map(u64Json),
    ...(tick.retractTo === undefined ? {} : { retractTo: tick.retractTo }),
    ...(tick.payload === undefined ? {} : { payload: toolRunBytesToHex(tick.payload) }),
  };
}
//#endregion 🔖️Json

//#region 🔖️Panel
/** 🪧️ Key of the framework ToolRun panel root; each run is one child group {@link toolRunPanelGroupId}. */
export const TOOL_RUN_PANEL_ID = "framework.toolRun";

/** 🪧️ The key of run `run`'s group in the ToolRun panel. */
export function toolRunPanelGroupId(run: bigint): string {
  return `${TOOL_RUN_PANEL_ID}.${run}`;
}

/** 🔎️ The run a ToolRun panel group key names; `null` for any other key. */
export function toolRunPanelGroupRun(key: string): bigint | null {
  const digits = key.startsWith(`${TOOL_RUN_PANEL_ID}.`) ? key.slice(TOOL_RUN_PANEL_ID.length + 1) : "";
  return /^(0|[1-9][0-9]*)$/.test(digits) ? BigInt(digits) : null;
}

/** 📣️ The runs a rendered ToolRun panel holds and those of them not in `previous`. */
export function toolRunPanelNewRuns(previous: ReadonlySet<bigint>, keys: Iterable<string>): { readonly current: ReadonlySet<bigint>; readonly added: readonly bigint[] } {
  const current = new Set<bigint>();
  for (const key of keys) {
    const run = toolRunPanelGroupRun(key);
    if (run !== null) current.add(run);
  }
  return { current, added: [...current].filter((run) => !previous.has(run)).sort((left, right) => (left < right ? -1 : left > right ? 1 : 0)) };
}
//#endregion 🔖️Panel

//#region 🔖️Actions
export const TOOL_RUN_START_ACTION_ID = "toolRunStart";
export const TOOL_RUN_PAUSE_ACTION_ID = "toolRunPause";
export const TOOL_RUN_RESUME_ACTION_ID = "toolRunResume";
export const TOOL_RUN_STEP_ACTION_ID = "toolRunStep";
export const TOOL_RUN_ABORT_ACTION_ID = "toolRunAbort";
export const TOOL_RUN_FINALIZE_ACTION_ID = "toolRunFinalize";
export const TOOL_RUN_DISMISS_ACTION_ID = "toolRunDismiss";
export const TOOL_RUN_ACTION_IDS = [TOOL_RUN_START_ACTION_ID, TOOL_RUN_PAUSE_ACTION_ID, TOOL_RUN_RESUME_ACTION_ID, TOOL_RUN_STEP_ACTION_ID, TOOL_RUN_ABORT_ACTION_ID, TOOL_RUN_FINALIZE_ACTION_ID, TOOL_RUN_DISMISS_ACTION_ID] as const;
export type ToolRunActionId = (typeof TOOL_RUN_ACTION_IDS)[number];

export const TOOL_RUN_START_CHORD = "mod+enter";
export const TOOL_RUN_PAUSE_RESUME_CHORD = "mod+alt+enter";
export const TOOL_RUN_STEP_CHORD = "mod+alt+arrowright";
export const TOOL_RUN_ABORT_CHORD = "mod+.";
export const TOOL_RUN_FINALIZE_CHORD = "mod+shift+enter";
export const TOOL_RUN_DISMISS_CHORD = "escape";

export const TOOL_RUN_ARG_TOOL_ID = "toolId";
export const TOOL_RUN_ARG_WINDOW_ID = "windowId";
export const TOOL_RUN_ARG_RUN_ID = "runId";
export const TOOL_RUN_ARG_GENERATION = "generation";

export type ToolRunActionDescriptor = { readonly id: ToolRunActionId; readonly chord: string; readonly label: ToolRunLabelKey; readonly args: readonly { readonly name: string; readonly required: boolean }[] };

const TARGETED_ARGS = [
  { name: TOOL_RUN_ARG_RUN_ID, required: true },
  { name: TOOL_RUN_ARG_GENERATION, required: true },
] as const;

/** 🎛️ The seven generic tool run actions (§2.5). */
export const TOOL_RUN_ACTIONS: readonly ToolRunActionDescriptor[] = [
  {
    id: TOOL_RUN_START_ACTION_ID,
    chord: TOOL_RUN_START_CHORD,
    label: "actionStart",
    args: [
      { name: TOOL_RUN_ARG_TOOL_ID, required: true },
      { name: TOOL_RUN_ARG_WINDOW_ID, required: false },
    ],
  },
  { id: TOOL_RUN_PAUSE_ACTION_ID, chord: TOOL_RUN_PAUSE_RESUME_CHORD, label: "actionPause", args: TARGETED_ARGS },
  { id: TOOL_RUN_RESUME_ACTION_ID, chord: TOOL_RUN_PAUSE_RESUME_CHORD, label: "actionResume", args: TARGETED_ARGS },
  { id: TOOL_RUN_STEP_ACTION_ID, chord: TOOL_RUN_STEP_CHORD, label: "actionStep", args: TARGETED_ARGS },
  { id: TOOL_RUN_ABORT_ACTION_ID, chord: TOOL_RUN_ABORT_CHORD, label: "actionAbort", args: TARGETED_ARGS },
  { id: TOOL_RUN_FINALIZE_ACTION_ID, chord: TOOL_RUN_FINALIZE_CHORD, label: "actionFinalize", args: TARGETED_ARGS },
  { id: TOOL_RUN_DISMISS_ACTION_ID, chord: TOOL_RUN_DISMISS_CHORD, label: "actionDismiss", args: [{ name: TOOL_RUN_ARG_RUN_ID, required: true }] },
];

/** 🚦️ Whether an action is enabled for a slot in `state` (`null` = no run). */
export function isToolRunActionLegal(id: ToolRunActionId, state: ToolRunState | null): boolean {
  switch (id) {
    case TOOL_RUN_START_ACTION_ID:
      return state === null || isToolRunTerminal(state);
    case TOOL_RUN_DISMISS_ACTION_ID:
      return state !== null && isToolRunTerminal(state);
    case TOOL_RUN_PAUSE_ACTION_ID:
      return state === "running";
    case TOOL_RUN_RESUME_ACTION_ID:
    case TOOL_RUN_STEP_ACTION_ID:
      return state === "paused";
    case TOOL_RUN_FINALIZE_ACTION_ID:
      return state === "complete";
    case TOOL_RUN_ABORT_ACTION_ID:
      return state === "starting" || state === "running" || state === "paused" || state === "complete" || state === "finalizing";
  }
}
//#endregion 🔖️Actions

//#region 🔖️Labels
/** 🗣️ Framework-owned EN/DE text of the tool run panel (§2.5), no default locale. */
export const TOOL_RUN_LABELS = {
  stateStarting: { en: "Starting", de: "Wird gestartet" },
  stateRunning: { en: "Running", de: "Läuft" },
  statePaused: { en: "Paused", de: "Pausiert" },
  stateComplete: { en: "Complete, ready to finalize", de: "Fertig, bereit zum Abschließen" },
  stateFinalizing: { en: "Finalizing", de: "Wird abgeschlossen" },
  stateFinalized: { en: "Finalized", de: "Abgeschlossen" },
  stateAborting: { en: "Aborting", de: "Wird abgebrochen" },
  stateAborted: { en: "Aborted, nothing was changed", de: "Abgebrochen, nichts wurde geändert" },
  stateFaulted: { en: "Failed, nothing was changed", de: "Fehlgeschlagen, nichts wurde geändert" },
  actionStart: { en: "Start", de: "Starten" },
  actionPause: { en: "Pause", de: "Pausieren" },
  actionResume: { en: "Resume", de: "Fortsetzen" },
  actionStep: { en: "Step", de: "Einzelschritt" },
  actionAbort: { en: "Abort", de: "Abbrechen" },
  actionFinalize: { en: "Finalize", de: "Abschließen" },
  actionDismiss: { en: "Dismiss", de: "Schließen" },
  finalizeDisabled: { en: "Available once the run is complete", de: "Verfügbar, sobald der Lauf fertig ist" },
  readyToStart: { en: "Ready to start", de: "Bereit zum Starten" },
  rebasingStep: { en: "Artifact changed, re-applying provisional result", de: "Artefakt geändert, vorläufiges Ergebnis wird neu angewendet" },
  conflictStep: { en: "{0} provisional changes conflict with the current artifact", de: "{0} vorläufige Änderungen stehen im Konflikt mit dem aktuellen Artefakt" },
  traceTruncatedStep: { en: "Oldest {0} rejected attempts are no longer shown", de: "Die ältesten {0} verworfenen Versuche werden nicht mehr angezeigt" },
  provisionalCapStep: { en: "Provisional change limit of {0} reached, run completed", de: "Grenze von {0} vorläufigen Änderungen erreicht, Lauf abgeschlossen" },
  progressValueText: { en: "{stage} ({i}/{n}): {completed} of {total} {unit} ({pct} %)", de: "{stage} ({i}/{n}): {completed} von {total} {unit} ({pct} %)" },
} as const satisfies Record<string, { en: string; de: string }>;
export type ToolRunLabelKey = keyof typeof TOOL_RUN_LABELS;

/** 💬️ Status label key of a state. */
export function toolRunStateLabel(state: ToolRunState): ToolRunLabelKey {
  return `state${state[0]!.toUpperCase()}${state.slice(1)}` as ToolRunLabelKey;
}

/** 🚧️ Label key of a framework-reserved reason code. */
export function toolRunReasonLabel(code: number): ToolRunLabelKey | undefined {
  return ({ [TOOL_RUN_REASON_REBASING]: "rebasingStep", [TOOL_RUN_REASON_CONFLICT]: "conflictStep", [TOOL_RUN_REASON_TRACE_TRUNCATED]: "traceTruncatedStep", [TOOL_RUN_REASON_PROVISIONAL_CAP]: "provisionalCapStep" } as Record<number, ToolRunLabelKey>)[code];
}

/** 🔣️ Single-pass `{name}` substitution; unknown or unterminated placeholders stay literal. */
export function toolRunFormat(template: string, value: (name: string) => string | undefined): string {
  let out = "";
  let rest = template;
  for (let open = rest.indexOf("{"); open !== -1; open = rest.indexOf("{")) {
    out += rest.slice(0, open);
    const after = rest.slice(open + 1);
    const close = after.search(/[{}]/);
    const text = close !== -1 && after[close] === "}" ? value(after.slice(0, close)) : undefined;
    if (text === undefined) {
      out += "{";
      rest = after;
    } else {
      out += text;
      rest = after.slice(close + 1);
    }
  }
  return out + rest;
}
//#endregion 🔖️Labels
