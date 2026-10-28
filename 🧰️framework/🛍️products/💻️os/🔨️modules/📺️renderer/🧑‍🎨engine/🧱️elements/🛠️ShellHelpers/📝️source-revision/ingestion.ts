import type { SourceRevisionWorkV1 } from "./work.ts";

type Begin = () => SourceRevisionWorkV1 | null;

/** 📬️ Transfers a queued document hold only after original receiving holds have been admitted. */
export class DocumentSourceIngestionV1<Entry extends object> {
  readonly #queued = new WeakMap<Entry, SourceRevisionWorkV1>();

  queued(entry: Entry, begin: Begin): void {
    if (this.#queued.has(entry)) return;
    const work = begin();
    if (work) this.#queued.set(entry, work);
  }

  drained(entry: Entry): void {
    const work = this.#queued.get(entry);
    this.#queued.delete(entry);
    work?.admit(undefined);
    work?.finish();
  }

  async receive<Result>(begin: Begin, produce: () => Promise<Result>): Promise<Result> {
    const work = begin();
    try {
      const result = await produce();
      work?.admit(undefined);
      return result;
    } finally {
      work?.finish();
    }
  }
}
