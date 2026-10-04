/** 📮️ The persisted local-only outbox of the commands the proctor has not decided yet, shared by every tab of the site.
 *
 * An answer applies locally at once and its `record-answer` command waits here until the proctor has decided it; so does
 * every command the device decided by itself while the proctor was away (a registration, a started run, an opened task,
 * a submission). A stored command comes back only whole: a start with its challenge, an opening and an answer with the
 * instant the learner acted.
 * Each command is its own storage record (keyed by command id), so tabs never overwrite each other's queued commands;
 * every tab merges the others' records as they appear, change or disappear, so each holds the union of all tabs' queues
 * in queued-at order. Per (run, task) only the latest queued answer stays — the proctor keeps the latest anyway —
 * except that a command already on the wire is never replaced, so a delivered answer is never mistaken for a newer one;
 * no other command is ever superseded. Delivery is one at a time in queued-at order — a run is started before its
 * answers arrive, a task is opened before its answers and a run is submitted after them — retried with jittered backoff through connection shortages under the
 * command's own id (the idempotency key), so the same command delivered by two tabs is applied once. Before every
 * attempt a tab checks that the record still exists: another tab may have delivered or superseded it meanwhile. A
 * proctor that asks to slow down (a rate limit) is not asked again before its wait is over — neither by the backoff nor
 * by {@link Outbox.wake}.
 *
 * @see ../../../../../../🔨️modules/⏳️async/🔁️jittered-backoff/🟦️.ts
 * @see ../💾️persistence/🟦️.ts
 */

import { retryWithJitteredBackoff } from "@semio-tech/framework";
import type { Answer, Command, Id, RecordAnswerCommand, Slug } from "@semio-tech/quiz";
import { isChallenge, isRecord, isTimestamp, type LocalChange, type LocalStore } from "../💾️persistence/🟦️.ts";
import { isTransient, pause, retryWait, type CommandVerdict, type RetryTiming } from "../🛂️proctor/🟦️.ts";

/** 📮️ One queued command and when it was queued. */
export interface OutboxEntry {
  readonly command: Command;
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
  readonly send: (command: Command, signal: AbortSignal) => Promise<CommandVerdict>;
  readonly store: LocalStore;
  readonly timing: RetryTiming;
  readonly now?: () => number;
  readonly onSettled?: (command: Command, verdict: CommandVerdict) => void;
}

interface Delivery {
  readonly command: Command;
  readonly verdict: CommandVerdict | undefined;
}

/** 🔑️ The coalescing key of an answer: its run and task. */
export function coalescingKey(command: Pick<RecordAnswerCommand, "run" | "task">): string {
  return `${command.run}/${command.task}`;
}

/** 🏃️ The run a command is about; a registration is about none. */
export function commandRun(command: Command): Id | undefined {
  return command.type === "identify-learner" ? undefined : command.run;
}

function supersedable(command: Command): string | undefined {
  return command.type === "record-answer" ? coalescingKey(command) : undefined;
}

function restoredCommand(value: unknown): Command | undefined {
  if (!isRecord(value) || typeof value.id !== "string" || typeof value.learner !== "string") return undefined;
  switch (value.type) {
    case "identify-learner":
      return isRecord(value.identity) && typeof value.identity.kind === "string" ? (value as unknown as Command) : undefined;
    case "start-run":
      return typeof value.run === "string" && typeof value.quiz === "string" && isChallenge(value.challenge) && isTimestamp(value.at) ? (value as unknown as Command) : undefined;
    case "open-task":
      return typeof value.run === "string" && typeof value.task === "string" && isTimestamp(value.at) ? (value as unknown as Command) : undefined;
    case "record-answer":
      return typeof value.run === "string" && typeof value.task === "string" && isRecord(value.answer) && isTimestamp(value.at) ? (value as unknown as Command) : undefined;
    case "submit-run":
      return typeof value.run === "string" ? (value as unknown as Command) : undefined;
    default:
      return undefined;
  }
}

function restoredEntry(value: unknown): OutboxEntry | undefined {
  if (!isRecord(value) || typeof value.queuedAt !== "number") return undefined;
  const command = restoredCommand(value.command);
  return command === undefined ? undefined : { command, queuedAt: value.queuedAt };
}

function queueOrder(left: OutboxEntry, right: OutboxEntry): number {
  return left.queuedAt - right.queuedAt || (left.command.id < right.command.id ? -1 : left.command.id > right.command.id ? 1 : 0);
}

/** 🧹️ `entries` in queued-at order with, per (run, task), only the latest answer kept — plus `keep` (the command on the
 * wire), which is never dropped; the rest is returned as superseded. Commands other than answers are all kept. */
export function coalesce(entries: readonly OutboxEntry[], keep?: Id): { readonly kept: readonly OutboxEntry[]; readonly superseded: readonly OutboxEntry[] } {
  const ordered = [...entries].sort(queueOrder);
  const latest = new Map(ordered.flatMap((entry) => (supersedable(entry.command) === undefined ? [] : [[supersedable(entry.command)!, entry.command.id] as const])));
  const stays = (entry: OutboxEntry): boolean => entry.command.id === keep || supersedable(entry.command) === undefined || latest.get(supersedable(entry.command)!) === entry.command.id;
  return { kept: ordered.filter(stays), superseded: ordered.filter((entry) => !stays(entry)) };
}

/** 📮️ The command outbox of one quiz client tab. */
export class Outbox {
  private readonly options: OutboxOptions;
  private entries: readonly OutboxEntry[] = [];
  private readonly stored = new Set<Id>();
  private sending: Id | undefined;
  private activity: OutboxActivity = "idle";
  private failures = 0;
  private lastQueuedAt = 0;
  private heldUntil = 0;
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
  queued(run?: Id): readonly Command[] {
    return this.entries.filter((entry) => run === undefined || commandRun(entry.command) === run).map((entry) => entry.command);
  }

  /** ⏳️ Whether a queued command of any tab is one `wanted` asks for. */
  waiting(wanted: (command: Command) => boolean): boolean {
    return this.entries.some((entry) => wanted(entry.command));
  }

  /** ✍️ The latest queued answer per task of `run`. */
  pendingAnswers(run: Id): Readonly<Record<Slug, Answer>> {
    return Object.fromEntries(this.queued(run).flatMap((command) => (command.type === "record-answer" ? [[command.task, command.answer] as const] : [])));
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

  /** ➕️ Queues `command` as its own record; an answer supersedes every queued (not in-flight) answer to the same run
   * and task. Its queued-at time is later than that of every command this tab knows of, so it is the latest everywhere. */
  enqueue(command: Command): void {
    this.lastQueuedAt = Math.max((this.options.now ?? Date.now)(), this.lastQueuedAt + 1);
    const entry: OutboxEntry = { command, queuedAt: this.lastQueuedAt };
    const key = supersedable(command);
    const superseded = key === undefined ? [] : this.entries.filter((queued) => queued.command.id !== this.sending && supersedable(queued.command) === key);
    this.options.store.put("outbox", command.id, entry);
    if (this.options.store.record("outbox", command.id) !== undefined) this.stored.add(command.id);
    for (const queued of superseded) this.release(queued.command.id);
    this.entries = [...this.entries.filter((queued) => !superseded.includes(queued)), entry].sort(queueOrder);
    this.notify();
    void this.pump();
  }

  /** 🗑️ Drops every command of `run`, e.g. once the run was submitted or voided elsewhere; one already in flight
   * finishes its attempt but is never retried nor restored. */
  discard(run: Id): void {
    this.remove((command) => commandRun(command) === run);
  }

  /** 🗑️ Drops every command of `learner`, e.g. once the proctor no longer knows them (in flight: as {@link discard}). */
  forget(learner: Id): void {
    this.remove((command) => command.learner === learner);
  }

  /** 🪪️ Re-addresses every queued command of the learner `from` to the learner `to` under its own id — the one on the
   * wire stays as it was sent. For a device whose learner turned out to be one the proctor already knows. */
  reassign(from: Id, to: Id): void {
    const moved = this.entries.filter((entry) => entry.command.learner === from && entry.command.id !== this.sending).map((entry): OutboxEntry => ({ ...entry, command: { ...entry.command, learner: to } }));
    if (moved.length === 0) return;
    for (const entry of moved) this.options.store.put("outbox", entry.command.id, entry);
    this.entries = this.entries.map((entry) => moved.find((changed) => changed.command.id === entry.command.id) ?? entry);
    this.notify();
  }

  /** ⏰️ Retries now instead of waiting out the current backoff (e.g. when the browser reports it is online again) —
   * but never before the wait a rate-limiting proctor asked for is over. */
  wake(): void {
    if (this.sending === undefined) this.cycle?.abort();
    void this.pump();
  }

  /** ⏳️ Resolves once no command of `run` is queued, reporting `(delivered, total)`; rejects when `signal` aborts. */
  settled(run: Id, signal?: AbortSignal, onProgress?: (delivered: number, total: number) => void): Promise<void> {
    const remaining = (): number => this.entries.filter((entry) => commandRun(entry.command) === run).length;
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
    const known = this.entries.find((queued) => queued.command.id === change.id);
    if (known !== undefined && (change.id === this.sending || JSON.stringify(known) === JSON.stringify(entry))) return;
    this.merge([...this.entries.filter((queued) => queued !== known), entry]);
    this.notify();
    void this.pump();
  }

  private remove(match: (command: Command) => boolean): void {
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
    const now = this.options.now ?? Date.now;
    if (this.heldUntil > now()) await pause(this.heldUntil - now(), signal);
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
      this.heldUntil = now() + retryWait(error);
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
