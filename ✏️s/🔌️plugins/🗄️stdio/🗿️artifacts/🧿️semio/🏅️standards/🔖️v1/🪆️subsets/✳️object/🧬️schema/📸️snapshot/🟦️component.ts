/** 🧬️ SemioObjectSnapshot facet mirror — the `🦀️component.rs` sibling is the real source of
 * truth; this interface tracks its fields 1:1 (see `POLICY_FACET_MIRROR_DRIFT`). */
export type ObjectId = { value: string };

export type SemioValue =
  | { kind: "null" }
  | { kind: "bool"; value: boolean }
  | { kind: "int"; lexeme: string }
  | { kind: "float"; lexeme: string }
  | { kind: "str"; value: string }
  | { kind: "bytes"; value: number[] }
  | { kind: "list"; items: SemioValue[] }
  | { kind: "map"; entries: SemioObjectEntry[] }
  | { kind: "ref"; id: ObjectId };

export interface SemioObjectEntry {
  key: string;
  value: SemioValue;
}

export interface SemioObjectNode {
  id: ObjectId;
  value: SemioValue;
}

export interface SemioObjectSnapshot {
  /** @state persistent */ schema: string;
  /** @state persistent */ root: SemioValue;
  /** @state persistent */ objects: SemioObjectNode[];
}
