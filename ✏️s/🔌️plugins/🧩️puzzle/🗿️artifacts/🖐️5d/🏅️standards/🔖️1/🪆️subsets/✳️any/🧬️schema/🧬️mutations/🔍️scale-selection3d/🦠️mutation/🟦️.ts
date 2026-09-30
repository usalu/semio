/** 🔍️ `scale-selection3d` payload — a relative per-axis scaling of parts and target volumes, each about its own origin; mirrors Rust `ScaleSelection3d` (`../🦀️.rs`). */
export interface ScaleSelection3d {
  targets: string[];
  factors: [number, number, number];
}
