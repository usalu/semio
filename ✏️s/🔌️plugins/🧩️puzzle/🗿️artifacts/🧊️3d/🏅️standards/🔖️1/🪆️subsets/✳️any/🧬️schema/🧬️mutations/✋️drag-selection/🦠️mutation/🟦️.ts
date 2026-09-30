/** ✋️ `drag-selection` payload — a relative drag of objects and target volumes by one world offset; mirrors Rust `DragSelection` (`../🦀️.rs`). */
export interface DragSelection {
  targets: string[];
  offset: [number, number, number];
}
