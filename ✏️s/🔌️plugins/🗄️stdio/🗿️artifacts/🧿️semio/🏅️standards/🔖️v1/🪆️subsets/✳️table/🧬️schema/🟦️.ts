/** 🧬️ SemioTableArtifact schema — real facet mirror of the Rust `🦀️.rs` sibling. */
export type SemioTableCellKind = "null" | "bool" | "int" | "float" | "str" | "bytes";
export interface SemioTableColumn {
  name: string;
  kind: SemioTableCellKind;
}
export type SemioValue =
  | { kind: "null" }
  | { kind: "bool"; value: boolean }
  | { kind: "int"; value: string }
  | { kind: "float"; lexeme: string }
  | { kind: "str"; value: string }
  | { kind: "bytes"; value: string };
export interface SemioTableRow {
  cells: SemioValue[];
}
export interface SemioTableArtifact {
  /** @state artifact */ schema: string;
  /** @state artifact */ columns: SemioTableColumn[];
  /** @state artifact */ rows: SemioTableRow[];
}
