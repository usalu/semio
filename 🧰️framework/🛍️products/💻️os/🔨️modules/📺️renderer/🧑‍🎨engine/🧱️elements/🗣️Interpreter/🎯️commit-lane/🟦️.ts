import type { InputOutcomeV1 } from "../../🏛️ShellHost/🎯️input-ledger/🟦️.ts";
import type { LocalDocumentOwnerV1 } from "../🧭️local-document-owner/🟦️.ts";

export type InputCommitSnapshotV1 = Readonly<{ target: string; value: string; revision: string }>;
export type InputCommitRequestV1 = Readonly<{
  read: () => InputCommitSnapshotV1 | null;
  send: () => void | Promise<void | InputOutcomeV1>;
  subscribe: (listener: () => void) => () => void;
  active: () => boolean;
  expected: string;
}>;
type Pending = { request: InputCommitRequestV1; base: InputCommitSnapshotV1; expectedRevision: string; allowed: Set<string>; resolve: (outcome: InputOutcomeV1) => void; reject: (error: Error) => void };

class InputCommitCancelledV1 extends Error {}

const lanes = new WeakMap<LocalDocumentOwnerV1, InputCommitLaneV1>();
export const INPUT_COMMIT_CAPACITY_V1 = 64;
const exact = (value: unknown): value is string => typeof value === "string" && /^(0|[1-9][0-9]*)$/u.test(value) && BigInt(value) <= 0xffff_ffff_ffff_ffffn;

/** 🪟️ Every projection of one live document shares its canonical mutation order. */
export function inputCommitLaneV1(owner: LocalDocumentOwnerV1): InputCommitLaneV1 {
  let lane = lanes.get(owner);
  if (!lane) { lane = new InputCommitLaneV1(owner); lanes.set(owner, lane); }
  return lane;
}

/** 🚦️ Serializes explicit field edits across exact native completion and retained publication. */
export class InputCommitLaneV1 {
  private readonly queue: Pending[] = [];
  private running = false;

  constructor(private readonly owner: LocalDocumentOwnerV1) {}

  submit(request: InputCommitRequestV1): Promise<InputOutcomeV1> {
    const base = request.read();
    if (!this.owner.active || !request.active() || !base || !exact(base.revision)) return Promise.reject(new Error("Input owner or target retired"));
    if (this.queue.length + Number(this.running) >= INPUT_COMMIT_CAPACITY_V1) return Promise.reject(new Error("Input commit queue is full"));
    return new Promise((resolve, reject) => { this.queue.push({ request, base, expectedRevision: base.revision, allowed: new Set([base.revision]), resolve, reject }); void this.drain(); });
  }

  private prepare(entry: Pending): InputCommitSnapshotV1 | Promise<InputCommitSnapshotV1> {
    const inspect = (): InputCommitSnapshotV1 | null => {
      const current = entry.request.read();
      if (this.owner.active && !entry.request.active()) throw new InputCommitCancelledV1("Queued input was discarded");
      if (!this.owner.active || !current || current.target !== entry.base.target || current.value !== entry.base.value || !entry.allowed.has(current.revision)) throw new Error("Queued input changed before dispatch");
      return current.revision === entry.expectedRevision ? current : null;
    };
    const current = inspect();
    if (current) return current;
    return new Promise((resolve, reject) => {
      let done = false;
      let unsubscribe = () => {};
      let retire = () => {};
      const finish = (value: InputCommitSnapshotV1 | Error) => { if (done) return; done = true; unsubscribe(); retire(); if (value instanceof Error) reject(value); else resolve(value); };
      const check = () => { try { const next = inspect(); if (next) finish(next); } catch (error) { finish(error instanceof Error ? error : new Error(String(error))); } };
      unsubscribe = entry.request.subscribe(check);
      retire = this.owner.subscribeRetirement(check);
      check();
    });
  }

  private complete(request: InputCommitRequestV1, base: InputCommitSnapshotV1): Promise<InputOutcomeV1> {
    return new Promise((resolve, reject) => {
      const admitted = request.read();
      if (this.owner.active && !request.active()) { reject(new InputCommitCancelledV1("Queued input was discarded")); return; }
      if (!this.owner.active || !admitted || admitted.target !== base.target || admitted.value !== base.value || admitted.revision !== base.revision) { reject(new Error("Input changed at dispatch")); return; }
      let done = false;
      let settled = false;
      let outcome: void | InputOutcomeV1;
      let unsubscribe = () => {};
      let retire = () => {};
      const finish = (error?: Error) => { if (done) return; done = true; unsubscribe(); retire(); if (error) reject(error); else resolve(outcome!); };
      const check = () => {
        if (done) return;
        if (!this.owner.active) { finish(new Error("Input document retired")); return; }
        if (!settled) return;
        if (!outcome || outcome.kind !== "applied" || !outcome.commit || !exact(outcome.commit.operation) || !exact(outcome.commit.revision)) { finish(new Error("Input did not return a native commit receipt")); return; }
        const current = request.read();
        if (!current || current.target !== base.target) { finish(new Error("Input target changed")); return; }
        if (current.revision === outcome.commit.revision) {
          if (current.value !== request.expected) { finish(new Error("Input publication changed the requested value")); return; }
          finish();
        } else if (current.revision !== base.revision) finish(new Error("Input publication belongs to another mutation"));
      };
      unsubscribe = request.subscribe(check);
      retire = this.owner.subscribeRetirement(check);
      check();
      if (done) return;
      try { Promise.resolve(request.send()).then((value) => { outcome = value; settled = true; check(); }, (error) => finish(error instanceof Error ? error : new Error(String(error)))); }
      catch (error) { finish(error instanceof Error ? error : new Error(String(error))); }
    });
  }

  private async drain(): Promise<void> {
    if (this.running) return;
    this.running = true;
    try {
      while (this.queue.length > 0) {
        const entry = this.queue.shift()!;
        try {
          const prepared = this.prepare(entry);
          const current = prepared instanceof Promise ? await prepared : prepared;
          const outcome = await this.complete(entry.request, current);
          if (outcome.kind === "applied" && outcome.commit) for (const queued of this.queue) {
            if (queued.expectedRevision === current.revision) { queued.allowed.add(outcome.commit.revision); queued.expectedRevision = outcome.commit.revision; }
          }
          entry.resolve(outcome);
        } catch (error) {
          const refusal = error instanceof Error ? error : new Error(String(error));
          entry.reject(refusal);
          if (!(refusal instanceof InputCommitCancelledV1)) for (const queued of this.queue.splice(0)) queued.reject(refusal);
        }
      }
    } finally { this.running = false; }
  }
}
