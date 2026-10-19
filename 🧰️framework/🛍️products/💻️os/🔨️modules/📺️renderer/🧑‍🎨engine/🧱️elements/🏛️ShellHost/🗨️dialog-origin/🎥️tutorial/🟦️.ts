import type { MutationEnvelope } from "@semio-tech/framework/replication";
import type { ShellDialogOriginV1 } from "../🟦️.ts";

type TutorialDriveJobV1 = Readonly<{ owns: () => boolean; apply: (token: number) => Promise<void>; resolve: () => void; reject: (error: unknown) => void }>;

/** 🎞️ Only the latest driven operation can release or continue its UI ownership. */
export class TutorialDriveV1 {
  #next = 0;
  #active: number | null = null;
  #generation = 0;
  #running: Promise<void> | null = null;
  #pending: TutorialDriveJobV1 | null = null;
  #fault: unknown = null;
  get active(): boolean { return this.#active !== null; }
  get busy(): boolean { return this.#running !== null || this.#pending !== null || this.active; }
  get queued(): boolean { return this.#pending !== null; }
  claim(): number {
    if (this.#running !== null) throw new Error("Cannot replace an admitted tutorial document drive");
    this.#active = ++this.#next;
    return this.#active;
  }
  accepts(token: number): boolean { return this.#active === token; }
  release(token: number): void { if (this.accepts(token)) this.#active = null; }
  retire(): void {
    this.#generation++;
    this.#active = null;
    this.#fault = null;
    this.#pending?.resolve();
    this.#pending = null;
  }

  async drain(): Promise<void> {
    while (this.#running !== null || this.#pending !== null) {
      this.startNext();
      await this.#running?.catch(() => {});
    }
  }

  enqueue(owns: () => boolean, apply: (token: number) => Promise<void>): Promise<void> {
    if (!owns()) return Promise.resolve();
    if (this.#fault !== null) return Promise.reject(this.#fault);
    return new Promise((resolve, reject) => {
      this.#pending?.resolve();
      this.#pending = { owns, apply, resolve, reject };
      this.startNext();
    });
  }

  private startNext(): void {
    if (this.#running !== null || this.#pending === null) return;
    const job = this.#pending;
    this.#pending = null;
    const generation = this.#generation;
    const token = this.claim();
    const execution = Promise.resolve().then(async () => {
      if (job.owns() && this.accepts(token)) await job.apply(token);
    });
    this.#running = execution;
    const finish = (failed: boolean, error?: unknown) => {
      this.release(token);
      this.#running = null;
      if (failed) {
        if (this.#generation === generation) {
          this.#fault = error ?? new Error("Tutorial document drive failed");
          this.#pending?.reject(this.#fault);
          this.#pending = null;
        }
        job.reject(error);
      } else job.resolve();
      this.startNext();
    };
    void execution.then(() => finish(false), error => finish(true, error));
  }
}

/** ⏩️ Suspends playback while a seek owns its mutations, resuming only unchanged user intent. */
export async function runPausedTutorialSeekV1(
  clock: Readonly<{ isPlaying: () => boolean; pause: () => void; play: () => void }>,
  drive: TutorialDriveV1,
  owns: () => boolean,
  wantsPlaying: () => boolean,
  apply: (token: number) => Promise<void>,
): Promise<void> {
  if (!owns()) return;
  clock.pause();
  await drive.enqueue(owns, async token => {
    await apply(token);
    if (owns() && drive.accepts(token) && !drive.queued && wantsPlaying()) clock.play();
  });
}

/** ↩️ The concrete inverse of one document step a shell tutorial applied: the recorded inverse mutation envelopes of an edit, the
 * history action (undo, redo, checkout of the previous checkpoint, switch back to the previous alternative) that undoes a history
 * step, or `retained` for a step the append-only history keeps by design: a committed checkpoint changes no document content and
 * the history has no command that withdraws it, so its replay is the empty inverse. */
export type TutorialInverseV1 =
  | Readonly<{ kind: "mutations"; envelopes: readonly MutationEnvelope[] }>
  | Readonly<{ kind: "history"; action: "undo" | "redo" | "checkoutCheckpoint" | "switchAlternative"; args?: Readonly<Record<string, unknown>> }>
  | Readonly<{ kind: "retained"; step: "checkpoint" }>;

/** 🧭️ What the history already named before a history step ran: the checkpoint the document stood on and the alternative it followed. */
export type TutorialHistoryStandV1 = Readonly<{ checkpointId?: string; alternativeId?: string }>;

/** ↩️ The journal outcome of one forward history step: its concrete inverse, or the reason the history names no state to return to. A
 * checkpoint commit is retained by design; a checkout returns to the checkpoint the document stood on; an alternative switch returns to
 * the alternative it followed, and switching to the alternative the document already follows changes nothing. */
export function tutorialHistoryStepInverseV1(
  step: Readonly<{ kind: "checkpoint" } | { kind: "checkoutCheckpoint"; checkpointId: string } | { kind: "switchAlternative"; alternativeId: string }>,
  before: TutorialHistoryStandV1,
): Readonly<{ inverse: TutorialInverseV1 } | { unrevertable: string } | { unchanged: true }> {
  if (step.kind === "checkpoint") return { inverse: { kind: "retained", step: "checkpoint" } };
  if (step.kind === "checkoutCheckpoint") {
    return before.checkpointId === undefined
      ? { unrevertable: `checkoutCheckpoint ${step.checkpointId}: no checkpoint to return to` }
      : { inverse: { kind: "history", action: "checkoutCheckpoint", args: { checkpointId: before.checkpointId } } };
  }
  if (before.alternativeId === undefined) return { unrevertable: `switchAlternative ${step.alternativeId}: no alternative to return to` };
  if (before.alternativeId === step.alternativeId) return { unchanged: true };
  return { inverse: { kind: "history", action: "switchAlternative", args: { alternativeId: before.alternativeId } } };
}

/** 🧾️ One document step a run applied and the concrete inverse that undoes it: `step` is the step's own identity (the tutorial's
 * document event), `forward` the direction it was applied in, `inverse` what undoes it. Applying the opposite direction of the same
 * step cancels the entry instead of stacking a second one. */
export type TutorialJournalEntryV1<I> = Readonly<{ step: object; forward: boolean; inverse: I }>;

/** 🔌️ What a run needs from its document: to wait for admitted writes, to replay one recorded inverse, and to hydrate the
 * surfaces once the replay ended. `settle` also receives the reasons of the steps the run applied but could not journal an inverse
 * for (see {@link OwnedTutorialRunV1.recordUnrevertable}), so the shell can say what a stop left behind. */
export type TutorialDocumentPortV1<I> = Readonly<{
  drain: () => Promise<void>;
  revert: (inverse: I) => Promise<void>;
  settle: (unreverted: readonly string[]) => Promise<void>;
}>;

/** 🎬️ Ephemeral run ownership. A run never captures the document: it journals the concrete inverse of every document step it
 * applies, and stopping replays those inverses last to first through the document's own mutation path — a restore of a captured
 * archive is a load, never a revert. A step undone by its own opposite direction (a scrub forward and back) leaves no entry. A
 * step still in flight when the run stops journals itself before the replay starts: stopping first waits for admitted writes. */
export class OwnedTutorialRunV1<I> {
  #closed = false;
  #ready = false;
  #draining = false;
  #journal: TutorialJournalEntryV1<I>[] = [];
  #unreverted: string[] = [];
  #starting: Promise<boolean> | null = null;
  #stopping: Promise<void> | null = null;

  constructor(
    readonly tutorialId: string,
    readonly origin: ShellDialogOriginV1,
    private readonly owns: () => boolean,
    private readonly port: TutorialDocumentPortV1<I>,
  ) {}

  get ready(): boolean { return this.#ready && this.isCurrent(); }

  get pending(): number { return this.#journal.length; }

  get unreverted(): readonly string[] { return this.#unreverted; }

  isCurrent(): boolean { return !this.#closed && this.owns(); }

  start(): Promise<boolean> {
    if (this.#starting !== null) return this.#starting;
    const current = this.isCurrent();
    this.#ready = current;
    this.#starting = Promise.resolve(current);
    return this.#starting;
  }

  record(step: object, forward: boolean, inverse: I): void {
    if (!(this.#draining ? this.owns() : this.#ready && this.isCurrent())) return;
    const last = this.#journal.at(-1);
    if (last !== undefined && last.step === step && last.forward !== forward) {
      this.#journal.pop();
      return;
    }
    this.#journal.push({ step, forward, inverse });
  }

  /** 🚫️ Notes a document step the run applied whose concrete inverse is unknown (e.g. a checkout from a state that names no
   * checkpoint to return to): stopping cannot replay it, so it is reported to `settle` instead of being silently dropped. */
  recordUnrevertable(reason: string): void {
    if (!(this.#draining ? this.owns() : this.#ready && this.isCurrent())) return;
    this.#unreverted.push(reason);
  }

  stop(): Promise<void> {
    if (this.#stopping !== null) return this.#stopping;
    const owned = this.owns();
    this.#closed = true;
    this.#ready = false;
    this.#draining = owned;
    this.#stopping = this.port.drain().then(async () => {
      this.#draining = false;
      const journal = this.#journal;
      const unreverted = this.#unreverted;
      this.#journal = [];
      this.#unreverted = [];
      if (!owned || (journal.length === 0 && unreverted.length === 0) || !this.owns()) return;
      for (let index = journal.length - 1; index >= 0; index -= 1) await this.port.revert(journal[index]!.inverse);
      await this.port.settle(unreverted);
    });
    return this.#stopping;
  }
}
