/** 🔄️ `rotate-selection` payload — a relative turn of objects and target volumes, each about its own origin; mirrors Rust `RotateSelection` (`../🦀️.rs`). */
export interface RotateSelection {
  targets: string[];
  axis: [number, number, number];
  angle: number;
}
