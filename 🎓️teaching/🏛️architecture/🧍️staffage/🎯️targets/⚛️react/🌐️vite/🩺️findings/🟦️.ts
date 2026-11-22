/** 🩺️ The findings of authored asset texts, computed on threads of their own: one check per revision however many ask, its progress reported to everyone who asked, its result kept for the revision, and stopped once nobody waits for it.
 *
 * A check that nobody waits for any more leaves the queue, or its thread is ended when it already runs; the next question starts a fresh thread. Idle threads end after {@link IDLE_MS}.
 *
 * @see ./🧵️worker/🟦️.ts — the thread
 * @see ../🟦️.ts — the plugin that asks */
// #region 🔌️Adapters
import { availableParallelism } from "node:os";
import { resolve } from "node:path";
import { Worker } from "node:worker_threads";
// #endregion 🔌️Adapters
import type { Issue } from "../../../../🔨️modules/🗂️catalog/🟦️.ts";
import type { Text } from "../../../../🧬️schema/🟦️.ts";
import type { FindingsAnswer, FindingsQuestion } from "./🧵️worker/🟦️.ts";

/** 📶️ How far one check has come. */
export type FindingsProgress = (done: number, total: number, label: Text) => void;

/** 🎫️ One question: `findings` settles with the findings, or rejects with an `AbortError` when everyone who asked cancelled; `cancel` withdraws this question alone. */
export type FindingsTicket = { readonly findings: Promise<readonly Issue[]>; readonly cancel: () => void };

/** 🧵️ What the service needs of a thread; tests hand in their own. */
export type FindingsThread = { post(question: FindingsQuestion): void; listen(answer: (answer: FindingsAnswer) => void, failure: (error: unknown) => void): void; end(): void; hold(busy: boolean): void };

/** ⚙️ How many threads check at once, how many finished revisions are kept and how threads are made. */
export type FindingsOptions = { readonly threads?: number; readonly kept?: number; readonly thread?: () => FindingsThread };

/** 🛑️ The single finding of a text whose check stopped before it finished. */
export const CHECK_THREW: Issue = { path: "$", code: "validation-threw", message: { en: "The check of this asset stopped before it finished.", de: "Die Prüfung dieses Assets brach vorzeitig ab." } };

/** ⏳️ How long an idle thread is kept, in milliseconds. */
export const IDLE_MS = 30_000;

type Job = { readonly id: number; readonly revision: string; readonly text: string; readonly waiting: Set<FindingsProgress>; readonly findings: Promise<readonly Issue[]>; readonly settle: (issues: readonly Issue[]) => void; readonly fail: (error: unknown) => void; last?: readonly [number, number, Text]; thread?: Lane };
type Lane = { readonly thread: FindingsThread; job: Job | null; idle?: ReturnType<typeof setTimeout> };

function nativeThread(): FindingsThread {
  const strips = "bun" in process.versions || (process.features as { typescript?: unknown }).typescript === "transform";
  const worker = new Worker(resolve(import.meta.dirname, "🧵️worker", "🟦️.ts"), { execArgv: strips ? process.execArgv : [...process.execArgv, "--experimental-transform-types", "--disable-warning=ExperimentalWarning"] });
  worker.unref();
  return {
    post: (question) => worker.postMessage(question),
    listen: (answer, failure) => {
      worker.on("message", answer);
      worker.on("error", failure);
      worker.on("exit", (code) => void (code === 0 || failure(new Error(`The checking thread ended with code ${code}`))));
    },
    end: () => void worker.terminate(),
    hold: (busy) => (busy ? worker.ref() : worker.unref()),
  };
}

/** 🧮️ The default number of checking threads: a quarter of the processors, at least one and at most four, so the server and the reader's browser keep theirs. */
export const defaultFindingsThreads = (): number => Math.max(1, Math.min(4, Math.floor(availableParallelism() / 4)));

/** 🩺️ The service: `ask` for the findings of a text under its revision, `close` to end every thread. */
export class StaffageFindings {
  readonly #jobs = new Map<string, Job>();
  readonly #done = new Map<string, readonly Issue[]>();
  readonly #queue: Job[] = [];
  readonly #lanes = new Set<Lane>();
  readonly #threads: number;
  readonly #kept: number;
  readonly #thread: () => FindingsThread;
  #next = 0;

  constructor(options: FindingsOptions = {}) {
    this.#threads = options.threads ?? defaultFindingsThreads();
    this.#kept = options.kept ?? 1024;
    this.#thread = options.thread ?? nativeThread;
  }

  /** 🔢️ The revisions being checked or waiting for a thread. */
  get pending(): readonly string[] {
    return [...this.#jobs.keys()];
  }

  /** ❓️ Asks for the findings of `text`, whose revision is `revision`; `progress` hears how far the check is, starting with where it already is. */
  ask(revision: string, text: string, progress?: FindingsProgress): FindingsTicket {
    const kept = this.#done.get(revision);
    if (kept !== undefined) return { findings: Promise.resolve(kept), cancel: () => undefined };
    let job = this.#jobs.get(revision);
    if (job === undefined) {
      const { promise, resolve, reject } = Promise.withResolvers<readonly Issue[]>();
      promise.catch(() => undefined);
      job = { id: this.#next++, revision, text, waiting: new Set(), findings: promise, settle: resolve, fail: reject };
      this.#jobs.set(revision, job);
      this.#queue.push(job);
    }
    const asked = job;
    const entry: FindingsProgress = (done, total, label) => progress?.(done, total, label);
    asked.waiting.add(entry);
    if (asked.last !== undefined) entry(...asked.last);
    this.#pump();
    return {
      findings: asked.findings,
      cancel: () => {
        if (!asked.waiting.delete(entry) || asked.waiting.size > 0 || this.#jobs.get(revision) !== asked) return;
        this.#drop(asked, new DOMException("The check was cancelled.", "AbortError"));
      },
    };
  }

  /** 🚪️ Withdraws every question and ends every thread. */
  async close(): Promise<void> {
    this.#queue.length = 0;
    for (const job of [...this.#jobs.values()]) this.#drop(job, new DOMException("The check was cancelled.", "AbortError"));
    for (const lane of [...this.#lanes]) this.#end(lane);
  }

  #drop(job: Job, reason: unknown): void {
    this.#jobs.delete(job.revision);
    const queued = this.#queue.indexOf(job);
    if (queued >= 0) this.#queue.splice(queued, 1);
    if (job.thread !== undefined) this.#end(job.thread);
    job.fail(reason);
    this.#pump();
  }

  #end(lane: Lane): void {
    clearTimeout(lane.idle);
    this.#lanes.delete(lane);
    lane.thread.end();
  }

  #pump(): void {
    while (this.#queue.length > 0) {
      let lane = [...this.#lanes].find((candidate) => candidate.job === null);
      if (lane === undefined) {
        if (this.#lanes.size >= this.#threads) return;
        lane = this.#open();
      }
      const job = this.#queue.shift()!;
      clearTimeout(lane.idle);
      lane.job = job;
      job.thread = lane;
      lane.thread.hold(true);
      lane.thread.post({ id: job.id, text: job.text });
    }
  }

  #open(): Lane {
    const lane: Lane = { thread: this.#thread(), job: null };
    this.#lanes.add(lane);
    lane.thread.listen(
      (answer) => {
        const job = lane.job;
        if (job === null || answer.id !== job.id) return;
        if ("progress" in answer) {
          job.last = answer.progress;
          for (const listener of job.waiting) listener(...answer.progress);
          return;
        }
        lane.job = null;
        lane.thread.hold(false);
        this.#jobs.delete(job.revision);
        this.#done.set(job.revision, answer.issues);
        if (this.#done.size > this.#kept) this.#done.delete(this.#done.keys().next().value!);
        job.settle(answer.issues);
        lane.idle = setTimeout(() => lane.job === null && this.#end(lane), IDLE_MS);
        (lane.idle as { unref?: () => void }).unref?.();
        this.#pump();
      },
      (error) => {
        const job = lane.job;
        lane.job = null;
        this.#end(lane);
        if (job === null || this.#jobs.get(job.revision) !== job) return;
        delete job.thread;
        this.#drop(job, error);
      },
    );
    return lane;
  }
}
