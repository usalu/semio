/** 🧬️ SemioTableSnapshot schema — real facet mirror of the Rust `🦀️component.rs` sibling. */
export type SemioTableCellKind = "null" | "bool" | "int" | "float" | "str" | "bytes";

export interface SemioTableColumn {
  /** native key — name-keyed collection, like cad layers / xlsx sheets */
  name: string;
  kind: SemioTableCellKind;
}

/** cell data reuses `SemioValue` verbatim from the `value` subset. */
export type SemioValue = import("../../../✳️value/🧬️schema/📸️snapshot/🟦️component.ts").SemioValue;

export interface SemioTableRow {
  /** positionally aligned with `SemioTableSnapshot.columns` — cells[j] belongs to columns[j] */
  cells: SemioValue[];
}

export interface SemioTableSnapshot {
  /** @state persistent */ schema: string;
  /** @state persistent */ columns: SemioTableColumn[];
  /** @state persistent */ rows: SemioTableRow[];
}
