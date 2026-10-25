/** 🕵️ Tool run trace: columnar trace pages, deltas, cursors and the resident `ToolRunTraceStore`. TypeScript twin of `🗄️trace-store/🦀️.rs`. */

import { TOOL_RUN_STEP_RING_CAPACITY, TOOL_RUN_TRACE_LOG_COMPACT_FLOOR, TOOL_RUN_TRACE_PAGE_OPS_MAX, TOOL_RUN_TRACE_PAGE_OVERHEAD_BYTES, TOOL_RUN_TRACE_RESIDENT_RECORDS, type ToolRunIdentity } from "../🟦️.ts";
import { isToolRunVerdictRejected, type ToolRunVerdict } from "../📊️progress/🟦️.ts";

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
