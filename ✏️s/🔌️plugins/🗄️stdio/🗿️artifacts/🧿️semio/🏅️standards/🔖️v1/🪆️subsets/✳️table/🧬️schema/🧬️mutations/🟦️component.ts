/** 🧬️ SemioTableMutation schema — real facet mirror of the Rust `🦀️component.rs` sibling. Closed,
 * eight-variant dispatch: one interface per triad payload, tagged by `mutation`. */
export type SemioTableMutation =
  | { mutation: "createColumn"; payload: { name: string; kind: string; index?: number | null } }
  | { mutation: "deleteColumn"; payload: { name: string } }
  | { mutation: "renameColumn"; payload: { name: string; newName: string } }
  | { mutation: "reorderColumns"; payload: { name: string; toIndex: number } }
  | { mutation: "insertRow"; payload: { index: number; row: { cells: unknown[] } } }
  | { mutation: "removeRow"; payload: { index: number } }
  | { mutation: "reorderRows"; payload: { from: number; to: number } }
  | { mutation: "editCell"; payload: { rowIndex: number; columnName: string; newValue: unknown } };
