/** 🚚️ `drag-selection3d` payload — a relative drag of parts and target volumes in the world by one offset; mirrors Rust `DragSelection3d` (`../🦀️.rs`). */
export interface DragSelection3d {
  targets: string[];
  offset: [number, number, number];
}
