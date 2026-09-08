import type { ShellDialogOriginV1 } from "../🟦️.ts";

export type TutorialSnapshotV1 = Readonly<{ pack: Uint8Array; spr: Uint8Array }>;
export type TutorialDocumentPortV1 = Readonly<{
  read: () => Promise<TutorialSnapshotV1 | null>;
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
    this.#stopping = snapshot === null ? Promise.resolve() : this.port.restore(snapshot);
    return this.#stopping;
  }
}
