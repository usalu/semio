/** 🧬️ SemioValueSnapshot facet mirror — the `🦀️component.rs` sibling is the real source of
 * truth; this interface tracks its fields 1:1 (see `POLICY_FACET_MIRROR_DRIFT`). */
export type ValueId = { value: string };

export type SemioValue =
  | { kind: "null" }
  | { kind: "bool"; value: boolean }
  | { kind: "int"; lexeme: string }
  | { kind: "float"; lexeme: string }
  | { kind: "str"; value: string }
  | { kind: "bytes"; value: number[] }
  | { kind: "list"; items: SemioValue[] }
  | { kind: "map"; entries: SemioValueEntry[] }
  | { kind: "ref"; id: ValueId };

export interface SemioValueEntry {
  key: string;
  value: SemioValue;
}

export interface SemioValueNode {
  id: ValueId;
  value: SemioValue;
}

export interface SemioValueSnapshot {
  /** @state artifact */ schema: string;
  /** @state artifact */ root: SemioValue;
  /** @state artifact */ nodes: SemioValueNode[];
}
