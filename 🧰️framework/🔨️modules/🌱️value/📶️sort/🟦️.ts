export type HeapSortWork = { kind: "advance" | "complete" } | { kind: "compare" | "swap"; left: number; right: number };
type Phase = "prepare" | "child" | "compare" | "swap" | "extract" | "complete";

export class HeapSortCursor {
  private end: number;
  private build: number;
  private root = 0;
  private child = 0;
  private building = true;
  private phase: Phase;
  constructor(readonly inputLength: number) {
    if (!Number.isSafeInteger(inputLength) || inputLength < 0) throw new RangeError("sort length must be a nonnegative exact integer");
    this.end = inputLength;
    this.build = Math.floor(inputLength / 2);
    this.phase = inputLength < 2 ? "complete" : "prepare";
  }
  snapshot(): readonly number[] { return [this.inputLength, this.end, this.build, this.root, this.child, Number(this.building), ["prepare", "child", "compare", "swap", "extract", "complete"].indexOf(this.phase), 1]; }
  static fromSnapshot(state: readonly number[]): HeapSortCursor {
    if (state.length !== 8 || state[7] !== 1 || state.some(value => !Number.isSafeInteger(value) || value < 0) || state[5]! > 1 || state[6]! > 5) throw new RangeError("invalid sort snapshot");
    const cursor = new HeapSortCursor(state[0]!);
    cursor.end = state[1]!; cursor.build = state[2]!; cursor.root = state[3]!; cursor.child = state[4]!; cursor.building = state[5] !== 0;
    cursor.phase = (["prepare", "child", "compare", "swap", "extract", "complete"] as const)[state[6]!]!;
    if (cursor.end > cursor.inputLength || cursor.build > Math.floor(cursor.inputLength / 2) || cursor.root >= Math.max(1, cursor.inputLength) || cursor.child >= Math.max(1, cursor.inputLength) || (cursor.phase !== "complete" && (cursor.inputLength < 2 || cursor.end === 0)) || ((cursor.phase === "compare" || cursor.phase === "swap") && (cursor.root >= cursor.end || cursor.child >= cursor.end))) throw new RangeError("invalid sort snapshot bounds");
    return cursor;
  }
  private leftChild(): number | null {
    const left = 2 * this.root + 1;
    return Number.isSafeInteger(left) && left < this.end ? left : null;
  }
  private finishSift(): void { this.phase = this.building ? "prepare" : "extract"; }
  next(): HeapSortWork {
    if (this.phase === "complete") return { kind: "complete" };
    if (this.phase === "compare") return { kind: "compare", left: this.root, right: this.child };
    if (this.phase === "swap") return { kind: "swap", left: this.root, right: this.child };
    if (this.phase === "extract" && this.end > 1) return { kind: "swap", left: 0, right: this.end - 1 };
    const left = this.phase === "child" ? this.leftChild() : null;
    if (left !== null && left + 1 < this.end) return { kind: "compare", left, right: left + 1 };
    return { kind: "advance" };
  }
  advance(comparison?: -1 | 0 | 1): void {
    const next = this.next();
    if (next.kind === "complete") throw new Error("sort cursor is complete");
    if ((next.kind === "compare") !== (comparison !== undefined)) throw new Error("sort observation does not match pending work");
    switch (this.phase) {
      case "prepare":
        if (this.build > 0) { this.root = --this.build; this.phase = "child"; }
        else { this.building = false; this.phase = "extract"; }
        break;
      case "child": {
        const left = this.leftChild();
        if (left === null) this.finishSift();
        else { this.child = comparison === -1 ? left + 1 : left; this.phase = "compare"; }
        break;
      }
      case "compare": if (comparison === -1) this.phase = "swap"; else this.finishSift(); break;
      case "swap": this.root = this.child; this.phase = "child"; break;
      case "extract":
        if (this.end > 1) { this.end--; this.root = 0; this.phase = "child"; }
        else this.phase = "complete";
        break;
    }
  }
}
