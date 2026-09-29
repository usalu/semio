/** 📮️ The persisted local-only outbox of recorded answers, shared by every tab of the site.
 *
 * An answer applies locally at once and its `record-answer` command waits here until the proctor has decided it. Each
 * command is its own storage record (keyed by command id), so tabs never overwrite each other's queued answers; every
 * tab merges the others' records as they appear or disappear, so each holds the union of all tabs' queues in
 * queued-at order. Per (run, task) only the latest queued answer stays — the proctor keeps the latest anyway — except
 * that a command already on the wire is never replaced, so a delivered answer is never mistaken for a newer one.
 * Delivery is one at a time, retried with jittered backoff through connection shortages under the command's own id
 * (the idempotency key), so the same command delivered by two tabs is applied once. Before every attempt a tab checks
 * that the record still exists: another tab may have delivered or superseded it meanwhile.
 *
 * @see ../../../../../../🔨️modules/⏳️async/🔁️jittered-backoff/🟦️.ts
 * @see ../💾️persistence/🟦️.ts
 */

import { retryWithJitteredBackoff } from "@semio-tech/framework";
import type { Answer, Id, RecordAnswerCommand, Slug } from "@semio-tech/quiz";
import { isRecord, type LocalChange, type LocalStore } from "../💾️persistence/🟦️.ts";
import { isTransient, type CommandVerdict, type RetryTiming } from "../🛂️proctor/🟦️.ts";

/** 📮️ One queued answer and when it was queued. */
export interface OutboxEntry {
  readonly command: RecordAnswerCommand;
  readonly queuedAt: number;
}

/** 📶️ What the outbox is doing: nothing to send, sending, or waiting to retry after a transient failure. */
export type OutboxActivity = "idle" | "sending" | "retrying";

/** 📊️ An immutable snapshot of the outbox for rendering. */
export interface OutboxStatus {
  readonly pending: number;
  readonly activity: OutboxActivity;
  readonly failures: number;
}

/** ⚙️ How an {@link Outbox} sends, persists, times and reports. */
export interface OutboxOptions {
  readonly send: (command: RecordAnswerCommand, signal: AbortSignal) => Promise<CommandVerdict>;
  readonly store: LocalStore;
  readonly timing: RetryTiming;
  readonly now?: () => number;
  readonly onSettled?: (command: RecordAnswerCommand, verdict: CommandVerdict) => void;
}

interface Delivery {
  readonly command: RecordAnswerCommand;
  readonly verdict: CommandVerdict | undefined;
}

/** 🔑️ The coalescing key of an answer: its run and task. */
export function coalescingKey(command: Pick<RecordAnswerCommand, "run" | "task">): string {
  return `${command.run}/${command.task}`;
}

function restoredEntry(value: unknown): OutboxEntry | undefined {
  if (!isRecord(value) || typeof value.queuedAt !== "number" || !isRecord(value.command)) return undefined;
  const command = value.command;
  if (command.type !== "record-answer" || typeof command.id !== "string" || typeof command.learner !== "string" || typeof command.run !== "string" || typeof command.task !== "string" || !isRecord(command.answer)) return undefined;
  return { command: command as unknown as RecordAnswerCommand, queuedAt: value.queuedAt };
}

function queueOrder(left: OutboxEntry, right: OutboxEntry): number {
  return left.queuedAt - right.queuedAt || (left.command.id < right.command.id ? -1 : left.command.id > right.command.id ? 1 : 0);
}

/** 🧹️ `entries` in queued-at order with, per (run, task), only the latest one kept — plus `keep` (the command on the
 * wire), which is never dropped; the rest is returned as superseded. */
export function coalesce(entries: readonly OutboxEntry[], keep?: Id): { readonly kept: readonly OutboxEntry[]; readonly superseded: readonly OutboxEntry[] } {
  const ordered = [...entries].sort(queueOrder);
  const latest = new Map(ordered.map((entry) => [coalescingKey(entry.command), entry.command.id]));
  return {
    kept: ordered.filter((entry) => entry.command.id === keep || latest.get(coalescingKey(entry.command)) === entry.command.id),
    superseded: ordered.filter((entry) => entry.command.id !== keep && latest.get(coalescingKey(entry.command)) !== entry.command.id),
  };
}

/** 📮️ The answer outbox of one quiz client tab. */
export class Outbox {
  private readonly options: OutboxOptions;
  private entries: readonly OutboxEntry[] = [];
  private readonly stored = new Set<Id>();
  private sending: Id | undefined;
  private activity: OutboxActivity = "idle";
  private failures = 0;
  private lastQueuedAt = 0;
  private cycle: AbortController | undefined;
  private running = false;
  private stopped = true;
  private unwatch: (() => void) | undefined;
  private snapshot: OutboxStatus;
  private readonly listeners = new Set<() => void>();

  constructor(options: OutboxOptions) {
    this.options = options;
    this.load();
    this.snapshot = this.freeze();
  }

  /** 📊️ The current status; the same object until something changes. */
  status(): OutboxStatus {
    return this.snapshot;
  }

  /** 🔔️ Calls `listener` on every change; returns the unsubscribe function. */
  subscribe(listener: () => void): () => void {
    this.listeners.add(listener);
    return () => this.listeners.delete(listener);
  }

  /** 📋️ The queued commands of every tab, optionally of one run, in delivery order. */
  queued(run?: Id): readonly RecordAnswerCommand[] {
    return this.entries.filter((entry) => run === undefined || entry.command.run === run).map((entry) => entry.command);
  }

  /** ✍️ The latest queued answer per task of `run`. */
  pendingAnswers(run: Id): Readonly<Record<Slug, Answer>> {
    return Object.fromEntries(this.queued(run).map((command) => [command.task, command.answer]));
  }

  /** ▶️ Starts (or resumes) delivering the queue and following the other tabs' queues. */
  start(): void {
    this.stopped = false;
    this.unwatch ??= this.options.store.watch((change) => this.remote(change));
    this.load();
    this.notify();
    void this.pump();
  }

  /** ⏹️ Stops delivering and following; the queue stays persisted for the next {@link start}. */
  stop(): void {
    this.stopped = true;
    this.unwatch?.();
    this.unwatch = undefined;
    this.cycle?.abort();
  }

  /** ➕️ Queues `command` as its own record, superseding every queued (not in-flight) answer to the same run and task;
   * its queued-at time is later than that of every command this tab knows of, so it is the latest everywhere. */
  enqueue(command: RecordAnswerCommand): void {
    this.lastQueuedAt = Math.max((this.options.now ?? Date.now)(), this.lastQueuedAt + 1);
    const entry: OutboxEntry = { command, queuedAt: this.lastQueuedAt };
    const key = coalescingKey(command);
    const superseded = this.entries.filter((queued) => queued.command.id !== this.sending && coalescingKey(queued.command) === key);
    this.options.store.put("outbox", command.id, entry);
    if (this.options.store.record("outbox", command.id) !== undefined) this.stored.add(command.id);
    for (const queued of superseded) this.release(queued.command.id);
    this.entries = [...this.entries.filter((queued) => !superseded.includes(queued)), entry].sort(queueOrder);
    this.notify();
    void this.pump();
  }

  /** 🗑️ Drops every answer of `run`, e.g. once the run was submitted or voided elsewhere; one already in flight finishes
   * its attempt but is never retried nor restored. */
  discard(run: Id): void {
    this.remove((command) => command.run === run);
  }

  /** 🗑️ Drops every answer of `learner`, e.g. once the proctor no longer knows them (in flight: as {@link discard}). */
  forget(learner: Id): void {
    this.remove((command) => command.learner === learner);
  }

  /** ⏰️ Retries now instead of waiting out the current backoff (e.g. when the browser reports it is online again). */
  wake(): void {
    if (this.sending === undefined) this.cycle?.abort();
    void this.pump();
  }

  /** ⏳️ Resolves once no answer of `run` is queued, reporting `(delivered, total)`; rejects when `signal` aborts. */
  settled(run: Id, signal?: AbortSignal, onProgress?: (delivered: number, total: number) => void): Promise<void> {
    const remaining = (): number => this.entries.filter((entry) => entry.command.run === run).length;
    let total = remaining();
    return new Promise<void>((resolve, reject) => {
      if (signal?.aborted) {
        reject(signal.reason);
        return;
      }
      const finish = (): void => {
        unsubscribe();
        signal?.removeEventListener("abort", abort);
      };
      const abort = (): void => {
        finish();
        reject(signal?.reason);
      };
      const check = (): void => {
        const left = remaining();
        total = Math.max(total, left);
        onProgress?.(total - left, total);
        if (left > 0) return;
        finish();
        resolve();
      };
      const unsubscribe = this.subscribe(check);
      signal?.addEventListener("abort", abort, { once: true });
      check();
      if (remaining() > 0) this.wake();
    });
  }

  private load(): void {
    const restored: OutboxEntry[] = [];
    this.stored.clear();
    for (const [id, value] of this.options.store.records("outbox")) {
      const entry = restoredEntry(value);
      if (entry === undefined || entry.command.id !== id) continue;
      this.stored.add(id);
      restored.push(entry);
    }
    const sending = this.entries.find((entry) => entry.command.id === this.sending);
    this.merge(sending === undefined || restored.some((entry) => entry.command.id === sending.command.id) ? restored : [...restored, sending]);
  }

  private merge(entries: readonly OutboxEntry[]): void {
    const { kept, superseded } = coalesce(entries, this.sending);
    for (const entry of superseded) this.release(entry.command.id);
    this.entries = kept;
    for (const entry of kept) this.lastQueuedAt = Math.max(this.lastQueuedAt, entry.queuedAt);
  }

  private remote(change: LocalChange): void {
    if (change.kind === "cleared") {
      this.load();
      this.notify();
      void this.pump();
      return;
    }
    if (change.kind !== "record" || change.collection !== "outbox") return;
    const entry = restoredEntry(this.options.store.record("outbox", change.id));
    if (entry === undefined || entry.command.id !== change.id) {
      this.stored.delete(change.id);
      this.entries = this.entries.filter((queued) => queued.command.id !== change.id || queued.command.id === this.sending);
      this.notify();
      return;
    }
    this.stored.add(change.id);
    if (this.entries.some((queued) => queued.command.id === change.id)) return;
    this.merge([...this.entries, entry]);
    this.notify();
    void this.pump();
  }

  private remove(match: (command: RecordAnswerCommand) => boolean): void {
    const inFlight = this.entries.find((entry) => entry.command.id === this.sending && match(entry.command));
    if (inFlight !== undefined) this.retire(inFlight.command.id);
    const removed = this.entries.filter((entry) => match(entry.command) && entry.command.id !== this.sending);
    for (const entry of removed) this.release(entry.command.id);
    this.entries = this.entries.filter((entry) => !removed.includes(entry));
    this.notify();
  }

  private retire(id: Id): void {
    this.options.store.drop("outbox", id);
    this.stored.add(id);
  }

  private release(id: Id): void {
    this.options.store.drop("outbox", id);
    this.stored.delete(id);
  }

  private async pump(): Promise<void> {
    if (this.running || this.stopped) return;
    this.running = true;
    try {
      while (!this.stopped && this.entries.length > 0) {
        const cycle = new AbortController();
        this.cycle = cycle;
        const delivery = await retryWithJitteredBackoff(() => this.attempt(cycle.signal), { ...this.options.timing, signal: cycle.signal }).catch(() => undefined);
        if (delivery === null) break;
        if (delivery === undefined) continue;
        this.entries = this.entries.filter((entry) => entry.command.id !== delivery.command.id);
        this.release(delivery.command.id);
        this.failures = 0;
        this.notify();
        if (delivery.verdict !== undefined) this.options.onSettled?.(delivery.command, delivery.verdict);
      }
    } finally {
      this.running = false;
      this.cycle = undefined;
      this.update("idle");
    }
  }

  private async attempt(signal: AbortSignal): Promise<Delivery | null> {
    const head = this.entries[0];
    if (head === undefined) return null;
    if (this.stored.has(head.command.id) && this.options.store.record("outbox", head.command.id) === undefined) return { command: head.command, verdict: undefined };
    this.sending = head.command.id;
    this.update(this.failures === 0 ? "sending" : "retrying");
    try {
      return { command: head.command, verdict: await this.options.send(head.command, signal) };
    } catch (error) {
      if (signal.aborted) throw error;
      if (!isTransient(error)) return { command: head.command, verdict: { kind: "refused", detail: error instanceof Error ? error.message : String(error) } };
      this.failures += 1;
      this.update("retrying");
      throw error;
    } finally {
      this.sending = undefined;
    }
  }

  private update(activity: OutboxActivity): void {
    if (this.activity === activity && this.snapshot.failures === this.failures) return;
    this.activity = activity;
    this.notify();
  }

  private notify(): void {
    this.snapshot = this.freeze();
    for (const listener of [...this.listeners]) listener();
  }

  private freeze(): OutboxStatus {
    return { pending: this.entries.length, activity: this.activity, failures: this.failures };
  }
}
