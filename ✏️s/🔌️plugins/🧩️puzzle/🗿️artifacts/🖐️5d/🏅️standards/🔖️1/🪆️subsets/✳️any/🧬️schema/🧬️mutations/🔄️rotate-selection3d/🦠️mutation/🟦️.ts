/** 🔄️ `rotate-selection3d` payload — a relative turn of parts and target volumes, each about its own origin; mirrors Rust `RotateSelection3d` (`../🦀️.rs`). */
export interface RotateSelection3d {
  targets: string[];
  axis: [number, number, number];
  angle: number;
}
