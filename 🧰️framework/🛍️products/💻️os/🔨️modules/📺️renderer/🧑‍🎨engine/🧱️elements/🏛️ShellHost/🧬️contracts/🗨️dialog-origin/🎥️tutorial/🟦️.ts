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

export type TutorialSnapshotV1 = Readonly<{ pack: Uint8Array; spr: Uint8Array }>;
export type TutorialDocumentPortV1 = Readonly<{
  read: () => Promise<TutorialSnapshotV1 | null>;
  drain: () => Promise<void>;
  restore: (snapshot: TutorialSnapshotV1) => Promise<void>;
}>;

/** 🎬️ Ephemeral run ownership keeps delayed snapshots and restores on their original document port. */
export class OwnedTutorialRunV1 {
  #closed = false;
  #ready = false;
  #snapshot: TutorialSnapshotV1 | null = null;
  #starting: Promise<boolean> | null = null;
  #stopping: Promise<void> | null = null;

  constructor(
    readonly tutorialId: string,
    readonly origin: ShellDialogOriginV1,
    private readonly owns: () => boolean,
    private readonly port: TutorialDocumentPortV1,
  ) {}

  get ready(): boolean { return this.#ready && this.isCurrent(); }

  isCurrent(): boolean { return !this.#closed && this.owns(); }

  start(): Promise<boolean> {
    if (this.#starting !== null) return this.#starting;
    this.#starting = this.readSnapshot();
    return this.#starting;
  }

  private async readSnapshot(): Promise<boolean> {
    if (!this.isCurrent()) return false;
    const snapshot = await this.port.read();
    if (!this.isCurrent()) return false;
    this.#snapshot = snapshot;
    this.#ready = true;
    return true;
  }

  stop(): Promise<void> {
    if (this.#stopping !== null) return this.#stopping;
    const snapshot = this.owns() ? this.#snapshot : null;
    this.#closed = true;
    this.#ready = false;
    this.#snapshot = null;
    this.#stopping = this.port.drain().then(async () => { if (snapshot !== null && this.owns()) await this.port.restore(snapshot); });
    return this.#stopping;
  }
}
