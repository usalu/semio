/** 🔍️ `scale-selection` payload — a relative per-axis scaling of objects and target volumes, each about its own origin; mirrors Rust `ScaleSelection` (`../🦀️.rs`). */
export interface ScaleSelection {
  targets: string[];
  factors: [number, number, number];
}
