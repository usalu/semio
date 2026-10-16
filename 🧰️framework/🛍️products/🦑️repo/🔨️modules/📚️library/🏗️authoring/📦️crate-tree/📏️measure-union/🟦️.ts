/** 📏️ Incremental crate measurement: the lower bound of a merged closure from the per-file entries of the closures it merges, so the packing loop re-reads sources only for merges that can fit.
 * A file keeps its path in every closure that reaches it and only ever loses text when a declaration is cut out of it, so its size in a merged closure is at least its size in each member closure. */

/** 📄️ Counter contribution of one source or embedded input. */
export type FileMeasure = Readonly<{ bytes: number; lines: number }>;

/** 🧮️ Counter totals of a set of inputs. */
export type MeasureTotals = Readonly<{ files: number; bytes: number; lines: number }>;

/** 🔢️ Whether totals are within a budget. */
export function totalsFit(totals: MeasureTotals, budget: MeasureTotals): boolean {
  return totals.files <= budget.files && totals.bytes <= budget.bytes && totals.lines <= budget.lines;
}

/** 🧰️ Union of per-file entries that keeps the largest size seen per path. Paths in `excluded` (the virtual mirror root, whose text depends on the group) never count as entries; `floor` is what the excluded files contribute at least. */
export class MeasureUnion {
  private readonly entries = new Map<string, FileMeasure>();
  private files: number;
  private bytes: number;
  private lines: number;

  constructor(private readonly excluded: ReadonlySet<string> = new Set(), floor: MeasureTotals = { files: 0, bytes: 0, lines: 0 }) {
    this.files = floor.files;
    this.bytes = floor.bytes;
    this.lines = floor.lines;
  }

  /** 🧮️ Totals of everything added so far. */
  totals(): MeasureTotals {
    return { files: this.files, bytes: this.bytes, lines: this.lines };
  }

  /** 🔭️ Totals after adding `parts`, without keeping them. */
  peek(parts: Iterable<ReadonlyMap<string, FileMeasure>>): MeasureTotals {
    const delta = new Map<string, FileMeasure>();
    let { files, bytes, lines } = this;
    for (const part of parts) for (const [path, measure] of part) {
      if (this.excluded.has(path)) continue;
      const current = delta.get(path) ?? this.entries.get(path);
      if (!current) {
        files += 1;
        bytes += measure.bytes;
        lines += measure.lines;
        delta.set(path, measure);
        continue;
      }
      if (measure.bytes <= current.bytes && measure.lines <= current.lines) continue;
      const next = { bytes: Math.max(current.bytes, measure.bytes), lines: Math.max(current.lines, measure.lines) };
      bytes += next.bytes - current.bytes;
      lines += next.lines - current.lines;
      delta.set(path, next);
    }
    return { files, bytes, lines };
  }

  /** ➕️ Keeps `parts` in the union. */
  add(parts: Iterable<ReadonlyMap<string, FileMeasure>>): void {
    for (const part of parts) for (const [path, measure] of part) {
      if (this.excluded.has(path)) continue;
      const current = this.entries.get(path);
      if (!current) {
        this.entries.set(path, measure);
        this.files += 1;
        this.bytes += measure.bytes;
        this.lines += measure.lines;
        continue;
      }
      if (measure.bytes <= current.bytes && measure.lines <= current.lines) continue;
      const next = { bytes: Math.max(current.bytes, measure.bytes), lines: Math.max(current.lines, measure.lines) };
      this.bytes += next.bytes - current.bytes;
      this.lines += next.lines - current.lines;
      this.entries.set(path, next);
    }
  }
}
