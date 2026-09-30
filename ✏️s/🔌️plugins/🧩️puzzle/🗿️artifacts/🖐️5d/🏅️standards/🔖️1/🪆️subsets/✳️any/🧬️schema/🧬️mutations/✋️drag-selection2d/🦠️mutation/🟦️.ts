/** ✋️ `drag-selection2d` payload — a relative drag of parts on the board by one flat offset; mirrors Rust `DragSelection2d` (`../🦀️.rs`). */
export interface DragSelection2d {
  targets: string[];
  dx: number;
  dy: number;
}
