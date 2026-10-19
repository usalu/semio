/** 🔁️ Memoised recursion over cyclic graphs: a lookup that reaches a computation still in progress answers with a fallback, every result that never saw such a cut is final, and a result that did stays reusable exactly as long as the outermost computation it was cut by is still in progress, so one top-level query visits each node once instead of once per path. */
export class CycleMemo {
  private readonly final = new Map<string, unknown>();
  private readonly provisional = new Map<string, { value: unknown; head: number }>();
  private readonly byHead = new Map<number, string[]>();
  private readonly active = new Map<string, number>();
  private readonly live = new Set<number>();
  private outermost = Number.POSITIVE_INFINITY;
  private next = 1;

  private recall(key: string): { value: unknown } | undefined {
    if (this.final.has(key)) return { value: this.final.get(key) };
    const known = this.provisional.get(key);
    if (!known || !this.live.has(known.head)) return undefined;
    this.outermost = Math.min(this.outermost, known.head);
    return { value: known.value };
  }

  compute<Value>(key: string, fallback: () => Value, run: () => Value): Value {
    const known = this.recall(key);
    if (known) return known.value as Value;
    const running = this.active.get(key);
    if (running !== undefined) { this.outermost = Math.min(this.outermost, running); return fallback(); }
    const id = this.next++;
    const outer = this.outermost;
    this.outermost = Number.POSITIVE_INFINITY;
    this.active.set(key, id);
    this.live.add(id);
    let value: Value;
    let head: number;
    try { value = run(); } finally {
      head = this.outermost;
      this.outermost = Number.POSITIVE_INFINITY;
      this.active.delete(key);
      this.live.delete(id);
      for (const stale of this.byHead.get(id) ?? []) this.provisional.delete(stale);
      this.byHead.delete(id);
    }
    if (head < id) {
      this.outermost = Math.min(outer, head);
      this.provisional.set(key, { value, head });
      this.byHead.set(head, [...(this.byHead.get(head) ?? []), key]);
    } else {
      this.outermost = outer;
      this.final.set(key, value);
    }
    return value;
  }
}
